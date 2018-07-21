extern crate lua;

use lua::prelude::*;

/// Lua library implemented in Rust
mod lib;

fn main() {
    let mut stdin = std::io::stdin();
    let mut source = String::new();

    let mut state = LuaState::new();

    state.open_libs();
    lib::load(&mut state);

    display_splash();

    loop {
        let read = read_line(&mut source) - 1;

        match state.load(&source) {
            Ok(_) => source.clear(),
            Err(Error::Syntax) => {
                if read == 0 {
                    source.clear();
                    eprintln!("ERROR: {:?}", state.get_string(Index::TOP).unwrap());
                }

                state.pop(1);
                continue;
            }
            _ => panic!(),
        }

        match state.call_protected(0, 0) {
            Ok(_) => {}
            Err(Error::Runtime) => {
                eprintln!("ERROR: {:?}", state.get_string(Index::TOP).unwrap());
                state.pop(1);
            }
            Err(err) => panic!("{:?}", err),
        }
    }
}

fn display_splash() {
    println!();
    eprintln!("# Lua shell");
    eprintln!();
    eprintln!("The following Rust functions can be called from the shell:");
    eprintln!("  * rust.error() - Raises a runtime error. The error message is also formatted in rust");
    eprintln!("  * rust.add(a, b) - Returns the sum of `a` and `b`");
    eprintln!("  * rust.len(c) - Returns the length of the string `c`");
    eprintln!();
}

fn read_line(source: &mut String) -> usize {
    if source.is_empty() {
        eprint!("lua> ");
    } else {
        eprint!(".... ");
    }

    let mut line = String::new();
    match io::stdin().read_line(&mut line) {
        Ok(b) => {
            source.push_str(line.as_str());
            b
        },
        _ => 0
    }
}

