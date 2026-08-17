// Take a look at the license at the top of the repository in the LICENSE file.

use proc_macro2::{Ident, Span, TokenStream};
use proc_macro_crate::crate_name;
use quote::{quote, quote_spanned, ToTokens};
use syn::{
    meta::ParseNestedMeta, parse::Parse, punctuated::Punctuated, spanned::Spanned, token::Comma,
    Token, Variant,
};

pub trait ParseNestedMetaItem {
    fn get_name(&self) -> &'static str;
    fn get_found(&self) -> bool;
    fn get_required(&self) -> bool;
    fn parse_nested(&mut self, meta: &ParseNestedMeta) -> Option<syn::Result<()>>;
}

#[derive(Default)]
pub struct NestedMetaItem<T> {
    pub name: &'static str,
    pub value_required: bool,
    pub found: bool,
    pub required: bool,
    pub value: Option<T>,
}

impl<T: Parse + ToTokens> std::fmt::Debug for NestedMetaItem<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NestedMetaItem")
            .field("name", &self.name)
            .field("required", &self.required)
            .field("value_required", &self.value_required)
            .field("found", &self.found)
            .field("value", &self.value.as_ref().map(|v| quote!(#v)))
            .finish()
    }
}

impl<T: Parse> NestedMetaItem<T> {
    pub const fn new(name: &'static str) -> Self {
        Self {
            required: false,
            name,
            found: false,
            value_required: false,
            value: None,
        }
    }
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }
    // Note: this flags the `value` as required, that is,
    // the parameter after the equal: `name = value`.
    pub const fn value_required(mut self) -> Self {
        self.value_required = true;
        self
    }
    pub const fn value_optional(mut self) -> Self {
        self.value_required = false;
        self
    }
    fn parse_nested_forced(&mut self, meta: &ParseNestedMeta) -> syn::Result<()> {
        if self.value_required || meta.input.peek(Token![=]) {
            let _eq: Token![=] = meta.input.parse()?;
            self.value = Some(meta.input.parse()?);
        }
        Ok(())
    }
}
impl<T: Parse> ParseNestedMetaItem for NestedMetaItem<T> {
    fn get_name(&self) -> &'static str {
        self.name
    }
    fn parse_nested(&mut self, meta: &ParseNestedMeta) -> Option<syn::Result<()>> {
        if meta.path.is_ident(self.name) {
            self.found = true;
            Some(self.parse_nested_forced(meta))
        } else {
            None
        }
    }
    fn get_found(&self) -> bool {
        self.found
    }
    fn get_required(&self) -> bool {
        self.required
    }
}

pub fn check_meta_items(span: Span, items: &mut [&mut dyn ParseNestedMetaItem]) -> syn::Result<()> {
    let mut err: Option<syn::Error> = None;
    for item in &mut *items {
        if item.get_required() && !item.get_found() {
            let nerr = syn::Error::new(
                span,
                format!("attribute `{}` must be specified", item.get_name()),
            );
            if let Some(ref mut err) = err {
                err.combine(nerr);
            } else {
                err = Some(nerr);
            }
        }
    }
    match err {
        Some(err) => Err(err),
        None => Ok(()),
    }
}
fn parse_nested_meta_items_from_fn(
    parse_nested_meta: impl FnOnce(
        &mut dyn FnMut(ParseNestedMeta) -> syn::Result<()>,
    ) -> syn::Result<()>,
    items: &mut [&mut dyn ParseNestedMetaItem],
) -> syn::Result<()> {
    parse_nested_meta(&mut |meta| {
        for item in &mut *items {
            if let Some(res) = item.parse_nested(&meta) {
                return res;
            }
        }
        Err(meta.error(format!(
            "unknown attribute `{}`. Possible attributes are {}",
            meta.path.get_ident().unwrap(),
            items
                .iter()
                .map(|i| format!("`{}`", i.get_name()))
                .collect::<Vec<_>>()
                .join(", ")
        )))
    })?;
    Ok(())
}

pub fn parse_nested_meta_items_from_stream(
    input: TokenStream,
    items: &mut [&mut dyn ParseNestedMetaItem],
) -> syn::Result<()> {
    parse_nested_meta_items_from_fn(
        |f| {
            let p = syn::meta::parser(f);
            syn::parse::Parser::parse(p, input.into())
        },
        items,
    )?;
    check_meta_items(Span::call_site(), items)
}

