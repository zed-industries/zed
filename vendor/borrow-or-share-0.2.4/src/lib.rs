#![warn(missing_docs, rust_2018_idioms)]
#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![no_std]

//! Traits for either borrowing or sharing data.
//!
//! # Walkthrough
//!
//! Suppose that you have a generic type that either owns some data or holds a reference to them.
//! You want to implement on this type a method taking `&self` that either borrows from `*self`
//! or from behind a reference it holds. A naive way to do this would be
//! to duplicate the method declaration:
//!
//! ```
//! struct Text<T>(T);
//!
//! impl Text<String> {
//!     // The returned reference is borrowed from `*self`
//!     // and lives as long as `self`.
//!     fn as_str(&self) -> &str {
//!         &self.0
//!     }
//! }
//!
//! impl<'a> Text<&'a str> {
//!     // The returned reference is borrowed from `*self.0`, lives
//!     // longer than `self` and is said to be shared with `*self`.
//!     fn as_str(&self) -> &'a str {
//!         self.0
//!     }
//! }
//! ```
//!
//! However, if you add more methods to `Text`, the code would become
//! intolerably verbose. This crate thus provides a [`BorrowOrShare`] trait
//! you can use to simplify the above code by making the `as_str` method
//! generic over `T`:
//!
//! ```
//! use borrow_or_share::BorrowOrShare;
//!
//! struct Text<T>(T);
//!
//! impl<'i, 'o, T: BorrowOrShare<'i, 'o, str>> Text<T> {
//!     fn as_str(&'i self) -> &'o str {
//!         self.0.borrow_or_share()
//!     }
//! }
//!
//! // The returned reference is borrowed from `*text`
//! // and lives as long as `text`.
//! fn borrow(text: &Text<String>) -> &str {
//!     text.as_str()
//! }
//!
//! // The returned reference is borrowed from `*text.0`, lives
//! // longer than `text` and is said to be shared with `*text`.
//! fn share<'a>(text: &Text<&'a str>) -> &'a str {
//!     text.as_str()
//! }
//! ```
//!
//! The [`BorrowOrShare`] trait takes two lifetime parameters `'i`, `'o`,
//! and a type parameter `T`. For `T = str` it is implemented on `String`
//! wherever `'i: 'o`, while on `&'a str` wherever `'a: 'i + 'o`.
//! The trait is also implemented on other types, which we'll cover later.
//!
//! On the trait is a [`borrow_or_share`] method that takes `&'i self`
//! and returns `&'o T`. You can use it to write your own
//! "data borrowing or sharing" functions. A typical usage would be
//! to put a `BorrowOrShare<'i, 'o, str>` bound on a type parameter `T`
//! taken by an `impl` block of your type. Within the block, you implement
//! a method that takes `&'i self` and returns something with lifetime `'o`,
//! by calling the [`borrow_or_share`] method on some `T`
//! contained in `self` and further processing the returned `&'o str`.
//!
//! [`borrow_or_share`]: BorrowOrShare::borrow_or_share
//!
//! While you're happy with the different behavior of the `as_str` method
//! on `Text<String>` (borrowing) and on `Text<&str>` (sharing), you still
//! have to fall back on borrowing when dealing with generic `Text<T>`.
//! For example, you may want to implement [`AsRef`] on `Text<T>`,
//! which requires an `as_ref` method that always borrows from `*self`.
//! The code won't compile, however, if you put the same [`BorrowOrShare`]
//! bound and write `self.as_str()` in the [`AsRef`] impl:
//!
//! ```compile_fail
//! use borrow_or_share::BorrowOrShare;
//!
//! struct Text<T>(T);
//!
//! impl<'i, 'o, T: BorrowOrShare<'i, 'o, str>> Text<T> {
//!     fn as_str(&'i self) -> &'o str {
//!         self.0.borrow_or_share()
//!     }
//! }
//!
//! impl<'i, 'o, T: BorrowOrShare<'i, 'o, str>> AsRef<str> for Text<T> {
//!     fn as_ref(&self) -> &str {
//!         self.as_str()
//!     }
//! }
//! ```
//!
//! The problem is that in the [`AsRef`] impl, the anonymous lifetime
//! `'1` of `self` does not satisfy the bounds `'1: 'i` and `'o: '1`.
//! The idiomatic solution is to put a [`Bos`] bound instead:
//!
//! ```
//! use borrow_or_share::{BorrowOrShare, Bos};
//!
//! struct Text<T>(T);
//!
//! impl<'i, 'o, T: BorrowOrShare<'i, 'o, str>> Text<T> {
//!     fn as_str(&'i self) -> &'o str {
//!         self.0.borrow_or_share()
//!     }
//! }
//!
//! impl<T: Bos<str>> AsRef<str> for Text<T> {
//!     fn as_ref(&self) -> &str {
//!         self.as_str()
//!     }
//! }
//! ```
//!
//! In the above example, the `as_str` method is also available on `Text<T>`
//! where `T: Bos<str>`, because [`BorrowOrShare`] is implemented on
//! all types that implement [`Bos`]. It also works the other way round
//! because [`Bos`] is a supertrait of [`BorrowOrShare`].
//!
//! This crate provides [`Bos`] (and [`BorrowOrShare`]) implementations
//! on [`&T`](reference), [`&mut T`](reference), [`[T; N]`](array),
//! [`Vec<T>`], [`String`], [`CString`], [`OsString`], [`PathBuf`],
//! [`Box<T>`], [`Cow<'_, B>`], [`Rc<T>`], and [`Arc<T>`]. If some of
//! these are out of scope, consider putting extra trait bounds in your
//! code, preferably on a function that constructs your type.
//!
//! [`CString`]: alloc::ffi::CString
//! [`Cow<'_, B>`]: Cow
//! [`Rc<T>`]: alloc::rc::Rc
//! [`Arc<T>`]: alloc::sync::Arc
//!
//! You can also implement [`Bos`] on your own type, for example:
//!
//! ```
//! use borrow_or_share::Bos;
//!
//! struct Text<'a>(&'a str);
//!
//! impl<'a> Bos<str> for Text<'a> {
//!     type Ref<'this> = &'a str where Self: 'this;
//!     
//!     fn borrow_or_share(this: &Self) -> Self::Ref<'_> {
//!         this.0
//!     }
//! }
//! ```
//!
//! # Limitations
//!
//! This crate only provides implementations of [`Bos`] on types that
//! currently implement [`Borrow`] in the standard library, not including
//! the blanket implementation. If this is too restrictive, feel free
//! to copy the code pattern from this crate as you wish.
//!
//! [`Borrow`]: core::borrow::Borrow
//!
//! # Crate features
//!
//! - `std` (disabled by default): Implies `alloc`. Required for
//!   [`Bos`] implementations on [`OsString`] and [`PathBuf`].
//!
//! - `alloc`: Required for [`Bos`] implementations on [`alloc`] types.

