use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use registry::Registry;

mod issuer;
pub mod registry;

// #[get("/")]
// async fn index() -> impl Responder {
//     HttpResponse::Ok().body("Hello, world!")
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let registry = Registry::new();
    HttpServer::new(|| App::new().service(issuer::init_routes()))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
