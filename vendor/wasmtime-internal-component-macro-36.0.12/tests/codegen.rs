#![allow(dead_code, reason = "lots of macro-generated code")]

macro_rules! gentest {
    ($id:ident $name:tt $path:tt) => {
        mod $id {
            mod sugar {
                wasmtime::component::bindgen!(in $path);
            }
            mod async_ {
                wasmtime::component::bindgen!({
                    path: $path,
                    imports: { default: async },
                    exports: { default: async },
                });
            }
            mod concurrent {
                wasmtime::component::bindgen!({
                    path: $path,
                    imports: { default: async | store },
                    exports: { default: async | store },
                });
            }
            mod tracing {
                wasmtime::component::bindgen!({
                    path: $path,
                    imports: { default: tracing | verbose_tracing },
                    exports: { default: tracing | verbose_tracing },
                    ownership: Borrowing {
                        duplicate_if_necessary: true
                    }
                });
            }
        }
    };
}

component_macro_test_helpers::foreach!(gentest);

mod with_key_and_resources {
    use anyhow::Result;
    use wasmtime::component::Resource;

    wasmtime::component::bindgen!({
        inline: "
            package demo:pkg;

            interface bar {
                resource a;
                resource b;
            }

            world foo {
                resource a;
                resource b;

                import foo: interface {
                    resource a;
                    resource b;
                }

                import bar;
            }
        ",
        with: {
            "a": MyA,
            "b": MyA,
            "foo/a": MyA,
            "foo/b": MyA,
            "demo:pkg/bar/a": MyA,
            "demo:pkg/bar/b": MyA,
        },
    });

    pub struct MyA;

    struct MyComponent;

    impl FooImports for MyComponent {}

    impl HostA for MyComponent {
        fn drop(&mut self, _: Resource<MyA>) -> Result<()> {
            loop {}
        }
    }

    impl HostB for MyComponent {
        fn drop(&mut self, _: Resource<MyA>) -> Result<()> {
            loop {}
        }
    }

    impl foo::Host for MyComponent {}

    impl foo::HostA for MyComponent {
        fn drop(&mut self, _: Resource<MyA>) -> Result<()> {
            loop {}
        }
    }

    impl foo::HostB for MyComponent {
        fn drop(&mut self, _: Resource<MyA>) -> Result<()> {
            loop {}
        }
    }

    impl demo::pkg::bar::Host for MyComponent {}

    impl demo::pkg::bar::HostA for MyComponent {
        fn drop(&mut self, _: Resource<MyA>) -> Result<()> {
            loop {}
        }
    }

    impl demo::pkg::bar::HostB for MyComponent {
        fn drop(&mut self, _: Resource<MyA>) -> Result<()> {
            loop {}
        }
    }
}

mod trappable_errors_with_versioned_and_unversioned_packages {
    wasmtime::component::bindgen!({
        world: "foo:foo/nope",
        inline: "
            package foo:foo@0.1.0;

            interface a {
                variant error {
                    other(string),
                }

                f: func() -> result<_, error>;
            }

            world foo {
                import a;
            }
        ",
        path: "tests/codegen/unversioned-foo.wit",
        trappable_error_type: {
            "foo:foo/a@0.1.0/error" => MyX,
        },
    });

    type MyX = u64;
}

mod trappable_errors {
    wasmtime::component::bindgen!({
        inline: "
            package demo:pkg;

            interface a {
                type b = u64;

                z1: func() -> result<_, b>;
                z2: func() -> result<_, b>;
            }

            interface b {
                use a.{b};
                z: func() -> result<_, b>;
            }

            interface c {
                type b = u64;
            }

            interface d {
                use c.{b};
                z: func() -> result<_, b>;
            }

            world foo {
                import a;
                import b;
                import d;
            }
        ",
        trappable_error_type: {
            "demo:pkg/a/b" => MyX,
            "demo:pkg/c/b" => MyX,
        },
    });

    type MyX = u32;
}

mod interface_name_with_rust_keyword {
    wasmtime::component::bindgen!({
        inline: "
            package foo:foo;

            interface crate { }

            world foo {
                export crate;
            }
        "
    });
}

mod with_works_with_hierarchy {
    mod bindings {
        wasmtime::component::bindgen!({
            inline: "
                package foo:foo;

                interface a {
                    record t {
                        x: u32,
                    }
                    x: func() -> t;
                }

                interface b {
                    use a.{t};
                    x: func() -> t;
                }

                interface c {
                    use b.{t};
                    x: func() -> t;
                }

                world foo {
                    import c;
                }
            "
        });
    }

