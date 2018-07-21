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

impl LuaFn for FnAdd {
    type Error = Error;
    fn call(state: &mut LuaState) -> Result<usize, Self::Error> {
        let a: i32 = state.get_value(Index::Arg(1))?;
        let b: i32 = state.get_value(Index::Arg(2))?;
        state.push_value(a + b);
        Ok(1)
    }
}

impl LuaFn for FnLen {
    type Error = Error;
    fn call(state: &mut LuaState) -> Result<usize, Self::Error> {
        let len = state.get_string(Index::Arg(1))
            .map(|s| s.as_slice().len());
        state.push_value(len?);
        Ok(1)
    }
}

