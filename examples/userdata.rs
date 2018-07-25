#[macro_use]
extern crate lua;

use lua::prelude::*;

use std::rc::Rc;

enum DebugFoo {}

impl LuaFunction for DebugFoo {
    type Error = Error;

    fn call(state: &mut LuaState) -> Result<usize, Error> {
        let debug = format!("{:?}", state.get_udata::<Foo>(Index::from(1))?);
        state.push(debug)?;
        Ok(1)
    }
}

#[derive(Debug)]
struct Field;

#[derive(Debug)]
struct Foo {
    bar: i32,
    baz: Rc<Field>,
}

impl LuaUserData for Foo {
    const METATABLE: &'static str = "Example.foo";

    fn register(m: &mut Meta) {
        m.set::<DebugFoo>(Metamethod::ToString);
    }
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
    state.eval("print(foo)").unwrap();

    state.get_global("foo").unwrap();
    let datum: &Foo = state.get_udata(Index::TOP).unwrap();
    println!("{:?}", datum);

    if state.is::<Foo>(Index::TOP) {
        println!("foo on top!");
    }
}
