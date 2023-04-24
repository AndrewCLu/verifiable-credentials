use rocksdb::{ColumnFamily, ColumnFamilyDescriptor, IteratorMode, Options, DB};
use std::error::Error;
use std::fmt;
use vc_core::{CredentialSchema, Issuer, VerificationMethod, URL};

#[derive(Debug)]
pub enum RegistryError {
    SerializationError(String),
    DatabaseError(String),
    ArgumentError(String),
}

impl Error for RegistryError {}

impl fmt::Display for RegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RegistryError::SerializationError(e) => write!(f, "Serialization error: {}", e),
            RegistryError::DatabaseError(e) => write!(f, "Database error: {}", e),
            RegistryError::ArgumentError(e) => write!(f, "Authorization error: {}", e),
        }
    }
}

pub struct VerifiableDataRegistry {
    db: DB,
}

impl VerifiableDataRegistry {
    const ISSUER_PATH: &'static str = "issuer";
    const SCHEMA_PATH: &'static str = "schema";
    const DEFAULT_RESOURCE_LIMIT: usize = 20;

    pub fn new(db_path: &str) -> Result<Self, RegistryError> {
        let mut db_options = Options::default();
        db_options.create_if_missing(true);
        db_options.create_missing_column_families(true);
        let issuer_cf = ColumnFamilyDescriptor::new(Self::ISSUER_PATH, Options::default());
        let schema_cf = ColumnFamilyDescriptor::new(Self::SCHEMA_PATH, Options::default());
        let db = DB::open_cf_descriptors(&db_options, db_path, vec![issuer_cf, schema_cf])
            .map_err(|_e| RegistryError::DatabaseError("Could not open database.".to_string()))?;

        Ok(Self { db })
    }

    fn issuer_cf(&self) -> Result<&ColumnFamily, RegistryError> {
        self.db
            .cf_handle(Self::ISSUER_PATH)
            .ok_or(RegistryError::DatabaseError(
                "Could not fetch issuer handler.".to_string(),
            ))
    }

    fn schema_cf(&self) -> Result<&ColumnFamily, RegistryError> {
        self.db
            .cf_handle(Self::SCHEMA_PATH)
            .ok_or(RegistryError::DatabaseError(
                "Could not fetch schema handler.".to_string(),
            ))
    }

    pub fn add_issuer(&mut self, issuer: Issuer) -> Result<(), RegistryError> {
        let issuer_json = serde_json::to_string(&issuer).map_err(|_e| {
            RegistryError::SerializationError("Could not serialize issuer.".to_string())
        })?;
        self.db
            .put_cf(
                self.issuer_cf()?,
                issuer.get_id().get_str().as_bytes(),
                issuer_json.as_bytes(),
            )
            .map_err(|_e| {
                RegistryError::DatabaseError(format!(
                    "Could not insert issuer {} into database.",
                    issuer.get_id()
                ))
            })?;

        Ok(())
    }

    pub fn get_issuer(&self, issuer_id: &URL) -> Result<Option<Issuer>, RegistryError> {
        self.db
            .get_cf(self.issuer_cf()?, issuer_id.get_str().as_bytes())
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

    pub fn get_all_issuers(self, limit: Option<usize>) -> Result<Vec<Issuer>, RegistryError> {
        let issuers = self.db.iterator_cf(self.issuer_cf()?, IteratorMode::Start);
        let limit = limit.unwrap_or(Self::DEFAULT_RESOURCE_LIMIT);

        Ok(issuers
            .filter_map(|result| {
                if let Ok((_key, value)) = result {
                    match String::from_utf8(value.to_vec()) {
                        Ok(issuer_json) => {
                            match serde_json::from_str::<Issuer>(issuer_json.as_str()) {
                                Ok(issuer) => Some(issuer),
                                Err(_) => {
                                    eprintln!(
                                        "{:?}",
                                        RegistryError::SerializationError(
                                            "Could not deserialize issuer.".to_string()
                                        )
                                    );
                                    None
                                }
                            }
                        }
                        Err(_) => {
                            eprintln!(
                                "{:?}",
                                RegistryError::SerializationError(
                                    "Could not deserialize issuer.".to_string()
                                )
                            );
                            None
                        }
                    }
                } else {
                    eprintln!(
                        "{:?}",
                        RegistryError::DatabaseError("Could not fetch issuers.".to_string())
                    );
                    None
                }
            })
            .take(limit)
            .collect())
    }

    pub fn add_verification_method(
        &mut self,
        issuer_id: &URL,
        verification_method: VerificationMethod,
    ) -> Result<(), RegistryError> {
        match self.get_issuer(issuer_id) {
            Ok(Some(mut issuer)) => {
                issuer.add_verification_method(verification_method);
                self.add_issuer(issuer)
            }
            Ok(None) => Err(RegistryError::ArgumentError(format!(
                "Issuer {} does not exist in the registry.",
                issuer_id
            ))),
            Err(e) => Err(e),
        }
    }

    pub fn add_schema(&mut self, schema: CredentialSchema) -> Result<(), RegistryError> {
        let schema_json = serde_json::to_string(&schema).map_err(|_e| {
            RegistryError::SerializationError("Could not serialize schema.".to_string())
        })?;
        self.db
            .put_cf(
                self.schema_cf()?,
                schema.get_id().get_str().as_bytes(),
                schema_json.as_bytes(),
            )
            .map_err(|_e| {
                RegistryError::DatabaseError(format!(
                    "Could not insert schema {} into database.",
                    schema.get_id()
                ))
            })?;

        Ok(())
    }
}
