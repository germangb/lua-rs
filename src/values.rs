use {ffi, CheckLua, Error, FromLua, Index, IntoLua, Result, State};

use std::{slice, str};

macro_rules! impl_numbers {
    ($($type:ty),+) => {
        $(impl IntoLua for $type {
            #[inline]
            unsafe fn into_lua(self, state: &mut State) {
                ffi::lua_pushinteger(state.pointer, self as _);
            }
        })+
        $(impl CheckLua for $type {
            #[inline]
            unsafe fn check(state: &State, idx: Index) -> bool {
                ffi::lua_isnumber(state.pointer, idx.as_absolute()) == 1
            }
        })+
        $(impl<'a> FromLua<'a> for $type {
            #[inline]
            unsafe fn from_lua(state: &'a State, idx: Index) -> Result<Self> {
                let mut isnum = 0;
                let res = ffi::lua_tonumberx(state.pointer, idx.as_absolute() as _, &mut isnum);
                if isnum == 0 { Err(Error::Type) }
                else { Ok(res as $type) }
            }
        })+
    }
}

macro_rules! impl_integers {
    ($($type:ty),+) => {
        $(impl IntoLua for $type {
            #[inline]
            unsafe fn into_lua(self, state: &mut State) {
                ffi::lua_pushnumber(state.pointer, self as _);
            }
        })+
        $(impl CheckLua for $type {
            #[inline]
            unsafe fn check(state: &State, idx: Index) -> bool {
                ffi::lua_isinteger(state.pointer, idx.as_absolute()) == 1
            }
        })+
        $(impl<'a> FromLua<'a> for $type {
            #[inline]
            unsafe fn from_lua(state: &'a State, idx: Index) -> Result<Self> {
                let mut isnum = 0;
                let res = ffi::lua_tointegerx(state.pointer, idx.as_absolute() as _, &mut isnum);
                if isnum == 0 { Err(Error::Type) }
                else { Ok(res as $type) }
            }
        })+
    }
}

impl_integers!{
    i8, i16, i32, i64, isize, i128,
    u8, u16, u32, u64, usize, u128
}
impl_numbers!{ f32, f64 }

impl IntoLua for bool {
    #[inline]
    unsafe fn into_lua(self, state: &mut State) {
        if self {
            ffi::lua_pushboolean(state.pointer, 1);
        } else {
            ffi::lua_pushboolean(state.pointer, 0);
        }
    }
}

impl CheckLua for bool {
    #[inline]
    unsafe fn check(state: &State, idx: Index) -> bool {
        ffi::lua_isboolean(state.pointer, idx.as_absolute())
    }
}

impl<'a> FromLua<'a> for bool {
    #[inline]
    unsafe fn from_lua(state: &'a State, idx: Index) -> Result<Self> {
        Ok(ffi::lua_toboolean(state.pointer, idx.as_absolute() as _) == 1)
    }
}

macro_rules! impl_string {
    ($(ref $type:ty),+) => {
        $(impl<'a> IntoLua for &'a $type {
            #[inline]
            unsafe fn into_lua(self, state: &mut State) {
                ffi::lua_pushlstring(state.pointer, self.as_ptr() as _, self.len() as _);
            }
        })+
        $(impl CheckLua for $type {
            #[inline]
            unsafe fn check(state: &State, idx: Index) -> bool {
                ffi::lua_isstring(state.pointer, idx.as_absolute()) == 1
            }
        })+
    };
    ($($type:ty),+) => {
        $(impl IntoLua for $type {
            #[inline]
            unsafe fn into_lua(self, state: &mut State) {
                ffi::lua_pushlstring(state.pointer, self.as_ptr() as _, self.len() as _);
            }
        })+
    }
}

impl_string! { ref str, ref [u8] }
impl_string! { String, Vec<u8> }

impl<'a> FromLua<'a> for &'a str {
    unsafe fn from_lua(state: &'a State, idx: Index) -> Result<Self> {
        let mut len = 0;
        let ptr = ffi::lua_tolstring(state.pointer, idx.as_absolute(), &mut len);

        if ptr.is_null() {
            Err(Error::Type)
        } else {
            let slice = slice::from_raw_parts(ptr as *const u8, len);
            str::from_utf8(slice).map_err(|_| Error::Utf8)
        }
    }
}

impl<'a> FromLua<'a> for &'a [u8] {
    unsafe fn from_lua(state: &'a State, idx: Index) -> Result<Self> {
        let mut len = 0;
        let ptr = ffi::lua_tolstring(state.pointer, idx.as_absolute(), &mut len);

        if ptr.is_null() {
            Err(Error::Type)
        } else {
            Ok(slice::from_raw_parts(ptr as *const u8, len))
        }
    }
}
