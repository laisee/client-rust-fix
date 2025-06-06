use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum FixClientError {
    ConfigError(String),
    ConnectionError(String),
    MessageError(String),
    // Add more specific error types
}

impl fmt::Display for FixClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FixClientError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            FixClientError::ConnectionError(msg) => write!(f, "Connection error: {}", msg),
            FixClientError::MessageError(msg) => write!(f, "Message error: {}", msg),
        }
    }
}

impl Error for FixClientError {}