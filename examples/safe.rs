#[macro_use]
extern crate lua;

use lua::prelude::*;

// Lua function
struct LuaMax;

impl LuaFunction for lua_max {
    type Input = (i64, i64);
    type Output = (i64);
}

fn lua_max(args: (i64, i64)) -> (i64) {
    if args.0 > args.1
}

fn main() {
    let mut state = LuaState::new();
    state.open_libs();

    state.push_value("hello, world!");

    // reading a string borrows the LuaState because the pointer is managed by lua
    {
        let value = state.get_string(Index::Top(1)).unwrap();

        println!("value at the top = {:?}", value.as_str());
    }

    state.push_value(42);
    println!(
        "value at the top = {}",
        state.get_value::<i64>(Index::TOP).unwrap()
    );

    state.push_value("Value defined from Rust!");
    state.set_global(lua_str!("foo"));

    state
        .eval("print('The following value is defined from Rust:', foo)")
        .unwrap();
}
