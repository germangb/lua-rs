use super::error::Error;
use super::{ffi, FromLua, Index, IntoLua, LuaState, Result};

use std::borrow::Cow;
use std::{slice, str};

macro_rules! impl_str {
    ($($type:ty),+) => {
        $(
            impl IntoLua for $type {
                fn into_lua(self, state: &mut LuaState) {
                    unsafe {
                        ffi::lua_pushlstring(state.lua_state, self.as_ptr() as _, self.len() as _);
                    }
                }
            }
        )+
    };
    ($( ref $type:ty ),+) => {
        $(
            impl<'a> IntoLua for &'a $type {
                fn into_lua(self, state: &mut LuaState) {
                    unsafe {
                        ffi::lua_pushlstring(state.lua_state, self.as_ptr() as _, self.len() as _);
                    }
                }
            }
        )+
    }
}

//impl_str! { ref str, ref String }
//impl_str! { String }

/// A view into a lua-owned string. A string in lua may contain zeroed bytes so it will not always
/// be possible to convert to a `&str`.
#[derive(Debug)]
pub struct LuaStr<'a> {
    state: &'a LuaState,
    ptr: *const ::std::os::raw::c_char,
    length: usize,
}

impl<'a> FromLua<'a> for LuaStr<'a> {
    fn from_lua(state: &'a LuaState, index: Index) -> Option<Self> {
        unsafe {
            let mut len = 0;

            let ptr = ffi::lua_tolstring(state.lua_state, index.as_absolute(), &mut len);
            if ptr.is_null() {
                None
            } else {
                Some(LuaStr {
                    state,
                    ptr,
                    length: len,
                })
            }
        }
    }
}

impl<'a> LuaStr<'a> {
    /// If the string contains valid UTF-8 text, returns a `&str`. If not, returns an
    /// `Error::Utf8`
    pub fn as_str(&self) -> Result<&str> {
        unsafe {
            let bytes = slice::from_raw_parts(self.ptr as *const u8, self.length);

            match str::from_utf8(bytes).map_err(|e| Error::Utf8) {
                Err(_) => Err(Error::Utf8),
                Ok(s) => Ok(s),
            }
        }
    }

    /// Converts data into a valid UTF-8 string using `String::from_utf8_lossy`, which turns any
    /// non-utf8 bytes into characters that look like this: �
    pub fn into_str_lossy(&self) -> Cow<str> {
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
