use {ffi, Error, IntoLua, LuaState, Result};

use std::fmt;
use std::os::raw;

#[doc(hidden)]
pub struct LuaFunctionWrapper<T>(T);

impl<F> LuaFunctionWrapper<F> {
    #[inline]
    pub fn wrap(f: F) -> Self {
        LuaFunctionWrapper(f)
    }
}

/// Trait to implement functions that can be called from Lua
pub trait LuaFunction {
    /// Error reported by the function
    type Error;

    /// Implement the call
    fn call(state: &mut LuaState) -> ::std::result::Result<usize, Self::Error>;
}

impl<F, E> IntoLua for LuaFunctionWrapper<F>
where
    E: fmt::Display,
    F: LuaFunction<Error = E>,
{
    unsafe fn into_lua(&self, state: &mut LuaState) {
        ffi::lua_pushcfunction(state.pointer, Some(function::<F, E>));
    }
}

extern "C" fn function<F, E>(state: *mut ffi::lua_State) -> raw::c_int
where
    E: fmt::Display,
    F: LuaFunction<Error = E>,
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
