use lua::prelude::*;

#[derive(Debug)]
struct Vector(Vec<i32>);

// --------------------------------
// Functions & Metamethods
// --------------------------------

struct MetaToString;    // `__tostring` metamethod
struct Get;             // Function to return values from the Vector
                        // This function is also set as the `__index` metamethod
struct New;             // Function to create a new Vector
struct Add;             // Function to append values to Vector

impl LuaUserData for Vector {
    const METATABLE: &'static str = "Vector.Vector";

    fn register(meta: &mut Meta) {
        meta.set(Metamethod::ToString, MetaToString);
        meta.set(Metamethod::Index, Get);
    }
}

// --------------------------------

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

impl LuaFunction for MetaToString {
    type Error = Error;

    fn call(state: &mut LuaState) -> Result<usize, Error> {
        let string = state.get(1).map(|v: Ref<Vector>| format!("{:?}", *v))?;
        state.push(string)?;
        Ok(1)
    }
}

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
            .and_then(|s: Ref<Vector>| s.get(index).ok() )?;

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

