use actix_web::{get, http::StatusCode, web, App, HttpServer, Responder};

#[get("/")]
async fn root() -> impl Responder {
    StatusCode::FORBIDDEN
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(root)
    })
    .bind(("127.0.0.1", "8080"))?
    .run()
    .await
}
