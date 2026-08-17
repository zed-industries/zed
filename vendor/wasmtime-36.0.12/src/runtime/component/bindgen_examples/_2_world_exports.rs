bindgen!({
    inline: r#"
        package my:project;

        interface host {
            gen-random-integer: func() -> u32;
            sha256: func(bytes: list<u8>) -> string;
        }

        world hello-world {
            import host;

            export demo: interface {
                run: func();
            }
        }
    "#,
});
