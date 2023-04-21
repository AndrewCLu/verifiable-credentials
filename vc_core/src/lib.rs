use serde::Serialize;
use std::error::Error;

#[derive(Clone, Serialize)]
pub struct URL(String);

impl URL {
    pub fn new(url: &str) -> Result<Self, Box<dyn Error>> {
        Ok(Self(url.to_string()))
    }

    pub fn get_str(&self) -> &str {
        &self.0
    }
}

pub struct VerifiableCredential {}

pub struct VerifiablePresentation {}

pub struct Issuer {
    id: URL,
    name: String,
    verification_methods: Vec<VerificationMethod>,
}

impl Issuer {
    pub fn new(id: URL, name: String) -> Self {
        Self {
            id,
            name,
            verification_methods: vec![],
        }
    }

    pub fn get_id(&self) -> &URL {
        &self.id
    }
}

pub struct VerificationMethod {
    id: URL,
    type_: String,
    controller: URL,
    public_key_multibase: String,
}

impl VerificationMethod {
    pub fn new(id: URL, type_: String, controller: URL, public_key_multibase: String) -> Self {
        Self {
            id,
            type_,
            controller,
            public_key_multibase,
        }
    }
}
