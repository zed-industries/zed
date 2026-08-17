#![allow(clippy::useless_let_if_seq)]
use std::borrow::Cow;

use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, TokenStreamExt};
use syn;

use BuilderFieldType;
use BuilderPattern;
use DeprecationNotes;
use Each;

/// Setter for the struct fields in the build method, implementing
/// `quote::ToTokens`.
///
/// # Examples
///
/// Will expand to something like the following (depending on settings):
///
/// ```rust,ignore
/// # extern crate proc_macro2;
/// # #[macro_use]
/// # extern crate quote;
/// # extern crate syn;
/// # #[macro_use]
/// # extern crate derive_builder_core;
/// # use derive_builder_core::{Setter, BuilderPattern};
/// # fn main() {
/// #     let mut setter = default_setter!();
/// #     setter.pattern = BuilderPattern::Mutable;
/// #
/// #     assert_eq!(quote!(#setter).to_string(), quote!(
/// # #[allow(unused_mut)]
/// pub fn foo(&mut self, value: Foo) -> &mut Self {
///     let mut new = self;
///     new.foo = ::derive_builder::export::core::option::Option::Some(value);
///     new
/// }
/// #     ).to_string());
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct Setter<'a> {
    /// Path to the root of the derive_builder crate.
    pub crate_root: &'a syn::Path,
    /// Enables code generation for this setter fn.
    pub setter_enabled: bool,
    /// Enables code generation for the `try_` variant of this setter fn.
    pub try_setter: bool,
    /// Visibility of the setter, e.g. `syn::Visibility::Public`.
    pub visibility: Cow<'a, syn::Visibility>,
    /// How the setter method takes and returns `self` (e.g. mutably).
    pub pattern: BuilderPattern,
    /// Attributes which will be attached to this setter fn.
    pub attrs: &'a [syn::Attribute],
    /// Name of this setter fn.
    pub ident: syn::Ident,
    /// Name of the target field.
    pub field_ident: &'a syn::Ident,
    /// Type of the builder field.
    ///
    /// The corresonding builder field will be `Option<field_type>`.
    pub field_type: BuilderFieldType<'a>,
    /// Make the setter generic over `Into<T>`, where `T` is the field type.
    pub generic_into: bool,
    /// Make the setter remove the Option wrapper from the setter, remove the need to call Some(...).
    /// when combined with into, the into is used on the content Type of the Option.
    pub strip_option: bool,
    /// Emit deprecation notes to the user.
    pub deprecation_notes: &'a DeprecationNotes,
    /// Emit extend method.
    pub each: Option<&'a Each>,
}

