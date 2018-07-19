#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

include!("bindgen.rs");
include!("lua_macros.rs");
include!("lauxlib_macros.rs");

use std::borrow::Cow;
use std::ffi::{CStr, CString};

/// Trait to convert types into nul-terminated strings, for when they need to be passed to API
/// functions such as `lua_setglobal` or `lua_getglobal`.
pub trait AsCStr {
    fn as_cstr(&self) -> Cow<CStr>;
}

impl<T> AsCStr for T
where
    T: AsRef<str>,
{
    fn as_cstr(&self) -> Cow<CStr> {
        let string = self.as_ref();
        if let Some(0) = string.as_bytes().last() {
            unsafe { Cow::Borrowed(CStr::from_ptr(string.as_ptr() as _)) }
        } else {
            Cow::Owned(CString::new(string).unwrap())
        }
    }
}
