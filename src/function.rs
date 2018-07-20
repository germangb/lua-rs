use ffi;
use {Index, IntoLua, LuaState};

/// A trait to implement functions that can be called from lua
pub trait LuaFunction {
    type Output: LuaOutput;
    fn call(state: &LuaState) -> Self::Output;
}

/// function output arguments
pub trait LuaOutput {
    fn length() -> usize;
    fn into_lua(self, state: &mut LuaState);
}

macro_rules! impl_out {
    (0) => {
        impl LuaOutput for () {
            #[inline]
            fn length() -> usize { 0 }
            #[inline]
            fn into_lua(self, _: &mut LuaState) {}
        }
    };
    (1, $type:ident . 0) => {
        impl<T: IntoLua> LuaOutput for T {
            #[inline]
            fn length() -> usize { 1 }
            #[inline]
            fn into_lua(self, state: &mut LuaState) {
                state.push_value(self);
            }
        }
    };
    ($len:expr, $($type:ident . $index:tt),+) => {
        impl<$($type: IntoLua ,)*> LuaOutput for ($($type ,)*) {
            #[inline]
            fn length() -> usize { $len }
            #[inline]
            fn into_lua(self, state: &mut LuaState) {
                $(state.push_value(self.$index);)+
            }
        }
    }
}

impl<F> IntoLua for F
where
    F: LuaFunction,
{
    fn into_lua(self, state: &mut LuaState) {
        unsafe {
            ffi::lua_pushcfunction(state.lua_state, Some(function::<F>));

            extern "C" fn function<F>(state: *mut ffi::lua_State) -> ::std::os::raw::c_int
            where
                F: LuaFunction,
            {
                let mut state = LuaState {
                    owned: false,
                    lua_state: state,
                };
                F::call(&state).into_lua(&mut state);
                F::Output::length() as _
            }
        }
    }
}

// implement up to 16 return values...

impl_out!(0);
impl_out!(1, A.0);
impl_out!(2, A.0, B.1);
impl_out!(3, A.0, B.1, C.2);
impl_out!(4, A.0, B.1, C.2, D.3);
impl_out!(5, A.0, B.1, C.2, D.3, E.4);
impl_out!(6, A.0, B.1, C.2, D.3, E.4, F.5);
impl_out!(7, A.0, B.1, C.2, D.3, E.4, F.5, G.6);
impl_out!(8, A.0, B.1, C.2, D.3, E.4, F.5, G.6, H.7);
impl_out!(9, A.0, B.1, C.2, D.3, E.4, F.5, G.6, H.7, I.8);
impl_out!(10, A.0, B.1, C.2, D.3, E.4, F.5, G.6, H.7, I.8, J.9);
impl_out!(11, A.0, B.1, C.2, D.3, E.4, F.5, G.6, H.7, I.8, J.9, K.10);
impl_out!(12, A.0, B.1, C.2, D.3, E.4, F.5, G.6, H.7, I.8, J.9, K.10, L.11);
impl_out!(13, A.0, B.1, C.2, D.3, E.4, F.5, G.6, H.7, I.8, J.9, K.10, L.11, M.12);
impl_out!(14, A.0, B.1, C.2, D.3, E.4, F.5, G.6, H.7, I.8, J.9, K.10, L.11, M.12, N.13);
impl_out!(15, A.0, B.1, C.2, D.3, E.4, F.5, G.6, H.7, I.8, J.9, K.10, L.11, M.12, N.13, O.14);
impl_out!(16, A.0, B.1, C.2, D.3, E.4, F.5, G.6, H.7, I.8, J.9, K.10, L.11, M.12, N.13, O.14, P.15);
