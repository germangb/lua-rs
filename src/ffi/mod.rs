#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

include!("bindgen.rs");
include!("lua_macros.rs");
include!("lauxlib_macros.rs");

use std::ffi::{CStr, CString};
use std::borrow::Cow;

/// Trait to convert types into nul-terminated strings, for when they need to be passed to API
/// functions such as `lua_setglobal` or `lua_getglobal`.
pub trait AsCStr<'a> {
    fn as_cstr(&'a self) -> Cow<'a, CStr>;
}
 
impl<'a> AsCStr<'a> for str {
    fn as_cstr(&'a self) -> Cow<'a, CStr> {
        if let Some(0) = self.as_bytes().last() {
            unsafe {
                Cow::Borrowed(CStr::from_ptr(self.as_ptr() as _))
            }
        } else {
            Cow::Owned(CString::new(self).unwrap())
        }
    }
}

impl<'a> AsCStr<'a> for &'a str {
    fn as_cstr(&'a self) -> Cow<'a, CStr> {
        if let Some(0) = self.as_bytes().last() {
            unsafe {
                Cow::Borrowed(CStr::from_ptr(self.as_ptr() as _))
            }
        } else {
            Cow::Owned(CString::new(*self).unwrap())
        }
    }
}
