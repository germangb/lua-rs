use std::error::Error as StdError;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Runtime,
    Syntax,
    Memory,
    Gc,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "Error::Io({})", e),
            Error::Runtime => write!(f, "Error::Runtime"),
            Error::Syntax => write!(f, "Error::Syntax"),
            Error::Memory => write!(f, "Error::Memory"),
            Error::Gc => write!(f, "Error::Gc"),
        }
    }
}

impl StdError for Error {
    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Io(ref e) => Some(e),
            _ => None,
        }
    }
}
