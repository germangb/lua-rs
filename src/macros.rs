/// Wraps a generic Rust value into a `LuaString`.
#[macro_export]
macro_rules! lua_string {
    ($eval:expr) => {
        $crate::strings::LuaStringWrapper::wrap($eval)
    };
}

/// Wraps a generic type into a `LuaFunctionWrapper`.
#[macro_export]
macro_rules! lua_function {
    ($eval:expr) => {
        $crate::functions::LuaFunctionWrapper::wrap($eval)
    };
}

/// Wraps a generic type into a `LuaUserDataWrapper`.
#[macro_export]
macro_rules! lua_userdata {
    ($eval:expr) => {
        $crate::userdata::LuaUserDataWrapper::wrap($eval)
    };
}
