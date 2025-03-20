use std::fmt;

#[derive(Debug)]
pub enum Error {
    StackUnderflow,
    StackOverflow,
    InvalidWord,
    DivisionByZero,
    MissingWord,
    FileOpenError,
    FileReadError,
    ParsingError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::StackOverflow => write!(f, "stack-overflow"),
            Self::StackUnderflow => write!(f, "stack-underflow"),
            Self::InvalidWord => write!(f, "invalid-word"),
            Self::DivisionByZero => write!(f, "division-by-zero"),
            Self::MissingWord => write!(f, "?"),
            Self::FileOpenError => write!(f, "file-open-error"),
            Self::FileReadError => write!(f, "file-read-error"),
            Self::ParsingError => write!(f, "parsing-error"),
        }
    }
}
