#[macro_use]
extern crate pretty_assertions;
#[macro_use]
extern crate derive_builder;

mod field_level {
    use derive_builder::UninitializedFieldError;
    #[derive(Debug, PartialEq, Default, Builder, Clone)]
    struct Lorem {
        required: String,
        #[builder(default)]
        explicit_default: String,
        #[builder(default = "\"foo\".to_string()")]
        escaped_default: String,
        #[builder(default = r#"format!("Hello {}!", "World")"#)]
        raw_default: String,
        #[builder(default = r#"format!("{}-{}-{}-{}",
                             Clone::clone(self.required
                                .as_ref()
                                .ok_or_else(|| UninitializedFieldError::new("required"))?),
                             match self.explicit_default { Some(ref x) => x, None => "EMPTY" },
                             self.escaped_default.as_ref().map(|x| x.as_ref()).unwrap_or("EMPTY"),
                             if let Some(ref x) = self.raw_default { x } else { "EMPTY" })"#)]
        computed_default: String,
    }

    #[test]
    fn error_if_uninitialized() {
        let error = LoremBuilder::default().build().unwrap_err();
        assert_eq!(&error.to_string(), "`required` must be initialized");
    }

    #[test]
    fn custom_default() {
        let x = LoremBuilder::default()
            .required("ipsum".to_string())
            .build()
            .unwrap();

        assert_eq!(
            x,
            Lorem {
                required: "ipsum".to_string(),
                explicit_default: "".to_string(),
                escaped_default: "foo".to_string(),
                raw_default: "Hello World!".to_string(),
                computed_default: "ipsum-EMPTY-EMPTY-EMPTY".to_string(),
            }
        );
    }

    #[test]
    fn builder_test() {
        let x = LoremBuilder::default()
            .required("ipsum".to_string())
            .explicit_default("lorem".to_string())
            .escaped_default("dolor".to_string())
            .raw_default("sit".to_string())
            .build()
            .unwrap();

        assert_eq!(
            x,
            Lorem {
                required: "ipsum".to_string(),
                explicit_default: "lorem".to_string(),
                escaped_default: "dolor".to_string(),
                raw_default: "sit".to_string(),
                computed_default: "ipsum-lorem-dolor-sit".to_string(),
            }
        );
    }
}

mod struct_level {
    #[derive(Debug, Clone, PartialEq, Eq, Builder)]
    #[builder(default = "explicit_default()")]
    struct Lorem {
        #[builder(default = "true")]
        overwritten: bool,
        not_type_default: Option<&'static str>,
    }

    fn explicit_default() -> Lorem {
        Lorem {
            overwritten: false,
            not_type_default: Some("defined on struct-level"),
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, Builder)]
    #[builder(default)]
    struct Ipsum {
        not_type_default: Option<u16>,
        also_custom: bool,
        is_type_default: String,
    }

    impl Default for Ipsum {
        fn default() -> Self {
            Ipsum {
                not_type_default: Some(20),
                also_custom: true,
                is_type_default: Default::default(),
            }
        }
    }

    #[test]
    fn explicit_defaults_are_equal() {
        let lorem = LoremBuilder::default().build().unwrap();

        assert_eq!(
            lorem,
            Lorem {
                overwritten: true,
                ..explicit_default()
            }
        );
    }

    #[test]
    fn implicit_defaults_are_equal() {
        let ipsum = IpsumBuilder::default().build().unwrap();

        assert_eq!(ipsum, Ipsum::default());
    }

    #[test]
    fn overrides_work() {
        let ipsum = IpsumBuilder::default()
            .not_type_default(None)
            .build()
            .expect("Struct-level default makes all fields optional");

        assert_eq!(ipsum.not_type_default, None);
    }
}
