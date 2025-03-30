use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
/// Tipo para describir los errores que pueden resultar de la ejecución del programa.
pub enum Error {
    /// Error al realizar una operación, los elementos del stack son insuficientes.
    StackUnderflow,
    /// Error al realizar una operación, el stack esta completo.
    StackOverflow,
    /// Error al definir un word, word-name invalido.
    InvalidWord,
    DivisionByZero,
    /// Error al utilizar un word, no se encuentra definido en el sistema.
    MissingWord,
    FileOpenError,
    FileReadError,
    ParsingError,
    /// Error al ejecutar el programa, los argumentos propocionados son insuficientes.
    InvalidAmountOfArguments,
    /// Error al eecutar el programa, los argumentos proporcionados son invalidos.
    InvalidArguments,
    /// Error al acceder al stack.
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
