#[macro_use]
extern crate pretty_assertions;
#[macro_use]
extern crate derive_builder;

pub mod foo {
    #[derive(Debug, PartialEq, Default, Builder, Clone)]
    #[builder(private, setter(into))]
    pub struct Lorem {
        pub private: String,
        #[builder(public)]
        pub public: String,
    }

    #[derive(Debug, PartialEq, Default, Builder, Clone)]
    #[builder(public, setter(into))]
    pub struct Ipsum {
        #[builder(private)]
        pub private: String,
        pub public: String,
    }

    #[test]
    fn setters_same_module() {
        let x = LoremBuilder::default()
            .public("Hello")
            .private("world!")
            .build()
            .unwrap();

        assert_eq!(
            x,
            Lorem {
                public: "Hello".into(),
                private: "world!".into(),
            }
        );

        let y = IpsumBuilder::default()
            .public("Hello")
            .private("world!")
            .build()
            .unwrap();

        assert_eq!(
            y,
            Ipsum {
                public: "Hello".into(),
                private: "world!".into(),
            }
        );
    }
}

#[test]
fn public_setters_foreign_module() {
    let error = foo::IpsumBuilder::default()
        .public("Hello")
        .build()
        .unwrap_err();

    assert_eq!(&error.to_string(), "`private` must be initialized");
}
