crate::generate!({
    inline: r#"
        package example:interface-imports;

        interface logging {
            enum level {
                debug,
                info,
                warn,
                error,
            }

            log: func(level: level, msg: string);
        }

        world with-imports {
            // Local interfaces can be imported.
            import logging;

            // Dependencies can also be referenced, and they're loaded from the
            // `path` directive specified below.
            import wasi:cli/environment@0.2.0;
        }
    "#,

    path: "wasi-cli@0.2.0.wasm",

    // specify that this interface dependency should be generated as well.
    with: {
        "wasi:cli/environment@0.2.0": generate,
    }
});
