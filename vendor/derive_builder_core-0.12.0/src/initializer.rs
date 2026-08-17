use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, TokenStreamExt};
use syn;
use BuilderPattern;
use DEFAULT_STRUCT_NAME;

use crate::{change_span, BlockContents, DefaultExpression};

/// Initializer for the target struct fields, implementing `quote::ToTokens`.
///
/// Lives in the body of `BuildMethod`.
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
/// # use derive_builder_core::{DeprecationNotes, Initializer, BuilderPattern};
/// # fn main() {
/// #    let mut initializer = default_initializer!();
/// #    initializer.default_value = Some("42".parse().unwrap());
/// #    initializer.builder_pattern = BuilderPattern::Owned;
/// #
/// #    assert_eq!(quote!(#initializer).to_string(), quote!(
/// foo: match self.foo {
///     Some(value) => value,
///     None => { 42 },
/// },
/// #    ).to_string());
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct Initializer<'a> {
    /// Path to the root of the derive_builder crate.
    pub crate_root: &'a syn::Path,
    /// Name of the target field.
    pub field_ident: &'a syn::Ident,
    /// Whether the builder implements a setter for this field.
    pub field_enabled: bool,
    /// How the build method takes and returns `self` (e.g. mutably).
    pub builder_pattern: BuilderPattern,
    /// Default value for the target field.
    ///
    /// This takes precedence over a default struct identifier.
    pub default_value: Option<&'a DefaultExpression>,
    /// Whether the build_method defines a default struct.
    pub use_default_struct: bool,
    /// Span where the macro was told to use a preexisting error type, instead of creating one,
    /// to represent failures of the `build` method.
    ///
    /// An initializer can force early-return if a field has no set value and no default is
    /// defined. In these cases, it will convert from `derive_builder::UninitializedFieldError`
    /// into the return type of its enclosing `build` method. That conversion is guaranteed to
    /// work for generated error types, but if the caller specified an error type to use instead
    /// they may have forgotten the conversion from `UninitializedFieldError` into their specified
    /// error type.
    pub custom_error_type_span: Option<Span>,
    /// Method to use to to convert the builder's field to the target field
    ///
    /// For sub-builder fields, this will be `build` (or similar)
    pub conversion: FieldConversion<'a>,
}

