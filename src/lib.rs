extern crate num_traits;

pub mod ffi;

use num_traits::Num;
use num_traits::FromPrimitive;
use num_traits::NumCast;
use num_traits::float::Float;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum LuaLib {
    Base,
    Bit,
    Coroutine,
    Debug,
    Io,
    Math,
    Package,
    Os,
    Str,
    Table,
    Utf8,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Index {
    /// index from the top of the stack
    Top(usize),

    /// index from the bottom of the stack
    Bottom(usize),
}

impl Index {
    #[inline(always)]
    pub fn as_absolute(&self) -> ::std::os::raw::c_int {
        match *self {
            Index::Top(i) => {
                let idx = i as ::std::os::raw::c_int;
                -idx
            },
            Index::Bottom(i) => i as _,
        }
    }
}

#[derive(Debug)]
pub struct LuaState {
    lua_state: *mut ffi::lua_State,
}

impl Drop for LuaState {
    fn drop(&mut self) {
        unsafe { ffi::lua_close(self.lua_state) }
    }
}

pub trait ToNumber: Num + NumCast {
    fn to_number(state: &LuaState, idx: Index) -> Option<Self> {
        unsafe {
            let mut result = 0;
            let value = ffi::lua_tonumberx(state.lua_state, idx.as_absolute(), &mut result);

            if result == 0 {
                None
            } else {
                <Self as NumCast>::from(value)
            }
        }
    }
}

pub trait PushNumber: Float {
    fn push_number(self, state: &mut LuaState) {
        unsafe {
            let value = 0.0;
            ffi::lua_pushnumber(state.lua_state, value)
        }
    }
}

macro_rules! impl_number {
    ( $($type:ty),+ ) => {
        $(
            impl PushNumber for $type {}
            impl ToNumber for $type {}
        )+
    }
}

impl_number! { f64, f32 }
//impl_number! { lua_pushinteger, i8, i16, i32, i64, u8, u16, u32, u64, usize, isize }

impl LuaState {
    /// Creata a bare bones lua VM
    pub fn new() -> Self {
        unsafe {
            LuaState {
                lua_state: ffi::luaL_newstate(),
            }
        }
    }

    /// Consumes this `LuaState` and closes the VM
    pub fn close(self) {}

    /// Pop n values from the stack
    pub fn pop(&mut self, n: usize) {
        unsafe { ffi::lua_pop(self.lua_state, n as _) }
    }

    /// Push a new number to the top of the stack
    pub fn push_number<N: PushNumber>(&mut self, value: N) {
        value.push_number(self)
    }
 
    /// Returns an optional of the number value at the given index. Equivalent to
    /// `ffi::lua_tonumber`
    pub fn to_number<N: ToNumber>(&self, idx: Index) -> Option<N> {
        N::to_number(self, idx)
    }

    /// Push nil to the top of the stack. Equivalent to `ffi::lua_pushnil()`
    pub fn push_nil(&mut self) {
        unsafe { ffi::lua_pushnil(self.lua_state) }
    }

    /// Returns true if the value at the given index is nil
    pub fn is_nil(&self, idx: Index) -> bool {
        unsafe { ffi::lua_isnil(self.lua_state, idx.as_absolute()) }
    }

    /// Load standard libraries
    pub fn open_libs(&self) {
        unsafe { ffi::luaL_openlibs(self.lua_state) }
    }
}
