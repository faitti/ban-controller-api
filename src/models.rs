use crate::schema::registered_servers;
use diesel::Insertable;
use serde::{Deserialize, Serialize};

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
