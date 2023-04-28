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
    id: String,
    type_: Vec<String>,
    issuer: String,
    #[serde(with = "ts_seconds")]
    valid_from: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    valid_until: DateTime<Utc>,
    credential_subject: HashMap<String, String>,
    credential_schema: Vec<CredentialSchema>,
}

#[post("/")]
async fn new_credential(
    req: web::Json<NewCredentialRequest>,
    registry: web::Data<Mutex<VerifiableDataRegistry>>,
) -> Result<HttpResponse, UserError> {
    let id = URL::new(&req.id).map_err(|_e| {
        error!("Invalid credential id.");
        UserError::BadRequest
    })?;

    info!("Generated new credential for user: {}", id);
    Ok(HttpResponse::Ok().json(id))
}

pub fn init_routes() -> Scope {
    web::scope("/credential").service(new_credential)
}
