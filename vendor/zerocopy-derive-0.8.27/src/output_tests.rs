// Copyright 2024 The Fuchsia Authors
//
// Licensed under a BSD-style license <LICENSE-BSD>, Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0>, or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to
// those terms.

use dissimilar::Chunk;
use proc_macro2::TokenStream;

use crate::IntoTokenStream;

macro_rules! use_as_trait_name {
    ($($alias:ident => $derive:ident),* $(,)?) => {
        $(use super::$derive as $alias;)*
    };
}

// This permits invocations of `test!` to be more ergonomic, passing the name of
// the trait under test rather than the name of the inner derive function.
use_as_trait_name!(
    KnownLayout => derive_known_layout_inner,
    Immutable => derive_no_cell_inner,
    TryFromBytes => derive_try_from_bytes_inner,
    FromZeros => derive_from_zeros_inner,
    FromBytes => derive_from_bytes_inner,
    IntoBytes => derive_into_bytes_inner,
    Unaligned => derive_unaligned_inner,
    ByteHash => derive_hash_inner,
    ByteEq => derive_eq_inner,
    SplitAt => derive_split_at_inner,
);

/// Test that the given derive input expands to the expected output.
///
/// Equality is tested by formatting both token streams using `prettyplease` and
/// performing string equality on the results. This has the effect of making the
/// tests less brittle and robust against meaningless formatting changes.
// Adapted from https://github.com/joshlf/synstructure/blob/400499aaf54840056ff56718beb7810540e6be59/src/macros.rs#L212-L317
macro_rules! test {
    ($name:ident { $($i:tt)* } expands to { $($o:tt)* }) => {
        {
            #[allow(dead_code)]
            fn ensure_compiles() {
                $($i)*
                $($o)*
            }

            test!($name { $($i)* } expands to { $($o)* } no_build);
        }
    };

    ($name:ident { $($i:tt)* } expands to { $($o:tt)* } no_build) => {
        {
            let ts: proc_macro2::TokenStream = quote::quote!( $($i)* );
            let ast = syn::parse2::<syn::DeriveInput>(ts).unwrap();
            let res = $name(&ast, crate::Trait::$name, &syn::parse_quote!(::zerocopy));
            let expected_toks = quote::quote!( $($o)* );
            assert_eq_streams(expected_toks.into(), res.into_ts().into());
        }
    };
}

#[track_caller]
fn assert_eq_streams(expect: TokenStream, res: TokenStream) {
    let pretty =
        |ts: TokenStream| prettyplease::unparse(&syn::parse_file(&ts.to_string()).unwrap());

    let expect = pretty(expect.clone());
    let res = pretty(res.clone());
    if expect != res {
        let diff = dissimilar::diff(&expect, &res)
            .into_iter()
            .flat_map(|chunk| {
                let (prefix, chunk) = match chunk {
                    Chunk::Equal(chunk) => (" ", chunk),
                    Chunk::Delete(chunk) => ("-", chunk),
                    Chunk::Insert(chunk) => ("+", chunk),
                };
                [prefix, chunk, "\n"]
            })
            .collect::<String>();

        panic!(
            "\
test failed:
got:
```
{}
```

diff (expected vs got):
```
{}
```\n",
            res, diff
        );
    }
}

#[test]
fn test_known_layout() {
    test! {
        KnownLayout {
            struct Foo;
        } expands to {
            #[allow(deprecated)]
            #[automatically_derived]
            unsafe impl ::zerocopy::KnownLayout for Foo
            where
                Self: ::zerocopy::util::macro_util::core_reexport::marker::Sized,
            {
                fn only_derive_is_allowed_to_implement_this_trait() {}

                type PointerMetadata = ();

                type MaybeUninit = ::zerocopy::util::macro_util::core_reexport::mem::MaybeUninit<Self>;

                const LAYOUT: ::zerocopy::DstLayout = ::zerocopy::DstLayout::for_type::<Self>();

                #[inline(always)]
                fn raw_from_ptr_len(
                    bytes: ::zerocopy::util::macro_util::core_reexport::ptr::NonNull<u8>,
                    _meta: (),
                ) -> ::zerocopy::util::macro_util::core_reexport::ptr::NonNull<Self> {
                    bytes.cast::<Self>()
                }

                #[inline(always)]
                fn pointer_to_metadata(_ptr: *mut Self) -> () {}
            }
        } no_build
    }

    test! {
        KnownLayout {
            #[repr(C, align(2))]
            struct Foo<T, U>(T, U);
        }
        expands to {
            const _: () = {
                #[allow(deprecated)]
                #[automatically_derived]
                unsafe impl<T, U> ::zerocopy::KnownLayout for Foo<T, U>
                where
                    U: ::zerocopy::KnownLayout,
                {
                    fn only_derive_is_allowed_to_implement_this_trait() {}
                    type PointerMetadata = <U as ::zerocopy::KnownLayout>::PointerMetadata;
                    type MaybeUninit = __ZerocopyKnownLayoutMaybeUninit<T, U>;
                    const LAYOUT: ::zerocopy::DstLayout = {
                        use ::zerocopy::util::macro_util::core_reexport::num::NonZeroUsize;
                        use ::zerocopy::{DstLayout, KnownLayout};
                        DstLayout::for_repr_c_struct(
                            ::zerocopy::util::macro_util::core_reexport::num::NonZeroUsize::new(
                                2u32 as usize,
                            ),
                            ::zerocopy::util::macro_util::core_reexport::option::Option::None,
                            &[DstLayout::for_type::<T>(), <U as KnownLayout>::LAYOUT],
                        )
                    };
                    #[inline(always)]
                    fn raw_from_ptr_len(
                        bytes: ::zerocopy::util::macro_util::core_reexport::ptr::NonNull<u8>,
                        meta: Self::PointerMetadata,
                    ) -> ::zerocopy::util::macro_util::core_reexport::ptr::NonNull<Self> {
                        use ::zerocopy::KnownLayout;
                        let trailing = <U as KnownLayout>::raw_from_ptr_len(bytes, meta);
                        let slf = trailing.as_ptr() as *mut Self;
                        unsafe {
                            ::zerocopy::util::macro_util::core_reexport::ptr::NonNull::new_unchecked(
                                slf,
                            )
                        }
                    }
                    #[inline(always)]
                    fn pointer_to_metadata(ptr: *mut Self) -> Self::PointerMetadata {
                        <U>::pointer_to_metadata(ptr as *mut _)
                    }
                }
                #[allow(non_camel_case_types)]
                struct __Zerocopy_Field_0;
                #[allow(non_camel_case_types)]
                struct __Zerocopy_Field_1;
                unsafe impl<T, U> ::zerocopy::util::macro_util::Field<__Zerocopy_Field_0>
                for Foo<T, U> {
                    type Type = T;
                }
                unsafe impl<T, U> ::zerocopy::util::macro_util::Field<__Zerocopy_Field_1>
                for Foo<T, U> {
                    type Type = U;
                }
                #[repr(C)]
                #[repr(align(2))]
                #[doc(hidden)]
                #[allow(private_bounds)]
                struct __ZerocopyKnownLayoutMaybeUninit<T, U>(
                    ::zerocopy::util::macro_util::core_reexport::mem::MaybeUninit<
                        <Foo<T, U> as ::zerocopy::util::macro_util::Field<__Zerocopy_Field_0>>::Type,
                    >,
                    ::zerocopy::util::macro_util::core_reexport::mem::ManuallyDrop<
                        <<Foo<
                            T,
                            U,
                        > as ::zerocopy::util::macro_util::Field<
                            __Zerocopy_Field_1,
                        >>::Type as ::zerocopy::KnownLayout>::MaybeUninit,
                    >,
                )
                where
                    <Foo<
                        T,
                        U,
                    > as ::zerocopy::util::macro_util::Field<
                        __Zerocopy_Field_1,
                    >>::Type: ::zerocopy::KnownLayout;
                unsafe impl<T, U> ::zerocopy::KnownLayout for __ZerocopyKnownLayoutMaybeUninit<T, U>
                where
                    <Foo<
                        T,
                        U,
                    > as ::zerocopy::util::macro_util::Field<
                        __Zerocopy_Field_1,
                    >>::Type: ::zerocopy::KnownLayout,
                {
                    #[allow(clippy::missing_inline_in_public_items)]
                    fn only_derive_is_allowed_to_implement_this_trait() {}
                    type PointerMetadata = <Foo<T, U> as ::zerocopy::KnownLayout>::PointerMetadata;
                    type MaybeUninit = Self;
                    const LAYOUT: ::zerocopy::DstLayout = <Foo<
                        T,
                        U,
                    > as ::zerocopy::KnownLayout>::LAYOUT;
                    #[inline(always)]
                    fn raw_from_ptr_len(
                        bytes: ::zerocopy::util::macro_util::core_reexport::ptr::NonNull<u8>,
                        meta: Self::PointerMetadata,
                    ) -> ::zerocopy::util::macro_util::core_reexport::ptr::NonNull<Self> {
                        use ::zerocopy::KnownLayout;
                        let trailing = <<<Foo<
                            T,
                            U,
                        > as ::zerocopy::util::macro_util::Field<
                            __Zerocopy_Field_1,
                        >>::Type as ::zerocopy::KnownLayout>::MaybeUninit as KnownLayout>::raw_from_ptr_len(
                            bytes,
                            meta,
                        );
                        let slf = trailing.as_ptr() as *mut Self;
                        unsafe {
                            ::zerocopy::util::macro_util::core_reexport::ptr::NonNull::new_unchecked(
                                slf,
                            )
                        }
                    }
                    #[inline(always)]
                    fn pointer_to_metadata(ptr: *mut Self) -> Self::PointerMetadata {
                        <<<Foo<
                            T,
                            U,
                        > as ::zerocopy::util::macro_util::Field<
                            __Zerocopy_Field_1,
                        >>::Type as ::zerocopy::KnownLayout>::MaybeUninit>::pointer_to_metadata(
                            ptr as *mut _,
                        )
                    }
                }
            };
        } no_build
    }
}

#[test]
fn test_immutable() {
    test! {
        Immutable {
            struct Foo;
        } expands to {
            #[allow(deprecated)]
            #[automatically_derived]
            unsafe impl ::zerocopy::Immutable for Foo {
                fn only_derive_is_allowed_to_implement_this_trait() {}
            }
        } no_build
    }
}

#[test]
fn test_try_from_bytes() {
    test! {
        TryFromBytes {
            struct Foo;
        } expands to {
            #[allow(deprecated)]
            #[automatically_derived]
            unsafe impl ::zerocopy::TryFromBytes for Foo {
                fn only_derive_is_allowed_to_implement_this_trait() {}

                fn is_bit_valid<___ZerocopyAliasing>(
                    mut candidate: ::zerocopy::Maybe<Self, ___ZerocopyAliasing>,
                ) -> ::zerocopy::util::macro_util::core_reexport::primitive::bool
                where
                    ___ZerocopyAliasing: ::zerocopy::pointer::invariant::Reference,
                {
                    use ::zerocopy::util::macro_util::core_reexport;
                    use ::zerocopy::pointer::PtrInner;
                    true
                }
            }
        } no_build
    }
}

