crate::generate!({
    inline: r#"
        package example:exported-resources;

        world import-some-resources {
            export logging;
        }

        interface logging {
            enum level {
                debug,
                info,
                warn,
                error,
            }
            resource logger {
                constructor(max-level: level);

                get-max-level: func() -> level;
                set-max-level: func(level: level);

                log: func(level: level, msg: string);
            }
        }
    "#,
});
