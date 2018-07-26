/// Macro that expands into a function that loads a table of functions into a global variable
#[macro_export]
macro_rules! lua_library {
    ( $($fn:ty => $fn_name:expr ),+ ) => {
        pub fn load_lib<N: $crate::ffi::AsCStr>(name: N, state: &mut $crate::State) -> $crate::Result<()> {
            state.push_table()?;

            $(
            state.push($fn_name)?;
            state.push_function::<$fn>()?;
            state.set_table(Index::Top(3));
            )+

            state.set_global(name);
            Ok(())
        }
    }
}
