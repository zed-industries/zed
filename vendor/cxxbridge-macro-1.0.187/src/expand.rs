use crate::syntax::atom::Atom::*;
use crate::syntax::attrs::{self, OtherAttrs};
use crate::syntax::cfg::{CfgExpr, ComputedCfg};
use crate::syntax::file::Module;
use crate::syntax::instantiate::{ImplKey, NamedImplKey};
use crate::syntax::map::OrderedMap;
use crate::syntax::message::Message;
use crate::syntax::namespace::Namespace;
use crate::syntax::qualified::QualifiedName;
use crate::syntax::report::Errors;
use crate::syntax::set::UnorderedSet;
use crate::syntax::symbol::Symbol;
use crate::syntax::trivial::TrivialReason;
use crate::syntax::types::ConditionalImpl;
use crate::syntax::unpin::UnpinReason;
use crate::syntax::{
    self, check, mangle, Api, Doc, Enum, ExternFn, ExternType, FnKind, Lang, Pair, Signature,
    Struct, Trait, Type, TypeAlias, Types,
};
use crate::type_id::Crate;
use crate::{derive, generics};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote, quote_spanned, ToTokens};
use std::fmt::{self, Display};
use std::mem;
use syn::{parse_quote, GenericParam, Generics, Lifetime, Result, Token, Visibility};

pub(crate) fn bridge(mut ffi: Module) -> Result<TokenStream> {
    let ref mut errors = Errors::new();

    let mut cfg = CfgExpr::Unconditional;
    let mut doc = Doc::new();
    let attrs = attrs::parse(
        errors,
        mem::take(&mut ffi.attrs),
        attrs::Parser {
            cfg: Some(&mut cfg),
            doc: Some(&mut doc),
            ..Default::default()
        },
    );

    let content = mem::take(&mut ffi.content);
    let trusted = ffi.unsafety.is_some();
    let namespace = &ffi.namespace;
    let ref mut apis = syntax::parse_items(errors, content, trusted, namespace);
    let ref types = Types::collect(errors, apis);
    errors.propagate()?;

    let generator = check::Generator::Macro;
    check::typecheck(errors, apis, types, generator);
    errors.propagate()?;

    Ok(expand(ffi, doc, attrs, apis, types))
}

fn expand(ffi: Module, doc: Doc, attrs: OtherAttrs, apis: &[Api], types: &Types) -> TokenStream {
    let mut expanded = TokenStream::new();
    let mut hidden = TokenStream::new();
    let mut forbid = TokenStream::new();

    for api in apis {
        if let Api::RustType(ety) = api {
            expanded.extend(expand_rust_type_import(ety));
            hidden.extend(expand_rust_type_assert_unpin(ety, types));
        }
    }

    for api in apis {
        match api {
            Api::Include(_) | Api::Impl(_) => {}
            Api::Struct(strct) => {
                expanded.extend(expand_struct(strct));
                expanded.extend(expand_associated_functions(&strct.name.rust, types));
                hidden.extend(expand_struct_nonempty(strct));
                hidden.extend(expand_struct_operators(strct));
                forbid.extend(expand_struct_forbid_drop(strct));
            }
            Api::Enum(enm) => expanded.extend(expand_enum(enm)),
            Api::CxxType(ety) => {
                let ident = &ety.name.rust;
                if types.structs.contains_key(ident) {
                    hidden.extend(expand_extern_shared_struct(ety, &ffi));
                } else if !types.enums.contains_key(ident) {
                    expanded.extend(expand_cxx_type(ety));
                    expanded.extend(expand_associated_functions(&ety.name.rust, types));
                    hidden.extend(expand_cxx_type_assert_pinned(ety, types));
                }
            }
            Api::CxxFunction(efn) => {
                if efn.self_type().is_none() {
                    expanded.extend(expand_cxx_function_shim(efn, types));
                }
            }
            Api::RustType(ety) => {
                expanded.extend(expand_rust_type_impl(ety));
                expanded.extend(expand_associated_functions(&ety.name.rust, types));
                hidden.extend(expand_rust_type_layout(ety, types));
            }
            Api::RustFunction(efn) => hidden.extend(expand_rust_function_shim(efn, types)),
            Api::TypeAlias(alias) => {
                expanded.extend(expand_type_alias(alias));
                expanded.extend(expand_associated_functions(&alias.name.rust, types));
                hidden.extend(expand_type_alias_verify(alias, types));
            }
        }
    }

    for (impl_key, conditional_impl) in &types.impls {
        match impl_key {
            ImplKey::RustBox(ident) => {
                hidden.extend(expand_rust_box(ident, types, conditional_impl));
            }
            ImplKey::RustVec(ident) => {
                hidden.extend(expand_rust_vec(ident, types, conditional_impl));
            }
            ImplKey::UniquePtr(ident) => {
                expanded.extend(expand_unique_ptr(ident, types, conditional_impl));
            }
            ImplKey::SharedPtr(ident) => {
                expanded.extend(expand_shared_ptr(ident, types, conditional_impl));
            }
            ImplKey::WeakPtr(ident) => {
                expanded.extend(expand_weak_ptr(ident, types, conditional_impl));
            }
            ImplKey::CxxVector(ident) => {
                expanded.extend(expand_cxx_vector(ident, conditional_impl, types));
            }
        }
    }

    if !forbid.is_empty() {
        hidden.extend(expand_forbid(forbid));
    }

    // Work around https://github.com/rust-lang/rust/issues/67851.
    if !hidden.is_empty() {
        expanded.extend(quote! {
            #[doc(hidden)]
            const _: () = {
                #hidden
            };
        });
    }

    let all_attrs = attrs.all();
    let vis = &ffi.vis;
    let mod_token = &ffi.mod_token;
    let ident = &ffi.ident;
    let span = ffi.brace_token.span;
    let expanded = quote_spanned!(span=> {#expanded});

    quote! {
        #doc
        #all_attrs
        #[deny(improper_ctypes, improper_ctypes_definitions)]
        #[allow(clippy::unknown_lints)]
        #[allow(
            non_camel_case_types,
            non_snake_case,
            clippy::extra_unused_type_parameters,
            clippy::items_after_statements,
            clippy::no_effect_underscore_binding,
            clippy::ptr_as_ptr,
            clippy::ref_as_ptr,
            clippy::unsafe_derive_deserialize,
            clippy::upper_case_acronyms,
            clippy::use_self,
        )]
        #vis #mod_token #ident #expanded
    }
}