#[cfg(any(feature = "alloc", doc))]
extern crate alloc;
#[cfg(any(feature = "std", doc))]
extern crate std;

mod internal {
    pub trait Ref<T: ?Sized> {
        fn cast<'a>(self) -> &'a T
        where
            Self: 'a;
    }

    impl<T: ?Sized> Ref<T> for &T {
        #[inline]
        fn cast<'a>(self) -> &'a T
        where
            Self: 'a,
        {
            self
        }
    }
}

use internal::Ref;

#[cfg(any(feature = "alloc", doc))]
use alloc::{
    borrow::{Cow, ToOwned},
    boxed::Box,
    string::String,
    vec::Vec,
};

#[cfg(any(feature = "std", doc))]
use std::{
    ffi::{OsStr, OsString},
    path::{Path, PathBuf},
};

/// A trait for either borrowing or sharing data.
///
/// See the [crate-level documentation](crate) for more details.
pub trait Bos<T: ?Sized> {
    /// The resulting reference type. May only be `&T`.
    type Ref<'this>: Ref<T>
    where
        Self: 'this;

    /// Borrows from `*this` or from behind a reference it holds,
    /// returning a reference of type [`Self::Ref`].
    ///
    /// In the latter case, the returned reference is said to be *shared* with `*this`.
    fn borrow_or_share(this: &Self) -> Self::Ref<'_>;
}

/// A helper trait for writing "data borrowing or sharing" functions.
///
/// See the [crate-level documentation](crate) for more details.
pub trait BorrowOrShare<'i, 'o, T: ?Sized>: Bos<T> {
    /// Borrows from `*self` or from behind a reference it holds.
    ///
    /// In the latter case, the returned reference is said to be *shared* with `*self`.
    fn borrow_or_share(&'i self) -> &'o T;
}

impl<'i, 'o, T: ?Sized, B> BorrowOrShare<'i, 'o, T> for B
where
    B: Bos<T> + ?Sized + 'i,
    B::Ref<'i>: 'o,
{
    #[inline]
    fn borrow_or_share(&'i self) -> &'o T {
        (B::borrow_or_share(self) as B::Ref<'i>).cast()
    }
}

impl<'a, T: ?Sized> Bos<T> for &'a T {
    type Ref<'this>
        = &'a T
    where
        Self: 'this;

    #[inline]
    fn borrow_or_share(this: &Self) -> Self::Ref<'_> {
        this
    }
}

macro_rules! impl_bos {
    ($($(#[$attr:meta])? $({$($params:tt)*})? $ty:ty => $target:ty)*) => {
        $(
            $(#[$attr])?
            impl $(<$($params)*>)? Bos<$target> for $ty {
                type Ref<'this> = &'this $target where Self: 'this;

                #[inline]
                fn borrow_or_share(this: &Self) -> Self::Ref<'_> {
                    this
                }
            }
        )*
    };
}

impl_bos! {
    // A blanket impl would show up everywhere in the
    // documentation of a dependent crate, which is noisy.
    // So we're omitting it for the moment.
    // {T: ?Sized} T => T

    {T: ?Sized} &mut T => T

    {T, const N: usize} [T; N] => [T]

    #[cfg(feature = "std")]
    OsString => OsStr
    #[cfg(feature = "std")]
    PathBuf => Path
}

#[cfg(feature = "alloc")]
impl_bos! {
    {T} Vec<T> => [T]

    String => str
    #[cfg(all(not(no_rc), not(no_sync), not(no_global_oom_handling)))]
    alloc::ffi::CString => core::ffi::CStr

    {T: ?Sized} Box<T> => T
    {B: ?Sized + ToOwned} Cow<'_, B> => B

    #[cfg(not(no_rc))]
    {T: ?Sized} alloc::rc::Rc<T> => T
    #[cfg(all(not(no_rc), not(no_sync), target_has_atomic = "ptr"))]
    {T: ?Sized} alloc::sync::Arc<T> => T
}
