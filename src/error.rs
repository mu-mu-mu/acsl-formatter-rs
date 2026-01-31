use std::fmt;

#[derive(Debug, Clone)]
pub enum Error {
    Lex(String),
    Parse(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Lex(msg) => write!(f, "lex error: {msg}"),
            Error::Parse(msg) => write!(f, "parse error: {msg}"),
        }
    }
}

impl std::error::Error for Error {}
