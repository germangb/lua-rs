use Index;
use {ffi, CheckLua, Error, IntoLua, Result, State};

use functions;
use functions::Function;

use ffi::AsCStr;

use std::ffi::CString;
use std::os::raw;
use std::rc::Rc;
use std::{mem, ops, ptr};

/// Type to implement Lua userdata, which is an arbitrary block of memory managed by Lua
pub trait UserData {
    /// Name of the metatable
    const METATABLE: &'static str;

    /// Called only once to register the metamethods of the type.
    fn register(meta: &mut MetaTable) {}
}

/// Trait to read userdatums from Lua
pub trait FromLua {
    /// Get a reference
    unsafe fn from_lua(state: &State, idx: Index) -> Result<&Self>;
}

/// Trait to read userdatums from Lua that allow mutation
pub trait FromLuaMut {
    /// Get a mutable reference
    unsafe fn from_lua_mut(state: &mut State, idx: Index) -> Result<&mut Self>;
}

impl<T: UserData> CheckLua for T {
    unsafe fn check(state: &State, idx: Index) -> bool {
        let meta = CString::new(T::METATABLE).unwrap();
        !ffi::luaL_checkudata(state.pointer, idx.as_absolute(), meta.as_ptr() as _).is_null()
    }
}

impl<T: UserData> FromLua for T {
    unsafe fn from_lua(state: &State, idx: Index) -> Result<&Self> {
        if ffi::lua_isuserdata(state.pointer, idx.as_absolute()) == 0 {
            return Err(Error::Type);
        }

        let meta = CString::new(T::METATABLE).unwrap();
        let pointer = ffi::luaL_checkudata(state.pointer, idx.as_absolute(), meta.as_ptr() as _);

        if pointer.is_null() {
            return Err(Error::Type);
        }

        Ok(mem::transmute(pointer as *const T))
    }
}

impl<T: UserData> FromLuaMut for T {
    unsafe fn from_lua_mut(state: &mut State, idx: Index) -> Result<&mut Self> {
        if ffi::lua_isuserdata(state.pointer, idx.as_absolute()) == 0 {
            return Err(Error::Type);
        }

        let meta = CString::new(T::METATABLE).unwrap();
        let pointer = ffi::luaL_checkudata(state.pointer, idx.as_absolute(), meta.as_ptr() as _);

        if pointer.is_null() {
            return Err(Error::Type);
        }

        Ok(mem::transmute(pointer as *mut T))
    }
}

/// Type used to register userdata metamethods from the `LuaUserData` trait.
pub struct MetaTable(pub(crate) *mut ffi::lua_State);

macro_rules! impl_meta {
    (
        $(#[$meta_h:meta])*
        pub enum $name:ident {
            $(
                $(#[$meta_v:meta])*
                $variant:ident => $meta:expr ,
            )+
        }
    ) => {

        $(#[$meta_h])*
        pub enum MetaMethod {
            $( $(#[$meta_v])* $variant,)+
        }
        impl MetaMethod {
            #[inline]
            fn as_cstr(&self) -> &str {
                match *self {
                    $(MetaMethod::$variant => concat!($meta, "\0"),)+
                }
            }
        }
    }
}

impl_meta! {
    /// Metamethods. All metamethods are supported except for `__gc`, which is implemented by the `Drop` trat.
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub enum MetaMethod {
        ToString => "__tostring",
        Index => "__index",
        NewIndex => "__newindex",
        Add => "__add",
        Sub => "__sub",
        Mul => "__mul",
        Div => "__div",
        Mod => "__mod",
        Pow => "__pow",
        Unm => "__unm",
        Concat => "__concat",
        Len => "__len",
        Eq => "__eq",
        Lt => "__lt",
        Le => "__le",
        Call => "__call",
    }
}

impl MetaTable {
    /// Register a metamethod
    pub fn set<F>(&mut self, method: MetaMethod)
    where
        F: Function,
    {
        unsafe {
            ffi::lua_pushstring(self.0, method.as_cstr().as_ptr() as _);
            ffi::lua_pushcfunction(self.0, Some(functions::function::<F>));
            ffi::lua_settable(self.0, -3);
        }
    }
}