    mod with_just_one_interface {
        wasmtime::component::bindgen!({
            inline: "
                package foo:foo;

                interface a {
                    record t {
                        x: u32,
                    }
                    x: func() -> t;
                }

                interface b {
                    use a.{t};
                    x: func() -> t;
                }

                interface c {
                    use b.{t};
                    x: func() -> t;
                }

                world foo {
                    use c.{t};

                    import x: func() -> t;
                }
            ",
            with: { "foo:foo/a": super::bindings::foo::foo::a }
        });

        struct X;

        impl FooImports for X {
            fn x(&mut self) -> super::bindings::foo::foo::a::T {
                loop {}
            }
        }
    }

    mod with_whole_package {
        wasmtime::component::bindgen!({
            inline: "
                package foo:foo;

                interface a {
                    record t {
                        x: u32,
                    }
                    x: func() -> t;
                }

                interface b {
                    use a.{t};
                    x: func() -> t;
                }

                interface c {
                    use b.{t};
                    x: func() -> t;
                }

                world foo {
                    use c.{t};

                    import x: func() -> t;
                }
            ",
            with: { "foo:foo": super::bindings::foo::foo }
        });

        struct X;

        impl FooImports for X {
            fn x(&mut self) -> super::bindings::foo::foo::a::T {
                loop {}
            }
        }
    }

    mod with_whole_namespace {
        wasmtime::component::bindgen!({
            inline: "
                package foo:foo;

                interface a {
                    record t {
                        x: u32,
                    }
                    x: func() -> t;
                }

                interface b {
                    use a.{t};
                    x: func() -> t;
                }

                interface c {
                    use b.{t};
                    x: func() -> t;
                }

                world foo {
                    use c.{t};

                    import x: func() -> t;
                }
            ",
            with: { "foo": super::bindings::foo }
        });

        struct X;

        impl FooImports for X {
            fn x(&mut self) -> super::bindings::foo::foo::a::T {
                loop {}
            }
        }
    }
}

mod trappable_imports {
    mod none {
        wasmtime::component::bindgen!({
            inline: "
                package foo:foo;

                world foo {
                    import foo: func();
                }
            ",
        });
        struct X;

        impl FooImports for X {
            fn foo(&mut self) {}
        }
    }

    mod all {
        wasmtime::component::bindgen!({
            inline: "
                package foo:foo;

                world foo {
                    import foo: func();
                }
            ",
            imports: { default: trappable },
        });
        struct X;

        impl FooImports for X {
            fn foo(&mut self) -> wasmtime::Result<()> {
                Ok(())
            }
        }
    }

    mod some {
        wasmtime::component::bindgen!({
            inline: "
                package foo:foo;

                world foo {
                    import foo: func();
                    import bar: func();
                }
            ",
            imports: { "foo": trappable },
        });
        struct X;

        impl FooImports for X {
            fn foo(&mut self) -> wasmtime::Result<()> {
                Ok(())
            }
            fn bar(&mut self) {}
        }
    }

    mod across_interfaces {
        use wasmtime::component::Resource;

        wasmtime::component::bindgen!({
            inline: "
                package foo:foo;

                interface a {
                    foo: func();
                    bar: func();

                    resource r {
                        constructor();
                        foo: func();
                        bar: static func();
                    }
                }

                world foo {
                    import a;
                    import foo: func();
                    import bar: func();
                    import i: interface {
                        foo: func();
                        bar: func();
                    }

                }
            ",
            imports: {
                "foo": trappable | exact,
                "i/foo": trappable,
                "foo:foo/a/foo": trappable,
            },
            with: { "foo:foo/a/r": R },
        });

        struct X;
        pub struct R;

        impl FooImports for X {
            fn foo(&mut self) -> wasmtime::Result<()> {
                Ok(())
            }
            fn bar(&mut self) {}
        }

        impl i::Host for X {
            fn foo(&mut self) -> wasmtime::Result<()> {
                Ok(())
            }
            fn bar(&mut self) {}
        }

        impl foo::foo::a::Host for X {
            fn foo(&mut self) -> wasmtime::Result<()> {
                Ok(())
            }
            fn bar(&mut self) {}
        }

        impl foo::foo::a::HostR for X {
            fn new(&mut self) -> Resource<R> {
                loop {}
            }
            fn foo(&mut self, _: Resource<R>) {}
            fn bar(&mut self) {}
            fn drop(&mut self, _: Resource<R>) -> wasmtime::Result<()> {
                Ok(())
            }
        }
    }

    mod resources {
        use wasmtime::component::Resource;

        wasmtime::component::bindgen!({
            inline: "
                package foo:foo;

                interface a {
                    resource r {
                        constructor();
                        foo: func();
                        bar: static func();
                    }
                }

                world foo {
                    import a;

                }
            ",
            imports: {
                "foo:foo/a/[constructor]r": trappable,
                "foo:foo/a/[method]r.foo": trappable,
                "foo:foo/a/[static]r.bar": trappable,
            },
            with: { "foo:foo/a/r": R },
        });

