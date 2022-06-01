use std::collections::HashSet;

use mlua::{FromLua, Lua, ToLua, Value};

pub type Interface = HashSet<String>;

pub trait Runtime
where
    Self: Sized,
{
    type Module;

    fn init(plugin: Self::Module) -> Option<Self>;
    fn interface(&self) -> Interface;
    // fn val<'a, T>(&'a self, name: String) -> Option<T>;
    // fn call<'a, A: MyFromLua<'a>, R>(&'a mut self, name: String, arg: A) -> Option<R>;
}
