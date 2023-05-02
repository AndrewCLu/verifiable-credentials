use super::UserError;
use crate::{AppState, ISSUER_SIGNING_KEY_CF_PATH};
use actix_web::{get, post, web, HttpResponse, Responder, Scope};
use k256::ecdsa::{SigningKey, VerifyingKey};
use log::{error, info};
use rand_core::OsRng;
use serde::Deserialize;
use vc_core::{Issuer, VerificationMethod, URL};

#[derive(Deserialize)]
struct AddIssuerRequest {
    id: String,
    name: String,
}

#[post("/")]
async fn new_issuer(
    req: web::Json<AddIssuerRequest>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, UserError> {
    let mut registry = app_state.registry.lock().map_err(|_e| {
        error!("Could not lock registry.");
        UserError::InternalServerError
    })?;
    let issuer_id = URL::new(&req.id).map_err(|_e| {
        error!("Invalid issuer id.");
        UserError::BadRequest
    })?;
    let issuer = Issuer::new(issuer_id.clone(), req.name.clone());

    registry.new_issuer(issuer).map_err(|e| {
        error!("Error adding issuer {} to registry: {:?}", issuer_id, e);
        UserError::InternalServerError
    })?;

    info!("Added issuer to registry: {}", issuer_id);
    Ok(HttpResponse::Ok().json(issuer_id))
}

#[get("/{id}")]
async fn get_issuer(
    path: web::Path<String>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, UserError> {
    let registry = app_state.registry.lock().map_err(|_e| {
        error!("Could not lock registry.");
        UserError::InternalServerError
    })?;
    let issuer_id = URL::new(&path.into_inner()).map_err(|_e| {
        error!("Invalid issuer id.");
        UserError::BadRequest
    })?;
    let issuer = registry.get_issuer(&issuer_id).map_err(|e| {
        error!("Error getting issuer {} from registry: {:?}", issuer_id, e);
        UserError::InternalServerError
    })?;

    Ok(HttpResponse::Ok().json(issuer))
}

#[derive(Deserialize)]
pub struct GetAllIssuersRequest {
    limit: Option<usize>,
}

#[get("/")]
async fn get_all_issuers(
    req: web::Query<GetAllIssuersRequest>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, UserError> {
    let registry = app_state.registry.lock().map_err(|_e| {
        error!("Could not lock registry.");
        UserError::InternalServerError
    })?;
    let limit = req.limit;
    let issuers = registry.get_all_issuers(limit).map_err(|e| {
        error!("Error getting issuers from registry: {:?}", e);
        UserError::InternalServerError
    })?;

    Ok(HttpResponse::Ok().json(issuers))
}

#[derive(Deserialize)]
struct AddVerificationMethodRequest {
    verification_method_id: String,
    type_: String,
}

#[post("/{id}/verification_method")]
async fn new_verification_method(
    req: web::Json<AddVerificationMethodRequest>,
    path: web::Path<String>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, UserError> {
    let mut registry = app_state.registry.lock().map_err(|_e| {
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
    let issuer_id = URL::new(&path.into_inner()).map_err(|_e| {
        error!("Invalid issuer id.");
        UserError::BadRequest
    })?;
    let verification_method_id = URL::new(&req.verification_method_id).map_err(|_e| {
        error!("Invalid verification method id.");
        UserError::BadRequest
    })?;

    let signing_key = SigningKey::random(&mut OsRng);
    let verifying_key = VerifyingKey::from(&signing_key).to_sec1_bytes();
    issuer_db
        .put_cf(
            signing_key_cf,
            verification_method_id.get_str().as_bytes(),
            signing_key.to_bytes(),
        )
        .map_err(|e| {
            error!("Error adding signing key to db: {:?}", e);
            UserError::InternalServerError
        })?;

    let verification_method = VerificationMethod::new(
        verification_method_id.clone(),
        req.type_.clone(),
        issuer_id.clone(),
        verifying_key.to_vec(),
    );

    registry
        .new_verification_method(&issuer_id, verification_method)
        .map_err(|e| {
            error!(
                "Error adding verification method {} to registry: {:?}",
                verification_method_id, e
            );
            UserError::InternalServerError
        })?;

    Ok(HttpResponse::Ok().json(verification_method_id))
}

#[get("/issue_credential")]
async fn issue_credential() -> impl Responder {
    HttpResponse::Ok().body("Added a credential.")
}

#[get("/revoke_credential")]
async fn revoke_credential() -> impl Responder {
    HttpResponse::Ok().body("Revoked a credential.")
}

pub fn init_routes() -> Scope {
    web::scope("/issuer")
        .service(new_issuer)
        .service(get_issuer)
        .service(get_all_issuers)
        .service(new_verification_method)
        .service(issue_credential)
        .service(revoke_credential)
}
