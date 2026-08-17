// Copyright 2019 The Fuchsia Authors
//
// Licensed under a BSD-style license <LICENSE-BSD>, Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0>, or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to
// those terms.

//! Derive macros for [zerocopy]'s traits.
//!
//! [zerocopy]: https://docs.rs/zerocopy

// Sometimes we want to use lints which were added after our MSRV.
// `unknown_lints` is `warn` by default and we deny warnings in CI, so without
// this attribute, any unknown lint would cause a CI failure when testing with
// our MSRV.
#![allow(unknown_lints)]
#![deny(renamed_and_removed_lints)]
#![deny(clippy::all, clippy::missing_safety_doc, clippy::undocumented_unsafe_blocks)]
// Inlining format args isn't supported on our MSRV.
#![allow(clippy::uninlined_format_args)]
#![deny(
    rustdoc::bare_urls,
    rustdoc::broken_intra_doc_links,
    rustdoc::invalid_codeblock_attributes,
    rustdoc::invalid_html_tags,
    rustdoc::invalid_rust_codeblocks,
    rustdoc::missing_crate_level_docs,
    rustdoc::private_intra_doc_links
)]
#![recursion_limit = "128"]

mod r#enum;
mod ext;
#[cfg(test)]
mod output_tests;
mod repr;

use proc_macro2::{Span, TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn::{
    parse_quote, Attribute, Data, DataEnum, DataStruct, DataUnion, DeriveInput, Error, Expr,
    ExprLit, ExprUnary, GenericParam, Ident, Lit, Meta, Path, Type, UnOp, WherePredicate,
};

use crate::{ext::*, repr::*};

// FIXME(https://github.com/rust-lang/rust/issues/54140): Some errors could be
// made better if we could add multiple lines of error output like this:
//
// error: unsupported representation
//   --> enum.rs:28:8
//    |
// 28 | #[repr(transparent)]
//    |
// help: required by the derive of FromBytes
//
// Instead, we have more verbose error messages like "unsupported representation
// for deriving FromZeros, FromBytes, IntoBytes, or Unaligned on an enum"
//
// This will probably require Span::error
// (https://doc.rust-lang.org/nightly/proc_macro/struct.Span.html#method.error),
// which is currently unstable. Revisit this once it's stable.

/// Defines a derive function named `$outer` which parses its input
/// `TokenStream` as a `DeriveInput` and then invokes the `$inner` function.
///
/// Note that the separate `$outer` parameter is required - proc macro functions
/// are currently required to live at the crate root, and so the caller must
/// specify the name in order to avoid name collisions.
macro_rules! derive {
    ($trait:ident => $outer:ident => $inner:ident) => {
        #[proc_macro_derive($trait, attributes(zerocopy))]
        pub fn $outer(ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
            let ast = syn::parse_macro_input!(ts as DeriveInput);
            let zerocopy_crate = match extract_zerocopy_crate(&ast.attrs) {
                Ok(zerocopy_crate) => zerocopy_crate,
                Err(e) => return e.into_compile_error().into(),
            };
            $inner(&ast, Trait::$trait, &zerocopy_crate).into_ts().into()
        }
    };
}

trait IntoTokenStream {
    fn into_ts(self) -> TokenStream;
}

impl IntoTokenStream for TokenStream {
    fn into_ts(self) -> TokenStream {
        self
    }
}

impl IntoTokenStream for Result<TokenStream, Error> {
    fn into_ts(self) -> TokenStream {
        match self {
            Ok(ts) => ts,
            Err(err) => err.to_compile_error(),
        }
    }
}

/// Attempt to extract a crate path from the provided attributes. Defaults to `::zerocopy` if not
/// found.
fn extract_zerocopy_crate(attrs: &[Attribute]) -> Result<Path, Error> {
    let mut path = parse_quote!(::zerocopy);

    for attr in attrs {
        if let Meta::List(ref meta_list) = attr.meta {
            if meta_list.path.is_ident("zerocopy") {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("crate") {
                        let expr = meta.value().and_then(|value| value.parse());
                        if let Ok(Expr::Lit(ExprLit { lit: Lit::Str(lit), .. })) = expr {
                            if let Ok(path_lit) = lit.parse() {
                                path = path_lit;
                                return Ok(());
                            }
                        }

                        return Err(Error::new(
                            Span::call_site(),
                            "`crate` attribute requires a path as the value",
                        ));
                    }

                    Err(Error::new(
                        Span::call_site(),
                        format!("unknown attribute encountered: {}", meta.path.into_token_stream()),
                    ))
                })?;
            }
        }
    }

    Ok(path)
}

derive!(KnownLayout => derive_known_layout => derive_known_layout_inner);
derive!(Immutable => derive_no_cell => derive_no_cell_inner);
derive!(TryFromBytes => derive_try_from_bytes => derive_try_from_bytes_inner);
derive!(FromZeros => derive_from_zeros => derive_from_zeros_inner);
derive!(FromBytes => derive_from_bytes => derive_from_bytes_inner);
derive!(IntoBytes => derive_into_bytes => derive_into_bytes_inner);
derive!(Unaligned => derive_unaligned => derive_unaligned_inner);
derive!(ByteHash => derive_hash => derive_hash_inner);
derive!(ByteEq => derive_eq => derive_eq_inner);
derive!(SplitAt => derive_split_at => derive_split_at_inner);

/// Deprecated: prefer [`FromZeros`] instead.
#[deprecated(since = "0.8.0", note = "`FromZeroes` was renamed to `FromZeros`")]
#[doc(hidden)]
#[proc_macro_derive(FromZeroes)]
pub fn derive_from_zeroes(ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_from_zeros(ts)
}

/// Deprecated: prefer [`IntoBytes`] instead.
#[deprecated(since = "0.8.0", note = "`AsBytes` was renamed to `IntoBytes`")]
#[doc(hidden)]
#[proc_macro_derive(AsBytes)]
pub fn derive_as_bytes(ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_into_bytes(ts)
}

