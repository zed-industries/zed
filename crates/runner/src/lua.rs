use mlua::Result;

use crate::*;

impl Runtime for Lua {
    type Plugin = LuaPlugin;
    type Error = mlua::Error;

    fn init(module: Self::Plugin) -> Result<Self> {
        let lua = Lua::new();

        // for action in module.actions {
        //     action(&mut lua).ok()?;
        // }

        lua.load(&module.source).exec()?;
        return Ok(lua);
    }

    fn constant<T: DeserializeOwned>(&mut self, handle: &Handle) -> Result<T> {
        let val: Value = self.globals().get(handle.inner())?;
        Ok(self.from_value(val)?)
    }

    fn call<A: Serialize, R: DeserializeOwned>(&mut self, handle: &Handle, arg: A) -> Result<R> {
        let fun: Function = self.globals().get(handle.inner())?;
        let arg: Value = self.to_value(&arg)?;
        let result = fun.call(arg)?;
        Ok(self.from_value(result)?)
    }

    fn register_handle<T: AsRef<str>>(&mut self, name: T) -> bool {
        self.globals()
            .contains_key(name.as_ref().to_string())
            .unwrap_or(false)
    }
}

pub struct LuaPlugin {
    // name: String,
    source: String,
    // actions: Vec<Box<dyn FnOnce(&mut Lua) -> Result<(), ()>>>,
}

impl LuaPlugin {
    pub fn new(
        // name: String,
        source: String,
    ) -> LuaPlugin {
        LuaPlugin {
            // name,
            source,
            // actions: Vec::new(),
        }
    }

    // pub fn setup(mut self, action: fn(&mut Lua) -> Result<(), ()>) -> LuaPlugin {
    //     let action = Box::new(action);
    //     self.actions.push(action);
    //     self
    // }
}
