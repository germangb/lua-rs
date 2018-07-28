use {ffi, CheckLua, Error, Index, IntoLua, Result, State};

use functions;
use functions::Function;

use ffi::AsCStr;

use std::ffi::CString;
use std::marker::PhantomData;
use std::os::raw;
use std::rc::Rc;
use std::{mem, ops, ptr};

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

/// Type to implement Lua userdata, which is an arbitrary block of memory managed by Lua
pub trait UserData: Sized {
    /// Return the name of the metatable
    unsafe fn metatable_name() -> &'static str;

    /// Register the metamethods
    fn register(meta: &mut MetaTable<Self>) {}
}

impl_meta! {
    /// Metamethods.
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub enum MetaMethod {
        /// This metamethod is called just before dropping the userdatum.
        Gc => "__gc",
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
        //let meta = CString::new(T::METATABLE).unwrap();
        let meta = CString::new(T::metatable_name()).unwrap();
        !ffi::luaL_checkudata(state.pointer, idx.as_absolute(), meta.as_ptr() as _).is_null()
    }
}

impl<T: UserData> FromLua for T {
    unsafe fn from_lua(state: &State, idx: Index) -> Result<&Self> {
        if ffi::lua_isuserdata(state.pointer, idx.as_absolute()) == 0 {
            return Err(Error::Type);
        }

        let meta = CString::new(T::metatable_name()).unwrap();
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

        let meta = CString::new(T::metatable_name()).unwrap();
        let pointer = ffi::luaL_checkudata(state.pointer, idx.as_absolute(), meta.as_ptr() as _);

        if pointer.is_null() {
            return Err(Error::Type);
        }

        Ok(mem::transmute(pointer as *mut T))
    }
}

/// Type used to register userdata metamethods from the `LuaUserData` trait.
pub struct MetaTable<U: UserData>(pub(crate) *mut ffi::lua_State, pub(crate) PhantomData<U>);

impl<U: UserData> MetaTable<U> {
    /// Register a metamethod
    pub fn set<F>(&mut self, method: MetaMethod)
    where
        F: Function,
    {
        unsafe {
            ffi::lua_pushstring(self.0, method.as_cstr().as_ptr() as _);
            if method == MetaMethod::Gc {
                ffi::lua_pushcfunction(self.0, Some(functions::function_gc::<F, U>));
            } else {
                ffi::lua_pushcfunction(self.0, Some(functions::function::<F>));
            }
            ffi::lua_settable(self.0, -3);
        }
    }
}
