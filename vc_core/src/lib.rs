use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

#[derive(Clone, Serialize, Deserialize)]
pub struct URL(String);

impl URL {
    pub fn new(url: &str) -> Result<Self, Box<dyn Error>> {
        Ok(Self(url.to_string()))
    }

    pub fn get_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for URL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_str())
    }
}

pub struct VerifiableCredential {}

pub struct VerifiablePresentation {}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
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
