use super::*;

pub struct FnError;
pub struct FnAdd;
pub struct FnLen;

/// Load library into a `rust` lua table
pub fn load(state: &mut LuaState) {
    state.new_table();
    state.push_value("error");
    state.push_value(lib::FnError);
    state.set_table(Index::Top(3));
    state.push_value("add");
    state.push_value(lib::FnAdd);
    state.set_table(Index::Top(3));
    state.push_value("len");
    state.push_value(lib::FnLen);
    state.set_table(Index::Top(3));
    state.set_global("rust");
}

impl LuaFn for FnError {
    type Error = &'static str;

    fn call(_: &mut LuaState) -> Result<usize, Self::Error> {
        Err("This is a rust runtime error")
    }
}

impl FnAdd {
    fn add(a: i32, b: i32) -> i32 {
        a+b
    }
}

impl LuaFn for FnAdd {
    type Error = Error;

    fn call(state: &mut LuaState) -> Result<usize, Self::Error> {
        let a = state.get_value(Index::Arg(1))?;
        let b = state.get_value(Index::Arg(2))?;

        //let (a, b) = state.get_value(Index::BOTTOM)?;

        state.push_value(Self::add(a, b));
        Ok(1)
    }
}

impl LuaFn for FnLen {
    type Error = Error;

    fn call(state: &mut LuaState) -> Result<usize, Self::Error> {
        let len = state.get_string(Index::Arg(1)).map(|s| s.len());

        state.push_value(len?);
        Ok(1)
    }
}

