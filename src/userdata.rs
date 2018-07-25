use index::Index;
use {ffi, Error, CheckLua, IntoLua, LuaState, Result};

use functions;
use functions::LuaFunction;

use ffi::AsCStr;

use std::ffi::CString;
use std::os::raw;
use std::rc::Rc;
use std::{mem, ops, ptr};

/// Type to implement Lua userdata, which is an arbitrary block of memory managed by Lua
pub trait LuaUserData {
    /// Name of the metatable
    const METATABLE: &'static str;

    /// Called only once to register the metamethods of the type.
    fn register(meta: &mut Meta) {}
}

/// Trait to read userdatums from Lua
pub trait FromLuaData {
    /// Get a reference
    unsafe fn from_lua(state: &LuaState, idx: Index) -> Result<&Self>;

    /// Get a mutable reference
    unsafe fn from_lua_mut(state: &mut LuaState, idx: Index) -> Result<&mut Self>;
}

impl<T: LuaUserData> CheckLua for T {
    unsafe fn check(state: &LuaState, idx: Index) -> bool {
        let meta = CString::new(T::METATABLE).unwrap();
        !ffi::luaL_checkudata(state.pointer, idx.as_absolute(), meta.as_ptr() as _).is_null()
    }
}

impl<T: LuaUserData> FromLuaData for T {
    /// Get a reference
    unsafe fn from_lua(state: &LuaState, idx: Index) -> Result<&Self> {
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

    /// Get a mutable reference
    unsafe fn from_lua_mut(state: &mut LuaState, idx: Index) -> Result<&mut Self> {
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

#[doc(hidden)]
pub struct LuaUserDataWrapper<T>(pub T);

/// Type used to register userdata metamethods from the `LuaUserData` trait.
pub struct Meta(*mut ffi::lua_State);

macro_rules! impl_meta {
    (pub enum $name:ident { $( $variant:ident => $meta:expr ,)+ }) => {
        /// Metamethods. All metamethods are supported except for `__gc`, which is implemented by the `Drop` trat.
        #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
        pub enum Metamethod {
            $($variant,)+
        }
        impl Metamethod {
            #[inline]
            fn as_cstr(&self) -> &str {
                match *self {
                    $(Metamethod::$variant => concat!($meta, "\0"),)+
                }
            }
        }
    }
}

impl_meta! {
    pub enum Metamethod {
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

impl Meta {
    /// Register a metamethod
    pub fn set<F>(&mut self, method: Metamethod)
    where
        F: LuaFunction,
    {
        unsafe {
            ffi::lua_pushstring(self.0, method.as_cstr().as_ptr() as _);
            ffi::lua_pushcfunction(self.0, Some(functions::function::<F>));
            ffi::lua_settable(self.0, -3);
        }
    }
}

impl<D> IntoLua for LuaUserDataWrapper<D>
where
    D: LuaUserData,
{
    unsafe fn into_lua(self, state: &mut LuaState) {
        let data = self.0;
        let ptr = ffi::lua_newuserdata(state.pointer, mem::size_of::<D>()) as *mut D;

        let table_name = CString::new(D::METATABLE).unwrap();
        if ffi::luaL_newmetatable(state.pointer, table_name.as_ptr() as _) == 1 {
            ffi::lua_pushstring(state.pointer, "__gc\0".as_ptr() as _);
            ffi::lua_pushcfunction(state.pointer, Some(__gc::<D>));
            ffi::lua_settable(state.pointer, -3);

            let mut meta = Meta(state.pointer);
            D::register(&mut meta);
        }
        ffi::lua_setmetatable(state.pointer, -2);

        ptr::copy(&data, ptr, 1);
        mem::forget(data);

        unsafe extern "C" fn __gc<D>(state: *mut ffi::lua_State) -> raw::c_int {
            ptr::drop_in_place(ffi::lua_touserdata(state, -1) as *mut D);
            0
        }

    }
}
