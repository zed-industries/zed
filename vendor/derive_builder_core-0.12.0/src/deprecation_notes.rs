use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, TokenStreamExt};
use syn;

/// Deprecation notes we want to emit to the user, implementing
/// `quote::ToTokens`.
///
/// Can be expanded at every place that accepts statements and item definitions
/// (e.g. function bodys).
///
/// # Examples
///
/// Will expand to something like the following (depending on settings):
///
/// ```rust,ignore
/// # #[macro_use]
/// # extern crate quote;
/// # extern crate derive_builder_core;
/// # use derive_builder_core::DeprecationNotes;
/// # fn main() {
/// #    let mut note = DeprecationNotes::default();
/// #    note.push("Some Warning".to_string());
/// #    assert_eq!(quote!(#note).to_string(), quote!(
///         {
///             #[deprecated(note = "Some Warning")]
///             fn derive_builder_core_deprecation_note() { }
///             derive_builder_core_deprecation_note();
///         }
/// #    ).to_string());
/// # }
/// ```
///
/// This will emit a deprecation warning in the downstream crate. Cool stuff. ^^
///
/// Proof of concept:
/// - <https://play.rust-lang.org/?gist=8394141c07d1f6d75d314818389eb4d8>
#[derive(Debug, Default, Clone)]
pub struct DeprecationNotes(Vec<String>);

impl ToTokens for DeprecationNotes {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for note in &self.0 {
            let fn_ident =
                syn::Ident::new("derive_builder_core_deprecation_note", Span::call_site());
            tokens.append_all(quote!(
                {
                    #[deprecated(note=#note)]
                    fn #fn_ident() { }
                    #fn_ident();
                }
            ));
        }
    }
}

impl DeprecationNotes {
    /// Appends a note to the collection.
    #[cfg(test)]
    pub fn push(&mut self, note: String) {
        self.0.push(note)
    }

    /// Create a view of these deprecation notes that can annotate a struct.
    pub const fn as_item(&self) -> DeprecationNotesAsItem {
        DeprecationNotesAsItem(self)
    }
}

/// A view of `DeprecationNotes` that can be used in any context that accept
/// items.
///
/// Expands to a function `__deprecation_notes` which emits the notes.
#[derive(Debug)]
pub struct DeprecationNotesAsItem<'a>(&'a DeprecationNotes);

impl<'a> ToTokens for DeprecationNotesAsItem<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let deprecation_notes = self.0;

        if !deprecation_notes.0.is_empty() {
            tokens.append_all(quote!(
                #[doc(hidden)]
                fn derive_builder_core_deprecation_note() {
                    #deprecation_notes
                }
            ))
        }
    }
}

#[test]
fn deprecation_note() {
    let mut note = DeprecationNotes::default();
    note.push("Some Warning".to_string());
    assert_eq!(
        quote!(#note).to_string(),
        quote!({
            #[deprecated(note = "Some Warning")]
            fn derive_builder_core_deprecation_note() {}
            derive_builder_core_deprecation_note();
        })
        .to_string()
    );
}
