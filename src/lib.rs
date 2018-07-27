//! ```
//! extern crate lua;
//!
//! use lua::Index;
//!
//! let mut state = lua::State::new();
//! state.open_libs();
//!
//! state.eval(r#"
//!     print ('hello world')
//!
//!     -- define global vars
//!     foo = 42
//! "#).unwrap();
//!
//! state.get_global("foo").unwrap();
//!
//! assert_eq!(Ok(42), state.get(-1));
//! assert_eq!(Ok("42"), state.get(-1));
//! ```
#[macro_use]
mod macros;
/// Error type
pub mod error;
/// Raw bindings from the Lua C API
pub mod ffi;
/// Traits to implement lua functions in Rust
pub mod functions;
/// Types for indexing the stack
pub mod index;
/// Traits to work with user defined Types from lua
pub mod userdata;
/// Implementations of `FromLua` and `IntoLua` for rust primitives.
///
/// ## Example
///
/// ```
/// extern crate lua;
///
/// use lua::Index;
///
/// let mut state = lua::State::new();
///
/// state.push("128").unwrap(); // Index::Top(4)
/// state.push_nil().unwrap();  // Index::Top(3)
/// state.push(16).unwrap();    // Index::Top(2)
/// state.push(true).unwrap();  // Index::Top(1)
///
/// // Numeric
/// assert_eq!(Ok(128), state.get(-4));
/// assert_eq!(Ok(16), state.get(-2));
/// assert_eq!(Ok(16.0), state.get(-2));
///
/// // Booleans return true for any value that is not `nil`
/// assert_eq!(Ok(true), state.get(-1));
/// assert_eq!(Ok(false), state.get(-3));
///
/// // Some values can also be read as strings. Because string
/// // in lua can contain arbitrary binary data, the `FromLua`
/// // trait is implemented for both &str and [u8] slices
/// assert_eq!(Ok("16.0"), state.get(-2));
/// ```
pub mod values;

pub use error::Error;
pub use functions::Function;
pub use index::Index;
pub use userdata::UserData;

use ffi::AsCStr;
use std::{fmt, fs::File, io::Read, os::raw, path::Path, str};

/// Custom type to return lua errors
pub type Result<T> = ::std::result::Result<T, Error>;

/// Type to perform operations over an underlying `lua_State` safely
#[derive(Debug)]
pub struct State {
    owned: bool,
    pointer: *mut ffi::lua_State,
}

/// To configure garbage collection
#[derive(Debug)]
pub struct Gc<'a> {
    state: &'a State,
}

impl<T: IntoLua> IntoLua for Option<T> {
    #[inline]
    unsafe fn into_lua(self, state: &mut State) {
        if let Some(v) = self {
            v.into_lua(state)
        } else {
            ffi::lua_pushnil(state.pointer);
        }
    }
}

impl<'a, T: FromLua<'a>> FromLua<'a> for Option<T> {
    #[inline]
    unsafe fn from_lua(state: &'a State, idx: Index) -> Result<Self> {
        Ok(T::from_lua(state, idx).ok())
    }
}

/// Trait for types that can be pushed to the lua stack
pub trait IntoLua {
    /// Push value to the stack.
    ///
    /// This method is unsafe because it doesn't check for available space in the stack.
    unsafe fn into_lua(self, state: &mut State);
}

/// Trait for type checking
pub trait CheckLua {
    /// check the type at the given position
    unsafe fn check(state: &State, idx: Index) -> bool;
}

/// Trait for types that can be read from the lua stack
pub trait FromLua<'a>: Sized {
    /// Read the value at the given index.
    unsafe fn from_lua(&'a State, Index) -> Result<Self>;
}

