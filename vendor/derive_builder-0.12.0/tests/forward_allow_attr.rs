#![deny(non_snake_case)]

#[macro_use]
extern crate derive_builder;

#[derive(Builder)]
// If this attribute is not forwarded to both the struct and the impl block, there would
// be a compile error on either the field or the setter method name. Therefore, forwarding
// is working as-expected if this test compiles.
#[allow(non_snake_case)]
pub struct Example {
    aPascalName: &'static str,
}

fn main() {
    assert_eq!(
        ExampleBuilder::default()
            .aPascalName("hello")
            .build()
            .unwrap()
            .aPascalName,
        "hello"
    );
}
