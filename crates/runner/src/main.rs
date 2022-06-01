use mlua::{Lua, Result};

use runner::*;

pub fn main() {
    let lua: Lua = Runtime::init("x = 7".to_string()).unwrap();
}
