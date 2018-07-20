/// Macro to create a nul terminated string literal.
///
/// ```
/// assert_eq!("hello\0", lua_str!("hello"));
/// ```
#[macro_export]
macro_rules! lua_str {
    ($line:expr) => {
        concat!($line, "\0")
    };
}
