#[macro_use]
extern crate pretty_assertions;
#[macro_use]
extern crate derive_builder;

#[derive(Debug, PartialEq, Default, Builder, Clone)]
#[builder(setter(name = "foo"))]
struct Lorem {
    ipsum: &'static str,
    pub dolor: &'static str,
}

fn main() {
    let x = LoremBuilder::default()
        .ipsum("ipsum")
        .foo("dolor")
        .build()
        .unwrap();

    assert_eq!(x,
               Lorem {
                   ipsum: "ipsum",
                   dolor: "dolor",
               });
}
