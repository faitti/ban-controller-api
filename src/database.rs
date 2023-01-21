use std::env;

use diesel::sql_types::Text;
use diesel::{prelude::*, sql_query};
use diesel::{
    r2d2::{self, ConnectionManager, Pool, PooledConnection},
    MysqlConnection, QueryDsl, RunQueryDsl,
};
use dotenv::dotenv;

use crate::models::{BanData, FullBanData, FullServerData, Identifiers, ServerData};

type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;
type MysqlPooled = PooledConnection<ConnectionManager<MysqlConnection>>;

pub struct Database {
    pub pool: Box<MysqlPool>,
}

pub async fn get_pool() -> MysqlPool {
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL env variable doesn't exist");
    let manager = ConnectionManager::<MysqlConnection>::new(url);

    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to build connection pool")
}

impl Database {
    pub fn get(&self) -> MysqlPooled {
        self.pool.get().unwrap()
    }

    pub fn add_server(&self, server: ServerData) -> Result<usize, diesel::result::Error> {
        use crate::schema::registered_servers;
        let mut connection = self.get();
        diesel::insert_into(registered_servers::table)
            .values(&server)
            .execute(&mut connection)
    }

    pub fn get_server_with_name(
        &self,
        name: String,
    ) -> Result<FullServerData, diesel::result::Error> {
        use crate::schema::registered_servers::dsl::*;
        let mut connection = self.get();
        registered_servers
            .filter(server.eq(name))
            .first::<FullServerData>(&mut connection)
    }

    pub fn get_server_with_apikey(
        &self,
        req_apikey: String,
    ) -> Result<FullServerData, diesel::result::Error> {
        use crate::schema::registered_servers::dsl::*;
        let mut connection = self.get();
        registered_servers
            .filter(apikey.eq(req_apikey))
            .first::<FullServerData>(&mut connection)
    }

    pub fn update_apikey(
        &self,
        server_name: String,
        new_key: String,
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::registered_servers::dsl::*;
        let mut connection = self.get();
        diesel::update(registered_servers)
            .filter(server.eq(server_name))
            .set(apikey.eq(new_key))
            .execute(&mut connection)
    }

    pub fn add_ban(&self, data: BanData) -> Result<usize, diesel::result::Error> {
        use crate::schema::bans;
        let mut connection = self.get();
        diesel::insert_into(bans::table)
            .values(&data)
            .execute(&mut connection)
    }

    pub fn get_ban(&self, data: Identifiers) -> Result<FullBanData, diesel::result::Error> {
        let mut connection = self.get();
        let query = sql_query("SELECT * FROM `bans` WHERE JSON_OVERLAPS(identifiers, ?);");
        query
            .bind::<Text, _>(serde_json::to_value(data).unwrap().to_string())
            .get_result::<FullBanData>(&mut connection)
    }

    pub fn update_identifiers(
        &self,
        ban_id: u64,
        data: serde_json::Value,
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::bans::dsl::*;
        let mut connection = self.get();
        diesel::update(bans)
            .filter(id.eq(ban_id))
            .set(identifiers.eq(data))
            .execute(&mut connection)
    }

    pub fn get_bans(&self) -> Result<Vec<FullBanData>, diesel::result::Error> {
        use crate::schema::bans::dsl::*;
        let mut connection = self.get();
        bans.get_results(&mut connection)
    }
}