fn derive_known_layout_inner(
    ast: &DeriveInput,
    _top_level: Trait,
    zerocopy_crate: &Path,
) -> Result<TokenStream, Error> {
    let is_repr_c_struct = match &ast.data {
        Data::Struct(..) => {
            let repr = StructUnionRepr::from_attrs(&ast.attrs)?;
            if repr.is_c() {
                Some(repr)
            } else {
                None
            }
        }
        Data::Enum(..) | Data::Union(..) => None,
    };

    let fields = ast.data.fields();

    let (self_bounds, inner_extras, outer_extras) = if let (
        Some(repr),
        Some((trailing_field, leading_fields)),
    ) = (is_repr_c_struct, fields.split_last())
    {
        let (_vis, trailing_field_name, trailing_field_ty) = trailing_field;
        let leading_fields_tys = leading_fields.iter().map(|(_vis, _name, ty)| ty);

        let core_path = quote!(#zerocopy_crate::util::macro_util::core_reexport);
        let repr_align = repr
            .get_align()
            .map(|align| {
                let align = align.t.get();
                quote!(#core_path::num::NonZeroUsize::new(#align as usize))
            })
            .unwrap_or_else(|| quote!(#core_path::option::Option::None));
        let repr_packed = repr
            .get_packed()
            .map(|packed| {
                let packed = packed.get();
                quote!(#core_path::num::NonZeroUsize::new(#packed as usize))
            })
            .unwrap_or_else(|| quote!(#core_path::option::Option::None));

        let make_methods = |trailing_field_ty| {
            quote! {
                // SAFETY:
                // - The returned pointer has the same address and provenance as
                //   `bytes`:
                //   - The recursive call to `raw_from_ptr_len` preserves both
                //     address and provenance.
                //   - The `as` cast preserves both address and provenance.
                //   - `NonNull::new_unchecked` preserves both address and
                //     provenance.
                // - If `Self` is a slice DST, the returned pointer encodes
                //   `elems` elements in the trailing slice:
                //   - This is true of the recursive call to `raw_from_ptr_len`.
                //   - `trailing.as_ptr() as *mut Self` preserves trailing slice
                //     element count [1].
                //   - `NonNull::new_unchecked` preserves trailing slice element
                //     count.
                //
                // [1] Per https://doc.rust-lang.org/reference/expressions/operator-expr.html#pointer-to-pointer-cast:
                //
                //   `*const T`` / `*mut T` can be cast to `*const U` / `*mut U`
                //   with the following behavior:
                //     ...
                //     - If `T` and `U` are both unsized, the pointer is also
                //       returned unchanged. In particular, the metadata is
                //       preserved exactly.
                //
                //       For instance, a cast from `*const [T]` to `*const [U]`
                //       preserves the number of elements. ... The same holds
                //       for str and any compound type whose unsized tail is a
                //       slice type, such as struct `Foo(i32, [u8])` or `(u64, Foo)`.
                #[inline(always)]
                fn raw_from_ptr_len(
                    bytes: #zerocopy_crate::util::macro_util::core_reexport::ptr::NonNull<u8>,
                    meta: Self::PointerMetadata,
                ) -> #zerocopy_crate::util::macro_util::core_reexport::ptr::NonNull<Self> {
                    use #zerocopy_crate::KnownLayout;
                    let trailing = <#trailing_field_ty as KnownLayout>::raw_from_ptr_len(bytes, meta);
                    let slf = trailing.as_ptr() as *mut Self;
                    // SAFETY: Constructed from `trailing`, which is non-null.
                    unsafe { #zerocopy_crate::util::macro_util::core_reexport::ptr::NonNull::new_unchecked(slf) }
                }

                #[inline(always)]
                fn pointer_to_metadata(ptr: *mut Self) -> Self::PointerMetadata {
                    <#trailing_field_ty>::pointer_to_metadata(ptr as *mut _)
                }
            }
        };

        let inner_extras = {
            let leading_fields_tys = leading_fields_tys.clone();
            let methods = make_methods(*trailing_field_ty);
            let (_, ty_generics, _) = ast.generics.split_for_impl();

            quote!(
                type PointerMetadata = <#trailing_field_ty as #zerocopy_crate::KnownLayout>::PointerMetadata;

                type MaybeUninit = __ZerocopyKnownLayoutMaybeUninit #ty_generics;

                // SAFETY: `LAYOUT` accurately describes the layout of `Self`.
                // The documentation of `DstLayout::for_repr_c_struct` vows that
                // invocations in this manner will accurately describe a type,
                // so long as:
                //
                //  - that type is `repr(C)`,
                //  - its fields are enumerated in the order they appear,
                //  - the presence of `repr_align` and `repr_packed` are
                //    correctly accounted for.
                //
                // We respect all three of these preconditions here. This
                // expansion is only used if `is_repr_c_struct`, we enumerate
                // the fields in order, and we extract the values of `align(N)`
                // and `packed(N)`.
                const LAYOUT: #zerocopy_crate::DstLayout = {
                    use #zerocopy_crate::util::macro_util::core_reexport::num::NonZeroUsize;
                    use #zerocopy_crate::{DstLayout, KnownLayout};

                    DstLayout::for_repr_c_struct(
                        #repr_align,
                        #repr_packed,
                        &[
                            #(DstLayout::for_type::<#leading_fields_tys>(),)*
                            <#trailing_field_ty as KnownLayout>::LAYOUT
                        ],
                    )
                };

                #methods
            )
        };

        let outer_extras = {
            let ident = &ast.ident;
            let vis = &ast.vis;
            let params = &ast.generics.params;
            let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

            let predicates = if let Some(where_clause) = where_clause {
                where_clause.predicates.clone()
            } else {
                Default::default()
            };

            // Generate a valid ident for a type-level handle to a field of a
            // given `name`.
            let field_index =
                |name| Ident::new(&format!("__Zerocopy_Field_{}", name), ident.span());

            let field_indices: Vec<_> =
                fields.iter().map(|(_vis, name, _ty)| field_index(name)).collect();

            // Define the collection of type-level field handles.
            let field_defs = field_indices.iter().zip(&fields).map(|(idx, (vis, _, _))| {
                quote! {
                    #[allow(non_camel_case_types)]
                    #vis struct #idx;
                }
            });

            let field_impls = field_indices.iter().zip(&fields).map(|(idx, (_, _, ty))| quote! {
                // SAFETY: `#ty` is the type of `#ident`'s field at `#idx`.
                unsafe impl #impl_generics #zerocopy_crate::util::macro_util::Field<#idx> for #ident #ty_generics
                where
                    #predicates
                {
                    type Type = #ty;
                }
            });

            let trailing_field_index = field_index(trailing_field_name);
            let leading_field_indices =
                leading_fields.iter().map(|(_vis, name, _ty)| field_index(name));

            let trailing_field_ty = quote! {
                <#ident #ty_generics as
                    #zerocopy_crate::util::macro_util::Field<#trailing_field_index>
                >::Type
            };

            let methods = make_methods(&parse_quote! {
                <#trailing_field_ty as #zerocopy_crate::KnownLayout>::MaybeUninit
            });

            quote! {
                #(#field_defs)*

                #(#field_impls)*

                // SAFETY: This has the same layout as the derive target type,
                // except that it admits uninit bytes. This is ensured by using
                // the same repr as the target type, and by using field types
                // which have the same layout as the target type's fields,
                // except that they admit uninit bytes. We indirect through
                // `Field` to ensure that occurrences of `Self` resolve to
                // `#ty`, not `__ZerocopyKnownLayoutMaybeUninit` (see #2116).
                #repr
                #[doc(hidden)]
                // Required on some rustc versions due to a lint that is only
                // triggered when `derive(KnownLayout)` is applied to `repr(C)`
                // structs that are generated by macros. See #2177 for details.
                #[allow(private_bounds)]
                #vis struct __ZerocopyKnownLayoutMaybeUninit<#params> (
                    #(#zerocopy_crate::util::macro_util::core_reexport::mem::MaybeUninit<
                        <#ident #ty_generics as
                            #zerocopy_crate::util::macro_util::Field<#leading_field_indices>
                        >::Type
                    >,)*
                    // NOTE(#2302): We wrap in `ManuallyDrop` here in case the
                    // type we're operating on is both generic and
                    // `repr(packed)`. In that case, Rust needs to know that the
                    // type is *either* `Sized` or has a trivial `Drop`.
                    // `ManuallyDrop` has a trivial `Drop`, and so satisfies
                    // this requirement.
                    #zerocopy_crate::util::macro_util::core_reexport::mem::ManuallyDrop<
                        <#trailing_field_ty as #zerocopy_crate::KnownLayout>::MaybeUninit
                    >
                )
                where
                    #trailing_field_ty: #zerocopy_crate::KnownLayout,
                    #predicates;

                // SAFETY: We largely defer to the `KnownLayout` implementation on
                // the derive target type (both by using the same tokens, and by
                // deferring to impl via type-level indirection). This is sound,
                // since  `__ZerocopyKnownLayoutMaybeUninit` is guaranteed to
                // have the same layout as the derive target type, except that
                // `__ZerocopyKnownLayoutMaybeUninit` admits uninit bytes.
                unsafe impl #impl_generics #zerocopy_crate::KnownLayout for __ZerocopyKnownLayoutMaybeUninit #ty_generics
                where
                    #trailing_field_ty: #zerocopy_crate::KnownLayout,
                    #predicates
                {
                    #[allow(clippy::missing_inline_in_public_items)]
                    fn only_derive_is_allowed_to_implement_this_trait() {}

                    type PointerMetadata = <#ident #ty_generics as #zerocopy_crate::KnownLayout>::PointerMetadata;

                    type MaybeUninit = Self;

                    const LAYOUT: #zerocopy_crate::DstLayout = <#ident #ty_generics as #zerocopy_crate::KnownLayout>::LAYOUT;

                    #methods
                }
            }
        };

        (SelfBounds::None, inner_extras, Some(outer_extras))
    } else {
        // For enums, unions, and non-`repr(C)` structs, we require that
        // `Self` is sized, and as a result don't need to reason about the
        // internals of the type.
        (
            SelfBounds::SIZED,
            quote!(
                type PointerMetadata = ();
                type MaybeUninit =
                    #zerocopy_crate::util::macro_util::core_reexport::mem::MaybeUninit<Self>;

                // SAFETY: `LAYOUT` is guaranteed to accurately describe the
                // layout of `Self`, because that is the documented safety
                // contract of `DstLayout::for_type`.
                const LAYOUT: #zerocopy_crate::DstLayout = #zerocopy_crate::DstLayout::for_type::<Self>();

                // SAFETY: `.cast` preserves address and provenance.
                //
                // FIXME(#429): Add documentation to `.cast` that promises that
                // it preserves provenance.
                #[inline(always)]
                fn raw_from_ptr_len(
                    bytes: #zerocopy_crate::util::macro_util::core_reexport::ptr::NonNull<u8>,
                    _meta: (),
                ) -> #zerocopy_crate::util::macro_util::core_reexport::ptr::NonNull<Self>
                {
                    bytes.cast::<Self>()
                }

                #[inline(always)]
                fn pointer_to_metadata(_ptr: *mut Self) -> () {}
            ),
            None,
        )
    };

    Ok(match &ast.data {
        Data::Struct(strct) => {
            let require_trait_bound_on_field_types = if self_bounds == SelfBounds::SIZED {
                FieldBounds::None
            } else {
                FieldBounds::TRAILING_SELF
            };

            // A bound on the trailing field is required, since structs are
            // unsized if their trailing field is unsized. Reflecting the layout
            // of an usized trailing field requires that the field is
            // `KnownLayout`.
            ImplBlockBuilder::new(
                ast,
                strct,
                Trait::KnownLayout,
                require_trait_bound_on_field_types,
                zerocopy_crate,
            )
            .self_type_trait_bounds(self_bounds)
            .inner_extras(inner_extras)
            .outer_extras(outer_extras)
            .build()
        }
        Data::Enum(enm) => {
            // A bound on the trailing field is not required, since enums cannot
            // currently be unsized.
            ImplBlockBuilder::new(ast, enm, Trait::KnownLayout, FieldBounds::None, zerocopy_crate)
                .self_type_trait_bounds(SelfBounds::SIZED)
                .inner_extras(inner_extras)
                .outer_extras(outer_extras)
                .build()
        }
        Data::Union(unn) => {
            // A bound on the trailing field is not required, since unions
            // cannot currently be unsized.
            ImplBlockBuilder::new(ast, unn, Trait::KnownLayout, FieldBounds::None, zerocopy_crate)
                .self_type_trait_bounds(SelfBounds::SIZED)
                .inner_extras(inner_extras)
                .outer_extras(outer_extras)
                .build()
        }
    })
}

fn derive_no_cell_inner(
    ast: &DeriveInput,
    _top_level: Trait,
    zerocopy_crate: &Path,
) -> TokenStream {
    match &ast.data {
        Data::Struct(strct) => ImplBlockBuilder::new(
            ast,
            strct,
            Trait::Immutable,
            FieldBounds::ALL_SELF,
            zerocopy_crate,
        )
        .build(),
        Data::Enum(enm) => {
            ImplBlockBuilder::new(ast, enm, Trait::Immutable, FieldBounds::ALL_SELF, zerocopy_crate)
                .build()
        }
        Data::Union(unn) => {
            ImplBlockBuilder::new(ast, unn, Trait::Immutable, FieldBounds::ALL_SELF, zerocopy_crate)
                .build()
        }
    }
}

fn derive_try_from_bytes_inner(
    ast: &DeriveInput,
    top_level: Trait,
    zerocopy_crate: &Path,
) -> Result<TokenStream, Error> {
    match &ast.data {
        Data::Struct(strct) => derive_try_from_bytes_struct(ast, strct, top_level, zerocopy_crate),
        Data::Enum(enm) => derive_try_from_bytes_enum(ast, enm, top_level, zerocopy_crate),
        Data::Union(unn) => Ok(derive_try_from_bytes_union(ast, unn, top_level, zerocopy_crate)),
    }
}

fn derive_from_zeros_inner(
    ast: &DeriveInput,
    top_level: Trait,
    zerocopy_crate: &Path,
) -> Result<TokenStream, Error> {
    let try_from_bytes = derive_try_from_bytes_inner(ast, top_level, zerocopy_crate)?;
    let from_zeros = match &ast.data {
        Data::Struct(strct) => derive_from_zeros_struct(ast, strct, zerocopy_crate),
        Data::Enum(enm) => derive_from_zeros_enum(ast, enm, zerocopy_crate)?,
        Data::Union(unn) => derive_from_zeros_union(ast, unn, zerocopy_crate),
    };
    Ok(IntoIterator::into_iter([try_from_bytes, from_zeros]).collect())
}

fn derive_from_bytes_inner(
    ast: &DeriveInput,
    top_level: Trait,
    zerocopy_crate: &Path,
) -> Result<TokenStream, Error> {
    let from_zeros = derive_from_zeros_inner(ast, top_level, zerocopy_crate)?;
    let from_bytes = match &ast.data {
        Data::Struct(strct) => derive_from_bytes_struct(ast, strct, zerocopy_crate),
        Data::Enum(enm) => derive_from_bytes_enum(ast, enm, zerocopy_crate)?,
        Data::Union(unn) => derive_from_bytes_union(ast, unn, zerocopy_crate),
    };

    Ok(IntoIterator::into_iter([from_zeros, from_bytes]).collect())
}

fn derive_into_bytes_inner(
    ast: &DeriveInput,
    _top_level: Trait,
    zerocopy_crate: &Path,
) -> Result<TokenStream, Error> {
    match &ast.data {
        Data::Struct(strct) => derive_into_bytes_struct(ast, strct, zerocopy_crate),
        Data::Enum(enm) => derive_into_bytes_enum(ast, enm, zerocopy_crate),
        Data::Union(unn) => derive_into_bytes_union(ast, unn, zerocopy_crate),
    }
}

fn derive_unaligned_inner(
    ast: &DeriveInput,
    _top_level: Trait,
    zerocopy_crate: &Path,
) -> Result<TokenStream, Error> {
    match &ast.data {
        Data::Struct(strct) => derive_unaligned_struct(ast, strct, zerocopy_crate),
        Data::Enum(enm) => derive_unaligned_enum(ast, enm, zerocopy_crate),
        Data::Union(unn) => derive_unaligned_union(ast, unn, zerocopy_crate),
    }
}

fn derive_hash_inner(
    ast: &DeriveInput,
    _top_level: Trait,
    zerocopy_crate: &Path,
) -> Result<TokenStream, Error> {
    // This doesn't delegate to `impl_block` because `impl_block` assumes it is deriving a
    // `zerocopy`-defined trait, and these trait impls share a common shape that `Hash` does not.
    // In particular, `zerocopy` traits contain a method that only `zerocopy_derive` macros
    // are supposed to implement, and `impl_block` generating this trait method is incompatible
    // with `Hash`.
    let type_ident = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let where_predicates = where_clause.map(|clause| &clause.predicates);
    Ok(quote! {
        // FIXME(#553): Add a test that generates a warning when
        // `#[allow(deprecated)]` isn't present.
        #[allow(deprecated)]
        // While there are not currently any warnings that this suppresses (that
        // we're aware of), it's good future-proofing hygiene.
        #[automatically_derived]
        impl #impl_generics #zerocopy_crate::util::macro_util::core_reexport::hash::Hash for #type_ident #ty_generics
        where
            Self: #zerocopy_crate::IntoBytes + #zerocopy_crate::Immutable,
            #where_predicates
        {
            fn hash<H>(&self, state: &mut H)
            where
                H: #zerocopy_crate::util::macro_util::core_reexport::hash::Hasher,
            {
                #zerocopy_crate::util::macro_util::core_reexport::hash::Hasher::write(
                    state,
                    #zerocopy_crate::IntoBytes::as_bytes(self)
                )
            }

            fn hash_slice<H>(data: &[Self], state: &mut H)
            where
                H: #zerocopy_crate::util::macro_util::core_reexport::hash::Hasher,
            {
                #zerocopy_crate::util::macro_util::core_reexport::hash::Hasher::write(
                    state,
                    #zerocopy_crate::IntoBytes::as_bytes(data)
                )
            }
        }
    })
}

fn derive_eq_inner(
    ast: &DeriveInput,
    _top_level: Trait,
    zerocopy_crate: &Path,
) -> Result<TokenStream, Error> {
    // This doesn't delegate to `impl_block` because `impl_block` assumes it is deriving a
    // `zerocopy`-defined trait, and these trait impls share a common shape that `Eq` does not.
    // In particular, `zerocopy` traits contain a method that only `zerocopy_derive` macros
    // are supposed to implement, and `impl_block` generating this trait method is incompatible
    // with `Eq`.
    let type_ident = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let where_predicates = where_clause.map(|clause| &clause.predicates);
    Ok(quote! {
        // FIXME(#553): Add a test that generates a warning when
        // `#[allow(deprecated)]` isn't present.
        #[allow(deprecated)]
        // While there are not currently any warnings that this suppresses (that
        // we're aware of), it's good future-proofing hygiene.
        #[automatically_derived]
        impl #impl_generics #zerocopy_crate::util::macro_util::core_reexport::cmp::PartialEq for #type_ident #ty_generics
        where
            Self: #zerocopy_crate::IntoBytes + #zerocopy_crate::Immutable,
            #where_predicates
        {
            fn eq(&self, other: &Self) -> bool {
                #zerocopy_crate::util::macro_util::core_reexport::cmp::PartialEq::eq(
                    #zerocopy_crate::IntoBytes::as_bytes(self),
                    #zerocopy_crate::IntoBytes::as_bytes(other),
                )
            }
        }

        // FIXME(#553): Add a test that generates a warning when
        // `#[allow(deprecated)]` isn't present.
        #[allow(deprecated)]
        // While there are not currently any warnings that this suppresses (that
        // we're aware of), it's good future-proofing hygiene.
        #[automatically_derived]
        impl #impl_generics #zerocopy_crate::util::macro_util::core_reexport::cmp::Eq for #type_ident #ty_generics
        where
            Self: #zerocopy_crate::IntoBytes + #zerocopy_crate::Immutable,
            #where_predicates
        {
        }
    })
}

fn derive_split_at_inner(
    ast: &DeriveInput,
    _top_level: Trait,
    zerocopy_crate: &Path,
) -> Result<TokenStream, Error> {
    let repr = StructUnionRepr::from_attrs(&ast.attrs)?;

    match &ast.data {
        Data::Struct(_) => {}
        Data::Enum(_) | Data::Union(_) => {
            return Err(Error::new(Span::call_site(), "can only be applied to structs"));
        }
    };

    if repr.get_packed().is_some() {
        return Err(Error::new(Span::call_site(), "must not have #[repr(packed)] attribute"));
    }

    if !(repr.is_c() || repr.is_transparent()) {
        return Err(Error::new(Span::call_site(), "must have #[repr(C)] or #[repr(transparent)] in order to guarantee this type's layout is splitable"));
    }

    let fields = ast.data.fields();
    let trailing_field = if let Some(((_, _, trailing_field), _)) = fields.split_last() {
        trailing_field
    } else {
        return Err(Error::new(Span::call_site(), "must at least one field"));
    };

    // SAFETY: `#ty`, per the above checks, is `repr(C)` or `repr(transparent)`
    // and is not packed; its trailing field is guaranteed to be well-aligned
    // for its type. By invariant on `FieldBounds::TRAILING_SELF`, the trailing
    // slice of the trailing field is also well-aligned for its type.
    Ok(ImplBlockBuilder::new(
        ast,
        &ast.data,
        Trait::SplitAt,
        FieldBounds::TRAILING_SELF,
        zerocopy_crate,
    )
    .inner_extras(quote! {
        type Elem = <#trailing_field as ::zerocopy::SplitAt>::Elem;
    })
    .build())
}

/// A struct is `TryFromBytes` if:
/// - all fields are `TryFromBytes`
fn derive_try_from_bytes_struct(
    ast: &DeriveInput,
    strct: &DataStruct,
    top_level: Trait,
    zerocopy_crate: &Path,
) -> Result<TokenStream, Error> {
    let extras =
        try_gen_trivial_is_bit_valid(ast, top_level, zerocopy_crate).unwrap_or_else(|| {
            let fields = strct.fields();
            let field_names = fields.iter().map(|(_vis, name, _ty)| name);
            let field_tys = fields.iter().map(|(_vis, _name, ty)| ty);
            quote!(
                // SAFETY: We use `is_bit_valid` to validate that each field is
                // bit-valid, and only return `true` if all of them are. The bit
                // validity of a struct is just the composition of the bit
                // validities of its fields, so this is a sound implementation of
                // `is_bit_valid`.
                fn is_bit_valid<___ZerocopyAliasing>(
                    mut candidate: #zerocopy_crate::Maybe<Self, ___ZerocopyAliasing>,
                ) -> #zerocopy_crate::util::macro_util::core_reexport::primitive::bool
                where
                    ___ZerocopyAliasing: #zerocopy_crate::pointer::invariant::Reference,
                {
                    use #zerocopy_crate::util::macro_util::core_reexport;
                    use #zerocopy_crate::pointer::PtrInner;

                    true #(&& {
                        // SAFETY:
                        // - `project` is a field projection, and so it addresses a
                        //   subset of the bytes addressed by `slf`
                        // - ..., and so it preserves provenance
                        // - ..., and `*slf` is a struct, so `UnsafeCell`s exist at
                        //   the same byte ranges in the returned pointer's referent
                        //   as they do in `*slf`
                        let field_candidate = unsafe {
                            let project = |slf: PtrInner<'_, Self>| {
                                let slf = slf.as_non_null().as_ptr();
                                let field = core_reexport::ptr::addr_of_mut!((*slf).#field_names);
                                // SAFETY: `cast_unsized_unchecked` promises that
                                // `slf` will either reference a zero-sized byte
                                // range, or else will reference a byte range that
                                // is entirely contained within an allocated
                                // object. In either case, this guarantees that
                                // field projection will not wrap around the address
                                // space, and so `field` will be non-null.
                                let ptr = unsafe { core_reexport::ptr::NonNull::new_unchecked(field) };
                                // SAFETY:
                                // 0. `ptr` addresses a subset of the bytes of
                                //    `slf`, so by invariant on `slf: PtrInner`,
                                //    if `ptr`'s referent is not zero sized,
                                //    then `ptr` has valid provenance for its
                                //    referent, which is entirely contained in
                                //    some Rust allocation, `A`.
                                // 1. By invariant on `slf: PtrInner`, if
                                //    `ptr`'s referent is not zero sized, `A` is
                                //    guaranteed to live for at least `'a`.
                                unsafe { PtrInner::new(ptr) }
                            };

                            candidate.reborrow().cast_unsized_unchecked(project)
                        };

                        <#field_tys as #zerocopy_crate::TryFromBytes>::is_bit_valid(field_candidate)
                    })*
                }
            )
        });
    Ok(ImplBlockBuilder::new(
        ast,
        strct,
        Trait::TryFromBytes,
        FieldBounds::ALL_SELF,
        zerocopy_crate,
    )
    .inner_extras(extras)
    .build())
}

