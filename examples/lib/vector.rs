use lua::{Index, Error, State, UserData, Function};
use lua::userdata::{MetaMethod, MetaTable};

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

impl UserData for Vector {
    const METATABLE: &'static str = "Vector.Vector";

    fn register(meta: &mut MetaTable) {
        meta.set::<ToString>(MetaMethod::ToString);
        meta.set::<Length>(MetaMethod::Len);
        meta.set::<Get>(MetaMethod::Index);
        meta.set::<Insert>(MetaMethod::NewIndex);
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

impl Function for ToString {
    type Error = Error;

    fn call(state: &mut State) -> Result<usize, Error> {
        let string = state.get_udata(Index::Bottom(1)).map(|v: &Vector| format!("{:?}", v))?;
        state.push(string);
        Ok(1)
    }
}

impl Function for New {
    type Error = Error;

    fn call(state: &mut State) -> Result<usize, Error> {
        let cap: usize = state.get(Index::Bottom(1))?;
        state.push_udata(Vector::new(cap));
        Ok(1)
    }
}

impl Function for Insert {
    type Error = Error;

    fn call(state: &mut State) -> Result<usize, Error> {
        {
            let index: usize = state.get(Index::Bottom(2))?;
            let value: i32 = state.get(Index::Bottom(3))?;
            let vector: &mut Vector = state.get_udata_mut(Index::Bottom(1))?;

            if index < vector.len() {
                vector.set(index, value);
            } else {
                return Err(Error::Runtime);
            }
        }
        Ok(0)
    }
}

impl Function for Get {
    type Error = Error;

    fn call(state: &mut State) -> Result<usize, Error> {
        let index: usize = state.get(Index::Bottom(2))?;
        let value = state.get_udata(Index::Bottom(1)).map(|s: &Vector| s.get(index));

        state.push(value?);
        Ok(1)
    }
}

impl Function for Length {
    type Error = Error;

    fn call(state: &mut State) -> Result<usize, Error> {
        let len = state.get_udata(Index::Bottom(1)).map(|s: &Vector| s.len())?;

        state.push(len);
        Ok(1)
    }
}

/*
pub fn load_lib(lua: &mut State) -> Result<(), Error> {
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
