use crate::schema::{bans, registered_servers};
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ApikeyResponse {
    pub apikey: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerRequest {
    pub server: String,
    pub password: String,
}

#[derive(Serialize, Insertable)]
#[diesel(table_name = registered_servers)]
pub struct ServerData {
    pub server: String,
    pub password: String,
    pub apikey: String,
}

#[derive(Queryable, Debug, Clone, PartialEq, Eq)]
#[diesel(table_name = registerd_servers)]
pub struct FullServerData {
    pub id: u64,
    pub server: String,
    pub password: String,
    pub apikey: String,
    pub verified: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Identifiers {
    pub steam: Option<String>,
    pub rockstar: Option<String>,
    pub discord: Option<String>,
    pub xbox: Option<String>,
    pub live: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BanRequestData {
    pub identifiers: serde_json::Value,
    pub reason: String,
    pub length: u64,
}

#[derive(Insertable)]
#[diesel(table_name = bans)]
pub struct BanData {
    pub identifiers: serde_json::Value,
    pub reason: String,
    pub server: String,
    pub expires: u64,
}
