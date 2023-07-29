use actix::SyncArbiter;
use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use actix_web_httpauth::middleware::HttpAuthentication;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use dotenv::dotenv;
use std::env;


use db::utils::{get_pool, AppState, DbActor};
use login::services::{basic_auth, create_user};
use note::services::{create_note, delete_note, fetch_notes, update_note};
use validator::validator::handle_validate;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool: Pool<ConnectionManager<PgConnection>> = get_pool(&db_url);
    let db_addr = SyncArbiter::start(5, move || DbActor(pool.clone()));

    HttpServer::new(move || {
        let bearer_middleware = HttpAuthentication::bearer(handle_validate);
        App::new()
            .app_data(Data::new(AppState {
                db: db_addr.clone(),
            }))
            .service(create_user)
            .service(basic_auth)
            .service(
                web::scope("")
                    .wrap(bearer_middleware.clone())
                    .service(create_note)
                    .service(fetch_notes)
                    .service(update_note)
                    .service(delete_note),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
