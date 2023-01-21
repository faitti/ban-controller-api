use std::future::Future;
use std::pin::Pin;

use actix_web::{
    get, patch, post,
    web::{block, Data, Json},
    FromRequest, HttpMessage, Responder,
};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};

use crate::{
    database::Database,
    error::ControllerError,
    middleware::AuthStatus,
    models::{ApikeyResponse, FullServerData, ServerData, ServerRequest},
};

impl FromRequest for FullServerData {
    type Error = ControllerError;
    type Future = Pin<Box<dyn Future<Output = actix_web::Result<Self, Self::Error>>>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let token = req.extensions().get::<AuthStatus>().cloned();
        Box::pin(async move {
            if let Some(AuthStatus::Authorized(data)) = token {
                Ok(data)
            } else {
                Err(ControllerError::Unauthorized)
            }
        })
    }
}

#[post("/register")]
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

    match block(move || db.add_server(server)).await.unwrap() {
        Ok(_) => Ok(Json(ApikeyResponse { apikey })),
        Err(e) => Err(ControllerError::DieselError(e)),
    }
}

#[post("/login")]
pub async fn request_key(
    data: Json<ServerRequest>,
    db: Data<Database>,
) -> Result<impl Responder, ControllerError> {
    let server = block(move || db.get_server_with_name(data.0.server).unwrap())
        .await
        .unwrap();
    if let Ok(true) = verify_password(data.0.password, server.password).await {
        Ok(Json(ApikeyResponse {
            apikey: server.apikey,
        }))
    } else {
        Err(ControllerError::VerifyError)
    }
}

#[patch("/key")]
pub async fn regenerate_key(
    data: FullServerData,
    db: Data<Database>,
) -> Result<impl Responder, ControllerError> {
    let new_key = generate_apikey().await;
    if let Ok(key) = new_key {
        let resp_key = key.clone();
        block(move || db.update_apikey(data.server, key).unwrap())
            .await
            .unwrap();
        return Ok(Json(ApikeyResponse { apikey: resp_key }));
    }
    Err(ControllerError::GenerationFailure)
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
