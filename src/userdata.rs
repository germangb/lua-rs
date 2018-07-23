use {ffi, Error, IntoLua, LuaState, Result};

/// Type to implement Lua userdata, which is an arbitrary block of memory managed by Lua
pub trait LuaUserData {}

#[doc(hidden)]
pub struct LuaUserDataWrapper<T>(T);
