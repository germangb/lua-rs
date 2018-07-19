extern crate lua;

use std::io;
use std::io::Read;

use lua::prelude::*;

fn main() {
    let mut stdin = io::stdin();

    let mut lua_source = String::new();
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
            Ok(_) => lua_source.push_str(line.as_str()),
        }

        match lua_state.load(&lua_source) {
            Ok(_) => lua_source.clear(),
            Err(Error::Syntax) => {
                if line.trim().is_empty() {
                    eprintln!("Syntax error\n===");
                    unsafe {
                        eprintln!(
                            "{}",
                            lua_state
                                .get_string(Index::Top(1))
                                .unwrap()
                                .as_str_unchecked()
                        );
                    }
                    lua_source.clear();
                }

                lua_state.pop(1);
                continue;
            }
            _ => panic!(),
        }

        //eprintln!("code so far:\n===\n{:?}\n===", lua_source);

        match lua_state.call_protected(0, 0) {
            Ok(_) => {}
            Err(Error::Runtime) => {
                eprintln!("Ruintime error\n===");
                unsafe {
                    eprintln!(
                        "{}",
                        lua_state
                            .get_string(Index::Top(1))
                            .unwrap()
                            .as_str_unchecked()
                    );
                }
                lua_state.pop(1);
            }
            _ => panic!(),
        }
    }
}
