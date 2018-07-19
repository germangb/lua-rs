extern crate lua;

use lua::prelude::*;

fn main() {
    let mut state = LuaState::new();
    state.open_libs();

    // define global from rust
    state.push_value(42);
    state.set_global("foo");

    state.eval("print(foo)").unwrap();

    // define global from lua
    state.eval("bar = 2.3").unwrap();

    state.get_global("bar");
    println!("bar = {}", state.get_value::<f64>(Index::TOP).unwrap());
}
