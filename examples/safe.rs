extern crate lua;

use lua::{LuaState, Index};

fn main() {
    let mut state = LuaState::new();
    state.open_libs();

    // f32
    state.push_number(0.32);
    state.push_number(1.32);
    state.push_number(2.32);

    // integers
    //state.push_number(42);

    println!("Top: {:?}", state.to_number::<f32>(Index::Top(1)));
    println!("Bottom: {:?}", state.to_number::<f64>(Index::Bottom(1)));
}