/// A union is `TryFromBytes` if:
/// - all of its fields are `TryFromBytes` and `Immutable`
fn derive_try_from_bytes_union(
    ast: &DeriveInput,
    unn: &DataUnion,
    top_level: Trait,
    zerocopy_crate: &Path,
) -> TokenStream {
    // FIXME(#5): Remove the `Immutable` bound.
    let field_type_trait_bounds =
        FieldBounds::All(&[TraitBound::Slf, TraitBound::Other(Trait::Immutable)]);
    let extras =
        try_gen_trivial_is_bit_valid(ast, top_level, zerocopy_crate).unwrap_or_else(|| {
            let fields = unn.fields();
            let field_names = fields.iter().map(|(_vis, name, _ty)| name);
            let field_tys = fields.iter().map(|(_vis, _name, ty)| ty);
            quote!(
                // SAFETY: We use `is_bit_valid` to validate that any field is
                // bit-valid; we only return `true` if at least one of them is. The
                // bit validity of a union is not yet well defined in Rust, but it
                // is guaranteed to be no more strict than this definition. See #696
                // for a more in-depth discussion.
                fn is_bit_valid<___ZerocopyAliasing>(
                    mut candidate: #zerocopy_crate::Maybe<'_, Self,___ZerocopyAliasing>
                ) -> #zerocopy_crate::util::macro_util::core_reexport::primitive::bool
                where
                    ___ZerocopyAliasing: #zerocopy_crate::pointer::invariant::Reference,
                {
                    use #zerocopy_crate::util::macro_util::core_reexport;
                    use #zerocopy_crate::pointer::PtrInner;

                    false #(|| {
                        // SAFETY:
                        // - `project` is a field projection, and so it addresses a
                        //   subset of the bytes addressed by `slf`
                        // - ..., and so it preserves provenance
                        // - Since `Self: Immutable` is enforced by
                        //   `self_type_trait_bounds`, neither `*slf` nor the
                        //   returned pointer's referent contain any `UnsafeCell`s
                        let field_candidate = unsafe {
                            let project = |slf: PtrInner<'_, Self>| {
                                let slf = slf.as_non_null().as_ptr();
                                let field = core_reexport::ptr::addr_of_mut!((*slf).#field_names);
                                // SAFETY: `cast_unsized_unchecked` promises that
                                // `slf` will either reference a zero-sized byte
                                // range, or else will reference a byte range that
                                // is entirely contained within an allocated
                                // object. In either case, this guarantees that
                                // field projection will not wrap around the address
                                // space, and so `field` will be non-null.
                                let ptr = unsafe { core_reexport::ptr::NonNull::new_unchecked(field) };
                                // SAFETY:
                                // 0. `ptr` addresses a subset of the bytes of
                                //    `slf`, so by invariant on `slf: PtrInner`,
                                //    if `ptr`'s referent is not zero sized,
                                //    then `ptr` has valid provenance for its
                                //    referent, which is entirely contained in
                                //    some Rust allocation, `A`.
                                // 1. By invariant on `slf: PtrInner`, if
                                //    `ptr`'s referent is not zero sized, `A` is
                                //    guaranteed to live for at least `'a`.
                                unsafe { PtrInner::new(ptr) }
                            };

                            candidate.reborrow().cast_unsized_unchecked(project)
                        };

                        <#field_tys as #zerocopy_crate::TryFromBytes>::is_bit_valid(field_candidate)
                    })*
                }
            )
        });
    ImplBlockBuilder::new(ast, unn, Trait::TryFromBytes, field_type_trait_bounds, zerocopy_crate)
        .inner_extras(extras)
        .build()
}

