pub mod error;
pub mod ffi;
pub mod source;

use error::Error;
use source::{IntoLuaSource, LuaSource};

use std::borrow::Cow;
use std::{slice, str};

use std::path::Path;
use std::io::Read;
use std::fs::File;

/// Custom type to return lua errors
pub type Result<T> = ::std::result::Result<T, Error>;

/// Used to index the lua stack relative to the Bottom and the Top positions
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

/// Type to perform operations over an underlying `lua_State` safely
#[derive(Debug)]
pub struct LuaState {
    lua_state: *mut ffi::lua_State,
}

/// Type to configura the garbage collector
#[derive(Debug)]
pub struct LuaGc<'a> {
    state: &'a LuaState,
}

impl Drop for LuaState {
    fn drop(&mut self) {
        unsafe { ffi::lua_close(self.lua_state) }
    }
}

/// Trait to obtain rust types from the stack
pub trait FromLua<'a>: Sized {
    unsafe fn from_lua(state: &'a LuaState, index: ::std::os::raw::c_int) -> Option<Self>;
}

/// Trait to move rust types into the lua stack
pub trait IntoLua {
    unsafe fn into_lua(self, state: *mut ffi::lua_State);
}

/// Trait for types that can be pushed to the stack as strings
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
    unsafe fn into_lua(self, state: *mut ffi::lua_State) {
        let string = self.as_ref();
        ffi::lua_pushlstring(state, string.as_ptr() as _, string.len() as _);
    }
}

impl<'a> FromLua<'a> for &'a str {
    unsafe fn from_lua(state: &'a LuaState, index: ::std::os::raw::c_int) -> Option<Self> {
        let mut len = 0;

        let ptr = ffi::lua_tolstring(state.lua_state, index, &mut len);
        if ptr.is_null() {
            None
        } else {
            let slice = slice::from_raw_parts(ptr as *const u8, len);
            str::from_utf8(slice).ok()
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
    #[inline]
    unsafe fn from_lua(state: &'a LuaState, index: ::std::os::raw::c_int) -> Option<Self> {
        Some(ffi::lua_toboolean(state.lua_state, index) != 0)
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

    pub fn eval<T>(&mut self, source: T) -> Result<()>
    where
        T: IntoLuaSource,
    {
        self.load(source)?;
        self.call_protected(0, 0)?;
        Ok(())
    }

    pub fn load_from_file<P>(&mut self, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        self.load_from_reader(File::open(path)?)
    }

    pub fn load_from_reader<R>(&mut self, read: R) -> Result<()>
    where
        R: Read,
    {
        let source = LuaSource::from_reader(read)?;
        self.load(source)
    }

    pub fn call_protected(&mut self, nargs: usize, nresults: usize) -> Result<()> {
        unsafe {
            match ffi::lua_pcall(self.lua_state, nargs as _, nresults as _, 0) as _ {
                ffi::LUA_OK => Ok(()),
                code @ _ => Err(Error::from_lua_result(code as _)),
            }
        }
    }

    pub fn load<T>(&mut self, source: T) -> Result<()>
    where
        T: IntoLuaSource,
    {
        unsafe {
            let source = source.into();
            match ffi::luaL_loadstring(self.lua_state, source.as_ptr() as _) as _ {
                ffi::LUA_OK => Ok(()),
                code @ _ => Err(Error::from_lua_result(code as _)),
            }
        }
    }

    pub fn pop(&mut self, n: usize) {
        unsafe { ffi::lua_pop(self.lua_state, n as _) }
    }

    pub fn gc(&self) -> LuaGc {
        LuaGc { state: self }
    }

    pub fn push_value<T>(&mut self, value: T)
    where
        T: IntoLua,
    {
        unsafe { value.into_lua(self.lua_state) }
    }

    pub fn get_value<'a, F>(&'a self, index: Index) -> Option<F>
    where
        F: FromLua<'a>,
    {
        unsafe { F::from_lua(self, index.as_absolute()) }
    }

    pub fn push_nil(&mut self) {
        unsafe { ffi::lua_pushnil(self.lua_state) }
    }

    pub fn is_nil(&self, idx: Index) -> bool {
        unsafe { ffi::lua_isnil(self.lua_state, idx.as_absolute()) }
    }

    pub fn replace(&mut self, idx: Index) {
        unsafe { ffi::lua_replace(self.lua_state, idx.as_absolute()) }
    }
}

impl<'a> LuaGc<'a> {
    #[inline]
    pub fn collect(&mut self) {
        unsafe { ffi::lua_gc(self.state.lua_state, ffi::LUA_GCCOLLECT as _, 0) };
    }

    #[inline]
    pub fn stop(&mut self) {
        unsafe { ffi::lua_gc(self.state.lua_state, ffi::LUA_GCSTOP as _, 0) };
    }

    #[inline]
    pub fn restart(&mut self) {
        unsafe { ffi::lua_gc(self.state.lua_state, ffi::LUA_GCRESTART as _, 0) };
    }

    #[inline]
    pub fn step(&mut self, arg: i32) {
        unsafe { ffi::lua_gc(self.state.lua_state, ffi::LUA_GCSTEP as _, arg) };
    }

    #[inline]
    pub fn set_pause(&mut self, arg: i32) -> i32 {
        unsafe { ffi::lua_gc(self.state.lua_state, ffi::LUA_GCSETPAUSE as _, arg) }
    }

    #[inline]
    pub fn set_step_mul(&mut self, arg: i32) -> i32 {
        unsafe { ffi::lua_gc(self.state.lua_state, ffi::LUA_GCSETPAUSE as _, arg) }
    }

    #[inline]
    pub fn is_running(&self) -> bool {
        unsafe { ffi::lua_gc(self.state.lua_state, ffi::LUA_GCISRUNNING as _, 0) != 0 }
    }

    #[inline]
    pub fn count(&self) -> i32 {
        unsafe { ffi::lua_gc(self.state.lua_state, ffi::LUA_GCCOUNT as _, 0) }
    }
}