#[test]
fn test_from_zeros() {
    test! {
        FromZeros {
            struct Foo;
        } expands to {
            #[allow(deprecated)]
            #[automatically_derived]
            unsafe impl ::zerocopy::TryFromBytes for Foo {
                fn only_derive_is_allowed_to_implement_this_trait() {}

                fn is_bit_valid<___ZerocopyAliasing>(
                    mut candidate: ::zerocopy::Maybe<Self, ___ZerocopyAliasing>,
                ) -> ::zerocopy::util::macro_util::core_reexport::primitive::bool
                where
                    ___ZerocopyAliasing: ::zerocopy::pointer::invariant::Reference,
                {
                    use ::zerocopy::util::macro_util::core_reexport;
                    use ::zerocopy::pointer::PtrInner;
                    true
                }
            }

            #[allow(deprecated)]
            #[automatically_derived]
            unsafe impl ::zerocopy::FromZeros for Foo {
                fn only_derive_is_allowed_to_implement_this_trait() {}
            }
        } no_build
    }
}

#[test]
fn test_from_bytes_struct() {
    test! {
        FromBytes {
            struct Foo;
        } expands to {
            #[allow(deprecated)]
            #[automatically_derived]
            unsafe impl ::zerocopy::TryFromBytes for Foo {
                fn only_derive_is_allowed_to_implement_this_trait() {}

                fn is_bit_valid<___ZerocopyAliasing>(
                    _candidate: ::zerocopy::Maybe<Self, ___ZerocopyAliasing>,
                ) -> ::zerocopy::util::macro_util::core_reexport::primitive::bool
                where
                    ___ZerocopyAliasing: ::zerocopy::pointer::invariant::Reference,
                {
                    if false {
                        fn assert_is_from_bytes<T>()
                        where
                            T: ::zerocopy::FromBytes,
                            T: ?::zerocopy::util::macro_util::core_reexport::marker::Sized,
                        {}
                        assert_is_from_bytes::<Self>();
                    }

                    true
                }
            }

            #[allow(deprecated)]
            #[automatically_derived]
            unsafe impl ::zerocopy::FromZeros for Foo {
                fn only_derive_is_allowed_to_implement_this_trait() {}
            }

            #[allow(deprecated)]
            #[automatically_derived]
            unsafe impl ::zerocopy::FromBytes for Foo {
                fn only_derive_is_allowed_to_implement_this_trait() {}
            }
        } no_build
    }
}

#[test]
fn test_from_bytes_union() {
    test! {
        FromBytes {
            union Foo {
                a: u8,
            }
        } expands to {
            #[allow(deprecated)]
            #[automatically_derived]
            unsafe impl ::zerocopy::TryFromBytes for Foo
            where
                u8: ::zerocopy::TryFromBytes + ::zerocopy::Immutable,
            {
                fn only_derive_is_allowed_to_implement_this_trait() {}

                fn is_bit_valid<___ZerocopyAliasing>(
                    _candidate: ::zerocopy::Maybe<Self, ___ZerocopyAliasing>,
                ) -> ::zerocopy::util::macro_util::core_reexport::primitive::bool
                where
                    ___ZerocopyAliasing: ::zerocopy::pointer::invariant::Reference,
                {
                    if false {
                        fn assert_is_from_bytes<T>()
                        where
                            T: ::zerocopy::FromBytes,
                            T: ?::zerocopy::util::macro_util::core_reexport::marker::Sized,
                        {}
                        assert_is_from_bytes::<Self>();
                    }

                    true
                }
            }

            #[allow(deprecated)]
            #[automatically_derived]
            unsafe impl ::zerocopy::FromZeros for Foo
            where
                u8: ::zerocopy::FromZeros + ::zerocopy::Immutable,
            {
                fn only_derive_is_allowed_to_implement_this_trait() {}
            }

            #[allow(deprecated)]
            #[automatically_derived]
            unsafe impl ::zerocopy::FromBytes for Foo
            where
                u8: ::zerocopy::FromBytes + ::zerocopy::Immutable,
            {
                fn only_derive_is_allowed_to_implement_this_trait() {}
            }
        } no_build
    }
}

#[test]
fn test_into_bytes_struct() {
    test! {
        IntoBytes {
            #[repr(C)]
            struct Foo;
        } expands to {
            #[allow(deprecated)]
            #[automatically_derived]
            unsafe impl ::zerocopy::IntoBytes for Foo {
                fn only_derive_is_allowed_to_implement_this_trait() {}
            }
        } no_build
    }

    test! {
        IntoBytes {
            #[repr(C)]
            struct Foo {
                a: u8,
                b: u8,
            }
        } expands to {
            #[allow(deprecated)]
            #[automatically_derived]
            unsafe impl ::zerocopy::IntoBytes for Foo
            where
                u8: ::zerocopy::IntoBytes,
                u8: ::zerocopy::IntoBytes,
                (): ::zerocopy::util::macro_util::PaddingFree<
                    Self,
                    { ::zerocopy::struct_padding!(Self, [u8, u8]) },
                >,
            {
                fn only_derive_is_allowed_to_implement_this_trait() {}
            }
        } no_build
    }

    test! {
        IntoBytes {
            #[repr(C)]
            struct Foo {
                a: u8,
                b: [Trailing],
            }
        } expands to {
            #[allow(deprecated)]
            #[automatically_derived]
            unsafe impl ::zerocopy::IntoBytes for Foo
            where
                u8: ::zerocopy::IntoBytes,
                [Trailing]: ::zerocopy::IntoBytes,
                (): ::zerocopy::util::macro_util::DynamicPaddingFree<
                    Self,
                    { ::zerocopy::repr_c_struct_has_padding!(Self, [u8, [Trailing]]) },
                >,
            {
                fn only_derive_is_allowed_to_implement_this_trait() {}
            }
        } no_build
    }

    test! {
        IntoBytes {
            #[repr(C)]
            struct Foo<Trailing> {
                a: u8,
                b: [Trailing],
            }
        } expands to {
            #[allow(deprecated)]
            #[automatically_derived]
            unsafe impl<Trailing> ::zerocopy::IntoBytes for Foo<Trailing>
            where
                u8: ::zerocopy::IntoBytes + ::zerocopy::Unaligned,
                [Trailing]: ::zerocopy::IntoBytes + ::zerocopy::Unaligned,
            {
                fn only_derive_is_allowed_to_implement_this_trait() {}
            }
        } no_build
    }
}

