#[macro_use]
extern crate pretty_assertions;
#[macro_use]
extern crate derive_builder;

#[derive(Debug, PartialEq, Default, Builder, Clone)]
#[builder(name = "MyBuilder")]
struct Lorem {
    ipsum: &'static str,
    pub dolor: Option<&'static str>,
    pub sit: i32,
    amet: bool,
}

#[test]
fn error_if_uninitialized() {
    let error = MyBuilder::default().build().unwrap_err();
    assert_eq!(&error.to_string(), "`ipsum` must be initialized");
}

#[test]
fn builder_test() {
    let x: Lorem = MyBuilder::default()
        .ipsum("lorem")
        .dolor(Some("dolor"))
        .sit(42)
        .amet(true)
        .build()
        .unwrap();

    assert_eq!(
        x,
        Lorem {
            ipsum: "lorem",
            dolor: Some("dolor"),
            sit: 42,
            amet: true,
        }
    );
}
