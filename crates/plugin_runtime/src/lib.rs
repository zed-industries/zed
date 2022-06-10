pub mod wasi;
use pollster::FutureExt as _;
pub use wasi::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_plugin() {
        pub struct TestPlugin {
            noop: WasiFn<(), ()>,
            constant: WasiFn<(), u32>,
            identity: WasiFn<u32, u32>,
            add: WasiFn<(u32, u32), u32>,
            swap: WasiFn<(u32, u32), (u32, u32)>,
            sort: WasiFn<Vec<u32>, Vec<u32>>,
            print: WasiFn<String, ()>,
            // and_back: WasiFn<u32, u32>,
        }

        async {
            let mut runtime = WasiPluginBuilder::new_with_default_ctx()
                .unwrap()
                .host_function("mystery_number", |input: u32| input + 7)
                .unwrap()
                .init(include_bytes!("../../../plugins/bin/test_plugin.wasm"))
                .await
                .unwrap();

            let plugin = TestPlugin {
                noop: runtime.function("noop").unwrap(),
                constant: runtime.function("constant").unwrap(),
                identity: runtime.function("identity").unwrap(),
                add: runtime.function("add").unwrap(),
                swap: runtime.function("swap").unwrap(),
                sort: runtime.function("sort").unwrap(),
                print: runtime.function("print").unwrap(),
                // and_back: runtime.function("and_back").unwrap(),
            };

            let unsorted = vec![1, 3, 4, 2, 5];
            let sorted = vec![1, 2, 3, 4, 5];

            assert_eq!(runtime.call(&plugin.noop, ()).await.unwrap(), ());
            assert_eq!(runtime.call(&plugin.constant, ()).await.unwrap(), 27);
            assert_eq!(runtime.call(&plugin.identity, 58).await.unwrap(), 58);
            assert_eq!(runtime.call(&plugin.add, (3, 4)).await.unwrap(), 7);
            assert_eq!(runtime.call(&plugin.swap, (1, 2)).await.unwrap(), (2, 1));
            assert_eq!(runtime.call(&plugin.sort, unsorted).await.unwrap(), sorted);
            assert_eq!(runtime.call(&plugin.print, "Hi!".into()).await.unwrap(), ());
            // assert_eq!(runtime.call(&plugin.and_back, 1).await.unwrap(), 8);
        }
        .block_on()
    }
}
