use std::error::Error;
use vc_core::{Issuer, VerificationMethod, URL};
pub struct Registry {}

impl Registry {
    pub fn new() -> Self {
        Self {}
    }

    pub fn register_issuer(&mut self, iss: Issuer) -> Result<(), String> {
        Ok(())
    }

    pub fn register_verification_method(
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
