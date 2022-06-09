pub mod wasi;
pub use wasi::*;

// #[cfg(test)]
// mod tests {
//     use super::*;

//     pub fn init_wasi() {
//         let plugin = WasiPluginBuilder::new().init(todo!()).unwrap();
//         let handle: WasiFn<u32, String> = plugin.function("hello").unwrap();
//         let result = plugin.call(handle, 27).unwrap();
//         assert_eq!(result, "world 27");
//     }
// }
