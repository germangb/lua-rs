#[macro_use]
extern crate lua;

use lua::prelude::*;

use std::rc::Rc;

enum DebugFoo {}

impl LuaFunction for DebugFoo {
    type Error = Error;

    fn call(state: &mut LuaState) -> Result<usize, Error> {
        let debug = {
            let data: Ref<Foo> = state.get(1)?;
            format!("{:?}", *data)
        };

        state.push(debug)?;
        Ok(1)
    }
}

#[derive(Debug)]
struct Field;

impl Drop for Field {
    fn drop(&mut self) {
        println!("drop the field");
    }
}

#[derive(Debug)]
struct Foo {
    bar: i32,
    baz: Rc<Field>,
}

impl LuaUserData for Foo {
    const METATABLE: &'static str = "Example.foo";
}

fn main() {
    let mut state = LuaState::new();
    state.open_libs();

    let baz = Rc::new(Field);

    state.push_udata(Foo {
        bar: 32,
        baz: Rc::clone(&baz),
    }).unwrap();

    state.set_global("foo");

    state.push_function::<DebugFoo>().unwrap();
    state.set_global("debug");

    state.eval("print(debug(foo))").unwrap();
}
