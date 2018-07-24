# Lua rust

[![Build Status](https://travis-ci.org/germangb/lua-rs.svg?branch=master)](https://travis-ci.org/germangb/lua-rs)

(WIP) Rust wrapper around the [Lua](https://www.lua.org/) C API.

**[Documentation](https://germangb.github.io/lua-rs/lua/index.html)**

## Examples

```bash
# Run an interactive lua shell
$ cargo run --example lua
```

### Evaluate Lua code

```rust
extern crate lua;

use lua::prelude::*;

let mut state = LuaState::new();

state.eval("bar = 2.3").unwrap();

state.get_global("bar");
println!("bar = {}", state.get::<f64>(-1).unwrap());
```

### Call Rust functions from Lua

```rust
#[macro_use]
extern crate lua;

use lua::prelude::*;

struct Square;

impl LuaFunction for Square {
    type Error = Error;
    
    fn call(state: &mut LuaState) -> Result<usize, Error> {
        let n: f64 = state.get(1)?;
        state.push(n*n)?;
        Ok(1)
    }
}

let mut state = LuaState::new();

// register function
state.push(lua_function!(Square)).unwrap();
state.set_global("square");

// call from Lua
state.eval("len = square(4)").unwrap(); // len = 16
```

### Custom types 

```rust
#[macro_use]
extern crate lua;

use lua::prelude::*;

#[derive(Debug)]
struct Foo {
    bar: i32,
    baz: String,
}

impl LuaUserData for Foo {
    const METATABLE: &'static str = "Example.foo";
}
```

```rust
let mut state = LuaState::new();
state.open_libs();

let data = Foo {
    bar: 32,
    baz: String::from("hello world"),
};

state.push(lua_userdata!(data)).unwrap();
state.set_global("foo");

state.eval("print(foo)").unwrap();
```
