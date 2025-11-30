use std::fmt;

#[derive(Debug)]
pub enum GloxError {
    // Used during Parsing, holds the bad token and the line number
    UnexpectedToken(String, i32),

    // Used during Interpreter pass
    RuntimeError(String)
}

impl fmt::Display for GloxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GloxError::UnexpectedToken(s, line) => write!(f, "Parsing error, unexpected token: {} at line: {}", s, line),
            GloxError::RuntimeError(s) => write!(f, "RuntimeError: {}", s)
        }
    }
}

impl std::error::Error for GloxError {}
