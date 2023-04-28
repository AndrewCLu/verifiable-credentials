use super::UserError;
use crate::registry::VerifiableDataRegistry;
use actix_web::{post, web, HttpResponse, Scope};
use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use log::{error, info};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Mutex;
use vc_core::{CredentialSchema, URL};

#[derive(Deserialize)]
struct NewCredentialRequest {
    context: Vec<String>,
    credential_id: String,
    type_: Vec<String>,
    issuer_id: String,
    #[serde(with = "ts_seconds")]
    valid_from: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    valid_until: DateTime<Utc>,
    credential_subject: HashMap<String, String>,
    credential_schema_ids: Vec<String>,
}

#[post("/")]
async fn new_credential(
    req: web::Json<NewCredentialRequest>,
    registry: web::Data<Mutex<VerifiableDataRegistry>>,
) -> Result<HttpResponse, UserError> {
    let registry = registry.lock().map_err(|_e| {
        error!("Could not lock registry.");
        UserError::InternalServerError
    })?;

    let credential_id = URL::new(&req.credential_id).map_err(|_e| {
        error!("Invalid credential id.");
        UserError::BadRequest
    })?;

    info!("Generated new credential for user: {}", credential_id);
    Ok(HttpResponse::Ok().json(credential_id))
}

pub fn init_routes() -> Scope {
    web::scope("/credential").service(new_credential)
}
