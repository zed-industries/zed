use darling::{Error, FromMeta};
use syn::parse_quote;

#[derive(Debug, FromMeta)]
struct Meta {
    #[darling(default)]
    meta1: Option<String>,
    #[darling(default)]
    meta2: bool,
}

#[test]
fn nested_meta_meta_value() {
    let meta = Meta::from_list(&[parse_quote! {
        meta1 = "thefeature"
    }])
    .unwrap();
    assert_eq!(meta.meta1, Some("thefeature".to_string()));
    assert!(!meta.meta2);
}

#[test]
fn nested_meta_meta_bool() {
    let meta = Meta::from_list(&[parse_quote! {
        meta2
    }])
    .unwrap();
    assert_eq!(meta.meta1, None);
    assert!(meta.meta2);
}

#[test]
fn nested_meta_lit_string_errors() {
    let err = Meta::from_list(&[parse_quote! {
        "meta2"
    }])
    .unwrap_err();
    assert_eq!(
        err.to_string(),
        Error::unsupported_format("literal").to_string()
    );
}

#[test]
fn nested_meta_lit_integer_errors() {
    let err = Meta::from_list(&[parse_quote! {
        2
    }])
    .unwrap_err();
    assert_eq!(
        err.to_string(),
        Error::unsupported_format("literal").to_string()
    );
}

#[test]
fn nested_meta_lit_bool_errors() {
    let err = Meta::from_list(&[parse_quote! {
        true
    }])
    .unwrap_err();
    assert_eq!(
        err.to_string(),
        Error::unsupported_format("literal").to_string()
    );
}
