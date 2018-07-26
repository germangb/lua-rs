#[macro_use]
extern crate lua;

mod lib;

use lib::vector;

use lua::{State, Index};

fn main() {
    let mut lua = State::new();
    lua.open_libs();
    vector::load_lib("vec", &mut lua).unwrap();

    if let Err(_) = lua.eval(include_str!("vector.lua")) {
        let error_msg: &str = lua.get(Index::TOP).unwrap();
        println!("ERROR: {}", error_msg);
    }
}
