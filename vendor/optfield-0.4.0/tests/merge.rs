use optfield::optfield;

#[test]
fn merge_struct() {
    #[optfield(Opt, attrs, merge_fn)]
    #[optfield(OptRewrap, attrs, rewrap, merge_fn = merge_rewrap)]
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

    let opt = Opt {
        number: None,
        text: None,
        generic: None,
        optional: None,
    };

    let mut original_clone = original.clone();

    original_clone.merge_opt(opt.clone());
    assert_eq!(original, original_clone);

    let opt2 = Opt {
        number: Some(99),
        ..opt.clone()
    };

    original_clone.merge_opt(opt2.clone());

    assert_eq!(original_clone.number, opt2.number.unwrap());
    assert_eq!(original_clone.text, original.text);
    assert_eq!(original_clone.generic, original.generic);
    assert_eq!(original_clone.optional, original.optional);

    let opt3 = Opt {
        text: Some("testing optional str field"),
        ..opt.clone()
    };

    original_clone = original.clone();
    original_clone.merge_opt(opt3.clone());

    assert_eq!(original_clone.text, opt3.text.unwrap());
    assert_eq!(original_clone.number, original.number);
    assert_eq!(original_clone.generic, original.generic);
    assert_eq!(original_clone.optional, original.optional);

    let opt4 = Opt {
        generic: Some("generic string test".to_string()),
        ..opt
    };

    original_clone = original.clone();
    original_clone.merge_opt(opt4.clone());

    assert_eq!(original_clone.generic, opt4.generic.unwrap());
    assert_eq!(original_clone.number, original.number);
    assert_eq!(original_clone.text, original.text);
    assert_eq!(original_clone.optional, original.optional);

    let opt5 = Opt {
        optional: Some(&[6, 7, 8, 9]),
        ..opt
    };

    original_clone = original.clone();
    original_clone.merge_opt(opt5.clone());

    assert_eq!(original_clone.optional, opt5.optional);
    assert_eq!(original_clone.number, original.number);
    assert_eq!(original_clone.text, original.text);
    assert_eq!(original_clone.generic, original.generic);

    let opt6 = Opt {
        number: Some(200),
        text: Some("testing all fields"),
        generic: Some("with some values".to_string()),
        optional: Some(&[0, 8, 3, 5]),
    };

    original_clone = original.clone();
    original_clone.merge_opt(opt6.clone());

    assert_eq!(original_clone.number, opt6.number.unwrap());
    assert_eq!(original_clone.text, opt6.text.unwrap());
    assert_eq!(original_clone.generic, opt6.generic.unwrap());
    assert_eq!(original_clone.optional, opt6.optional);

    let opt7 = OptRewrap {
        number: None,
        text: None,
        generic: None,
        optional: Some(Some(&[0])),
    };

    original_clone = original.clone();
    original_clone.merge_rewrap(opt7.clone());

    assert_eq!(original_clone.optional, opt7.optional.unwrap());
    assert_eq!(original_clone.number, original.number);
    assert_eq!(original_clone.text, original.text);
    assert_eq!(original_clone.generic, original.generic);
}

#[test]
fn merge_tuple_struct() {
    #[optfield(Opt, attrs, merge_fn)]
    #[derive(Clone, Debug, PartialEq)]
    struct Original(i32, String);

    let original = Original(21, "test".to_string());

    let mut original_clone = original.clone();

    let opt = Opt(None, None);

    original_clone.merge_opt(opt);

    assert_eq!(original_clone, original);

    let opt2 = Opt(Some(345), None);

    original_clone.merge_opt(opt2.clone());

    assert_eq!(original_clone.0, opt2.0.unwrap());
    assert_eq!(original_clone.1, original.1);

    let opt3 = Opt(None, Some("optional string".to_string()));

    original_clone = original.clone();
    original_clone.merge_opt(opt3.clone());

    assert_eq!(original_clone.1, opt3.1.unwrap());
    assert_eq!(original_clone.0, original.0);

    let opt4 = Opt(Some(789), Some("optional test string".to_string()));

    original_clone = original.clone();
    original_clone.merge_opt(opt4.clone());

    assert_eq!(original_clone.0, opt4.0.unwrap());
    assert_eq!(original_clone.1, opt4.1.unwrap());
}

#[test]
fn merge_cfg_field() {
    #![allow(unexpected_cfgs)]

    #[optfield(Opt, attrs, merge_fn)]
    #[optfield(OptFieldAttrs, field_attrs, attrs, merge_fn = merge_opt_attrs)]
    #[derive(Clone, Debug)]
    struct Original {
        #[cfg(some_feature)]
        feature_field: String,
        field: i32,
    }

    let mut original = Original { field: 2 };
    let mut opt = Opt {
        field: None,
        feature_field: None,
    };

    original.merge_opt(opt.clone());
    assert_eq!(original.field, 2);

    opt.field = Some(3);
    opt.feature_field = Some("test".to_string());

    original.merge_opt(opt);
    assert_eq!(original.field, 3);

    let mut opt_attrs = OptFieldAttrs { field: None };

    original.merge_opt_attrs(opt_attrs.clone());
    assert_eq!(original.field, 3);

    opt_attrs.field = Some(4);

    original.merge_opt_attrs(opt_attrs);
    assert_eq!(original.field, 4);
}
