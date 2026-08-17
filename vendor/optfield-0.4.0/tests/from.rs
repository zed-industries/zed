use optfield::optfield;

#[test]
fn from_struct() {
    #[optfield(Opt, attrs, from)]
    #[optfield(OptRewrap, attrs, rewrap, from)]
    #[derive(Clone, Debug, PartialEq)]
    struct Original<'a, T> {
        number: u32,
        text: &'a str,
        generic: T,
        optional: Option<&'a [u8]>,
    }

    let original = Original {
        number: 12,
        text: "test",
        generic: "testing".to_string(),
        optional: Some(&[1, 2, 3, 4, 5]),
    };

    let opt = Opt::from(original.clone());
    assert_eq!(original.number, opt.number.unwrap());
    assert_eq!(original.text, opt.text.unwrap());
    assert_eq!(original.generic, opt.generic.unwrap());
    assert_eq!(original.optional, opt.optional);

    let opt_rewrap = OptRewrap::from(original.clone());
    assert_eq!(original.number, opt_rewrap.number.unwrap());
    assert_eq!(original.text, opt_rewrap.text.unwrap());
    assert_eq!(original.generic, opt_rewrap.generic.unwrap());
    assert_eq!(original.optional, opt_rewrap.optional.unwrap());
}

#[test]
fn from_tuple_struct() {
    #[optfield(Opt, attrs, from)]
    #[optfield(OptRewrap, attrs, rewrap, from)]
    #[derive(Clone, Debug, PartialEq)]
    struct Original<T>(i32, String, Option<T>);

    let original = Original(21, "test".to_string(), Some(1));
    let opt = Opt::from(original.clone());
    assert_eq!(original.0, opt.0.unwrap());
    assert_eq!(original.1, opt.1.unwrap());
    assert_eq!(original.2, opt.2);

    let opt_rewrap = OptRewrap::from(original.clone());
    assert_eq!(original.0, opt_rewrap.0.unwrap());
    assert_eq!(original.1, opt_rewrap.1.unwrap());
    assert_eq!(original.2, opt_rewrap.2.unwrap());
}

#[test]
fn from_cfg_field() {
    #![allow(unexpected_cfgs)]

    #[optfield(Opt, field_attrs, from)]
    #[derive(Clone, Debug)]
    struct Original {
        #[cfg(some_feature)]
        feature_field: String,
        field: i32,
    }

    let original = Original { field: 1 };
    let opt = Opt::from(original.clone());
    assert_eq!(original.field, opt.field.unwrap());
}
