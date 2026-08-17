// Copyright 2024 The Fuchsia Authors
//
// Licensed under a BSD-style license <LICENSE-BSD>, Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0>, or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to
// those terms.

// Make sure that the derive macros will respect the
// `#[zerocopy(crate = "...")]` attribute when renaming the crate.

// See comment in `include.rs` for why we disable the prelude.
#![no_implicit_prelude]
#![allow(warnings)]

include!("include.rs");

#[test]
fn test_gen_custom_zerocopy() {
    #[derive(
        imp::ByteEq,
        imp::ByteHash,
        imp::IntoBytes,
        imp::FromBytes,
        imp::Unaligned,
        imp::Immutable,
        imp::KnownLayout,
    )]
    #[zerocopy(crate = "fake_zerocopy")]
    #[repr(packed)]
    struct SomeStruct {
        a: u16,
        b: u32,
    }

    impl AssertNotZerocopyIntoBytes for SomeStruct {}
    impl AssertNotZerocopyFromBytes for SomeStruct {}
    impl AssertNotZerocopyUnaligned for SomeStruct {}
    impl AssertNotZerocopyImmutable for SomeStruct {}
    impl AssertNotZerocopyKnownLayout for SomeStruct {}

    fake_zerocopy::assert::<SomeStruct>();
}

mod fake_zerocopy {
    use ::std::{io, ptr::NonNull, unimplemented};

    pub use super::imp::*;

    pub fn assert<T>()
    where
        T: IntoBytes + FromBytes + Unaligned + Immutable,
    {
    }

    pub unsafe trait IntoBytes {
        fn only_derive_is_allowed_to_implement_this_trait()
        where
            Self: Sized;

        fn as_bytes(&self) -> &[u8]
        where
            Self: Immutable,
        {
            unimplemented!()
        }
    }

    pub unsafe trait FromBytes: FromZeros {
        fn only_derive_is_allowed_to_implement_this_trait()
        where
            Self: Sized;
    }

    pub unsafe trait Unaligned {
        fn only_derive_is_allowed_to_implement_this_trait()
        where
            Self: Sized;
    }

    pub unsafe trait Immutable {
        fn only_derive_is_allowed_to_implement_this_trait()
        where
            Self: Sized;
    }

    pub unsafe trait KnownLayout {
        fn only_derive_is_allowed_to_implement_this_trait()
        where
            Self: Sized;

        type PointerMetadata: PointerMetadata;

        type MaybeUninit: ?Sized + KnownLayout<PointerMetadata = Self::PointerMetadata>;

        const LAYOUT: DstLayout;

        fn raw_from_ptr_len(bytes: NonNull<u8>, meta: Self::PointerMetadata) -> NonNull<Self>;

        fn pointer_to_metadata(ptr: *mut Self) -> Self::PointerMetadata;
    }

    macro_rules! impl_ty {
        ($ty:ty $(as $generic:ident)?) => {
            unsafe impl$(<$generic: IntoBytes>)? IntoBytes for $ty {
                fn only_derive_is_allowed_to_implement_this_trait()
                where
                    Self: Sized,
                {
                    unimplemented!()
                }
            }

            unsafe impl$(<$generic: FromBytes>)? FromBytes for $ty {
                fn only_derive_is_allowed_to_implement_this_trait()
                where
                    Self: Sized,
                {
                    unimplemented!()
                }
            }

            unsafe impl$(<$generic: Unaligned>)? Unaligned for $ty {
                fn only_derive_is_allowed_to_implement_this_trait()
                where
                    Self: Sized,
                {
                    unimplemented!()
                }
            }

            unsafe impl$(<$generic: Immutable>)? Immutable for $ty {
                fn only_derive_is_allowed_to_implement_this_trait()
                where
                    Self: Sized,
                {
                    unimplemented!()
                }
            }

            unsafe impl$(<$generic: KnownLayout>)? KnownLayout for $ty {
                fn only_derive_is_allowed_to_implement_this_trait()
                where
                    Self: Sized,
                {
                    unimplemented!()
                }

                type PointerMetadata = ();

                type MaybeUninit = ();

                const LAYOUT: DstLayout = DstLayout::new_zst(None);

                fn raw_from_ptr_len(
                    bytes: NonNull<u8>,
                    meta: Self::PointerMetadata,
                ) -> NonNull<Self> {
                    unimplemented!()
                }

                fn pointer_to_metadata(ptr: *mut Self) -> Self::PointerMetadata {
                    unimplemented!()
                }
            }
        };
    }

    impl_ty!(());
    impl_ty!(u16);
    impl_ty!(u32);
    impl_ty!([T] as T);
    impl_ty!(::std::mem::MaybeUninit<T> as T);
}

pub trait AssertNotZerocopyIntoBytes {}
impl<T: imp::IntoBytes> AssertNotZerocopyIntoBytes for T {}

pub trait AssertNotZerocopyFromBytes {}
impl<T: imp::FromBytes> AssertNotZerocopyFromBytes for T {}

pub trait AssertNotZerocopyUnaligned {}
impl<T: imp::Unaligned> AssertNotZerocopyUnaligned for T {}

pub trait AssertNotZerocopyImmutable {}
impl<T: imp::Immutable> AssertNotZerocopyImmutable for T {}

pub trait AssertNotZerocopyKnownLayout {}
impl<T: imp::KnownLayout> AssertNotZerocopyKnownLayout for T {}
