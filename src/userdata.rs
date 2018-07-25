use index::Index;
use {ffi, Error, FromLua, FromLuaMut, IntoLua, LuaState, Result};
use functions::LuaFunction;

use ffi::AsCStr;

use std::ffi::CString;
use std::os::raw;
use std::rc::Rc;
use std::{mem, ops, ptr};

#[doc(hidden)]
pub struct LuaUserDataWrapper<T>(pub T);

/// Type used to register userdata metamethods from the `LuaUserData` trait.
pub struct Meta(*mut ffi::lua_State);

/// Metamethods. All metamethods are supported except for `__gc`, which is implemented by the `Drop` trat.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Metamethod {
    ToString,
    Index,
    NewIndex,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Unm,
    Concat,
    Len,
    Eq,
    Lt,
    Le,
    Call,
}

impl Metamethod {
    #[inline]
    fn as_cstr(&self) -> &str {
        match *self {
            Metamethod::ToString => "__tostring\0",
            Metamethod::Index => "__index\0",
            Metamethod::NewIndex => "__newindex\0",
            Metamethod::Add => "__add\0",
            Metamethod::Sub => "__sub\0",
            Metamethod::Mul => "__mul\0",
            Metamethod::Div => "__div\0",
            Metamethod::Mod => "__mod\0",
            Metamethod::Pow => "__pow\0",
            Metamethod::Unm => "__unm\0",
            Metamethod::Concat => "__concat\0",
            Metamethod::Len => "__len\0",
            Metamethod::Eq => "__eq\0",
            Metamethod::Lt => "__lt\0",
            Metamethod::Le => "__le\0",
            Metamethod::Call => "__call\0",
        }
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
            ffi::lua_pushcfunction(self.0, Some(__metamethod::<F>));
            ffi::lua_settable(self.0, -3);

            extern "C" fn __metamethod<F>(state: *mut ffi::lua_State) -> raw::c_int
            where
                F: LuaFunction,
            {
                let mut pointer = LuaState {
                    owned: false,
                    pointer: state,
                };

                match F::call(&mut pointer) {
                    Ok(n) => n as _,
                    Err(e) => unsafe {
                        pointer
                            .push(format!("{}", e))
                            .expect("Unable to push error message");
                        ffi::lua_error(state);
                        unreachable!()
                    },
                }
            }
        }
    }
}

/// Type to implement Lua userdata, which is an arbitrary block of memory managed by Lua
pub trait LuaUserData {
    /// Name of the metatable
    const METATABLE: &'static str;

    /// Called only once to register the metamethods of the type.
    fn register(meta: &mut Meta) {}
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

            /*
            ffi::lua_pushstring(state.pointer, "__tostring\0".as_ptr() as _);
            ffi::lua_pushcfunction(state.pointer, Some(__tostring::<D>));
            ffi::lua_settable(state.pointer, -3);
            */

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

        /*
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
        */
    }
}

impl<'a, D> FromLua<'a> for Ref<'a, D>
where
    D: LuaUserData,
{
    #[inline]
    unsafe fn from_lua(state: &'a LuaState, idx: Index) -> Result<Self> {
        //let pointer = ffi::lua_touserdata(state.pointer, idx.as_absolute()) as *const D;
        let meta = CString::new(D::METATABLE).unwrap();
        let pointer = ffi::luaL_checkudata(state.pointer, idx.as_absolute(), meta.as_ptr() as _) as *const D;

        if pointer.is_null() {
            return Err(Error::Type);
        }
        Ok(Ref { state, pointer })
    }

    #[inline]
    unsafe fn check(state: &LuaState, idx: Index) -> bool {
        let meta = CString::new(D::METATABLE).unwrap();
        !ffi::luaL_checkudata(state.pointer, idx.as_absolute(), meta.as_ptr() as _).is_null()
    }
}

impl<'a, D> FromLuaMut<'a> for RefMut<'a, D>
where
    D: LuaUserData,
{
    #[inline]
    unsafe fn from_lua_mut(state: &'a mut LuaState, idx: Index) -> Result<Self> {
        //let pointer = ffi::lua_touserdata(state.pointer, idx.as_absolute()) as *const D;
        let meta = CString::new(D::METATABLE).unwrap();
        let pointer = ffi::luaL_checkudata(state.pointer, idx.as_absolute(), meta.as_ptr() as _) as *mut D;

        if pointer.is_null() {
            Err(Error::Type)
        } else {
            Ok(RefMut { state, pointer })
        }
    }
}
