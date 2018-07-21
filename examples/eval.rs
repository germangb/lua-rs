extern crate lua;

use lua::prelude::*;

mod math {
    use *;

    pub struct SinFn;
    pub struct CosFn;

    impl LuaFn for SinFn {
        type Error = Error;

        fn call(state: &mut LuaState) -> Result<usize, Self::Error> {
            state.push_value(0.0)?;
            Ok(1)
        }
    }

    impl LuaFn for CosFn {
        type Error = Error;

        // this function generates a runtime error
        fn call(state: &mut LuaState) -> Result<usize, Self::Error> {
            let _ = state.get_value::<f64>(Index::Arg(0))?;
            let _ = state.get_value::<LuaStr>(Index::Arg(1))?;
            let _ = state.get_value::<Nil>(Index::Arg(2))?;
            let _ = state.get_value::<bool>(Index::Arg(3))?;
            state.push_value(1.0)?;
            Ok(1)
        }
    }
}

struct FooFun;

impl LuaFn for FooFun {
    type Error = Error;

    fn call(state: &mut LuaState) -> Result<usize, Self::Error> {
        state.push_value("hello")?;

        // crashes
        //let _ = state.get_value::<Nil>(Index::Arg(3))?;
        Ok(1)
    }
}

fn main() {
    let mut state = LuaState::new();
    state.open_libs();

    // push functions
    state.push_value(FooFun).unwrap();
    state.set_global("dummy");

    // run rust functions
    state.eval("bar = dummy()").unwrap();
    state.eval("print(bar)").unwrap();
}
