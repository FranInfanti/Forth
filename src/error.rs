use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
/// Enum que representa todos los errores que pueden llegar a ocurrir durante la ejecución del programa.
pub enum Error {
    /// Se lanza cuando se intenta realizar una operación que requiere de mas elementos de los que el stack actualmente tiene.
    StackUnderflow,
    /// Se lanza cuando se intenta seguir insertando elementos en un stack lleno.
    StackOverflow,
    /// Se lanza cuando se intenta definir una word cuyo word-name es invalido.
    InvalidWord,
    /// Se lanza cuando se intenta dividir por cero.
    DivisionByZero,
    /// Se lanza cuando se intenta utilizar un word que no esta definido en el sistema.
    MissingWord,
    /// Se lanza cuando ocurre algún error al intentar abrir un archivo.
    FileOpenError,
    /// Se lanza cuando ocurre algún error al leer un archivo.
    FileReadError,
    /// Se lanza cuando ocurre un error de parseo.
    ParsingError,
    /// Se lanza cuando se intenta correr el programa con una cantidad invalida de argumentos.
    InvalidAmountOfArguments,
    /// Se lanza cuando se intenta correr el programa con argumentos invalidos.
    InvalidArguments,
    /// Se lanza cuando ocurre algun error al realizar una operación con el stack.
    StackError,
    /// Se lanza cuando ocurre algun error al crear un archivo.
    FileCreateError,
    /// Se lanza cuando ocurre algun error al escrbir en un archivo.
    FileWriteError,
}

impl Display for Error {
    /// Se implementa el trait fmt para poder mostrar por stdout de forma clara el error ocurrido durante la ejecución.
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
