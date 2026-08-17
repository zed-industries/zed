bindgen!({
    inline: r#"
        package example:imported-resources;

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

        world import-some-resources {
            import logging;
        }
    "#,

    // NEW: Make all imports/exports `async` by default, and additionally
    // interactions with `ResourceTable` can possibly trap so enable the ability
    // to return traps from generated functions with `trappable`.
    imports: { default: async | trappable },
    exports: { default: async },

    with: {
        // Specify that our host resource is going to point to the `MyLogger`
        // which is defined just below this macro.
        "example:imported-resources/logging/logger": MyLogger,
    },
});

/// A sample host-defined type which contains arbitrary host-defined data.
///
/// In this case this is relatively simple but there's no restrictions on what
/// this type can hold other than that it must be `'static + Send`.
pub struct MyLogger {
    pub max_level: example::imported_resources::logging::Level,
}
