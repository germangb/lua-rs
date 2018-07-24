#[macro_use]
extern crate lua;

mod lib;

use lib::vector;
use lua::prelude::*;

fn main() {
    let mut lua = LuaState::new();
    lua.open_libs();
    vector::load_lib("vec", &mut lua).unwrap();

    if let Err(_) = lua.eval(include_str!("vector.lua")) {
        let error_msg: &str = lua.get(-1).unwrap();
        println!("ERROR: {}", error_msg);
    }
}
