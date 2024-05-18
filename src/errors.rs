use std::net::AddrParseError;

use thiserror::Error;

/// Custom error type for the WaitService.
#[derive(Debug, Error)]
pub enum WaitServiceError {
    /// Remove non existing key.
    #[error("Cannot parse url")]
    UrlNotParsed,

    /// Urls empty.
    #[error("Urls are empty")]
    UrlsEmpty,

    /// Url parse error.
    #[error("{}", _0)]
    Url(#[from] url::ParseError),

    /// Address parse error.
    #[error("{}", _0)]
    Address(#[from] AddrParseError),

    /// IO error.
    #[error("{}", _0)]
    Io(#[from] std::io::Error),

    /// Service is not available.
    #[error("Service is not available")]
    ServiceNotAvailable,
}

pub type Result<T> = std::result::Result<T, WaitServiceError>;