impl<'a> ToTokens for Setter<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if self.setter_enabled {
            let crate_root = self.crate_root;
            let pattern = self.pattern;
            let vis = &self.visibility;
            let field_ident = self.field_ident;
            let ident = &self.ident;
            let attrs = self.attrs;
            let deprecation_notes = self.deprecation_notes;

            let self_param: TokenStream;
            let return_ty: TokenStream;
            let self_into_return_ty: TokenStream;

            match pattern {
                BuilderPattern::Owned => {
                    self_param = quote!(self);
                    return_ty = quote!(Self);
                    self_into_return_ty = quote!(self);
                }
                BuilderPattern::Mutable => {
                    self_param = quote!(&mut self);
                    return_ty = quote!(&mut Self);
                    self_into_return_ty = quote!(self);
                }
                BuilderPattern::Immutable => {
                    self_param = quote!(&self);
                    return_ty = quote!(Self);
                    self_into_return_ty =
                        quote!(#crate_root::export::core::clone::Clone::clone(self));
                }
            };

            let ty_params: TokenStream;
            let param_ty: TokenStream;
            let mut into_value: TokenStream;

            let (field_type, builder_field_is_option) = self.field_type.setter_type_info();

            let (ty, stripped_option) = {
                if self.strip_option {
                    match extract_type_from_option(field_type) {
                        Some(ty) => (ty, true),
                        None => (field_type, false),
                    }
                } else {
                    (field_type, false)
                }
            };

            if self.generic_into {
                ty_params = quote!(<VALUE: #crate_root::export::core::convert::Into<#ty>>);
                param_ty = quote!(VALUE);
                into_value = quote!(value.into());
            } else {
                ty_params = quote!();
                param_ty = quote!(#ty);
                into_value = quote!(value);
            }
            // If both `stripped_option` and `builder_field_is_option`, the target field is `Option<field_type>`,
            // the builder field is `Option<Option<field_type>>`, and the setter takes `file_type`, so we must wrap it twice.
            if stripped_option {
                into_value = wrap_expression_in_some(crate_root, into_value);
            }
            if builder_field_is_option {
                into_value = wrap_expression_in_some(crate_root, into_value);
            }

            tokens.append_all(quote!(
                #(#attrs)*
                #[allow(unused_mut)]
                #vis fn #ident #ty_params (#self_param, value: #param_ty)
                    -> #return_ty
                {
                    #deprecation_notes
                    let mut new = #self_into_return_ty;
                    new.#field_ident = #into_value;
                    new
                }
            ));

            if self.try_setter {
                let try_ty_params =
                    quote!(<VALUE: #crate_root::export::core::convert::TryInto<#ty>>);
                let try_ident = syn::Ident::new(&format!("try_{}", ident), Span::call_site());

                let mut converted = quote! {converted};
                if builder_field_is_option {
                    converted = wrap_expression_in_some(crate_root, converted);
                }

                tokens.append_all(quote!(
                    #(#attrs)*
                    #vis fn #try_ident #try_ty_params (#self_param, value: VALUE)
                        -> #crate_root::export::core::result::Result<#return_ty, VALUE::Error>
                    {
                        let converted : #ty = value.try_into()?;
                        let mut new = #self_into_return_ty;
                        new.#field_ident = #converted;
                        Ok(new)
                    }
                ));
            }

            if let Some(each) = self.each {
                let ident_each = &each.name;

                // Access the collection to extend, initialising with default value if necessary.
                let get_initialized_collection = if stripped_option {
                    // Outer (builder) Option -> Inner (field) Option -> collection.
                    quote!(get_or_insert_with(|| Some(
                        #crate_root::export::core::default::Default::default()
                    ))
                    .get_or_insert_with(#crate_root::export::core::default::Default::default))
                } else {
                    // Outer (builder) Option -> collection.
                    quote!(get_or_insert_with(
                        #crate_root::export::core::default::Default::default
                    ))
                };

                let ty_params: TokenStream;
                let param_ty: TokenStream;
                let into_item: TokenStream;

                if each.into {
                    ty_params = quote!(<VALUE, FROM_VALUE: #crate_root::export::core::convert::Into<VALUE>>);
                    param_ty = quote!(FROM_VALUE);
                    into_item = quote!(#crate_root::export::core::convert::Into::into(item));
                } else {
                    ty_params = quote!(<VALUE>);
                    param_ty = quote!(VALUE);
                    into_item = quote!(item);
                }

                tokens.append_all(quote!(
                    #(#attrs)*
                    #[allow(unused_mut)]
                    #vis fn #ident_each #ty_params(#self_param, item: #param_ty) -> #return_ty
                    where
                        #ty: #crate_root::export::core::default::Default + #crate_root::export::core::iter::Extend<VALUE>,
                    {
                        #deprecation_notes
                        let mut new = #self_into_return_ty;
                        new.#field_ident
                            .#get_initialized_collection
                            .extend(#crate_root::export::core::option::Option::Some(#into_item));
                        new
                    }
                ));
            }
        }
    }
}

/// Returns expression wrapping `bare_value` in `Some`
fn wrap_expression_in_some(crate_root: &syn::Path, bare_value: impl ToTokens) -> TokenStream {
    quote!( #crate_root::export::core::option::Option::Some(#bare_value) )
}

// adapted from https://stackoverflow.com/a/55277337/469066
// Note that since syn is a parser, it works with tokens.
// We cannot know for sure that this is an Option.
// The user could, for example, `type MaybeString = std::option::Option<String>`
// We cannot handle those arbitrary names.
fn extract_type_from_option(ty: &syn::Type) -> Option<&syn::Type> {
    use syn::punctuated::Pair;
    use syn::token::Colon2;
    use syn::{GenericArgument, Path, PathArguments, PathSegment};

    fn extract_type_path(ty: &syn::Type) -> Option<&Path> {
        match *ty {
            syn::Type::Path(ref typepath) if typepath.qself.is_none() => Some(&typepath.path),
            _ => None,
        }
    }

    // TODO store (with lazy static) precomputed parsing of Option when support of rust 1.18 will be removed (incompatible with lazy_static)
    // TODO maybe optimization, reverse the order of segments
    fn extract_option_segment(path: &Path) -> Option<Pair<&PathSegment, &Colon2>> {
        let idents_of_path = path.segments.iter().fold(String::new(), |mut acc, v| {
            acc.push_str(&v.ident.to_string());
            acc.push('|');
            acc
        });
        vec!["Option|", "std|option|Option|", "core|option|Option|"]
            .into_iter()
            .find(|s| idents_of_path == *s)
            .and_then(|_| path.segments.last().map(Pair::End))
    }

    extract_type_path(ty)
        .and_then(extract_option_segment)
        .and_then(|pair_path_segment| {
            let type_params = &pair_path_segment.into_value().arguments;
            // It should have only on angle-bracketed param ("<String>"):
            match *type_params {
                PathArguments::AngleBracketed(ref params) => params.args.first(),
                _ => None,
            }
        })
        .and_then(|generic_arg| match *generic_arg {
            GenericArgument::Type(ref ty) => Some(ty),
            _ => None,
        })
}

/// Helper macro for unit tests. This is _only_ public in order to be accessible
/// from doc-tests too.
#[doc(hidden)]
#[macro_export]
macro_rules! default_setter {
    () => {
        Setter {
            // Deliberately don't use the default value here - make sure
            // that all test cases are passing crate_root through properly.
            crate_root: &parse_quote!(::db),
            setter_enabled: true,
            try_setter: false,
            visibility: ::std::borrow::Cow::Owned(parse_quote!(pub)),
            pattern: BuilderPattern::Mutable,
            attrs: &vec![],
            ident: syn::Ident::new("foo", ::proc_macro2::Span::call_site()),
            field_ident: &syn::Ident::new("foo", ::proc_macro2::Span::call_site()),
            field_type: BuilderFieldType::Optional(Box::leak(Box::new(parse_quote!(Foo)))),
            generic_into: false,
            strip_option: false,
            deprecation_notes: &Default::default(),
            each: None,
        }
    };
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn immutable() {
        let mut setter = default_setter!();
        setter.pattern = BuilderPattern::Immutable;

        assert_eq!(
            quote!(#setter).to_string(),
            quote!(
                #[allow(unused_mut)]
                pub fn foo(&self, value: Foo) -> Self {
                    let mut new = ::db::export::core::clone::Clone::clone(self);
                    new.foo = ::db::export::core::option::Option::Some(value);
                    new
                }
            )
            .to_string()
        );
    }

    #[test]
    fn mutable() {
        let mut setter = default_setter!();
        setter.pattern = BuilderPattern::Mutable;

        assert_eq!(
            quote!(#setter).to_string(),
            quote!(
                #[allow(unused_mut)]
                pub fn foo(&mut self, value: Foo) -> &mut Self {
                    let mut new = self;
                    new.foo = ::db::export::core::option::Option::Some(value);
                    new
                }
            )
            .to_string()
        );
    }

    #[test]
    fn owned() {
        let mut setter = default_setter!();
        setter.pattern = BuilderPattern::Owned;

        assert_eq!(
            quote!(#setter).to_string(),
            quote!(
                #[allow(unused_mut)]
                pub fn foo(self, value: Foo) -> Self {
                    let mut new = self;
                    new.foo = ::db::export::core::option::Option::Some(value);
                    new
                }
            )
            .to_string()
        );
    }

    #[test]
    fn private() {
        let vis = Cow::Owned(syn::Visibility::Inherited);

        let mut setter = default_setter!();
        setter.visibility = vis;

        assert_eq!(
            quote!(#setter).to_string(),
            quote!(
                #[allow(unused_mut)]
                fn foo(&mut self, value: Foo) -> &mut Self {
                    let mut new = self;
                    new.foo = ::db::export::core::option::Option::Some(value);
                    new
                }
            )
            .to_string()
        );
    }

    #[test]
    fn generic() {
        let mut setter = default_setter!();
        setter.generic_into = true;

        #[rustfmt::skip]
        assert_eq!(
            quote!(#setter).to_string(),
            quote!(
                #[allow(unused_mut)]
                pub fn foo<VALUE: ::db::export::core::convert::Into<Foo>>(
                    &mut self,
                    value: VALUE
                ) -> &mut Self {
                    let mut new = self;
                    new.foo = ::db::export::core::option::Option::Some(value.into());
                    new
                }
            )
            .to_string()
        );
    }

    #[test]
    fn strip_option() {
        let ty = parse_quote!(Option<Foo>);
        let mut setter = default_setter!();
        setter.strip_option = true;
        setter.field_type = BuilderFieldType::Optional(&ty);

        #[rustfmt::skip]
        assert_eq!(
            quote!(#setter).to_string(),
            quote!(
                #[allow(unused_mut)]
                pub fn foo(&mut self, value: Foo) -> &mut Self {
                    let mut new = self;
                    new.foo = ::db::export::core::option::Option::Some(
                        ::db::export::core::option::Option::Some(value)
                    );
                    new
                }
            )
            .to_string()
        );
    }

    #[test]
    fn strip_option_into() {
        let ty = parse_quote!(Option<Foo>);
        let mut setter = default_setter!();
        setter.strip_option = true;
        setter.generic_into = true;
        setter.field_type = BuilderFieldType::Optional(&ty);

        #[rustfmt::skip]
        assert_eq!(
            quote!(#setter).to_string(),
            quote!(
                #[allow(unused_mut)]
                pub fn foo<VALUE: ::db::export::core::convert::Into<Foo>>(
                    &mut self,
                    value: VALUE
                ) -> &mut Self {
                    let mut new = self;
                    new.foo = ::db::export::core::option::Option::Some(
                        ::db::export::core::option::Option::Some(value.into())
                    );
                    new
                }
            )
            .to_string()
        );
    }

    // including try_setter
    #[test]
    fn full() {
        //named!(outer_attrs -> Vec<syn::Attribute>, many0!(syn::Attribute::parse_outer));
        //let attrs = outer_attrs.parse_str("#[some_attr]").unwrap();
        let attrs: Vec<syn::Attribute> = vec![parse_quote!(#[some_attr])];

        let mut deprecated = DeprecationNotes::default();
        deprecated.push("Some example.".to_string());

        let mut setter = default_setter!();
        setter.attrs = attrs.as_slice();
        setter.generic_into = true;
        setter.deprecation_notes = &deprecated;
        setter.try_setter = true;

        assert_eq!(
            quote!(#setter).to_string(),
            quote!(
            #[some_attr]
            #[allow(unused_mut)]
            pub fn foo <VALUE: ::db::export::core::convert::Into<Foo>>(&mut self, value: VALUE) -> &mut Self {
                #deprecated
                let mut new = self;
                new.foo = ::db::export::core::option::Option::Some(value.into());
                new
            }

            #[some_attr]
            pub fn try_foo<VALUE: ::db::export::core::convert::TryInto<Foo>>(&mut self, value: VALUE)
                -> ::db::export::core::result::Result<&mut Self, VALUE::Error> {
                let converted : Foo = value.try_into()?;
                let mut new = self;
                new.foo = ::db::export::core::option::Option::Some(converted);
                Ok(new)
            }
        ).to_string()
        );
    }

    #[test]
    fn no_std() {
        let mut setter = default_setter!();
        setter.pattern = BuilderPattern::Immutable;

        assert_eq!(
            quote!(#setter).to_string(),
            quote!(
                #[allow(unused_mut)]
                pub fn foo(&self, value: Foo) -> Self {
                    let mut new = ::db::export::core::clone::Clone::clone(self);
                    new.foo = ::db::export::core::option::Option::Some(value);
                    new
                }
            )
            .to_string()
        );
    }

    #[test]
    fn no_std_generic() {
        let mut setter = default_setter!();
        setter.generic_into = true;

        #[rustfmt::skip]
        assert_eq!(
            quote!(#setter).to_string(),
            quote!(
                #[allow(unused_mut)]
                pub fn foo<VALUE: ::db::export::core::convert::Into<Foo>>(
                    &mut self,
                    value: VALUE
                ) -> &mut Self {
                    let mut new = self;
                    new.foo = ::db::export::core::option::Option::Some(value.into());
                    new
                }
            )
            .to_string()
        );
    }

    #[test]
    fn setter_disabled() {
        let mut setter = default_setter!();
        setter.setter_enabled = false;

        assert_eq!(quote!(#setter).to_string(), quote!().to_string());
    }

    #[test]
    fn try_setter() {
        let mut setter: Setter = default_setter!();
        setter.pattern = BuilderPattern::Mutable;
        setter.try_setter = true;

        #[rustfmt::skip]
        assert_eq!(
            quote!(#setter).to_string(),
            quote!(
                #[allow(unused_mut)]
                pub fn foo(&mut self, value: Foo) -> &mut Self {
                    let mut new = self;
                    new.foo = ::db::export::core::option::Option::Some(value);
                    new
                }

                pub fn try_foo<VALUE: ::db::export::core::convert::TryInto<Foo>>(
                    &mut self,
                    value: VALUE
                ) -> ::db::export::core::result::Result<&mut Self, VALUE::Error> {
                    let converted: Foo = value.try_into()?;
                    let mut new = self;
                    new.foo = ::db::export::core::option::Option::Some(converted);
                    Ok(new)
                }
            )
            .to_string()
        );
    }

    #[test]
    fn extract_type_from_option_on_simple_type() {
        let ty_foo = parse_quote!(Foo);
        assert_eq!(extract_type_from_option(&ty_foo), None);

        for s in vec![
            parse_quote!(Option<Foo>),
            parse_quote!(std::option::Option<Foo>),
            parse_quote!(::std::option::Option<Foo>),
            parse_quote!(core::option::Option<Foo>),
            parse_quote!(::core::option::Option<Foo>),
        ] {
            assert_eq!(extract_type_from_option(&s), Some(&ty_foo));
        }
    }
}
