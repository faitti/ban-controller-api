use std::env;

use diesel::{
    r2d2::{self, ConnectionManager, Pool, PooledConnection},
    MysqlConnection,
};
use dotenv::dotenv;

type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;
type MysqlPooled = PooledConnection<ConnectionManager<MysqlConnection>>;

pub struct Database {
    pub pool: Box<MysqlPool>,
}

impl Database {
    pub fn get(&self) -> MysqlPooled {
        self.pool.get().unwrap()
    }
}

pub async fn get_pool() -> MysqlPool {
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL env variable doesn't exist");
    let manager = ConnectionManager::<MysqlConnection>::new(url);

    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to build connection pool")
}
