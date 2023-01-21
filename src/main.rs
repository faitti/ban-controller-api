mod api;
mod database;
mod error;
mod middleware;
mod models;
mod schema;

use actix_web::{get, http::StatusCode, web::Data, App, HttpServer, Responder, Scope};
use database::Database;
use middleware::BearerAuth;
use std::env;

#[get("/")]
async fn root() -> impl Responder {
    ("Forbidden", StatusCode::FORBIDDEN)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    let db_pool = Data::new(Database {
        pool: Box::new(database::get_pool().await),
    });

    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .wrap(actix_cors::Cors::permissive())
            .app_data(Data::clone(&db_pool))
            .service(root)
            .service(api::key::request_key /* GET /key */)
            .service(api::key::register_key /* POST /key */)
            .service({
                let auth_scope = Scope::new("");
                auth_scope
                    .wrap(BearerAuth)
                    .service(api::key::regenerate_key /* PATCH /key */)
                    .service(api::ban::add_ban /* POST /ban */)
                    .service(api::ban::is_banned /* GET /ban */)
            })
    })
    .bind("0.0.0.0:8888")?
    .run()
    .await
}
