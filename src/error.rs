use thiserror::Error;

#[derive(Error, Debug)]
pub enum ScanError {
    #[error("Invalid CIDR format: {0}")]
    InvalidCidr(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Ping failed: {0}")]
    PingError(String),
}
