#[macro_use]
extern crate lua;

use lua::prelude::*;

struct TestFunc;

impl LuaFunction for TestFunc {
    type Error = Error;

    fn call(state: &mut LuaState) -> Result<usize, Self::Error> {
        let n: i32 = state.get(1)?;
        state.push(n*n)?;
        //state.push(Nil)?;
        Ok(1)
    }
}

fn main() {
    let mut state = LuaState::new();
    state.open_libs();

    // push numbers
    state.push(42).unwrap();
    state.set_global("number");

    // push strings
    state.push("hello world!").unwrap();
    state.set_global("string");

    state.push(lua_function!(TestFunc)).unwrap();
    state.set_global("crash");

    {
        state.push("hello rust");

        let line: &str = state.get(-1).unwrap();
        println!("{}", line);
    }
    state.pop(1);

    // ---
    state.eval("print(string)").unwrap();
    state.eval("print(crash('4'))").unwrap();
}
