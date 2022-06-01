use std::collections::{HashMap, HashSet};

use mlua::{Error, FromLua, Function, Lua, ToLua, UserData, Value};

pub mod runtime;
pub use runtime::*;

impl Runtime for Lua {
    type Module = String;

    fn init(module: Self::Module) -> Option<Self> {
        let lua = Lua::new();
        lua.load(&module).exec().ok()?;
        return Some(lua);
    }

    fn interface(&self) -> Interface {
        let mut globals = HashSet::new();
        for pair in self.globals().pairs::<String, Value>() {
            if let Ok((k, _)) = pair {
                globals.insert(k);
            }
        }

        globals
    }
}
