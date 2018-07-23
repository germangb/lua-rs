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
