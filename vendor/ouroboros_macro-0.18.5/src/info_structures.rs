use crate::utils::{make_generic_arguments, make_generic_consumers, replace_this_with_lifetime};
use proc_macro2::{Ident, TokenStream};
use proc_macro2_diagnostics::{Diagnostic, SpanDiagnosticExt};
use quote::{format_ident, quote, ToTokens};
use syn::{
    punctuated::Punctuated, token::Comma, Attribute, ConstParam, Error, GenericParam, Generics,
    LifetimeParam, Type, TypeParam, Visibility, spanned::Spanned, 
};

#[derive(Clone, Copy)]
pub struct Options {
    pub do_no_doc: bool,
    pub do_pub_extras: bool,
}

impl Options {
    // pub fn documentation_to_tokens(&self, documentation: &str) -> TokenStream {
    //     if self.do_no_doc {
    //         quote! { #[doc(hidden)] }
    //     } else {
    //         quote! { #[doc=#documentation] }
    //     }
    // }
}

#[derive(Clone, Copy, PartialEq)]
pub enum FieldType {
    /// Not borrowed by other parts of the struct.
    Tail,
    /// Immutably borrowed by at least one other field.
    Borrowed,
    /// Mutably borrowed by one other field.
    BorrowedMut,
}

impl FieldType {
    pub fn is_tail(self) -> bool {
        self == Self::Tail
    }
}

#[derive(Clone)]
pub struct BorrowRequest {
    pub index: usize,
    pub mutable: bool,
}

#[derive(Clone)]
pub enum Derive {
    Debug,
    PartialEq,
    Eq,
}

#[derive(Copy, Clone)]
pub enum BuilderType {
    Sync,
    Async,
    AsyncSend,
}

impl BuilderType {
    pub fn is_async(&self) -> bool {
        !matches!(self, BuilderType::Sync)
    }
}

#[derive(Clone)]
pub struct StructInfo {
    pub derives: Vec<Derive>,
    pub ident: Ident,
    pub internal_ident: Ident,
    pub generics: Generics,
    pub vis: Visibility,
    pub fields: Vec<StructFieldInfo>,
    pub first_lifetime: Ident,
    pub attributes: Vec<Attribute>,
}

impl StructInfo {
    // The lifetime to use in place of 'this for internal implementations,
    // should never be exposed to the user.
    pub fn fake_lifetime(&self) -> Ident {
        self.first_lifetime.clone()
    }

    pub fn generic_params(&self) -> &Punctuated<GenericParam, Comma> {
        &self.generics.params
    }

