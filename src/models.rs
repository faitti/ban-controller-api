use crate::schema::{bans, registered_servers};
use diesel::{Insertable, Queryable, QueryableByName};
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

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq, Clone)]
pub struct Identifiers {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steam: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discord: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xbox: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fivem: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BanRequestData {
    pub identifiers: serde_json::Value,
    pub reason: String,
    pub length: u64,
}

#[derive(Serialize)]
pub struct BanResponseData {
    pub reason: String,
    pub server: String,
    pub expires: u64,
}

#[derive(Insertable, QueryableByName, Serialize)]
#[diesel(table_name = bans)]
pub struct BanData {
    pub identifiers: serde_json::Value,
    pub reason: String,
    pub server: String,
    pub expires: u64,
}

#[derive(Queryable, QueryableByName, Serialize)]
#[diesel(table_name = bans)]
pub struct FullBanData {
    pub id: u64,
    pub identifiers: serde_json::Value,
    pub reason: String,
    pub server: String,
    pub expires: u64,
}
