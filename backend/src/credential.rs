use super::UserError;
use crate::registry::VerifiableDataRegistry;
use actix_web::{post, web, HttpResponse, Scope};
use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use log::{error, info};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Mutex;
use vc_core::{ClaimProperty, Proof, VerifiableCredential, VerificationMethod, URL};

fn generate_proof(credential: &VerifiableCredential, verification: &VerificationMethod) -> Proof {
    return Proof::new(
        "proof id".to_string(),
        Utc::now(),
        verification.get_id().clone(),
        "proof purpose".to_string(),
        "proof_value".to_string(),
    );
}

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
    credential_subject: HashMap<String, ClaimProperty>,
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

    let mut context = Vec::new();
    for context_url in &req.context.clone() {
        let context_url = URL::new(context_url).map_err(|_e| {
            error!("Invalid context url.");
            UserError::BadRequest
        })?;
        context.push(context_url);
    }
    let credential_id = URL::new(&req.credential_id).map_err(|_e| {
        error!("Invalid credential id.");
        UserError::BadRequest
    })?;
    let mut type_ = Vec::new();
    for type_url in &req.type_.clone() {
        let type_url = URL::new(type_url).map_err(|_e| {
            error!("Invalid type url.");
            UserError::BadRequest
        })?;
        type_.push(type_url);
    }
    let issuer_id = URL::new(&req.issuer_id).map_err(|_e| {
        error!("Invalid issuer id.");
        UserError::BadRequest
    })?;
    let issuer = registry
        .get_issuer(&issuer_id)
        .map_err(|e| {
            error!("Error getting issuer {} from registry: {:?}", issuer_id, e);
            UserError::InternalServerError
        })?
        .ok_or({
            error!("Could not find issuer {} in registry.", issuer_id);
            UserError::BadRequest
        })?;
    let valid_from = req.valid_from;
    let valid_until = req.valid_until;
    let credential_subject = req.credential_subject.clone();
    let mut credential_schema = Vec::new();
    for credential_schema_id in &req.credential_schema_ids.clone() {
        let credential_schema_id = URL::new(credential_schema_id).map_err(|_e| {
            error!("Invalid credential schema id.");
            UserError::BadRequest
        })?;
        let schema = registry
            .get_schema(&credential_schema_id)
            .map_err(|e| {
                error!(
                    "Error getting credential schema {} from registry: {:?}",
                    credential_schema_id, e
                );
                UserError::InternalServerError
            })?
            .ok_or({
                error!(
                    "Could not find credential schema {} in registry.",
                    credential_schema_id
                );
                UserError::BadRequest
            })?;
        credential_schema.push(schema.get_link());
    }
    let proof = Vec::new();

    let credential = VerifiableCredential::new(
        context,
        credential_id.clone(),
        type_,
        issuer_id,
        valid_from,
        valid_until,
        credential_subject,
        credential_schema,
        proof, // TODO
    );

    info!("Generated new credential for user: {}", credential_id);
    Ok(HttpResponse::Ok().json(credential))
}

pub fn init_routes() -> Scope {
    web::scope("/credential").service(new_credential)
}
