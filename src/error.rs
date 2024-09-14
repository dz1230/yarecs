use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum RecsError {
    PoolAccessOrCreationError,
    Other(Box<dyn Error + Send + Sync>), // Generic error for wrapping others
}

impl fmt::Display for RecsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RecsError::PoolAccessOrCreationError => write!(f, "Error accessing or creating recs pool. This error is technically impossible."),
            RecsError::Other(e) => write!(f, "Error in recs: {}", e),
        }
    }
}