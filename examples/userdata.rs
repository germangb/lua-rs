#[macro_use]
extern crate lua;

use lua::prelude::*;

struct DebugFoo;

impl LuaFunction for DebugFoo {
    type Error = Error;

    fn call(state: &mut LuaState) -> Result<usize, Error> {
        let debug = {
            let data: Ref<Foo> = state.get(1)?;
            format!("{:?}", *data)
        };

        state.push(debug)?;
        Ok(1)
    }
}

#[derive(Debug)]
struct Foo {
    bar: i32,
    baz: String,
}

impl LuaUserData for Foo {
    const METATABLE: &'static str = "Example.foo";
}

fn main() {
    let mut state = LuaState::new();
    state.open_libs();

    let data = Foo {
        bar: 32,
        baz: String::from("hello world"),
    };

    state.push(lua_userdata!(data)).unwrap();
    state.set_global("foo");

    state.push(lua_function!(DebugFoo)).unwrap();
    state.set_global("debug");

    state.eval("print(foo)").unwrap();
}
