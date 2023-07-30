use serde::{Deserialize, Serialize};

use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use sha2::Sha256;

use actix_web::{dev::ServiceRequest, error::Error, HttpMessage};

use actix_web_httpauth::extractors::{
    bearer::{self, BearerAuth},
    AuthenticationError,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct TokenClaims {
    pub id: i32,
}

pub async fn handle_validate(
    req: ServiceRequest,
    credentials: BearerAuth,
    jwt_secret: &str,
) -> anyhow::Result<ServiceRequest, (Error, ServiceRequest)> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_bytes()).unwrap();
    let token_string = credentials.token();

    let claims: Result<TokenClaims, &str> = token_string
        .verify_with_key(&key)
        .map_err(|_| "Invalid token");

    match claims {
        Ok(value) => {
            req.extensions_mut().insert(value);
            Ok(req)
        }
        Err(_) => {
            let config = req
                .app_data::<bearer::Config>()
                .cloned()
                .unwrap_or_default()
                .scope("");

            Err((AuthenticationError::from(config).into(), req))
        }
    }
}
