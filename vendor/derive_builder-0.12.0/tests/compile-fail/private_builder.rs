#[macro_use]
extern crate derive_builder;

pub mod foo {
    /// The builder struct's declaration of privacy should override the field's
    /// attempt to be public later on.
    #[derive(Debug, PartialEq, Default, Builder, Clone)]
    #[builder(private, setter(into))]
    pub struct Lorem {
        pub private: String,
        #[builder(public)]
        pub public: String,
    }
}

fn main() {
    let x = foo::LoremBuilder::default()
        .public("Hello")
        .build()
        .unwrap();

    assert_eq!(x.public, "Hello".to_string());
}
