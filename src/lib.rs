pub mod error;
pub mod ffi;
pub mod macros;
pub mod prelude;
pub mod string;

use error::Error;
use ffi::AsCStr;
use string::LuaStr;

use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str;

/// Custom type to return lua errors
pub type Result<T> = ::std::result::Result<T, Error>;

/// Type that represents a lua nil
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Nil;

/// Type to represent a generic lua table. NOT a specific one.
#[derive(Debug, Copy, Clone)]
pub struct Table;

/// Used to index the lua stack relative to the Bottom and the Top positions
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Index {
    /// index from the top of the stack
    Top(usize),

    /// index from the bottom of the stack
    Bottom(usize),

    /// Index function arguments. Equivalent to using `Bottom`
    Arg(usize),
}

impl Index {
    /// Top of the stack. Equivalent to `-1`
    pub const TOP: Index = Index::Top(1);

    /// Bottom of the stack. Equivalent to `1`
    pub const BOTTOM: Index = Index::Bottom(1);

    /// Index of the registry table. Equivalent to `LUA_REGISTRYINDEX`
    pub const REGITRY: Index = Index::Top(1001000);

    #[inline]
    pub fn from_absolute(v: ::std::os::raw::c_int) -> Self {
        if v < 0 {
            Index::Top((-v) as _)
        } else {
            Index::Bottom(v as _)
        }
    }

    #[inline]
    pub fn as_absolute(&self) -> ::std::os::raw::c_int {
        match *self {
            Index::Top(i) => {
                let idx = i as ::std::os::raw::c_int;
                -idx
            }
            Index::Bottom(i) => i as _,
            Index::Arg(i) => i as _,
        }
    }
}

/// Type to perform operations over an underlying `lua_State` safely
#[derive(Debug)]
pub struct LuaState {
    owned: bool,
    lua_state: *mut ffi::lua_State,
}

/// Type to configura the garbage collector
#[derive(Debug)]
pub struct LuaGc<'a> {
    state: &'a LuaState,
}

impl Drop for LuaState {
    fn drop(&mut self) {
        if self.owned {
            unsafe { ffi::lua_close(self.lua_state) }
        }
    }
}

/// Trait to obtain rust types from the stack
pub trait FromLua<'a>: Sized {
    /// Attempts to read a value from the stack and returns an optional where `None` means either
    /// nil or the conversion to the desired type couldn't be made.
    fn from_lua(state: &'a LuaState, index: Index) -> Result<Self>;
}

/// Trait to move rust types into the lua stack
pub trait IntoLua {
    /// consumes the value to pushed it into the stack
    fn into_lua(&self, state: &mut LuaState) -> Result<()>;
}

/// A trait to implement functions that can be called from lua
pub trait LuaFn {
    type Error;

    fn call(state: &mut LuaState) -> ::std::result::Result<usize, Self::Error>;
}

macro_rules! impl_numeric {
    (
        $( ( $( $type:ty , )+ ) => $lua_push:ident, $lua_to:ident )+
    ) => {
        $(
            $(impl IntoLua for $type {
                fn into_lua(&self, state: &mut LuaState) -> Result<()> {
                    unsafe {
                        if ffi::lua_checkstack(state.lua_state, 1) == 0 {
                            Err(Error::Memory)
                        } else {
                            ffi::$lua_push(state.lua_state, *self as _);
                            Ok(())
                        }
                    }
                }
            })+

            $(impl<'a> FromLua<'a> for $type {
                fn from_lua(state: &'a LuaState, index: Index) -> Result<Self> {
                    unsafe {
                        let mut isnum = 0;
                        let value = ffi::$lua_to(state.lua_state, index.as_absolute(), &mut isnum);
                        if isnum == 0 { Err(Error::Type) } else { Ok(value as $type) }
                    }
                }
            })+
        )+
    }
}

