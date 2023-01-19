use std::time::{SystemTime, UNIX_EPOCH};

use actix_web::{
    post,
    web::{block, Data, Json},
    Responder,
};
use serde_json::json;

use crate::{
    database::Database,
    error::ControllerError,
    models::{BanData, BanRequestData, FullServerData},
};

#[post("/ban")]
pub async fn add_ban(
    data: Json<BanRequestData>,
    server: FullServerData,
    db: Data<Database>,
) -> Result<impl Responder, ControllerError> {
    if server.verified {
        let data = BanData {
            identifiers: data.0.identifiers,
            reason: data.0.reason,
            server: server.server,
            expires: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                + data.0.length,
        };
        match block(move || db.add_ban(data)).await.unwrap() {
            Ok(r) => match r {
                1 => Ok(Json(json!({"message": "Successfully banned player"}))),
                _ => Err(ControllerError::InsertError),
            },
            Err(e) => Err(ControllerError::DieselError(e)),
        }
    } else {
        Err(ControllerError::Unauthorized)
    }
}
