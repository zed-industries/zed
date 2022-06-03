use mlua::Lua;

use plugin_runtime::*;

pub fn main() -> anyhow::Result<()> {
    let plugin = WasmPlugin {
        source_bytes: include_bytes!(
            "../plugin/target/wasm32-unknown-unknown/release/cargo_test.wasm"
        )
        .to_vec(),
        store_data: (),
    };

    let mut sum = Wasm::init(plugin)?;
    let strings = "I hope you have a nice day"
        .split(" ")
        .map(|x| x.to_string())
        .collect();
    let result = sum.sum_lengths(strings);

    dbg!(result);

    Ok(())
}

// struct SumLengths {
//     sum_lengths: Handle,
// }

// impl Interface for SumLengths {
//     fn from_runtime<T: Runtime>(runtime: &mut T) -> Option<Self> {
//         Some(SumLengths {
//             sum_lengths: runtime.handle_for("sum_lengths")?,
//         })
//     }
// }

// impl SumLengths {
//     fn sum_lengths<T: Runtime>(&self, runtime: &mut T, strings: Vec<String>) -> Option<usize> {
//         runtime.call(&self.sum_lengths, strings).ok()
//     }
// }

// #[plugin::interface]
trait SumLengths {
    fn sum_lengths(&mut self, strings: Vec<String>) -> usize;
}

impl<T: Runtime> SumLengths for T {
    fn sum_lengths(&mut self, strings: Vec<String>) -> usize {
        let result = self.call("sum_lengths", strings).ok().unwrap();
        return result;
    }
}
