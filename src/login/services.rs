use crate::{
    db::messages::{CreateUser, GetUser},
    db::utils::TokenClaims,
    db::utils::{AppState, DbActor},
};
use actix::Addr;
use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use actix_web_httpauth::extractors::basic::BasicAuth;
use argonautica::{Hasher, Verifier};
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use serde::Deserialize;
use sha2::Sha256;

#[derive(Deserialize)]
struct CreateUserBody {
    username: String,
    password: String,
}

#[post("/api/register")]
async fn create_user(state: Data<AppState>, body: Json<CreateUserBody>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();

    let user: CreateUserBody = body.into_inner();
    let hash_secret = std::env::var("HASH_SECRET").expect("HASH_SECRET must be set!");
    let mut hasher = Hasher::default();
    let hash = hasher
        .with_password(user.password)
        .with_secret_key(hash_secret)
        .hash()
        .unwrap();

    match db
        .send(CreateUser {
            username: user.username,
            password: hash,
        })
        .await
    {
        Ok(Ok(info)) => HttpResponse::Ok().json(info), // handle all cases with responses
        Err(err) => HttpResponse::InternalServerError().json(format!("Failed: {}", err)),
        _ => HttpResponse::InternalServerError().json("Failed to create user"),
    }
}

#[post("/api/login")]
async fn basic_auth(state: Data<AppState>, credentials: BasicAuth) -> impl Responder {
    let jwt_secret: Hmac<Sha256> = Hmac::new_from_slice(
        std::env::var("JWT_SECRET")
            .expect("JWT_SECRET must be set!")
            .as_bytes(),
    )
    .unwrap();
    let username = credentials.user_id().to_string();
    let password = credentials.password();

    match password {
        None => HttpResponse::Unauthorized().json("Must provide username and password"),
        Some(pass) => {
            let db: Addr<DbActor> = state.as_ref().db.clone();

            match db.send(GetUser { username }).await {
                Ok(Ok(info)) => {
                    let hash_secret =
                        std::env::var("HASH_SECRET").expect("HASH_SECRET must be set!");
                    let mut verifier = Verifier::default();
                    let is_valid = verifier
                        .with_hash(info.password)
                        .with_password(pass)
                        .with_secret_key(hash_secret)
                        .verify()
                        .unwrap();

                    if is_valid {
                        let claims = TokenClaims { id: info.id };
                        let token_str = claims.sign_with_key(&jwt_secret).unwrap();
                        HttpResponse::Ok().json(token_str)
                    } else {
                        HttpResponse::Unauthorized().json("Incorrect password")
                    }
                }
                _ => HttpResponse::InternalServerError().json("Incorrect username"),
            }
        }
    }
}
