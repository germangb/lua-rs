use {ffi, Error, IntoLua, LuaState, Result};

#[doc(hidden)]
pub struct LuaStringWrapper<T>(T);

impl<T> LuaStringWrapper<T> {
    #[inline]
    pub fn wrap(w: T) -> Self {
        LuaStringWrapper(w)
    }
}

impl<T> IntoLua for LuaStringWrapper<T>
where
    T: AsRef<[u8]>,
{
    #[inline]
    unsafe fn into_lua(&self, state: &mut LuaState) {
        let string = self.0.as_ref();
        ffi::lua_pushlstring(state.pointer, string.as_ptr() as _, string.len() as _);
    }
}
