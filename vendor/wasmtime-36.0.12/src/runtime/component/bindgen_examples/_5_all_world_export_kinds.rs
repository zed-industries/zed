bindgen!({
    inline: r#"
        package example:world-exports;

        world with-exports {
            import log: func(msg: string);

            export run: func();

            /// An example of exporting an interface inline and naming it
            /// directly.
            export environment: interface {
                get: func(var: string) -> string;
                set: func(var: string, val: string);
            }

            /// An example of exporting an interface.
            export units;
        }

        interface units {
            /// Renders the number of bytes as a human readable string.
            bytes-to-string: func(bytes: u64) -> string;

            /// Renders the provided duration as a human readable string.
            duration-to-string: func(seconds: u64, ns: u32) -> string;
        }
    "#,
});
