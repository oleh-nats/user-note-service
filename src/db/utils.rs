use actix::{Actor, Addr, SyncContext};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use serde::{Deserialize, Serialize};

pub struct AppState {
    pub db: Addr<DbActor>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TokenClaims {
    pub id: i32,
}

pub struct DbActor(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for DbActor {
    type Context = SyncContext<Self>;
}

pub fn get_pool(db_url: &str) -> anyhow::Result<Pool<ConnectionManager<PgConnection>>> {
    let manager: ConnectionManager<PgConnection> = ConnectionManager::<PgConnection>::new(db_url);
    Ok(Pool::builder().build(manager)?)
}
