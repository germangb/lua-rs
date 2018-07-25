use index::Index;
use {ffi, Error, CheckLua, FromLua, IntoLua, LuaState, Result};

macro_rules! impl_numbers {
    ($($type:ty),+) => {
        $(impl IntoLua for $type {
            #[inline]
            unsafe fn into_lua(self, state: &mut LuaState) {
                ffi::lua_pushinteger(state.pointer, self as _);
            }
        })+
        $(impl CheckLua for $type {
            #[inline]
            unsafe fn check(state: &LuaState, idx: Index) -> bool {
                ffi::lua_isnumber(state.pointer, idx.as_absolute()) == 1
            }
        })+
        $(impl<'a> FromLua<'a> for $type {
            #[inline]
            unsafe fn from_lua(state: &'a LuaState, idx: Index) -> Result<Self> {
                let mut isnum = 0;
                let res = ffi::lua_tonumberx(state.pointer, idx.as_absolute() as _, &mut isnum);
                if isnum == 0 { Err(Error::Type) }
                else { Ok(res as $type) }
            }

            #[inline]
            unsafe fn check(state: &LuaState, idx: Index) -> bool {
                ffi::lua_isnumber(state.pointer, idx.as_absolute() as _) == 1
            }
        })+
    }
}

macro_rules! impl_integers {
    ($($type:ty),+) => {
        $(impl IntoLua for $type {
            #[inline]
            unsafe fn into_lua(self, state: &mut LuaState) {
                ffi::lua_pushnumber(state.pointer, self as _);
            }
        })+
        $(impl CheckLua for $type {
            #[inline]
            unsafe fn check(state: &LuaState, idx: Index) -> bool {
                ffi::lua_isinteger(state.pointer, idx.as_absolute()) == 1
            }
        })+
        $(impl<'a> FromLua<'a> for $type {
            #[inline]
            unsafe fn from_lua(state: &'a LuaState, idx: Index) -> Result<Self> {
                let mut isnum = 0;
                let res = ffi::lua_tointegerx(state.pointer, idx.as_absolute() as _, &mut isnum);
                if isnum == 0 { Err(Error::Type) }
                else { Ok(res as $type) }
            }

            #[inline]
            unsafe fn check(state: &LuaState, idx: Index) -> bool {
                ffi::lua_isinteger(state.pointer, idx.as_absolute() as _) == 1
            }
        })+
    }
}

impl_integers!{
    i8, i16, i32, i64, isize, i128,
    u8, u16, u32, u64, usize, u128
}
impl_numbers!{ f32, f64 }

impl IntoLua for bool {
    #[inline]
    unsafe fn into_lua(self, state: &mut LuaState) {
        if self {
            ffi::lua_pushboolean(state.pointer, 1);
        } else {
            ffi::lua_pushboolean(state.pointer, 0);
        }
    }
}

impl CheckLua for bool {
    #[inline]
    unsafe fn check(state: &LuaState, idx: Index) -> bool {
        ffi::lua_isboolean(state.pointer, idx.as_absolute())
    }
}

impl<'a> FromLua<'a> for bool {
    #[inline]
    unsafe fn from_lua(state: &'a LuaState, idx: Index) -> Result<Self> {
        Ok(ffi::lua_toboolean(state.pointer, idx.as_absolute() as _) == 1)
    }

    #[inline]
    unsafe fn check(state: &LuaState, idx: Index) -> bool {
        ffi::lua_isboolean(state.pointer, idx.as_absolute() as _)
    }
}
