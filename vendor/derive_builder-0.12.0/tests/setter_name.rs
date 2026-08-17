#[macro_use]
extern crate pretty_assertions;
#[macro_use]
extern crate derive_builder;

#[derive(Debug, PartialEq, Default, Builder, Clone)]
#[builder(setter(prefix = "with"))]
struct Lorem {
    ipsum: &'static str,
    #[builder(setter(name = "foo"))]
    pub dolor: &'static str,
}

#[test]
fn renamed_setter() {
    let x = LoremBuilder::default()
        .with_ipsum("ipsum")
        .foo("dolor")
        .build()
        .unwrap();

    assert_eq!(
        x,
        Lorem {
            ipsum: "ipsum",
            dolor: "dolor",
        }
    );
}
