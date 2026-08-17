// Copyright 2019 The Fuchsia Authors
//
// Licensed under a BSD-style license <LICENSE-BSD>, Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0>, or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to
// those terms.

// ON THE PRELUDE: All of the tests in this directory (excepting UI tests)
// disable the prelude via `#![no_implicit_prelude]`. This ensures that all code
// emitted by our derives doesn't accidentally assume that the prelude is
// included, which helps ensure that items are referred to by absolute path,
// which in turn ensures that these items can't accidentally refer to names
// which have been shadowed. For example, the code `x == None` could behave
// incorrectly if, in the scope in which the derive is invoked, `None` has been
// shadowed by `CONST None: Option<usize> = Some(1)`.
//
// `mod imp` allows us to import items and refer to them in this module without
// introducing the risk that this hides bugs in which derive-emitted code uses
// names which are not fully-qualified. For such a bug to manifest, it would
// need to be of the form `imp::Foo`, which is unlikely to happen by accident.
mod imp {
    // Since this file is included in every test file, and since not every test
    // file uses every item here, we allow unused imports to avoid generating
    // warnings.
    #[allow(unused)]
    pub use {
        ::core::{
            self, assert_eq, assert_ne,
            cell::UnsafeCell,
            convert::TryFrom,
            hash,
            marker::PhantomData,
            mem::{ManuallyDrop, MaybeUninit},
            option::IntoIter,
            prelude::v1::*,
            primitive::*,
        },
        ::std::{collections::hash_map::DefaultHasher, prelude::v1::*},
        ::zerocopy::*,
    };
}

// These items go in their own module (rather than the top level) for the same
// reason that we use `mod imp` above. See its comment for more details.
pub mod util {
    /// A type that doesn't implement any zerocopy traits.
    pub struct NotZerocopy<T = ()>(pub T);

    /// A `u16` with alignment 2.
    ///
    /// Though `u16` has alignment 2 on some platforms, it's not guaranteed. By
    /// contrast, `util::AU16` is guaranteed to have alignment 2.
    #[derive(
        super::imp::KnownLayout,
        super::imp::Immutable,
        super::imp::FromBytes,
        super::imp::IntoBytes,
        Copy,
        Clone,
    )]
    #[repr(C, align(2))]
    pub struct AU16(pub u16);

    // Since we can't import these by path (ie, `util::assert_impl_all!`), use a
    // name prefix to ensure our derive-emitted code isn't accidentally relying
    // on `assert_impl_all!` being in scope.
    #[macro_export]
    macro_rules! util_assert_impl_all {
        ($type:ty: $($trait:path),+ $(,)?) => {
            const _: fn() = || {
                use ::core::prelude::v1::*;
                ::static_assertions::assert_impl_all!($type: $($trait),+);
            };
        };
    }

    // Since we can't import these by path (ie, `util::assert_not_impl_any!`),
    // use a name prefix to ensure our derive-emitted code isn't accidentally
    // relying on `assert_not_impl_any!` being in scope.
    #[macro_export]
    macro_rules! util_assert_not_impl_any {
        ($x:ty: $($t:path),+ $(,)?) => {
            const _: fn() = || {
                use ::core::prelude::v1::*;
                ::static_assertions::assert_not_impl_any!($x: $($t),+);
            };
        };
    }

    #[macro_export]
    macro_rules! test_trivial_is_bit_valid {
        ($x:ty => $name:ident) => {
            #[test]
            fn $name() {
                util::test_trivial_is_bit_valid::<$x>();
            }
        };
    }

    // Under some circumstances, our `TryFromBytes` derive generates a trivial
    // `is_bit_valid` impl that unconditionally returns `true`. This test
    // attempts to validate that this is, indeed, the behavior of our
    // `TryFromBytes` derive. It is not foolproof, but is likely to catch some
    // mistakes.
    //
    // As of this writing, this happens when deriving `TryFromBytes` thanks to a
    // top-level `#[derive(FromBytes)]`.
    pub fn test_trivial_is_bit_valid<T: super::imp::TryFromBytes>() {
        // This test works based on the insight that a trivial `is_bit_valid`
        // impl should never load any bytes from memory. Thus, while it is
        // technically a violation of `is_bit_valid`'s safety precondition to
        // pass a pointer to uninitialized memory, the `is_bit_valid` impl we
        // expect our derives to generate should never touch this memory, and
        // thus should never exhibit UB. By contrast, if our derives are
        // spuriously generating non-trivial `is_bit_valid` impls, this should
        // cause UB which may be caught by Miri.

        let buf = super::imp::MaybeUninit::<T>::uninit();
        let ptr = super::imp::Ptr::from_ref(&buf);
        // SAFETY: This is intentionally unsound; see the preceding comment.
        let ptr = unsafe { ptr.assume_initialized() };

        // SAFETY: `T` and `MaybeUninit<T>` have the same layout, so this is a
        // size-preserving cast. It is also a provenance-preserving cast.
        let ptr = unsafe { ptr.cast_unsized_unchecked(|p| p.cast_sized()) };
        assert!(<T as super::imp::TryFromBytes>::is_bit_valid(ptr));
    }
}
