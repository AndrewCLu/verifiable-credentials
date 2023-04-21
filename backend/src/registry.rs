use rocksdb::DB;
use std::error::Error;
use std::fmt;
use vc_core::{Issuer, VerificationMethod, URL};

#[derive(Debug)]
pub enum RegistryError {
    SerializationError(String),
    DatabaseError(String),
}

impl Error for RegistryError {}

impl fmt::Display for RegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RegistryError::SerializationError(e) => write!(f, "Serialization error: {}", e),
            RegistryError::DatabaseError(e) => write!(f, "Database error: {}", e),
        }
    }
}

pub struct VerifiableDataRegistry {
    db: DB,
}

impl VerifiableDataRegistry {
    pub fn new(db: DB) -> Self {
        Self { db }
    }

    pub fn add_issuer(&mut self, issuer: Issuer) -> Result<(), RegistryError> {
        let issuer_json = serde_json::to_string(&issuer).map_err(|_e| {
            RegistryError::SerializationError("Could not serialize issuer.".to_string())
        })?;
        self.db
            .put(issuer.get_id().get_str().as_bytes(), issuer_json.as_bytes())
            .map_err(|_e| {
                RegistryError::DatabaseError(format!(
                    "Could not insert issuer {} into database.",
                    issuer.get_id()
                ))
            })?;

        Ok(())
    }

    pub fn get_issuer(&self, issuer_id: URL) -> Result<Option<Issuer>, RegistryError> {
        self.db
            .get(issuer_id.get_str().as_bytes())
            .map_err(|_e| {
                RegistryError::DatabaseError(format!(
                    "Could not retrieve issuer {} from database.",
                    issuer_id
                ))
            })
            .and_then(|maybe_issuer_json_bytes| {
                maybe_issuer_json_bytes
                    .map(|issuer_json_bytes| {
                        String::from_utf8(issuer_json_bytes)
                            .map_err(|_e| {
                                RegistryError::SerializationError(
                                    "Could not deserialize issuer.".to_string(),
                                )
                            })
                            .and_then(|issuer_json| {
                                serde_json::from_str::<Issuer>(issuer_json.as_str()).map_err(|_e| {
                                    RegistryError::SerializationError(
                                        "Could not deserialize issuer.".to_string(),
                                    )
                                })
                            })
                    })
                    .transpose()
            })
    }

    pub fn add_verification_method(
        &mut self,
        issuser_id: URL,
        verification_method: VerificationMethod,
    ) -> Result<(), String> {
        Ok(())
    }
}