fn expand_struct(strct: &Struct) -> TokenStream {
    let ident = &strct.name.rust;
    let doc = &strct.doc;
    let all_attrs = strct.attrs.all();
    let cfg_and_lint_attrs = strct.attrs.cfg_and_lint();
    let generics = &strct.generics;
    let type_id = type_id(&strct.name);
    let fields = strct.fields.iter().map(|field| {
        let doc = &field.doc;
        let all_attrs = field.attrs.all();
        // This span on the pub makes "private type in public interface" errors
        // appear in the right place.
        let vis = field.visibility;
        quote!(#doc #all_attrs #vis #field)
    });
    let mut derives = None;
    let derived_traits = derive::expand_struct(strct, &mut derives);

    let span = ident.span();
    let visibility = strct.visibility;
    let struct_token = strct.struct_token;
    let struct_def = quote_spanned! {span=>
        #visibility #struct_token #ident #generics {
            #(#fields,)*
        }
    };

    let align = strct.align.as_ref().map(|align| quote!(, align(#align)));

    quote! {
        #doc
        #derives
        #all_attrs
        #[repr(C #align)]
        #struct_def

        #cfg_and_lint_attrs
        #[automatically_derived]
        unsafe impl #generics ::cxx::ExternType for #ident #generics {
            #[allow(unused_attributes)] // incorrect lint
            #[doc(hidden)]
            type Id = #type_id;
            type Kind = ::cxx::kind::Trivial;
        }

        #derived_traits
    }
}

fn expand_struct_nonempty(strct: &Struct) -> TokenStream {
    let has_unconditional_field = strct
        .fields
        .iter()
        .any(|field| matches!(field.cfg, CfgExpr::Unconditional));
    if has_unconditional_field {
        return TokenStream::new();
    }

    let mut fields = strct.fields.iter();
    let mut cfg = ComputedCfg::from(&fields.next().unwrap().cfg);
    fields.for_each(|field| cfg.merge_or(&field.cfg));

    if let ComputedCfg::Leaf(CfgExpr::Unconditional) = cfg {
        // At least one field is unconditional, nothing to check.
        TokenStream::new()
    } else {
        let meta = cfg.as_meta();
        let msg = "structs without any fields are not supported";
        let error = syn::Error::new_spanned(strct, msg).into_compile_error();
        quote! {
            #[cfg(not(#meta))]
            #error
        }
    }
}

fn expand_struct_operators(strct: &Struct) -> TokenStream {
    let ident = &strct.name.rust;
    let generics = &strct.generics;
    let cfg_and_lint_attrs = strct.attrs.cfg_and_lint();
    let mut operators = TokenStream::new();

    for derive in &strct.derives {
        let span = derive.span;
        match derive.what {
            Trait::PartialEq => {
                let link_name = mangle::operator(&strct.name, "eq");
                let local_name = format_ident!("__operator_eq_{}", strct.name.rust);
                let prevent_unwind_label = format!("::{} as PartialEq>::eq", strct.name.rust);
                operators.extend(quote_spanned! {span=>
                    #cfg_and_lint_attrs
                    #[doc(hidden)]
                    #[unsafe(export_name = #link_name)]
                    extern "C" fn #local_name #generics(lhs: &#ident #generics, rhs: &#ident #generics) -> ::cxx::core::primitive::bool {
                        let __fn = ::cxx::core::concat!("<", ::cxx::core::module_path!(), #prevent_unwind_label);
                        ::cxx::private::prevent_unwind(__fn, || *lhs == *rhs)
                    }
                });

                if !derive::contains(&strct.derives, Trait::Eq) {
                    let link_name = mangle::operator(&strct.name, "ne");
                    let local_name = format_ident!("__operator_ne_{}", strct.name.rust);
                    let prevent_unwind_label = format!("::{} as PartialEq>::ne", strct.name.rust);
                    operators.extend(quote_spanned! {span=>
                        #cfg_and_lint_attrs
                        #[doc(hidden)]
                        #[unsafe(export_name = #link_name)]
                        extern "C" fn #local_name #generics(lhs: &#ident #generics, rhs: &#ident #generics) -> ::cxx::core::primitive::bool {
                            let __fn = ::cxx::core::concat!("<", ::cxx::core::module_path!(), #prevent_unwind_label);
                            ::cxx::private::prevent_unwind(__fn, || *lhs != *rhs)
                        }
                    });
                }
            }
            Trait::PartialOrd => {
                let link_name = mangle::operator(&strct.name, "lt");
                let local_name = format_ident!("__operator_lt_{}", strct.name.rust);
                let prevent_unwind_label = format!("::{} as PartialOrd>::lt", strct.name.rust);
                operators.extend(quote_spanned! {span=>
                    #cfg_and_lint_attrs
                    #[doc(hidden)]
                    #[unsafe(export_name = #link_name)]
                    extern "C" fn #local_name #generics(lhs: &#ident #generics, rhs: &#ident #generics) -> ::cxx::core::primitive::bool {
                        let __fn = ::cxx::core::concat!("<", ::cxx::core::module_path!(), #prevent_unwind_label);
                        ::cxx::private::prevent_unwind(__fn, || *lhs < *rhs)
                    }
                });

                let link_name = mangle::operator(&strct.name, "le");
                let local_name = format_ident!("__operator_le_{}", strct.name.rust);
                let prevent_unwind_label = format!("::{} as PartialOrd>::le", strct.name.rust);
                operators.extend(quote_spanned! {span=>
                    #cfg_and_lint_attrs
                    #[doc(hidden)]
                    #[unsafe(export_name = #link_name)]
                    extern "C" fn #local_name #generics(lhs: &#ident #generics, rhs: &#ident #generics) -> ::cxx::core::primitive::bool {
                        let __fn = ::cxx::core::concat!("<", ::cxx::core::module_path!(), #prevent_unwind_label);
                        ::cxx::private::prevent_unwind(__fn, || *lhs <= *rhs)
                    }
                });

                if !derive::contains(&strct.derives, Trait::Ord) {
                    let link_name = mangle::operator(&strct.name, "gt");
                    let local_name = format_ident!("__operator_gt_{}", strct.name.rust);
                    let prevent_unwind_label = format!("::{} as PartialOrd>::gt", strct.name.rust);
                    operators.extend(quote_spanned! {span=>
                        #cfg_and_lint_attrs
                        #[doc(hidden)]
                        #[unsafe(export_name = #link_name)]
                        extern "C" fn #local_name #generics(lhs: &#ident #generics, rhs: &#ident #generics) -> ::cxx::core::primitive::bool {
                            let __fn = ::cxx::core::concat!("<", ::cxx::core::module_path!(), #prevent_unwind_label);
                            ::cxx::private::prevent_unwind(__fn, || *lhs > *rhs)
                        }
                    });

                    let link_name = mangle::operator(&strct.name, "ge");
                    let local_name = format_ident!("__operator_ge_{}", strct.name.rust);
                    let prevent_unwind_label = format!("::{} as PartialOrd>::ge", strct.name.rust);
                    operators.extend(quote_spanned! {span=>
                        #cfg_and_lint_attrs
                        #[doc(hidden)]
                        #[unsafe(export_name = #link_name)]
                        extern "C" fn #local_name #generics(lhs: &#ident #generics, rhs: &#ident #generics) -> ::cxx::core::primitive::bool {
                            let __fn = ::cxx::core::concat!("<", ::cxx::core::module_path!(), #prevent_unwind_label);
                            ::cxx::private::prevent_unwind(__fn, || *lhs >= *rhs)
                        }
                    });
                }
            }
            Trait::Hash => {
                let link_name = mangle::operator(&strct.name, "hash");
                let local_name = format_ident!("__operator_hash_{}", strct.name.rust);
                let prevent_unwind_label = format!("::{} as Hash>::hash", strct.name.rust);
                operators.extend(quote_spanned! {span=>
                    #cfg_and_lint_attrs
                    #[doc(hidden)]
                    #[unsafe(export_name = #link_name)]
                    #[allow(clippy::cast_possible_truncation)]
                    extern "C" fn #local_name #generics(this: &#ident #generics) -> ::cxx::core::primitive::usize {
                        let __fn = ::cxx::core::concat!("<", ::cxx::core::module_path!(), #prevent_unwind_label);
                        ::cxx::private::prevent_unwind(__fn, || ::cxx::private::hash(this))
                    }
                });
            }
            _ => {}
        }
    }

    operators
}

fn expand_struct_forbid_drop(strct: &Struct) -> TokenStream {
    let ident = &strct.name.rust;
    let generics = &strct.generics;
    let cfg_and_lint_attrs = strct.attrs.cfg_and_lint();
    let span = ident.span();
    let impl_token = Token![impl](strct.visibility.span);

    quote_spanned! {span=>
        #cfg_and_lint_attrs
        #[automatically_derived]
        #impl_token #generics self::Drop for super::#ident #generics {}
    }
}

fn expand_enum(enm: &Enum) -> TokenStream {
    let ident = &enm.name.rust;
    let doc = &enm.doc;
    let all_attrs = enm.attrs.all();
    let cfg_and_lint_attrs = enm.attrs.cfg_and_lint();
    let repr = &enm.repr;
    let type_id = type_id(&enm.name);
    let variants = enm.variants.iter().map(|variant| {
        let doc = &variant.doc;
        let all_attrs = variant.attrs.all();
        let variant_ident = &variant.name.rust;
        let discriminant = &variant.discriminant;
        let span = variant_ident.span();
        Some(quote_spanned! {span=>
            #doc
            #all_attrs
            #[allow(dead_code)]
            pub const #variant_ident: Self = #ident { repr: #discriminant };
        })
    });
    let mut derives = None;
    let derived_traits = derive::expand_enum(enm, &mut derives);

    let span = ident.span();
    let visibility = enm.visibility;
    let struct_token = Token![struct](enm.enum_token.span);
    let enum_repr = quote! {
        #[allow(missing_docs)]
        pub repr: #repr,
    };
    let enum_def = quote_spanned! {span=>
        #visibility #struct_token #ident {
            #enum_repr
        }
    };

    quote! {
        #doc
        #derives
        #all_attrs
        #[repr(transparent)]
        #enum_def

        #cfg_and_lint_attrs
        #[allow(non_upper_case_globals)]
        impl #ident {
            #(#variants)*
        }

        #cfg_and_lint_attrs
        #[automatically_derived]
        unsafe impl ::cxx::ExternType for #ident {
            #[allow(unused_attributes)] // incorrect lint
            #[doc(hidden)]
            type Id = #type_id;
            type Kind = ::cxx::kind::Trivial;
        }

        #derived_traits
    }
}

fn expand_cxx_type(ety: &ExternType) -> TokenStream {
    let ident = &ety.name.rust;
    let doc = &ety.doc;
    let all_attrs = ety.attrs.all();
    let cfg_and_lint_attrs = ety.attrs.cfg_and_lint();
    let generics = &ety.generics;
    let type_id = type_id(&ety.name);

    let lifetime_fields = ety.generics.lifetimes.iter().map(|lifetime| {
        let field = format_ident!("_lifetime_{}", lifetime.ident);
        quote!(#field: ::cxx::core::marker::PhantomData<&#lifetime ()>)
    });
    let repr_fields = quote! {
        _private: ::cxx::private::Opaque,
        #(#lifetime_fields,)*
    };

    let span = ident.span();
    let visibility = &ety.visibility;
    let struct_token = Token![struct](ety.type_token.span);
    let extern_type_def = quote_spanned! {span=>
        #visibility #struct_token #ident #generics {
            #repr_fields
        }
    };

    quote! {
        #doc
        #all_attrs
        #[repr(C)]
        #extern_type_def

        #cfg_and_lint_attrs
        #[automatically_derived]
        unsafe impl #generics ::cxx::ExternType for #ident #generics {
            #[allow(unused_attributes)] // incorrect lint
            #[doc(hidden)]
            type Id = #type_id;
            type Kind = ::cxx::kind::Opaque;
        }
    }
}

fn expand_cxx_type_assert_pinned(ety: &ExternType, types: &Types) -> TokenStream {
    let ident = &ety.name.rust;
    let cfg_and_lint_attrs = ety.attrs.cfg_and_lint();
    let infer = Token![_](ident.span());

    let resolve = types.resolve(ident);
    let lifetimes = resolve.generics.to_underscore_lifetimes();

    quote! {
        #cfg_and_lint_attrs
        let _: fn() = {
            // Derived from https://github.com/nvzqz/static-assertions-rs.
            trait __AmbiguousIfImpl<A> {
                fn infer() {}
            }

            #[automatically_derived]
            impl<T> __AmbiguousIfImpl<()> for T
            where
                T: ?::cxx::core::marker::Sized
            {}

            #[allow(dead_code)]
            struct __Invalid;

            #[automatically_derived]
            impl<T> __AmbiguousIfImpl<__Invalid> for T
            where
                T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin,
            {}

            // If there is only one specialized trait impl, type inference with
            // `_` can be resolved and this can compile. Fails to compile if
            // user has added a manual Unpin impl for their opaque C++ type as
            // then `__AmbiguousIfImpl<__Invalid>` also exists.
            <#ident #lifetimes as __AmbiguousIfImpl<#infer>>::infer
        };
    }
}

fn expand_extern_shared_struct(ety: &ExternType, ffi: &Module) -> TokenStream {
    let module = &ffi.ident;
    let name = &ety.name.rust;
    let namespaced_name = display_namespaced(&ety.name);
    let cfg_and_lint_attrs = ety.attrs.cfg_and_lint();

    let visibility = match &ffi.vis {
        Visibility::Public(_) => "pub ".to_owned(),
        Visibility::Restricted(vis) => {
            format!(
                "pub(in {}) ",
                vis.path
                    .segments
                    .iter()
                    .map(|segment| segment.ident.to_string())
                    .collect::<Vec<_>>()
                    .join("::"),
            )
        }
        Visibility::Inherited => String::new(),
    };

    let namespace_attr = if ety.name.namespace == Namespace::ROOT {
        String::new()
    } else {
        format!(
            "#[namespace = \"{}\"]\n        ",
            ety.name
                .namespace
                .iter()
                .map(Ident::to_string)
                .collect::<Vec<_>>()
                .join("::"),
        )
    };

    let message = format!(
        "\
        \nShared struct redeclared as an unsafe extern C++ type is deprecated.\
        \nIf this is intended to be a shared struct, remove this `type {name}`.\
        \nIf this is intended to be an extern type, change it to:\
        \n\
        \n    use cxx::ExternType;\
        \n    \
        \n    #[repr(C)]\
        \n    {visibility}struct {name} {{\
        \n        ...\
        \n    }}\
        \n    \
        \n    unsafe impl ExternType for {name} {{\
        \n        type Id = cxx::type_id!(\"{namespaced_name}\");\
        \n        type Kind = cxx::kind::Trivial;\
        \n    }}\
        \n    \
        \n    {visibility}mod {module} {{\
        \n        {namespace_attr}extern \"C++\" {{\
        \n            type {name} = crate::{name};\
        \n        }}\
        \n        ...\
        \n    }}",
    );

    quote! {
        #cfg_and_lint_attrs
        #[deprecated = #message]
        struct #name {}

        #cfg_and_lint_attrs
        let _ = #name {};
    }
}

