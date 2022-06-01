use mlua::{Lua, Result};

use runner::*;

pub fn main() {
    let lua: Lua = Runtime::init("x = 7".to_string()).unwrap();
    println!("{:?}", lua.interface());
}

struct InterfaceX;

impl InterfaceX {
    pub fn get_x<T: Runtime>(runtime: T) -> usize {
        // runtime.get("x")
        todo!()
    }
}
