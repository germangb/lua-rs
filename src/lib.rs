#[macro_use]
pub mod macros;

/// Error types
pub mod error;
/// Raw bindings from the Lua C API
pub mod ffi;
/// Utilities to implement rust functions that can be called from Lua
pub mod functions;
/// Utilities for indexing the stack
pub mod index;
/// contains implementations of `FromLua` and `IntoLua` for rust numeric types
pub mod numbers;
/// Re-exports of common types & traits
///
/// ```rust
/// extern crate lua;
///
/// use lua::prelude::*;
/// ```
pub mod prelude;
/// Utilities to work with Lua strings
pub mod strings;
/// Utilities to work with Lua userdata
pub mod userdata;

use functions::LuaFunction;
use userdata::{LuaUserData, FromLuaData};

use error::Error;
use ffi::AsCStr;
use index::Index;

use std::{fs::File, io::Read, path::Path, str, fmt};

/// Custom type to return lua errors
pub type Result<T> = ::std::result::Result<T, Error>;

/// Nil empty type to implement `CheckLua` on
pub enum Nil {}

/// Table empty type to implement `CheckLua` on
pub enum Table {}

/// Type to perform operations over an underlying `lua_State` safely
#[derive(Debug)]
pub struct LuaState {
    owned: bool,
    pointer: *mut ffi::lua_State,
}

/// Standard libraries
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg(feature = "stdlib")]
pub enum Lib {
    Base,
    Bit32,
    Coroutine,
    Debug,
    Io,
    Math,
    Os,
    Package,
    String,
    Table,
    Utf8,
}

/// Type to configura the garbage collector
#[derive(Debug)]
pub struct LuaGc<'a> {
    state: &'a LuaState,
}

impl<T: IntoLua> IntoLua for Option<T> {
    #[inline]
    unsafe fn into_lua(self, state: &mut LuaState) {
        if let Some(v) = self {
            v.into_lua(state)
        } else {
            ffi::lua_pushnil(state.pointer);
        }
    }
}

impl<'a, T: FromLua<'a>> FromLua<'a> for Option<T> {
    #[inline]
    unsafe fn from_lua(state: &'a LuaState, idx: Index) -> Result<Self> {
        Ok(T::from_lua(state, idx).ok())
    }

    #[inline]
    unsafe fn check(_: &LuaState, _: Index) -> bool {
        true
    }
}

impl CheckLua for Nil {
    #[inline]
    unsafe fn check(state: &LuaState, idx: Index) -> bool {
        ffi::lua_isnil(state.pointer, idx.as_absolute())
    }
}

impl CheckLua for Table {
    #[inline]
    unsafe fn check(state: &LuaState, idx: Index) -> bool {
        ffi::lua_istable(state.pointer, idx.as_absolute())
    }
}

/// Trait for types that can be pushed to the lua stack
pub trait IntoLua {
    /// Push value to the stack.
    ///
    /// This method is unsafe because it doesn't check for available space in the stack.
    unsafe fn into_lua(self, state: &mut LuaState);
}

/// Trait for type checking
pub trait CheckLua {
    /// check the type at the given position
    unsafe fn check(state: &LuaState, idx: Index) -> bool;
}

/// Trait for types that can be read from the lua stack
pub trait FromLua<'a>: Sized {
    /// Read the value at the given index.
    unsafe fn from_lua(&'a LuaState, Index) -> Result<Self>;

    /// Check if the valuea the given index is of this type
    unsafe fn check(&LuaState, Index) -> bool;
}

impl LuaState {
    pub fn new() -> Self {
        unsafe {
            LuaState {
                owned: true,
                pointer: ffi::luaL_newstate(),
            }
        }
    }

    pub fn into_raw(mut self) -> *mut ffi::lua_State {
        self.owned = false;
        self.pointer
    }

    #[inline]
    pub unsafe fn from_raw_parts(state: *mut ffi::lua_State) -> Self {
        LuaState {
            owned: true,
            pointer: state,
        }
    }

    #[inline]
    #[cfg(feature = "stdlib")]
    pub fn open_libs(&mut self) {
        unsafe { ffi::luaL_openlibs(self.pointer) }
    }

