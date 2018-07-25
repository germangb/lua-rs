use {ffi, Error, LuaState};

use std::fmt;
use std::os::raw;
use std::fmt::Display;

/// Trait to implement functions that can be called from Lua
pub trait LuaFunction {
    /// Error reported by the function
    type Error: Display;

    /// Implement the call
    fn call(state: &mut LuaState) -> Result<usize, Self::Error>;
}

pub(crate) extern "C" fn function<F>(state: *mut ffi::lua_State) -> raw::c_int
where
    F: LuaFunction,
{
    let mut pointer = LuaState { owned: false, pointer: state };
    match F::call(&mut pointer) {
        Ok(n) => n as raw::c_int,
        Err(e) => unsafe {
            pointer
                .push(format!("{}", e))
                .expect("Unable to push error message");
            ffi::lua_error(state);
            unreachable!()
        }
    }
}
