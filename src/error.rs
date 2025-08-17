use std::fmt;

pub type TranspilerResult<T> = Result<T, TranspilerError>;

#[derive(Debug, Clone)]
pub struct TranspilerError {
    pub message: String,
    pub position: Option<usize>,
}

impl TranspilerError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            position: None,
        }
    }

    pub fn with_position(message: impl Into<String>, position: usize) -> Self {
        Self {
            message: message.into(),
            position: Some(position),
        }
    }
}

impl fmt::Display for TranspilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.position {
            Some(pos) => write!(f, "{} at position {}", self.message, pos),
            None => write!(f, "{}", self.message),
        }
    }
}

impl std::error::Error for TranspilerError {}

impl From<String> for TranspilerError {
    fn from(message: String) -> Self {
        Self::new(message)
    }
}

impl From<&str> for TranspilerError {
    fn from(message: &str) -> Self {
        Self::new(message)
    }
}
