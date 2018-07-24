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

/// Macro that expands into a useful function that loads a group of `LuaFunction`s into a Lua table
/// and assigns it to a global variable.
#[macro_export]
macro_rules! lua_library {
    ( $($fn_struct:ident => $fn_name:expr ),+ ) => {
        pub fn load_lib<N: $crate::ffi::AsCStr>(name: N, state: &mut $crate::LuaState) -> $crate::Result<()> {
            state.push(Table)?;

            $(
            state.push($fn_name)?;
            state.push(lua_function!($fn_struct))?;
            state.set_table(-3);
            )+

            state.set_global(name);
            Ok(())
        }
    }
}
