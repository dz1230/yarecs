use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum RecsError {
    PoolAccessOrCreationError,
    InvalidEntityError,
    Other(Box<dyn Error + Send + Sync>), // Generic error for wrapping others
}

impl fmt::Display for RecsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RecsError::PoolAccessOrCreationError => write!(f, "[RecsError] Error accessing or creating pool. This error is technically impossible. Please open a bug report issue at <url>."),
            RecsError::InvalidEntityError => write!(f, "[RecsError] Invalid entity error"),
            RecsError::Other(e) => write!(f, "[RecsError] Other error: {}", e),
        }
    }
}