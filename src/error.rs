use ffi;

use std::error::Error as StdError;
use std::{fmt, io};

#[derive(Debug)]
pub enum Error {
    /// Error during UTF-8 encoding/decoding
    Utf8,
    /// Error during IO
    Io(io::Error),
    /// Lua program execution error
    Runtime,
    /// Malformed lua syntax
    Syntax,
    /// Internal memory error
    Memory,
    /// Garbage collector error
    Gc,
    /// Some unknown error
    Unknown,
}

impl Error {
    #[inline]
    pub fn from_lua_result(res: ::std::os::raw::c_int) -> Error {
        match res as _ {
            ffi::LUA_ERRSYNTAX => Error::Syntax,
            ffi::LUA_ERRRUN => Error::Runtime,
            ffi::LUA_ERRMEM => Error::Memory,
            ffi::LUA_ERRGCMM => Error::Gc,
            _ => Error::Unknown,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Utf8 => write!(f, "UTF-8 error"),
            Error::Io(e) => write!(f, "IO error: {}", e),
            Error::Runtime => write!(f, "Runtime error"),
            Error::Syntax => write!(f, "Syntax error"),
            Error::Memory => write!(f, "Memory error"),
            Error::Gc => write!(f, "Garbade collector error"),
            Error::Unknown => write!(f, "Unknown error"),
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
