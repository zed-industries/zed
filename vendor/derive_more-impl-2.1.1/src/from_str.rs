//! Implementation of a [`FromStr`] derive macro.

#[cfg(doc)]
use std::str::FromStr;
use std::{collections::HashMap, iter};

use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse::Parse, parse_quote, spanned::Spanned as _};

use crate::utils::{
    attr::{self, ParseMultiple as _},
    Either, GenericsSearch, Spanning,
};

/// Expands a [`FromStr`] derive macro.
pub fn expand(input: &syn::DeriveInput, _: &'static str) -> syn::Result<TokenStream> {
    match &input.data {
        syn::Data::Struct(data) => Ok(if data.fields.is_empty() {
            FlatExpansion::try_from(input)?.into_token_stream()
        } else {
            ForwardExpansion::try_from(input)?.into_token_stream()
        }),
        syn::Data::Enum(_) => Ok(FlatExpansion::try_from(input)?.into_token_stream()),
        syn::Data::Union(data) => Err(syn::Error::new(
            data.union_token.span(),
            "`FromStr` cannot be derived for unions",
        )),
    }
}

/// Expansion of a macro for generating a forwarding [`FromStr`] implementation of a struct.
struct ForwardExpansion<'i> {
    /// [`syn::Ident`] and [`syn::Generics`] of the struct.
    ///
    /// [`syn::Ident`]: struct@syn::Ident
    self_ty: (&'i syn::Ident, &'i syn::Generics),

    /// [`syn::Field`] representing the wrapped type to forward implementation on.
    inner: &'i syn::Field,

    /// Optional [`attr::Error`] enabling conversion into a custom error type.
    custom_error: Option<attr::Error>,
}

impl<'i> TryFrom<&'i syn::DeriveInput> for ForwardExpansion<'i> {
    type Error = syn::Error;

    fn try_from(input: &'i syn::DeriveInput) -> syn::Result<Self> {
        let syn::Data::Struct(data) = &input.data else {
            return Err(syn::Error::new(
                input.span(),
                "expected a struct for forward `FromStr` derive",
            ));
        };

        // TODO: Unite these two conditions via `&&` once MSRV is bumped to 1.88 or above.
        if data.fields.len() != 1 {
            return Err(syn::Error::new(
                data.fields.span(),
                "only structs with single field can derive `FromStr`",
            ));
        }
        let Some(inner) = data.fields.iter().next() else {
            return Err(syn::Error::new(
                data.fields.span(),
                "only structs with single field can derive `FromStr`",
            ));
        };

        let custom_error =
            attr::Error::parse_attrs(&input.attrs, &format_ident!("from_str"))?
                .map(Spanning::into_inner);

        Ok(Self {
            self_ty: (&input.ident, &input.generics),
            inner,
            custom_error,
        })
    }
}

impl ToTokens for ForwardExpansion<'_> {
    /// Expands a forwarding [`FromStr`] implementations for a struct.
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let inner_ty = &self.inner.ty;
        let ty = self.self_ty.0;

        let generics_search = GenericsSearch::from(self.self_ty.1);
        let mut generics = self.self_ty.1.clone();
        if generics_search.any_in(inner_ty) {
            generics.make_where_clause().predicates.push(parse_quote! {
                #inner_ty: derive_more::core::str::FromStr
            });
        }
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        let constructor = self.inner.self_constructor([parse_quote! { v }]);

        let mut error_ty =
            quote! { <#inner_ty as derive_more::core::str::FromStr>::Err };
        let mut error_conv = quote! {};
        if let Some(custom_error) = &self.custom_error {
            error_ty = custom_error.ty.to_token_stream();
            error_conv = custom_error.conv.as_ref().map_or_else(
                || quote! { .map_err(derive_more::core::convert::Into::into) },
                |conv| quote! { .map_err(#conv) },
            );
        }

        quote! {
            #[automatically_derived]
            impl #impl_generics derive_more::core::str::FromStr for #ty #ty_generics #where_clause {
                type Err = #error_ty;

                #[inline]
                fn from_str(s: &str) -> derive_more::core::result::Result<Self, Self::Err> {
                    derive_more::core::str::FromStr::from_str(s).map(|v| #constructor)#error_conv
                }
            }
        }.to_tokens(tokens);
    }
}

/// Expansion of a macro for generating a flat [`FromStr`] implementation of an enum or a struct.
struct FlatExpansion<'i> {
    /// [`syn::Ident`] and [`syn::Generics`] of the enum/struct.
    ///
    /// [`syn::Ident`]: struct@syn::Ident
    self_ty: (&'i syn::Ident, &'i syn::Generics),

    /// [`syn::Ident`]s along with the matched values (enum variants or struct itself), and
    /// a value-specific [`attr::RenameAll`] overriding [`FlatExpansion::rename_all`], if any.
    ///
    /// [`syn::Ident`]: struct@syn::Ident
    matches: Vec<(
        &'i syn::Ident,
        Either<&'i syn::DataStruct, &'i syn::Variant>,
        Option<attr::RenameAll>,
    )>,

    /// [`FlatExpansion::matches`] grouped by its similar representation for detecting whether their
    /// case-insensitivity should be disabled.
    similar_matches: HashMap<String, Vec<&'i syn::Ident>>,

    /// Optional [`attr::RenameAll`] indicating the case conversion to be applied to all the matched
    /// values (enum variants or struct itself).
    rename_all: Option<attr::RenameAll>,

    /// Optional [`attr::Error`] enabling conversion into a custom error type.
    custom_error: Option<attr::Error>,
}

