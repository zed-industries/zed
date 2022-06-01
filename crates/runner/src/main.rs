use mlua::Lua;

use runner::*;

pub fn main() -> Result<(), mlua::Error> {
    let source = include_str!("../plugin/cargo_test.lua").to_string();

    let module = LuaPlugin::new(source);
    // .setup(|runtime| {
    //     let greet = runtime
    //         .create_function(|_, name: String| {
    //             println!("Hello, {}!", name);
    //             Ok(())
    //         })
    //         .map_err(|_| ())?;

    //     runtime.globals().set("greet", greet).map_err(|_| ())?;
    //     Ok(())
    // });

    let mut lua: Lua = Runtime::init(module)?;
    let runner: TestRunner = lua.as_interface::<TestRunner>().unwrap();

    println!("extracted interface: {:#?}", &runner);

    let contents = runner.run_test(&mut lua, "it_works".into());

    println!("test results:{}", contents.unwrap());

    Ok(())
}

#[allow(dead_code)]
#[derive(Debug)]
struct TestRunner {
    pub query: String,
    run_test: Handle,
}

impl Interface for TestRunner {
    fn from_runtime<T: Runtime>(runtime: &mut T) -> Option<Self> {
        let run_test = runtime.handle_for("run_test")?;
        let query = runtime.handle_for("query")?;
        let query: String = runtime.constant(&query).ok()?;
        Some(TestRunner { query, run_test })
    }
}

impl TestRunner {
    pub fn run_test<T: Runtime>(&self, runtime: &mut T, test_name: String) -> Option<String> {
        runtime.call(&self.run_test, test_name).ok()
    }
}

#[test]
pub fn it_works() {
    panic!("huh, that was surprising...");
}