macro_rules! impl_str {
    () => {
        fn into_lua(&self, state: &mut LuaState) -> Result<()> {
            unsafe {
                if ffi::lua_checkstack(state.lua_state, 1) == 0 {
                    Err(Error::Memory)
                } else {
                    ffi::lua_pushlstring(state.lua_state, self.as_ptr() as _, self.len() as _);
                    Ok(())
                }
            }
        }
    };
    ($($type:ty),+) => {
        $(impl IntoLua for $type {
            impl_str!();
        })+
    };
    ($( ref $type:ty ),+) => {
        $(impl<'a> IntoLua for $type {
            impl_str!();
        })+
    }
}

macro_rules! impl_num {
    ( $( $type:ty ),+ ) => { impl_numeric!{ ( $( $type , )+ ) => lua_pushnumber, lua_tonumberx } }
}

macro_rules! impl_int {
    ( $( $type:ty ),+ ) => { impl_numeric!{ ( $( $type , )+ ) => lua_pushinteger, lua_tointegerx } }
}

impl_num! { f64, f32 }
impl_int! { i64, i32, i16, i8, u64, u32, u16, u8, usize, isize }
impl_str! { ref &'a str, ref &'a String, ref LuaStr<'a> }
impl_str! { String }

impl IntoLua for bool {
    fn into_lua(&self, state: &mut LuaState) -> Result<()> {
        unsafe {
            if ffi::lua_checkstack(state.lua_state, 1) == 0 {
                return Err(Error::Memory)
            }

            if *self {
                ffi::lua_pushboolean(state.lua_state, 1);
            } else {
                ffi::lua_pushboolean(state.lua_state, 0);
            }

            Ok(())
        }
    }
}

impl IntoLua for Nil {
    #[inline]
    fn into_lua(&self, state: &mut LuaState) -> Result<()> {
        unsafe {
            if ffi::lua_checkstack(state.lua_state, 1) == 0 {
                Err(Error::Memory)
            } else {
                ffi::lua_pushnil(state.lua_state);
                Ok(())
            }
        }
    }
}

impl IntoLua for Table {
    #[inline]
    fn into_lua(&self, state: &mut LuaState) -> Result<()> {
        unsafe {
            if ffi::lua_checkstack(state.lua_state, 1) == 0 {
                Err(Error::Memory)
            } else {
                ffi::lua_createtable(state.lua_state, 0, 0);
                Ok(())
            }
        }
    }
}

impl<'a> FromLua<'a> for Nil {
    #[inline]
    fn from_lua(state: &'a LuaState, index: Index) -> Result<Self> {
        if state.is_nil(index) {
            Ok(Nil)
        } else {
            Err(Error::Type)
        }
    }
}

impl<'a> FromLua<'a> for Table {
    #[inline]
    fn from_lua(state: &'a LuaState, index: Index) -> Result<Self> {
        if state.is_table(index) {
            Ok(Table)
        } else {
            Err(Error::Type)
        }
    }
}

impl<T> IntoLua for Option<T>
where
    T: IntoLua,
{
    fn into_lua(&self, state: &mut LuaState) -> Result<()> {
        unsafe {
            if ffi::lua_checkstack(state.lua_state, 1) == 0 {
                return Err(Error::Memory)
            }

            if let Some(ref v) = *self {
                v.into_lua(state)
            } else {
                state.push_nil()
            }
        }
    }
}

impl<'a> FromLua<'a> for bool {
    #[inline]
    fn from_lua(state: &'a LuaState, index: Index) -> Result<Self> {
        unsafe { Ok(ffi::lua_toboolean(state.lua_state, index.as_absolute()) != 0) }
    }
}

impl<F, E> IntoLua for F
where
    E: ::std::fmt::Display,
    F: LuaFn<Error = E>,
{
    fn into_lua(&self, state: &mut LuaState) -> Result<()> {
        unsafe {
            if ffi::lua_checkstack(state.lua_state, 1) == 0 {
                return Err(Error::Memory)
            } else {
                ffi::lua_pushcfunction(state.lua_state, Some(function::<F, E>));
                return Ok(())
            }

            extern "C" fn function<F, E>(state: *mut ffi::lua_State) -> ::std::os::raw::c_int
            where
                E: ::std::fmt::Display,
                F: LuaFn<Error = E>,
            {
                let mut lua_state = LuaState {
                    owned: false,
                    lua_state: state,
                };

                match F::call(&mut lua_state) {
                    Ok(n) => n as _,
                    Err(e) => unsafe {
                        // TODO unwrap
                        lua_state.push_value(format!("{}", e)).unwrap();
                        ffi::lua_error(state);
                        unreachable!()
                    },
                }
            }
        }
    }
}

impl LuaState {
    pub fn new() -> Self {
        unsafe {
            LuaState {
                owned: true,
                lua_state: ffi::luaL_newstate(),
            }
        }
    }

    pub fn into_raw(mut self) -> *mut ffi::lua_State {
        self.owned = false;
        self.lua_state
    }

    pub unsafe fn from_raw_parts(state: *mut ffi::lua_State) -> Self {
        LuaState {
            owned: true,
            lua_state: state,
        }
    }

    #[cfg(feature = "stdlib")]
    pub fn open_libs(&self) {
        unsafe { ffi::luaL_openlibs(self.lua_state) }
    }

    pub fn eval<T>(&mut self, source: T) -> Result<()>
    where
        T: AsCStr,
    {
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

    pub fn load_from_reader<R>(&mut self, mut read: R) -> Result<()>
    where
        R: Read,
    {
        let mut data = Vec::new();
        read.read_to_end(&mut data)?;
        self.load(data.as_slice())
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
        T: AsCStr,
    {
        unsafe {
            let source = source.as_cstr();
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

    pub fn push_value<T>(&mut self, value: T) -> Result<()>
    where
        T: IntoLua,
    {
        value.into_lua(self)
    }

    pub fn get_value<'a, F>(&'a self, index: Index) -> Result<F>
    where
        F: FromLua<'a>,
    {
        F::from_lua(self, index)
    }

    pub fn get_string(&self, index: Index) -> Result<LuaStr> {
        self.get_value(index)
    }

    pub fn push_nil(&mut self) -> Result<()> {
        self.push_value(Nil)
    }

    pub fn is_nil(&self, idx: Index) -> bool {
        unsafe { ffi::lua_isnil(self.lua_state, idx.as_absolute()) }
    }

    pub fn replace(&mut self, idx: Index) {
        unsafe { ffi::lua_replace(self.lua_state, idx.as_absolute()) }
    }

    pub fn remove(&mut self, idx: Index) {
        unsafe { ffi::lua_remove(self.lua_state, idx.as_absolute()) }
    }

    pub fn create_table(&mut self, narr: usize, nrec: usize) -> Result<()> {
        self.push_value(Table)
    }

    pub fn set_global<N>(&mut self, n: N)
    where
        N: AsCStr,
    {
        unsafe {
            let cstr = n.as_cstr();
            ffi::lua_setglobal(self.lua_state, cstr.as_ptr());
        }
    }

    pub fn get_global<N>(&mut self, n: N)
    where
        N: AsCStr,
    {
        unsafe {
            let cstr = n.as_cstr();
            ffi::lua_getglobal(self.lua_state, cstr.as_ptr());
        }
    }

    pub fn new_table(&mut self) {
        self.create_table(0, 0);
    }

    pub fn is_table(&self, idx: Index) -> bool {
        unsafe { ffi::lua_istable(self.lua_state, idx.as_absolute()) }
    }

    pub fn set_table(&mut self, idx: Index) {
        unsafe { ffi::lua_settable(self.lua_state, idx.as_absolute()) };
    }

    pub fn raw_seti(&mut self, idx: Index, i: i64) {
        unsafe { ffi::lua_rawseti(self.lua_state, idx.as_absolute(), i) };
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
