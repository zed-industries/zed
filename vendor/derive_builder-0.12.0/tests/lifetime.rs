#[macro_use]
extern crate pretty_assertions;
#[macro_use]
extern crate derive_builder;

#[derive(Debug, PartialEq, Default, Builder, Clone)]
struct Lorem<'a> {
    ipsum: &'a str,
}

#[test]
fn error_if_uninitialized() {
    let error = LoremBuilder::default().build().unwrap_err();
    assert_eq!(&error.to_string(), "`ipsum` must be initialized");
}

#[test]
fn builder_test() {
    let x = LoremBuilder::default().ipsum("ipsum").build().unwrap();

    assert_eq!(x, Lorem { ipsum: "ipsum" });
}
