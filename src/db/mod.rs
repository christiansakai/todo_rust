use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

pub mod schema;
pub mod models;

type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn init_pool(database_url: &str) -> PgPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url.to_owned());

    Pool::new(manager)
        .expect(&format!("Error connecting to {}", database_url))
}
