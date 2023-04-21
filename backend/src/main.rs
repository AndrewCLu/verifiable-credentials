use actix_web::{web, App, HttpServer};
use registry::VerifiableDataRegistry;
use rocksdb::DB;
use std::sync::Mutex;

mod issuer;
pub mod registry;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_path = "verifiable_data_registry";
    let db = DB::open_default(db_path).expect("Could not open database.");
    let registry = web::Data::new(Mutex::new(VerifiableDataRegistry::new(db)));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(registry.clone()))
            .service(issuer::init_routes())
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
