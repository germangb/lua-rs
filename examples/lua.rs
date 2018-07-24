#[macro_use]
extern crate lua;

mod lib;

use lib::{example, vector};
use lua::prelude::*;

use std::{env, fs, io};

fn main() {
    let mut state = LuaState::new();

    state.open_libs();

    vector::load_lib("vec", &mut state).unwrap();
    example::load_lib("rust", &mut state).unwrap();

    // evaluate file from first argument
    if let Some(file) = env::args().nth(1) {
        state.eval(fs::read(file).unwrap()).unwrap();
    }

    splash();

    let mut stdin = io::stdin();
    let mut source = String::new();

    loop {
        let bytes_read = read_line(&mut stdin, &mut source);

        match (bytes_read, state.eval(&source)) {
            (_, Ok(_)) => source.clear(),
            (b, Err(Error::Syntax)) if b > 0 => state.pop(1),
            (_, Err(Error::Runtime)) | (_, Err(Error::Syntax)) => {
                {
                    let error: &str = state.get(-1).unwrap();
                    eprintln!("ERROR: {:?}", error);
                }
                state.pop(1);
                source.clear();
            }
            _ => panic!(),
        }
    }
}

fn read_line(stdin: &mut io::Stdin, source: &mut String) -> usize {
    if source.is_empty() {
        eprint!("lua> ");
    } else {
        eprint!(".... ");
    }

    let mut line = String::new();
    if let Ok(b) = stdin.read_line(&mut line) {
        source.push_str(line.as_str());
        b - 1
    } else {
        0
    }
}

fn splash() {
    eprintln!("# Welcome to the Lua shell! (written in Rust)");
    eprintln!();
    eprintln!("The following Rust functions can be called from the shell:");
    eprintln!(
        "  * rust.error() - Raises a runtime error. The error message is also formatted in rust"
    );
    eprintln!("  * rust.add(a, b) - Returns the sum of `a` and `b`");
    eprintln!("  * rust.len(c) - Returns the length of the string `c`");
    eprintln!();
}
