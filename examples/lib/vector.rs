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
        let string = state.get_udata(Index::from(1)).map(|v: &Vector| format!("{:?}", v))?;
        state.push(string)?;
        Ok(1)
    }
}

impl LuaFunction for New {
    type Error = Error;

    fn call(state: &mut LuaState) -> Result<usize, Error> {
        let cap: usize = state.get(Index::from(1))?;
        state.push_udata(Vector::new(cap))?;
        Ok(1)
    }
}

impl LuaFunction for Insert {
    type Error = Error;

    fn call(state: &mut LuaState) -> Result<usize, Error> {
        {
            let index: usize = state.get(Index::from(2))?;
            let value: i32 = state.get(Index::from(3))?;
            let vector: &mut Vector = state.get_udata_mut(Index::from(1))?;

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
        let index: usize = state.get(Index::from(2))?;
        let value = state.get_udata(Index::from(1)).map(|s: &Vector| s.get(index));

        state.push(value?);
        Ok(1)
    }
}

impl LuaFunction for Length {
    type Error = Error;

    fn call(state: &mut LuaState) -> Result<usize, Error> {
        let len = state.get_udata(Index::from(1)).map(|s: &Vector| s.len())?;

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