pub fn parse_nested_meta_items<'a>(
    attrs: impl IntoIterator<Item = &'a syn::Attribute>,
    attr_name: &str,
    items: &mut [&mut dyn ParseNestedMetaItem],
) -> syn::Result<Option<&'a syn::Attribute>> {
    let attr = attrs
        .into_iter()
        .find(|attr| attr.path().is_ident(attr_name));
    if let Some(attr) = attr {
        parse_nested_meta_items_from_fn(|x| attr.parse_nested_meta(x), items)?;
        check_meta_items(attr.span(), items)?;
        Ok(Some(attr))
    } else {
        Ok(None)
    }
}

pub fn parse_optional_nested_meta_items<'a>(
    attrs: impl IntoIterator<Item = &'a syn::Attribute>,
    attr_name: &str,
    items: &mut [&mut dyn ParseNestedMetaItem],
) -> syn::Result<Option<&'a syn::Attribute>> {
    let attr = attrs
        .into_iter()
        .find(|attr| attr.path().is_ident(attr_name));
    if let Some(attr) = attr {
        if let syn::Meta::Path(_) = attr.meta {
            Ok(Some(attr))
        } else {
            parse_nested_meta_items_from_fn(|x| attr.parse_nested_meta(x), items)?;
            check_meta_items(attr.span(), items)?;
            Ok(Some(attr))
        }
    } else {
        Ok(None)
    }
}

pub fn crate_ident_new() -> TokenStream {
    use proc_macro_crate::FoundCrate;

    match crate_name("glib") {
        Ok(FoundCrate::Name(name)) => Some(name),
        Ok(FoundCrate::Itself) => Some("glib".to_string()),
        Err(_) => None,
    }
    .map(|s| {
        let glib = Ident::new(&s, Span::call_site());
        quote!(#glib)
    })
    .unwrap_or_else(|| {
        // We couldn't find the glib crate (renamed or not) so let's just hope it's in scope!
        //
        // We will be able to have this information once this code is stable:
        //
        // ```
        // let span = Span::call_site();
        // let source = span.source_file();
        // let file_path = source.path();
        // ```
        //
        // Then we can use proc_macro to parse the file and check if glib is imported somehow.
        let glib = Ident::new("glib", Span::call_site());
        quote!(#glib)
    })
}

// Generate i32 to enum mapping, used to implement
// glib::translate::TryFromGlib<i32>, such as:
//
//   if value == Animal::Goat as i32 {
//       return Some(Animal::Goat);
//   }
pub fn gen_enum_from_glib(
    enum_name: &Ident,
    enum_variants: &Punctuated<Variant, Comma>,
) -> TokenStream {
    // FIXME: can we express this with a match()?
    let recurse = enum_variants.iter().map(|v| {
        let name = &v.ident;
        quote_spanned! { v.span() =>
            if value == #enum_name::#name as i32 {
                return ::core::option::Option::Some(#enum_name::#name);
            }
        }
    });
    quote! {
        #(#recurse)*
        ::core::option::Option::None
    }
}

// These tests are useful to pinpoint the exact location of a macro panic
// by running `cargo test --lib`
#[cfg(test)]
mod tests {
    use syn::{parse_quote, DeriveInput};

    use super::*;

    fn boxed_stub() -> DeriveInput {
        parse_quote!(
            #[boxed_type(name = "Author")]
            struct Author {
                name: String,
            }
        )
    }

    #[test]
    fn check_attr_found() {
        let input = boxed_stub();
        let found = parse_nested_meta_items(&input.attrs, "boxed_type", &mut []);
        matches!(found, Ok(Some(_)));
    }
    #[test]
    fn required_name_present() {
        let input = boxed_stub();
        let mut gtype_name = NestedMetaItem::<syn::LitStr>::new("name")
            .required()
            .value_required();
        let _ = parse_nested_meta_items(&input.attrs, "boxed_type", &mut [&mut gtype_name]);
        assert!(gtype_name.get_found());
        assert_eq!(
            gtype_name.value.map(|x| x.value()),
            Some("Author".to_string())
        );
    }
    #[test]
    fn required_name_none() {
        let input: DeriveInput = parse_quote!(
            #[boxed_type(name)]
            struct Author {
                name: String,
            }
        );
        let mut gtype_name = NestedMetaItem::<syn::LitStr>::new("name")
            .required()
            .value_required();
        let found = parse_nested_meta_items(&input.attrs, "boxed_type", &mut [&mut gtype_name]);
        // The argument value was specified as required, so an error is returned
        assert!(found.is_err());
        assert!(gtype_name.value.is_none());

        // The argument key must be found though
        assert!(gtype_name.get_found());
    }
}
