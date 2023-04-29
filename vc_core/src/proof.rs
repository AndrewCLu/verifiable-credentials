use super::*;
use chrono::{DateTime, Utc};

pub struct ProofOptions {
    verification_method: VerificationMethod,
    proof_purpose: String,
    created: DateTime<Utc>,
    domain: String,
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

pub trait CryptographicSuite {
    fn get_id(&self) -> &URL;

    fn get_type(&self) -> &String;

    fn transform(
        &self,
        data: &VerifiableCredential,
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
        options: &ProofOptions,
    ) -> Result<Proof, ProofGenerationError>;

    fn generate_proof(
        &self,
        data: &VerifiableCredential,
        options: &ProofOptions,
    ) -> Result<Proof, ProofGenerationError> {
        let transformed_data = self.transform(data, options)?;
        let hash_data = self.hash(&transformed_data, options)?;
        let proof = self.prove(&hash_data, options)?;
        Ok(proof)
    }
}

pub struct ECDSAProof2021 {
    id: URL,
    type_: String,
}

impl ECDSAProof2021 {
    pub fn new() -> Self {
        Self {
            id: URL::new("https://w3id.org/security#proof-ecdsa-secp256k1-2021").unwrap(),
            type_: "EcdsaSecp256k1Signature2021".to_string(),
        }
    }
}

impl CryptographicSuite for ECDSAProof2021 {
    fn get_id(&self) -> &URL {
        &self.id
    }

    fn get_type(&self) -> &String {
        &self.type_
    }

    fn transform(
        &self,
        data: &VerifiableCredential,
        options: &ProofOptions,
    ) -> Result<Vec<u8>, ProofGenerationError> {
        Ok(vec![])
    }

    fn hash(
        &self,
        transformed_data: &[u8],
        options: &ProofOptions,
    ) -> Result<Vec<u8>, ProofGenerationError> {
        Ok(vec![])
    }

    fn prove(
        &self,
        hash_data: &[u8],
        options: &ProofOptions,
    ) -> Result<Proof, ProofGenerationError> {
        let type_ = self.get_type().clone();
        let created = options.created;
        let verification_method = options.verification_method.get_id().clone();
        let proof_purpose = options.proof_purpose.clone();
        let proof_value = "Example proof value".to_string();
        Ok(Proof::new(
            type_,
            created,
            verification_method,
            proof_purpose,
            proof_value,
        ))
    }
}
