use lua::prelude::*;

#[derive(Debug)]
struct Vector(Vec<i32>);

impl Drop for Vector {
    fn drop(&mut self) {
        println!("Dropping Vector...");
    }
}

// --------------------------------
// Functions & Metamethods
// --------------------------------

enum New {}         // Function to create a new Vector

enum ToString {}    // `__tostring` metamethod
enum Length {}      // `__len` metamethod
enum Get {}         // `__index` metamethod
enum Insert {}      // `__newindex` metamethid

impl LuaUserData for Vector {
    const METATABLE: &'static str = "Vector.Vector";

    fn register(meta: &mut Meta) {
        meta.set::<ToString>(Metamethod::ToString);
        meta.set::<Length>(Metamethod::Len);
        meta.set::<Get>(Metamethod::Index);
        meta.set::<Insert>(Metamethod::NewIndex);
    }
}

// --------------------------------

impl Vector {
    fn new(cap: usize) -> Self {
        Vector(vec![0; cap])
    }
    fn set(&mut self, idx: usize, v: i32) {
        self.0[idx] = v;
    }
    fn get(&self, idx: usize) -> Option<i32> {
        self.0.get(idx).map(|s| *s)
    }
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl LuaFunction for ToString {
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
        let cap: usize = state.get(-1)?;
        let vec = Vector::new(cap);
        state.push(lua_userdata!(vec))?;
        Ok(1)
    }
}

impl LuaFunction for Insert {
    type Error = Error;

    fn call(state: &mut LuaState) -> Result<usize, Error> {
        {
            let index: usize = state.get(2)?;
            let value: i32 = state.get(3)?;
            let mut vector: RefMut<Vector> = state.get_mut(1)?;

            if index < vector.len() {
                vector.set(index, value);
            } else {
                return Err(Error::Runtime);
            }
        }
        Ok(0)
    }
}

impl LuaFunction for Get {
    type Error = Error;

    fn call(state: &mut LuaState) -> Result<usize, Error> {
        let index: usize = state.get(2)?;
        let value = state.get(1).ok().and_then(|s: Ref<Vector>| s.get(index));

        state.push(value);
        Ok(1)
    }
}

impl LuaFunction for Length {
    type Error = Error;

    fn call(state: &mut LuaState) -> Result<usize, Error> {
        let len = state.get(1).map(|s: Ref<Vector>| s.len())?;

        state.push(len);
        Ok(1)
    }
}

/*
pub fn load_lib(lua: &mut LuaState) -> Result<(), Error> {
    lua.push(Table)?;
    lua.push("new")?;
    lua.push(lua_function!(New))?;
    lua.set_table(-3);
    lua.push("add")?;
    lua.push(lua_function!(Insert))?;
    lua.set_table(-3);
    lua.push("get")?;
    lua.push(lua_function!(Get))?;
    lua.set_table(-3);
    lua.set_global("vec");
    Ok(())
}
*/

lua_library! {
    ToString    => "tostring",
    Length      => "length",
    Get         => "length",
    New         => "new",
    Insert      => "set"
}
