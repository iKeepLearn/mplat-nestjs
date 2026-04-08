use actix_web::{HttpResponse, ResponseError, http::StatusCode, mime};
use jsonwebtoken::errors::Error as JwtError;
use serde::Serialize;
use std::convert::Infallible;
use thiserror::Error;
use tracing::error;
use wechat_third_platform::error::Error as WtpError;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Internal error:{0}")]
    Internal(String),
    #[error("Not Found:{0}")]
    NotFound(String),
    #[error("{0}")]
    InvalidArgument(String),
    #[error("InvalidAuth")]
    InvalidAuth,
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    InvalidPermission(String),
}

impl Error {
    pub fn internal(str: &str) -> Self {
        Error::Internal(str.to_string())
    }

    pub fn bad_request(str: &str) -> Self {
        Error::BadRequest(str.to_string())
    }

    pub fn invalid_arg(str: &str) -> Self {
        Error::InvalidArgument(str.to_string())
    }

    pub fn not_found(str: &str) -> Self {
        Error::NotFound(str.to_string())
    }

    pub fn invalid_permission(str: &str) -> Self {
        Error::InvalidPermission(str.to_string())
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ErrorResponse {
    code: u16,
    data: Option<String>,
    #[serde(rename = "errorMsg")]
    error_msg: String,
}

impl ErrorResponse {
    pub fn new(code: u16, message: &str) -> Self {
        ErrorResponse {
            code,
            data: None,
            error_msg: message.to_string(),
        }
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        StatusCode::OK
    }

    fn error_response(&self) -> HttpResponse {
        let res = match self {
            Error::Internal(message) => ErrorResponse::new(500, message),
            Error::InvalidArgument(message) => ErrorResponse::new(1001, message),
            Error::NotFound(message) => ErrorResponse::new(1001, message),
            Error::InvalidAuth => ErrorResponse::new(1000, "invalid auth"),
            Error::BadRequest(message) => ErrorResponse::new(400, message),
            Error::InvalidPermission(message) => ErrorResponse::new(1000, message),
        };
        HttpResponse::build(self.status_code())
            .content_type(mime::APPLICATION_JSON)
            .json(res)
    }
}

impl From<sqlx::migrate::MigrateError> for Error {
    fn from(err: sqlx::migrate::MigrateError) -> Self {
        error!("MigrateError: {:?}", err);
        Error::Internal("Internal Server Error".into())
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Error::InvalidArgument(err.to_string())
    }
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        error!("Sqlx Error: {:?}", err);
        match err {
            sqlx::Error::RowNotFound => Error::NotFound("row not exists".into()),
            _ => Error::Internal("Sqlx Error".into()),
        }
    }
}

impl From<Infallible> for Error {
    fn from(value: Infallible) -> Self {
        Error::InvalidArgument(value.to_string())
    }
}

impl From<Vec<u8>> for Error {
    fn from(value: Vec<u8>) -> Self {
        Error::InvalidArgument(String::from_utf8_lossy(&value).to_string())
    }
}

impl From<JwtError> for Error {
    fn from(value: JwtError) -> Self {
        error!("JwtError: {:?}", value);
        Error::InvalidAuth
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::Internal(format!("Io Error:{:?}", value.to_string()))
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::Internal(value.to_string())
    }
}

impl From<std::string::String> for Error {
    fn from(value: std::string::String) -> Self {
        Error::Internal(value)
    }
}

impl From<systemd_service::Error> for Error {
    fn from(value: systemd_service::Error) -> Self {
        Error::Internal(value.to_string())
    }
}

impl From<strum::ParseError> for Error {
    fn from(value: strum::ParseError) -> Self {
        Error::Internal(value.to_string())
    }
}

impl From<validator::ValidationErrors> for Error {
    fn from(errors: validator::ValidationErrors) -> Self {
        let error_messages: Vec<String> = errors
            .field_errors()
            .into_iter()
            .flat_map(|(_field, field_errors)| {
                field_errors.iter().map(move |err| {
                    err.message
                        .as_ref()
                        .map_or("invalid argument", |msg| msg)
                        .to_string()
                })
            })
            .collect();

        let message = error_messages.join(",");

        Error::invalid_arg(&message)
    }
}

impl From<WtpError> for Error {
    fn from(value: WtpError) -> Self {
        error!("wechat third platform: {:?}", value);
        Error::internal("internal error")
    }
}
