use index::Index;
use {ffi, Error, FromLua, FromLuaMut, IntoLua, LuaState, Result};

use std::ffi::CString;
use std::os::raw;
use std::rc::Rc;
use std::{fmt, mem, ops, ptr};

#[doc(hidden)]
pub struct LuaUserDataWrapper<T>(pub T);

/// Type to implement Lua userdata, which is an arbitrary block of memory managed by Lua
pub trait LuaUserData {
    /// Name of the metatable
    const METATABLE: &'static str;
}

/// Immutable reference to a userdatum
pub struct Ref<'a, T> {
    state: &'a LuaState,
    pointer: *const T,
}

/// Reference to a userdatum that allows mutation
pub struct RefMut<'a, T> {
    state: &'a LuaState,
    pointer: *mut T,
}

impl<'a, T> ops::Deref for Ref<'a, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { mem::transmute(self.pointer) }
    }
}

impl<'a, T> ops::Deref for RefMut<'a, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { mem::transmute(self.pointer) }
    }
}

impl<'a, T> ops::DerefMut for RefMut<'a, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
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
    D: LuaUserData + fmt::Debug,
{
    unsafe fn into_lua(self, state: &mut LuaState) {
        let data = self.0;
        let ptr = ffi::lua_newuserdata(state.pointer, mem::size_of::<D>()) as *mut D;

        let table_name = CString::new(D::METATABLE).unwrap();
        if ffi::luaL_newmetatable(state.pointer, table_name.as_ptr() as _) == 1 {
            ffi::lua_pushstring(state.pointer, "__gc\0".as_ptr() as _);
            ffi::lua_pushcfunction(state.pointer, Some(__gc::<D>));
            ffi::lua_settable(state.pointer, -3);

            ffi::lua_pushstring(state.pointer, "__tostring\0".as_ptr() as _);
            ffi::lua_pushcfunction(state.pointer, Some(__tostring::<D>));
            ffi::lua_settable(state.pointer, -3);
        }
        ffi::lua_setmetatable(state.pointer, -2);

        ptr::copy(&data, ptr, 1);
        mem::forget(data);

        unsafe extern "C" fn __gc<D>(state: *mut ffi::lua_State) -> raw::c_int {
            ptr::drop_in_place(ffi::lua_touserdata(state, -1) as *mut D);
            0
        }

        unsafe extern "C" fn __tostring<D>(state: *mut ffi::lua_State) -> raw::c_int
        where
            D: LuaUserData + fmt::Debug,
        {
            let mut state = LuaState {
                owned: false,
                pointer: state,
            };

            let repr = {
                let data: Ref<D> = state.get(1).unwrap();
                format!("{:?}", *data)
            };

            state.push(repr);
            1
        }
    }
}

impl<'a, D> FromLua<'a> for Ref<'a, D>
where
    D: LuaUserData,
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

impl<'a, D> FromLuaMut<'a> for RefMut<'a, D>
where
    D: LuaUserData,
{
    #[inline]
    unsafe fn from_lua_mut(state: &'a mut LuaState, idx: Index) -> Result<Self> {
        let ptr = ffi::lua_touserdata(state.pointer, idx.as_absolute());

        if ptr.is_null() {
            Err(Error::Type)
        } else {
            Ok(RefMut {
                state,
                pointer: ptr as *mut D,
            })
        }
    }
}