impl State {
    pub fn new() -> Self {
        unsafe {
            State {
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
        State {
            owned: true,
            pointer: state,
        }
    }

    #[inline]
    #[cfg(feature = "stdlib")]
    pub fn open_libs(&mut self) {
        unsafe { ffi::luaL_openlibs(self.pointer) }
    }

    pub fn eval<T: AsCStr>(&mut self, source: T) -> Result<()> {
        self.load(source)?;
        self.call_protected(0, ffi::LUA_MULTRET as _)?;
        Ok(())
    }

    pub fn eval_from_reader<R: Read>(&mut self, mut read: R) -> Result<()> {
        let mut data = Vec::new();
        read.read_to_end(&mut data)?;
        self.eval(data.as_slice())
    }

    pub fn eval_from_file<P>(&mut self, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        self.eval_from_reader(File::open(path)?)
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
    pub fn gc(&self) -> Gc {
        Gc { state: self }
    }

    #[inline]
    pub fn push_function<F: Function>(&mut self) -> Result<()> {
        unsafe {
            if ffi::lua_checkstack(self.pointer, 1) == 0 {
                Err(Error::Memory)
            } else {
                ffi::lua_pushcfunction(self.pointer, Some(functions::function::<F>));
                Ok(())
            }
        }
    }

    pub fn push_udata<U: UserData>(&mut self, data: U) -> Result<()> {
        self.push(userdata::LuaUserDataWrapper(data))
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
    pub fn get_udata<T: userdata::FromLua, I: Into<Index>>(&self, idx: I) -> Result<&T> {
        unsafe { T::from_lua(self, idx.into()) }
    }

    #[inline]
    pub fn get_udata_mut<T: userdata::FromLuaMut, I: Into<Index>>(
        &mut self,
        idx: I,
    ) -> Result<&mut T> {
        unsafe { T::from_lua_mut(self, idx.into()) }
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
    pub fn is_function<I: Into<Index>>(&self, idx: I) -> bool {
        unsafe { ffi::lua_isfunction(self.pointer, idx.into().as_absolute()) }
    }

    #[inline]
    pub fn is_udata<I: Into<Index>>(&self, idx: I) -> bool {
        unsafe { ffi::lua_isuserdata(self.pointer, idx.into().as_absolute()) == 1 }
    }

    #[inline]
    pub fn is_light_udata<I: Into<Index>>(&self, idx: I) -> bool {
        unsafe { ffi::lua_islightuserdata(self.pointer, idx.into().as_absolute()) }
    }

    #[inline]
    pub fn is_nil<I: Into<Index>>(&self, idx: I) -> bool {
        unsafe { ffi::lua_isnil(self.pointer, idx.into().as_absolute()) }
    }

    #[inline]
    pub fn is_number<I: Into<Index>>(&self, idx: I) -> bool {
        unsafe { ffi::lua_isnumber(self.pointer, idx.into().as_absolute()) == 1 }
    }

    #[inline]
    pub fn is_integer<I: Into<Index>>(&self, idx: I) -> bool {
        unsafe { ffi::lua_isinteger(self.pointer, idx.into().as_absolute()) == 1 }
    }

    #[inline]
    pub fn is_string<I: Into<Index>>(&self, idx: I) -> bool {
        unsafe { ffi::lua_isstring(self.pointer, idx.into().as_absolute()) == 1 }
    }

    #[inline]
    pub fn is<T: ?Sized + CheckLua, I: Into<Index>>(&self, idx: I) -> bool {
        unsafe { T::check(self, idx.into()) }
    }

    #[inline]
    pub fn get<'a, T, I>(&'a self, idx: I) -> Result<T>
    where
        T: FromLua<'a>,
        I: Into<Index>,
    {
        unsafe { T::from_lua(self, idx.into()) }
    }

    #[inline]
    pub fn insert<I: Into<Index>>(&mut self, idx: I) {
        unsafe { ffi::lua_insert(self.pointer, idx.into().as_absolute()) }
    }

    #[inline]
    pub fn replace<I: Into<Index>>(&mut self, idx: I) {
        unsafe { ffi::lua_replace(self.pointer, idx.into().as_absolute()) }
    }

    #[inline]
    pub fn remove<I: Into<Index>>(&mut self, idx: I) {
        unsafe { ffi::lua_remove(self.pointer, idx.into().as_absolute()) }
    }

    #[inline]
    pub fn raw_len<I: Into<Index>>(&self, idx: I) -> usize {
        unsafe { ffi::lua_rawlen(self.pointer, idx.into().as_absolute()) }
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
    pub fn set_table<I: Into<Index>>(&mut self, idx: I) {
        unsafe { ffi::lua_settable(self.pointer, idx.into().as_absolute()) };
    }

    #[inline]
    pub fn raw_seti<I: Into<Index>>(&mut self, idx: I, i: i64) {
        unsafe { ffi::lua_rawseti(self.pointer, idx.into().as_absolute(), i) };
    }
}

macro_rules! std_libs {
    ( $(#[$meta:meta])* pub enum Lib { $( $(#[$meta_var:meta])* $variant:ident => $loader:path ,)+ }) => {
        $(#[$meta])+
        pub enum Lib {
            $($(#[$meta_var:meta])* $variant,)+
        }
        impl State {
            #[inline]
            #[cfg(feature = "stdlib")]
            pub fn open_lib(&mut self, lib: Lib) {
                unsafe {
                    match lib {
                        $( Lib::$variant => $loader(self.pointer), )+
                    };
                }
            }
        }
    }
}

std_libs! {
    /// Standard libraries
    #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
    #[cfg(feature = "stdlib")]
    pub enum Lib {
        Base => ffi::luaopen_base,
        Bit32 => ffi::luaopen_bit32,
        Coroutine => ffi::luaopen_coroutine,
        Debug => ffi::luaopen_debug,
        Io => ffi::luaopen_io,
        Math => ffi::luaopen_math,
        Os => ffi::luaopen_math,
        Package => ffi::luaopen_package,
        String => ffi::luaopen_string,
        Table => ffi::luaopen_table,
        Utf8 => ffi::luaopen_utf8,
    }
}

impl Drop for State {
    fn drop(&mut self) {
        if self.owned {
            unsafe { ffi::lua_close(self.pointer) }
        }
    }
}

impl<'a> Gc<'a> {
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
