#[macro_use]
extern crate lua;

mod lib;

use lib::example;
use lua::prelude::*;
use std::{fs, io, env};

fn main() {
    let mut stdin = io::stdin();
    let mut source = String::new();

    let mut state = LuaState::new();

    state.open_libs();
    example::load(&mut state).unwrap();

    // load program
    if let Some(file) = env::args().nth(1) {
        state.eval(fs::read(file).unwrap()).unwrap();
    }

    display_splash();
    loop {
        let read = read_line(&mut stdin, &mut source) - 1;

        match state.load(&source) {
            Ok(_) => source.clear(),
            Err(Error::Syntax) => {
                if read == 0 {
                    source.clear();
                    let error: &str = state.get(-1).unwrap();
                    eprintln!("ERROR: {:?}", error);
                }

                state.pop(1);
                continue;
            }
            _ => panic!(),
        }

        match state.call_protected(0, 0) {
            Ok(_) => {}
            Err(Error::Runtime) => {
                {
                    let error: &str = state.get(-1).unwrap();
                    eprintln!("ERROR: {:?}", error);
                }
                state.pop(1);
            }
            Err(err) => panic!("{:?}", err),
        }
    }
}

fn display_splash() {
    eprintln!("# Welcome to the Lua shell! (written in Rust)");
    eprintln!();
    eprintln!("The following Rust functions can be called from the shell:");
    eprintln!("  * rust.error() - Raises a runtime error. The error message is also formatted in rust");
    eprintln!("  * rust.add(a, b) - Returns the sum of `a` and `b`");
    eprintln!("  * rust.len(c) - Returns the length of the string `c`");
    eprintln!();
}

fn read_line(stdin: &mut io::Stdin, source: &mut String) -> usize {
    if source.is_empty() {
        eprint!("lua> ");
    } else {
        eprint!(".... ");
    }

    let mut line = String::new();
    match stdin.read_line(&mut line) {
        Ok(b) => {
            source.push_str(line.as_str());
            b
        },
        _ => 0
    }
}

