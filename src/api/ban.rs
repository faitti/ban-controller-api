use actix_web::{
    post,
    web::{block, Data, Json},
    Responder,
};
use serde_json::json;
use std::default::Default;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    database::Database,
    error::ControllerError,
    models::{BanData, BanRequestData, BanResponseData, FullServerData, Identifiers},
};

#[post("/ban/add")]
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

#[post("/ban/check")]
pub async fn is_banned(
    data: Json<Identifiers>,
    db: Data<Database>,
) -> Result<impl Responder, ControllerError> {
    let identifiers_json = serde_json::to_value(&data.0).unwrap();
    let db_clone = db.clone();
    if data.0 == Default::default() {
        return Err(ControllerError::EmptyStruct);
    }
    match block(move || db.get_ban(data.0)).await.unwrap() {
        Ok(ban) => {
            let new = generate_identifiers(
                &serde_json::from_value::<Identifiers>(identifiers_json.clone()).unwrap(),
                &serde_json::from_value::<Identifiers>(ban.identifiers).unwrap(),
            );
            if let Err(e) = block(move || db_clone.update_identifiers(ban.id, new))
                .await
                .unwrap()
            {
                return Err(ControllerError::DieselError(e));
            };

            Ok(Json(BanResponseData {
                reason: ban.reason,
                server: ban.server,
                expires: ban.expires,
            }))
        }
        Err(e) => Err(ControllerError::DieselError(e)),
    }
}

fn generate_identifiers(new: &Identifiers, old: &Identifiers) -> serde_json::Value {
    let mut updated: Identifiers = new.clone();
    if new.discord.is_none() && old.discord.is_some() {
        updated.discord = old.discord.clone();
    }
    if new.fivem.is_none() && old.fivem.is_some() {
        updated.fivem = old.fivem.clone();
    }
    if new.license.is_none() && old.license.is_some() {
        updated.license = old.license.clone();
    }
    if new.license2.is_none() && old.license2.is_some() {
        updated.license2 = old.license2.clone();
    }
    if new.live.is_none() && old.live.is_some() {
        updated.live = old.live.clone();
    }
    if new.xbox.is_none() && old.xbox.is_some() {
        updated.xbox = old.xbox.clone();
    }
    if new.steam.is_none() && old.steam.is_some() {
        updated.steam = old.steam.clone();
    }
    serde_json::to_value(updated).unwrap()
}
