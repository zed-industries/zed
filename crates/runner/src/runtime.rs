use std::collections::HashSet;

use mlua::{FromLua, Lua, ToLua, Value};
use serde::{de::DeserializeOwned, Serialize};

pub type Handles = HashSet<String>;

pub trait Interface
where
    Self: Sized,
{
    fn from_runtime<T: Runtime>(runtime: &T) -> Option<Self>;
}

pub trait Runtime
where
    Self: Sized,
{
    type Module;

    fn init(plugin: Self::Module) -> Option<Self>;
    fn handles(&self) -> Handles;
    fn val<T: DeserializeOwned>(&self, name: String) -> Option<T>;
    fn call<T: Serialize + DeserializeOwned>(&self, name: String, arg: T) -> Option<T>;

    fn as_interface<T: Interface>(&self) -> Option<T> {
        Interface::from_runtime(self)
    }
}