#[test]
fn test_into_bytes_enum() {
    macro_rules! test_repr {
        ($(#[$attr:meta])*) => {
            $(test! {
                IntoBytes {
                    #[$attr]
                    enum Foo {
                        Bar,
                    }
                } expands to {
                    #[allow(deprecated)]
                    #[automatically_derived]
                    unsafe impl ::zerocopy::IntoBytes for Foo {
                        fn only_derive_is_allowed_to_implement_this_trait() {}
                    }
                } no_build
            })*
        };
    }

    test_repr! {
        #[repr(C)]
        #[repr(u8)]
        #[repr(u16)]
        #[repr(u32)]
        #[repr(u64)]
        #[repr(u128)]
        #[repr(usize)]
        #[repr(i8)]
        #[repr(i16)]
        #[repr(i32)]
        #[repr(i64)]
        #[repr(i128)]
        #[repr(isize)]
    }
}

#[test]
fn test_unaligned() {
    test! {
        Unaligned {
            #[repr(C)]
            struct Foo;
        } expands to {
            #[allow(deprecated)]
            #[automatically_derived]
            unsafe impl ::zerocopy::Unaligned for Foo {
                fn only_derive_is_allowed_to_implement_this_trait() {}
            }
        } no_build
    }
}

#[test]
fn test_try_from_bytes_enum() {
    test! {
        TryFromBytes {
            #[repr(u8)]
            enum ComplexWithGenerics<'a: 'static, const N: usize, X, Y: Deref>
            where
                X: Deref<Target = &'a [(X, Y); N]>,
            {
                UnitLike,
                StructLike { a: u8, b: X, c: X::Target, d: Y::Target, e: [(X, Y); N] },
                TupleLike(bool, Y, PhantomData<&'a [(X, Y); N]>),
            }
        } expands to {
            #[allow(deprecated)]
            #[automatically_derived]
            unsafe impl<'a: 'static, const N: usize, X, Y: Deref> ::zerocopy::TryFromBytes
                for ComplexWithGenerics<'a, { N }, X, Y>
            where
                X: Deref<Target = &'a [(X, Y); N]>,
                u8: ::zerocopy::TryFromBytes,
                X: ::zerocopy::TryFromBytes,
                X::Target: ::zerocopy::TryFromBytes,
                Y::Target: ::zerocopy::TryFromBytes,
                [(X, Y); N]: ::zerocopy::TryFromBytes,
                bool: ::zerocopy::TryFromBytes,
                Y: ::zerocopy::TryFromBytes,
                PhantomData<&'a [(X, Y); N]>: ::zerocopy::TryFromBytes,
            {
                fn only_derive_is_allowed_to_implement_this_trait() {}
                fn is_bit_valid<___ZerocopyAliasing>(
                    mut candidate: ::zerocopy::Maybe<'_, Self, ___ZerocopyAliasing>,
                ) -> ::zerocopy::util::macro_util::core_reexport::primitive::bool
                where
                    ___ZerocopyAliasing: ::zerocopy::pointer::invariant::Reference,
                {
                    use ::zerocopy::util::macro_util::core_reexport;

                    #[repr(u8)]
                    #[allow(dead_code, non_camel_case_types)]
                    enum ___ZerocopyTag {
                        UnitLike,
                        StructLike,
                        TupleLike,
                    }
                    type ___ZerocopyTagPrimitive = ::zerocopy::util::macro_util::SizeToTag<
                        { core_reexport::mem::size_of::<___ZerocopyTag>() },
                    >;
                    #[allow(non_upper_case_globals)]
                    const ___ZEROCOPY_TAG_UnitLike: ___ZerocopyTagPrimitive =
                        ___ZerocopyTag::UnitLike as ___ZerocopyTagPrimitive;
                    #[allow(non_upper_case_globals)]
                    const ___ZEROCOPY_TAG_StructLike: ___ZerocopyTagPrimitive =
                        ___ZerocopyTag::StructLike as ___ZerocopyTagPrimitive;
                    #[allow(non_upper_case_globals)]
                    const ___ZEROCOPY_TAG_TupleLike: ___ZerocopyTagPrimitive =
                        ___ZerocopyTag::TupleLike as ___ZerocopyTagPrimitive;
                    type ___ZerocopyOuterTag = ();
                    type ___ZerocopyInnerTag = ___ZerocopyTag;
                    #[repr(C)]
                    #[allow(non_snake_case)]
                    struct ___ZerocopyVariantStruct_StructLike<'a: 'static, const N: usize, X, Y: Deref>(
                        core_reexport::mem::MaybeUninit<___ZerocopyInnerTag>,
                        u8,
                        X,
                        X::Target,
                        Y::Target,
                        [(X, Y); N],
                        core_reexport::marker::PhantomData<ComplexWithGenerics<'a, N, X, Y>>,
                    )
                    where
                        X: Deref<Target = &'a [(X, Y); N]>;
                    #[allow(deprecated)]
                    #[automatically_derived]
                    unsafe impl<'a: 'static, const N: usize, X, Y: Deref> ::zerocopy::TryFromBytes
                        for ___ZerocopyVariantStruct_StructLike<'a, { N }, X, Y>
                    where
                        X: Deref<Target = &'a [(X, Y); N]>,
                        core_reexport::mem::MaybeUninit<___ZerocopyInnerTag>: ::zerocopy::TryFromBytes,
                        u8: ::zerocopy::TryFromBytes,
                        X: ::zerocopy::TryFromBytes,
                        X::Target: ::zerocopy::TryFromBytes,
                        Y::Target: ::zerocopy::TryFromBytes,
                        [(X, Y); N]: ::zerocopy::TryFromBytes,
                        core_reexport::marker::PhantomData<ComplexWithGenerics<'a, N, X, Y>>:
                            ::zerocopy::TryFromBytes,
                    {
                        fn only_derive_is_allowed_to_implement_this_trait() {}
                        fn is_bit_valid<___ZerocopyAliasing>(
                            mut candidate: ::zerocopy::Maybe<Self, ___ZerocopyAliasing>,
                        ) -> ::zerocopy::util::macro_util::core_reexport::primitive::bool
                        where
                            ___ZerocopyAliasing: ::zerocopy::pointer::invariant::Reference,
                        {
                            use ::zerocopy::util::macro_util::core_reexport;
                            use ::zerocopy::pointer::PtrInner;

                            true && {
                                let field_candidate = unsafe {
                                    let project = |slf: PtrInner<'_, Self>| {
                                        let slf = slf.as_non_null().as_ptr();
                                        let field = core_reexport::ptr::addr_of_mut!((*slf).0);
                                        let ptr = unsafe { core_reexport::ptr::NonNull::new_unchecked(field) };
                                        unsafe { PtrInner::new(ptr) }
                                    };
                                    candidate.reborrow().cast_unsized_unchecked(project)
                                };
                                <core_reexport::mem::MaybeUninit<
                                        ___ZerocopyInnerTag,
                                    > as ::zerocopy::TryFromBytes>::is_bit_valid(field_candidate)
                            } && {
                                let field_candidate = unsafe {
                                    let project = |slf: PtrInner<'_, Self>| {
                                        let slf = slf.as_non_null().as_ptr();
                                        let field = core_reexport::ptr::addr_of_mut!((*slf).1);
                                        let ptr = unsafe { core_reexport::ptr::NonNull::new_unchecked(field) };
                                        unsafe { PtrInner::new(ptr) }
                                    };
                                    candidate.reborrow().cast_unsized_unchecked(project)
                                };
                                <u8 as ::zerocopy::TryFromBytes>::is_bit_valid(field_candidate)
                            } && {
                                let field_candidate = unsafe {
                                    let project = |slf: PtrInner<'_, Self>| {
                                        let slf = slf.as_non_null().as_ptr();
                                        let field = core_reexport::ptr::addr_of_mut!((*slf).2);
                                        let ptr = unsafe { core_reexport::ptr::NonNull::new_unchecked(field) };
                                        unsafe { PtrInner::new(ptr) }
                                    };
                                    candidate.reborrow().cast_unsized_unchecked(project)
                                };
                                <X as ::zerocopy::TryFromBytes>::is_bit_valid(field_candidate)
                            } && {
                                let field_candidate = unsafe {
                                    let project = |slf: PtrInner<'_, Self>| {
                                        let slf = slf.as_non_null().as_ptr();
                                        let field = core_reexport::ptr::addr_of_mut!((*slf).3);
                                        let ptr = unsafe { core_reexport::ptr::NonNull::new_unchecked(field) };
                                        unsafe { PtrInner::new(ptr) }
                                    };
                                    candidate.reborrow().cast_unsized_unchecked(project)
                                };
                                <X::Target as ::zerocopy::TryFromBytes>::is_bit_valid(field_candidate)
                            } && {
                                let field_candidate = unsafe {
                                    let project = |slf: PtrInner<'_, Self>| {
                                        let slf = slf.as_non_null().as_ptr();
                                        let field = core_reexport::ptr::addr_of_mut!((*slf).4);
                                        let ptr = unsafe { core_reexport::ptr::NonNull::new_unchecked(field) };
                                        unsafe { PtrInner::new(ptr) }
                                    };
                                    candidate.reborrow().cast_unsized_unchecked(project)
                                };
                                <Y::Target as ::zerocopy::TryFromBytes>::is_bit_valid(field_candidate)
                            } && {
                                let field_candidate = unsafe {
                                    let project = |slf: PtrInner<'_, Self>| {
                                        let slf = slf.as_non_null().as_ptr();
                                        let field = core_reexport::ptr::addr_of_mut!((*slf).5);
                                        let ptr = unsafe { core_reexport::ptr::NonNull::new_unchecked(field) };
                                        unsafe { PtrInner::new(ptr) }
                                    };
                                    candidate.reborrow().cast_unsized_unchecked(project)
                                };
                                <[(X, Y); N] as ::zerocopy::TryFromBytes>::is_bit_valid(field_candidate)
                            } && {
                                let field_candidate = unsafe {
                                    let project = |slf: PtrInner<'_, Self>| {
                                        let slf = slf.as_non_null().as_ptr();
                                        let field = core_reexport::ptr::addr_of_mut!((*slf).6);
                                        let ptr = unsafe { core_reexport::ptr::NonNull::new_unchecked(field) };
                                        unsafe { PtrInner::new(ptr) }
                                    };
                                    candidate.reborrow().cast_unsized_unchecked(project)
                                };
                                <core_reexport::marker::PhantomData<
                                        ComplexWithGenerics<'a, N, X, Y>,
                                    > as ::zerocopy::TryFromBytes>::is_bit_valid(field_candidate)
                            }
                        }
                    }
                    #[repr(C)]
                    #[allow(non_snake_case)]
                    struct ___ZerocopyVariantStruct_TupleLike<'a: 'static, const N: usize, X, Y: Deref>(
                        core_reexport::mem::MaybeUninit<___ZerocopyInnerTag>,
                        bool,
                        Y,
                        PhantomData<&'a [(X, Y); N]>,
                        core_reexport::marker::PhantomData<ComplexWithGenerics<'a, N, X, Y>>,
                    )
                    where
                        X: Deref<Target = &'a [(X, Y); N]>;
                    #[allow(deprecated)]
                    #[automatically_derived]
                    unsafe impl<'a: 'static, const N: usize, X, Y: Deref> ::zerocopy::TryFromBytes
                        for ___ZerocopyVariantStruct_TupleLike<'a, { N }, X, Y>
                    where
                        X: Deref<Target = &'a [(X, Y); N]>,
                        core_reexport::mem::MaybeUninit<___ZerocopyInnerTag>: ::zerocopy::TryFromBytes,
                        bool: ::zerocopy::TryFromBytes,
                        Y: ::zerocopy::TryFromBytes,
                        PhantomData<&'a [(X, Y); N]>: ::zerocopy::TryFromBytes,
                        core_reexport::marker::PhantomData<ComplexWithGenerics<'a, N, X, Y>>:
                            ::zerocopy::TryFromBytes,
                    {
                        fn only_derive_is_allowed_to_implement_this_trait() {}
                        fn is_bit_valid<___ZerocopyAliasing>(
                            mut candidate: ::zerocopy::Maybe<Self, ___ZerocopyAliasing>,
                        ) -> ::zerocopy::util::macro_util::core_reexport::primitive::bool
                        where
                            ___ZerocopyAliasing: ::zerocopy::pointer::invariant::Reference,
                        {
                            use ::zerocopy::util::macro_util::core_reexport;
                            use ::zerocopy::pointer::PtrInner;

                            true && {
                                let field_candidate = unsafe {
                                    let project = |slf: PtrInner<'_, Self>| {
                                        let slf = slf.as_non_null().as_ptr();
                                        let field = core_reexport::ptr::addr_of_mut!((*slf).0);
                                        let ptr = unsafe { core_reexport::ptr::NonNull::new_unchecked(field) };
                                        unsafe { PtrInner::new(ptr) }
                                    };
                                    candidate.reborrow().cast_unsized_unchecked(project)
                                };
                                <core_reexport::mem::MaybeUninit<
                                        ___ZerocopyInnerTag,
                                    > as ::zerocopy::TryFromBytes>::is_bit_valid(field_candidate)
                            } && {
                                let field_candidate = unsafe {
                                    let project = |slf: PtrInner<'_, Self>| {
                                        let slf = slf.as_non_null().as_ptr();
                                        let field = core_reexport::ptr::addr_of_mut!((*slf).1);
                                        let ptr = unsafe { core_reexport::ptr::NonNull::new_unchecked(field) };
                                        unsafe { PtrInner::new(ptr) }
                                    };
                                    candidate.reborrow().cast_unsized_unchecked(project)
                                };
                                <bool as ::zerocopy::TryFromBytes>::is_bit_valid(field_candidate)
                            } && {
                                let field_candidate = unsafe {
                                    let project = |slf: PtrInner<'_, Self>| {
                                        let slf = slf.as_non_null().as_ptr();
                                        let field = core_reexport::ptr::addr_of_mut!((*slf).2);
                                        let ptr = unsafe { core_reexport::ptr::NonNull::new_unchecked(field) };
                                        unsafe { PtrInner::new(ptr) }
                                    };
                                    candidate.reborrow().cast_unsized_unchecked(project)
                                };
                                <Y as ::zerocopy::TryFromBytes>::is_bit_valid(field_candidate)
                            } && {
                                let field_candidate = unsafe {
                                    let project = |slf: PtrInner<'_, Self>| {
                                        let slf = slf.as_non_null().as_ptr();
                                        let field = core_reexport::ptr::addr_of_mut!((*slf).3);
                                        let ptr = unsafe { core_reexport::ptr::NonNull::new_unchecked(field) };
                                        unsafe { PtrInner::new(ptr) }
                                    };
                                    candidate.reborrow().cast_unsized_unchecked(project)
                                };
                                <PhantomData<&'a [(X, Y); N]> as ::zerocopy::TryFromBytes>::is_bit_valid(
                                    field_candidate,
                                )
                            } && {
                                let field_candidate = unsafe {
                                    let project = |slf: PtrInner<'_, Self>| {
                                        let slf = slf.as_non_null().as_ptr();
                                        let field = core_reexport::ptr::addr_of_mut!((*slf).4);
                                        let ptr = unsafe { core_reexport::ptr::NonNull::new_unchecked(field) };
                                        unsafe { PtrInner::new(ptr) }
                                    };
                                    candidate.reborrow().cast_unsized_unchecked(project)
                                };
                                <core_reexport::marker::PhantomData<
                                        ComplexWithGenerics<'a, N, X, Y>,
                                    > as ::zerocopy::TryFromBytes>::is_bit_valid(field_candidate)
                            }
                        }
                    }
                    #[repr(C)]
                    #[allow(non_snake_case)]
                    union ___ZerocopyVariants<'a: 'static, const N: usize, X, Y: Deref> {
                        __field_StructLike:
                            core_reexport::mem::ManuallyDrop<___ZerocopyVariantStruct_StructLike<'a, N, X, Y>>,
                        __field_TupleLike:
                            core_reexport::mem::ManuallyDrop<___ZerocopyVariantStruct_TupleLike<'a, N, X, Y>>,
                        __nonempty: (),
                    }
                    #[repr(C)]
                    struct ___ZerocopyRawEnum<'a: 'static, const N: usize, X, Y: Deref> {
                        tag: ___ZerocopyOuterTag,
                        variants: ___ZerocopyVariants<'a, N, X, Y>,
                    }
                    let tag = {
                        let tag_ptr = unsafe {
                            candidate.reborrow().cast_unsized_unchecked(|p: ::zerocopy::pointer::PtrInner<'_, Self>| { p.cast_sized::<___ZerocopyTagPrimitive>() })
                        };
                        let tag_ptr = unsafe { tag_ptr.assume_initialized() };
                        tag_ptr.recall_validity::<_, (_, (_, _))>().read_unaligned::<::zerocopy::BecauseImmutable>()
                    };
                    let raw_enum = unsafe {
                        candidate.cast_unsized_unchecked(|p: ::zerocopy::pointer::PtrInner<'_, Self>| { p.cast_sized::<___ZerocopyRawEnum<'a, N, X, Y>>() })
                    };
                    let raw_enum = unsafe { raw_enum.assume_initialized() };
                    let variants = unsafe {
                        use ::zerocopy::pointer::PtrInner;
                        raw_enum.cast_unsized_unchecked(|p: PtrInner<'_, ___ZerocopyRawEnum<'a, N, X, Y>>| {
                            let p = p.as_non_null().as_ptr();
                            let ptr = core_reexport::ptr::addr_of_mut!((*p).variants);
                            let ptr = unsafe { core_reexport::ptr::NonNull::new_unchecked(ptr) };
                            unsafe { PtrInner::new(ptr) }
                        })
                    };
                    #[allow(non_upper_case_globals)]
                    match tag {
                        ___ZEROCOPY_TAG_UnitLike => true,
                        ___ZEROCOPY_TAG_StructLike => {
                            let variant = unsafe {
                                variants.cast_unsized_unchecked(|p: ::zerocopy::pointer::PtrInner<'_, ___ZerocopyVariants<'a, N, X, Y>>| {
                                    p.cast_sized::<___ZerocopyVariantStruct_StructLike<'a, N, X, Y>>()
                                })
                            };
                            let variant = unsafe { variant.assume_initialized() };
                        <___ZerocopyVariantStruct_StructLike<'a, N, X, Y> as ::zerocopy ::TryFromBytes>::is_bit_valid (
                                            variant)
                        }
                        ___ZEROCOPY_TAG_TupleLike => {
                            let variant = unsafe {
                                variants.cast_unsized_unchecked(|p: ::zerocopy::pointer::PtrInner<'_, ___ZerocopyVariants<'a, N, X, Y>>| {
                                    p.cast_sized::<___ZerocopyVariantStruct_TupleLike<'a, N, X, Y>>()
                                })
                            };
                            let variant = unsafe { variant.assume_initialized() };
                        <___ZerocopyVariantStruct_TupleLike<'a, N, X, Y> as ::zerocopy ::TryFromBytes>::is_bit_valid (
                                            variant)
                        }
                        _ => false,
                    }
                }
            }
        } no_build
    }

    test! {
        TryFromBytes {
            #[repr(u32)]
            enum ComplexWithGenerics<'a: 'static, const N: usize, X, Y: Deref>
            where
                X: Deref<Target = &'a [(X, Y); N]>,
            {
                UnitLike,
                StructLike { a: u8, b: X, c: X::Target, d: Y::Target, e: [(X, Y); N] },
                TupleLike(bool, Y, PhantomData<&'a [(X, Y); N]>),
            }
        } expands to {
            #[allow(deprecated)]
            #[automatically_derived]
            unsafe impl<'a: 'static, const N: usize, X, Y: Deref> ::zerocopy::TryFromBytes
                for ComplexWithGenerics<'a, { N }, X, Y>
            where
                X: Deref<Target = &'a [(X, Y); N]>,
                u8: ::zerocopy::TryFromBytes,
                X: ::zerocopy::TryFromBytes,
                X::Target: ::zerocopy::TryFromBytes,
                Y::Target: ::zerocopy::TryFromBytes,
                [(X, Y); N]: ::zerocopy::TryFromBytes,
                bool: ::zerocopy::TryFromBytes,
                Y: ::zerocopy::TryFromBytes,
                PhantomData<&'a [(X, Y); N]>: ::zerocopy::TryFromBytes,
            {
                fn only_derive_is_allowed_to_implement_this_trait() {}
                fn is_bit_valid<___ZerocopyAliasing>(
                    mut candidate: ::zerocopy::Maybe<'_, Self, ___ZerocopyAliasing>,
                ) -> ::zerocopy::util::macro_util::core_reexport::primitive::bool
                where
                    ___ZerocopyAliasing: ::zerocopy::pointer::invariant::Reference,
                {
                    use ::zerocopy::util::macro_util::core_reexport;

                    #[repr(u32)]
                    #[allow(dead_code, non_camel_case_types)]
                    enum ___ZerocopyTag {
                        UnitLike,
                        StructLike,
                        TupleLike,
                    }
                    type ___ZerocopyTagPrimitive = ::zerocopy::util::macro_util::SizeToTag<
                        { core_reexport::mem::size_of::<___ZerocopyTag>() },
                    >;
                    #[allow(non_upper_case_globals)]
                    const ___ZEROCOPY_TAG_UnitLike: ___ZerocopyTagPrimitive =
                        ___ZerocopyTag::UnitLike as ___ZerocopyTagPrimitive;
                    #[allow(non_upper_case_globals)]
                    const ___ZEROCOPY_TAG_StructLike: ___ZerocopyTagPrimitive =
                        ___ZerocopyTag::StructLike as ___ZerocopyTagPrimitive;
                    #[allow(non_upper_case_globals)]
                    const ___ZEROCOPY_TAG_TupleLike: ___ZerocopyTagPrimitive =
                        ___ZerocopyTag::TupleLike as ___ZerocopyTagPrimitive;
                    type ___ZerocopyOuterTag = ();
                    type ___ZerocopyInnerTag = ___ZerocopyTag;
                    #[repr(C)]
                    #[allow(non_snake_case)]
                    struct ___ZerocopyVariantStruct_StructLike<'a: 'static, const N: usize, X, Y: Deref>(
                        core_reexport::mem::MaybeUninit<___ZerocopyInnerTag>,
                        u8,
                        X,
                        X::Target,
                        Y::Target,
                        [(X, Y); N],
                        core_reexport::marker::PhantomData<ComplexWithGenerics<'a, N, X, Y>>,
                    )
                    where
                        X: Deref<Target = &'a [(X, Y); N]>;
                    #[allow(deprecated)]
                    #[automatically_derived]
                    unsafe impl<'a: 'static, const N: usize, X, Y: Deref> ::zerocopy::TryFromBytes
                        for ___ZerocopyVariantStruct_StructLike<'a, { N }, X, Y>
                    where
                        X: Deref<Target = &'a [(X, Y); N]>,
                        core_reexport::mem::MaybeUninit<___ZerocopyInnerTag>: ::zerocopy::TryFromBytes,
                        u8: ::zerocopy::TryFromBytes,
                        X: ::zerocopy::TryFromBytes,
                        X::Target: ::zerocopy::TryFromBytes,
                        Y::Target: ::zerocopy::TryFromBytes,
                        [(X, Y); N]: ::zerocopy::TryFromBytes,
                        core_reexport::marker::PhantomData<ComplexWithGenerics<'a, N, X, Y>>:
                            ::zerocopy::TryFromBytes,
                    {
                        fn only_derive_is_allowed_to_implement_this_trait() {}
                        fn is_bit_valid<___ZerocopyAliasing>(
                            mut candidate: ::zerocopy::Maybe<Self, ___ZerocopyAliasing>,
                        ) -> ::zerocopy::util::macro_util::core_reexport::primitive::bool
                        where
                            ___ZerocopyAliasing: ::zerocopy::pointer::invariant::Reference,
                        {
                            use ::zerocopy::util::macro_util::core_reexport;
                            use ::zerocopy::pointer::PtrInner;

                            true && {
                                let field_candidate = unsafe {
                                    let project = |slf: PtrInner<'_, Self>| {
                                        let slf = slf.as_non_null().as_ptr();
                                        let field = core_reexport::ptr::addr_of_mut!((*slf).0);
                                        let ptr = unsafe { core_reexport::ptr::NonNull::new_unchecked(field) };
                                        unsafe { PtrInner::new(ptr) }
                                    };
                                    candidate.reborrow().cast_unsized_unchecked(project)
                                };
                                <core_reexport::mem::MaybeUninit<
                                        ___ZerocopyInnerTag,
                                    > as ::zerocopy::TryFromBytes>::is_bit_valid(field_candidate)
                            } && {
                                let field_candidate = unsafe {
                                    let project = |slf: PtrInner<'_, Self>| {
                                        let slf = slf.as_non_null().as_ptr();
                                        let field = core_reexport::ptr::addr_of_mut!((*slf).1);
                                        let ptr = unsafe { core_reexport::ptr::NonNull::new_unchecked(field) };
                                        unsafe { PtrInner::new(ptr) }
                                    };
                                    candidate.reborrow().cast_unsized_unchecked(project)
                                };
                                <u8 as ::zerocopy::TryFromBytes>::is_bit_valid(field_candidate)
                            } && {
                                let field_candidate = unsafe {
                                    let project = |slf: PtrInner<'_, Self>| {
                                        let slf = slf.as_non_null().as_ptr();
                                        let field = core_reexport::ptr::addr_of_mut!((*slf).2);
                                        let ptr = unsafe { core_reexport::ptr::NonNull::new_unchecked(field) };
                                        unsafe { PtrInner::new(ptr) }
                                    };
                                    candidate.reborrow().cast_unsized_unchecked(project)
                                };
                                <X as ::zerocopy::TryFromBytes>::is_bit_valid(field_candidate)
                            } && {
                                let field_candidate = unsafe {
                                    let project = |slf: PtrInner<'_, Self>| {
                                        let slf = slf.as_non_null().as_ptr();
                                        let field = core_reexport::ptr::addr_of_mut!((*slf).3);
                                        let ptr = unsafe { core_reexport::ptr::NonNull::new_unchecked(field) };
                                        unsafe { PtrInner::new(ptr) }
                                    };
                                    candidate.reborrow().cast_unsized_unchecked(project)
                                };
                                <X::Target as ::zerocopy::TryFromBytes>::is_bit_valid(field_candidate)
                            } && {
                                let field_candidate = unsafe {
                                    let project = |slf: PtrInner<'_, Self>| {
                                        let slf = slf.as_non_null().as_ptr();
                                        let field = core_reexport::ptr::addr_of_mut!((*slf).4);
                                        let ptr = unsafe { core_reexport::ptr::NonNull::new_unchecked(field) };
                                        unsafe { PtrInner::new(ptr) }
                                    };
                                    candidate.reborrow().cast_unsized_unchecked(project)
                                };
                                <Y::Target as ::zerocopy::TryFromBytes>::is_bit_valid(field_candidate)
                            } && {
                                let field_candidate = unsafe {
                                    let project = |slf: PtrInner<'_, Self>| {
                                        let slf = slf.as_non_null().as_ptr();
                                        let field = core_reexport::ptr::addr_of_mut!((*slf).5);
                                        let ptr = unsafe { core_reexport::ptr::NonNull::new_unchecked(field) };
                                        unsafe { PtrInner::new(ptr) }
                                    };
                                    candidate.reborrow().cast_unsized_unchecked(project)
                                };
                                <[(X, Y); N] as ::zerocopy::TryFromBytes>::is_bit_valid(field_candidate)
                            } && {
                                let field_candidate = unsafe {
                                    let project = |slf: PtrInner<'_, Self>| {
                                        let slf = slf.as_non_null().as_ptr();
                                        let field = core_reexport::ptr::addr_of_mut!((*slf).6);
                                        let ptr = unsafe { core_reexport::ptr::NonNull::new_unchecked(field) };
                                        unsafe { PtrInner::new(ptr) }
                                    };
                                    candidate.reborrow().cast_unsized_unchecked(project)
                                };
                                <core_reexport::marker::PhantomData<
                                        ComplexWithGenerics<'a, N, X, Y>,
                                    > as ::zerocopy::TryFromBytes>::is_bit_valid(field_candidate)
                            }
                        }
                    }
                    #[repr(C)]
                    #[allow(non_snake_case)]
                    struct ___ZerocopyVariantStruct_TupleLike<'a: 'static, const N: usize, X, Y: Deref>(
                        core_reexport::mem::MaybeUninit<___ZerocopyInnerTag>,
                        bool,
                        Y,
                        PhantomData<&'a [(X, Y); N]>,
                        core_reexport::marker::PhantomData<ComplexWithGenerics<'a, N, X, Y>>,
                    )
                    where
                        X: Deref<Target = &'a [(X, Y); N]>;
                    #[allow(deprecated)]
                    #[automatically_derived]
                    unsafe impl<'a: 'static, const N: usize, X, Y: Deref> ::zerocopy::TryFromBytes
                        for ___ZerocopyVariantStruct_TupleLike<'a, { N }, X, Y>
                    where
                        X: Deref<Target = &'a [(X, Y); N]>,
                        core_reexport::mem::MaybeUninit<___ZerocopyInnerTag>: ::zerocopy::TryFromBytes,
                        bool: ::zerocopy::TryFromBytes,
                        Y: ::zerocopy::TryFromBytes,
                        PhantomData<&'a [(X, Y); N]>: ::zerocopy::TryFromBytes,
                        core_reexport::marker::PhantomData<ComplexWithGenerics<'a, N, X, Y>>:
                            ::zerocopy::TryFromBytes,
                    {
                        fn only_derive_is_allowed_to_implement_this_trait() {}
                        fn is_bit_valid<___ZerocopyAliasing>(
                            mut candidate: ::zerocopy::Maybe<Self, ___ZerocopyAliasing>,
                        ) -> ::zerocopy::util::macro_util::core_reexport::primitive::bool
                        where
                            ___ZerocopyAliasing: ::zerocopy::pointer::invariant::Reference,
                        {
                            use ::zerocopy::util::macro_util::core_reexport;
                            use ::zerocopy::pointer::PtrInner;

                            true && {
                                let field_candidate = unsafe {
                                    let project = |slf: PtrInner<'_, Self>| {
                                        let slf = slf.as_non_null().as_ptr();
                                        let field = core_reexport::ptr::addr_of_mut!((*slf).0);
                                        let ptr = unsafe { core_reexport::ptr::NonNull::new_unchecked(field) };
                                        unsafe { PtrInner::new(ptr) }
                                    };
                                    candidate.reborrow().cast_unsized_unchecked(project)
                                };
                                <core_reexport::mem::MaybeUninit<
                                        ___ZerocopyInnerTag,
                                    > as ::zerocopy::TryFromBytes>::is_bit_valid(field_candidate)
                            } && {
                                let field_candidate = unsafe {
                                    let project = |slf: PtrInner<'_, Self>| {
                                        let slf = slf.as_non_null().as_ptr();
                                        let field = core_reexport::ptr::addr_of_mut!((*slf).1);
                                        let ptr = unsafe { core_reexport::ptr::NonNull::new_unchecked(field) };
                                        unsafe { PtrInner::new(ptr) }
                                    };
                                    candidate.reborrow().cast_unsized_unchecked(project)
                                };
                                <bool as ::zerocopy::TryFromBytes>::is_bit_valid(field_candidate)
                            } && {
                                let field_candidate = unsafe {
                                    let project = |slf: PtrInner<'_, Self>| {
                                        let slf = slf.as_non_null().as_ptr();
                                        let field = core_reexport::ptr::addr_of_mut!((*slf).2);
                                        let ptr = unsafe { core_reexport::ptr::NonNull::new_unchecked(field) };
                                        unsafe { PtrInner::new(ptr) }
                                    };
                                    candidate.reborrow().cast_unsized_unchecked(project)
                                };
                                <Y as ::zerocopy::TryFromBytes>::is_bit_valid(field_candidate)
                            } && {
                                let field_candidate = unsafe {
                                    let project = |slf: PtrInner<'_, Self>| {
                                        let slf = slf.as_non_null().as_ptr();
                                        let field = core_reexport::ptr::addr_of_mut!((*slf).3);
                                        let ptr = unsafe { core_reexport::ptr::NonNull::new_unchecked(field) };
                                        unsafe { PtrInner::new(ptr) }
                                    };
                                    candidate.reborrow().cast_unsized_unchecked(project)
                                };
                                <PhantomData<&'a [(X, Y); N]> as ::zerocopy::TryFromBytes>::is_bit_valid(
                                    field_candidate,
                                )
                            } && {
                                let field_candidate = unsafe {
                                    let project = |slf: PtrInner<'_, Self>| {
                                        let slf = slf.as_non_null().as_ptr();
                                        let field = core_reexport::ptr::addr_of_mut!((*slf).4);
                                        let ptr = unsafe { core_reexport::ptr::NonNull::new_unchecked(field) };
                                        unsafe { PtrInner::new(ptr) }
                                    };
                                    candidate.reborrow().cast_unsized_unchecked(project)
                                };
                                <core_reexport::marker::PhantomData<
                                        ComplexWithGenerics<'a, N, X, Y>,
                                    > as ::zerocopy::TryFromBytes>::is_bit_valid(field_candidate)
                            }
                        }
                    }
                    #[repr(C)]
                    #[allow(non_snake_case)]
                    union ___ZerocopyVariants<'a: 'static, const N: usize, X, Y: Deref> {
                        __field_StructLike:
                            core_reexport::mem::ManuallyDrop<___ZerocopyVariantStruct_StructLike<'a, N, X, Y>>,
                        __field_TupleLike:
                            core_reexport::mem::ManuallyDrop<___ZerocopyVariantStruct_TupleLike<'a, N, X, Y>>,
                        __nonempty: (),
                    }
                    #[repr(C)]
                    struct ___ZerocopyRawEnum<'a: 'static, const N: usize, X, Y: Deref> {
                        tag: ___ZerocopyOuterTag,
                        variants: ___ZerocopyVariants<'a, N, X, Y>,
                    }
                    let tag = {
                        let tag_ptr = unsafe {
                            candidate.reborrow().cast_unsized_unchecked(|p: ::zerocopy::pointer::PtrInner<'_, Self>| { p.cast_sized::<___ZerocopyTagPrimitive> ()})
                        };
                        let tag_ptr = unsafe { tag_ptr.assume_initialized() };
                        tag_ptr.recall_validity::<_, (_, (_, _))>().read_unaligned::<::zerocopy::BecauseImmutable>()
                    };
                    let raw_enum = unsafe {
                        candidate.cast_unsized_unchecked(|p: ::zerocopy::pointer::PtrInner<'_, Self>| { p.cast_sized::<___ZerocopyRawEnum<'a, N, X, Y>> ()})
                    };
                    let raw_enum = unsafe { raw_enum.assume_initialized() };
                    let variants = unsafe {
                        use ::zerocopy::pointer::PtrInner;
                        raw_enum.cast_unsized_unchecked(|p: PtrInner<'_, ___ZerocopyRawEnum<'a, N, X, Y>>| {
                            let p = p.as_non_null().as_ptr();
                            let ptr = core_reexport::ptr::addr_of_mut!((*p).variants);
                            let ptr = unsafe { core_reexport::ptr::NonNull::new_unchecked(ptr) };
                            unsafe { PtrInner::new(ptr) }
                        })
                    };
                    #[allow(non_upper_case_globals)]
                    match tag {
                        ___ZEROCOPY_TAG_UnitLike => true,
                        ___ZEROCOPY_TAG_StructLike => {
                            let variant = unsafe {
                                variants.cast_unsized_unchecked(|p: ::zerocopy::pointer::PtrInner<'_, ___ZerocopyVariants<'a, N, X, Y>>| {
                                    p.cast_sized::<___ZerocopyVariantStruct_StructLike<'a, N, X, Y>>()
                                })
                            };
                            let variant = unsafe { variant.assume_initialized() };
                        <___ZerocopyVariantStruct_StructLike<'a, N, X, Y> as ::zerocopy ::TryFromBytes>::is_bit_valid (
                                            variant)
                        }
                        ___ZEROCOPY_TAG_TupleLike => {
                            let variant = unsafe {
                                variants.cast_unsized_unchecked(|p: ::zerocopy::pointer::PtrInner<'_, ___ZerocopyVariants<'a, N, X, Y>>| {
                                    p.cast_sized::<___ZerocopyVariantStruct_TupleLike<'a, N, X, Y>>()
                                })
                            };
                            let variant = unsafe { variant.assume_initialized() };
                        <___ZerocopyVariantStruct_TupleLike<'a, N, X, Y> as ::zerocopy ::TryFromBytes>::is_bit_valid (
                                            variant)
                        }
                        _ => false,
                    }
                }
            }
        } no_build
    }

    test! {
        TryFromBytes {
            #[repr(C)]
            enum ComplexWithGenerics<'a: 'static, const N: usize, X, Y: Deref>
            where
                X: Deref<Target = &'a [(X, Y); N]>,
            {
                UnitLike,
                StructLike { a: u8, b: X, c: X::Target, d: Y::Target, e: [(X, Y); N] },
                TupleLike(bool, Y, PhantomData<&'a [(X, Y); N]>),
            }
        } expands to {
            #[allow(deprecated)]
            #[automatically_derived]
            unsafe impl<'a: 'static, const N: usize, X, Y: Deref> ::zerocopy::TryFromBytes
                for ComplexWithGenerics<'a, { N }, X, Y>
            where
                X: Deref<Target = &'a [(X, Y); N]>,
                u8: ::zerocopy::TryFromBytes,
                X: ::zerocopy::TryFromBytes,
                X::Target: ::zerocopy::TryFromBytes,
                Y::Target: ::zerocopy::TryFromBytes,
                [(X, Y); N]: ::zerocopy::TryFromBytes,
                bool: ::zerocopy::TryFromBytes,
                Y: ::zerocopy::TryFromBytes,
                PhantomData<&'a [(X, Y); N]>: ::zerocopy::TryFromBytes,
            {
                fn only_derive_is_allowed_to_implement_this_trait() {}
                fn is_bit_valid<___ZerocopyAliasing>(
                    mut candidate: ::zerocopy::Maybe<'_, Self, ___ZerocopyAliasing>,
                ) -> ::zerocopy::util::macro_util::core_reexport::primitive::bool
                where
                    ___ZerocopyAliasing: ::zerocopy::pointer::invariant::Reference,
                {
                    use ::zerocopy::util::macro_util::core_reexport;

                    #[repr(C)]
                    #[allow(dead_code, non_camel_case_types)]
                    enum ___ZerocopyTag {
                        UnitLike,
                        StructLike,
                        TupleLike,
                    }
                    type ___ZerocopyTagPrimitive = ::zerocopy::util::macro_util::SizeToTag<
                        { core_reexport::mem::size_of::<___ZerocopyTag>() },
                    >;
                    #[allow(non_upper_case_globals)]
                    const ___ZEROCOPY_TAG_UnitLike: ___ZerocopyTagPrimitive =
                        ___ZerocopyTag::UnitLike as ___ZerocopyTagPrimitive;
                    #[allow(non_upper_case_globals)]
                    const ___ZEROCOPY_TAG_StructLike: ___ZerocopyTagPrimitive =
                        ___ZerocopyTag::StructLike as ___ZerocopyTagPrimitive;
                    #[allow(non_upper_case_globals)]
                    const ___ZEROCOPY_TAG_TupleLike: ___ZerocopyTagPrimitive =
                        ___ZerocopyTag::TupleLike as ___ZerocopyTagPrimitive;
                    type ___ZerocopyOuterTag = ___ZerocopyTag;
                    type ___ZerocopyInnerTag = ();
                    #[repr(C)]
                    #[allow(non_snake_case)]
                    struct ___ZerocopyVariantStruct_StructLike<'a: 'static, const N: usize, X, Y: Deref>(
                        core_reexport::mem::MaybeUninit<___ZerocopyInnerTag>,
                        u8,
                        X,
                        X::Target,
                        Y::Target,
                        [(X, Y); N],
                        core_reexport::marker::PhantomData<ComplexWithGenerics<'a, N, X, Y>>,
                    )
                    where
                        X: Deref<Target = &'a [(X, Y); N]>;
                    #[allow(deprecated)]
                    #[automatically_derived]
                    unsafe impl<'a: 'static, const N: usize, X, Y: Deref> ::zerocopy::TryFromBytes
                        for ___ZerocopyVariantStruct_StructLike<'a, { N }, X, Y>
                    where
                        X: Deref<Target = &'a [(X, Y); N]>,
                        core_reexport::mem::MaybeUninit<___ZerocopyInnerTag>: ::zerocopy::TryFromBytes,
                        u8: ::zerocopy::TryFromBytes,
                        X: ::zerocopy::TryFromBytes,
                        X::Target: ::zerocopy::TryFromBytes,
                        Y::Target: ::zerocopy::TryFromBytes,
                        [(X, Y); N]: ::zerocopy::TryFromBytes,
                        core_reexport::marker::PhantomData<ComplexWithGenerics<'a, N, X, Y>>:
                            ::zerocopy::TryFromBytes,
                    {
                        fn only_derive_is_allowed_to_implement_this_trait() {}
                        fn is_bit_valid<___ZerocopyAliasing>(
                            mut candidate: ::zerocopy::Maybe<Self, ___ZerocopyAliasing>,
                        ) -> ::zerocopy::util::macro_util::core_reexport::primitive::bool
                        where
                            ___ZerocopyAliasing: ::zerocopy::pointer::invariant::Reference,
                        {
                            use ::zerocopy::util::macro_util::core_reexport;
                            use ::zerocopy::pointer::PtrInner;

                            true && {
                                let field_candidate = unsafe {
                                    let project = |slf: PtrInner<'_, Self>| {
                                        let slf = slf.as_non_null().as_ptr();
                                        let field = core_reexport::ptr::addr_of_mut!((*slf).0);
                                        let ptr = unsafe { core_reexport::ptr::NonNull::new_unchecked(field) };
                                        unsafe { PtrInner::new(ptr) }
                                    };
                                    candidate.reborrow().cast_unsized_unchecked(project)
                                };
                                <core_reexport::mem::MaybeUninit<
                                        ___ZerocopyInnerTag,
                                    > as ::zerocopy::TryFromBytes>::is_bit_valid(field_candidate)
                            } && {
                                let field_candidate = unsafe {
                                    let project = |slf: PtrInner<'_, Self>| {
                                        let slf = slf.as_non_null().as_ptr();
                                        let field = core_reexport::ptr::addr_of_mut!((*slf).1);
                                        let ptr = unsafe { core_reexport::ptr::NonNull::new_unchecked(field) };
                                        unsafe { PtrInner::new(ptr) }
                                    };
                                    candidate.reborrow().cast_unsized_unchecked(project)
                                };
                                <u8 as ::zerocopy::TryFromBytes>::is_bit_valid(field_candidate)
                            } && {
                                let field_candidate = unsafe {
                                    let project = |slf: PtrInner<'_, Self>| {
                                        let slf = slf.as_non_null().as_ptr();
                                        let field = core_reexport::ptr::addr_of_mut!((*slf).2);
                                        let ptr = unsafe { core_reexport::ptr::NonNull::new_unchecked(field) };
                                        unsafe { PtrInner::new(ptr) }
                                    };
                                    candidate.reborrow().cast_unsized_unchecked(project)
                                };
                                <X as ::zerocopy::TryFromBytes>::is_bit_valid(field_candidate)
                            } && {
                                let field_candidate = unsafe {
                                    let project = |slf: PtrInner<'_, Self>| {
                                        let slf = slf.as_non_null().as_ptr();
                                        let field = core_reexport::ptr::addr_of_mut!((*slf).3);
                                        let ptr = unsafe { core_reexport::ptr::NonNull::new_unchecked(field) };
                                        unsafe { PtrInner::new(ptr) }
                                    };
                                    candidate.reborrow().cast_unsized_unchecked(project)
                                };
                                <X::Target as ::zerocopy::TryFromBytes>::is_bit_valid(field_candidate)
                            } && {
                                let field_candidate = unsafe {
                                    let project = |slf: PtrInner<'_, Self>| {
                                        let slf = slf.as_non_null().as_ptr();
                                        let field = core_reexport::ptr::addr_of_mut!((*slf).4);
                                        let ptr = unsafe { core_reexport::ptr::NonNull::new_unchecked(field) };
                                        unsafe { PtrInner::new(ptr) }
                                    };
                                    candidate.reborrow().cast_unsized_unchecked(project)
                                };
                                <Y::Target as ::zerocopy::TryFromBytes>::is_bit_valid(field_candidate)
                            } && {
                                let field_candidate = unsafe {
                                    let project = |slf: PtrInner<'_, Self>| {
                                        let slf = slf.as_non_null().as_ptr();
                                        let field = core_reexport::ptr::addr_of_mut!((*slf).5);
                                        let ptr = unsafe { core_reexport::ptr::NonNull::new_unchecked(field) };
                                        unsafe { PtrInner::new(ptr) }
                                    };
                                    candidate.reborrow().cast_unsized_unchecked(project)
                                };
                                <[(X, Y); N] as ::zerocopy::TryFromBytes>::is_bit_valid(field_candidate)
                            } && {
                                let field_candidate = unsafe {
                                    let project = |slf: PtrInner<'_, Self>| {
                                        let slf = slf.as_non_null().as_ptr();
                                        let field = core_reexport::ptr::addr_of_mut!((*slf).6);
                                        let ptr = unsafe { core_reexport::ptr::NonNull::new_unchecked(field) };
                                        unsafe { PtrInner::new(ptr) }
                                    };
                                    candidate.reborrow().cast_unsized_unchecked(project)
                                };
                                <core_reexport::marker::PhantomData<
                                        ComplexWithGenerics<'a, N, X, Y>,
                                    > as ::zerocopy::TryFromBytes>::is_bit_valid(field_candidate)
                            }
                        }
                    }
                    #[repr(C)]
                    #[allow(non_snake_case)]
                    struct ___ZerocopyVariantStruct_TupleLike<'a: 'static, const N: usize, X, Y: Deref>(
                        core_reexport::mem::MaybeUninit<___ZerocopyInnerTag>,
                        bool,
                        Y,
                        PhantomData<&'a [(X, Y); N]>,
                        core_reexport::marker::PhantomData<ComplexWithGenerics<'a, N, X, Y>>,
                    )
                    where
                        X: Deref<Target = &'a [(X, Y); N]>;
                    #[allow(deprecated)]
                    #[automatically_derived]
                    unsafe impl<'a: 'static, const N: usize, X, Y: Deref> ::zerocopy::TryFromBytes
                        for ___ZerocopyVariantStruct_TupleLike<'a, { N }, X, Y>
                    where
                        X: Deref<Target = &'a [(X, Y); N]>,
                        core_reexport::mem::MaybeUninit<___ZerocopyInnerTag>: ::zerocopy::TryFromBytes,
                        bool: ::zerocopy::TryFromBytes,
                        Y: ::zerocopy::TryFromBytes,
                        PhantomData<&'a [(X, Y); N]>: ::zerocopy::TryFromBytes,
                        core_reexport::marker::PhantomData<ComplexWithGenerics<'a, N, X, Y>>:
                            ::zerocopy::TryFromBytes,
                    {
                        fn only_derive_is_allowed_to_implement_this_trait() {}
                        fn is_bit_valid<___ZerocopyAliasing>(
                            mut candidate: ::zerocopy::Maybe<Self, ___ZerocopyAliasing>,
                        ) -> ::zerocopy::util::macro_util::core_reexport::primitive::bool
                        where
                            ___ZerocopyAliasing: ::zerocopy::pointer::invariant::Reference,
                        {
                            use ::zerocopy::util::macro_util::core_reexport;
                            use ::zerocopy::pointer::PtrInner;

                            true && {
                                let field_candidate = unsafe {
                                    let project = |slf: PtrInner<'_, Self>| {
                                        let slf = slf.as_non_null().as_ptr();
                                        let field = core_reexport::ptr::addr_of_mut!((*slf).0);
                                        let ptr = unsafe { core_reexport::ptr::NonNull::new_unchecked(field) };
                                        unsafe { PtrInner::new(ptr) }
                                    };
                                    candidate.reborrow().cast_unsized_unchecked(project)
                                };
                                <core_reexport::mem::MaybeUninit<
                                        ___ZerocopyInnerTag,
                                    > as ::zerocopy::TryFromBytes>::is_bit_valid(field_candidate)
                            } && {
                                let field_candidate = unsafe {
                                    let project = |slf: PtrInner<'_, Self>| {
                                        let slf = slf.as_non_null().as_ptr();
                                        let field = core_reexport::ptr::addr_of_mut!((*slf).1);
                                        let ptr = unsafe { core_reexport::ptr::NonNull::new_unchecked(field) };
                                        unsafe { PtrInner::new(ptr) }
                                    };
                                    candidate.reborrow().cast_unsized_unchecked(project)
                                };
                                <bool as ::zerocopy::TryFromBytes>::is_bit_valid(field_candidate)
                            } && {
                                let field_candidate = unsafe {
                                    let project = |slf: PtrInner<'_, Self>| {
                                        let slf = slf.as_non_null().as_ptr();
                                        let field = core_reexport::ptr::addr_of_mut!((*slf).2);
                                        let ptr = unsafe { core_reexport::ptr::NonNull::new_unchecked(field) };
                                        unsafe { PtrInner::new(ptr) }
                                    };
                                    candidate.reborrow().cast_unsized_unchecked(project)
                                };
                                <Y as ::zerocopy::TryFromBytes>::is_bit_valid(field_candidate)
                            } && {
                                let field_candidate = unsafe {
                                    let project = |slf: PtrInner<'_, Self>| {
                                        let slf = slf.as_non_null().as_ptr();
                                        let field = core_reexport::ptr::addr_of_mut!((*slf).3);
                                        let ptr = unsafe { core_reexport::ptr::NonNull::new_unchecked(field) };
                                        unsafe { PtrInner::new(ptr) }
                                    };
                                    candidate.reborrow().cast_unsized_unchecked(project)
                                };
                                <PhantomData<&'a [(X, Y); N]> as ::zerocopy::TryFromBytes>::is_bit_valid(
                                    field_candidate,
                                )
                            } && {
                                let field_candidate = unsafe {
                                    let project = |slf: PtrInner<'_, Self>| {
                                        let slf = slf.as_non_null().as_ptr();
                                        let field = core_reexport::ptr::addr_of_mut!((*slf).4);
                                        let ptr = unsafe { core_reexport::ptr::NonNull::new_unchecked(field) };
                                        unsafe { PtrInner::new(ptr) }
                                    };
                                    candidate.reborrow().cast_unsized_unchecked(project)
                                };
                                <core_reexport::marker::PhantomData<
                                        ComplexWithGenerics<'a, N, X, Y>,
                                    > as ::zerocopy::TryFromBytes>::is_bit_valid(field_candidate)
                            }
                        }
                    }
                    #[repr(C)]
                    #[allow(non_snake_case)]
                    union ___ZerocopyVariants<'a: 'static, const N: usize, X, Y: Deref> {
                        __field_StructLike:
                            core_reexport::mem::ManuallyDrop<___ZerocopyVariantStruct_StructLike<'a, N, X, Y>>,
                        __field_TupleLike:
                            core_reexport::mem::ManuallyDrop<___ZerocopyVariantStruct_TupleLike<'a, N, X, Y>>,
                        __nonempty: (),
                    }
                    #[repr(C)]
                    struct ___ZerocopyRawEnum<'a: 'static, const N: usize, X, Y: Deref> {
                        tag: ___ZerocopyOuterTag,
                        variants: ___ZerocopyVariants<'a, N, X, Y>,
                    }
                    let tag = {
                        let tag_ptr = unsafe {
                            candidate.reborrow().cast_unsized_unchecked(|p: ::zerocopy::pointer::PtrInner<'_, Self>| { p.cast_sized::<___ZerocopyTagPrimitive> ()})
                        };
                        let tag_ptr = unsafe { tag_ptr.assume_initialized() };
                        tag_ptr.recall_validity::<_, (_, (_, _))>().read_unaligned::<::zerocopy::BecauseImmutable>()
                    };
                    let raw_enum = unsafe {
                        candidate.cast_unsized_unchecked(|p: ::zerocopy::pointer::PtrInner<'_, Self>| { p.cast_sized::<___ZerocopyRawEnum<'a, N, X, Y>> ()})
                    };
                    let raw_enum = unsafe { raw_enum.assume_initialized() };
                    let variants = unsafe {
                        use ::zerocopy::pointer::PtrInner;
                        raw_enum.cast_unsized_unchecked(|p: PtrInner<'_, ___ZerocopyRawEnum<'a, N, X, Y>>| {
                            let p = p.as_non_null().as_ptr();
                            let ptr = core_reexport::ptr::addr_of_mut!((*p).variants);
                            let ptr = unsafe { core_reexport::ptr::NonNull::new_unchecked(ptr) };
                            unsafe { PtrInner::new(ptr) }
                        })
                    };
                    #[allow(non_upper_case_globals)]
                    match tag {
                        ___ZEROCOPY_TAG_UnitLike => true,
                        ___ZEROCOPY_TAG_StructLike => {
                            let variant = unsafe {
                                variants.cast_unsized_unchecked(|p: ::zerocopy::pointer::PtrInner<'_, ___ZerocopyVariants<'a, N, X, Y>>| {
                                    p.cast_sized::<___ZerocopyVariantStruct_StructLike<'a, N, X, Y>>()
                                })
                            };
                            let variant = unsafe { variant.assume_initialized() };
                        <___ZerocopyVariantStruct_StructLike<'a, N, X, Y> as ::zerocopy ::TryFromBytes>::is_bit_valid (
                                            variant)
                        }
                        ___ZEROCOPY_TAG_TupleLike => {
                            let variant = unsafe {
                                variants.cast_unsized_unchecked(|p: ::zerocopy::pointer::PtrInner<'_, ___ZerocopyVariants<'a, N, X, Y>>| {
                                    p.cast_sized::<___ZerocopyVariantStruct_TupleLike<'a, N, X, Y>>()
                                })
                            };
                            let variant = unsafe { variant.assume_initialized() };
                        <___ZerocopyVariantStruct_TupleLike<'a, N, X, Y> as ::zerocopy ::TryFromBytes>::is_bit_valid (
                                            variant)
                        }
                        _ => false,
                    }
                }
            }
        } no_build
    }
}

