use super::UserError;
use crate::AppState;
use crate::ISSUER_SIGNING_KEY_CF_PATH;
use actix_web::{post, web, HttpResponse, Scope};
use chrono::{DateTime, Utc};
use log::{error, info};
use serde::Deserialize;
use std::collections::HashMap;
use vc_core::proof::{CryptographicSuite, MyEcdsaSecp256k1, ProofOptions};
use vc_core::{ClaimProperty, Credential, VerifiableCredential, URL};

#[derive(Deserialize)]
struct NewCredentialRequest {
    context: Vec<String>,
    credential_id: String,
    type_: Vec<String>,
    issuer_id: String,
    valid_from: String,  // Expects a RFC3339 formatted DateTime string
    valid_until: String, // Expects a RFC3339 formatted DateTime string
    credential_subject: HashMap<String, ClaimProperty>,
    credential_schema_ids: Vec<String>,
}

#[post("/")]
async fn new_credential(
    req: web::Json<NewCredentialRequest>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, UserError> {
    let registry = app_state.registry.lock().map_err(|_e| {
        error!("Could not lock registry.");
        UserError::InternalServerError
    })?;
    let issuer_db = app_state.issuer_db.lock().map_err(|_e| {
        error!("Could not lock issuer db.");
        UserError::InternalServerError
    })?;
    let signing_key_cf = issuer_db
        .cf_handle(ISSUER_SIGNING_KEY_CF_PATH)
        .ok_or_else(|| {
            error!("Could not get issuer signing key cf.");
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
        .ok_or_else(|| {
            error!("Could not find issuer {} in registry.", issuer_id);
            UserError::BadRequest
        })?;
    let valid_from = DateTime::parse_from_rfc3339(&req.valid_from)
        .map_err(|_e| {
            error!("Invalid valid_from date.");
            UserError::BadRequest
        })?
        .with_timezone(&Utc);
    let valid_until = DateTime::parse_from_rfc3339(&req.valid_until)
        .map_err(|_e| {
            error!("Invalid valid_until date.");
            UserError::BadRequest
        })?
        .with_timezone(&Utc);
    let credential_subject = req.credential_subject.clone();
    let mut credential_schema = Vec::new();
    for credential_schema_id in &req.credential_schema_ids {
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
            .ok_or_else(|| {
                error!(
                    "Could not find credential schema {} in registry.",
                    credential_schema_id
                );
                UserError::BadRequest
            })?;
        credential_schema.push(schema.get_link());
    }
    let credential = Credential::new(
        context,
        credential_id.clone(),
        type_,
        issuer_id.clone(),
        valid_from,
        valid_until,
        credential_subject,
        credential_schema,
    );
    let cryptographic_suite = MyEcdsaSecp256k1::new();
    let issuer_verification_methods = issuer.get_verification_methods();
    if issuer_verification_methods.is_empty() {
        error!("Issuer {} has no verification methods.", issuer_id);
        return Err(UserError::InternalServerError);
    }
    let verification_method = issuer_verification_methods[0].clone();
    let issuer_signing_key = issuer_db
        .get_cf(
            &signing_key_cf,
            verification_method.get_id().get_str().as_bytes(),
        )
        .map_err(|e| {
            error!(
                "Error getting signing key for issuer {} from db: {:?}",
                issuer_id, e
            );
            UserError::InternalServerError
        })?
        .ok_or_else(|| {
            error!("Could not find signing key for issuer {} in db.", issuer_id);
            UserError::BadRequest
        })?;
    let proof_purpose = "Proof Purpose".to_string();
    let created = Utc::now();
    let domain = "Proof Domain".to_string();
    let challenge = "Proof Challenge".to_string();
    let proof_options = ProofOptions::new(
        verification_method,
        proof_purpose,
        created,
        domain,
        challenge,
    );
    let proof = cryptographic_suite
        .generate_proof(&credential, &issuer_signing_key, &proof_options)
        .map_err(|e| {
            error!("Error generating proof for verifiable credential: {:?}", e);
            UserError::InternalServerError
        })?;
    let verifiable_credential = VerifiableCredential::new(credential, vec![proof]);

    info!("Generated new credential for user: {}", credential_id);
    Ok(HttpResponse::Ok().json(verifiable_credential))
}

pub fn init_routes() -> Scope {
    web::scope("/credential").service(new_credential)
}
