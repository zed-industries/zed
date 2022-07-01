pub mod plugin;
pub use plugin::*;

#[cfg(test)]
mod tests {
    use super::*;
    use pollster::FutureExt as _;

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
            and_back: WasiFn<u32, u32>,
            imports: WasiFn<u32, u32>,
            half_async: WasiFn<u32, u32>,
            echo_async: WasiFn<String, String>,
        }

        async {
            let mut runtime = PluginBuilder::new_with_default_ctx()
                .unwrap()
                .host_function("mystery_number", |input: u32| input + 7)
                .unwrap()
                .host_function("import_noop", |_: ()| ())
                .unwrap()
                .host_function("import_identity", |input: u32| input)
                .unwrap()
                .host_function("import_swap", |(a, b): (u32, u32)| (b, a))
                .unwrap()
                .host_function_async("import_half", |a: u32| async move { a / 2 })
                .unwrap()
                .host_function_async("command_async", |command: String| async move {
                    // TODO: actual thing
                    dbg!(&command);
                    let mut args = command.split(' ');
                    let command = args.next().unwrap();
                    smol::process::Command::new(command)
                        .args(args)
                        .output()
                        .await
                        .ok()
                        .map(|output| {
                            dbg!("Did run command!");
                            output.stdout
                        })
                })
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
                and_back: runtime.function("and_back").unwrap(),
                imports: runtime.function("imports").unwrap(),
                half_async: runtime.function("half_async").unwrap(),
                echo_async: runtime.function("echo_async").unwrap(),
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
            assert_eq!(runtime.call(&plugin.and_back, 1).await.unwrap(), 8);
            assert_eq!(runtime.call(&plugin.imports, 1).await.unwrap(), 8);
            assert_eq!(runtime.call(&plugin.half_async, 4).await.unwrap(), 2);
            assert_eq!(
                runtime
                    .call(&plugin.echo_async, "eko".into())
                    .await
                    .unwrap(),
                "eko\n"
            );

            // dbg!("{}", runtime.call(&plugin.and_back, 1).await.unwrap());
        }
        .block_on()
    }
}
