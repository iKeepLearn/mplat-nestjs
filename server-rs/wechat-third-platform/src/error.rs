use std::{string::FromUtf8Error, time::SystemTimeError};

use thiserror::Error;
use tracing::error;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Config error:{0}")]
    Config(String),
    #[error("Network error:{0}")]
    Network(String),
}

impl From<base64::DecodeError> for Error {
    fn from(value: base64::DecodeError) -> Self {
        error!("base64 DecodeError:{}", value);
        Error::Config(value.to_string())
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        error!("reqwest::Error:{}", value);
        Error::Network(value.to_string())
    }
}

impl From<FromUtf8Error> for Error {
    fn from(value: FromUtf8Error) -> Self {
        Error::Config(value.to_string())
    }
}

impl From<SystemTimeError> for Error {
    fn from(value: SystemTimeError) -> Self {
        Error::Config(value.to_string())
    }
}
