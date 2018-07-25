use {ffi, Error, IntoLua, LuaState, Result};

use std::fmt;
use std::os::raw;
use std::marker::PhantomData;
use std::fmt::Display;

#[doc(hidden)]
pub struct LuaFunctionWrapper<T>(pub PhantomData<T>);

/// Trait to implement functions that can be called from Lua
pub trait LuaFunction {
    /// Error reported by the function
    type Error: Display;

    /// Implement the call
    fn call(state: &mut LuaState) -> ::std::result::Result<usize, Self::Error>;
}

impl<F> IntoLua for LuaFunctionWrapper<F>
where
    F: LuaFunction,
{
    unsafe fn into_lua(self, state: &mut LuaState) {
        ffi::lua_pushcfunction(state.pointer, Some(function::<F>));
    }
}

extern "C" fn function<F>(state: *mut ffi::lua_State) -> raw::c_int
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
