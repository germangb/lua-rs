use lua::prelude::*;

pub struct FnError; // raises a lua runtime error
pub struct FnAdd;   // adds two integers
pub struct FnLen;   // returns the length of a string

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

pub fn load(state: &mut LuaState) -> Result<(), Error> {
    state.push(Table)?;
    state.push("error");
    state.push(lua_function!(FnError))?;
    state.set_table(-3);
    state.push("add")?;
    state.push(lua_function!(FnAdd))?;
    state.set_table(-3);
    state.push("len")?;
    state.push(lua_function!(FnLen))?;
    state.set_table(-3);
    state.set_global("rust");
    Ok(())
}

