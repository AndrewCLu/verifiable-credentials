use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

#[derive(Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Clone, Serialize, Deserialize, PartialEq)]
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

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn add_verification_method(&mut self, verification_method: VerificationMethod) {
        self.verification_methods.push(verification_method);
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Serialize, Deserialize)]
pub enum SchemaPropertyType {
    Text,
    Number,
    Boolean,
}

#[derive(Serialize, Deserialize)]
pub struct SchemaPropertyValue {
    type_: SchemaPropertyType,
    description: String,
}

impl SchemaPropertyValue {
    pub fn new(type_: SchemaPropertyType, description: String) -> Self {
        Self { type_, description }
    }
}

#[derive(Serialize, Deserialize)]
pub enum SchemaProperty {
    Value(SchemaPropertyValue),
    Array(Vec<SchemaProperty>),
    Map(HashMap<String, SchemaProperty>),
}

#[derive(Serialize, Deserialize)]
pub struct CredentialSchema {
    id: URL,
    type_: String,
    name: String,
    description: String,
    creator_id: URL,
    properties: HashMap<String, SchemaProperty>,
}

impl CredentialSchema {
    pub fn new(
        id: URL,
        type_: String,
        name: String,
        description: String,
        creator_id: URL,
        properties: HashMap<String, SchemaProperty>,
    ) -> Self {
        Self {
            id,
            type_,
            name,
            description,
            creator_id,
            properties,
        }
    }

    pub fn get_id(&self) -> &URL {
        &self.id
    }
}

pub struct VerifiableCredential {}

pub struct VerifiablePresentation {}