    #[inline]
    #[cfg(feature = "stdlib")]
    pub fn open_lib(&mut self, lib: Lib) {
        unsafe {
            match lib {
                Lib::Base => ffi::luaopen_base(self.pointer),
                Lib::Bit32 => ffi::luaopen_bit32(self.pointer),
                Lib::Coroutine => ffi::luaopen_coroutine(self.pointer),
                Lib::Debug => ffi::luaopen_debug(self.pointer),
                Lib::Io => ffi::luaopen_io(self.pointer),
                Lib::Math => ffi::luaopen_math(self.pointer),
                Lib::Os => ffi::luaopen_math(self.pointer),
                Lib::Package => ffi::luaopen_package(self.pointer),
                Lib::String => ffi::luaopen_string(self.pointer),
                Lib::Table => ffi::luaopen_table(self.pointer),
                Lib::Utf8 => ffi::luaopen_utf8(self.pointer),
            };
        }
    }

    pub fn eval<T: AsCStr>(&mut self, source: T) -> Result<()> {
        self.load(source)?;
        self.call_protected(0, ffi::LUA_MULTRET as _)?;
        Ok(())
    }

    pub fn load_from_file<P>(&mut self, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        self.load_from_reader(File::open(path)?)
    }

    pub fn load_from_reader<R: Read>(&mut self, mut read: R) -> Result<()> {
        let mut data = Vec::new();
        read.read_to_end(&mut data)?;
        self.load(data.as_slice())
    }

    pub fn call_protected(&mut self, nargs: usize, nresults: usize) -> Result<()> {
        unsafe {
            match ffi::lua_pcall(self.pointer, nargs as _, nresults as _, 0) as _ {
                ffi::LUA_OK => Ok(()),
                code @ _ => Err(Error::from_lua_result(code as _)),
            }
        }
    }

    pub fn load<T: AsCStr>(&mut self, source: T) -> Result<()> {
        unsafe {
            let source = source.as_cstr();
            match ffi::luaL_loadstring(self.pointer, source.as_ptr() as _) as _ {
                ffi::LUA_OK => Ok(()),
                code @ _ => Err(Error::from_lua_result(code as _)),
            }
        }
    }

    #[inline]
    pub fn pop(&mut self, n: usize) {
        unsafe { ffi::lua_pop(self.pointer, n as _) }
    }

    #[inline]
    pub fn gc(&self) -> LuaGc {
        LuaGc { state: self }
    }

    #[inline]
    pub fn push_function<F: LuaFunction>(&mut self) -> Result<()> {
        unsafe {
            if ffi::lua_checkstack(self.pointer, 1) == 0 {
                Err(Error::Memory)
            } else {
                ffi::lua_pushcfunction(self.pointer, Some(functions::function::<F>));
                Ok(())
            }
        }
    }

    pub fn push_udata<U: LuaUserData>(&mut self, data: U) -> Result<()> {
        self.push(lua_userdata!(data))
    }

    #[inline]
    pub fn push<T: IntoLua>(&mut self, value: T) -> Result<()> {
        unsafe {
            if ffi::lua_checkstack(self.pointer, 1) == 0 {
                Err(Error::Memory)
            } else {
                value.into_lua(self);
                Ok(())
            }
        }
    }

    #[inline]
    pub fn get_udata<T: FromLuaData>(&self, idx: Index) -> Result<&T> {
        unsafe { T::from_lua(self, idx) }
    }

    #[inline]
    pub fn get_udata_mut<T: LuaUserData>(&mut self, idx: Index) -> Result<&mut T> {
        unsafe { T::from_lua_mut(self, idx) }
    }

    #[inline]
    pub fn push_nil(&mut self) -> Result<()> {
        unsafe {
            if ffi::lua_checkstack(self.pointer, 1) == 0 {
                Err(Error::Memory)
            } else {
                ffi::lua_pushnil(self.pointer);
                Ok(())
            }
        }
    }

