extern crate lua;

use lua::ffi::AsCStr;

#[test]
fn test_as_cstr() {
    let non_nul = "hello, world!";
    let nul = "hello, world!\0";
    assert_eq!(nul.as_ptr(), nul.as_cstr().as_ptr() as _);
    assert_ne!(non_nul.as_ptr(), non_nul.as_cstr().as_ptr() as _);

    let non_nul = "hello, world!".to_string();
    let nul = "hello, world!\0".to_string();
    assert_eq!(nul.as_ptr(), nul.as_cstr().as_ptr() as _);
    assert_ne!(non_nul.as_ptr(), non_nul.as_cstr().as_ptr() as _);
}
