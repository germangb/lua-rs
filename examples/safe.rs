extern crate lua;

use lua::{LuaState, Index};

fn main() {
    let mut state = LuaState::new();
    state.open_libs();

    state.push_value(1.1);
    state.push_value(2.2);
    state.push_value(3.3);

    println!("top: {:?}", state.get_value::<f64>(Index::Top(1)));
    println!("bot: {:?}", state.get_value::<f32>(Index::Bottom(1)));

    // attempt to get top as integer
    println!("bot as int: {:?}", state.get_value::<i32>(Index::Bottom(1)));

    // replace the bottom value with an integer
    state.push_value(3);
    state.replace(Index::Bottom(1));
    println!("bot as int: {:?}", state.get_value::<i32>(Index::Bottom(1)));

    state.push_value("42.0");
    println!("top as int: {:?}", state.get_value::<i32>(Index::Top(1)));
    println!("top as f64: {:?}", state.get_value::<f64>(Index::Top(1)));
    println!("top as str: {:?}", state.get_value::<&str>(Index::Top(1)));
}
