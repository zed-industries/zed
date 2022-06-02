use mlua::Lua;

use runner::*;

// pub fn main() -> Result<(), mlua::Error> {
//     let source = include_str!("../plugin/cargo_test.lua").to_string();

//     let module = LuaPlugin::new(source);
//     let mut lua: Lua = Runtime::init(module)?;
//     let runner: TestRunner = lua.as_interface::<TestRunner>().unwrap();

//     println!("extracted interface: {:#?}", &runner);

//     let contents = runner.run_test(&mut lua, "it_works".into());

//     println!("test results:{}", contents.unwrap());

//     Ok(())
// }

// pub fn main() -> mlua::Result<()> {
//     let module = LuaPlugin::new(include_str!("../plugin/cargo_test.lua").to_string());
//     let mut lua: Lua = Runtime::init(module)?;
//     let runner = lua.as_interface::<TestRunner>().unwrap();
//     let test_results = runner.run_test(&mut lua, "it_works".into());
//     Ok(())
// }

pub fn main() -> anyhow::Result<()> {
    let plugin = WasmPlugin {
        source_bytes: include_bytes!(
            "../plugin/target/wasm32-unknown-unknown/release/cargo_test.wasm"
        )
        .to_vec(),
        store_data: (),
    };

    let mut wasm: Wasm<()> = Runtime::init(plugin)?;
    let banana = wasm.as_interface::<Banana>().unwrap();
    let result = banana.banana(&mut wasm, 420.69);

    dbg!("{}", result);

    Ok(())
}

struct Banana {
    banana: Handle,
}

impl Interface for Banana {
    fn from_runtime<T: Runtime>(runtime: &mut T) -> Option<Self> {
        let banana = runtime.handle_for("banana")?;
        Some(Banana { banana })
    }
}

impl Banana {
    fn banana<T: Runtime>(&self, runtime: &mut T, number: f64) -> Option<f64> {
        runtime.call(&self.banana, number).ok()
    }
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
