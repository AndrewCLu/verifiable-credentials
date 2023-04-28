use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, ResponseError};
use registry::VerifiableDataRegistry;
use std::fmt;
use std::sync::Mutex;

mod credential;
mod issuer;
mod registry;
mod schema;

#[derive(Debug)]
pub enum UserError {
    BadRequest,
    NotFound,
    InternalServerError,
}

impl ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            UserError::BadRequest => HttpResponse::BadRequest().body("Bad Request"),
            UserError::NotFound => HttpResponse::NotFound().body("Resource Not Found"),
            UserError::InternalServerError => {
                HttpResponse::InternalServerError().body("Internal Server Error")
            }
        }
    }
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

async fn not_found(_req: HttpRequest) -> HttpResponse {
    UserError::NotFound.error_response()
}

#[get("/")]
async fn hello_world() -> HttpResponse {
    HttpResponse::Ok().body("Hello, world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "backend=debug,actix_web=debug,vc_core=debug");
    env_logger::init();
    let registry = VerifiableDataRegistry::new("verifiable_data_registry")
        .expect("Could not create registry.");
    let data = web::Data::new(Mutex::new(registry));

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(data.clone())
            .service(hello_world)
            .service(issuer::init_routes())
            .service(schema::init_routes())
            .service(credential::init_routes())
            .default_service(web::to(not_found))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