impl<'i> TryFrom<&'i syn::DeriveInput> for FlatExpansion<'i> {
    type Error = syn::Error;

    fn try_from(input: &'i syn::DeriveInput) -> syn::Result<Self> {
        let attr_ident = &format_ident!("from_str");

        let matches = match &input.data {
            syn::Data::Struct(data) => {
                if !data.fields.is_empty() {
                    return Err(syn::Error::new(
                        data.fields.span(),
                        "only structs with no fields can derive `FromStr`",
                    ));
                }
                vec![(&input.ident, Either::Left(data), None)]
            }
            syn::Data::Enum(data) => data
                .variants
                .iter()
                .map(|variant| {
                    if !variant.fields.is_empty() {
                        return Err(syn::Error::new(
                            variant.fields.span(),
                            "only enums with no fields can derive `FromStr`",
                        ));
                    }
                    let attr =
                        attr::RenameAll::parse_attrs(&variant.attrs, attr_ident)?
                            .map(Spanning::into_inner);
                    Ok((&variant.ident, Either::Right(variant), attr))
                })
                .collect::<syn::Result<_>>()?,
            syn::Data::Union(_) => {
                return Err(syn::Error::new(
                    input.span(),
                    "expected an enum or a struct for flat `FromStr` derive",
                ))
            }
        };

        let FlatContainerAttributes {
            rename_all,
            error: custom_error,
        } = FlatContainerAttributes::parse_attrs(&input.attrs, attr_ident)?
            .map(Spanning::into_inner)
            .unwrap_or_default();

        let mut similar_matches = <HashMap<_, Vec<_>>>::new();
        if rename_all.is_none() {
            for (ident, _, renaming) in &matches {
                let name = ident.to_string();
                let lowercased = name.to_lowercase();
                if let Some(rename) = renaming {
                    let renamed_lowercased = rename.convert_case(&name);
                    if renamed_lowercased != lowercased {
                        similar_matches
                            .entry(renamed_lowercased)
                            .or_default()
                            .push(*ident);
                    }
                }
                similar_matches.entry(lowercased).or_default().push(*ident);
            }
        }

        let mut exact_matches = <HashMap<String, Vec<String>>>::new();
        for (ident, _, renaming) in &matches {
            let name = ident.to_string();
            let exact = if let Some(default_renaming) = &rename_all {
                renaming
                    .as_ref()
                    .unwrap_or(default_renaming)
                    .convert_case(&name)
            } else if let Some(renaming) = renaming {
                renaming.convert_case(&name)
            } else {
                let lowercased = name.to_lowercase();
                if similar_matches[&lowercased].len() > 1 {
                    name.clone()
                } else {
                    lowercased
                }
            };
            exact_matches.entry(exact).or_default().push(name);
        }
        if let Some((string, variants)) =
            exact_matches.into_iter().find(|(_, vs)| vs.len() > 1)
        {
            return Err(syn::Error::new(
                input.ident.span(),
                format!(
                    "`{}` variants cannot have the same \"{string}\" string representation",
                    variants.join("`, `"),
                ),
            ));
        }

        Ok(Self {
            self_ty: (&input.ident, &input.generics),
            matches,
            similar_matches,
            rename_all,
            custom_error,
        })
    }
}