// This goes at the bottom because it's so verbose and it makes scrolling past
// other code a pain.
#[test]
fn test_from_bytes_enum() {
    test! {
        FromBytes {
            #[repr(u8)]
            enum Foo {
                Variant0,
                Variant1,
                Variant2,
                Variant3,
                Variant4,
                Variant5,
                Variant6,
                Variant7,
                Variant8,
                Variant9,
                Variant10,
                Variant11,
                Variant12,
                Variant13,
                Variant14,
                Variant15,
                Variant16,
                Variant17,
                Variant18,
                Variant19,
                Variant20,
                Variant21,
                Variant22,
                Variant23,
                Variant24,
                Variant25,
                Variant26,
                Variant27,
                Variant28,
                Variant29,
                Variant30,
                Variant31,
                Variant32,
                Variant33,
                Variant34,
                Variant35,
                Variant36,
                Variant37,
                Variant38,
                Variant39,
                Variant40,
                Variant41,
                Variant42,
                Variant43,
                Variant44,
                Variant45,
                Variant46,
                Variant47,
                Variant48,
                Variant49,
                Variant50,
                Variant51,
                Variant52,
                Variant53,
                Variant54,
                Variant55,
                Variant56,
                Variant57,
                Variant58,
                Variant59,
                Variant60,
                Variant61,
                Variant62,
                Variant63,
                Variant64,
                Variant65,
                Variant66,
                Variant67,
                Variant68,
                Variant69,
                Variant70,
                Variant71,
                Variant72,
                Variant73,
                Variant74,
                Variant75,
                Variant76,
                Variant77,
                Variant78,
                Variant79,
                Variant80,
                Variant81,
                Variant82,
                Variant83,
                Variant84,
                Variant85,
                Variant86,
                Variant87,
                Variant88,
                Variant89,
                Variant90,
                Variant91,
                Variant92,
                Variant93,
                Variant94,
                Variant95,
                Variant96,
                Variant97,
                Variant98,
                Variant99,
                Variant100,
                Variant101,
                Variant102,
                Variant103,
                Variant104,
                Variant105,
                Variant106,
                Variant107,
                Variant108,
                Variant109,
                Variant110,
                Variant111,
                Variant112,
                Variant113,
                Variant114,
                Variant115,
                Variant116,
                Variant117,
                Variant118,
                Variant119,
                Variant120,
                Variant121,
                Variant122,
                Variant123,
                Variant124,
                Variant125,
                Variant126,
                Variant127,
                Variant128,
                Variant129,
                Variant130,
                Variant131,
                Variant132,
                Variant133,
                Variant134,
                Variant135,
                Variant136,
                Variant137,
                Variant138,
                Variant139,
                Variant140,
                Variant141,
                Variant142,
                Variant143,
                Variant144,
                Variant145,
                Variant146,
                Variant147,
                Variant148,
                Variant149,
                Variant150,
                Variant151,
                Variant152,
                Variant153,
                Variant154,
                Variant155,
                Variant156,
                Variant157,
                Variant158,
                Variant159,
                Variant160,
                Variant161,
                Variant162,
                Variant163,
                Variant164,
                Variant165,
                Variant166,
                Variant167,
                Variant168,
                Variant169,
                Variant170,
                Variant171,
                Variant172,
                Variant173,
                Variant174,
                Variant175,
                Variant176,
                Variant177,
                Variant178,
                Variant179,
                Variant180,
                Variant181,
                Variant182,
                Variant183,
                Variant184,
                Variant185,
                Variant186,
                Variant187,
                Variant188,
                Variant189,
                Variant190,
                Variant191,
                Variant192,
                Variant193,
                Variant194,
                Variant195,
                Variant196,
                Variant197,
                Variant198,
                Variant199,
                Variant200,
                Variant201,
                Variant202,
                Variant203,
                Variant204,
                Variant205,
                Variant206,
                Variant207,
                Variant208,
                Variant209,
                Variant210,
                Variant211,
                Variant212,
                Variant213,
                Variant214,
                Variant215,
                Variant216,
                Variant217,
                Variant218,
                Variant219,
                Variant220,
                Variant221,
                Variant222,
                Variant223,
                Variant224,
                Variant225,
                Variant226,
                Variant227,
                Variant228,
                Variant229,
                Variant230,
                Variant231,
                Variant232,
                Variant233,
                Variant234,
                Variant235,
                Variant236,
                Variant237,
                Variant238,
                Variant239,
                Variant240,
                Variant241,
                Variant242,
                Variant243,
                Variant244,
                Variant245,
                Variant246,
                Variant247,
                Variant248,
                Variant249,
                Variant250,
                Variant251,
                Variant252,
                Variant253,
                Variant254,
                Variant255,
            }
        } expands to {
            #[allow(deprecated)]
            #[automatically_derived]
            unsafe impl ::zerocopy::TryFromBytes for Foo {
                fn only_derive_is_allowed_to_implement_this_trait() {}

                fn is_bit_valid<___ZerocopyAliasing>(
                    _candidate: ::zerocopy::Maybe<Self, ___ZerocopyAliasing>,
                ) -> ::zerocopy::util::macro_util::core_reexport::primitive::bool
                where
                    ___ZerocopyAliasing: ::zerocopy::pointer::invariant::Reference,
                {
                    if false {
                        fn assert_is_from_bytes<T>()
                        where
                            T: ::zerocopy::FromBytes,
                            T: ?::zerocopy::util::macro_util::core_reexport::marker::Sized,
                        {}
                        assert_is_from_bytes::<Self>();
                    }

                    true
                }
            }

            #[allow(deprecated)]
            #[automatically_derived]
            unsafe impl ::zerocopy::FromZeros for Foo {
                fn only_derive_is_allowed_to_implement_this_trait() {}
            }

            #[allow(deprecated)]
            #[automatically_derived]
            unsafe impl ::zerocopy::FromBytes for Foo {
                fn only_derive_is_allowed_to_implement_this_trait() {}
            }
        } no_build
    }
}

