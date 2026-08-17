use darling::FromDeriveInput;
use syn::{parse_quote, Ident, LitStr, Path};

#[derive(Debug, FromDeriveInput)]
#[darling(supports(struct_unit), attributes(bar))]
pub struct Bar {
    pub ident: Ident,
    pub st: Path,
    pub file: LitStr,
}

/// Per [#96](https://github.com/TedDriggs/darling/issues/96), make sure that an
/// attribute which isn't a valid meta gets an error.
#[test]
fn non_meta_attribute_gets_own_error() {
    let di = parse_quote! {
        #[derive(Bar)]
        #[bar(file = "motors/example_6.csv", st = RocketEngine)]
        pub struct EstesC6;
    };

    let errors: darling::Error = Bar::from_derive_input(&di).unwrap_err().flatten();
    // The number of errors here is 1 for the bad attribute + 2 for the missing fields
    assert_eq!(3, errors.len());
    // Make sure one of the errors propagates the syn error
    assert!(errors
        .into_iter()
        .any(|e| e.to_string().contains("expected lit")));
}

/// Properties can be split across multiple attributes; this test ensures that one
/// non-meta attribute does not interfere with the parsing of other, well-formed attributes.
#[test]
fn non_meta_attribute_does_not_block_others() {
    let di = parse_quote! {
        #[derive(Bar)]
        #[bar(st = RocketEngine)]
        #[bar(file = "motors/example_6.csv")]
        pub struct EstesC6;
    };

    let errors: darling::Error = Bar::from_derive_input(&di).unwrap_err().flatten();
    // The number of errors here is 1 for the bad attribute + 1 for the missing "st" field
    assert_eq!(2, errors.len());
    // Make sure one of the errors propagates the syn error
    assert!(errors
        .into_iter()
        .any(|e| e.to_string().contains("expected lit")));
}
