extern crate lua;

use lua::prelude::*;
use lua::function::*;

/// a dummy function that returns a single string literal
struct Foo;

/// A function to sort number pairs
struct SortPair;

impl LuaFunction for Foo {
    type Output = (&'static str);

    fn call(state: &LuaState) -> Self::Output {
        ("hello")
    }
}

impl LuaFunction for SortPair {
    type Output = (f64, f64);

    fn call(state: &LuaState) -> Self::Output {
        let l: f64 = state.get_value(Index::Bottom(1)).unwrap_or(0.0);
        let r: f64 = state.get_value(Index::Bottom(2)).unwrap_or(0.0);

        if l < r {
            (l, r)
        } else {
            (r, l)
        }
    }
}

fn main() {
    let mut state = LuaState::new();
    state.open_libs();

    // push functions
    state.push_value(Foo);
    state.set_global("dummy");
    state.push_value(SortPair);
    state.set_global("sort");

    // run rust functions
    state.eval(stringify!{
        l, r = sort(2, 4)
        print("sored:", l, r)
    }).unwrap();

    state.eval("bar = dummy()").unwrap();

    state.get_global("bar");
    println!(
        "from rust: bar = {:?}",
         state.get_string(Index::TOP).unwrap().into_str_lossy(),
     );
}
