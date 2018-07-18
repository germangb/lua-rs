extern crate lua;

use lua::{Index, LuaState, LuaSource};

fn main() {
    let mut state = LuaState::new();
    state.open_libs();

    state.push_value("hello, world!");

    // reading a string borrows the LuaState because the pointer is managed by lua
    {
        let value: &str = state.get_value(Index::Top(1)).unwrap();

        println!("value at the top = {}", value);
    }

    state.push_value(42);
    println!("value at the top = {}", state.get_value::<i64>(Index::Top(1)).unwrap());

    state.pop(1);
}
