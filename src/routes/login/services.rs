use crate::{
    db::messages::{CreateUser, GetUser},
    db::utils::TokenClaims,
    db::utils::{AppState, DbActor},
    error::AppResult,
};
use actix::Addr;
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use actix_web_httpauth::extractors::basic::BasicAuth;
use argonautica::{Hasher, Verifier};
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use serde::Deserialize;
use sha2::Sha256;

#[derive(Deserialize)]
pub(crate) struct CreateUserBody {
    username: String,
    password: String,
}

pub(crate) async fn create_user(
    state: Data<AppState>,
    body: Json<CreateUserBody>,
) -> AppResult<HttpResponse> {
    let db: Addr<DbActor> = state.as_ref().db.clone();

    let user: CreateUserBody = body.into_inner();
    let hash_secret = state.hash_secret.clone();
    let mut hasher = Hasher::default();
    let hash = hasher
        .with_password(user.password)
        .with_secret_key(hash_secret)
        .hash()?;

    let res = db
        .send(CreateUser {
            username: user.username,
            password: hash,
        })
        .await??;

    Ok(HttpResponse::Ok().json(res))
}

pub(crate) async fn basic_auth(
    state: Data<AppState>,
    credentials: BasicAuth,
) -> AppResult<HttpResponse> {
    let jwt_secret: Hmac<Sha256> = Hmac::new_from_slice(
        state.jwt_secret.as_bytes(),
    )?;
    let username = credentials.user_id().to_string();
    let password = credentials.password();

    match password {
        None => Ok(HttpResponse::Unauthorized().json("Must provide password")),
        Some(pass) => {
            let db: Addr<DbActor> = state.as_ref().db.clone();

            let user = db.send(GetUser { username }).await??;

            let hash_secret = std::env::var("HASH_SECRET")?;

            let mut verifier = Verifier::default();
            let is_valid = verifier
                .with_hash(user.password)
                .with_password(pass)
                .with_secret_key(hash_secret)
                .verify()?;

            let res = if is_valid {
                let claims = TokenClaims { id: user.id };
                let token_str = claims.sign_with_key(&jwt_secret)?;
                HttpResponse::Ok().json(token_str)
            } else {
                HttpResponse::Unauthorized().json("Incorrect password")
            };

            Ok(res)
        }
    }
}
