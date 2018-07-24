use lua::prelude::*;

pub struct FnError; // raises a lua runtime error
pub struct FnAdd;   // adds two integers
pub struct FnLen;   // returns the length of a string

lua_library! {
    FnError => "error",
    FnAdd => "add",
    FnLen => "len"
}

impl LuaFunction for FnError {
    type Error = &'static str;

    fn call(_: &mut LuaState) -> Result<usize, Self::Error> {
        Err("This is a rust runtime error")
    }
}

impl FnAdd {
    fn add(a: i32, b: i32) -> i32 {
        a+b
    }
}

impl LuaFunction for FnAdd {
    type Error = Error;

    fn call(state: &mut LuaState) -> Result<usize, Self::Error> {
        let a = state.get(1)?;
        let b = state.get(2)?;
        state.push(Self::add(a, b));
        Ok(1)
    }
}

impl LuaFunction for FnLen {
    type Error = Error;

    fn call(state: &mut LuaState) -> Result<usize, Self::Error> {
        let length = {
            let s: &str = state.get(1)?;
            s.len()
        };

        state.push(length);
        Ok(1)
    }
}
