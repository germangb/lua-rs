use super::error::Error;
use super::{ffi, FromLua, Index, IntoLua, LuaState, Result};

use std::borrow::Cow;
use std::{slice, str, fmt};

/// A view into a Lua-owned string. Strings in Lua may contain arbitrary binary data such as zeros.
pub struct LuaStr<'a> {
    state: &'a LuaState,
    ptr: *const ::std::os::raw::c_char,
    length: usize,
}

impl<'a> FromLua<'a> for LuaStr<'a> {
    fn from_lua(state: &'a LuaState, index: Index) -> Result<Self> {
        unsafe {
            let mut len = 0;

            let ptr = ffi::lua_tolstring(state.pointer, index.as_absolute(), &mut len);
            if ptr.is_null() {
                Err(Error::Type)
            } else {
                Ok(LuaStr {
                    state,
                    ptr,
                    length: len,
                })
            }
        }
    }
}

impl<'a> LuaStr<'a> {
    pub fn as_ptr(&self) -> *const u8 {
        self.ptr as _
    }
    
    pub fn len(&self) -> usize {
        self.length
    }

    /// Returns this LuaStr as a UTF-8 string slice, if it cans.
    pub fn as_str(&self) -> Result<&str> {
        unsafe {
            let bytes = slice::from_raw_parts(self.ptr as *const u8, self.length);

            match str::from_utf8(bytes).map_err(|e| Error::Utf8) {
                Err(_) => Err(Error::Utf8),
                Ok(s) => Ok(s),
            }
        }
    }

    /// Returns string as a byte slice
    pub fn as_slice(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.ptr as *const u8, self.length) }
    }

    /// Converts data into a valid UTF-8 string using `String::from_utf8_lossy`.
    pub fn into_string_lossy(&self) -> Cow<str> {
        unsafe {
            let bytes = slice::from_raw_parts(self.ptr as *const u8, self.length);
            String::from_utf8_lossy(bytes)
        }
    }

    /// Reads the data as a `&str`
    ///
    /// # Panics
    /// When the string is not utf-8
    pub unsafe fn as_str_unchecked(&self) -> &str {
        let bytes = slice::from_raw_parts(self.ptr as *const u8, self.length);
        str::from_utf8_unchecked(bytes)
    }
}

impl<'a> fmt::Debug for LuaStr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Ok(s) = self.as_str() {
            write!(f, "{:?}", self.into_string_lossy())
        } else {
            write!(f, "{:?}", self.ptr)
        }
    }
}
