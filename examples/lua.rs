extern crate lua;

use std::io;
use std::io::Read;

use lua::{LuaState, LuaSource, LuaError, Index};

fn main() {
    let mut stdin = io::stdin();

    let mut lua_source = LuaSource::new();
    let mut lua_state = LuaState::new();
    lua_state.open_libs();

    eprintln!("Welcome to Lua in Rust!");
    loop {
        if lua_source.is_empty() {
            eprint!("> ");
        } else {
            eprint!("..");
        }

        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Err(_) => break,
            Ok(_) => lua_source.extend(&line),
        }

        match lua_state.load(&lua_source) {
            Ok(_) => {},
            Err(LuaError::Syntax) => {
                if !line.trim().is_empty() {
                    continue
                }
            },
            _ => panic!(),
        }

        //eprintln!("code so far:\n===\n{:?}\n===", lua_source);

        match lua_state.call_protected(0, 0) {
            Ok(_) => {},
            Err(LuaError::Runtime) => {
                //eprintln!("runtime error");
                {
                    let error: &str = lua_state.get_value(Index::Top(1)).unwrap();
                    eprintln!("{}", error);
                }
                lua_state.pop(1);
            },
            _ => panic!(),
        }

        lua_source.clear();
    }
}
