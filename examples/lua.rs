extern crate lua;

use std::io;
use std::io::Read;

use lua::prelude::*;

struct FnError;

impl LuaFn for FnError {
    type Error = &'static str;
    fn call(_: &mut LuaState) -> Result<usize, Self::Error> {
        Err("This is a rust runtime error")
    }
}

struct FnAdd;

impl LuaFn for FnAdd {
    type Error = Error;
    fn call(state: &mut LuaState) -> Result<usize, Self::Error> {
        let a: f64 = state.get_value(Index::Arg(1))?;
        let b: f64 = state.get_value(Index::Arg(2))?;
        state.push_value(a + b);
        Ok(1)
    }
}

fn main() {
    let mut stdin = io::stdin();

    let mut source = String::new();
    let mut state = LuaState::new();
    state.open_libs();

    state.push_value(FnError);
    state.set_global("error");

    state.push_value(FnAdd);
    state.set_global("add");

    eprintln!("Welcome to Lua from Rust (and vice versa)!");
    eprintln!();
    eprintln!("The following Rust functions can be called from the shell:");
    eprintln!("  * error() - Raises a runtime error. The error message is also formatted in rust");
    eprintln!("  * add(a, b) - Returns the sum of two numbers");
    eprintln!();

    loop {
        if source.is_empty() {
            eprint!("> ");
        } else {
            eprint!("..");
        }

        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Err(_) => break,
            Ok(_) => source.push_str(line.as_str()),
        }

        match state.load(&source) {
            Ok(_) => source.clear(),
            Err(Error::Syntax) => {
                if line.trim().is_empty() {
                    eprintln!("Syntax error\n===");
                    unsafe {
                        eprintln!(
                            "{}",
                            state.get_string(Index::Top(1)).unwrap().as_str_unchecked()
                        );
                    }
                    source.clear();
                }

                state.pop(1);
                continue;
            }
            _ => panic!(),
        }

        //eprintln!("code so far:\n===\n{:?}\n===", source);

        match state.call_protected(0, 0) {
            Ok(_) => {}
            Err(Error::Runtime) => {
                eprintln!("Ruintime error\n===");
                unsafe {
                    eprintln!(
                        "{}",
                        state.get_string(Index::Top(1)).unwrap().as_str_unchecked()
                    );
                }
                state.pop(1);
            }
            _ => panic!(),
        }
    }
}
