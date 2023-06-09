use super::*;
use chrono::{DateTime, Utc};
use k256::ecdsa::{
    signature::{Signer, Verifier},
    Signature, SigningKey, VerifyingKey,
};

pub struct ProofOptions {
    verification_method: VerificationMethod,
    proof_purpose: String,
    created: DateTime<Utc>,
    #[allow(dead_code)]
    domain: String,
    #[allow(dead_code)]
    challenge: String,
}

impl ProofOptions {
    pub fn new(
        verification_method: VerificationMethod,
        proof_purpose: String,
        created: DateTime<Utc>,
        domain: String,
        challenge: String,
    ) -> Self {
        Self {
            verification_method,
            proof_purpose,
            created,
            domain,
            challenge,
        }
    }
}

#[derive(Debug)]
pub enum ProofGenerationError {
    Error,
}

impl fmt::Display for ProofGenerationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub enum ProofVerificationError {
    MismatchedProofPurposeError,
    BadTransformationError,
    BadHashingError,
    MalformedProofError,
    InvalidPublicKeyError,
}

impl fmt::Display for ProofVerificationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MismatchedProofPurposeError => {
                write!(f, "Mismatched proof purpose.")
            }
            Self::BadTransformationError => {
                write!(f, "Bad transformation.")
            }
            Self::BadHashingError => {
                write!(f, "Bad hashing.")
            }
            Self::MalformedProofError => {
                write!(f, "Malformed proof.")
            }
            Self::InvalidPublicKeyError => {
                write!(f, "Invalid public key.")
            }
        }
    }
}

pub trait CryptographicSuite {
    type DataDocument;
    type OutputProof;

    fn get_id(&self) -> &URL;

    fn get_type(&self) -> &String;

    fn transform(
        &self,
        data: &Self::DataDocument,
        options: &ProofOptions,
    ) -> Result<Vec<u8>, ProofGenerationError>;

    fn hash(
        &self,
        transformed_data: &[u8],
        options: &ProofOptions,
    ) -> Result<Vec<u8>, ProofGenerationError>;

    fn prove(
        &self,
        hash_data: &[u8],
        proving_key: &[u8],
        options: &ProofOptions,
    ) -> Result<Self::OutputProof, ProofGenerationError>;

    fn generate_proof(
        &self,
        data: &Self::DataDocument,
        proving_key: &[u8],
        options: &ProofOptions,
    ) -> Result<Self::OutputProof, ProofGenerationError> {
        let transformed_data = self.transform(data, options)?;
        let hash_data = self.hash(&transformed_data, options)?;
        let proof = self.prove(&hash_data, proving_key, options)?;
        Ok(proof)
    }

    fn verify_proof(
        &self,
        data: &Self::DataDocument,
        proof: &Self::OutputProof,
        verifying_key: &[u8],
        options: &ProofOptions,
    ) -> Result<bool, ProofVerificationError>;
}

pub struct MyEcdsaSecp256k1 {
    id: URL,
    type_: String,
}

impl MyEcdsaSecp256k1 {
    pub fn new() -> Self {
        Self {
            id: URL::new("https://w3id.org/security#proof-ecdsa-secp256k1-2021").unwrap(),
            type_: "MyEcdsaSecp256k1Signature".to_string(),
        }
    }
}

impl CryptographicSuite for MyEcdsaSecp256k1 {
    type DataDocument = Credential;
    type OutputProof = Proof;

    fn get_id(&self) -> &URL {
        &self.id
    }

    fn get_type(&self) -> &String {
        &self.type_
    }

    fn transform(
        &self,
        data: &Credential,
        _options: &ProofOptions,
    ) -> Result<Vec<u8>, ProofGenerationError> {
        let credential_string =
            serde_json::to_string(data).map_err(|_| ProofGenerationError::Error)?;
        Ok(credential_string.as_bytes().to_vec())
    }

    fn hash(
        &self,
        transformed_data: &[u8],
        _options: &ProofOptions,
    ) -> Result<Vec<u8>, ProofGenerationError> {
        Ok(blake3::hash(transformed_data).as_bytes().to_vec())
    }

    fn prove(
        &self,
        hash_data: &[u8],
        proving_key: &[u8],
        options: &ProofOptions,
    ) -> Result<Proof, ProofGenerationError> {
        let type_ = self.get_type().clone();
        let created = options.created;
        let verification_method = options.verification_method.get_id().clone();
        let proof_purpose = options.proof_purpose.clone();
        let signing_key =
            SigningKey::from_slice(proving_key).map_err(|_| ProofGenerationError::Error)?;
        let signature: Signature = signing_key.sign(hash_data);
        let proof_value = signature.to_vec();
        Ok(Proof::new(
            type_,
            created,
            verification_method,
            proof_purpose,
            proof_value,
        ))
    }

    fn verify_proof(
        &self,
        data: &Credential,
        proof: &Proof,
        verifying_key: &[u8],
        options: &ProofOptions,
    ) -> Result<bool, ProofVerificationError> {
        if *proof.get_proof_purpose() != options.proof_purpose {
            return Err(ProofVerificationError::MismatchedProofPurposeError);
        }
        let transformed_data = self
            .transform(data, options)
            .map_err(|_| ProofVerificationError::BadTransformationError)?;
        let hash_data = self
            .hash(&transformed_data, options)
            .map_err(|_| ProofVerificationError::BadHashingError)?;
        let signature = Signature::from_slice(proof.get_proof_value())
            .map_err(|_| ProofVerificationError::MalformedProofError)?;
        let public_key = VerifyingKey::from_sec1_bytes(verifying_key)
            .map_err(|_| ProofVerificationError::InvalidPublicKeyError)?;

        Ok(public_key.verify(&hash_data, &signature).is_ok())
    }
}
