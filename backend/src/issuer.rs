use super::UserError;
use actix_web::{get, post, web, HttpResponse, Responder, Scope};
use serde::Deserialize;
use std::sync::Mutex;
use vc_core::{Issuer, VerificationMethod, URL};

use crate::registry::VerifiableDataRegistry;

#[derive(Deserialize)]
struct CreateIssuerRequest {
    id: String,
    name: String,
}

#[post("/create")]
async fn create(
    request: web::Json<CreateIssuerRequest>,
    registry: web::Data<Mutex<VerifiableDataRegistry>>,
) -> Result<HttpResponse, UserError> {
    let mut registry = registry
        .lock()
        .map_err(|_e| UserError::InternalServerError)?;
    let issuer_id = URL::new(&request.id).map_err(|_e| UserError::BadRequest)?;
    let issuer = Issuer::new(issuer_id.clone(), request.name.clone());

    // TODO: Process distinct possible errors here accordingly
    registry
        .add_issuer(issuer)
        .map_err(|_e| UserError::InternalServerError)?;

    Ok(HttpResponse::Ok().json(issuer_id))
}

#[derive(Deserialize)]
struct AddVerificationRequest {
    issuer_id: String,
    verification_method_id: String,
    type_: String,
    public_key_multibase: String,
}

#[get("/add_verification")]
async fn add_verification(
    request: web::Json<AddVerificationRequest>,
    registry: web::Data<Mutex<VerifiableDataRegistry>>,
) -> Result<HttpResponse, UserError> {
    let mut registry = registry
        .lock()
        .map_err(|_e| UserError::InternalServerError)?;
    let issuer_id = URL::new(&request.issuer_id).map_err(|_e| UserError::BadRequest)?;
    let verification_method_id =
        URL::new(&request.verification_method_id).map_err(|_e| UserError::BadRequest)?;
    let verification_method = VerificationMethod::new(
        verification_method_id.clone(),
        request.type_.clone(),
        issuer_id.clone(),
        request.public_key_multibase.clone(),
    );

    // TODO: Process distinct possible errors here accordingly
    registry
        .add_verification_method(&issuer_id, verification_method)
        .map_err(|_e| UserError::InternalServerError)?;

    Ok(HttpResponse::Ok().json(verification_method_id))
}

#[get("/add_schema")]
async fn add_schema() -> impl Responder {
    HttpResponse::Ok().body("Added a schema.")
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
        .service(create)
        .service(add_verification)
        .service(add_schema)
        .service(issue_credential)
        .service(revoke_credential)
}
