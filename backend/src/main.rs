use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, ResponseError};
use registry::VerifiableDataRegistry;
use rocksdb::{ColumnFamilyDescriptor, Options, DB};
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

pub struct AppState {
    pub registry: Mutex<VerifiableDataRegistry>,
    pub issuer_db: Mutex<DB>,
}

pub const VERIFIABLE_DATA_REGISTRY_DB_PATH: &'static str = "verifiable_data_registry";
pub const ISSUER_DB_PATH: &'static str = "issuer";
pub const ISSUER_SIGNING_KEY_CF_PATH: &'static str = "signing_key";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "backend=debug,actix_web=debug,vc_core=debug");
    env_logger::init();
    let registry = VerifiableDataRegistry::new(VERIFIABLE_DATA_REGISTRY_DB_PATH)
        .expect("Could not create registry.");
    let mut issuer_db_options = Options::default();
    issuer_db_options.create_if_missing(true);
    issuer_db_options.create_missing_column_families(true);
    let signing_key_cf =
        ColumnFamilyDescriptor::new(ISSUER_SIGNING_KEY_CF_PATH, Options::default());
    let issuer_db =
        DB::open_cf_descriptors(&issuer_db_options, ISSUER_DB_PATH, vec![signing_key_cf])
            .expect("Could not open issuer db.");

    let app_state = AppState {
        registry: Mutex::new(registry),
        issuer_db: Mutex::new(issuer_db),
    };
    let app_data = web::Data::new(app_state);

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(app_data.clone())
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
