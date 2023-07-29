use actix::MailboxError;
use actix_web::{http::header::ToStrError, HttpResponse, ResponseError};
use backtrace::Backtrace;

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
    FromHexError(#[from] rustc_hex::FromHexError),

    #[error(transparent)]
    RequestError(#[from] reqwest::Error),

    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
}

#[derive(serde::Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

impl ResponseError for AppErrors {
    fn error_response(&self) -> HttpResponse {
        println!("Call stack:\n{:?}", Backtrace::new());

        HttpResponse::InternalServerError().json(ErrorResponse {
            message: self.to_string(),
        })
    }
}

pub type AppResult<T> = Result<T, AppErrors>;
