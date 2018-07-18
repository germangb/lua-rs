pub mod ffi;

use std::ffi::{OsString, OsStr};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Error {
    Runtime,
    Syntax,
    Memory,
    Gc,
}

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Index {
    /// index from the top of the stack
    Top(usize),

    /// index from the bottom of the stack
    Bottom(usize),
}

impl Index {
    #[inline]
    pub fn as_absolute(&self) -> ::std::os::raw::c_int {
        match *self {
            Index::Top(i) => {
                let idx = i as ::std::os::raw::c_int;
                -idx
            }
            Index::Bottom(i) => i as _,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LuaSource {
    buffer: Vec<u8>,
    len: usize,
}

impl LuaSource {
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(capacity),
            len: 0,
        }
    }

    pub fn extend<T: AsRef<[u8]>>(&mut self, s: T) {
        let slice = s.as_ref();

        self.buffer.pop();
        self.buffer.extend_from_slice(slice);
        self.buffer.push(0);
        self.len += slice.len();
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.buffer.as_ptr()
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

#[derive(Debug)]
pub struct LuaState {
    lua_state: *mut ffi::lua_State,
}

impl Drop for LuaState {
    fn drop(&mut self) {
        unsafe { ffi::lua_close(self.lua_state) }
    }
}

pub trait FromLua<'a>: Sized {
    unsafe fn from_lua(state: &'a LuaState, index: ::std::os::raw::c_int) -> Option<Self>;
}

pub trait IntoLua {
    unsafe fn into_lua(self, state: *mut ffi::lua_State);
}

impl<'a> IntoLua for &'a str {
    unsafe fn into_lua(self, state: *mut ffi::lua_State) {
        ffi::lua_pushlstring(state, self.as_ptr() as _, self.len() as _);
    }
}

impl<'a> FromLua<'a> for &'a str {
    unsafe fn from_lua(state: &'a LuaState, index: ::std::os::raw::c_int) -> Option<Self> {
        let mut len = 0;

        let ptr = ffi::lua_tolstring(state.lua_state, index, &mut len);
        if ptr.is_null() {
            None
        } else {
            let slice = ::std::slice::from_raw_parts(ptr as *const u8, len);
            ::std::str::from_utf8(slice).ok()
        }
    }
}

macro_rules! impl_numeric {
    (
        $( ( $( $type:ty ),+ ) => $lua_push:ident, $lua_to:ident )+
    ) => {
        $(
            $(
                impl IntoLua for $type {
                    unsafe fn into_lua(self, state: *mut ffi::lua_State) {
                        ffi::$lua_push(state, self as _);
                    }
                }

                impl<'a> FromLua<'a> for $type {
                    unsafe fn from_lua(state: &'a LuaState, index: ::std::os::raw::c_int) -> Option<Self> {
                        let mut isnum = 0;

                        let value = ffi::$lua_to(state.lua_state, index, &mut isnum);

                        if isnum == 0 {
                            None
                        } else {
                            Some(value as $type)
                        }
                    }
                }
            )+
        )+
    }
}

impl_numeric! {
    (f64, f32) => lua_pushnumber, lua_tonumberx
    (i64, i32, i16, i8) => lua_pushinteger, lua_tointegerx
}

impl IntoLua for bool {
    unsafe fn into_lua(self, state: *mut ffi::lua_State) {
        if self {
            ffi::lua_pushboolean(state, 1);
        } else {
            ffi::lua_pushboolean(state, 0);
        }
    }
}

impl<'a> FromLua<'a> for bool {
    unsafe fn from_lua(state: &'a LuaState, index: ::std::os::raw::c_int) -> Option<Self> {
        if ffi::lua_toboolean(state.lua_state, index) == 0 {
            Some(true)
        } else {
            Some(false)
        }
    }
}

impl LuaState {
    pub fn new() -> Self {
        unsafe {
            LuaState {
                lua_state: ffi::luaL_newstate(),
            }
        }
    }

    #[cfg(feature = "stdlib")]
    pub fn open_libs(&self) {
        unsafe { ffi::luaL_openlibs(self.lua_state) }
    }

    pub fn close(self) {}

    pub fn execute(&mut self, source: &LuaSource) -> Result<()> {
        unsafe {
            ffi::luaL_dostring(self.lua_state, source.as_ptr() as _);
        }
        Ok(())
    }

    pub fn load(&mut self, source: &LuaSource) -> Result<()> {
        unsafe {
            match ffi::luaL_loadstring(self.lua_state, source.as_ptr() as _) as _ {
                ffi::LUA_OK => Ok(()),
                ffi::LUA_ERRRUN => Err(Error::Runtime),
                ffi::LUA_ERRSYNTAX => Err(Error::Syntax),
                ffi::LUA_ERRMEM => Err(Error::Memory),
                ffi::LUA_ERRGCMM => Err(Error::Gc),
                _ => unreachable!(),
            }
        }
    }

    #[inline]
    pub fn pop(&mut self, n: usize) {
        unsafe { ffi::lua_pop(self.lua_state, n as _) }
    }

    #[inline]
    pub fn push_value<T: IntoLua>(&mut self, value: T) {
        unsafe { value.into_lua(self.lua_state) }
    }

    #[inline]
    pub fn get_value<'a, F: FromLua<'a>>(&'a self, index: Index) -> Option<F> {
        unsafe { F::from_lua(self, index.as_absolute()) }
    }

    #[inline]
    pub fn push_nil(&mut self) {
        unsafe { ffi::lua_pushnil(self.lua_state) }
    }

    #[inline]
    pub fn is_nil(&self, idx: Index) -> bool {
        unsafe { ffi::lua_isnil(self.lua_state, idx.as_absolute()) }
    }

    #[inline]
    pub fn replace(&mut self, idx: Index) {
        unsafe { ffi::lua_replace(self.lua_state, idx.as_absolute()) }
    }
}
