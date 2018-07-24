use lua::prelude::*;

#[derive(Debug)]
struct Vector(Vec<i32>);

impl Vector {
    fn new() -> Self {
        Vector(Vec::new())
    }
    fn add(&mut self, v: i32) {
        self.0.push(v)
    }
    fn get(&self, idx: usize) -> Option<i32> {
        self.0.get(idx).map(|s| *s)
    }
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl LuaUserData for Vector {
    const METATABLE: &'static str = "Vector.Vector";
}

struct New; // function to create a new Vector
struct Add; // function to append values to Vector
struct Get; // function to return values from the Vector

impl LuaFunction for New {
    type Error = Error;

    fn call(state: &mut LuaState) -> Result<usize, Error> {
        let vec = Vector::new();
        state.push(lua_userdata!(vec))?;
        Ok(1)
    }
}

impl LuaFunction for Add {
    type Error = Error;

    fn call(state: &mut LuaState) -> Result<usize, Error> {
        let value: i32 = state.get(2)?;
        state.get_mut(1).map(|mut v: RefMut<Vector>| v.add(value))?;
        Ok(0)
    }
}

impl LuaFunction for Get {
    type Error = Error;

    fn call(state: &mut LuaState) -> Result<usize, Error> {
        let index: usize = state.get(2)?;
        let value = state.get(1)
            .and_then(|s: Ref<Vector>|
                // map Out of bounds error to a Lua runtime error
                s.get(index).ok_or(Error::Runtime)
            )?;

        state.push(value);
        Ok(1)
    }
}

pub fn load_lib(lua: &mut LuaState) -> Result<(), Error> {
    lua.push(Table)?;
    lua.push("new")?;
    lua.push(lua_function!(New))?;
    lua.set_table(-3);
    lua.push("add")?;
    lua.push(lua_function!(Add))?;
    lua.set_table(-3);
    lua.push("get")?;
    lua.push(lua_function!(Get))?;
    lua.set_table(-3);
    lua.set_global("vec");
    Ok(())
}

