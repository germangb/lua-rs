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
println!("bar = {}", state.get_value::<f64>(Index::TOP).unwrap());
```

### Call Rust functions from Lua

```rust
extern crate lua;

use lua::prelude::*;

struct LengthFn;

impl LuaFn for LengthFn {
    type Error = Error;
    
    fn call(state: &mut LuaState) -> Result<usize, Error> {
        let len = state.get_string(Index::Arg(1)).map(|s| s.len());
        state.push_value(len?)?;
        Ok(1)
    }
}

let mut state = LuaState::new();

// push rust functions
state.push_value(LengthFn).unwrap();
state.set_global("str_len");

// call from Lua
state.eval("len = str_len()").unwrap();
```
