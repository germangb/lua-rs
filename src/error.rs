use ffi;

use std::error::Error as StdError;
use std::{fmt, io, io::ErrorKind};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Error {
    /// Error during UTF-8 encoding/decoding
    Utf8,
    /// Error during IO
    Io(io::ErrorKind),
    /// Lua program execution error
    Runtime,
    /// Malformed lua syntax
    Syntax,
    /// Internal memory error
    Memory,
    /// Garbage collector error
    Gc,
    /// Type error
    Type,
}

impl Error {
    #[inline]
    pub fn from_lua_result(res: ::std::os::raw::c_int) -> Error {
        match res as _ {
            ffi::LUA_ERRSYNTAX => Error::Syntax,
            ffi::LUA_ERRRUN => Error::Runtime,
            ffi::LUA_ERRMEM => Error::Memory,
            ffi::LUA_ERRGCMM => Error::Gc,
            _ => unreachable!(),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err.kind())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Utf8 => write!(f, "UTF-8 error"),
            Error::Io(e) => write!(f, "IO error: {:?}", e),
            Error::Runtime => write!(f, "Runtime error"),
            Error::Syntax => write!(f, "Syntax error"),
            Error::Memory => write!(f, "Memory error"),
            Error::Gc => write!(f, "Garbage collector error"),
            Error::Type => write!(f, "Unexpected type"),
        }
    }
}
