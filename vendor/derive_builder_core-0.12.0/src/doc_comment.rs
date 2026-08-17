use syn::Attribute;

/// Doc-comment, implementing `quote::ToTokens`.
///
/// # Examples
///
/// Will expand to something like the following (depending on inner value):
///
/// ```rust,ignore
/// # #[macro_use]
/// # extern crate quote;
/// # extern crate syn;
/// # extern crate derive_builder_core;
/// # use derive_builder_core::doc_comment_from;
/// # fn main() {
/// #    let doc_comment = doc_comment_from("foo".to_string());
/// #
/// #    assert_eq!(quote!(#doc_comment).to_string(), quote!(
/// #[doc = "foo"]
/// #    ).to_string());
/// # }
/// ```
pub fn doc_comment_from(s: String) -> Attribute {
    parse_quote!(#[doc=#s])
}
