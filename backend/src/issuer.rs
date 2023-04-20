use actix_web::{get, post, web, HttpResponse, Responder, Scope};
use serde::Deserialize;
use vc_core::{Issuer, URL};

#[derive(Deserialize)]
struct CreateIssuerRequest {
    id: String,
    name: String,
}

#[post("/create")]
async fn create(request: web::Json<CreateIssuerRequest>) -> HttpResponse {
    let issuer_id = URL::new(&request.id).expect("Invalid issuer ID.");
    let issuer = Issuer::new(issuer_id, request.name.clone());

    HttpResponse::Ok().json(issuer.get_id())
}

#[get("/add_verification")]
async fn add_verification() -> impl Responder {
    HttpResponse::Ok().body("Added a verification method.")
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
