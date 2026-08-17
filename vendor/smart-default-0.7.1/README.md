[![Build Status](https://github.com/idanarye/rust-smart-default/workflows/CI/badge.svg)](https://github.com/idanarye/rust-smart-default/actions)
[![Latest Version](https://img.shields.io/crates/v/smart-default.svg)](https://crates.io/crates/smart-default)
[![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://idanarye.github.io/rust-smart-default/)

# Rust SmartDefault

Custom derive for automatically implementing the `Default` trait with customized default values:

```rust
use smart_default::SmartDefault;

#[derive(SmartDefault)]
enum Foo {
    Bar,
    #[default]
    Baz {
        #[default = 12]
        a: i32,
        b: i32,
        #[default(Some(Default::default()))]
        c: Option<i32>,
        #[default(_code = "vec![1, 2, 3]")]
        d: Vec<u32>,
        #[default = "four"]
        e: String,
    },
    Qux(i32),
}

assert!(Foo::default() == Foo::Baz {
    a: 12,
    b: 0,
    c: Some(0),
    d: vec![1, 2, 3],
    e: "four".to_owned(),
});
```

Requires Rust 1.30+ (for non-string values in attributes)