fn derive_try_from_bytes_enum(
    ast: &DeriveInput,
    enm: &DataEnum,
    top_level: Trait,
    zerocopy_crate: &Path,
) -> Result<TokenStream, Error> {
    let repr = EnumRepr::from_attrs(&ast.attrs)?;

    // If an enum has no fields, it has a well-defined integer representation,
    // and every possible bit pattern corresponds to a valid discriminant tag,
    // then it *could* be `FromBytes` (even if the user hasn't derived
    // `FromBytes`). This holds if, for `repr(uN)` or `repr(iN)`, there are 2^N
    // variants.
    let could_be_from_bytes = enum_size_from_repr(&repr)
        .map(|size| enm.fields().is_empty() && enm.variants.len() == 1usize << size)
        .unwrap_or(false);

    let trivial_is_bit_valid = try_gen_trivial_is_bit_valid(ast, top_level, zerocopy_crate);
    let extra = match (trivial_is_bit_valid, could_be_from_bytes) {
        (Some(is_bit_valid), _) => is_bit_valid,
        // SAFETY: It would be sound for the enum to implement `FromBytes`, as
        // required by `gen_trivial_is_bit_valid_unchecked`.
        (None, true) => unsafe { gen_trivial_is_bit_valid_unchecked(zerocopy_crate) },
        (None, false) => {
            r#enum::derive_is_bit_valid(&ast.ident, &repr, &ast.generics, enm, zerocopy_crate)?
        }
    };

    Ok(ImplBlockBuilder::new(ast, enm, Trait::TryFromBytes, FieldBounds::ALL_SELF, zerocopy_crate)
        .inner_extras(extra)
        .build())
}

