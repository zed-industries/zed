use std::collections::{HashMap, HashSet};

use mlua::{Error, FromLua, Function, Lua, LuaSerdeExt, ToLua, UserData, Value};

pub mod runtime;
pub use runtime::*;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

impl Runtime for Lua {
    type Module = String;

    fn init(module: Self::Module) -> Option<Self> {
        let lua = Lua::new();
        lua.load(&module).exec().ok()?;
        return Some(lua);
    }

    fn handles(&self) -> Handles {
        let mut globals = HashSet::new();
        for pair in self.globals().pairs::<String, Value>() {
            if let Ok((k, _)) = pair {
                globals.insert(k);
            }
        }

        globals
    }

    fn val<T: DeserializeOwned>(&self, name: String) -> Option<T> {
        let val: Value = self.globals().get(name).ok()?;
        Some(self.from_value(val).ok()?)
    }

    fn call<T: Serialize + DeserializeOwned>(&self, name: String, arg: T) -> Option<T> {
        let fun: Function = self.globals().get(name).ok()?;
        let arg: Value = self.to_value(&arg).ok()?;
        let result = fun.call(arg).ok()?;
        Some(self.from_value(result).ok()?)
    }
}
