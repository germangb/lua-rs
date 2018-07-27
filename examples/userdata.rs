#[macro_use]
extern crate lua;

use lua::{State, Error, Index, UserData, Function};
use lua::userdata::{MetaTable, MetaMethod};

use std::rc::Rc;

enum DebugFoo {}

impl lua::Function for DebugFoo {
    type Error = Error;

    fn call(state: &mut State) -> Result<usize, Error> {
        let debug = format!("{:?}", state.get_udata::<Foo, _>(Index::Bottom(1))?);
        state.push(debug);
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

impl lua::UserData for Foo {
    const METATABLE: &'static str = "Example.foo";

    fn register(m: &mut MetaTable) {
        m.set::<DebugFoo>(MetaMethod::ToString);
    }
}

fn main() {
    let mut state = lua::State::new();
    state.open_libs();

    let baz = Rc::new(Field);

    state.push_udata(Foo {
        bar: 32,
        baz: Rc::clone(&baz),
    });

    state.set_global("foo");

    state.push_function::<DebugFoo>();
    state.set_global("debug");

    state.eval("print(debug(foo))").unwrap();
    state.eval("print(foo)").unwrap();

    {
        state.get_global("foo");
        let datum: &Foo = state.get_udata(-1).unwrap();
        println!("{:?}", datum);
    }

    state.push("hello world!");
    state.push(true);

    assert_eq!(Some("true"), state.get::<&str, _>(Index::TOP).ok());
}