impl ToTokens for FlatExpansion<'_> {
    /// Expands a flat [`FromStr`] implementations for an enum.
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ty = self.self_ty.0;
        let (impl_generics, ty_generics, where_clause) =
            self.self_ty.1.split_for_impl();
        let ty_name = ty.to_string();

        let scrutinee_lowercased = self
            .rename_all
            .is_none()
            .then(|| quote! { .to_lowercase().as_str() });
        let match_arms = if let Some(default_renaming) = self.rename_all {
            self.matches
                .iter()
                .map(|(ident, value, renaming)| {
                    let converted = renaming
                        .unwrap_or(default_renaming)
                        .convert_case(&ident.to_string());
                    let constructor = value.self_constructor_empty();

                    quote! { #converted => #constructor, }
                })
                .collect::<Vec<_>>()
        } else {
            self.matches
                .iter()
                .map(|(ident, value, renaming)| {
                    let name = ident.to_string();
                    let constructor = value.self_constructor_empty();
                    if let Some(rename) = renaming {
                        let exact_name = rename.convert_case(&name);

                        quote! { _ if s == #exact_name => #constructor, }
                    } else {
                        let lowercased = name.to_lowercase();
                        let exact_guard = (self.similar_matches[&lowercased].len() > 1)
                            .then(|| quote! { if s == #name });

                        quote! { #lowercased #exact_guard => #constructor, }
                    }
                })
                .collect()
        };

        let default_error = quote! { derive_more::FromStrError::new(#ty_name) };
        let mut error_ty = quote! { derive_more::FromStrError };
        let mut error_val = default_error.clone();
        if let Some(custom_error) = &self.custom_error {
            error_ty = custom_error.ty.to_token_stream();
            error_val = custom_error.conv.as_ref().map_or_else(
                || quote! { derive_more::core::convert::Into::into(#default_error) },
                |conv| quote! { (#conv)(#default_error) },
            );
        }

        quote! {
            #[allow(unreachable_code)] // for empty enums
            #[automatically_derived]
            impl #impl_generics derive_more::core::str::FromStr for #ty #ty_generics #where_clause {
                type Err = #error_ty;

                fn from_str(s: &str) -> derive_more::core::result::Result<
                    Self, <Self as derive_more::core::str::FromStr>::Err,
                > {
                    derive_more::core::result::Result::Ok(match s #scrutinee_lowercased {
                        #( #match_arms )*
                        _ => return derive_more::core::result::Result::Err(#error_val),
                    })
                }
            }
        }.to_tokens(tokens);
    }
}

/// Extension of [`syn::Fields`] used by this expansion.
trait FieldsExt {
    /// Generates a `name`d constructor with the provided `values` assigned to these
    /// [`syn::Fields`].
    ///
    /// # Panics
    ///
    /// If number of provided `values` doesn't match number of these [`syn::Fields`].
    fn constructor(
        &self,
        name: &syn::Path,
        values: impl IntoIterator<Item = syn::Ident>,
    ) -> TokenStream;

    /// Generates a `Self` type constructor with the provided `values` assigned to these
    /// [`syn::Fields`].
    ///
    /// # Panics
    ///
    /// If number of provided `values` doesn't match number of these [`syn::Fields`].
    fn self_constructor(
        &self,
        values: impl IntoIterator<Item = syn::Ident>,
    ) -> TokenStream {
        self.constructor(&self.self_ty(), values)
    }

    /// Generates a `Self` type constructor with no fields.
    ///
    /// # Panics
    ///
    /// If these [`syn::Fields`] are not [empty].
    ///
    /// [empty]: syn::Fields::is_empty
    fn self_constructor_empty(&self) -> TokenStream {
        self.self_constructor(iter::empty())
    }

    /// Returns a [`syn::Path`] representing a `Self` type of these [`syn::Fields`].
    fn self_ty(&self) -> syn::Path {
        parse_quote! { Self }
    }
}

impl FieldsExt for syn::Fields {
    fn constructor(
        &self,
        name: &syn::Path,
        values: impl IntoIterator<Item = syn::Ident>,
    ) -> TokenStream {
        let values = values.into_iter();
        let fields = match self {
            Self::Named(fields) => {
                let initializers = fields.named.iter().zip(values).map(|(f, value)| {
                    let ident = &f.ident;
                    quote! { #ident: #value }
                });
                Some(quote! { { #( #initializers, )*} })
            }
            Self::Unnamed(_) => Some(quote! { ( #( #values, )* ) }),
            Self::Unit => None,
        };
        quote! { #name #fields }
    }
}

impl FieldsExt for syn::Field {
    fn constructor(
        &self,
        name: &syn::Path,
        values: impl IntoIterator<Item = syn::Ident>,
    ) -> TokenStream {
        let mut values = values.into_iter();
        let value = values.next().expect("expected a single value");
        if values.next().is_some() {
            panic!("expected a single value");
        }

        if let Some(ident) = &self.ident {
            quote! { #name { #ident: #value } }
        } else {
            quote! { #name(#value) }
        }
    }
}

impl FieldsExt for syn::Variant {
    fn constructor(
        &self,
        name: &syn::Path,
        values: impl IntoIterator<Item = syn::Ident>,
    ) -> TokenStream {
        self.fields.constructor(name, values)
    }

    fn self_ty(&self) -> syn::Path {
        let variant = &self.ident;

        parse_quote! { Self::#variant }
    }
}

impl FieldsExt for syn::DataStruct {
    fn constructor(
        &self,
        name: &syn::Path,
        values: impl IntoIterator<Item = syn::Ident>,
    ) -> TokenStream {
        self.fields.constructor(name, values)
    }
}

impl<L: FieldsExt, R: FieldsExt> FieldsExt for Either<&L, &R> {
    fn constructor(
        &self,
        name: &syn::Path,
        values: impl IntoIterator<Item = syn::Ident>,
    ) -> TokenStream {
        match self {
            Self::Left(l) => l.constructor(name, values),
            Self::Right(r) => r.constructor(name, values),
        }
    }

    fn self_ty(&self) -> syn::Path {
        match self {
            Self::Left(l) => l.self_ty(),
            Self::Right(r) => r.self_ty(),
        }
    }
}

/// Representation of possible [`FromStr`] derive macro attributes placed on an enum or a struct for
/// a [`FlatExpansion`].
///
/// ```rust,ignore
/// #[<attribute>(rename_all = "<casing>")]
/// #[<attribute>(error(<ty>))]
/// #[<attribute>(error(<ty>, <conv>))]
/// ```
///
/// Both `#[<attribute>(rename_all = "<casing>")]` and `#[<attribute>(error(<ty>[, <conv>]))]` can
/// be specified only once.
#[derive(Default)]
struct FlatContainerAttributes {
    /// [`attr::RenameAll`] for case conversion.
    rename_all: Option<attr::RenameAll>,

    /// [`attr::Error`] for conversion into a custom error type.
    error: Option<attr::Error>,
}

impl Parse for FlatContainerAttributes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        mod ident {
            use syn::custom_keyword;

            custom_keyword!(error);
            custom_keyword!(rename_all);
        }

        let ahead = input.lookahead1();
        if ahead.peek(ident::error) {
            Ok(Self {
                error: Some(input.parse()?),
                ..Default::default()
            })
        } else if ahead.peek(ident::rename_all) {
            Ok(Self {
                rename_all: Some(input.parse()?),
                ..Default::default()
            })
        } else {
            Err(ahead.error())
        }
    }
}

impl attr::ParseMultiple for FlatContainerAttributes {
    fn merge_attrs(
        prev: Spanning<Self>,
        new: Spanning<Self>,
        name: &syn::Ident,
    ) -> syn::Result<Spanning<Self>> {
        let Spanning {
            span: prev_span,
            item: mut prev,
        } = prev;
        let Spanning {
            span: new_span,
            item: new,
        } = new;

        if new
            .rename_all
            .and_then(|n| prev.rename_all.replace(n))
            .is_some()
        {
            return Err(syn::Error::new(
                new_span,
                format!("multiple `#[{name}(rename_all=\"...\")]` attributes aren't allowed"),
            ));
        }

        if prev.error.is_some() && new.error.is_some() {
            return Err(syn::Error::new(
                new_span,
                format!(
                    "multiple `#[{name}(error(\"...\")]` attributes aren't allowed",
                ),
            ));
        }

        prev.error = prev.error.or(new.error);

        Ok(Spanning::new(
            prev,
            prev_span.join(new_span).unwrap_or(prev_span),
        ))
    }
}