#[test]
fn test_try_from_bytes_trivial_is_bit_valid_enum() {
    // Even when we aren't deriving `FromBytes` as the top-level trait,
    // `TryFromBytes` on enums still detects whether we *could* derive
    // `FromBytes`, and if so, performs the same "trivial `is_bit_valid`"
    // optimization.
    test! {
        TryFromBytes {
            #[repr(u8)]
            enum Foo {
                Variant0,
                Variant1,
                Variant2,
                Variant3,
                Variant4,
                Variant5,
                Variant6,
                Variant7,
                Variant8,
                Variant9,
                Variant10,
                Variant11,
                Variant12,
                Variant13,
                Variant14,
                Variant15,
                Variant16,
                Variant17,
                Variant18,
                Variant19,
                Variant20,
                Variant21,
                Variant22,
                Variant23,
                Variant24,
                Variant25,
                Variant26,
                Variant27,
                Variant28,
                Variant29,
                Variant30,
                Variant31,
                Variant32,
                Variant33,
                Variant34,
                Variant35,
                Variant36,
                Variant37,
                Variant38,
                Variant39,
                Variant40,
                Variant41,
                Variant42,
                Variant43,
                Variant44,
                Variant45,
                Variant46,
                Variant47,
                Variant48,
                Variant49,
                Variant50,
                Variant51,
                Variant52,
                Variant53,
                Variant54,
                Variant55,
                Variant56,
                Variant57,
                Variant58,
                Variant59,
                Variant60,
                Variant61,
                Variant62,
                Variant63,
                Variant64,
                Variant65,
                Variant66,
                Variant67,
                Variant68,
                Variant69,
                Variant70,
                Variant71,
                Variant72,
                Variant73,
                Variant74,
                Variant75,
                Variant76,
                Variant77,
                Variant78,
                Variant79,
                Variant80,
                Variant81,
                Variant82,
                Variant83,
                Variant84,
                Variant85,
                Variant86,
                Variant87,
                Variant88,
                Variant89,
                Variant90,
                Variant91,
                Variant92,
                Variant93,
                Variant94,
                Variant95,
                Variant96,
                Variant97,
                Variant98,
                Variant99,
                Variant100,
                Variant101,
                Variant102,
                Variant103,
                Variant104,
                Variant105,
                Variant106,
                Variant107,
                Variant108,
                Variant109,
                Variant110,
                Variant111,
                Variant112,
                Variant113,
                Variant114,
                Variant115,
                Variant116,
                Variant117,
                Variant118,
                Variant119,
                Variant120,
                Variant121,
                Variant122,
                Variant123,
                Variant124,
                Variant125,
                Variant126,
                Variant127,
                Variant128,
                Variant129,
                Variant130,
                Variant131,
                Variant132,
                Variant133,
                Variant134,
                Variant135,
                Variant136,
                Variant137,
                Variant138,
                Variant139,
                Variant140,
                Variant141,
                Variant142,
                Variant143,
                Variant144,
                Variant145,
                Variant146,
                Variant147,
                Variant148,
                Variant149,
                Variant150,
                Variant151,
                Variant152,
                Variant153,
                Variant154,
                Variant155,
                Variant156,
                Variant157,
                Variant158,
                Variant159,
                Variant160,
                Variant161,
                Variant162,
                Variant163,
                Variant164,
                Variant165,
                Variant166,
                Variant167,
                Variant168,
                Variant169,
                Variant170,
                Variant171,
                Variant172,
                Variant173,
                Variant174,
                Variant175,
                Variant176,
                Variant177,
                Variant178,
                Variant179,
                Variant180,
                Variant181,
                Variant182,
                Variant183,
                Variant184,
                Variant185,
                Variant186,
                Variant187,
                Variant188,
                Variant189,
                Variant190,
                Variant191,
                Variant192,
                Variant193,
                Variant194,
                Variant195,
                Variant196,
                Variant197,
                Variant198,
                Variant199,
                Variant200,
                Variant201,
                Variant202,
                Variant203,
                Variant204,
                Variant205,
                Variant206,
                Variant207,
                Variant208,
                Variant209,
                Variant210,
                Variant211,
                Variant212,
                Variant213,
                Variant214,
                Variant215,
                Variant216,
                Variant217,
                Variant218,
                Variant219,
                Variant220,
                Variant221,
                Variant222,
                Variant223,
                Variant224,
                Variant225,
                Variant226,
                Variant227,
                Variant228,
                Variant229,
                Variant230,
                Variant231,
                Variant232,
                Variant233,
                Variant234,
                Variant235,
                Variant236,
                Variant237,
                Variant238,
                Variant239,
                Variant240,
                Variant241,
                Variant242,
                Variant243,
                Variant244,
                Variant245,
                Variant246,
                Variant247,
                Variant248,
                Variant249,
                Variant250,
                Variant251,
                Variant252,
                Variant253,
                Variant254,
                Variant255,
            }
        } expands to {
            #[allow(deprecated)]
            #[automatically_derived]
            unsafe impl ::zerocopy::TryFromBytes for Foo {
                fn only_derive_is_allowed_to_implement_this_trait() {}

                fn is_bit_valid<___ZerocopyAliasing>(
                    _candidate: ::zerocopy::Maybe<Self, ___ZerocopyAliasing>,
                ) -> ::zerocopy::util::macro_util::core_reexport::primitive::bool
                where
                    ___ZerocopyAliasing: ::zerocopy::pointer::invariant::Reference,
                {
                    true
                }
            }
        } no_build
    }
}

