use super::UserError;
use crate::registry::VerifiableDataRegistry;
use actix_web::{post, web, HttpResponse, Scope};
use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use log::{error, info};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Mutex;
use vc_core::{Proof, VerifiableCredential, VerificationMethod, URL};

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

    let mut context = Vec::new();
    let req_context = &req.context.clone();
    for context_url in req_context {
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
    let req_type = &req.type_.clone();
    for type_url in req_type {
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
    let valid_from = req.valid_from;
    let valid_until = req.valid_until;
    let credential_subject = Vec::new();
    let credential_schema = Vec::new();
    let proof = Vec::new();

    let credential = VerifiableCredential::new(
        context,
        credential_id.clone(),
        type_,
        issuer_id,
        valid_from,
        valid_until,
        credential_subject, // TODO
        credential_schema,  // TODO
        proof,              // TODO
    );

    info!("Generated new credential for user: {}", credential_id);
    Ok(HttpResponse::Ok().json(credential))
}

pub fn init_routes() -> Scope {
    web::scope("/credential").service(new_credential)
}
