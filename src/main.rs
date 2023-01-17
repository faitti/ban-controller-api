mod api;
mod database;
mod error;
mod models;
mod schema;

use actix_web::{get, http::StatusCode, web::Data, App, HttpServer, Responder};
use database::Database;

#[get("/")]
async fn root() -> impl Responder {
    ("Forbidden", StatusCode::FORBIDDEN)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_pool = Data::new(Database {
        pool: Box::new(database::get_pool().await),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&db_pool))
            .service(root)
            .service(api::key::request_key /* GET /key */)
            .service(api::key::register_key /* POST /key */)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