#[test]
fn test_hash() {
    test! {
        ByteHash {
            struct Foo<T: Clone>(T) where Self: Sized;
        } expands to {
            #[allow(deprecated)]
            #[automatically_derived]
            impl<T: Clone> ::zerocopy::util::macro_util::core_reexport::hash::Hash for Foo<T>
            where
                Self: ::zerocopy::IntoBytes + ::zerocopy::Immutable,
                Self: Sized,
            {
                fn hash<H>(&self, state: &mut H)
                where
                    H: ::zerocopy::util::macro_util::core_reexport::hash::Hasher,
                {
                    ::zerocopy::util::macro_util::core_reexport::hash::Hasher::write(
                        state,
                        ::zerocopy::IntoBytes::as_bytes(self)
                    )
                }

                fn hash_slice<H>(data: &[Self], state: &mut H)
                where
                    H: ::zerocopy::util::macro_util::core_reexport::hash::Hasher,
                {
                    ::zerocopy::util::macro_util::core_reexport::hash::Hasher::write(
                        state,
                        ::zerocopy::IntoBytes::as_bytes(data)
                    )
                }
            }
        } no_build
    }
}

#[test]
fn test_eq() {
    test! {
        ByteEq {
            struct Foo<T: Clone>(T) where Self: Sized;
        } expands to {
            #[allow(deprecated)]
            #[automatically_derived]
            impl<T: Clone> ::zerocopy::util::macro_util::core_reexport::cmp::PartialEq for Foo<T>
            where
                Self: ::zerocopy::IntoBytes + ::zerocopy::Immutable,
                Self: Sized,
            {
                fn eq(&self, other: &Self) -> bool {
                    ::zerocopy::util::macro_util::core_reexport::cmp::PartialEq::eq(
                        ::zerocopy::IntoBytes::as_bytes(self),
                        ::zerocopy::IntoBytes::as_bytes(other),
                    )
                }
            }

            #[allow(deprecated)]
            #[automatically_derived]
            impl<T: Clone> ::zerocopy::util::macro_util::core_reexport::cmp::Eq for Foo<T>
            where
                Self: ::zerocopy::IntoBytes + ::zerocopy::Immutable,
                Self: Sized,
            {
            }
        } no_build
    }
}

