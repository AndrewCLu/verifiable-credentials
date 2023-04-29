use super::*;
use chrono::{DateTime, Utc};

pub struct ProofOptions {
    verification_method: VerificationMethod,
    cryptographic_suite: Box<dyn CryptographicSuite>,
    proof_purpose: String,
    created: DateTime<Utc>,
    domain: String,
    challenge: String,
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
