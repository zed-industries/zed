#[macro_use]
extern crate pretty_assertions;
#[macro_use]
extern crate derive_builder;

#[derive(Debug, PartialEq, Default, Builder, Clone)]
#[builder(setter(prefix = "with"))]
struct Lorem {
    ipsum: &'static str,
    #[builder(setter(prefix = "set"))]
    pub dolor: &'static str,
}

#[test]
fn prefixed_setters() {
    let x = LoremBuilder::default()
        .with_ipsum("ipsum")
        .set_dolor("dolor")
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
