use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

pub mod proof;

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
    public_key_multibase: Vec<u8>,
}

impl VerificationMethod {
    pub fn new(id: URL, type_: String, controller_id: URL, public_key_multibase: Vec<u8>) -> Self {
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

    pub fn get_public_key_multibase(&self) -> &Vec<u8> {
        &self.public_key_multibase
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub enum SchemaPropertyValueType {
    Text,
    Number,
    Boolean,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct SchemaPropertyValue {
    type_: SchemaPropertyValueType,
    description: String,
}

impl SchemaPropertyValue {
    pub fn new(type_: SchemaPropertyValueType, description: String) -> Self {
        Self { type_, description }
    }

    pub fn get_type(&self) -> &SchemaPropertyValueType {
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

    pub fn get_link(&self) -> CredentialSchemaLink {
        CredentialSchemaLink::new(self.id.clone(), self.type_.clone())
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct CredentialSchemaLink {
    id: URL,
    type_: String,
}

impl CredentialSchemaLink {
    pub fn new(id: URL, type_: String) -> Self {
        Self { id, type_ }
    }

    pub fn get_id(&self) -> &URL {
        &self.id
    }

    pub fn get_type(&self) -> &String {
        &self.type_
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub enum ClaimPropertyValue {
    Text(String),
    Number(i32),
    Boolean(bool),
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub enum ClaimProperty {
    Value(ClaimPropertyValue),
    Array(Vec<ClaimProperty>),
    Map(HashMap<String, ClaimProperty>),
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct CredentialStatus {}

impl CredentialStatus {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct RefreshService {}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct TermsOfUse {}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Evidence {}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Proof {
    type_: String,
    created: DateTime<Utc>,
    verification_method: URL,
    proof_purpose: String,
    proof_value: Vec<u8>,
}

impl Proof {
    pub fn new(
        type_: String,
        created: DateTime<Utc>,
        verification_method: URL,
        proof_purpose: String,
        proof_value: Vec<u8>,
    ) -> Self {
        Self {
            type_,
            created,
            verification_method,
            proof_purpose,
            proof_value,
        }
    }

    pub fn get_type(&self) -> &String {
        &self.type_
    }

    pub fn get_created(&self) -> &DateTime<Utc> {
        &self.created
    }

    pub fn get_verification_method(&self) -> &URL {
        &self.verification_method
    }

    pub fn get_proof_purpose(&self) -> &String {
        &self.proof_purpose
    }

    pub fn get_proof_value(&self) -> &Vec<u8> {
        &self.proof_value
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Credential {
    context: Vec<URL>,
    id: URL,
    type_: Vec<URL>,
    issuer: URL,
    #[serde(with = "ts_seconds")]
    valid_from: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    valid_until: DateTime<Utc>,
    credential_subject: HashMap<String, ClaimProperty>,
    credential_schema: Vec<CredentialSchemaLink>,
    credential_status: CredentialStatus,
    refresh_service: Vec<RefreshService>,
    terms_of_use: Vec<TermsOfUse>,
    evidence: Vec<Evidence>,
}

impl Credential {
    pub fn new(
        context: Vec<URL>,
        id: URL,
        type_: Vec<URL>,
        issuer: URL,
        valid_from: DateTime<Utc>,
        valid_until: DateTime<Utc>,
        credential_subject: HashMap<String, ClaimProperty>,
        credential_schema: Vec<CredentialSchemaLink>,
    ) -> Self {
        let credential_status = CredentialStatus::new();
        let refresh_service = Vec::new();
        let terms_of_use = Vec::new();
        let evidence = Vec::new();
        Self {
            context,
            id,
            type_,
            issuer,
            valid_from,
            valid_until,
            credential_subject,
            credential_schema,
            credential_status,
            refresh_service,
            terms_of_use,
            evidence,
        }
    }

    pub fn get_context(&self) -> &Vec<URL> {
        &self.context
    }

    pub fn get_id(&self) -> &URL {
        &self.id
    }

    pub fn get_type(&self) -> &Vec<URL> {
        &self.type_
    }

    pub fn get_issuer(&self) -> &URL {
        &self.issuer
    }

    pub fn get_valid_from(&self) -> &DateTime<Utc> {
        &self.valid_from
    }

    pub fn get_valid_until(&self) -> &DateTime<Utc> {
        &self.valid_until
    }

    pub fn get_credential_subject(&self) -> &HashMap<String, ClaimProperty> {
        &self.credential_subject
    }

    pub fn get_credential_schema(&self) -> &Vec<CredentialSchemaLink> {
        &self.credential_schema
    }

    pub fn get_credential_status(&self) -> &CredentialStatus {
        &self.credential_status
    }

    pub fn get_refresh_service(&self) -> &Vec<RefreshService> {
        &self.refresh_service
    }

    pub fn get_terms_of_use(&self) -> &Vec<TermsOfUse> {
        &self.terms_of_use
    }

    pub fn get_evidence(&self) -> &Vec<Evidence> {
        &self.evidence
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct VerifiableCredential {
    credential: Credential,
    proof: Vec<Proof>,
}

impl VerifiableCredential {
    pub fn new(credential: Credential, proof: Vec<Proof>) -> Self {
        Self { credential, proof }
    }

    pub fn get_credential(&self) -> &Credential {
        &self.credential
    }

    pub fn get_proof(&self) -> &Vec<Proof> {
        &self.proof
    }
}

pub struct VerifiablePresentation {
    context: Vec<URL>,
    id: URL,
    type_: Vec<URL>,
    verifiable_credential: Vec<VerifiableCredential>,
    holder: Option<URL>,
    proof: Vec<Proof>,
}

impl VerifiablePresentation {
    pub fn new(
        context: Vec<URL>,
        id: URL,
        type_: Vec<URL>,
        verifiable_credential: Vec<VerifiableCredential>,
        holder: Option<URL>,
        proof: Vec<Proof>,
    ) -> Self {
        Self {
            context,
            id,
            type_,
            verifiable_credential,
            holder,
            proof,
        }
    }

    pub fn get_context(&self) -> &Vec<URL> {
        &self.context
    }

    pub fn get_id(&self) -> &URL {
        &self.id
    }

    pub fn get_type(&self) -> &Vec<URL> {
        &self.type_
    }

    pub fn get_verifiable_credential(&self) -> &Vec<VerifiableCredential> {
        &self.verifiable_credential
    }

    pub fn get_holder(&self) -> &Option<URL> {
        &self.holder
    }

    pub fn get_proof(&self) -> &Vec<Proof> {
        &self.proof
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Verifier {
    id: URL,
    schema_id: URL,
}

impl Verifier {
    pub fn new(id: URL, schema_id: URL) -> Self {
        Self { id, schema_id }
    }

    pub fn get_id(&self) -> &URL {
        &self.id
    }

    pub fn get_schema_id(&self) -> &URL {
        &self.schema_id
    }
}