/// Attempts to generate a `TryFromBytes::is_bit_valid` instance that
/// unconditionally returns true.
///
/// This is possible when the `top_level` trait is `FromBytes` and there are no
/// generic type parameters. In this case, we know that compilation will succeed
/// only if the type is unconditionally `FromBytes`. Type parameters are not
/// supported because a type with type parameters could be `TryFromBytes` but
/// not `FromBytes` depending on its type parameters, and so deriving a trivial
/// `is_bit_valid` would be either unsound or, assuming we add a defensive
/// `Self: FromBytes` bound (as we currently do), overly restrictive. Consider,
/// for example, that `Foo<bool>` ought to be `TryFromBytes` but not `FromBytes`
/// in this example:
///
/// ```rust,ignore
/// #[derive(FromBytes)]
/// #[repr(transparent)]
/// struct Foo<T>(T);
/// ```
///
/// This should be used where possible. Using this impl is faster to codegen,
/// faster to compile, and is friendlier on the optimizer.
fn try_gen_trivial_is_bit_valid(
    ast: &DeriveInput,
    top_level: Trait,
    zerocopy_crate: &Path,
) -> Option<proc_macro2::TokenStream> {
    // If the top-level trait is `FromBytes` and `Self` has no type parameters,
    // then the `FromBytes` derive will fail compilation if `Self` is not
    // actually soundly `FromBytes`, and so we can rely on that for our
    // `is_bit_valid` impl. It's plausible that we could make changes - or Rust
    // could make changes (such as the "trivial bounds" language feature) - that
    // make this no longer true. To hedge against these, we include an explicit
    // `Self: FromBytes` check in the generated `is_bit_valid`, which is
    // bulletproof.
    if top_level == Trait::FromBytes && ast.generics.params.is_empty() {
        Some(quote!(
            // SAFETY: See inline.
            fn is_bit_valid<___ZerocopyAliasing>(
                _candidate: #zerocopy_crate::Maybe<Self, ___ZerocopyAliasing>,
            ) -> #zerocopy_crate::util::macro_util::core_reexport::primitive::bool
            where
                ___ZerocopyAliasing: #zerocopy_crate::pointer::invariant::Reference,
            {
                if false {
                    fn assert_is_from_bytes<T>()
                    where
                        T: #zerocopy_crate::FromBytes,
                        T: ?#zerocopy_crate::util::macro_util::core_reexport::marker::Sized,
                    {
                    }

                    assert_is_from_bytes::<Self>();
                }

                // SAFETY: The preceding code only compiles if `Self:
                // FromBytes`. Thus, this code only compiles if all initialized
                // byte sequences represent valid instances of `Self`.
                true
            }
        ))
    } else {
        None
    }
}

/// Generates a `TryFromBytes::is_bit_valid` instance that unconditionally
/// returns true.
///
/// This should be used where possible, (although `try_gen_trivial_is_bit_valid`
/// should be preferred over this for safety reasons). Using this impl is faster
/// to codegen, faster to compile, and is friendlier on the optimizer.
///
/// # Safety
///
/// The caller must ensure that all initialized bit patterns are valid for
/// `Self`.
unsafe fn gen_trivial_is_bit_valid_unchecked(zerocopy_crate: &Path) -> proc_macro2::TokenStream {
    quote!(
        // SAFETY: The caller of `gen_trivial_is_bit_valid_unchecked` has
        // promised that all initialized bit patterns are valid for `Self`.
        fn is_bit_valid<___ZerocopyAliasing>(
            _candidate: #zerocopy_crate::Maybe<Self, ___ZerocopyAliasing>,
        ) -> #zerocopy_crate::util::macro_util::core_reexport::primitive::bool
        where
            ___ZerocopyAliasing: #zerocopy_crate::pointer::invariant::Reference,
        {
            true
        }
    )
}

/// A struct is `FromZeros` if:
/// - all fields are `FromZeros`
fn derive_from_zeros_struct(
    ast: &DeriveInput,
    strct: &DataStruct,
    zerocopy_crate: &Path,
) -> TokenStream {
    ImplBlockBuilder::new(ast, strct, Trait::FromZeros, FieldBounds::ALL_SELF, zerocopy_crate)
        .build()
}

/// Returns `Ok(index)` if variant `index` of the enum has a discriminant of
/// zero. If `Err(bool)` is returned, the boolean is true if the enum has
/// unknown discriminants (e.g. discriminants set to const expressions which we
/// can't evaluate in a proc macro). If the enum has unknown discriminants, then
/// it might have a zero variant that we just can't detect.
fn find_zero_variant(enm: &DataEnum) -> Result<usize, bool> {
    // Discriminants can be anywhere in the range [i128::MIN, u128::MAX] because
    // the discriminant type may be signed or unsigned. Since we only care about
    // tracking the discriminant when it's less than or equal to zero, we can
    // avoid u128 -> i128 conversions and bounds checking by making the "next
    // discriminant" value implicitly negative.
    // Technically 64 bits is enough, but 128 is better for future compatibility
    // with https://github.com/rust-lang/rust/issues/56071
    let mut next_negative_discriminant = Some(0);

    // Sometimes we encounter explicit discriminants that we can't know the
    // value of (e.g. a constant expression that requires evaluation). These
    // could evaluate to zero or a negative number, but we can't assume that
    // they do (no false positives allowed!). So we treat them like strictly-
    // positive values that can't result in any zero variants, and track whether
    // we've encountered any unknown discriminants.
    let mut has_unknown_discriminants = false;

    for (i, v) in enm.variants.iter().enumerate() {
        match v.discriminant.as_ref() {
            // Implicit discriminant
            None => {
                match next_negative_discriminant.as_mut() {
                    Some(0) => return Ok(i),
                    // n is nonzero so subtraction is always safe
                    Some(n) => *n -= 1,
                    None => (),
                }
            }
            // Explicit positive discriminant
            Some((_, Expr::Lit(ExprLit { lit: Lit::Int(int), .. }))) => {
                match int.base10_parse::<u128>().ok() {
                    Some(0) => return Ok(i),
                    Some(_) => next_negative_discriminant = None,
                    None => {
                        // Numbers should never fail to parse, but just in case:
                        has_unknown_discriminants = true;
                        next_negative_discriminant = None;
                    }
                }
            }
            // Explicit negative discriminant
            Some((_, Expr::Unary(ExprUnary { op: UnOp::Neg(_), expr, .. }))) => match &**expr {
                Expr::Lit(ExprLit { lit: Lit::Int(int), .. }) => {
                    match int.base10_parse::<u128>().ok() {
                        Some(0) => return Ok(i),
                        // x is nonzero so subtraction is always safe
                        Some(x) => next_negative_discriminant = Some(x - 1),
                        None => {
                            // Numbers should never fail to parse, but just in
                            // case:
                            has_unknown_discriminants = true;
                            next_negative_discriminant = None;
                        }
                    }
                }
                // Unknown negative discriminant (e.g. const repr)
                _ => {
                    has_unknown_discriminants = true;
                    next_negative_discriminant = None;
                }
            },
            // Unknown discriminant (e.g. const expr)
            _ => {
                has_unknown_discriminants = true;
                next_negative_discriminant = None;
            }
        }
    }

    Err(has_unknown_discriminants)
}

