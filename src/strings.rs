use index::Index;
use {ffi, Error, FromLua, IntoLua, LuaState, Result};

use std::{slice, str};

macro_rules! impl_string {
    ($(ref $type:ty),+) => {
        $(impl<'a> IntoLua for &'a $type {
            #[inline]
            unsafe fn into_lua(self, state: &mut LuaState) {
                ffi::lua_pushlstring(state.pointer, self.as_ptr() as _, self.len() as _);
            }
        })+
    };
    ($($type:ty),+) => {
        $(impl IntoLua for $type {
            #[inline]
            unsafe fn into_lua(self, state: &mut LuaState) {
                ffi::lua_pushlstring(state.pointer, self.as_ptr() as _, self.len() as _);
            }
        })+
    }
}

impl_string! { ref str, ref [u8] }
impl_string! { String, Vec<u8> }

impl<'a> FromLua<'a> for &'a str {
    unsafe fn from_lua(state: &'a LuaState, idx: Index) -> Result<Self> {
        let mut len = 0;
        let ptr = ffi::lua_tolstring(state.pointer, idx.as_absolute(), &mut len);

        if ptr.is_null() {
            Err(Error::Type)
        } else {
            let slice = slice::from_raw_parts(ptr as *const u8, len);
            str::from_utf8(slice).map_err(|_| Error::Utf8)
        }
    }

    unsafe fn check(state: &LuaState, idx: Index) -> bool {
        ffi::lua_isstring(state.pointer, idx.as_absolute()) == 1
    }
}

impl<'a> FromLua<'a> for &'a [u8] {
    unsafe fn from_lua(state: &'a LuaState, idx: Index) -> Result<Self> {
        let mut len = 0;
        let ptr = ffi::lua_tolstring(state.pointer, idx.as_absolute(), &mut len);

        if ptr.is_null() {
            Err(Error::Type)
        } else {
            Ok(slice::from_raw_parts(ptr as *const u8, len))
        }
    }

    unsafe fn check(state: &LuaState, idx: Index) -> bool {
        ffi::lua_isstring(state.pointer, idx.as_absolute()) == 1
    }
}
