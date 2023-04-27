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

    pub fn get_verification_methods(&self) -> &Vec<VerificationMethod> {
        &self.verification_methods
    }

    pub fn new_verification_method(&mut self, verification_method: VerificationMethod) {
        self.verification_methods.push(verification_method);
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct VerificationMethod {
    id: URL,
    type_: String,
    controller_id: URL,
    public_key_multibase: String,
}

impl VerificationMethod {
    pub fn new(id: URL, type_: String, controller_id: URL, public_key_multibase: String) -> Self {
        Self {
            id,
            type_,
            controller_id,
            public_key_multibase,
        }
    }

    pub fn get_id(&self) -> &URL {
        &self.id
    }

    pub fn get_type(&self) -> &String {
        &self.type_
    }

    pub fn get_controller_id(&self) -> &URL {
        &self.controller_id
    }

    pub fn get_public_key_multibase(&self) -> &String {
        &self.public_key_multibase
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub enum SchemaPropertyType {
    Text,
    Number,
    Boolean,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct SchemaPropertyValue {
    type_: SchemaPropertyType,
    description: String,
}

impl SchemaPropertyValue {
    pub fn new(type_: SchemaPropertyType, description: String) -> Self {
        Self { type_, description }
    }

    pub fn get_type(&self) -> &SchemaPropertyType {
        &self.type_
    }

    pub fn get_description(&self) -> &String {
        &self.description
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub enum SchemaProperty {
    Value(SchemaPropertyValue),
    Array(Vec<SchemaProperty>),
    Map(HashMap<String, SchemaProperty>),
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct CredentialSchema {
    id: URL,
    type_: String,
    name: String,
    description: String,
    properties: HashMap<String, SchemaProperty>,
}

impl CredentialSchema {
    pub fn new(
        id: URL,
        type_: String,
        name: String,
        description: String,
        properties: HashMap<String, SchemaProperty>,
    ) -> Self {
        Self {
            id,
            type_,
            name,
            description,
            properties,
        }
    }

    pub fn get_id(&self) -> &URL {
        &self.id
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_description(&self) -> &String {
        &self.description
    }

    pub fn get_properties(&self) -> &HashMap<String, SchemaProperty> {
        &self.properties
    }
}

pub struct VerifiableCredential {}

pub struct VerifiablePresentation {}
