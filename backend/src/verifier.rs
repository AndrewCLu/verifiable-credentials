use super::UserError;
use crate::{AppState, DEFAULT_RESOURCE_LIMIT, VERIFIER_VERIFIER_CF_PATH};
use actix_web::{get, post, web, HttpResponse, Scope};
use chrono::Utc;
use log::{error, info, warn};
use rocksdb::IteratorMode;
use serde::{Deserialize, Serialize};
use vc_core::{
    proof::{CryptographicSuite, MyEcdsaSecp256k1, ProofOptions},
    ClaimProperty, ClaimPropertyValue, Credential, CredentialSchema, Proof, SchemaProperty,
    SchemaPropertyValue, SchemaPropertyValueType, VerifiableCredential, VerificationMethod,
    Verifier, URL,
};

#[derive(Deserialize)]
struct AddVerifierRequest {
    id: String,
    name: String,
    schema_id: String,
}

#[post("/")]
async fn new_verifier(
    req: web::Json<AddVerifierRequest>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, UserError> {
    let verifier_db = app_state.verifier_db.lock().map_err(|_e| {
        error!("Could not lock verifier db.");
        UserError::InternalServerError
    })?;
    let verifier_cf = verifier_db
        .cf_handle(VERIFIER_VERIFIER_CF_PATH)
        .ok_or_else(|| {
            error!("Could not get verifier cf.");
            UserError::InternalServerError
        })?;
    let verifier_id = URL::new(&req.id).map_err(|_e| {
        error!("Invalid verifier id.");
        UserError::BadRequest
    })?;
    let name = req.name.clone();
    let schema_id = URL::new(&req.schema_id).map_err(|_e| {
        error!("Invalid schema id.");
        UserError::BadRequest
    })?;
    let verifier = Verifier::new(verifier_id.clone(), name, schema_id.clone());

    let verifier_json = serde_json::to_string(&verifier).map_err(|_e| {
        error!("Could not serialize verifier.");
        UserError::InternalServerError
    })?;
    verifier_db
        .put_cf(
            verifier_cf,
            verifier.get_id().get_str().as_bytes(),
            verifier_json.as_bytes(),
        )
        .map_err(|e| {
            error!("Error adding verifier to db: {:?}", e);
            UserError::InternalServerError
        })?;

    info!("Added verifier to registry: {}", verifier_id);
    Ok(HttpResponse::Ok().json(verifier_id))
}

#[get("/{id}")]
async fn get_verifier(
    path: web::Path<String>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, UserError> {
    let verifier_db = app_state.verifier_db.lock().map_err(|_e| {
        error!("Could not lock verifier db.");
        UserError::InternalServerError
    })?;
    let verifier_cf = verifier_db
        .cf_handle(VERIFIER_VERIFIER_CF_PATH)
        .ok_or_else(|| {
            error!("Could not get verifier cf.");
            UserError::InternalServerError
        })?;
    let verifier_id = URL::new(&path.into_inner()).map_err(|_e| {
        error!("Invalid verifier id.");
        UserError::BadRequest
    })?;
    let verifier_bytes = verifier_db
        .get_cf(verifier_cf, verifier_id.get_str().as_bytes())
        .map_err(|e| {
            error!("Error getting verifier from db: {:?}", e);
            UserError::InternalServerError
        })?
        .ok_or_else(|| {
            error!("Could not find verifier {} in db.", verifier_id);
            UserError::BadRequest
        })?;
    let verifier = String::from_utf8(verifier_bytes)
        .map_err(|_e| {
            error!("Could not deserialize verifier.");
            UserError::InternalServerError
        })
        .and_then(|verifier_json| {
            serde_json::from_str::<Verifier>(verifier_json.as_str()).map_err(|_e| {
                error!("Could not deserialize verifier.");
                UserError::InternalServerError
            })
        })?;

    Ok(HttpResponse::Ok().json(verifier))
}

#[derive(Deserialize)]
pub struct GetAllVerifiersRequest {
    limit: Option<usize>,
}

#[get("/")]
async fn get_all_verifiers(
    req: web::Query<GetAllVerifiersRequest>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, UserError> {
    let verifier_db = app_state.verifier_db.lock().map_err(|_e| {
        error!("Could not lock verifier db.");
        UserError::InternalServerError
    })?;
    let verifier_cf = verifier_db
        .cf_handle(VERIFIER_VERIFIER_CF_PATH)
        .ok_or_else(|| {
            error!("Could not get verifier cf.");
            UserError::InternalServerError
        })?;
    let limit = req.limit.unwrap_or(DEFAULT_RESOURCE_LIMIT);
    let verifier_iter = verifier_db.iterator_cf(verifier_cf, IteratorMode::Start);
    let verifiers: Vec<Verifier> = verifier_iter
        .filter_map(|result| {
            if let Ok((_key, value)) = result {
                String::from_utf8(value.to_vec())
                    .map_err(|_| {
                        warn!("Could not deserialize an verifier from bytes to string.");
                    })
                    .and_then(|verifier_json| {
                        serde_json::from_str::<Verifier>(&verifier_json).map_err(|_| {
                            warn!("Could not deserialize an verifier from json string.");
                        })
                    })
                    .ok()
            } else {
                warn!("Could not fetch an verifier.");
                None
            }
        })
        .take(limit)
        .collect();

    Ok(HttpResponse::Ok().json(verifiers))
}

#[derive(Deserialize)]
pub struct VerifyCredentialRequest {
    verifier_id: String,
    verifiable_credential: String,
}

#[derive(Serialize, Deserialize)]
pub struct VerifyCredentialResponse {
    verified: bool,
    reason: String,
}

#[post("/verify")]
async fn verify_credential(
    req: web::Json<VerifyCredentialRequest>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, UserError> {
    let registry = app_state.registry.lock().map_err(|_e| {
        error!("Could not lock registry.");
        UserError::InternalServerError
    })?;
    let verifier_db = app_state.verifier_db.lock().map_err(|_e| {
        error!("Could not lock verifier db.");
        UserError::InternalServerError
    })?;
    let verifier_cf = verifier_db
        .cf_handle(VERIFIER_VERIFIER_CF_PATH)
        .ok_or_else(|| {
            error!("Could not get verifier cf.");
            UserError::InternalServerError
        })?;
    let verifier_id = URL::new(&req.verifier_id).map_err(|_e| {
        error!("Invalid verifier id.");
        UserError::BadRequest
    })?;
    let verifier_bytes = verifier_db
        .get_cf(verifier_cf, verifier_id.get_str().as_bytes())
        .map_err(|e| {
            error!("Error getting verifier from db: {:?}", e);
            UserError::InternalServerError
        })?
        .ok_or_else(|| {
            error!("Could not find verifier {} in db.", verifier_id);
            UserError::BadRequest
        })?;
    let verifier = String::from_utf8(verifier_bytes)
        .map_err(|_e| {
            error!("Could not deserialize verifier.");
            UserError::InternalServerError
        })
        .and_then(|verifier_json| {
            serde_json::from_str::<Verifier>(verifier_json.as_str()).map_err(|_e| {
                error!("Could not deserialize verifier.");
                UserError::InternalServerError
            })
        })?;
    let schema_id = verifier.get_schema_id();
    let schema = registry
        .get_schema(&schema_id)
        .map_err(|e| {
            error!("Could not get schema from registry: {:?}", e);
            UserError::BadRequest
        })?
        .ok_or_else(|| {
            error!("Could not find schema {} in registry.", schema_id);
            UserError::BadRequest
        })?;

    let verifiable_credential_string = req.verifiable_credential.clone();
    let verifiable_credential = serde_json::from_str::<VerifiableCredential>(
        &verifiable_credential_string,
    )
    .map_err(|_| {
        error!("Could not deserialize verifiable credential.");
        UserError::BadRequest
    })?;
    let credential = verifiable_credential.get_credential();
    let proofs = verifiable_credential.get_proof();
    if proofs.len() == 0 {
        error!("No proofs found in verifiable credential.");
        return Err(UserError::BadRequest);
    }
    let proof = proofs
        .get(0)
        .expect("Proofs should have at least one element.");

    let issuer_id = credential.get_issuer();
    let issuer = registry
        .get_issuer(issuer_id)
        .map_err(|e| {
            error!("Could not get issuer from registry: {:?}", e);
            UserError::BadRequest
        })?
        .ok_or_else(|| {
            error!("Could not find issuer {} in registry.", issuer_id);
            UserError::BadRequest
        })?;
    let verification_methods = issuer.get_verification_methods();
    if verification_methods.len() == 0 {
        error!("No verification methods found for issuer.");
        return Err(UserError::BadRequest);
    }
    let verification_method = verification_methods
        .get(0)
        .expect("Verification methods should have at least one element.");
    let verifying_key = verification_method.get_public_key_multibase();

    let mut resp = VerifyCredentialResponse {
        verified: true,
        reason: "".to_string(),
    };
    if !is_valid_credential_format(&credential) {
        resp.verified = false;
        resp.reason = "Invalid credential format.".to_string();
    } else if !is_valid_credential_expiry(&credential) {
        resp.verified = false;
        resp.reason = "Invalid credential expiry.".to_string();
    } else if !is_valid_credential_schema(&credential, &schema) {
        resp.verified = false;
        resp.reason = "Invalid credential schema.".to_string();
    } else if !is_valid_verifiable_credential_proof(
        &credential,
        &proof,
        verifying_key,
        verification_method.clone(),
    ) {
        resp.verified = false;
        resp.reason = "Invalid verifiable credential proof.".to_string();
    }
    Ok(HttpResponse::Ok().json(resp))
}

fn is_valid_credential_format(cred: &Credential) -> bool {
    let context = cred.get_context();
    for ctx in context.iter() {
        if ctx.get_str() == "https://www.w3.org/ns/credentials/v2" {
            return true;
        }
    }

    false
}

fn is_valid_credential_expiry(cred: &Credential) -> bool {
    let current_time = Utc::now();
    let after_valid_from = current_time >= *cred.get_valid_from();
    let before_valid_until = current_time <= *cred.get_valid_until();

    after_valid_from && before_valid_until
}

fn is_valid_credential_schema(cred: &Credential, schema: &CredentialSchema) -> bool {
    let claims = cred.get_credential_subject();
    let schema_properties = schema.get_properties();
    for (key, schema_prop) in schema_properties {
        if let Some(claim_prop) = claims.get(key) {
            if !is_valid_credential_schema_property(claim_prop, schema_prop) {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

fn is_valid_credential_schema_property(
    claim_property: &ClaimProperty,
    schema_property: &SchemaProperty,
) -> bool {
    match (claim_property, schema_property) {
        (ClaimProperty::Value(claim_value), SchemaProperty::Value(schema_value)) => {
            is_valid_credential_schema_property_value(claim_value, schema_value)
        }
        (ClaimProperty::Array(claim_array), SchemaProperty::Array(schema_array)) => {
            if claim_array.len() != schema_array.len() {
                return false;
            }
            for (claim_prop, schema_prop) in claim_array.iter().zip(schema_array.iter()) {
                if !is_valid_credential_schema_property(claim_prop, schema_prop) {
                    return false;
                }
            }
            true
        }
        (ClaimProperty::Map(claim_map), SchemaProperty::Map(schema_map)) => {
            for (key, schema_prop) in schema_map {
                if let Some(claim_prop) = claim_map.get(key) {
                    if !is_valid_credential_schema_property(claim_prop, schema_prop) {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            true
        }
        _ => false,
    }
}

fn is_valid_credential_schema_property_value(
    claim_value: &ClaimPropertyValue,
    schema_value: &SchemaPropertyValue,
) -> bool {
    match (claim_value, schema_value.get_type()) {
        (ClaimPropertyValue::Text(_), SchemaPropertyValueType::Text) => true,
        (ClaimPropertyValue::Number(_), SchemaPropertyValueType::Number) => true,
        (ClaimPropertyValue::Boolean(_), SchemaPropertyValueType::Boolean) => true,
        _ => false,
    }
}

fn is_valid_verifiable_credential_proof(
    cred: &Credential,
    proof: &Proof,
    verifying_key: &[u8],
    verification_method: VerificationMethod,
) -> bool {
    let cryptographic_suite = MyEcdsaSecp256k1::new();
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
    match cryptographic_suite.verify_proof(cred, proof, verifying_key, &proof_options) {
        Ok(is_valid_proof) => is_valid_proof,
        Err(e) => {
            error!("Error verifying proof: {:?}", e);
            false
        }
    }
}

pub fn init_routes() -> Scope {
    web::scope("/verifier")
        .service(new_verifier)
        .service(get_verifier)
        .service(get_all_verifiers)
        .service(verify_credential)
}
