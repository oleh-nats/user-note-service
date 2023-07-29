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
) -> anyhow::Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    let jwt_secret: String = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set!");
    let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_bytes())?;
    let token_string = credentials.token();

    let claims: TokenClaims = token_string.verify_with_key(&key)?;

    req.extensions_mut().insert(claims);

    Ok(req)

    // match claims {
    //     Ok(value) => {
    //         req.extensions_mut().insert(value);
    //         Ok(req)
    //     }
    //     Err(_) => {
    //         let config = req
    //             .app_data::<bearer::Config>()
    //             .cloned()
    //             .unwrap_or_default()
    //             .scope("");

    //         Err((AuthenticationError::from(config).into(), req))
    //     }
    // }
}
