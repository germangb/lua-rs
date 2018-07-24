# Lua rust

[![Build Status](https://travis-ci.org/germangb/lua-rs.svg?branch=master)](https://travis-ci.org/germangb/lua-rs)

(WIP) Rust wrapper around the [Lua](https://www.lua.org/) C API.

**[Documentation](https://germangb.github.io/lua-rs/lua/index.html)**

## Examples

```bash
# Run an interactive lua shell ([FILE] is optional)
$ cargo run --example lua [FILE]
```

### Functions

Types that implement the `LuaFunction` trait can be used as lua functions:

```rust
use lua::prelude::*;

// A type for a function that returns the length of a string
struct StringLength;

impl LuaFunction for StringLength {
    type Error = Error;

    fn call(state: &mut LuaState) -> Result<usize, Self::Error> {
        let length = state.get(1).map(|s: &str| s.len())?;
        state.push(length)?;
        Ok(1)
    }
}

let mut state = LuaState::new();

state.push(lua_function!(LuaFunction)).unwrap();
state.set_global("length");

state.eval("len = length('hello world')").unwrap(); // len = 11
```

### Custom Userdata

Any type that implements the `LuaUserData` trait can be used as [userdata](https://www.lua.org/pil/28.1.html). When a type is moved into the stack, the `LuaState` becomes its owner and will eventually be dropped by the garbage collector.

For a more complete example, including setting up metamethods, see [this example](./examples/lib/vector.rs).

```rust
use lua::prelude::*;

#[derive(Debug)]
struct Foo {
    bar: Vec<i32>,
    baz: String,
}

impl LuaUserData for Foo {
    // An identifier, unique to the Type
    const METATABLE: &'static str = "Example.foo";
}

let mut state = LuaState::new();

state.push(lua_userdata!(Foo {
    bar: vec![0; 16],
    baz: String::from("Hello world!"),
})).unwrap();

// Get a reference to the stack
let foo: Ref<Foo> = state.get(-1).unwrap();

// To get a mutable reference, use this instead:
// let mut foomut: RefMut<Foo> = state.get_mut(-1).unwrap();
```