fn expand_associated_functions(self_type: &Ident, types: &Types) -> TokenStream {
    let Some(functions) = types.associated_fn.get(self_type) else {
        return TokenStream::new();
    };

    let resolve = types.resolve(self_type);
    let self_type_cfg_attrs = resolve.attrs.cfg();
    let elided_lifetime = Lifetime::new("'_", Span::call_site());
    let mut group_by_lifetimes = OrderedMap::new();
    let mut tokens = TokenStream::new();

    for efn in functions {
        match efn.lang {
            Lang::Cxx | Lang::CxxUnwind => {}
            Lang::Rust => continue,
        }
        let mut impl_lifetimes = Vec::new();
        let mut self_type_lifetimes = Vec::new();
        let self_lt_token;
        let self_gt_token;
        match &efn.kind {
            FnKind::Method(receiver) if receiver.ty.generics.lt_token.is_some() => {
                for lifetime in &receiver.ty.generics.lifetimes {
                    if lifetime.ident != "_"
                        && efn
                            .generics
                            .lifetimes()
                            .any(|param| param.lifetime == *lifetime)
                    {
                        impl_lifetimes.push(lifetime);
                    }
                    self_type_lifetimes.push(lifetime);
                }
                self_lt_token = receiver.ty.generics.lt_token;
                self_gt_token = receiver.ty.generics.gt_token;
            }
            _ => {
                self_type_lifetimes.resize(resolve.generics.lifetimes.len(), &elided_lifetime);
                self_lt_token = resolve.generics.lt_token;
                self_gt_token = resolve.generics.gt_token;
            }
        }
        if efn.undeclared_lifetimes().is_empty()
            && self_type_lifetimes.len() == resolve.generics.lifetimes.len()
        {
            group_by_lifetimes
                .entry((impl_lifetimes, self_type_lifetimes))
                .or_insert_with(Vec::new)
                .push(efn);
        } else {
            let impl_token = Token![impl](efn.name.rust.span());
            let impl_lt_token = efn.generics.lt_token;
            let impl_gt_token = efn.generics.gt_token;
            let self_type = efn.self_type().unwrap();
            let function = expand_cxx_function_shim(efn, types);
            tokens.extend(quote! {
                #self_type_cfg_attrs
                #impl_token #impl_lt_token #(#impl_lifetimes),* #impl_gt_token #self_type #self_lt_token #(#self_type_lifetimes),* #self_gt_token {
                    #function
                }
            });
        }
    }

    for ((impl_lifetimes, self_type_lifetimes), functions) in &group_by_lifetimes {
        let functions = functions
            .iter()
            .map(|efn| expand_cxx_function_shim(efn, types));
        tokens.extend(quote! {
            #self_type_cfg_attrs
            impl <#(#impl_lifetimes),*> #self_type <#(#self_type_lifetimes),*> {
                #(#functions)*
            }
        });
    }

    tokens
}

