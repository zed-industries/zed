use super::member::{GetterConfig, GetterKind};
use super::{BuilderGenCtx, NamedMember};
use crate::parsing::SpannedKey;
use crate::util::prelude::*;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;

pub(crate) struct GettersCtx<'a> {
    base: &'a BuilderGenCtx,
    member: &'a NamedMember,
    config: &'a GetterConfig,
}

impl<'a> GettersCtx<'a> {
    pub(crate) fn new(base: &'a BuilderGenCtx, member: &'a NamedMember) -> Option<Self> {
        Some(Self {
            base,
            member,
            config: member.config.getter.as_ref()?,
        })
    }

    pub(crate) fn getter_methods(self) -> Result<TokenStream> {
        let name = self.config.name.as_deref().cloned().unwrap_or_else(|| {
            syn::Ident::new(
                &format!("get_{}", self.member.name.snake.raw_name()),
                self.member.name.snake.span(),
            )
        });

        let vis = self
            .config
            .vis
            .as_deref()
            .unwrap_or(&self.base.builder_type.vis)
            .clone();

        let docs = self.config.docs.as_deref().cloned().unwrap_or_else(|| {
            let header = format!(
                "_**Getter.**_ Returns `{}`, which must be set before calling this method.\n\n",
                self.member.name.snake,
            );

            std::iter::once(syn::parse_quote!(#[doc = #header]))
                .chain(self.member.docs.iter().cloned())
                .collect()
        });

        let return_ty = self.return_ty()?;
        let body = self.body();

        let state_var = &self.base.state_var;
        let member_pascal = &self.member.name.pascal;
        let state_mod = &self.base.state_mod.ident;
        let const_ = &self.base.const_;
        let fn_modifiers = self.member.respan(quote!(#vis #const_));

        // It's important to keep the span of `self` the same across all
        // references to it. Otherwise `self`s that have different spans will
        // be treated as totally different symbols due to the hygiene rules.
        let self_ = quote!(self);

        Ok(quote_spanned! {self.member.span=>
            #( #docs )*
            #[allow(
                // This is intentional. We want the builder syntax to compile away
                clippy::inline_always,
                clippy::missing_const_for_fn,
            )]
            #[inline(always)]
            #[must_use = "this method has no side effects; it only returns a value"]
            #(#fn_modifiers)* fn #name(&#self_) -> #return_ty
            where
                #state_var::#member_pascal: #state_mod::IsSet,
            {
                #body
            }
        })
    }

    fn body(&self) -> TokenStream {
        let index = &self.member.index;
        let member = quote! {
            self.__unsafe_private_named.#index
        };

        let bon = &self.base.bon;

        match self.config.kind.as_deref() {
            Some(GetterKind::Copy) => {
                // Use a `_` type hint with the span of the original type
                // to make the compiler point to the original type in case
                // if the type doesn't implement `Copy`.
                let span = self.member.underlying_orig_ty().span();
                let ty = quote_spanned!(span=> _);

                let copy = quote! {
                    #bon::__::better_errors::copy_member::<#ty>(&#member)
                };

                if !self.member.is_required() {
                    return copy;
                }
                quote! {
                    // SAFETY: the method requires S::{Member}: IsSet, so it's Some
                    unsafe {
                        ::core::option::Option::unwrap_unchecked(#copy)
                    }
                }
            }
            Some(GetterKind::Clone) => {
                // Use a `_` type hint with the span of the original type
                // to make the compiler point to the original type in case
                // if the type doesn't implement `Clone`.
                let span = self.member.underlying_orig_ty().span();
                let ty = quote_spanned!(span=> _);

                let clone = quote! {
                    <#ty as ::core::clone::Clone>::clone
                };

                if !self.member.is_required() {
                    return quote! {
                        #clone(&#member)
                    };
                }
                quote! {
                    match &#member {
                        Some(value) => #clone(value),

                        // SAFETY: the method requires S::{Member}: IsSet, so it's Some
                        None => unsafe {
                            ::core::hint::unreachable_unchecked()
                        },
                    }
                }
            }
            Some(GetterKind::Deref(ty)) => {
                // Assign the span of the deref target type to the `value` variable
                // so that compiler points to that type if there is a type mismatch.
                let span = ty.span();
                let value = quote_spanned!(span=> value);

                if !self.member.is_required() {
                    return quote! {
                        // Explicit match is important to trigger an implicit deref coercion
                        // that can potentially do multiple derefs to the reach the target type.
                        match &#member {
                            Some(#value) => Some(#value),
                            None => None,
                        }
                    };
                }
                quote! {
                    // Explicit match is important to trigger an implicit deref coercion
                    // that can potentially do multiple derefs to the reach the target type.
                    match &#member {
                        Some(#value) => #value,

                        // SAFETY: the method requires S::{Member}: IsSet, so it's Some
                        None => unsafe {
                            ::core::hint::unreachable_unchecked()
                        },
                    }
                }
            }
            None => {
                if !self.member.is_required() {
                    return quote! {
                        ::core::option::Option::as_ref(&#member)
                    };
                }
                quote! {
                    match &#member {
                        Some(value) => value,

                        // SAFETY: the method requires S::{Member}: IsSet, so it's Some
                        None => unsafe {
                            ::core::hint::unreachable_unchecked()
                        },
                    }
                }
            }
        }
    }

    fn return_ty(&self) -> Result<TokenStream> {
        let underlying_return_ty = self.underlying_return_ty()?;

        Ok(if self.member.is_required() {
            quote! { #underlying_return_ty }
        } else {
            // We are not using the fully qualified path to `Option` here
            // to make function signature in IDE popus shorter and more
            // readable.
            quote! { Option<#underlying_return_ty> }
        })
    }

    fn underlying_return_ty(&self) -> Result<TokenStream> {
        let ty = self.member.underlying_norm_ty();

        let kind = match &self.config.kind {
            Some(kind) => kind,
            None => return Ok(quote! { &#ty }),
        };

        match &kind.value {
            GetterKind::Copy | GetterKind::Clone => Ok(quote! { #ty }),
            GetterKind::Deref(Some(deref_target)) => Ok(quote! { &#deref_target }),
            GetterKind::Deref(None) => Self::infer_deref_target(ty, kind),
        }
    }

    fn infer_deref_target(
        underlying_member_ty: &syn::Type,
        kind: &SpannedKey<GetterKind>,
    ) -> Result<TokenStream> {
        use quote_spanned as qs;

        let span = underlying_member_ty.span();

        #[allow(clippy::type_complexity)]
        let deref_target_inference_table: &[(_, &dyn Fn(&Punctuated<_, _>) -> _)] = &[
            ("Vec", &|args| args.first().map(|arg| qs!(span=> [#arg]))),
            ("Box", &|args| args.first().map(ToTokens::to_token_stream)),
            ("Rc", &|args| args.first().map(ToTokens::to_token_stream)),
            ("Arc", &|args| args.first().map(ToTokens::to_token_stream)),
            ("String", &|args| args.is_empty().then(|| qs!(span=> str))),
            ("CString", &|args| {
                // CStr is available via `core` since 1.64.0:
                // https://blog.rust-lang.org/2022/09/22/Rust-1.64.0.html#c-compatible-ffi-types-in-core-and-alloc
                let module = if rustversion::cfg!(since(1.64.0)) {
                    format_ident!("core")
                } else {
                    format_ident!("std")
                };
                args.is_empty().then(|| qs!(span=> ::#module::ffi::CStr))
            }),
            ("OsString", &|args| {
                args.is_empty().then(|| qs!(span=> ::std::ffi::OsStr))
            }),
            ("PathBuf", &|args| {
                args.is_empty().then(|| qs!(span=> ::std::path::Path))
            }),
            ("Cow", &|args| {
                args.iter()
                    .find(|arg| matches!(arg, syn::GenericArgument::Type(_)))
                    .map(ToTokens::to_token_stream)
            }),
        ];

        let err = || {
            let inferable_types = deref_target_inference_table
                .iter()
                .map(|(name, _)| format!("- {name}"))
                .join("\n");

            err!(
                &kind.key,
                "can't infer the `Deref::Target` for the getter from the member's type; \
                please specify the return type (target of the deref coercion) explicitly \
                in parentheses without the leading `&`;\n\
                example: `#[builder(getter(deref(TargetTypeHere))]`\n\
                \n\
                automatic deref target detection is supported only for the following types:\n\
                {inferable_types}",
            )
        };

        let path = underlying_member_ty.as_path_no_qself().ok_or_else(err)?;

        let last_segment = path.segments.last().ok_or_else(err)?;

        let empty_punctuated = Punctuated::new();

        let args = match &last_segment.arguments {
            syn::PathArguments::AngleBracketed(args) => &args.args,
            _ => &empty_punctuated,
        };

        let last_segment_ident_str = last_segment.ident.to_string();

        let inferred = deref_target_inference_table
            .iter()
            .find(|(name, _)| last_segment_ident_str == *name)
            .and_then(|(_, infer)| infer(args))
            .ok_or_else(err)?;

        Ok(quote!(&#inferred))
    }
}
