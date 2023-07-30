use crate::config::CommandLine;
use actix::SyncArbiter;
use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use actix_web_httpauth::middleware::HttpAuthentication;
use clap::Parser;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

mod config;
mod db;
mod error;
mod routes;
mod validator;

use db::utils::{get_pool, AppState, DbActor};
use validator::validator_utils::handle_validate;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let cmd: CommandLine = CommandLine::parse();
    let properties = cmd.load_configurations()?;

    let pool: Pool<ConnectionManager<PgConnection>> = get_pool(&properties.postgres.database_url)?;
    let db_addr = SyncArbiter::start(5, move || DbActor(pool.clone()));

    HttpServer::new(move || {
        let jwt_secret = properties.jwt_secret.clone();
        let bearer_middleware = HttpAuthentication::bearer(move |req, credentials| {
            handle_validate(req, credentials, jwt_secret.clone())
        });
        App::new()
            .app_data(Data::new(AppState {
                db: db_addr.clone(),
                hash_secret: properties.hash_secret.to_owned(),
            }))
            .configure(routes::auth_config)
            .service(
                web::scope("")
                    .wrap(bearer_middleware)
                    .configure(routes::config),
            )
    })
    .bind(properties.web.bind)?
    .run()
    .await?;

    Ok(())
}
