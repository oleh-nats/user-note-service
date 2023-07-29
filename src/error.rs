use actix::MailboxError;
use actix_web::{http::header::ToStrError, HttpResponse, ResponseError};
use argonautica::Error as ArgonauticaError;

#[derive(Debug)]
pub struct ArgonauticaErrorWrapper(argonautica::Error);

impl std::fmt::Display for ArgonauticaErrorWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ArgonauticaErrorWrapper {}

impl From<ArgonauticaError> for AppErrors {
    fn from(err: ArgonauticaError) -> Self {
        AppErrors::ArgonauticaError(ArgonauticaErrorWrapper(err))
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(thiserror::Error, Debug)]
pub enum AppErrors {
    #[error(transparent)]
    MailboxError(#[from] MailboxError),

    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),

    #[error(transparent)]
    ToStrError(#[from] ToStrError),

    #[error(transparent)]
    DigestInvalidLength(#[from] hmac::digest::InvalidLength),

    #[error(transparent)]
    EnvVarError(#[from] std::env::VarError),

    #[error(transparent)]
    JwtError(#[from] jwt::Error),

    #[error(transparent)]
    ArgonauticaError(#[from] ArgonauticaErrorWrapper),
}

#[derive(serde::Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

impl ResponseError for AppErrors {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::InternalServerError().json(ErrorResponse {
            message: self.to_string(),
        })
    }
}

pub type AppResult<T> = Result<T, AppErrors>;
