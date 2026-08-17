#[macro_use]
extern crate pretty_assertions;
#[macro_use]
extern crate derive_builder;

#[derive(Debug, PartialEq, Default, Builder, Clone)]
struct Lorem {
    #[builder(setter(into))]
    full_opt: Option<String>,
    #[builder(setter(into, strip_option))]
    strip_opt: Option<String>,
    #[builder(setter(strip_option))]
    strip_opt_i32: Option<i32>,
    #[builder(setter(strip_option))]
    strip_opt_vec: Option<Vec<i32>>,
}

#[derive(Debug, PartialEq, Default, Builder, Clone)]
#[builder(setter(into, strip_option))]
struct Ipsum {
    foo: u32,
    strip_opt: Option<String>,
    #[builder(default)]
    strip_opt_with_default: Option<String>,
}

#[test]
fn generic_field() {
    let x = LoremBuilder::default()
        .full_opt(Some("foo".to_string()))
        .strip_opt("bar")
        .strip_opt_i32(32)
        .strip_opt_vec(vec![33])
        .build()
        .unwrap();

    assert_eq!(
        x,
        Lorem {
            full_opt: Some("foo".to_string()),
            strip_opt: Some("bar".to_string()),
            strip_opt_i32: Some(32),
            strip_opt_vec: Some(vec![33]),
        }
    );
}

#[test]
fn generic_struct() {
    let x = IpsumBuilder::default()
        .foo(42u8)
        .strip_opt("bar")
        .build()
        .unwrap();

    assert_eq!(
        x,
        Ipsum {
            foo: 42u32,
            strip_opt: Some("bar".to_string()),
            strip_opt_with_default: None,
        }
    );
}
