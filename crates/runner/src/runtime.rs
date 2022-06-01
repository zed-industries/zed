use mlua::{FromLua, ToLua};

pub trait FromRuntime: Sized {
    fn from_runtime() -> Option<Self>;
}

pub trait Interface {
    type Handle;
    fn handles(&self) -> &[Self::Handle];
}

pub trait Runtime
where
    Self: Sized,
{
    type Module;
    type Interface: Interface;

    fn init(plugin: Self::Module) -> Option<Self>;
    fn interface(&self) -> Self::Interface;
    fn val<'lua, K: ToLua<'lua>, V: FromLua<'lua>>(&'lua self, key: K) -> Option<V>;
}
