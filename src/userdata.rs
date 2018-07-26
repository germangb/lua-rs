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

#[doc(hidden)]
pub struct LuaUserDataWrapper<T>(pub T);

/// Type used to register userdata metamethods from the `LuaUserData` trait.
pub struct MetaTable(*mut ffi::lua_State);

macro_rules! impl_meta {
    (pub enum $name:ident { $( $variant:ident => $meta:expr ,)+ }) => {
        /// Metamethods. All metamethods are supported except for `__gc`, which is implemented by the `Drop` trat.
        #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
        pub enum MetaMethod {
            $($variant,)+
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

impl<D> IntoLua for LuaUserDataWrapper<D>
where
    D: UserData,
{
    unsafe fn into_lua(self, state: &mut State) {
        let data = self.0;
        let ptr = ffi::lua_newuserdata(state.pointer, mem::size_of::<D>()) as *mut D;

        let table_name = CString::new(D::METATABLE).unwrap();
        if ffi::luaL_newmetatable(state.pointer, table_name.as_ptr() as _) == 1 {
            ffi::lua_pushstring(state.pointer, "__gc\0".as_ptr() as _);
            ffi::lua_pushcfunction(state.pointer, Some(__gc::<D>));
            ffi::lua_settable(state.pointer, -3);

            let mut meta = MetaTable(state.pointer);
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