    /// Same as generic_params but with 'this and 'outer_borrow prepended.
    pub fn borrowed_generic_params(&self) -> TokenStream {
        if self.generic_params().is_empty() {
            quote! { <'outer_borrow, 'this> }
        } else {
            let mut new_generic_params = self.generic_params().clone();
            new_generic_params.insert(0, syn::parse_quote! { 'this });
            new_generic_params.insert(0, syn::parse_quote! { 'outer_borrow });
            quote! { <#new_generic_params> }
        }
    }

    /// Same as generic_params but without bounds and with '_ prepended twice.
    pub fn borrowed_generic_params_inferred(&self) -> TokenStream {
        use GenericParam::*;
        let params = self.generic_params().iter().map(|p| match p {
            Type(TypeParam { ident, .. }) | Const(ConstParam { ident, .. }) => {
                ident.to_token_stream()
            }
            Lifetime(LifetimeParam { lifetime, .. }) => lifetime.to_token_stream(),
        });
        quote! { <'_, '_, #(#params,)*> }
    }

    pub fn generic_arguments(&self) -> Vec<TokenStream> {
        make_generic_arguments(self.generics.params.iter().collect())
    }

    /// Same as generic_arguments but with 'outer_borrow and 'this prepended.
    pub fn borrowed_generic_arguments(&self) -> Vec<TokenStream> {
        let mut args = self.generic_arguments();
        args.insert(0, quote! { 'this });
        args.insert(0, quote! { 'outer_borrow });
        args
    }

    pub fn generic_consumers(&self) -> impl Iterator<Item = (TokenStream, Ident)> {
        make_generic_consumers(&self.generics)
    }
}

#[derive(Clone)]
pub struct StructFieldInfo {
    pub name: Ident,
    pub typ: Type,
    pub field_type: FieldType,
    pub vis: Visibility,
    pub borrows: Vec<BorrowRequest>,
    /// If this is true and borrows is empty, the struct will borrow from self in the future but
    /// does not require a builder to be initialized. It should not be able to be removed from the
    /// struct with into_heads.
    pub self_referencing: bool,
    /// If it is None, the user has not specified whether or not the field is covariant. If this is
    /// Some(false), we should avoid making borrow_* or borrow_*_mut functions as they will not
    /// be able to compile.
    pub covariant: Option<bool>,
}

#[derive(Clone)]
pub enum ArgType {
    /// Used when the initial value of a field can be passed directly into the constructor.
    Plain(TokenStream),
    /// Used when a field requires self references and thus requires something that implements
    /// a builder function trait instead of a simple plain type.
    TraitBound(TokenStream),
}

impl StructFieldInfo {
    pub fn builder_name(&self) -> Ident {
        format_ident!("{}_builder", self.name)
    }

    pub fn illegal_ref_name(&self) -> Ident {
        format_ident!("{}_illegal_static_reference", self.name)
    }

    pub fn is_borrowed(&self) -> bool {
        self.field_type != FieldType::Tail
    }

    pub fn is_mutably_borrowed(&self) -> bool {
        self.field_type == FieldType::BorrowedMut
    }

    pub fn boxed(&self) -> TokenStream {
        let name = &self.name;
        quote! { ::ouroboros::macro_help::aliasable_boxed(#name) }
    }

    pub fn stored_type(&self) -> TokenStream {
        let t = &self.typ;
        if self.is_borrowed() {
            quote! { ::ouroboros::macro_help::AliasableBox<#t> }
        } else {
            quote! { #t }
        }
    }

    /// Returns code which takes a variable with the same name and type as this field and turns it
    /// into a static reference to its dereffed contents.
    pub fn make_illegal_static_reference(&self) -> TokenStream {
        let field_name = &self.name;
        let ref_name = self.illegal_ref_name();
        quote! {
            let #ref_name = unsafe {
                ::ouroboros::macro_help::change_lifetime(&*#field_name)
            };
        }
    }

    /// Like make_illegal_static_reference, but provides a mutable reference instead.
    pub fn make_illegal_static_mut_reference(&self) -> TokenStream {
        let field_name = &self.name;
        let ref_name = self.illegal_ref_name();
        quote! {
            let #ref_name = unsafe {
                ::ouroboros::macro_help::change_lifetime_mut(&mut *#field_name)
            };
        }
    }

    /// Generates an error requesting that the user explicitly specify whether or not the
    /// field's type is covariant.
    #[must_use]
    pub fn covariance_error(&self) -> Diagnostic {
        let error = concat!(
            "Ouroboros cannot automatically determine if this type is covariant.\n\n",
            "As an example, a Box<&'this ()> is covariant because it can be used as a\n",
            "Box<&'smaller ()> for any lifetime smaller than 'this. In contrast,\n",
            "a Fn(&'this ()) is not covariant because it cannot be used as a\n",
            "Fn(&'smaller ()). In general, values that are safe to use with smaller\n",
            "lifetimes than they were defined with are covariant, breaking this \n",
            "guarantee means the value is not covariant.\n\n",
            "To resolve this error, add #[covariant] or #[not_covariant] to the field.\n",
        );
        self.typ.span().error(error)
    }

    pub fn make_constructor_arg_type_impl(
        &self,
        info: &StructInfo,
        make_builder_return_type: impl FnOnce() -> TokenStream,
    ) -> Result<ArgType, Error> {
        let field_type = &self.typ;
        let fake_lifetime = info.fake_lifetime();
        if self.borrows.is_empty() {
            // Even if self_referencing is true, as long as borrows is empty, we don't need to use a
            // builder to construct it.
            let field_type =
                replace_this_with_lifetime(field_type.into_token_stream(), fake_lifetime.clone());
            Ok(ArgType::Plain(quote! { #field_type }))
        } else {
            let mut field_builder_params = Vec::new();
            for borrow in &self.borrows {
                if borrow.mutable {
                    let field = &info.fields[borrow.index];
                    let field_type = &field.typ;
                    field_builder_params.push(quote! {
                        &'this mut #field_type
                    });
                } else {
                    let field = &info.fields[borrow.index];
                    let field_type = &field.typ;
                    field_builder_params.push(quote! {
                        &'this #field_type
                    });
                }
            }
            let return_type = make_builder_return_type();
            let bound = quote! { for<'this> ::core::ops::FnOnce(#(#field_builder_params),*) -> #return_type };
            Ok(ArgType::TraitBound(bound))
        }
    }

    /// Returns a trait bound if `for_field` refers to any other fields, and a plain type if not. This
    /// is the type used in the constructor to initialize the value of `for_field`.
    pub fn make_constructor_arg_type(
        &self,
        info: &StructInfo,
        builder_type: BuilderType,
    ) -> Result<ArgType, Error> {
        let field_type = &self.typ;
        let return_ty_constructor = || match builder_type {
            BuilderType::AsyncSend => {
                quote! {
                    ::core::pin::Pin<::ouroboros::macro_help::alloc::boxed::Box<
                        dyn ::core::future::Future<Output=#field_type> + ::core::marker::Send + 'this>>
                }
            }
            BuilderType::Async => {
                quote! { ::core::pin::Pin<::ouroboros::macro_help::alloc::boxed::Box<
                dyn ::core::future::Future<Output=#field_type> + 'this>> }
            }
            BuilderType::Sync => quote! { #field_type },
        };
        self.make_constructor_arg_type_impl(info, return_ty_constructor)
    }

    /// Like make_constructor_arg_type, but used for the try_new constructor.
    pub fn make_try_constructor_arg_type(
        &self,
        info: &StructInfo,
        builder_type: BuilderType,
    ) -> Result<ArgType, Error> {
        let field_type = &self.typ;
        let return_ty_constructor = || match builder_type {
            BuilderType::AsyncSend => {
                quote! {
                    ::core::pin::Pin<::ouroboros::macro_help::alloc::boxed::Box<
                        dyn ::core::future::Future<Output=::core::result::Result<#field_type, Error_>>
                            + ::core::marker::Send + 'this>>
                }
            }
            BuilderType::Async => {
                quote! {
                    ::core::pin::Pin<::ouroboros::macro_help::alloc::boxed::Box<
                        dyn ::core::future::Future<Output=::core::result::Result<#field_type, Error_>>
                            + 'this>>
                }
            }
            BuilderType::Sync => quote! { ::core::result::Result<#field_type, Error_> },
        };
        self.make_constructor_arg_type_impl(info, return_ty_constructor)
    }
}
