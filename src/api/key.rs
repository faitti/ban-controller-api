use actix_web::{
    get, post,
    web::{Data, Json},
    Responder,
};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};

use crate::{
    database::Database,
    error::ControllerError,
    models::{ApikeyResponse, ServerData, ServerRequest},
};

#[post("/key")]
pub async fn register_key(
    data: Json<ServerRequest>,
    db: Data<Database>,
) -> Result<impl Responder, ControllerError> {
    if data.password.len() < 8 || data.password.len() > 64 {
        return Err(ControllerError::InvalidPassword);
    }

    let hashed_password = hash_password(data.password.clone()).await.unwrap();
    let apikey = generate_apikey().await.unwrap();

    let server = ServerData {
        server: data.server.to_owned(),
        password: hashed_password,
        apikey: apikey.clone(),
    };

    match db.add_server(server).await {
        Ok(_) => return Ok(Json(ApikeyResponse { apikey })),
        Err(e) => return Err(ControllerError::DieselError(e)),
    }
}

#[get("/key")]
pub async fn request_key(
    data: Json<ServerRequest>,
    db: Data<Database>,
) -> Result<impl Responder, ControllerError> {
    let server = db.get_server(data.0.server).await.unwrap();
    if let Ok(true) = verify_password(data.0.password, server.password).await {
        return Ok(Json(ApikeyResponse {
            apikey: server.apikey,
        }));
    } else {
        return Err(ControllerError::VerifyError);
    }
}

/// Self explanatory
async fn hash_password(password: String) -> Result<String, ControllerError> {
    let salt = SaltString::generate(&mut OsRng);
    let arg2 = Argon2::default();
    match arg2.hash_password(password.as_bytes(), &salt) {
        Ok(hash) => Ok(hash.to_string()),
        Err(_) => Err(ControllerError::HashError),
    }
}

/// Self explanatory :)
async fn verify_password(
    user_password: String,
    server_password: String,
) -> Result<bool, ControllerError> {
    let parsed_hash = PasswordHash::new(&server_password).unwrap();
    Ok(Argon2::default()
        .verify_password(user_password.as_bytes(), &parsed_hash)
        .is_ok())
}

/// Generates Uuid-V4, it is used as the apikey
async fn generate_apikey() -> Result<String, ControllerError> {
    Ok(uuid::Uuid::new_v4().to_string())
}