#[test]
fn test_split_at() {
    test! {
        SplitAt {
            #[repr(C)]
            struct Foo<T: ?Sized + Copy>(T) where Self: Copy;
        } expands to {
            #[allow(deprecated)]
            #[automatically_derived]
            unsafe impl<T: ?Sized + Copy> ::zerocopy::SplitAt for Foo<T>
            where
                Self: Copy,
                T: ::zerocopy::SplitAt,
            {
                fn only_derive_is_allowed_to_implement_this_trait() {}
                type Elem = <T as ::zerocopy::SplitAt>::Elem;
            }
        } no_build
    }

    test! {
        SplitAt {
            #[repr(transparent)]
            struct Foo<T: ?Sized + Copy>(T) where Self: Copy;
        } expands to {
            #[allow(deprecated)]
            #[automatically_derived]
            unsafe impl<T: ?Sized + Copy> ::zerocopy::SplitAt for Foo<T>
            where
                Self: Copy,
                T: ::zerocopy::SplitAt,
            {
                fn only_derive_is_allowed_to_implement_this_trait() {}
                type Elem = <T as ::zerocopy::SplitAt>::Elem;
            }
        } no_build
    }

    test! {
        SplitAt {
            #[repr(packed)]
            struct Foo<T: ?Sized + Copy>(T) where Self: Copy;
        } expands to {
            ::core::compile_error! {
                "must not have #[repr(packed)] attribute"
            }
        } no_build
    }

    test! {
        SplitAt {
            #[repr(packed(2))]
            struct Foo<T: ?Sized + Copy>(T) where Self: Copy;
        } expands to {
            ::core::compile_error! {
                "must not have #[repr(packed)] attribute"
            }
        } no_build
    }

    test! {
        SplitAt {
            enum Foo {}
        } expands to {
            ::core::compile_error! {
                "can only be applied to structs"
            }
        } no_build
    }

    test! {
        SplitAt {
            union Foo { a: () }
        } expands to {
            ::core::compile_error! {
                "can only be applied to structs"
            }
        } no_build
    }

    test! {
        SplitAt {
            struct Foo<T: ?Sized + Copy>(T) where Self: Copy;
        } expands to {
            ::core::compile_error! {
                "must have #[repr(C)] or #[repr(transparent)] in order to guarantee this type's layout is splitable"
            }
        } no_build
    }
}
