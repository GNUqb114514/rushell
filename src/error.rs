use thiserror::Error;

#[derive(Error, Debug)]
pub enum RushellError {
    #[error("I/O Error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Command not found: {0}")]
    CommandNotFound(String),
    #[error("Syntax error: {0}")]
    SyntaxError(#[from] peg::error::ParseError<peg::str::LineCol>),
}

pub type Result<T> = std::result::Result<T, RushellError>;
