use actix_web::{
    post,
    web::{Data, Json},
    Responder,
};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};

use crate::{
    database::Database,
    error::ControllerError,
    models::{ServerData, ServerRequest},
};

#[post("/key")]
pub async fn register_key(
    data: Json<ServerRequest>,
    db: Data<Database>,
) -> Result<impl Responder, ControllerError> {
    if data.password.len() < 8 || data.password.len() > 64 {
        return Err(ControllerError::InvalidPassword);
    }

    let _connection = db.get();

    let hashed_password = hash_password(data.password.clone()).await.unwrap();
    let apikey = generate_apikey().await.unwrap();

    let server = ServerData {
        server: data.server.to_owned(),
        password: hashed_password,
        apikey,
    };

    Ok(Json(server))
}

async fn hash_password(password: String) -> Result<String, ControllerError> {
    let salt = SaltString::generate(&mut OsRng);
    let arg2 = Argon2::default();
    match arg2.hash_password(password.as_bytes(), &salt) {
        Ok(hash) => Ok(hash.to_string()),
        Err(_) => Err(ControllerError::HashError),
    }
}

async fn generate_apikey() -> Result<String, ControllerError> {
    Ok(uuid::Uuid::new_v4().to_string())
}
