use core::{error, fmt};
use std::io;

use lazy_json::{JSON, a_bc};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Lexer(a_bc::Error<JSON>),
    Tokio(tokio::task::JoinError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "I/O error: {e}"),
            Self::Lexer(e) => write!(f, "{e}"),
            Self::Tokio(e) => write!(f, "Tokio error: {e}"),
        }
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<a_bc::Error<JSON>> for Error {
    fn from(value: a_bc::Error<JSON>) -> Self {
        Self::Lexer(value)
    }
}

impl From<tokio::task::JoinError> for Error {
    fn from(value: tokio::task::JoinError) -> Self {
        Self::Tokio(value)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            Self::Lexer(e) => Some(e),
            Self::Tokio(e) => Some(e),
        }
    }
}
