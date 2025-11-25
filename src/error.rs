use std::fmt;
use std::error;

pub enum GloxError {
    // Used during Parsing
    UnexpectedToken(String),

    // Used during Interpreter pass
    RuntimeError(String)
}

impl fmt::Display for GloxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GloxError::UnexpectedToken(s) => write!(f, "Parsing error, unexpected token: {}", s),
            GloxError::RuntimeError(s) => write!(f, "RuntimeError: {}", s)
        }
    }
}

impl std::error::Error for GloxError {}
