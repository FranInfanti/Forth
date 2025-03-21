use std::fmt::{Display, Formatter, Result};

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
    InvalidAmountOfArguments,
    InvalidArguments,
    StackError,
    FileCreateError,
    FileWriteError,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::StackOverflow => write!(f, "stack-overflow"),
            Self::StackUnderflow => write!(f, "stack-underflow"),
            Self::InvalidWord => write!(f, "invalid-word"),
            Self::DivisionByZero => write!(f, "division-by-zero"),
            Self::MissingWord => write!(f, "?"),
            Self::FileOpenError => write!(f, "file-open-error"),
            Self::FileReadError => write!(f, "file-read-error"),
            Self::ParsingError => write!(f, "parsing-error"),
            Self::InvalidAmountOfArguments => write!(f, "invalid-amount-of-arguments"),
            Self::InvalidArguments => write!(f, "invalid-arguments"),
            Self::StackError => write!(f, "stack-error"),
            Self::FileCreateError => write!(f, "file-create-error"),
            Self::FileWriteError => write!(f, "file-write-error"),
        }
    }
}
