use super::UserError;
use crate::{AppState, DEFAULT_RESOURCE_LIMIT, VERIFIER_VERIFIER_CF_PATH};
use actix_web::{get, post, web, HttpResponse, Scope};
use log::{error, info, warn};
use rocksdb::IteratorMode;
use serde::{Deserialize, Serialize};
use vc_core::{Credential, CredentialSchema, Proof, VerifiableCredential, Verifier, URL};

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
    verifiable_credential: VerifiableCredential,
}

#[derive(Serialize)]
pub struct VerifyCredentialResponse {
    result: bool,
    reason: String,
}

#[get("/verify")]
async fn verify_credential(
    req: web::Query<VerifyCredentialRequest>,
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
    let verifiable_credential = req.verifiable_credential.clone();
    let credential = verifiable_credential.get_credential();
    let proofs = verifiable_credential.get_proof();
    if proofs.len() == 0 {
        error!("No proofs found in verifiable credential.");
        return Err(UserError::BadRequest);
    }
    let proof = proofs
        .get(0)
        .expect("Proofs should have at least one element.");

    let mut resp = VerifyCredentialResponse {
        result: true,
        reason: "".to_string(),
    };
    if !is_valid_credential_format(&credential) {
        resp.result = false;
        resp.reason = "Invalid credential format.".to_string();
    } else if !is_valid_credential_expiry(&credential) {
        resp.result = false;
        resp.reason = "Invalid credential expiry.".to_string();
    } else if !is_valid_credential_schema(&credential, &schema) {
        resp.result = false;
        resp.reason = "Invalid credential schema.".to_string();
    } else if !is_valid_verifiable_credential_proof(&credential, &proof) {
        resp.result = false;
        resp.reason = "Invalid verifiable credential proof.".to_string();
    }
    Ok(HttpResponse::Ok().json(resp))
}

fn is_valid_credential_format(cred: &Credential) -> bool {
    true
}

fn is_valid_credential_expiry(cred: &Credential) -> bool {
    true
}

fn is_valid_credential_schema(cred: &Credential, schema: &CredentialSchema) -> bool {
    true
}

fn is_valid_verifiable_credential_proof(cred: &Credential, proof: &Proof) -> bool {
    true
}

pub fn init_routes() -> Scope {
    web::scope("/verifier")
        .service(new_verifier)
        .service(get_verifier)
        .service(get_all_verifiers)
        .service(verify_credential)
}