    #[inline]
    pub fn push_table(&mut self) -> Result<()> {
        unsafe {
            if ffi::lua_checkstack(self.pointer, 1) == 0 {
                Err(Error::Memory)
            } else {
                ffi::lua_newtable(self.pointer);
                Ok(())
            }
        }
    }

    #[inline]
    pub fn is<T: CheckLua>(&self, idx: Index) -> bool {
        unsafe { T::check(self, idx) }
    }

    #[inline]
    pub fn get<'a, T>(&'a self, idx: Index) -> Result<T>
    where
        T: FromLua<'a>,
    {
        unsafe { T::from_lua(self, idx) }
    }

    #[inline]
    pub fn insert(&mut self, idx: Index) {
        unsafe { ffi::lua_insert(self.pointer, idx.as_absolute()) }
    }

    #[inline]
    pub fn replace<I>(&mut self, idx: Index) {
        unsafe { ffi::lua_replace(self.pointer, idx.as_absolute()) }
    }

    #[inline]
    pub fn remove<I>(&mut self, idx: Index)
    {
        unsafe { ffi::lua_remove(self.pointer, idx.as_absolute()) }
    }

    #[inline]
    pub fn raw_len<I>(&self, idx: Index) -> usize
    {
        unsafe { ffi::lua_rawlen(self.pointer, idx.as_absolute()) }
    }

    #[inline]
    pub fn stack_size(&self) -> usize {
        unsafe { ffi::lua_gettop(self.pointer) as _ }
    }

    #[inline]
    pub fn set_global<N: AsCStr>(&mut self, n: N) {
        unsafe { ffi::lua_setglobal(self.pointer, n.as_cstr().as_ptr()) };
    }

    #[inline]
    pub fn get_global<N: AsCStr>(&mut self, n: N) -> Result<()> {
        unsafe {
            if ffi::lua_checkstack(self.pointer, 1) == 0 {
                Err(Error::Memory)
            } else {
                ffi::lua_getglobal(self.pointer, n.as_cstr().as_ptr());
                Ok(())
            }
        }
    }

    #[inline]
    pub fn set_table(&mut self, idx: Index) {
        unsafe { ffi::lua_settable(self.pointer, idx.as_absolute()) };
    }

    #[inline]
    pub fn raw_seti(&mut self, idx: Index, i: i64) {
        unsafe { ffi::lua_rawseti(self.pointer, idx.as_absolute(), i) };
    }
}

impl Drop for LuaState {
    fn drop(&mut self) {
        if self.owned {
            unsafe { ffi::lua_close(self.pointer) }
        }
    }
}

impl<'a> LuaGc<'a> {
    #[inline]
    pub fn collect(&mut self) {
        unsafe { ffi::lua_gc(self.state.pointer, ffi::LUA_GCCOLLECT as _, 0) };
    }

    #[inline]
    pub fn stop(&mut self) {
        unsafe { ffi::lua_gc(self.state.pointer, ffi::LUA_GCSTOP as _, 0) };
    }

    #[inline]
    pub fn restart(&mut self) {
        unsafe { ffi::lua_gc(self.state.pointer, ffi::LUA_GCRESTART as _, 0) };
    }

    #[inline]
    pub fn step(&mut self, arg: i32) {
        unsafe { ffi::lua_gc(self.state.pointer, ffi::LUA_GCSTEP as _, arg) };
    }

    #[inline]
    pub fn set_pause(&mut self, arg: i32) -> i32 {
        unsafe { ffi::lua_gc(self.state.pointer, ffi::LUA_GCSETPAUSE as _, arg) }
    }

    #[inline]
    pub fn set_step_mul(&mut self, arg: i32) -> i32 {
        unsafe { ffi::lua_gc(self.state.pointer, ffi::LUA_GCSETPAUSE as _, arg) }
    }

    #[inline]
    pub fn is_running(&self) -> bool {
        unsafe { ffi::lua_gc(self.state.pointer, ffi::LUA_GCISRUNNING as _, 0) != 0 }
    }

    #[inline]
    pub fn count(&self) -> i32 {
        unsafe { ffi::lua_gc(self.state.pointer, ffi::LUA_GCCOUNT as _, 0) }
    }
}
