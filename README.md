# Lua rust

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

// define global from rust
state.push_value(42);
state.set_global("foo");

state.eval("print(foo)").unwrap();

// define global from lua
state.eval("bar = 2.3").unwrap();

state.get_global("bar");
println!("bar = {}", state.get_value::<f64>(Index::TOP).unwrap());
```

### Working with the stack

```rust
extern crate lua;

use lua::prelude::*;

let mut state = LuaState::new();

// Values that implements the `IntoLua` and `FromLua` traits can be pushed an read from the stack.
state.push_value(42);
assert_eq!(Some(42.0), state.get_value::<f64>(Index::TOP));

state.push_value(16);
assert_eq!(Some(16), state.get_value::<i64>(Index::TOP));

state.push_value("hello");
assert_eq!(None, state.get_value::<i32>(Index::TOP));

state.push_nil();
assert!(state.is_nil(Index::TOP));
```

## Safety

```rust
state.push_value(3.14159265);

// `LuaStr` is a view into a string owned by Lua.
let value = state.get_string(Index::TOP).unwrap();

state.pop(1); // **Compilation error**: potential dangling pointer
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
