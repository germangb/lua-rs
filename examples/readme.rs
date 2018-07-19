extern crate lua;

use lua::prelude::*;

fn main() {
    let mut state = LuaState::new();

    state.push_value(42);
    assert_eq!(Some(42.0), state.get_value::<f64>(Index::TOP));

    state.push_value(16);
    assert_eq!(Some(16), state.get_value::<i64>(Index::TOP));

    state.push_value("hello");
    assert_eq!(None, state.get_value::<i32>(Index::TOP));

    state.push_nil();
    assert!(state.is_nil(Index::TOP));
}
