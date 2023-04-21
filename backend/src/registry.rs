use vc_core::{Issuer, VerificationMethod, URL};

pub struct VerifiableDataRegistry {}

impl VerifiableDataRegistry {
    pub fn new() -> Self {
        Self {}
    }

    pub fn add_issuer(&mut self, iss: Issuer) -> Result<(), String> {
        Ok(())
    }

    pub fn add_verification_method(
        &mut self,
        iss_id: URL,
        vm: VerificationMethod,
    ) -> Result<(), String> {
        Ok(())
    }

    pub fn get_issuer(&self, iss_id: URL) -> Result<Issuer, String> {
        return Err("asd".to_string());
    }
}
