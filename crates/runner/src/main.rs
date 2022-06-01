use mlua::{Lua, Result};

use runner::*;

pub fn main() {
    let lua: Lua = Runtime::init(
        "query = \"Some random tree-sitter query\"\nprint(\"Hello from the Lua test runner!\")"
            .to_string(),
    )
    .unwrap();
    let runner: TestRunner = lua.as_interface::<TestRunner>().unwrap();
    println!("{:#?}", runner);
}

#[derive(Debug)]
struct TestRunner {
    query: String,
}

impl Interface for TestRunner {
    fn from_runtime<T: Runtime>(runtime: &T) -> Option<TestRunner> {
        let query: String = runtime.val("query".to_string())?;
        Some(TestRunner { query })
    }
}