impl<'a> ToTokens for Initializer<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let struct_field = &self.field_ident;
        let builder_field = struct_field;

        // This structure prevents accidental failure to add the trailing `,` due to incautious `return`
        let append_rhs = |tokens: &mut TokenStream| {
            if !self.field_enabled {
                let default = self.default();
                tokens.append_all(quote!(
                    #default
                ));
            } else {
                match &self.conversion {
                    FieldConversion::Block(conv) => {
                        conv.to_tokens(tokens);
                    }
                    FieldConversion::Move => tokens.append_all(quote!( self.#builder_field )),
                    FieldConversion::OptionOrDefault => {
                        let match_some = self.match_some();
                        let match_none = self.match_none();
                        tokens.append_all(quote!(
                            match self.#builder_field {
                                #match_some,
                                #match_none,
                            }
                        ));
                    }
                }
            }
        };

        tokens.append_all(quote!(#struct_field:));
        append_rhs(tokens);
        tokens.append_all(quote!(,));
    }
}

impl<'a> Initializer<'a> {
    /// To be used inside of `#struct_field: match self.#builder_field { ... }`
    fn match_some(&'a self) -> MatchSome {
        match self.builder_pattern {
            BuilderPattern::Owned => MatchSome::Move,
            BuilderPattern::Mutable | BuilderPattern::Immutable => MatchSome::Clone {
                crate_root: self.crate_root,
            },
        }
    }

    /// To be used inside of `#struct_field: match self.#builder_field { ... }`
    fn match_none(&'a self) -> MatchNone<'a> {
        match self.default_value {
            Some(expr) => MatchNone::DefaultTo {
                expr,
                crate_root: self.crate_root,
            },
            None => {
                if self.use_default_struct {
                    MatchNone::UseDefaultStructField(self.field_ident)
                } else {
                    MatchNone::ReturnError {
                        crate_root: self.crate_root,
                        field_name: self.field_ident.to_string(),
                        span: self.custom_error_type_span,
                    }
                }
            }
        }
    }

    fn default(&'a self) -> TokenStream {
        let crate_root = self.crate_root;
        match self.default_value {
            Some(expr) => expr.with_crate_root(crate_root).into_token_stream(),
            None if self.use_default_struct => {
                let struct_ident = syn::Ident::new(DEFAULT_STRUCT_NAME, Span::call_site());
                let field_ident = self.field_ident;
                quote!(#struct_ident.#field_ident)
            }
            None => {
                quote!(#crate_root::export::core::default::Default::default())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum FieldConversion<'a> {
    /// Usual conversion: unwrap the Option from the builder, or (hope to) use a default value
    OptionOrDefault,
    /// Custom conversion is a block contents expression
    Block(&'a BlockContents),
    /// Custom conversion is just to move the field from the builder
    Move,
}

/// To be used inside of `#struct_field: match self.#builder_field { ... }`
enum MatchNone<'a> {
    /// Inner value must be a valid Rust expression
    DefaultTo {
        expr: &'a DefaultExpression,
        crate_root: &'a syn::Path,
    },
    /// Inner value must be the field identifier
    ///
    /// The default struct must be in scope in the build_method.
    UseDefaultStructField(&'a syn::Ident),
    /// Inner value must be the field name
    ReturnError {
        crate_root: &'a syn::Path,
        field_name: String,
        span: Option<Span>,
    },
}

impl<'a> ToTokens for MatchNone<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match *self {
            MatchNone::DefaultTo { expr, crate_root } => {
                let expr = expr.with_crate_root(crate_root);
                tokens.append_all(quote!(None => #expr));
            }
            MatchNone::UseDefaultStructField(field_ident) => {
                let struct_ident = syn::Ident::new(DEFAULT_STRUCT_NAME, Span::call_site());
                tokens.append_all(quote!(
                    None => #struct_ident.#field_ident
                ))
            }
            MatchNone::ReturnError {
                ref field_name,
                ref span,
                crate_root,
            } => {
                let conv_span = span.unwrap_or_else(Span::call_site);
                // If the conversion fails, the compiler error should point to the error declaration
                // rather than the crate root declaration, but the compiler will see the span of #crate_root
                // and produce an undesired behavior (possibly because that's the first span in the bad expression?).
                // Creating a copy with deeply-rewritten spans preserves the desired error behavior.
                let crate_root = change_span(crate_root.into_token_stream(), conv_span);
                let err_conv = quote_spanned!(conv_span => #crate_root::export::core::convert::Into::into(
                    #crate_root::UninitializedFieldError::from(#field_name)
                ));
                tokens.append_all(quote!(
                    None => return #crate_root::export::core::result::Result::Err(#err_conv)
                ));
            }
        }
    }
}

/// To be used inside of `#struct_field: match self.#builder_field { ... }`
enum MatchSome<'a> {
    Move,
    Clone { crate_root: &'a syn::Path },
}

impl ToTokens for MatchSome<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match *self {
            Self::Move => tokens.append_all(quote!(
                Some(value) => value
            )),
            Self::Clone { crate_root } => tokens.append_all(quote!(
                Some(ref value) => #crate_root::export::core::clone::Clone::clone(value)
            )),
        }
    }
}

/// Helper macro for unit tests. This is _only_ public in order to be accessible
/// from doc-tests too.
#[doc(hidden)]
#[macro_export]
macro_rules! default_initializer {
    () => {
        Initializer {
            // Deliberately don't use the default value here - make sure
            // that all test cases are passing crate_root through properly.
            crate_root: &parse_quote!(::db),
            field_ident: &syn::Ident::new("foo", ::proc_macro2::Span::call_site()),
            field_enabled: true,
            builder_pattern: BuilderPattern::Mutable,
            default_value: None,
            use_default_struct: false,
            conversion: FieldConversion::OptionOrDefault,
            custom_error_type_span: None,
        }
    };
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn immutable() {
        let mut initializer = default_initializer!();
        initializer.builder_pattern = BuilderPattern::Immutable;

        assert_eq!(
            quote!(#initializer).to_string(),
            quote!(
                foo: match self.foo {
                    Some(ref value) => ::db::export::core::clone::Clone::clone(value),
                    None => return ::db::export::core::result::Result::Err(::db::export::core::convert::Into::into(
                        ::db::UninitializedFieldError::from("foo")
                    )),
                },
            )
            .to_string()
        );
    }

    #[test]
    fn mutable() {
        let mut initializer = default_initializer!();
        initializer.builder_pattern = BuilderPattern::Mutable;

        assert_eq!(
            quote!(#initializer).to_string(),
            quote!(
                foo: match self.foo {
                    Some(ref value) => ::db::export::core::clone::Clone::clone(value),
                    None => return ::db::export::core::result::Result::Err(::db::export::core::convert::Into::into(
                        ::db::UninitializedFieldError::from("foo")
                    )),
                },
            )
            .to_string()
        );
    }

    #[test]
    fn owned() {
        let mut initializer = default_initializer!();
        initializer.builder_pattern = BuilderPattern::Owned;

        assert_eq!(
            quote!(#initializer).to_string(),
            quote!(
                foo: match self.foo {
                    Some(value) => value,
                    None => return ::db::export::core::result::Result::Err(::db::export::core::convert::Into::into(
                        ::db::UninitializedFieldError::from("foo")
                    )),
                },
            )
            .to_string()
        );
    }

    #[test]
    fn default_value() {
        let mut initializer = default_initializer!();
        let default_value = DefaultExpression::explicit::<syn::Expr>(parse_quote!(42));
        initializer.default_value = Some(&default_value);

        assert_eq!(
            quote!(#initializer).to_string(),
            quote!(
                foo: match self.foo {
                    Some(ref value) => ::db::export::core::clone::Clone::clone(value),
                    None => { 42 },
                },
            )
            .to_string()
        );
    }

    #[test]
    fn default_struct() {
        let mut initializer = default_initializer!();
        initializer.use_default_struct = true;

        assert_eq!(
            quote!(#initializer).to_string(),
            quote!(
                foo: match self.foo {
                    Some(ref value) => ::db::export::core::clone::Clone::clone(value),
                    None => __default.foo,
                },
            )
            .to_string()
        );
    }

    #[test]
    fn setter_disabled() {
        let mut initializer = default_initializer!();
        initializer.field_enabled = false;

        assert_eq!(
            quote!(#initializer).to_string(),
            quote!(foo: ::db::export::core::default::Default::default(),).to_string()
        );
    }

    #[test]
    fn no_std() {
        let initializer = default_initializer!();

        assert_eq!(
            quote!(#initializer).to_string(),
            quote!(
                foo: match self.foo {
                    Some(ref value) => ::db::export::core::clone::Clone::clone(value),
                    None => return ::db::export::core::result::Result::Err(::db::export::core::convert::Into::into(
                        ::db::UninitializedFieldError::from("foo")
                    )),
                },
            )
            .to_string()
        );
    }
}
