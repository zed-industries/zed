bindgen!({
    inline: r#"
        package example:imports;

        world my-world {
            /// Fetch a greeting to present.
            import greet: func() -> string;

            /// Log a message to the host.
            import log: func(msg: string);

            import my-custom-host: interface {
                tick: func();
            }
        }
    "#,
});
