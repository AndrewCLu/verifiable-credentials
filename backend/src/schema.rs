use super::UserError;
use crate::registry::VerifiableDataRegistry;
use actix_web::{get, post, web, HttpResponse, Scope};
use log::{error, info};
use serde::Deserialize;
use std::sync::Mutex;
use vc_core::{CredentialSchema, URL};

#[derive(Deserialize)]
struct AddSchemaRequest {
    id: String,
    name: String,
}

#[post("/")]
async fn new_schema(
    req: web::Json<AddSchemaRequest>,
    registry: web::Data<Mutex<VerifiableDataRegistry>>,
) -> Result<HttpResponse, UserError> {
    let mut registry = registry.lock().map_err(|_e| {
        error!("Could not lock registry.");
        UserError::InternalServerError
    })?;
    let schema_id = URL::new(&req.id).map_err(|_e| {
        error!("Invalid schema id.");
        UserError::BadRequest
    })?;
    let schema = CredentialSchema::new(schema_id.clone(), req.name.clone());

    registry.new_schema(schema).map_err(|e| {
        error!("Error adding schema {} to registry: {:?}", schema_id, e);
        UserError::InternalServerError
    })?;

    info!("Added schema to registry: {}", schema_id);
    Ok(HttpResponse::Ok().json(schema_id))
}

#[get("/{id}")]
async fn get_schema(
    path: web::Path<String>,
    registry: web::Data<Mutex<VerifiableDataRegistry>>,
) -> Result<HttpResponse, UserError> {
    let registry = registry.lock().map_err(|_e| {
        error!("Could not lock registry.");
        UserError::InternalServerError
    })?;
    let schema_id = URL::new(&path.into_inner()).map_err(|_e| {
        error!("Invalid schema id.");
        UserError::BadRequest
    })?;
    let schema = registry.get_schema(&schema_id).map_err(|e| {
        error!("Error getting schema {} from registry: {:?}", schema_id, e);
        UserError::InternalServerError
    })?;

    Ok(HttpResponse::Ok().json(schema))
}

#[derive(Deserialize)]
pub struct GetAllSchemasRequest {
    limit: Option<usize>,
}

#[get("/")]
async fn get_all_schemas(
    req: web::Query<GetAllSchemasRequest>,
    registry: web::Data<Mutex<VerifiableDataRegistry>>,
) -> Result<HttpResponse, UserError> {
    let registry = registry.lock().map_err(|_e| {
        error!("Could not lock registry.");
        UserError::InternalServerError
    })?;
    let limit = req.limit;
    let schemas = registry.get_all_schemas(limit).map_err(|e| {
        error!("Error getting schemas from registry: {:?}", e);
        UserError::InternalServerError
    })?;

    Ok(HttpResponse::Ok().json(schemas))
}

pub fn init_routes() -> Scope {
    web::scope("/schema")
        .service(new_schema)
        .service(get_schema)
        .service(get_all_schemas)
}