        struct X;
        pub struct R;

        impl foo::foo::a::Host for X {}

        impl foo::foo::a::HostR for X {
            fn new(&mut self) -> wasmtime::Result<Resource<R>> {
                loop {}
            }
            fn foo(&mut self, _: Resource<R>) -> wasmtime::Result<()> {
                Ok(())
            }
            fn bar(&mut self) -> wasmtime::Result<()> {
                Ok(())
            }
            fn drop(&mut self, _: Resource<R>) -> wasmtime::Result<()> {
                Ok(())
            }
        }
    }
}

mod custom_derives {
    use std::collections::{HashSet, hash_map::RandomState};

    wasmtime::component::bindgen!({
        inline: "
            package my:inline;

            interface blah {
                variant abc {
                    a,
                    b,
                    c
                }

                record foo {
                    field1: string,
                    field2: list<u32>,
                    field3: abc
                }

                bar: func(cool: foo);
            }

            world baz {
                import blah;
            }
        ",
        // Clone is included by default almost everywhere, so include it here to make sure it
        // doesn't conflict
        additional_derives: [serde::Serialize, serde::Deserialize, Hash, Clone, PartialEq, Eq],
    });

    use my::inline::blah::{Abc, Foo, Host};

    struct X;

    impl Host for X {
        fn bar(&mut self, cool: Foo) {
            // Check that built in derives that I've added actually work by seeing that this hashes
            let _blah: HashSet<Foo, RandomState> = HashSet::from_iter([Foo {
                field1: "hello".to_string(),
                field2: vec![1, 2, 3],
                field3: Abc::B,
            }]);

            // Check that the attributes from an external crate actually work. If they don't work,
            // compilation will fail here
            let _ = serde_json::to_string(&cool);
        }
    }
}

mod with_and_mixing_async {
    mod with_async {
        wasmtime::component::bindgen!({
            inline: "
                package my:inline;
                interface foo {
                    type t = u32;
                    foo: func() -> t;
                }
                interface bar {
                    use foo.{t};
                    bar: func() -> t;
                }
                world x {
                    import bar;
                }
            ",
            imports: {
                "my:inline/bar/bar": async,
            },
        });
    }

    mod without_async {
        wasmtime::component::bindgen!({
            inline: "
                package my:inline;
                interface foo {
                    type t = u32;
                    foo: func() -> t;
                }
                interface bar {
                    use foo.{t};
                    bar: func() -> t;
                }
                world x {
                    import bar;
                }
            ",
            with: {
                "my:inline/foo": super::with_async::my::inline::foo,
            },
            require_store_data_send: true,
        });
    }

    mod third {
        wasmtime::component::bindgen!({
            inline: "
                package my:inline;
                interface foo {
                    type t = u32;
                    foo: func() -> t;
                }
                interface bar {
                    use foo.{t};
                    bar: func() -> t;
                }
                interface baz {
                    use bar.{t};
                    baz: func() -> t;
                }
                world x {
                    import baz;
                }
            ",
            with: {
                "my:inline/foo": super::with_async::my::inline::foo,
                "my:inline/bar": super::without_async::my::inline::bar,
            },
            require_store_data_send: true,
        });
    }
}

mod trappable_error_type_and_versions {
    struct MyError;

    mod package_no_version_path_no_version {
        wasmtime::component::bindgen!({
            inline: "
                package my:inline;
                interface i {
                    enum e { a, b, c }
                }
                world foo {}
            ",
            trappable_error_type: {
                "my:inline/i/e" => super::MyError,
            },
        });
    }
    mod package_version_path_no_version {
        wasmtime::component::bindgen!({
            inline: "
                package my:inline@1.0.0;
                interface i {
                    enum e { a, b, c }
                }
                world foo {}
            ",
            trappable_error_type: {
                "my:inline/i/e" => super::MyError,
            },
        });
    }
    mod package_version_path_version {
        wasmtime::component::bindgen!({
            inline: "
                package my:inline@1.0.0;
                interface i {
                    enum e { a, b, c }
                }
                world foo {}
            ",
            trappable_error_type: {
                "my:inline/i@1.0.0/e" => super::MyError,
            },
        });
    }
}

mod paths {
    mod multiple_paths {
        wasmtime::component::bindgen!({
            world: "test:paths/test",
            inline: r#"
            package test:paths;
            world test {
                import paths:path1/test;
                export paths:path2/test;
            }
            "#,
            path: ["tests/codegen/path1", "tests/codegen/path2"],
        });
    }
}
