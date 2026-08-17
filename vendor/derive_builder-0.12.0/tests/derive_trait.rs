#![allow(dead_code)]

#[macro_use]
extern crate derive_builder;

#[derive(Debug, Default, Clone)]
struct NotPartialEq(String);

#[derive(Debug, Clone, Builder)]
#[builder(derive(Debug, PartialEq, Eq))]
struct Lorem {
    foo: u8,

    /// This type doesn't have `PartialEq` support, but that's fine
    /// since we don't want it in the builder.
    #[builder(setter(skip))]
    excluded: NotPartialEq,
}

#[test]
fn defaults() {
    // This macro requires that the two sides implement `PartialEq` AND `Debug`,
    // so this one line is testing that the requested traits were really generated.
    assert_eq!(LoremBuilder::default(), LoremBuilder::default());
}
