# Lua rust

[![Build Status](https://travis-ci.org/germangb/lua-rs.svg?branch=master)](https://travis-ci.org/germangb/lua-rs)

(WIP) Rust wrapper around the [Lua](https://www.lua.org/) C API.

**[Documentation](https://germangb.github.io/lua-rs/)**

## Examples

```bash
# Run an interactive lua shell ([FILE] is optional)
$ cargo run --example lua [FILE]
```

### Hello World

```rust
extern crate lua;

let mut state = lua::State::new();
state.open_libs();

state.eval("print ('hello world')").unwrap();
```

### Functions

Types that implement the `Function` trait can be used as lua functions:

```rust
extern crate lua;

use lua::{Function, Index};

// A Type for a function that returns the length of a string
enum StringLength {}

impl Function for StringLength {
    type Error = lua::Error;

    fn call(state: &mut lua::State) -> Result<usize, Self::Error> {
        let length = state.get(Index::Bottom(1)).map(|s: &str| s.len())?;
        state.push(length)?;
        Ok(1)
    }
}

let mut state = lua::State::new();

state.push_function::<StringLength>().unwrap();
state.set_global("length");

state.eval("len = length('hello world')").unwrap(); // len = 11
```

### Custom Userdata

Types that implements the `UserData` trait can be used as [userdata](https://www.lua.org/pil/28.1.html). When a type is moved into the stack, the `State` becomes its owner and will eventually be dropped by the garbage collector.

For a more complete example, including setting up metamethods, see [this example](./examples/lib/vector.rs).

```rust
extern crate lua;

use lua::{UserData, Index};

#[derive(Debug)]
struct Foo {
    bar: Vec<i32>,
    baz: String,
}

impl UserData for Foo {
    // An identifier, unique to the Type
    const METATABLE: &'static str = "Example.foo";
}

let mut state = lua::State::new();

state.push_udata(Foo {
    bar: vec![0; 16],
    baz: String::from("Hello world!"),
}).unwrap();

// Get a reference to the stack
let foo: &Foo = state.get_udata(Index::TOP).unwrap();

// To get a mutable reference, use this instead:
// let foomut: &mut Foo = state.get_udata_mut(Index::TOP).unwrap();
```
