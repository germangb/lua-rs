extern crate lua;

use lua::prelude::*;

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
        state.get_value::<i64>(Index::Top(1)).unwrap()
    );

    state.push_value("Value defined from Rust!");
    state.set_global("foo");

    state
        .eval("print('The following value is defined from Rust:', foo)")
        .unwrap();
    state.pop(1);
}
