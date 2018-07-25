///// Wraps a generic type into a `LuaUserDataWrapper`.
//#[macro_export]
macro_rules! lua_userdata {
    ($eval:expr) => {
        $crate::userdata::LuaUserDataWrapper($eval)
    };
}

/// Macro that expands into a useful function that loads a group of `LuaFunction`s into a Lua table
/// and assigns it to a global variable.
#[macro_export]
macro_rules! lua_library {
    ( $($fn:ty => $fn_name:expr ),+ ) => {
        pub fn load_lib<N: $crate::ffi::AsCStr>(name: N, state: &mut $crate::LuaState) -> $crate::Result<()> {
            state.push_table()?;

            $(
            state.push($fn_name)?;
            state.push_function::<$fn>()?;
            state.set_table(Index::from(-3));
            )+

            state.set_global(name);
            Ok(())
        }
    }
}
