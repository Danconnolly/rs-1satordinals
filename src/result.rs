use std::fmt::Formatter;

/// Standard Result used in the library
pub type OrdinalResult<T> = Result<T, OrdinalError>;

/// Standard error type used in the library
#[derive(Debug)]
pub enum OrdinalError {
    /// An argument provided is invalid
    BadArgument(String),
}

impl std::fmt::Display for OrdinalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OrdinalError::BadArgument(s) => f.write_str(&format!("Bad argument: {}", s)),
        }
    }
}
