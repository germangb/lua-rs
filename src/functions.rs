use {ffi, Error, State};

use std::fmt;
use std::fmt::Display;
use std::os::raw;

/// Trait to implement functions that can be called from Lua
pub trait Function {
    /// Error reported by the function
    type Error: Display;

    /// Implement the call
    fn call(state: &mut State) -> Result<usize, Self::Error>;
}

pub(crate) extern "C" fn function<F>(state: *mut ffi::lua_State) -> raw::c_int
where
    F: Function,
{
    let mut pointer = State {
        owned: false,
        pointer: state,
    };
    match F::call(&mut pointer) {
        Ok(n) => n as raw::c_int,
        Err(e) => unsafe {
            pointer
                .push(format!("{}", e))
                .expect("Unable to push error message");
            ffi::lua_error(state);
            unreachable!()
        },
    }
}
