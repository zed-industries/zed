bindgen!({
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
            import logging;
        }
    "#,
});
