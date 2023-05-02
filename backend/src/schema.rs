use super::UserError;
use crate::AppState;
use actix_web::{get, post, web, HttpResponse, Scope};
use log::{error, info};
use serde::Deserialize;
use std::collections::HashMap;
use vc_core::{
    CredentialSchema, SchemaProperty, SchemaPropertyValue, SchemaPropertyValueType, URL,
};

#[derive(Deserialize)]
struct AddSchemaRequest {
    id: String,
    name: String,
}

#[post("/")]
async fn new_schema(
    req: web::Json<AddSchemaRequest>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, UserError> {
    let mut registry = app_state.registry.lock().map_err(|_e| {
        error!("Could not lock registry.");
        UserError::InternalServerError
    })?;
    let schema_id = URL::new(&req.id).map_err(|_e| {
        error!("Invalid schema id.");
        UserError::BadRequest
    })?;

    let one = SchemaPropertyValue::new(SchemaPropertyValueType::Text, "leaf 1 desc".to_string());
    let two = SchemaPropertyValue::new(SchemaPropertyValueType::Number, "leaf 2 desc".to_string());
    let three =
        SchemaPropertyValue::new(SchemaPropertyValueType::Boolean, "leaf 3 desc".to_string());
    let four = SchemaPropertyValue::new(SchemaPropertyValueType::Text, "leaf 4 desc".to_string());
    let five = SchemaPropertyValue::new(SchemaPropertyValueType::Text, "leaf 5 desc".to_string());
    let mut schema_props = HashMap::<String, SchemaProperty>::new();
    schema_props.insert("one".to_string(), SchemaProperty::Value(one));
    let mut three_map = HashMap::<String, SchemaProperty>::new();
    three_map.insert("three".to_string(), SchemaProperty::Value(three));
    schema_props.insert(
        "two".to_string(),
        SchemaProperty::Array(vec![
            SchemaProperty::Value(two),
            SchemaProperty::Map(three_map),
        ]),
    );
    let mut four_five_map = HashMap::<String, SchemaProperty>::new();
    four_five_map.insert("four".to_string(), SchemaProperty::Value(four));
    four_five_map.insert(
        "five".to_string(),
        SchemaProperty::Array(vec![SchemaProperty::Value(five)]),
    );
    schema_props.insert("four".to_string(), SchemaProperty::Map(four_five_map));
    let schema = CredentialSchema::new(
        schema_id.clone(),
        "type".to_string(),
        req.name.clone(),
        "desc".to_string(),
        schema_props,
    );

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
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, UserError> {
    let registry = app_state.registry.lock().map_err(|_e| {
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
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, UserError> {
    let registry = app_state.registry.lock().map_err(|_e| {
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
