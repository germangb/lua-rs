use lua::{Index, State, Error, Function};

enum FnError {} // raises a lua runtime error
enum FnAdd {}   // adds two integers
enum FnLen {}   // returns the length of a string

lua_library! {
    FnError => "error",
    FnAdd => "add",
    FnLen => "len"
}

impl Function for FnError {
    type Error = &'static str;

    fn call(_: &mut State) -> Result<usize, Self::Error> {
        Err("This is a rust runtime error")
    }
}

impl FnAdd {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

impl Function for FnAdd {
    type Error = Error;

    fn call(state: &mut State) -> Result<usize, Self::Error> {
        let a = state.get(Index::Bottom(1))?;
        let b = state.get(Index::Bottom(2))?;
        state.push(Self::add(a, b));
        Ok(1)
    }
}

impl Function for FnLen {
    type Error = Error;

    fn call(state: &mut State) -> Result<usize, Self::Error> {
        let length = {
            let s: &str = state.get(Index::Bottom(1))?;
            s.len()
        };

        state.push(length);
        Ok(1)
    }
}