/// An enum is `FromZeros` if:
/// - one of the variants has a discriminant of `0`
/// - that variant's fields are all `FromZeros`
fn derive_from_zeros_enum(
    ast: &DeriveInput,
    enm: &DataEnum,
    zerocopy_crate: &Path,
) -> Result<TokenStream, Error> {
    let repr = EnumRepr::from_attrs(&ast.attrs)?;

    // We don't actually care what the repr is; we just care that it's one of
    // the allowed ones.
    match repr {
         Repr::Compound(
            Spanned { t: CompoundRepr::C | CompoundRepr::Primitive(_), span: _ },
            _,
        ) => {}
        Repr::Transparent(_)
        | Repr::Compound(Spanned { t: CompoundRepr::Rust, span: _ }, _) => return Err(Error::new(Span::call_site(), "must have #[repr(C)] or #[repr(Int)] attribute in order to guarantee this type's memory layout")),
    }

    let zero_variant = match find_zero_variant(enm) {
        Ok(index) => enm.variants.iter().nth(index).unwrap(),
        // Has unknown variants
        Err(true) => {
            return Err(Error::new_spanned(
                ast,
                "FromZeros only supported on enums with a variant that has a discriminant of `0`\n\
                help: This enum has discriminants which are not literal integers. One of those may \
                define or imply which variant has a discriminant of zero. Use a literal integer to \
                define or imply the variant with a discriminant of zero.",
            ));
        }
        // Does not have unknown variants
        Err(false) => {
            return Err(Error::new_spanned(
                ast,
                "FromZeros only supported on enums with a variant that has a discriminant of `0`",
            ));
        }
    };

    let explicit_bounds = zero_variant
        .fields
        .iter()
        .map(|field| {
            let ty = &field.ty;
            parse_quote! { #ty: #zerocopy_crate::FromZeros }
        })
        .collect::<Vec<WherePredicate>>();

    Ok(ImplBlockBuilder::new(
        ast,
        enm,
        Trait::FromZeros,
        FieldBounds::Explicit(explicit_bounds),
        zerocopy_crate,
    )
    .build())
}

/// Unions are `FromZeros` if
/// - all fields are `FromZeros` and `Immutable`
fn derive_from_zeros_union(
    ast: &DeriveInput,
    unn: &DataUnion,
    zerocopy_crate: &Path,
) -> TokenStream {
    // FIXME(#5): Remove the `Immutable` bound. It's only necessary for
    // compatibility with `derive(TryFromBytes)` on unions; not for soundness.
    let field_type_trait_bounds =
        FieldBounds::All(&[TraitBound::Slf, TraitBound::Other(Trait::Immutable)]);
    ImplBlockBuilder::new(ast, unn, Trait::FromZeros, field_type_trait_bounds, zerocopy_crate)
        .build()
}

/// A struct is `FromBytes` if:
/// - all fields are `FromBytes`
fn derive_from_bytes_struct(
    ast: &DeriveInput,
    strct: &DataStruct,
    zerocopy_crate: &Path,
) -> TokenStream {
    ImplBlockBuilder::new(ast, strct, Trait::FromBytes, FieldBounds::ALL_SELF, zerocopy_crate)
        .build()
}

/// An enum is `FromBytes` if:
/// - Every possible bit pattern must be valid, which means that every bit
///   pattern must correspond to a different enum variant. Thus, for an enum
///   whose layout takes up N bytes, there must be 2^N variants.
/// - Since we must know N, only representations which guarantee the layout's
///   size are allowed. These are `repr(uN)` and `repr(iN)` (`repr(C)` implies an
///   implementation-defined size). `usize` and `isize` technically guarantee the
///   layout's size, but would require us to know how large those are on the
///   target platform. This isn't terribly difficult - we could emit a const
///   expression that could call `core::mem::size_of` in order to determine the
///   size and check against the number of enum variants, but a) this would be
///   platform-specific and, b) even on Rust's smallest bit width platform (32),
///   this would require ~4 billion enum variants, which obviously isn't a thing.
/// - All fields of all variants are `FromBytes`.
fn derive_from_bytes_enum(
    ast: &DeriveInput,
    enm: &DataEnum,
    zerocopy_crate: &Path,
) -> Result<TokenStream, Error> {
    let repr = EnumRepr::from_attrs(&ast.attrs)?;

    let variants_required = 1usize << enum_size_from_repr(&repr)?;
    if enm.variants.len() != variants_required {
        return Err(Error::new_spanned(
            ast,
            format!(
                "FromBytes only supported on {} enum with {} variants",
                repr.repr_type_name(),
                variants_required
            ),
        ));
    }

    Ok(ImplBlockBuilder::new(ast, enm, Trait::FromBytes, FieldBounds::ALL_SELF, zerocopy_crate)
        .build())
}

// Returns `None` if the enum's size is not guaranteed by the repr.
fn enum_size_from_repr(repr: &EnumRepr) -> Result<usize, Error> {
    use CompoundRepr::*;
    use PrimitiveRepr::*;
    use Repr::*;
    match repr {
        Transparent(span)
        | Compound(
            Spanned { t: C | Rust | Primitive(U32 | I32 | U64 | I64 | U128 | I128 | Usize | Isize), span },
            _,
        ) => Err(Error::new(*span, "`FromBytes` only supported on enums with `#[repr(...)]` attributes `u8`, `i8`, `u16`, or `i16`")),
        Compound(Spanned { t: Primitive(U8 | I8), span: _ }, _align) => Ok(8),
        Compound(Spanned { t: Primitive(U16 | I16), span: _ }, _align) => Ok(16),
    }
}

/// Unions are `FromBytes` if
/// - all fields are `FromBytes` and `Immutable`
fn derive_from_bytes_union(
    ast: &DeriveInput,
    unn: &DataUnion,
    zerocopy_crate: &Path,
) -> TokenStream {
    // FIXME(#5): Remove the `Immutable` bound. It's only necessary for
    // compatibility with `derive(TryFromBytes)` on unions; not for soundness.
    let field_type_trait_bounds =
        FieldBounds::All(&[TraitBound::Slf, TraitBound::Other(Trait::Immutable)]);
    ImplBlockBuilder::new(ast, unn, Trait::FromBytes, field_type_trait_bounds, zerocopy_crate)
        .build()
}

fn derive_into_bytes_struct(
    ast: &DeriveInput,
    strct: &DataStruct,
    zerocopy_crate: &Path,
) -> Result<TokenStream, Error> {
    let repr = StructUnionRepr::from_attrs(&ast.attrs)?;

    let is_transparent = repr.is_transparent();
    let is_c = repr.is_c();
    let is_packed_1 = repr.is_packed_1();
    let num_fields = strct.fields().len();

    let (padding_check, require_unaligned_fields) = if is_transparent || is_packed_1 {
        // No padding check needed.
        // - repr(transparent): The layout and ABI of the whole struct is the
        //   same as its only non-ZST field (meaning there's no padding outside
        //   of that field) and we require that field to be `IntoBytes` (meaning
        //   there's no padding in that field).
        // - repr(packed): Any inter-field padding bytes are removed, meaning
        //   that any padding bytes would need to come from the fields, all of
        //   which we require to be `IntoBytes` (meaning they don't have any
        //   padding). Note that this holds regardless of other `repr`
        //   attributes, including `repr(Rust)`. [1]
        //
        // [1] Per https://doc.rust-lang.org/1.81.0/reference/type-layout.html#the-alignment-modifiers:
        //
        //   An important consequence of these rules is that a type with
        //   `#[repr(packed(1))]`` (or `#[repr(packed)]``) will have no
        //   inter-field padding.
        (None, false)
    } else if is_c && !repr.is_align_gt_1() && num_fields <= 1 {
        // No padding check needed. A repr(C) struct with zero or one field has
        // no padding unless #[repr(align)] explicitly adds padding, which we
        // check for in this branch's condition.
        (None, false)
    } else if ast.generics.params.is_empty() {
        // Is the last field a syntactic slice, i.e., `[SomeType]`.
        let is_syntactic_dst =
            strct.fields().last().map(|(_, _, ty)| matches!(ty, Type::Slice(_))).unwrap_or(false);
        // Since there are no generics, we can emit a padding check. All reprs
        // guarantee that fields won't overlap [1], so the padding check is
        // sound. This is more permissive than the next case, which requires
        // that all field types implement `Unaligned`.
        //
        // [1] Per https://doc.rust-lang.org/1.81.0/reference/type-layout.html#the-rust-representation:
        //
        //   The only data layout guarantees made by [`repr(Rust)`] are those
        //   required for soundness. They are:
        //   ...
        //   2. The fields do not overlap.
        //   ...
        if is_c && is_syntactic_dst {
            (Some(PaddingCheck::ReprCStruct), false)
        } else {
            (Some(PaddingCheck::Struct), false)
        }
    } else if is_c && !repr.is_align_gt_1() {
        // We can't use a padding check since there are generic type arguments.
        // Instead, we require all field types to implement `Unaligned`. This
        // ensures that the `repr(C)` layout algorithm will not insert any
        // padding unless #[repr(align)] explicitly adds padding, which we check
        // for in this branch's condition.
        //
        // FIXME(#10): Support type parameters for non-transparent, non-packed
        // structs without requiring `Unaligned`.
        (None, true)
    } else {
        return Err(Error::new(Span::call_site(), "must have a non-align #[repr(...)] attribute in order to guarantee this type's memory layout"));
    };

    let field_bounds = if require_unaligned_fields {
        FieldBounds::All(&[TraitBound::Slf, TraitBound::Other(Trait::Unaligned)])
    } else {
        FieldBounds::ALL_SELF
    };

    Ok(ImplBlockBuilder::new(ast, strct, Trait::IntoBytes, field_bounds, zerocopy_crate)
        .padding_check(padding_check)
        .build())
}

/// If the type is an enum:
/// - It must have a defined representation (`repr`s `C`, `u8`, `u16`, `u32`,
///   `u64`, `usize`, `i8`, `i16`, `i32`, `i64`, or `isize`).
/// - It must have no padding bytes.
/// - Its fields must be `IntoBytes`.
fn derive_into_bytes_enum(
    ast: &DeriveInput,
    enm: &DataEnum,
    zerocopy_crate: &Path,
) -> Result<TokenStream, Error> {
    let repr = EnumRepr::from_attrs(&ast.attrs)?;
    if !repr.is_c() && !repr.is_primitive() {
        return Err(Error::new(Span::call_site(), "must have #[repr(C)] or #[repr(Int)] attribute in order to guarantee this type's memory layout"));
    }

    let tag_type_definition = r#enum::generate_tag_enum(&repr, enm);
    Ok(ImplBlockBuilder::new(ast, enm, Trait::IntoBytes, FieldBounds::ALL_SELF, zerocopy_crate)
        .padding_check(PaddingCheck::Enum { tag_type_definition })
        .build())
}

/// A union is `IntoBytes` if:
/// - all fields are `IntoBytes`
/// - `repr(C)`, `repr(transparent)`, or `repr(packed)`
/// - no padding (size of union equals size of each field type)
fn derive_into_bytes_union(
    ast: &DeriveInput,
    unn: &DataUnion,
    zerocopy_crate: &Path,
) -> Result<TokenStream, Error> {
    // See #1792 for more context.
    //
    // By checking for `zerocopy_derive_union_into_bytes` both here and in the
    // generated code, we ensure that `--cfg zerocopy_derive_union_into_bytes`
    // need only be passed *either* when compiling this crate *or* when
    // compiling the user's crate. The former is preferable, but in some
    // situations (such as when cross-compiling using `cargo build --target`),
    // it doesn't get propagated to this crate's build by default.
    let cfg_compile_error = if cfg!(zerocopy_derive_union_into_bytes) {
        quote!()
    } else {
        let error_message = "requires --cfg zerocopy_derive_union_into_bytes;
please let us know you use this feature: https://github.com/google/zerocopy/discussions/1802";
        quote!(
            const _: () = {
                #[cfg(not(zerocopy_derive_union_into_bytes))]
                #zerocopy_crate::util::macro_util::core_reexport::compile_error!(#error_message);
            };
        )
    };

    // FIXME(#10): Support type parameters.
    if !ast.generics.params.is_empty() {
        return Err(Error::new(Span::call_site(), "unsupported on types with type parameters"));
    }

    // Because we don't support generics, we don't need to worry about
    // special-casing different reprs. So long as there is *some* repr which
    // guarantees the layout, our `PaddingCheck::Union` guarantees that there is
    // no padding.
    let repr = StructUnionRepr::from_attrs(&ast.attrs)?;
    if !repr.is_c() && !repr.is_transparent() && !repr.is_packed_1() {
        return Err(Error::new(
            Span::call_site(),
            "must be #[repr(C)], #[repr(packed)], or #[repr(transparent)]",
        ));
    }

    let impl_block =
        ImplBlockBuilder::new(ast, unn, Trait::IntoBytes, FieldBounds::ALL_SELF, zerocopy_crate)
            .padding_check(PaddingCheck::Union)
            .build();
    Ok(quote!(#cfg_compile_error #impl_block))
}

/// A struct is `Unaligned` if:
/// - `repr(align)` is no more than 1 and either
///   - `repr(C)` or `repr(transparent)` and
///     - all fields `Unaligned`
///   - `repr(packed)`
fn derive_unaligned_struct(
    ast: &DeriveInput,
    strct: &DataStruct,
    zerocopy_crate: &Path,
) -> Result<TokenStream, Error> {
    let repr = StructUnionRepr::from_attrs(&ast.attrs)?;
    repr.unaligned_validate_no_align_gt_1()?;

    let field_bounds = if repr.is_packed_1() {
        FieldBounds::None
    } else if repr.is_c() || repr.is_transparent() {
        FieldBounds::ALL_SELF
    } else {
        return Err(Error::new(Span::call_site(), "must have #[repr(C)], #[repr(transparent)], or #[repr(packed)] attribute in order to guarantee this type's alignment"));
    };

    Ok(ImplBlockBuilder::new(ast, strct, Trait::Unaligned, field_bounds, zerocopy_crate).build())
}

/// An enum is `Unaligned` if:
/// - No `repr(align(N > 1))`
/// - `repr(u8)` or `repr(i8)`
fn derive_unaligned_enum(
    ast: &DeriveInput,
    enm: &DataEnum,
    zerocopy_crate: &Path,
) -> Result<TokenStream, Error> {
    let repr = EnumRepr::from_attrs(&ast.attrs)?;
    repr.unaligned_validate_no_align_gt_1()?;

    if !repr.is_u8() && !repr.is_i8() {
        return Err(Error::new(Span::call_site(), "must have #[repr(u8)] or #[repr(i8)] attribute in order to guarantee this type's alignment"));
    }

    Ok(ImplBlockBuilder::new(ast, enm, Trait::Unaligned, FieldBounds::ALL_SELF, zerocopy_crate)
        .build())
}

/// Like structs, a union is `Unaligned` if:
/// - `repr(align)` is no more than 1 and either
///   - `repr(C)` or `repr(transparent)` and
///     - all fields `Unaligned`
///   - `repr(packed)`
fn derive_unaligned_union(
    ast: &DeriveInput,
    unn: &DataUnion,
    zerocopy_crate: &Path,
) -> Result<TokenStream, Error> {
    let repr = StructUnionRepr::from_attrs(&ast.attrs)?;
    repr.unaligned_validate_no_align_gt_1()?;

    let field_type_trait_bounds = if repr.is_packed_1() {
        FieldBounds::None
    } else if repr.is_c() || repr.is_transparent() {
        FieldBounds::ALL_SELF
    } else {
        return Err(Error::new(Span::call_site(), "must have #[repr(C)], #[repr(transparent)], or #[repr(packed)] attribute in order to guarantee this type's alignment"));
    };

    Ok(ImplBlockBuilder::new(ast, unn, Trait::Unaligned, field_type_trait_bounds, zerocopy_crate)
        .build())
}

/// This enum describes what kind of padding check needs to be generated for the
/// associated impl.
enum PaddingCheck {
    /// Check that the sum of the fields' sizes exactly equals the struct's
    /// size.
    Struct,
    /// Check that a `repr(C)` struct has no padding.
    ReprCStruct,
    /// Check that the size of each field exactly equals the union's size.
    Union,
    /// Check that every variant of the enum contains no padding.
    ///
    /// Because doing so requires a tag enum, this padding check requires an
    /// additional `TokenStream` which defines the tag enum as `___ZerocopyTag`.
    Enum { tag_type_definition: TokenStream },
}

impl PaddingCheck {
    /// Returns the idents of the trait to use and the macro to call in order to
    /// validate that a type passes the relevant padding check.
    fn validator_trait_and_macro_idents(&self) -> (Ident, Ident) {
        let (trt, mcro) = match self {
            PaddingCheck::Struct => ("PaddingFree", "struct_padding"),
            PaddingCheck::ReprCStruct => ("DynamicPaddingFree", "repr_c_struct_has_padding"),
            PaddingCheck::Union => ("PaddingFree", "union_padding"),
            PaddingCheck::Enum { .. } => ("PaddingFree", "enum_padding"),
        };

        let trt = Ident::new(trt, Span::call_site());
        let mcro = Ident::new(mcro, Span::call_site());
        (trt, mcro)
    }

    /// Sometimes performing the padding check requires some additional
    /// "context" code. For enums, this is the definition of the tag enum.
    fn validator_macro_context(&self) -> Option<&TokenStream> {
        match self {
            PaddingCheck::Struct | PaddingCheck::ReprCStruct | PaddingCheck::Union => None,
            PaddingCheck::Enum { tag_type_definition } => Some(tag_type_definition),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Trait {
    KnownLayout,
    Immutable,
    TryFromBytes,
    FromZeros,
    FromBytes,
    IntoBytes,
    Unaligned,
    Sized,
    ByteHash,
    ByteEq,
    SplitAt,
}

impl ToTokens for Trait {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        // According to [1], the format of the derived `Debug`` output is not
        // stable and therefore not guaranteed to represent the variant names.
        // Indeed with the (unstable) `fmt-debug` compiler flag [2], it can
        // return only a minimalized output or empty string. To make sure this
        // code will work in the future and independent of the compiler flag, we
        // translate the variants to their names manually here.
        //
        // [1] https://doc.rust-lang.org/1.81.0/std/fmt/trait.Debug.html#stability
        // [2] https://doc.rust-lang.org/beta/unstable-book/compiler-flags/fmt-debug.html
        let s = match self {
            Trait::KnownLayout => "KnownLayout",
            Trait::Immutable => "Immutable",
            Trait::TryFromBytes => "TryFromBytes",
            Trait::FromZeros => "FromZeros",
            Trait::FromBytes => "FromBytes",
            Trait::IntoBytes => "IntoBytes",
            Trait::Unaligned => "Unaligned",
            Trait::Sized => "Sized",
            Trait::ByteHash => "ByteHash",
            Trait::ByteEq => "ByteEq",
            Trait::SplitAt => "SplitAt",
        };
        let ident = Ident::new(s, Span::call_site());
        tokens.extend(core::iter::once(TokenTree::Ident(ident)));
    }
}

impl Trait {
    fn crate_path(&self, zerocopy_crate: &Path) -> Path {
        match self {
            Self::Sized => {
                parse_quote!(#zerocopy_crate::util::macro_util::core_reexport::marker::#self)
            }
            _ => parse_quote!(#zerocopy_crate::#self),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum TraitBound {
    Slf,
    Other(Trait),
}

enum FieldBounds<'a> {
    None,
    All(&'a [TraitBound]),
    Trailing(&'a [TraitBound]),
    Explicit(Vec<WherePredicate>),
}

impl<'a> FieldBounds<'a> {
    const ALL_SELF: FieldBounds<'a> = FieldBounds::All(&[TraitBound::Slf]);
    const TRAILING_SELF: FieldBounds<'a> = FieldBounds::Trailing(&[TraitBound::Slf]);
}

#[derive(Debug, Eq, PartialEq)]
enum SelfBounds<'a> {
    None,
    All(&'a [Trait]),
}

// FIXME(https://github.com/rust-lang/rust-clippy/issues/12908): This is a false
// positive. Explicit lifetimes are actually necessary here.
#[allow(clippy::needless_lifetimes)]
impl<'a> SelfBounds<'a> {
    const SIZED: Self = Self::All(&[Trait::Sized]);
}

/// Normalizes a slice of bounds by replacing [`TraitBound::Slf`] with `slf`.
fn normalize_bounds(slf: Trait, bounds: &[TraitBound]) -> impl '_ + Iterator<Item = Trait> {
    bounds.iter().map(move |bound| match bound {
        TraitBound::Slf => slf,
        TraitBound::Other(trt) => *trt,
    })
}

struct ImplBlockBuilder<'a, D: DataExt> {
    input: &'a DeriveInput,
    data: &'a D,
    trt: Trait,
    field_type_trait_bounds: FieldBounds<'a>,
    zerocopy_crate: &'a Path,
    self_type_trait_bounds: SelfBounds<'a>,
    padding_check: Option<PaddingCheck>,
    inner_extras: Option<TokenStream>,
    outer_extras: Option<TokenStream>,
}

impl<'a, D: DataExt> ImplBlockBuilder<'a, D> {
    fn new(
        input: &'a DeriveInput,
        data: &'a D,
        trt: Trait,
        field_type_trait_bounds: FieldBounds<'a>,
        zerocopy_crate: &'a Path,
    ) -> Self {
        Self {
            input,
            data,
            trt,
            field_type_trait_bounds,
            zerocopy_crate,
            self_type_trait_bounds: SelfBounds::None,
            padding_check: None,
            inner_extras: None,
            outer_extras: None,
        }
    }

    fn self_type_trait_bounds(mut self, self_type_trait_bounds: SelfBounds<'a>) -> Self {
        self.self_type_trait_bounds = self_type_trait_bounds;
        self
    }

    fn padding_check<P: Into<Option<PaddingCheck>>>(mut self, padding_check: P) -> Self {
        self.padding_check = padding_check.into();
        self
    }

    fn inner_extras(mut self, inner_extras: TokenStream) -> Self {
        self.inner_extras = Some(inner_extras);
        self
    }

    fn outer_extras<T: Into<Option<TokenStream>>>(mut self, outer_extras: T) -> Self {
        self.outer_extras = outer_extras.into();
        self
    }

    fn build(self) -> TokenStream {
        // In this documentation, we will refer to this hypothetical struct:
        //
        //   #[derive(FromBytes)]
        //   struct Foo<T, I: Iterator>
        //   where
        //       T: Copy,
        //       I: Clone,
        //       I::Item: Clone,
        //   {
        //       a: u8,
        //       b: T,
        //       c: I::Item,
        //   }
        //
        // We extract the field types, which in this case are `u8`, `T`, and
        // `I::Item`. We re-use the existing parameters and where clauses. If
        // `require_trait_bound == true` (as it is for `FromBytes), we add where
        // bounds for each field's type:
        //
        //   impl<T, I: Iterator> FromBytes for Foo<T, I>
        //   where
        //       T: Copy,
        //       I: Clone,
        //       I::Item: Clone,
        //       T: FromBytes,
        //       I::Item: FromBytes,
        //   {
        //   }
        //
        // NOTE: It is standard practice to only emit bounds for the type
        // parameters themselves, not for field types based on those parameters
        // (e.g., `T` vs `T::Foo`). For a discussion of why this is standard
        // practice, see https://github.com/rust-lang/rust/issues/26925.
        //
        // The reason we diverge from this standard is that doing it that way
        // for us would be unsound. E.g., consider a type, `T` where `T:
        // FromBytes` but `T::Foo: !FromBytes`. It would not be sound for us to
        // accept a type with a `T::Foo` field as `FromBytes` simply because `T:
        // FromBytes`.
        //
        // While there's no getting around this requirement for us, it does have
        // the pretty serious downside that, when lifetimes are involved, the
        // trait solver ties itself in knots:
        //
        //     #[derive(Unaligned)]
        //     #[repr(C)]
        //     struct Dup<'a, 'b> {
        //         a: PhantomData<&'a u8>,
        //         b: PhantomData<&'b u8>,
        //     }
        //
        //     error[E0283]: type annotations required: cannot resolve `core::marker::PhantomData<&'a u8>: zerocopy::Unaligned`
        //      --> src/main.rs:6:10
        //       |
        //     6 | #[derive(Unaligned)]
        //       |          ^^^^^^^^^
        //       |
        //       = note: required by `zerocopy::Unaligned`

        let type_ident = &self.input.ident;
        let trait_path = self.trt.crate_path(self.zerocopy_crate);
        let fields = self.data.fields();
        let variants = self.data.variants();
        let tag = self.data.tag();
        let zerocopy_crate = self.zerocopy_crate;

        fn bound_tt(
            ty: &Type,
            traits: impl Iterator<Item = Trait>,
            zerocopy_crate: &Path,
        ) -> WherePredicate {
            let traits = traits.map(|t| t.crate_path(zerocopy_crate));
            parse_quote!(#ty: #(#traits)+*)
        }
        let field_type_bounds: Vec<_> = match (self.field_type_trait_bounds, &fields[..]) {
            (FieldBounds::All(traits), _) => fields
                .iter()
                .map(|(_vis, _name, ty)| {
                    bound_tt(ty, normalize_bounds(self.trt, traits), zerocopy_crate)
                })
                .collect(),
            (FieldBounds::None, _) | (FieldBounds::Trailing(..), []) => vec![],
            (FieldBounds::Trailing(traits), [.., last]) => {
                vec![bound_tt(last.2, normalize_bounds(self.trt, traits), zerocopy_crate)]
            }
            (FieldBounds::Explicit(bounds), _) => bounds,
        };

        // Don't bother emitting a padding check if there are no fields.
        #[allow(unstable_name_collisions)] // See `BoolExt` below
        let padding_check_bound = self
            .padding_check
            .and_then(|check| (!fields.is_empty()).then_some(check))
            .map(|check| {
                let variant_types = variants.iter().map(|var| {
                    let types = var.iter().map(|(_vis, _name, ty)| ty);
                    quote!([#(#types),*])
                });
                let validator_context = check.validator_macro_context();
                let (trt, validator_macro) = check.validator_trait_and_macro_idents();
                let t = tag.iter();
                parse_quote! {
                    (): #zerocopy_crate::util::macro_util::#trt<
                        Self,
                        {
                            #validator_context
                            #zerocopy_crate::#validator_macro!(Self, #(#t,)* #(#variant_types),*)
                        }
                    >
                }
            });

        let self_bounds: Option<WherePredicate> = match self.self_type_trait_bounds {
            SelfBounds::None => None,
            SelfBounds::All(traits) => {
                Some(bound_tt(&parse_quote!(Self), traits.iter().copied(), zerocopy_crate))
            }
        };

        let bounds = self
            .input
            .generics
            .where_clause
            .as_ref()
            .map(|where_clause| where_clause.predicates.iter())
            .into_iter()
            .flatten()
            .chain(field_type_bounds.iter())
            .chain(padding_check_bound.iter())
            .chain(self_bounds.iter());

        // The parameters with trait bounds, but without type defaults.
        let params = self.input.generics.params.clone().into_iter().map(|mut param| {
            match &mut param {
                GenericParam::Type(ty) => ty.default = None,
                GenericParam::Const(cnst) => cnst.default = None,
                GenericParam::Lifetime(_) => {}
            }
            quote!(#param)
        });

        // The identifiers of the parameters without trait bounds or type
        // defaults.
        let param_idents = self.input.generics.params.iter().map(|param| match param {
            GenericParam::Type(ty) => {
                let ident = &ty.ident;
                quote!(#ident)
            }
            GenericParam::Lifetime(l) => {
                let ident = &l.lifetime;
                quote!(#ident)
            }
            GenericParam::Const(cnst) => {
                let ident = &cnst.ident;
                quote!({#ident})
            }
        });

        let inner_extras = self.inner_extras;
        let impl_tokens = quote! {
            // FIXME(#553): Add a test that generates a warning when
            // `#[allow(deprecated)]` isn't present.
            #[allow(deprecated)]
            // While there are not currently any warnings that this suppresses
            // (that we're aware of), it's good future-proofing hygiene.
            #[automatically_derived]
            unsafe impl < #(#params),* > #trait_path for #type_ident < #(#param_idents),* >
            where
                #(#bounds,)*
            {
                fn only_derive_is_allowed_to_implement_this_trait() {}

                #inner_extras
            }
        };

        if let Some(outer_extras) = self.outer_extras {
            // So that any items defined in `#outer_extras` don't conflict with
            // existing names defined in this scope.
            quote! {
                const _: () = {
                    #impl_tokens

                    #outer_extras
                };
            }
        } else {
            impl_tokens
        }
    }
}

// A polyfill for `Option::then_some`, which was added after our MSRV.
//
// The `#[allow(unused)]` is necessary because, on sufficiently recent toolchain
// versions, `b.then_some(...)` resolves to the inherent method rather than to
// this trait, and so this trait is considered unused.
//
// FIXME(#67): Remove this once our MSRV is >= 1.62.
#[allow(unused)]
trait BoolExt {
    fn then_some<T>(self, t: T) -> Option<T>;
}

impl BoolExt for bool {
    fn then_some<T>(self, t: T) -> Option<T> {
        if self {
            Some(t)
        } else {
            None
        }
    }
}
