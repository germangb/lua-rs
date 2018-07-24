#[macro_use]
extern crate lua;

use lua::prelude::*;

use std::rc::Rc;

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

struct InspectFunc;

impl LuaFunction for InspectFunc {
    type Error = Error;

    fn call(state: &mut LuaState) -> Result<usize, Self::Error> {
        {
            let arg0: Ref<UserData> = state.get(1)?;

            println!("{:?}", *arg0);
        }
        state.push("hello")?;
        Ok(1)
    }
}

#[derive(Debug)]
struct Field;

impl Drop for Field {
    fn drop(&mut self) {
        println!("Dropping Field");
    }
}

#[derive(Clone, Debug)]
struct UserData {
    bar: Rc<Field>,
    baz: String,
}

impl LuaUserData for UserData {
    const METATABLE: &'static str = "rust.UserData";
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

    {
        let data = UserData { bar: Rc::new(Field), baz: String::from("hello!") };
        state.push(lua_userdata!(data)).unwrap();
        state.set_global("udata0");

        println!("Global variable is set");
    }
    //state.pop(1);

    state.push(lua_function!(TestFunc)).unwrap();
    state.set_global("crash");

    state.push(lua_function!(InspectFunc)).unwrap();
    state.set_global("inspect");

    {
        state.push("hello rust");

        let line: &str = state.get(-1).unwrap();
        //println!("{}", line);
    }
    state.pop(1);

    // ---

    //state.eval("print(string)").unwrap();
    //state.eval("print(crash('4'))").unwrap();
    state.eval("print(inspect(udata0))").unwrap();

    println!("Finishing app");
}
