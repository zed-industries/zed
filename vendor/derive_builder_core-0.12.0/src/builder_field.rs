use std::borrow::Cow;

use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt};
use syn;

/// Field for the builder struct, implementing `quote::ToTokens`.
///
/// # Examples
///
/// Will expand to something like the following (depending on settings):
///
/// ```rust,ignore
/// # extern crate proc_macro2;
/// # #[macro_use]
/// # extern crate quote;
/// # #[macro_use]
/// # extern crate syn;
/// # #[macro_use]
/// # extern crate derive_builder_core;
/// # use derive_builder_core::{BuilderField, BuilderPattern};
/// # fn main() {
/// #    let attrs = vec![parse_quote!(#[some_attr])];
/// #    let mut field = default_builder_field!();
/// #    field.attrs = attrs.as_slice();
/// #
/// #    assert_eq!(quote!(#field).to_string(), quote!(
/// #[some_attr] pub foo: ::derive_builder::export::core::option::Option<String>,
/// #    ).to_string());
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct BuilderField<'a> {
    /// Path to the root of the derive_builder crate.
    pub crate_root: &'a syn::Path,
    /// Name of the target field.
    pub field_ident: &'a syn::Ident,
    /// Type of the builder field.
    pub field_type: BuilderFieldType<'a>,
    /// Visibility of this builder field, e.g. `syn::Visibility::Public`.
    pub field_visibility: Cow<'a, syn::Visibility>,
    /// Attributes which will be attached to this builder field.
    pub attrs: &'a [syn::Attribute],
}

impl<'a> ToTokens for BuilderField<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = self.field_ident;
        let vis = &self.field_visibility;
        let ty = &self.field_type.with_crate_root(self.crate_root);
        let attrs = self.attrs;
        tokens.append_all(quote!(
            #(#attrs)* #vis #ident: #ty,
        ));
    }
}

impl<'a> BuilderField<'a> {
    /// Emits a struct field initializer that initializes the field to `Default::default`.
    pub fn default_initializer_tokens(&self) -> TokenStream {
        let ident = self.field_ident;
        let crate_root = self.crate_root;
        quote! { #ident : #crate_root::export::core::default::Default::default(), }
    }
}

/// The type of a field in the builder struct
#[derive(Debug, Clone)]
pub enum BuilderFieldType<'a> {
    /// The corresonding builder field will be `Option<field_type>`.
    Optional(&'a syn::Type),
    /// The corresponding builder field will be just this type
    Precise(&'a syn::Type),
    /// The corresponding builder field will be a PhantomData
    ///
    /// We do this if if the field is disabled.  We mustn't just completely omit the field from the builder:
    /// if we did that, the builder might have unused generic parameters (since we copy the generics from
    /// the target struct).   Using a PhantomData of the original field type provides the right generic usage
    /// (and the right variance).  The alternative would be to give the user a way to separately control
    /// the generics of the builder struct, which would be very awkward to use and complex to document.
    /// We could just include the field anyway, as `Option<T>`, but this is wasteful of space, and it
    /// seems good to explicitly suppress the existence of a variable that won't be set or read.
    Phantom(&'a syn::Type),
}

impl<'a> BuilderFieldType<'a> {
    /// Obtain type information for the builder field setter
    ///
    /// Return value:
    ///  * `.0`: type of the argument to the setter function
    ///          (before application of `strip_option`, `into`)
    ///  * `.1`: whether the builder field is `Option<type>` rather than just `type`
    pub fn setter_type_info(&'a self) -> (&'a syn::Type, bool) {
        match self {
            BuilderFieldType::Optional(ty) => (ty, true),
            BuilderFieldType::Precise(ty) => (ty, false),
            BuilderFieldType::Phantom(_ty) => panic!("phantom fields should never have setters"),
        }
    }

    fn with_crate_root(&'a self, crate_root: &'a syn::Path) -> BuilderFieldTypeWithCrateRoot<'a> {
        BuilderFieldTypeWithCrateRoot {
            crate_root,
            field_type: self,
        }
    }
}

struct BuilderFieldTypeWithCrateRoot<'a> {
    crate_root: &'a syn::Path,
    field_type: &'a BuilderFieldType<'a>,
}

impl<'a> ToTokens for BuilderFieldTypeWithCrateRoot<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let crate_root = self.crate_root;
        match self.field_type {
            BuilderFieldType::Optional(ty) => tokens.append_all(quote!(
                #crate_root::export::core::option::Option<#ty>
            )),
            BuilderFieldType::Precise(ty) => ty.to_tokens(tokens),
            BuilderFieldType::Phantom(ty) => tokens.append_all(quote!(
                #crate_root::export::core::marker::PhantomData<#ty>
            )),
        }
    }
}

/// Helper macro for unit tests. This is _only_ public in order to be accessible
/// from doc-tests too.
#[cfg(test)] // This contains a Box::leak, so is suitable only for tests
#[doc(hidden)]
#[macro_export]
macro_rules! default_builder_field {
    () => {{
        BuilderField {
            // Deliberately don't use the default value here - make sure
            // that all test cases are passing crate_root through properly.
            crate_root: &parse_quote!(::db),
            field_ident: &syn::Ident::new("foo", ::proc_macro2::Span::call_site()),
            field_type: BuilderFieldType::Optional(Box::leak(Box::new(parse_quote!(String)))),
            field_visibility: ::std::borrow::Cow::Owned(parse_quote!(pub)),
            attrs: &[parse_quote!(#[some_attr])],
        }
    }};
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn setter_enabled() {
        let field = default_builder_field!();

        assert_eq!(
            quote!(#field).to_string(),
            quote!(
                #[some_attr] pub foo: ::db::export::core::option::Option<String>,
            )
            .to_string()
        );
    }

    #[test]
    fn setter_disabled() {
        let mut field = default_builder_field!();
        field.field_visibility = Cow::Owned(syn::Visibility::Inherited);
        field.field_type = match field.field_type {
            BuilderFieldType::Optional(ty) => BuilderFieldType::Phantom(ty),
            _ => panic!(),
        };

        assert_eq!(
            quote!(#field).to_string(),
            quote!(
                #[some_attr]
                foo: ::db::export::core::marker::PhantomData<String>,
            )
            .to_string()
        );
    }

    #[test]
    fn private_field() {
        let private = Cow::Owned(syn::Visibility::Inherited);
        let mut field = default_builder_field!();
        field.field_visibility = private;

        assert_eq!(
            quote!(#field).to_string(),
            quote!(
                #[some_attr]
                foo: ::db::export::core::option::Option<String>,
            )
            .to_string()
        );
    }
}
