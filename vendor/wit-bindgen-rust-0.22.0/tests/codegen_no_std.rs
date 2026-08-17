//! Like `codegen_tests` in codegen.rs, but with no_std.
//!
//! We use `std_feature` and don't enable the "std" feature.

#![no_std]
#![allow(unused_macros)]
#![allow(dead_code, unused_variables)]

// This test expects `"std"` to be absent.
#[cfg(feature = "std")]
fn std_enabled() -> CompileError;

extern crate alloc;

mod codegen_tests {
    macro_rules! codegen_test {
        ($id:ident $name:tt $test:tt) => {
            mod $id {
                wit_bindgen::generate!({
                    path: $test,
                    std_feature,
                    stubs
                });

                #[test]
                fn works() {}
            }

        };
    }
    test_helpers::codegen_tests!();
}

mod strings {
    use alloc::string::String;

    wit_bindgen::generate!({
        inline: "
            package my:strings;
            world not-used-name {
                import cat: interface {
                    foo: func(x: string);
                    bar: func() -> string;
                }
            }
        ",
        std_feature,
    });

    #[allow(dead_code)]
    fn test() {
        // Test the argument is `&str`.
        cat::foo("hello");

        // Test the return type is `String`.
        let _t: String = cat::bar();
    }
}

/// Like `strings` but with raw_strings`.
mod raw_strings {
    use alloc::vec::Vec;

    wit_bindgen::generate!({
        inline: "
            package raw:strings;
            world not-used-name {
                import cat: interface {
                    foo: func(x: string);
                    bar: func() -> string;
                }
            }
        ",
        raw_strings,
        std_feature,
    });

    #[allow(dead_code)]
    fn test() {
        // Test the argument is `&[u8]`.
        cat::foo(b"hello");

        // Test the return type is `Vec<u8>`.
        let _t: Vec<u8> = cat::bar();
    }
}

mod skip {
    wit_bindgen::generate!({
        inline: "
            package foo:foo;
            world baz {
                export exports: interface {
                    foo: func();
                    bar: func();
                }
            }
        ",
        skip: ["foo"],
        std_feature,
    });

    struct Component;

    impl exports::exports::Guest for Component {
        fn bar() {}
    }

    export!(Component);
}
