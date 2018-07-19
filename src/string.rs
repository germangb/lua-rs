use super::error::Error;
use super::{ffi, FromLua, Index, IntoLua, LuaState, Result};

use std::{slice, str};

/// A type that implements LuaString can be pushed into the lua stack
pub trait LuaString {}

impl<'a> LuaString for &'a str {}
impl<'a> LuaString for &'a String {}
impl LuaString for String {}
impl LuaString for Vec<u8> {}
impl<'a> LuaString for &'a [u8] {}

impl<T> IntoLua for T
where
    T: AsRef<[u8]> + LuaString,
{
    fn into_lua(self, state: &mut LuaState) {
        unsafe {
            let string = self.as_ref();
            ffi::lua_pushlstring(state.lua_state, string.as_ptr() as _, string.len() as _);
        }
    }
}

/// View into an internal lua String that may contain zero bytes within.
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
    pub fn as_str(&self) -> Result<&str> {
        unsafe {
            let bytes = slice::from_raw_parts(self.ptr as *const u8, self.length);

            match str::from_utf8(bytes).map_err(|e| Error::Utf8) {
                Err(_) => Err(Error::Utf8),
                Ok(s) => Ok(s),
            }
        }
    }

    pub unsafe fn as_str_unchecked(&self) -> &str {
        let bytes = slice::from_raw_parts(self.ptr as *const u8, self.length);
        str::from_utf8_unchecked(bytes)
    }
}