fn expand_cxx_function_decl(efn: &ExternFn, types: &Types) -> TokenStream {
    let receiver = efn.receiver().into_iter().map(|receiver| {
        if types.is_considered_improper_ctype(&receiver.ty) {
            if receiver.mutable {
                quote!(_: *mut ::cxx::core::ffi::c_void)
            } else {
                quote!(_: *const ::cxx::core::ffi::c_void)
            }
        } else {
            let receiver_type = receiver.ty();
            quote!(_: #receiver_type)
        }
    });
    let args = efn.args.iter().map(|arg| {
        let var = &arg.name.rust;
        let colon = arg.colon_token;
        let ty = expand_extern_type(&arg.ty, types, true);
        if arg.ty == RustString {
            quote!(#var #colon *const #ty)
        } else if let Type::RustVec(_) = arg.ty {
            quote!(#var #colon *const #ty)
        } else if let Type::Fn(_) = arg.ty {
            quote!(#var #colon ::cxx::private::FatFunction)
        } else if types.needs_indirect_abi(&arg.ty) {
            quote!(#var #colon *mut #ty)
        } else {
            quote!(#var #colon #ty)
        }
    });
    let all_args = receiver.chain(args);
    let ret = if efn.throws {
        quote!(-> ::cxx::private::Result)
    } else {
        expand_extern_return_type(efn, types, true, efn.lang)
    };
    let mut outparam = None;
    if indirect_return(efn, types, efn.lang) {
        let ret = expand_extern_type(efn.ret.as_ref().unwrap(), types, true);
        outparam = Some(quote!(__return: *mut #ret));
    }
    let link_name = mangle::extern_fn(efn, types);
    let local_name = format_ident!("__{}", efn.name.rust);
    let lt_token = efn.generics.lt_token.unwrap_or_default();
    let undeclared_lifetimes = efn.undeclared_lifetimes().into_iter();
    let declared_lifetimes = &efn.generics.params;
    let gt_token = efn.generics.gt_token.unwrap_or_default();
    quote! {
        #[link_name = #link_name]
        fn #local_name #lt_token #(#undeclared_lifetimes,)* #declared_lifetimes #gt_token(#(#all_args,)* #outparam) #ret;
    }
}

fn expand_cxx_function_shim(efn: &ExternFn, types: &Types) -> TokenStream {
    let doc = &efn.doc;
    let all_attrs = efn.attrs.all();
    let decl = expand_cxx_function_decl(efn, types);
    let receiver = efn.receiver().into_iter().map(|receiver| {
        let var = receiver.var;
        if receiver.pinned {
            let colon = receiver.colon_token;
            let ty = receiver.ty_self();
            quote!(#var #colon #ty)
        } else {
            let ampersand = receiver.ampersand;
            let lifetime = &receiver.lifetime;
            let mutability = receiver.mutability;
            quote!(#ampersand #lifetime #mutability #var)
        }
    });
    let args = efn.args.iter().map(|arg| quote!(#arg));
    let all_args = receiver.chain(args);
    let ret = if efn.throws {
        let ok = match &efn.ret {
            Some(ret) => quote!(#ret),
            None => quote!(()),
        };
        quote!(-> ::cxx::core::result::Result<#ok, ::cxx::Exception>)
    } else {
        expand_return_type(&efn.ret)
    };
    let indirect_return = indirect_return(efn, types, efn.lang);
    let receiver_var = efn.receiver().into_iter().map(|receiver| {
        if types.is_considered_improper_ctype(&receiver.ty) {
            let var = receiver.var;
            let ty = &receiver.ty.rust;
            let resolve = types.resolve(ty);
            let lifetimes = resolve.generics.to_underscore_lifetimes();
            if receiver.pinned {
                quote!(::cxx::core::pin::Pin::into_inner_unchecked(#var) as *mut #ty #lifetimes as *mut ::cxx::core::ffi::c_void)
            } else if receiver.mutable {
                quote!(#var as *mut #ty #lifetimes as *mut ::cxx::core::ffi::c_void)
            } else {
                quote!(#var as *const #ty #lifetimes as *const ::cxx::core::ffi::c_void)
            }
        } else {
            receiver.var.to_token_stream()
        }
    });
    let arg_vars = efn.args.iter().map(|arg| {
        let var = &arg.name.rust;
        let span = var.span();
        match &arg.ty {
            Type::Ident(ident) if ident.rust == RustString => {
                quote_spanned!(span=> #var.as_mut_ptr() as *const ::cxx::private::RustString)
            }
            Type::RustBox(ty) => {
                if types.is_considered_improper_ctype(&ty.inner) {
                    quote_spanned!(span=> ::cxx::alloc::boxed::Box::into_raw(#var).cast())
                } else {
                    quote_spanned!(span=> ::cxx::alloc::boxed::Box::into_raw(#var))
                }
            }
            Type::UniquePtr(ty) => {
                if types.is_considered_improper_ctype(&ty.inner) {
                    quote_spanned!(span=> ::cxx::UniquePtr::into_raw(#var).cast())
                } else {
                    quote_spanned!(span=> ::cxx::UniquePtr::into_raw(#var))
                }
            }
            Type::RustVec(_) => quote_spanned!(span=> #var.as_mut_ptr() as *const ::cxx::private::RustVec<_>),
            Type::Ref(ty) => match &ty.inner {
                Type::Ident(ident) if ident.rust == RustString => match ty.mutable {
                    false => quote_spanned!(span=> ::cxx::private::RustString::from_ref(#var)),
                    true => quote_spanned!(span=> ::cxx::private::RustString::from_mut(#var)),
                },
                Type::RustVec(_) => match ty.mutable {
                    false => quote_spanned!(span=> ::cxx::private::RustVec::from_ref(#var)),
                    true => quote_spanned!(span=> ::cxx::private::RustVec::from_mut(#var)),
                },
                inner if types.is_considered_improper_ctype(inner) => {
                    let var = match ty.pinned {
                        false => quote!(#var),
                        true => quote_spanned!(span=> ::cxx::core::pin::Pin::into_inner_unchecked(#var)),
                    };
                    match ty.mutable {
                        false => {
                            quote_spanned!(span=> #var as *const #inner as *const ::cxx::core::ffi::c_void)
                        }
                        true => quote_spanned!(span=> #var as *mut #inner as *mut ::cxx::core::ffi::c_void),
                    }
                }
                _ => quote!(#var),
            },
            Type::Ptr(ty) => {
                if types.is_considered_improper_ctype(&ty.inner) {
                    quote_spanned!(span=> #var.cast())
                } else {
                    quote!(#var)
                }
            }
            Type::Str(_) => quote_spanned!(span=> ::cxx::private::RustStr::from(#var)),
            Type::SliceRef(ty) => match ty.mutable {
                false => quote_spanned!(span=> ::cxx::private::RustSlice::from_ref(#var)),
                true => quote_spanned!(span=> ::cxx::private::RustSlice::from_mut(#var)),
            },
            ty if types.needs_indirect_abi(ty) => quote_spanned!(span=> #var.as_mut_ptr()),
            _ => quote!(#var),
        }
    });
    let vars = receiver_var.chain(arg_vars);
    let trampolines = efn
        .args
        .iter()
        .filter_map(|arg| {
            if let Type::Fn(f) = &arg.ty {
                let var = &arg.name;
                Some(expand_function_pointer_trampoline(efn, var, f, types))
            } else {
                None
            }
        })
        .collect::<TokenStream>();
    let mut setup = efn
        .args
        .iter()
        .filter(|arg| types.needs_indirect_abi(&arg.ty))
        .map(|arg| {
            let var = &arg.name.rust;
            let span = var.span();
            // These are arguments for which C++ has taken ownership of the data
            // behind the mut reference it received.
            quote_spanned! {span=>
                let mut #var = ::cxx::core::mem::MaybeUninit::new(#var);
            }
        })
        .collect::<TokenStream>();
    let local_name = format_ident!("__{}", efn.name.rust);
    let span = efn.semi_token.span;
    let call = if indirect_return {
        let ret = expand_extern_type(efn.ret.as_ref().unwrap(), types, true);
        setup.extend(quote_spanned! {span=>
            let mut __return = ::cxx::core::mem::MaybeUninit::<#ret>::uninit();
        });
        setup.extend(if efn.throws {
            quote_spanned! {span=>
                #local_name(#(#vars,)* __return.as_mut_ptr()).exception()?;
            }
        } else {
            quote_spanned! {span=>
                #local_name(#(#vars,)* __return.as_mut_ptr());
            }
        });
        quote_spanned!(span=> __return.assume_init())
    } else if efn.throws {
        quote_spanned! {span=>
            #local_name(#(#vars),*).exception()
        }
    } else {
        quote_spanned! {span=>
            #local_name(#(#vars),*)
        }
    };
    let mut expr;
    if let Some(ret) = &efn.ret {
        expr = match ret {
            Type::Ident(ident) if ident.rust == RustString => {
                quote_spanned!(span=> #call.into_string())
            }
            Type::RustBox(ty) => {
                if types.is_considered_improper_ctype(&ty.inner) {
                    quote_spanned!(span=> ::cxx::alloc::boxed::Box::from_raw(#call.cast()))
                } else {
                    quote_spanned!(span=> ::cxx::alloc::boxed::Box::from_raw(#call))
                }
            }
            Type::RustVec(_) => {
                quote_spanned!(span=> #call.into_vec())
            }
            Type::UniquePtr(ty) => {
                if types.is_considered_improper_ctype(&ty.inner) {
                    quote_spanned!(span=> ::cxx::UniquePtr::from_raw(#call.cast()))
                } else {
                    quote_spanned!(span=> ::cxx::UniquePtr::from_raw(#call))
                }
            }
            Type::Ref(ty) => match &ty.inner {
                Type::Ident(ident) if ident.rust == RustString => match ty.mutable {
                    false => quote_spanned!(span=> #call.as_string()),
                    true => quote_spanned!(span=> #call.as_mut_string()),
                },
                Type::RustVec(_) => match ty.mutable {
                    false => quote_spanned!(span=> #call.as_vec()),
                    true => quote_spanned!(span=> #call.as_mut_vec()),
                },
                inner if types.is_considered_improper_ctype(inner) => {
                    let mutability = ty.mutability;
                    let deref_mut = quote_spanned!(span=> &#mutability *#call.cast());
                    match ty.pinned {
                        false => deref_mut,
                        true => {
                            quote_spanned!(span=> ::cxx::core::pin::Pin::new_unchecked(#deref_mut))
                        }
                    }
                }
                _ => call,
            },
            Type::Ptr(ty) => {
                if types.is_considered_improper_ctype(&ty.inner) {
                    quote_spanned!(span=> #call.cast())
                } else {
                    call
                }
            }
            Type::Str(_) => quote_spanned!(span=> #call.as_str()),
            Type::SliceRef(slice) => {
                let inner = &slice.inner;
                match slice.mutable {
                    false => quote_spanned!(span=> #call.as_slice::<#inner>()),
                    true => quote_spanned!(span=> #call.as_mut_slice::<#inner>()),
                }
            }
            _ => call,
        };
        if efn.throws {
            expr = quote_spanned!(span=> ::cxx::core::result::Result::Ok(#expr));
        }
    } else if efn.throws {
        expr = call;
    } else {
        expr = quote! { #call; };
    }
    let dispatch = quote_spanned!(span=> unsafe { #setup #expr });
    let visibility = efn.visibility;
    let unsafety = &efn.unsafety;
    let fn_token = efn.fn_token;
    let ident = &efn.name.rust;
    let lt_token = efn.generics.lt_token;
    let lifetimes = {
        let mut self_type_lifetimes = UnorderedSet::new();
        if let FnKind::Method(receiver) = &efn.kind {
            self_type_lifetimes.extend(&receiver.ty.generics.lifetimes);
        }
        efn.generics
            .params
            .pairs()
            .filter(move |param| match param.value() {
                GenericParam::Lifetime(param) => !self_type_lifetimes.contains(&param.lifetime),
                GenericParam::Type(_) | GenericParam::Const(_) => unreachable!(),
            })
    };
    let gt_token = efn.generics.gt_token;
    let arg_list = quote_spanned!(efn.paren_token.span=> (#(#all_args,)*));
    let calling_conv = match efn.lang {
        Lang::Cxx => quote_spanned!(span=> "C"),
        Lang::CxxUnwind => quote_spanned!(span=> "C-unwind"),
        Lang::Rust => unreachable!(),
    };
    quote_spanned! {span=>
        #doc
        #all_attrs
        #visibility #unsafety #fn_token #ident #lt_token #(#lifetimes)* #gt_token #arg_list #ret {
            unsafe extern #calling_conv {
                #decl
            }
            #trampolines
            #dispatch
        }
    }
}

fn expand_function_pointer_trampoline(
    efn: &ExternFn,
    var: &Pair,
    sig: &Signature,
    types: &Types,
) -> TokenStream {
    let c_trampoline = mangle::c_trampoline(efn, var, types);
    let r_trampoline = mangle::r_trampoline(efn, var, types);
    let local_name = parse_quote!(__);
    let prevent_unwind_label = format!("::{}::{}", efn.name.rust, var.rust);
    let body_span = efn.semi_token.span;
    let shim = expand_rust_function_shim_impl(
        sig,
        types,
        &r_trampoline,
        local_name,
        prevent_unwind_label,
        None,
        Some(&efn.generics),
        &efn.attrs,
        body_span,
    );
    let calling_conv = match efn.lang {
        Lang::Cxx => "C",
        Lang::CxxUnwind => "C-unwind",
        Lang::Rust => unreachable!(),
    };
    let var = &var.rust;

    quote! {
        let #var = ::cxx::private::FatFunction {
            trampoline: {
                unsafe extern #calling_conv {
                    #[link_name = #c_trampoline]
                    fn trampoline();
                }
                #shim
                trampoline as ::cxx::core::primitive::usize as *const ::cxx::core::ffi::c_void
            },
            ptr: #var as ::cxx::core::primitive::usize as *const ::cxx::core::ffi::c_void,
        };
    }
}

fn expand_rust_type_import(ety: &ExternType) -> TokenStream {
    let ident = &ety.name.rust;
    let all_attrs = ety.attrs.all();
    let span = ident.span();

    quote_spanned! {span=>
        #all_attrs
        use super::#ident;
    }
}

fn expand_rust_type_impl(ety: &ExternType) -> TokenStream {
    let ident = &ety.name.rust;
    let generics = &ety.generics;
    let cfg_and_lint_attrs = ety.attrs.cfg_and_lint();
    let span = ident.span();
    let unsafe_impl = quote_spanned!(ety.type_token.span=> unsafe impl);

    let mut impls = quote_spanned! {span=>
        #cfg_and_lint_attrs
        #[automatically_derived]
        #[doc(hidden)]
        #unsafe_impl #generics ::cxx::private::RustType for #ident #generics {}
    };

    for derive in &ety.derives {
        if derive.what == Trait::ExternType {
            let type_id = type_id(&ety.name);
            let span = derive.span;
            impls.extend(quote_spanned! {span=>
                #cfg_and_lint_attrs
                #[automatically_derived]
                unsafe impl #generics ::cxx::ExternType for #ident #generics {
                    #[allow(unused_attributes)] // incorrect lint
                    #[doc(hidden)]
                    type Id = #type_id;
                    type Kind = ::cxx::kind::Opaque;
                }
            });
        }
    }

    impls
}

fn expand_rust_type_assert_unpin(ety: &ExternType, types: &Types) -> TokenStream {
    let ident = &ety.name.rust;
    let cfg_and_lint_attrs = ety.attrs.cfg_and_lint();

    let resolve = types.resolve(ident);
    let lifetimes = resolve.generics.to_underscore_lifetimes();

    quote_spanned! {ident.span()=>
        #cfg_and_lint_attrs
        const _: fn() = ::cxx::private::require_unpin::<#ident #lifetimes>;
    }
}

fn expand_rust_type_layout(ety: &ExternType, types: &Types) -> TokenStream {
    // Rustc will render as follows if not sized:
    //
    //     type TheirType;
    //     -----^^^^^^^^^-
    //     |    |
    //     |    doesn't have a size known at compile-time
    //     required by this bound in `__AssertSized`

    let ident = &ety.name.rust;
    let cfg_and_lint_attrs = ety.attrs.cfg_and_lint();
    let begin_span = Token![::](ety.type_token.span);
    let sized = quote_spanned! {ety.semi_token.span=>
        #begin_span cxx::core::marker::Sized
    };

    let link_sizeof = mangle::operator(&ety.name, "sizeof");
    let link_alignof = mangle::operator(&ety.name, "alignof");

    let local_sizeof = format_ident!("__sizeof_{}", ety.name.rust);
    let local_alignof = format_ident!("__alignof_{}", ety.name.rust);

    let resolve = types.resolve(ident);
    let lifetimes = resolve.generics.to_underscore_lifetimes();

    quote_spanned! {ident.span()=>
        #cfg_and_lint_attrs
        {
            #[doc(hidden)]
            #[allow(clippy::needless_maybe_sized)]
            fn __AssertSized<T: ?#sized + #sized>() -> ::cxx::core::alloc::Layout {
                ::cxx::core::alloc::Layout::new::<T>()
            }
            #[doc(hidden)]
            #[unsafe(export_name = #link_sizeof)]
            extern "C" fn #local_sizeof() -> ::cxx::core::primitive::usize {
                __AssertSized::<#ident #lifetimes>().size()
            }
            #[doc(hidden)]
            #[unsafe(export_name = #link_alignof)]
            extern "C" fn #local_alignof() -> ::cxx::core::primitive::usize {
                __AssertSized::<#ident #lifetimes>().align()
            }
        }
    }
}

fn expand_forbid(impls: TokenStream) -> TokenStream {
    quote! {
        mod forbid {
            pub trait Drop {}
            #[automatically_derived]
            #[allow(drop_bounds)]
            impl<T: ?::cxx::core::marker::Sized + ::cxx::core::ops::Drop> self::Drop for T {}
            #impls
        }
    }
}

fn expand_rust_function_shim(efn: &ExternFn, types: &Types) -> TokenStream {
    let link_name = mangle::extern_fn(efn, types);
    let local_name = match efn.self_type() {
        None => format_ident!("__{}", efn.name.rust),
        Some(self_type) => format_ident!("__{}__{}", self_type, efn.name.rust),
    };
    let prevent_unwind_label = match efn.self_type() {
        None => format!("::{}", efn.name.rust),
        Some(self_type) => format!("::{}::{}", self_type, efn.name.rust),
    };
    let invoke = Some(&efn.name.rust);
    let body_span = efn.semi_token.span;
    expand_rust_function_shim_impl(
        efn,
        types,
        &link_name,
        local_name,
        prevent_unwind_label,
        invoke,
        None,
        &efn.attrs,
        body_span,
    )
}

fn expand_rust_function_shim_impl(
    sig: &Signature,
    types: &Types,
    link_name: &Symbol,
    local_name: Ident,
    prevent_unwind_label: String,
    invoke: Option<&Ident>,
    outer_generics: Option<&Generics>,
    attrs: &OtherAttrs,
    body_span: Span,
) -> TokenStream {
    let all_attrs = attrs.all();
    let generics = outer_generics.unwrap_or(&sig.generics);
    let receiver_var = sig
        .receiver()
        .map(|receiver| quote_spanned!(receiver.var.span=> __self));
    let receiver = sig.receiver().map(|receiver| {
        let colon = receiver.colon_token;
        let receiver_type = receiver.ty();
        quote!(#receiver_var #colon #receiver_type)
    });
    let args = sig.args.iter().map(|arg| {
        let var = &arg.name.rust;
        let colon = arg.colon_token;
        let ty = expand_extern_type(&arg.ty, types, false);
        if types.needs_indirect_abi(&arg.ty) {
            quote!(#var #colon *mut #ty)
        } else {
            quote!(#var #colon #ty)
        }
    });
    let all_args = receiver.into_iter().chain(args);

    let mut requires_unsafe = false;
    let arg_vars = sig.args.iter().map(|arg| {
        let var = &arg.name.rust;
        let span = var.span();
        match &arg.ty {
            Type::Ident(i) if i.rust == RustString => {
                requires_unsafe = true;
                quote_spanned!(span=> ::cxx::core::mem::take((*#var).as_mut_string()))
            }
            Type::RustBox(_) => {
                requires_unsafe = true;
                quote_spanned!(span=> ::cxx::alloc::boxed::Box::from_raw(#var))
            }
            Type::RustVec(_) => {
                requires_unsafe = true;
                quote_spanned!(span=> ::cxx::core::mem::take((*#var).as_mut_vec()))
            }
            Type::UniquePtr(_) => {
                requires_unsafe = true;
                quote_spanned!(span=> ::cxx::UniquePtr::from_raw(#var))
            }
            Type::Ref(ty) => match &ty.inner {
                Type::Ident(i) if i.rust == RustString => match ty.mutable {
                    false => quote_spanned!(span=> #var.as_string()),
                    true => quote_spanned!(span=> #var.as_mut_string()),
                },
                Type::RustVec(_) => match ty.mutable {
                    false => quote_spanned!(span=> #var.as_vec()),
                    true => quote_spanned!(span=> #var.as_mut_vec()),
                },
                _ => quote!(#var),
            },
            Type::Str(_) => {
                requires_unsafe = true;
                quote_spanned!(span=> #var.as_str())
            }
            Type::SliceRef(slice) => {
                requires_unsafe = true;
                let inner = &slice.inner;
                match slice.mutable {
                    false => quote_spanned!(span=> #var.as_slice::<#inner>()),
                    true => quote_spanned!(span=> #var.as_mut_slice::<#inner>()),
                }
            }
            ty if types.needs_indirect_abi(ty) => {
                requires_unsafe = true;
                quote_spanned!(span=> ::cxx::core::ptr::read(#var))
            }
            _ => quote!(#var),
        }
    });
    let vars: Vec<_> = receiver_var.into_iter().chain(arg_vars).collect();

    let mut requires_closure;
    let mut call = match invoke {
        Some(_) => {
            requires_closure = false;
            quote!(#local_name)
        }
        None => {
            requires_closure = true;
            requires_unsafe = true;
            quote!(::cxx::core::mem::transmute::<*const (), #sig>(__extern))
        }
    };
    requires_closure |= !vars.is_empty();
    call.extend(quote! { (#(#vars),*) });

    let wrap_super = invoke.map(|invoke| {
        // If the wrapper function is being passed directly to prevent_unwind,
        // it must implement `FnOnce() -> R` and cannot be an unsafe fn.
        let unsafety = sig.unsafety.filter(|_| requires_closure);
        expand_rust_function_shim_super(sig, &local_name, invoke, unsafety)
    });

    let span = body_span;
    let conversion = sig.ret.as_ref().and_then(|ret| match ret {
        Type::Ident(ident) if ident.rust == RustString => {
            Some(quote_spanned!(span=> ::cxx::private::RustString::from))
        }
        Type::RustBox(_) => Some(quote_spanned!(span=> ::cxx::alloc::boxed::Box::into_raw)),
        Type::RustVec(_) => Some(quote_spanned!(span=> ::cxx::private::RustVec::from)),
        Type::UniquePtr(_) => Some(quote_spanned!(span=> ::cxx::UniquePtr::into_raw)),
        Type::Ref(ty) => match &ty.inner {
            Type::Ident(ident) if ident.rust == RustString => match ty.mutable {
                false => Some(quote_spanned!(span=> ::cxx::private::RustString::from_ref)),
                true => Some(quote_spanned!(span=> ::cxx::private::RustString::from_mut)),
            },
            Type::RustVec(_) => match ty.mutable {
                false => Some(quote_spanned!(span=> ::cxx::private::RustVec::from_ref)),
                true => Some(quote_spanned!(span=> ::cxx::private::RustVec::from_mut)),
            },
            _ => None,
        },
        Type::Str(_) => Some(quote_spanned!(span=> ::cxx::private::RustStr::from)),
        Type::SliceRef(ty) => match ty.mutable {
            false => Some(quote_spanned!(span=> ::cxx::private::RustSlice::from_ref)),
            true => Some(quote_spanned!(span=> ::cxx::private::RustSlice::from_mut)),
        },
        _ => None,
    });

    let mut expr = match conversion {
        None => call,
        Some(conversion) if !sig.throws => {
            requires_closure = true;
            quote_spanned!(span=> #conversion(#call))
        }
        Some(conversion) => {
            requires_closure = true;
            quote_spanned!(span=> ::cxx::core::result::Result::map(#call, #conversion))
        }
    };

    let mut outparam = None;
    let indirect_return = indirect_return(sig, types, Lang::Rust);
    if indirect_return {
        let ret = expand_extern_type(sig.ret.as_ref().unwrap(), types, false);
        outparam = Some(quote_spanned!(span=> __return: *mut #ret,));
    }
    if sig.throws {
        let out = match sig.ret {
            Some(_) => quote_spanned!(span=> __return),
            None => quote_spanned!(span=> &mut ()),
        };
        requires_closure = true;
        requires_unsafe = true;
        expr = quote_spanned!(span=> ::cxx::private::r#try(#out, #expr));
    } else if indirect_return {
        requires_closure = true;
        requires_unsafe = true;
        expr = quote_spanned!(span=> ::cxx::core::ptr::write(__return, #expr));
    }

    if requires_unsafe {
        expr = quote_spanned!(span=> unsafe { #expr });
    }

    let closure = if requires_closure {
        quote_spanned!(span=> move || #expr)
    } else {
        quote!(#local_name)
    };

    expr = quote_spanned!(span=> ::cxx::private::prevent_unwind(__fn, #closure));

    let ret = if sig.throws {
        quote!(-> ::cxx::private::Result)
    } else {
        expand_extern_return_type(sig, types, false, Lang::Rust)
    };

    let pointer = match invoke {
        None => Some(quote_spanned!(span=> __extern: *const ())),
        Some(_) => None,
    };

    quote_spanned! {span=>
        #all_attrs
        #[doc(hidden)]
        #[unsafe(export_name = #link_name)]
        unsafe extern "C" fn #local_name #generics(#(#all_args,)* #outparam #pointer) #ret {
            let __fn = ::cxx::core::concat!(::cxx::core::module_path!(), #prevent_unwind_label);
            #wrap_super
            #expr
        }
    }
}

// A wrapper like `fn f(x: Arg) { super::f(x) }` just to ensure we have the
// accurate unsafety declaration and no problematic elided lifetimes.
fn expand_rust_function_shim_super(
    sig: &Signature,
    local_name: &Ident,
    invoke: &Ident,
    unsafety: Option<Token![unsafe]>,
) -> TokenStream {
    let generics = &sig.generics;

    let receiver_var = sig
        .receiver()
        .map(|receiver| Ident::new("__self", receiver.var.span));
    let receiver = sig.receiver().into_iter().map(|receiver| {
        let receiver_type = receiver.ty();
        quote!(#receiver_var: #receiver_type)
    });
    let args = sig.args.iter().map(|arg| quote!(#arg));
    let all_args = receiver.chain(args);

    let ret = if let Some((result, _langle, rangle)) = sig.throws_tokens {
        let ok = match &sig.ret {
            Some(ret) => quote!(#ret),
            None => quote!(()),
        };
        // Set spans that result in the `Result<...>` written by the user being
        // highlighted as the cause if their error type has no Display impl.
        let result_begin = quote_spanned!(result.span=> ::cxx::core::result::Result<#ok, impl);
        let result_end = quote_spanned!(rangle.span=> ::cxx::core::fmt::Display + use<>>);
        quote!(-> #result_begin #result_end)
    } else {
        expand_return_type(&sig.ret)
    };

    let arg_vars = sig.args.iter().map(|arg| &arg.name.rust);
    let vars = receiver_var.iter().chain(arg_vars);

    let span = invoke.span();
    let call = match sig.self_type() {
        None => quote_spanned!(span=> super::#invoke),
        Some(self_type) => quote_spanned!(span=> #self_type::#invoke),
    };

    let mut body = quote_spanned!(span=> #call(#(#vars,)*));
    let mut allow_unused_unsafe = None;
    if sig.unsafety.is_some() {
        body = quote_spanned!(span=> unsafe { #body });
        allow_unused_unsafe = Some(quote_spanned!(span=> #[allow(unused_unsafe)]));
    }

    quote_spanned! {span=>
        #allow_unused_unsafe
        #unsafety fn #local_name #generics(#(#all_args,)*) #ret {
            #body
        }
    }
}

fn expand_type_alias(alias: &TypeAlias) -> TokenStream {
    let doc = &alias.doc;
    let all_attrs = alias.attrs.all();
    let visibility = alias.visibility;
    let type_token = alias.type_token;
    let ident = &alias.name.rust;
    let generics = &alias.generics;
    let eq_token = alias.eq_token;
    let ty = &alias.ty;
    let semi_token = alias.semi_token;

    quote! {
        #doc
        #all_attrs
        #visibility #type_token #ident #generics #eq_token #ty #semi_token
    }
}

fn expand_type_alias_verify(alias: &TypeAlias, types: &Types) -> TokenStream {
    let cfg_and_lint_attrs = alias.attrs.cfg_and_lint();
    let ident = &alias.name.rust;
    let type_id = type_id(&alias.name);
    let begin_span = alias.type_token.span;
    let end_span = alias.semi_token.span;
    let begin = quote_spanned!(begin_span=> ::cxx::private::verify_extern_type::<);
    let end = quote_spanned!(end_span=> >);

    let resolve = types.resolve(ident);
    let lifetimes = resolve.generics.to_underscore_lifetimes();

    let mut verify = quote! {
        #cfg_and_lint_attrs
        const _: fn() = #begin #ident #lifetimes, #type_id #end;
    };

    let mut require_unpin = false;
    let mut require_box = false;
    let mut require_vec = false;
    let mut require_extern_type_trivial = false;
    let mut require_rust_type_or_trivial = None;
    if let Some(reasons) = types.required_trivial.get(&alias.name.rust) {
        for reason in reasons {
            match reason {
                TrivialReason::BoxTarget { local: true }
                | TrivialReason::VecElement { local: true } => require_unpin = true,
                TrivialReason::BoxTarget { local: false } => require_box = true,
                TrivialReason::VecElement { local: false } => require_vec = true,
                TrivialReason::StructField(_)
                | TrivialReason::FunctionArgument(_)
                | TrivialReason::FunctionReturn(_) => require_extern_type_trivial = true,
                TrivialReason::SliceElement(slice) => require_rust_type_or_trivial = Some(slice),
            }
        }
    }

    'unpin: {
        if let Some(reason) = types.required_unpin.get(ident) {
            let ampersand;
            let reference_lifetime;
            let mutability;
            let mut inner;
            let generics;
            let shorthand;
            match reason {
                UnpinReason::Receiver(receiver) => {
                    ampersand = &receiver.ampersand;
                    reference_lifetime = &receiver.lifetime;
                    mutability = &receiver.mutability;
                    inner = receiver.ty.rust.clone();
                    generics = &receiver.ty.generics;
                    shorthand = receiver.shorthand;
                    if receiver.shorthand {
                        inner.set_span(receiver.var.span);
                    }
                }
                UnpinReason::Ref(mutable_reference) => {
                    ampersand = &mutable_reference.ampersand;
                    reference_lifetime = &mutable_reference.lifetime;
                    mutability = &mutable_reference.mutability;
                    let Type::Ident(inner_type) = &mutable_reference.inner else {
                        unreachable!();
                    };
                    inner = inner_type.rust.clone();
                    generics = &inner_type.generics;
                    shorthand = false;
                }
                UnpinReason::Slice(mutable_slice) => {
                    ampersand = &mutable_slice.ampersand;
                    mutability = &mutable_slice.mutability;
                    let inner = quote_spanned!(mutable_slice.bracket.span=> [#ident #lifetimes]);
                    let trait_name = format_ident!("SliceOfUnpin_{ident}");
                    let label = format!("requires `{ident}: Unpin`");
                    verify.extend(quote! {
                        #cfg_and_lint_attrs
                        let _ = {
                            #[diagnostic::on_unimplemented(
                                message = "mutable slice of pinned type is not supported",
                                label = #label,
                            )]
                            trait #trait_name {
                                fn check_unpin() {}
                            }
                            #[diagnostic::do_not_recommend]
                            impl<'a, T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin> #trait_name for &'a #mutability T {}
                            <#ampersand #mutability #inner as #trait_name>::check_unpin
                        };
                    });
                    require_unpin = false;
                    break 'unpin;
                }
            }
            let trait_name = format_ident!("ReferenceToUnpin_{ident}");
            let message =
                format!("mutable reference to C++ type requires a pin -- use Pin<&mut {ident}>");
            let label = {
                let mut label = Message::new();
                write!(label, "use `");
                if shorthand {
                    write!(label, "self: ");
                }
                write!(label, "Pin<&");
                if let Some(reference_lifetime) = reference_lifetime {
                    write!(label, "{reference_lifetime} ");
                }
                write!(label, "mut {ident}");
                if !generics.lifetimes.is_empty() {
                    write!(label, "<");
                    for (i, lifetime) in generics.lifetimes.iter().enumerate() {
                        if i > 0 {
                            write!(label, ", ");
                        }
                        write!(label, "{lifetime}");
                    }
                    write!(label, ">");
                } else if shorthand && !alias.generics.lifetimes.is_empty() {
                    write!(label, "<");
                    for i in 0..alias.generics.lifetimes.len() {
                        if i > 0 {
                            write!(label, ", ");
                        }
                        write!(label, "'_");
                    }
                    write!(label, ">");
                }
                write!(label, ">`");
                label
            };
            let lifetimes = generics.to_underscore_lifetimes();
            verify.extend(quote! {
                #cfg_and_lint_attrs
                let _ = {
                    #[diagnostic::on_unimplemented(message = #message, label = #label)]
                    trait #trait_name {
                        fn check_unpin() {}
                    }
                    #[diagnostic::do_not_recommend]
                    impl<'a, T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin> #trait_name for &'a mut T {}
                    <#ampersand #mutability #inner #lifetimes as #trait_name>::check_unpin
                };
            });
            require_unpin = false;
        }
    }

    if require_unpin {
        verify.extend(quote! {
            #cfg_and_lint_attrs
            const _: fn() = ::cxx::private::require_unpin::<#ident #lifetimes>;
        });
    }

    if require_box {
        verify.extend(quote! {
            #cfg_and_lint_attrs
            const _: fn() = ::cxx::private::require_box::<#ident #lifetimes>;
        });
    }

    if require_vec {
        verify.extend(quote! {
            #cfg_and_lint_attrs
            const _: fn() = ::cxx::private::require_vec::<#ident #lifetimes>;
        });
    }

    if require_extern_type_trivial {
        let begin = quote_spanned!(begin_span=> ::cxx::private::verify_extern_kind::<);
        verify.extend(quote! {
            #cfg_and_lint_attrs
            const _: fn() = #begin #ident #lifetimes, ::cxx::kind::Trivial #end;
        });
    } else if let Some(slice_type) = require_rust_type_or_trivial {
        let ampersand = &slice_type.ampersand;
        let mutability = &slice_type.mutability;
        let inner = quote_spanned!(slice_type.bracket.span.join()=> [#ident #lifetimes]);
        verify.extend(quote! {
            #cfg_and_lint_attrs
            let _ = || ::cxx::private::with::<#ident #lifetimes>().check_slice::<#ampersand #mutability #inner>();
        });
    }

    verify
}

fn type_id(name: &Pair) -> TokenStream {
    let namespace_segments = name.namespace.iter();
    let mut segments = Vec::with_capacity(namespace_segments.len() + 1);
    segments.extend(namespace_segments.cloned());
    segments.push(Ident::new(&name.cxx.to_string(), Span::call_site()));
    let qualified = QualifiedName { segments };
    crate::type_id::expand(Crate::Cxx, qualified)
}

fn expand_rust_box(
    key: &NamedImplKey,
    types: &Types,
    conditional_impl: &ConditionalImpl,
) -> TokenStream {
    let ident = key.rust;
    let resolve = types.resolve(ident);
    let link_prefix = format!("cxxbridge1$box${}$", resolve.name.to_symbol());
    let link_alloc = format!("{}alloc", link_prefix);
    let link_dealloc = format!("{}dealloc", link_prefix);
    let link_drop = format!("{}drop", link_prefix);

    let local_prefix = format_ident!("{}__box_", ident);
    let local_alloc = format_ident!("{}alloc", local_prefix);
    let local_dealloc = format_ident!("{}dealloc", local_prefix);
    let local_drop = format_ident!("{}drop", local_prefix);

    let (impl_generics, ty_generics) = generics::split_for_impl(key, conditional_impl, resolve);

    let cfg = conditional_impl.cfg.into_attr();
    let begin_span = conditional_impl
        .explicit_impl
        .map_or(key.begin_span, |explicit| explicit.impl_token.span);
    let end_span = conditional_impl
        .explicit_impl
        .map_or(key.end_span, |explicit| explicit.brace_token.span.join());
    let unsafe_token = format_ident!("unsafe", span = begin_span);
    let prevent_unwind_drop_label = format!("::{} as Drop>::drop", ident);

    quote_spanned! {end_span=>
        #cfg
        #[automatically_derived]
        #[doc(hidden)]
        #unsafe_token impl #impl_generics ::cxx::private::ImplBox for #ident #ty_generics {}

        #cfg
        #[doc(hidden)]
        #[unsafe(export_name = #link_alloc)]
        unsafe extern "C" fn #local_alloc #impl_generics() -> *mut ::cxx::core::mem::MaybeUninit<#ident #ty_generics> {
            // No prevent_unwind: the global allocator is not allowed to panic.
            //
            // TODO: replace with Box::new_uninit when stable.
            // https://doc.rust-lang.org/std/boxed/struct.Box.html#method.new_uninit
            // https://github.com/rust-lang/rust/issues/63291
            ::cxx::alloc::boxed::Box::into_raw(::cxx::alloc::boxed::Box::new(::cxx::core::mem::MaybeUninit::uninit()))
        }

        #cfg
        #[doc(hidden)]
        #[unsafe(export_name = #link_dealloc)]
        unsafe extern "C" fn #local_dealloc #impl_generics(ptr: *mut ::cxx::core::mem::MaybeUninit<#ident #ty_generics>) {
            // No prevent_unwind: the global allocator is not allowed to panic.
            let _ = unsafe { ::cxx::alloc::boxed::Box::from_raw(ptr) };
        }

        #cfg
        #[doc(hidden)]
        #[unsafe(export_name = #link_drop)]
        unsafe extern "C" fn #local_drop #impl_generics(this: *mut ::cxx::alloc::boxed::Box<#ident #ty_generics>) {
            let __fn = ::cxx::core::concat!("<", ::cxx::core::module_path!(), #prevent_unwind_drop_label);
            ::cxx::private::prevent_unwind(__fn, || unsafe { ::cxx::core::ptr::drop_in_place(this) });
        }
    }
}

fn expand_rust_vec(
    key: &NamedImplKey,
    types: &Types,
    conditional_impl: &ConditionalImpl,
) -> TokenStream {
    let elem = key.rust;
    let resolve = types.resolve(elem);
    let link_prefix = format!("cxxbridge1$rust_vec${}$", resolve.name.to_symbol());
    let link_new = format!("{}new", link_prefix);
    let link_drop = format!("{}drop", link_prefix);
    let link_len = format!("{}len", link_prefix);
    let link_capacity = format!("{}capacity", link_prefix);
    let link_data = format!("{}data", link_prefix);
    let link_reserve_total = format!("{}reserve_total", link_prefix);
    let link_set_len = format!("{}set_len", link_prefix);
    let link_truncate = format!("{}truncate", link_prefix);

    let local_prefix = format_ident!("{}__vec_", elem);
    let local_new = format_ident!("{}new", local_prefix);
    let local_drop = format_ident!("{}drop", local_prefix);
    let local_len = format_ident!("{}len", local_prefix);
    let local_capacity = format_ident!("{}capacity", local_prefix);
    let local_data = format_ident!("{}data", local_prefix);
    let local_reserve_total = format_ident!("{}reserve_total", local_prefix);
    let local_set_len = format_ident!("{}set_len", local_prefix);
    let local_truncate = format_ident!("{}truncate", local_prefix);

    let (impl_generics, ty_generics) = generics::split_for_impl(key, conditional_impl, resolve);

    let cfg = conditional_impl.cfg.into_attr();
    let begin_span = conditional_impl
        .explicit_impl
        .map_or(key.begin_span, |explicit| explicit.impl_token.span);
    let end_span = conditional_impl
        .explicit_impl
        .map_or(key.end_span, |explicit| explicit.brace_token.span.join());
    let unsafe_token = format_ident!("unsafe", span = begin_span);
    let prevent_unwind_drop_label = format!("::{} as Drop>::drop", elem);

    quote_spanned! {end_span=>
        #cfg
        #[automatically_derived]
        #[doc(hidden)]
        #unsafe_token impl #impl_generics ::cxx::private::ImplVec for #elem #ty_generics {}

        #cfg
        #[doc(hidden)]
        #[unsafe(export_name = #link_new)]
        unsafe extern "C" fn #local_new #impl_generics(this: *mut ::cxx::private::RustVec<#elem #ty_generics>) {
            // No prevent_unwind: cannot panic.
            unsafe {
                ::cxx::core::ptr::write(this, ::cxx::private::RustVec::new());
            }
        }

        #cfg
        #[doc(hidden)]
        #[unsafe(export_name = #link_drop)]
        unsafe extern "C" fn #local_drop #impl_generics(this: *mut ::cxx::private::RustVec<#elem #ty_generics>) {
            let __fn = ::cxx::core::concat!("<", ::cxx::core::module_path!(), #prevent_unwind_drop_label);
            ::cxx::private::prevent_unwind(
                __fn,
                || unsafe { ::cxx::core::ptr::drop_in_place(this) },
            );
        }

        #cfg
        #[doc(hidden)]
        #[unsafe(export_name = #link_len)]
        unsafe extern "C" fn #local_len #impl_generics(this: *const ::cxx::private::RustVec<#elem #ty_generics>) -> ::cxx::core::primitive::usize {
            // No prevent_unwind: cannot panic.
            unsafe { (*this).len() }
        }

        #cfg
        #[doc(hidden)]
        #[unsafe(export_name = #link_capacity)]
        unsafe extern "C" fn #local_capacity #impl_generics(this: *const ::cxx::private::RustVec<#elem #ty_generics>) -> ::cxx::core::primitive::usize {
            // No prevent_unwind: cannot panic.
            unsafe { (*this).capacity() }
        }

        #cfg
        #[doc(hidden)]
        #[unsafe(export_name = #link_data)]
        unsafe extern "C" fn #local_data #impl_generics(this: *const ::cxx::private::RustVec<#elem #ty_generics>) -> *const #elem #ty_generics {
            // No prevent_unwind: cannot panic.
            unsafe { (*this).as_ptr() }
        }

        #cfg
        #[doc(hidden)]
        #[unsafe(export_name = #link_reserve_total)]
        unsafe extern "C" fn #local_reserve_total #impl_generics(this: *mut ::cxx::private::RustVec<#elem #ty_generics>, new_cap: ::cxx::core::primitive::usize) {
            // No prevent_unwind: the global allocator is not allowed to panic.
            unsafe {
                (*this).reserve_total(new_cap);
            }
        }

        #cfg
        #[doc(hidden)]
        #[unsafe(export_name = #link_set_len)]
        unsafe extern "C" fn #local_set_len #impl_generics(this: *mut ::cxx::private::RustVec<#elem #ty_generics>, len: ::cxx::core::primitive::usize) {
            // No prevent_unwind: cannot panic.
            unsafe {
                (*this).set_len(len);
            }
        }

        #cfg
        #[doc(hidden)]
        #[unsafe(export_name = #link_truncate)]
        unsafe extern "C" fn #local_truncate #impl_generics(this: *mut ::cxx::private::RustVec<#elem #ty_generics>, len: ::cxx::core::primitive::usize) {
            let __fn = ::cxx::core::concat!("<", ::cxx::core::module_path!(), #prevent_unwind_drop_label);
            ::cxx::private::prevent_unwind(
                __fn,
                || unsafe { (*this).truncate(len) },
            );
        }
    }
}

fn expand_unique_ptr(
    key: &NamedImplKey,
    types: &Types,
    conditional_impl: &ConditionalImpl,
) -> TokenStream {
    let ident = key.rust;
    let name = ident.to_string();
    let resolve = types.resolve(ident);
    let prefix = format!("cxxbridge1$unique_ptr${}$", resolve.name.to_symbol());
    let link_null = format!("{}null", prefix);
    let link_uninit = format!("{}uninit", prefix);
    let link_raw = format!("{}raw", prefix);
    let link_get = format!("{}get", prefix);
    let link_release = format!("{}release", prefix);
    let link_drop = format!("{}drop", prefix);

    let (impl_generics, ty_generics) = generics::split_for_impl(key, conditional_impl, resolve);

    let can_construct_from_value = types.is_maybe_trivial(ident);
    let new_method = if can_construct_from_value {
        Some(quote! {
            fn __new(value: Self) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                unsafe extern "C" {
                    #[link_name = #link_uninit]
                    fn __uninit(this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>) -> *mut ::cxx::core::ffi::c_void;
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe {
                    __uninit(&raw mut repr).cast::<#ident #ty_generics>().write(value);
                }
                repr
            }
        })
    } else {
        None
    };

    let cfg = conditional_impl.cfg.into_attr();
    let begin_span = conditional_impl
        .explicit_impl
        .map_or(key.begin_span, |explicit| explicit.impl_token.span);
    let end_span = conditional_impl
        .explicit_impl
        .map_or(key.end_span, |explicit| explicit.brace_token.span.join());
    let unsafe_token = format_ident!("unsafe", span = begin_span);

    quote_spanned! {end_span=>
        #cfg
        #[automatically_derived]
        #unsafe_token impl #impl_generics ::cxx::memory::UniquePtrTarget for #ident #ty_generics {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str(#name)
            }
            fn __null() -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                unsafe extern "C" {
                    #[link_name = #link_null]
                    fn __null(this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>);
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe {
                    __null(&raw mut repr);
                }
                repr
            }
            #new_method
            unsafe fn __raw(raw: *mut Self) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                unsafe extern "C" {
                    #[link_name = #link_raw]
                    fn __raw(this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>, raw: *mut ::cxx::core::ffi::c_void);
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe {
                    __raw(&raw mut repr, raw.cast());
                }
                repr
            }
            unsafe fn __get(repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>) -> *const Self {
                unsafe extern "C" {
                    #[link_name = #link_get]
                    fn __get(this: *const ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>) -> *const ::cxx::core::ffi::c_void;
                }
                unsafe { __get(&raw const repr).cast() }
            }
            unsafe fn __release(mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>) -> *mut Self {
                unsafe extern "C" {
                    #[link_name = #link_release]
                    fn __release(this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>) -> *mut ::cxx::core::ffi::c_void;
                }
                unsafe { __release(&raw mut repr).cast() }
            }
            unsafe fn __drop(mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>) {
                unsafe extern "C" {
                    #[link_name = #link_drop]
                    fn __drop(this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>);
                }
                unsafe {
                    __drop(&raw mut repr);
                }
            }
        }
    }
}

fn expand_shared_ptr(
    key: &NamedImplKey,
    types: &Types,
    conditional_impl: &ConditionalImpl,
) -> TokenStream {
    let ident = key.rust;
    let name = ident.to_string();
    let resolve = types.resolve(ident);
    let prefix = format!("cxxbridge1$shared_ptr${}$", resolve.name.to_symbol());
    let link_null = format!("{}null", prefix);
    let link_uninit = format!("{}uninit", prefix);
    let link_raw = format!("{}raw", prefix);
    let link_clone = format!("{}clone", prefix);
    let link_get = format!("{}get", prefix);
    let link_drop = format!("{}drop", prefix);

    let (impl_generics, ty_generics) = generics::split_for_impl(key, conditional_impl, resolve);

    let can_construct_from_value = types.is_maybe_trivial(ident);
    let new_method = if can_construct_from_value {
        Some(quote! {
            unsafe fn __new(value: Self, new: *mut ::cxx::core::ffi::c_void) {
                unsafe extern "C" {
                    #[link_name = #link_uninit]
                    fn __uninit(new: *mut ::cxx::core::ffi::c_void) -> *mut ::cxx::core::ffi::c_void;
                }
                unsafe {
                    __uninit(new).cast::<#ident #ty_generics>().write(value);
                }
            }
        })
    } else {
        None
    };

    let cfg = conditional_impl.cfg.into_attr();
    let begin_span = conditional_impl
        .explicit_impl
        .map_or(key.begin_span, |explicit| explicit.impl_token.span);
    let end_span = conditional_impl
        .explicit_impl
        .map_or(key.end_span, |explicit| explicit.brace_token.span.join());
    let unsafe_token = format_ident!("unsafe", span = begin_span);
    let not_destructible_err = format!("{} is not destructible", display_namespaced(resolve.name));

    quote_spanned! {end_span=>
        #cfg
        #[automatically_derived]
        #unsafe_token impl #impl_generics ::cxx::memory::SharedPtrTarget for #ident #ty_generics {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str(#name)
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                unsafe extern "C" {
                    #[link_name = #link_null]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                unsafe {
                    __null(new);
                }
            }
            #new_method
            #[track_caller]
            unsafe fn __raw(new: *mut ::cxx::core::ffi::c_void, raw: *mut Self) {
                unsafe extern "C" {
                    #[link_name = #link_raw]
                    fn __raw(new: *const ::cxx::core::ffi::c_void, raw: *mut ::cxx::core::ffi::c_void) -> ::cxx::core::primitive::bool;
                }
                if !unsafe { __raw(new, raw as *mut ::cxx::core::ffi::c_void) } {
                    ::cxx::core::panic!(#not_destructible_err);
                }
            }
            unsafe fn __clone(this: *const ::cxx::core::ffi::c_void, new: *mut ::cxx::core::ffi::c_void) {
                unsafe extern "C" {
                    #[link_name = #link_clone]
                    fn __clone(this: *const ::cxx::core::ffi::c_void, new: *mut ::cxx::core::ffi::c_void);
                }
                unsafe {
                    __clone(this, new);
                }
            }
            unsafe fn __get(this: *const ::cxx::core::ffi::c_void) -> *const Self {
                unsafe extern "C" {
                    #[link_name = #link_get]
                    fn __get(this: *const ::cxx::core::ffi::c_void) -> *const ::cxx::core::ffi::c_void;
                }
                unsafe { __get(this).cast() }
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                unsafe extern "C" {
                    #[link_name = #link_drop]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                unsafe {
                    __drop(this);
                }
            }
        }
    }
}

fn expand_weak_ptr(
    key: &NamedImplKey,
    types: &Types,
    conditional_impl: &ConditionalImpl,
) -> TokenStream {
    let ident = key.rust;
    let name = ident.to_string();
    let resolve = types.resolve(ident);
    let prefix = format!("cxxbridge1$weak_ptr${}$", resolve.name.to_symbol());
    let link_null = format!("{}null", prefix);
    let link_clone = format!("{}clone", prefix);
    let link_downgrade = format!("{}downgrade", prefix);
    let link_upgrade = format!("{}upgrade", prefix);
    let link_drop = format!("{}drop", prefix);

    let (impl_generics, ty_generics) = generics::split_for_impl(key, conditional_impl, resolve);

    let cfg = conditional_impl.cfg.into_attr();
    let begin_span = conditional_impl
        .explicit_impl
        .map_or(key.begin_span, |explicit| explicit.impl_token.span);
    let end_span = conditional_impl
        .explicit_impl
        .map_or(key.end_span, |explicit| explicit.brace_token.span.join());
    let unsafe_token = format_ident!("unsafe", span = begin_span);

    quote_spanned! {end_span=>
        #cfg
        #[automatically_derived]
        #unsafe_token impl #impl_generics ::cxx::memory::WeakPtrTarget for #ident #ty_generics {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str(#name)
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                unsafe extern "C" {
                    #[link_name = #link_null]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                unsafe {
                    __null(new);
                }
            }
            unsafe fn __clone(this: *const ::cxx::core::ffi::c_void, new: *mut ::cxx::core::ffi::c_void) {
                unsafe extern "C" {
                    #[link_name = #link_clone]
                    fn __clone(this: *const ::cxx::core::ffi::c_void, new: *mut ::cxx::core::ffi::c_void);
                }
                unsafe {
                    __clone(this, new);
                }
            }
            unsafe fn __downgrade(shared: *const ::cxx::core::ffi::c_void, weak: *mut ::cxx::core::ffi::c_void) {
                unsafe extern "C" {
                    #[link_name = #link_downgrade]
                    fn __downgrade(shared: *const ::cxx::core::ffi::c_void, weak: *mut ::cxx::core::ffi::c_void);
                }
                unsafe {
                    __downgrade(shared, weak);
                }
            }
            unsafe fn __upgrade(weak: *const ::cxx::core::ffi::c_void, shared: *mut ::cxx::core::ffi::c_void) {
                unsafe extern "C" {
                    #[link_name = #link_upgrade]
                    fn __upgrade(weak: *const ::cxx::core::ffi::c_void, shared: *mut ::cxx::core::ffi::c_void);
                }
                unsafe {
                    __upgrade(weak, shared);
                }
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                unsafe extern "C" {
                    #[link_name = #link_drop]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                unsafe {
                    __drop(this);
                }
            }
        }
    }
}

fn expand_cxx_vector(
    key: &NamedImplKey,
    conditional_impl: &ConditionalImpl,
    types: &Types,
) -> TokenStream {
    let elem = key.rust;
    let name = elem.to_string();
    let resolve = types.resolve(elem);
    let prefix = format!("cxxbridge1$std$vector${}$", resolve.name.to_symbol());
    let link_new = format!("{}new", prefix);
    let link_size = format!("{}size", prefix);
    let link_capacity = format!("{}capacity", prefix);
    let link_get_unchecked = format!("{}get_unchecked", prefix);
    let link_reserve = format!("{}reserve", prefix);
    let link_push_back = format!("{}push_back", prefix);
    let link_pop_back = format!("{}pop_back", prefix);
    let unique_ptr_prefix = format!(
        "cxxbridge1$unique_ptr$std$vector${}$",
        resolve.name.to_symbol(),
    );
    let link_unique_ptr_null = format!("{}null", unique_ptr_prefix);
    let link_unique_ptr_raw = format!("{}raw", unique_ptr_prefix);
    let link_unique_ptr_get = format!("{}get", unique_ptr_prefix);
    let link_unique_ptr_release = format!("{}release", unique_ptr_prefix);
    let link_unique_ptr_drop = format!("{}drop", unique_ptr_prefix);

    let (impl_generics, ty_generics) = generics::split_for_impl(key, conditional_impl, resolve);

    let cfg = conditional_impl.cfg.into_attr();
    let begin_span = conditional_impl
        .explicit_impl
        .map_or(key.begin_span, |explicit| explicit.impl_token.span);
    let end_span = conditional_impl
        .explicit_impl
        .map_or(key.end_span, |explicit| explicit.brace_token.span.join());
    let unsafe_token = format_ident!("unsafe", span = begin_span);

    let can_pass_element_by_value = types.is_maybe_trivial(elem);
    let by_value_methods = if can_pass_element_by_value {
        Some(quote_spanned! {end_span=>
            unsafe fn __push_back(
                this: ::cxx::core::pin::Pin<&mut ::cxx::CxxVector<Self>>,
                value: &mut ::cxx::core::mem::ManuallyDrop<Self>,
            ) {
                unsafe extern "C" {
                    #[link_name = #link_push_back]
                    fn __push_back #impl_generics(
                        this: ::cxx::core::pin::Pin<&mut ::cxx::CxxVector<#elem #ty_generics>>,
                        value: *mut ::cxx::core::ffi::c_void,
                    );
                }
                unsafe {
                    __push_back(
                        this,
                        value as *mut ::cxx::core::mem::ManuallyDrop<Self> as *mut ::cxx::core::ffi::c_void,
                    );
                }
            }
            unsafe fn __pop_back(
                this: ::cxx::core::pin::Pin<&mut ::cxx::CxxVector<Self>>,
                out: &mut ::cxx::core::mem::MaybeUninit<Self>,
            ) {
                unsafe extern "C" {
                    #[link_name = #link_pop_back]
                    fn __pop_back #impl_generics(
                        this: ::cxx::core::pin::Pin<&mut ::cxx::CxxVector<#elem #ty_generics>>,
                        out: *mut ::cxx::core::ffi::c_void,
                    );
                }
                unsafe {
                    __pop_back(
                        this,
                        out as *mut ::cxx::core::mem::MaybeUninit<Self> as *mut ::cxx::core::ffi::c_void,
                    );
                }
            }
        })
    } else {
        None
    };

    let not_move_constructible_err = format!(
        "{} is not move constructible",
        display_namespaced(resolve.name),
    );

    quote_spanned! {end_span=>
        #cfg
        #[automatically_derived]
        #unsafe_token impl #impl_generics ::cxx::vector::VectorElement for #elem #ty_generics {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str(#name)
            }
            fn __vector_new() -> *mut ::cxx::CxxVector<Self> {
                unsafe extern "C" {
                    #[link_name = #link_new]
                    fn __vector_new #impl_generics() -> *mut ::cxx::CxxVector<#elem #ty_generics>;
                }
                unsafe { __vector_new() }
            }
            fn __vector_size(v: &::cxx::CxxVector<Self>) -> ::cxx::core::primitive::usize {
                unsafe extern "C" {
                    #[link_name = #link_size]
                    fn __vector_size #impl_generics(_: &::cxx::CxxVector<#elem #ty_generics>) -> ::cxx::core::primitive::usize;
                }
                unsafe { __vector_size(v) }
            }
            fn __vector_capacity(v: &::cxx::CxxVector<Self>) -> ::cxx::core::primitive::usize {
                unsafe extern "C" {
                    #[link_name = #link_capacity]
                    fn __vector_capacity #impl_generics(_: &::cxx::CxxVector<#elem #ty_generics>) -> ::cxx::core::primitive::usize;
                }
                unsafe { __vector_capacity(v) }
            }
            unsafe fn __get_unchecked(v: *mut ::cxx::CxxVector<Self>, pos: ::cxx::core::primitive::usize) -> *mut Self {
                unsafe extern "C" {
                    #[link_name = #link_get_unchecked]
                    fn __get_unchecked #impl_generics(
                        v: *mut ::cxx::CxxVector<#elem #ty_generics>,
                        pos: ::cxx::core::primitive::usize,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                unsafe { __get_unchecked(v, pos) as *mut Self }
            }
            unsafe fn __reserve(v: ::cxx::core::pin::Pin<&mut ::cxx::CxxVector<Self>>, new_cap: ::cxx::core::primitive::usize) {
                unsafe extern "C" {
                    #[link_name = #link_reserve]
                    fn __reserve #impl_generics(
                        v: ::cxx::core::pin::Pin<&mut ::cxx::CxxVector<#elem #ty_generics>>,
                        new_cap: ::cxx::core::primitive::usize,
                    ) -> ::cxx::core::primitive::bool;
                }
                if !unsafe { __reserve(v, new_cap) } {
                    ::cxx::core::panic!(#not_move_constructible_err);
                }
            }
            #by_value_methods
            fn __unique_ptr_null() -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                unsafe extern "C" {
                    #[link_name = #link_unique_ptr_null]
                    fn __unique_ptr_null(this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>);
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe {
                    __unique_ptr_null(&raw mut repr);
                }
                repr
            }
            unsafe fn __unique_ptr_raw(raw: *mut ::cxx::CxxVector<Self>) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                unsafe extern "C" {
                    #[link_name = #link_unique_ptr_raw]
                    fn __unique_ptr_raw #impl_generics(this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>, raw: *mut ::cxx::CxxVector<#elem #ty_generics>);
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe {
                    __unique_ptr_raw(&raw mut repr, raw);
                }
                repr
            }
            unsafe fn __unique_ptr_get(repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>) -> *const ::cxx::CxxVector<Self> {
                unsafe extern "C" {
                    #[link_name = #link_unique_ptr_get]
                    fn __unique_ptr_get #impl_generics(this: *const ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>) -> *const ::cxx::CxxVector<#elem #ty_generics>;
                }
                unsafe { __unique_ptr_get(&raw const repr) }
            }
            unsafe fn __unique_ptr_release(mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>) -> *mut ::cxx::CxxVector<Self> {
                unsafe extern "C" {
                    #[link_name = #link_unique_ptr_release]
                    fn __unique_ptr_release #impl_generics(this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>) -> *mut ::cxx::CxxVector<#elem #ty_generics>;
                }
                unsafe { __unique_ptr_release(&raw mut repr) }
            }
            unsafe fn __unique_ptr_drop(mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>) {
                unsafe extern "C" {
                    #[link_name = #link_unique_ptr_drop]
                    fn __unique_ptr_drop(this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>);
                }
                unsafe {
                    __unique_ptr_drop(&raw mut repr);
                }
            }
        }
    }
}

fn expand_return_type(ret: &Option<Type>) -> TokenStream {
    match ret {
        Some(ret) => quote!(-> #ret),
        None => TokenStream::new(),
    }
}

fn indirect_return(sig: &Signature, types: &Types, lang: Lang) -> bool {
    sig.ret.as_ref().is_some_and(|ret| {
        sig.throws
            || types.needs_indirect_abi(ret)
            || match lang {
                Lang::Cxx | Lang::CxxUnwind => types.contains_elided_lifetime(ret),
                Lang::Rust => false,
            }
    })
}

fn expand_extern_type(ty: &Type, types: &Types, proper: bool) -> TokenStream {
    match ty {
        Type::Ident(ident) if ident.rust == RustString => {
            let span = ident.rust.span();
            quote_spanned!(span=> ::cxx::private::RustString)
        }
        Type::RustBox(ty) | Type::UniquePtr(ty) => {
            let span = ty.name.span();
            if proper && types.is_considered_improper_ctype(&ty.inner) {
                quote_spanned!(span=> *mut ::cxx::core::ffi::c_void)
            } else {
                let inner = expand_extern_type(&ty.inner, types, proper);
                quote_spanned!(span=> *mut #inner)
            }
        }
        Type::RustVec(ty) => {
            // Replace Vec<Foo> with ::cxx::private::RustVec<Foo>. Both have the
            // same layout but only the latter has a predictable ABI. Note that
            // the overall size and alignment are independent of the element
            // type, but the field order inside of Vec may not be.
            let span = ty.name.span();
            let langle = ty.langle;
            let elem = &ty.inner;
            let rangle = ty.rangle;
            quote_spanned!(span=> ::cxx::private::RustVec #langle #elem #rangle)
        }
        Type::Ref(ty) => {
            let ampersand = ty.ampersand;
            let lifetime = &ty.lifetime;
            let mutability = ty.mutability;
            match &ty.inner {
                Type::Ident(ident) if ident.rust == RustString => {
                    let span = ident.rust.span();
                    quote_spanned!(span=> #ampersand #lifetime #mutability ::cxx::private::RustString)
                }
                Type::RustVec(ty) => {
                    let span = ty.name.span();
                    let langle = ty.langle;
                    let inner = &ty.inner;
                    let rangle = ty.rangle;
                    quote_spanned!(span=> #ampersand #lifetime #mutability ::cxx::private::RustVec #langle #inner #rangle)
                }
                inner if proper && types.is_considered_improper_ctype(inner) => {
                    let star = Token![*](ampersand.span);
                    match ty.mutable {
                        false => quote!(#star const ::cxx::core::ffi::c_void),
                        true => quote!(#star #mutability ::cxx::core::ffi::c_void),
                    }
                }
                _ => quote!(#ty),
            }
        }
        Type::Ptr(ty) => {
            if proper && types.is_considered_improper_ctype(&ty.inner) {
                let star = ty.star;
                let mutability = ty.mutability;
                let constness = ty.constness;
                quote!(#star #mutability #constness ::cxx::core::ffi::c_void)
            } else {
                quote!(#ty)
            }
        }
        Type::Str(ty) => {
            let span = ty.ampersand.span;
            let rust_str = Ident::new("RustStr", syn::spanned::Spanned::span(&ty.inner));
            quote_spanned!(span=> ::cxx::private::#rust_str)
        }
        Type::SliceRef(ty) => {
            let span = ty.ampersand.span;
            let rust_slice = Ident::new("RustSlice", ty.bracket.span.join());
            quote_spanned!(span=> ::cxx::private::#rust_slice)
        }
        _ => quote!(#ty),
    }
}

fn expand_extern_return_type(
    sig: &Signature,
    types: &Types,
    proper: bool,
    lang: Lang,
) -> TokenStream {
    let ret = match &sig.ret {
        Some(ret) if !indirect_return(sig, types, lang) => ret,
        _ => return TokenStream::new(),
    };
    let ty = expand_extern_type(ret, types, proper);
    quote!(-> #ty)
}

fn display_namespaced(name: &Pair) -> impl Display + '_ {
    struct Namespaced<'a>(&'a Pair);

    impl<'a> Display for Namespaced<'a> {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            for segment in &self.0.namespace {
                write!(formatter, "{segment}::")?;
            }
            write!(formatter, "{}", self.0.cxx)
        }
    }

    Namespaced(name)
}
