use index::Index;
use {ffi, Error, FromLua, IntoLua, LuaState, Result};

use std::ffi::CString;
use std::os::raw;
use std::rc::Rc;
use std::{mem, ops, ptr};

/// Type to implement Lua userdata, which is an arbitrary block of memory managed by Lua
pub trait LuaUserData {
    const METATABLE: &'static str;
}

#[doc(hidden)]
pub struct LuaUserDataWrapper<T>(pub T);

pub struct Ref<'a, T> {
    state: &'a LuaState,
    pointer: *const T,
}

impl<'a, T> ops::Deref for Ref<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { mem::transmute(self.pointer) }
    }
}

impl<F> LuaUserDataWrapper<F> {
    #[inline]
    pub fn wrap(f: F) -> Self {
        LuaUserDataWrapper(f)
    }
}

impl<D> IntoLua for LuaUserDataWrapper<D>
where
    D: LuaUserData + Clone,
{
    unsafe fn into_lua(&self, state: &mut LuaState) {
        let data = self.0.clone();
        let ptr = ffi::lua_newuserdata(state.pointer, mem::size_of::<D>()) as *mut D;

        let table_name = CString::new(D::METATABLE).unwrap();
        if ffi::luaL_newmetatable(state.pointer, table_name.as_ptr() as _) == 1 {
            ffi::lua_pushstring(state.pointer, "__gc\0".as_ptr() as _);
            ffi::lua_pushcfunction(state.pointer, Some(__gc::<D>));
            ffi::lua_settable(state.pointer, -3);
        }
        ffi::lua_setmetatable(state.pointer, -2);

        ptr::copy(&data, ptr, 1);
        mem::forget(data);
    }
}

impl<'a, D> FromLua<'a> for Ref<'a, D>
where
    D: LuaUserData + Clone,
{
    #[inline]
    unsafe fn from_lua(state: &'a LuaState, idx: Index) -> Result<Self> {
        let pointer = ffi::lua_touserdata(state.pointer, idx.as_absolute()) as *const D;
        if pointer.is_null() {
            return Err(Error::Type);
        }
        Ok(Ref { state, pointer })
    }

    #[inline]
    unsafe fn check(state: &LuaState, idx: Index) -> bool {
        unimplemented!()
    }
}

unsafe extern "C" fn __gc<D>(state: *mut ffi::lua_State) -> raw::c_int {
    ptr::drop_in_place(ffi::lua_touserdata(state, -1) as *mut D);
    0
}
