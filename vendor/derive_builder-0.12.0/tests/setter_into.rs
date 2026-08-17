#[macro_use]
extern crate pretty_assertions;
#[macro_use]
extern crate derive_builder;

#[derive(Debug, PartialEq, Default, Builder, Clone)]
struct Lorem {
    #[builder(setter(into))]
    foo: String,
}

#[derive(Debug, PartialEq, Default, Builder, Clone)]
#[builder(setter(into))]
struct Ipsum {
    foo: u32,
}

#[test]
fn generic_field() {
    let x = LoremBuilder::default().foo("foo").build().unwrap();

    assert_eq!(
        x,
        Lorem {
            foo: "foo".to_string()
        }
    );
}

#[test]
fn generic_struct() {
    let x = IpsumBuilder::default().foo(42u8).build().unwrap();

    assert_eq!(x, Ipsum { foo: 42u32 });
}
