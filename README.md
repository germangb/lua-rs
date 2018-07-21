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
state.open_libs();

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
state.open_libs();

// push rust functions
state.push_value(LengthFn).unwrap();
state.set_global("str_len");

// call from Lua
state.eval("len = str_len()").unwrap();
```

## License

```
MIT License

Copyright (c) 2018 German Gomez Bajo

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```
