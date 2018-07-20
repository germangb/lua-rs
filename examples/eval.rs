extern crate lua;

use lua::prelude::*;

struct FooFun;

impl LuaFunction for FooFun {
    fn call(state: &mut LuaState) -> Result<usize, ()> {
        state.push_value("hello");
        Ok(1)
    }
}

fn main() {
    let mut state = LuaState::new();
    state.open_libs();

    // push functions
    state.push_value(FooFun);
    state.set_global("dummy");

    // run rust functions
    state.eval("bar = dummy()").unwrap();
    state.eval("print(bar)");
}
