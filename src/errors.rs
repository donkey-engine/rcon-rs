use serde::Serialize;
use thiserror::Error;

/// Errors enum for project
#[derive(Error, Debug, Serialize)]
pub enum RCONError {
    /// Error with TCP connection
    #[error("TCP connection error: {0}")]
    TcpConnectionError(String),
    /// Error with types converting
    #[error("Error with types conversion: {0}")]
    TypeError(String),
}
