use mlua::{Error, FromLua, Lua, ToLua, UserData};

pub mod runtime;
pub use runtime::*;

impl Runtime for Lua {
    type Module = String;
    // type Error = Error;
    type Interface = LuaInterface;

    fn init(module: Self::Module) -> Option<Self> {
        let lua = Lua::new();
        lua.load(&module).exec().ok()?;
        return Some(lua);
    }

    fn interface(&self) -> Self::Interface {
        todo!()
    }

    fn val<'lua, K: ToLua<'lua>, V: FromLua<'lua>>(&'lua self, key: K) -> Option<V> {
        self.globals().get(key).ok()
    }
}

pub struct LuaInterface {
    funs: Vec<String>,
    vals: Vec<String>,
}

impl Interface for LuaInterface {
    type Handle = String;

    fn handles(&self) -> &[Self::Handle] {
        todo!()
    }
}
