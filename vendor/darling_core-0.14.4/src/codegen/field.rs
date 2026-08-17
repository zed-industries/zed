use std::borrow::Cow;

use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens, TokenStreamExt};
use syn::{spanned::Spanned, Ident, Path, Type};

use crate::codegen::{DefaultExpression, PostfixTransform};
use crate::usage::{self, IdentRefSet, IdentSet, UsesTypeParams};

/// Properties needed to generate code for a field in all the contexts
/// where one may appear.
#[derive(Debug, Clone)]
pub struct Field<'a> {
    /// The name presented to the user of the library. This will appear
    /// in error messages and will be looked when parsing names.
    pub name_in_attr: Cow<'a, String>,

    /// The name presented to the author of the library. This will appear
    /// in the setters or temporary variables which contain the values.
    pub ident: &'a Ident,

    /// The type of the field in the input.
    pub ty: &'a Type,
    pub default_expression: Option<DefaultExpression<'a>>,
    pub with_path: Cow<'a, Path>,
    pub post_transform: Option<&'a PostfixTransform>,
    pub skip: bool,
    pub multiple: bool,
}

impl<'a> Field<'a> {
    pub fn as_name(&'a self) -> &'a str {
        &self.name_in_attr
    }

    pub fn as_declaration(&'a self) -> Declaration<'a> {
        Declaration(self)
    }

    pub fn as_match(&'a self) -> MatchArm<'a> {
        MatchArm(self)
    }

    pub fn as_initializer(&'a self) -> Initializer<'a> {
        Initializer(self)
    }

    pub fn as_presence_check(&'a self) -> CheckMissing<'a> {
        CheckMissing(self)
    }
}

impl<'a> UsesTypeParams for Field<'a> {
    fn uses_type_params<'b>(
        &self,
        options: &usage::Options,
        type_set: &'b IdentSet,
    ) -> IdentRefSet<'b> {
        self.ty.uses_type_params(options, type_set)
    }
}

/// An individual field during variable declaration in the generated parsing method.
pub struct Declaration<'a>(&'a Field<'a>);

impl<'a> ToTokens for Declaration<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let field: &Field = self.0;
        let ident = field.ident;
        let ty = field.ty;

        tokens.append_all(if field.multiple {
            // This is NOT mutable, as it will be declared mutable only temporarily.
            quote!(let mut #ident: #ty = ::darling::export::Default::default();)
        } else {
            quote!(let mut #ident: (bool, ::darling::export::Option<#ty>) = (false, None);)
        });
    }
}

/// Represents an individual field in the match.
pub struct MatchArm<'a>(&'a Field<'a>);

impl<'a> ToTokens for MatchArm<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let field: &Field = self.0;
        if !field.skip {
            let name_str = &field.name_in_attr;
            let ident = field.ident;
            let with_path = &field.with_path;
            let post_transform = field.post_transform.as_ref();

            // Errors include the location of the bad input, so we compute that here.
            // Fields that take multiple values add the index of the error for convenience,
            // while single-value fields only expose the name in the input attribute.
            let location = if field.multiple {
                // we use the local variable `len` here because location is accessed via
                // a closure, and the borrow checker gets very unhappy if we try to immutably
                // borrow `#ident` in that closure when it was declared `mut` outside.
                quote!(&format!("{}[{}]", #name_str, __len))
            } else {
                quote!(#name_str)
            };

            // Give darling's generated code the span of the `with_path` so that if the target
            // type doesn't impl FromMeta, darling's immediate user gets a properly-spanned error.
            //
            // Within the generated code, add the span immediately on extraction failure, so that it's
            // as specific as possible.
            // The behavior of `with_span` makes this safe to do; if the child applied an
            // even-more-specific span, our attempt here will not overwrite that and will only cost
            // us one `if` check.
            let extractor = quote_spanned!(with_path.span()=>#with_path(__inner)#post_transform.map_err(|e| e.with_span(&__inner).at(#location)));

            tokens.append_all(if field.multiple {
                quote!(
                    #name_str => {
                        // Store the index of the name we're assessing in case we need
                        // it for error reporting.
                        let __len = #ident.len();
                        if let ::darling::export::Some(__val) = __errors.handle(#extractor) {
                            #ident.push(__val)
                        }
                    }
                )
            } else {
                quote!(
                    #name_str => {
                        if !#ident.0 {
                            #ident = (true, __errors.handle(#extractor));
                        } else {
                            __errors.push(::darling::Error::duplicate_field(#name_str).with_span(&__inner));
                        }
                    }
                )
            });
        }
    }
}

/// Wrapper to generate initialization code for a field.
pub struct Initializer<'a>(&'a Field<'a>);

impl<'a> ToTokens for Initializer<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let field: &Field = self.0;
        let ident = field.ident;
        tokens.append_all(if field.multiple {
            if let Some(ref expr) = field.default_expression {
                quote_spanned!(expr.span()=> #ident: if !#ident.is_empty() {
                    #ident
                } else {
                    #expr
                })
            } else {
                quote!(#ident: #ident)
            }
        } else if let Some(ref expr) = field.default_expression {
            quote_spanned!(expr.span()=> #ident: if let Some(__val) = #ident.1 {
                __val
            } else {
                #expr
            })
        } else {
            quote!(#ident: #ident.1.expect("Uninitialized fields without defaults were already checked"))
        });
    }
}

/// Creates an error if a field has no value and no default.
pub struct CheckMissing<'a>(&'a Field<'a>);

impl<'a> ToTokens for CheckMissing<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if !self.0.multiple && self.0.default_expression.is_none() {
            let ident = self.0.ident;
            let ty = self.0.ty;
            let name_in_attr = &self.0.name_in_attr;

            // If `ty` does not impl FromMeta, the compiler error should point
            // at the offending type rather than at the derive-macro call site.
            let from_none_call =
                quote_spanned!(ty.span()=> <#ty as ::darling::FromMeta>::from_none());

            tokens.append_all(quote! {
                if !#ident.0 {
                    match #from_none_call {
                        ::darling::export::Some(__type_fallback) => {
                            #ident.1 = ::darling::export::Some(__type_fallback);
                        }
                        ::darling::export::None => {
                            __errors.push(::darling::Error::missing_field(#name_in_attr))
                        }
                    }
                }
            })
        }
    }
}
