use crate::schema::registered_servers;
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
}
