//! [![github]](https://github.com/dtolnay/typeid)&ensp;[![crates-io]](https://crates.io/crates/typeid)&ensp;[![docs-rs]](https://docs.rs/typeid)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! # Const `TypeId` and non-'static `TypeId`
//!
//! <br>
//!
//! #### Const `TypeId`
//!
//! This crate provides [`ConstTypeId`], which is like [`core::any::TypeId`] but
//! is constructible in const in stable Rust. (The standard library's TypeId's
//! is nightly-only to construct in const; the tracking issue for this is
//! [rust#77125].)
//!
//! [rust#77125]: https://github.com/rust-lang/rust/issues/77125
//!
//! Being able to construct `ConstTypeId` in const makes it suitable for use
//! cases that rely on static promotion:
//!
//! ```
//! use std::fmt::{self, Debug, Display};
//! use std::ptr;
//! use typeid::ConstTypeId;
//!
//! pub struct ObjectVTable {
//!     type_id: ConstTypeId,
//!     drop_in_place: unsafe fn(*mut ()),
//!     display: unsafe fn(*const (), &mut fmt::Formatter) -> fmt::Result,
//!     debug: unsafe fn(*const (), &mut fmt::Formatter) -> fmt::Result,
//! }
//!
//! impl ObjectVTable {
//!     pub const fn new<T: Display + Debug>() -> &'static Self {
//!         &ObjectVTable {
//!             type_id: const { ConstTypeId::of::<T>() },
//!             drop_in_place: |ptr| unsafe { ptr::drop_in_place(ptr.cast::<T>()) },
//!             display: |ptr, f| unsafe { Display::fmt(&*ptr.cast::<T>(), f) },
//!             debug: |ptr, f| unsafe { Debug::fmt(&*ptr.cast::<T>(), f) },
//!         }
//!     }
//! }
//! ```
//!
//! and in associated constants:
//!
//! ```
//! use typeid::ConstTypeId;
//!
//! pub trait GetTypeId {
//!     const TYPEID: ConstTypeId;
//! }
//!
//! impl<T: 'static> GetTypeId for T {
//!     const TYPEID: ConstTypeId = ConstTypeId::of::<Self>();
//! }
//! ```
//!
//! <br>
//!
//! #### Non-'static `TypeId`
//!
//! This crate provides [`typeid::of`], which takes an arbitrary non-'static
//! type `T` and produces the `TypeId` for the type obtained by replacing all
//! lifetimes in `T` by `'static`, other than higher-rank lifetimes found in
//! trait objects.
//!
//! For example if `T` is `&'b dyn for<'a> Trait<'a, 'c>`, then
//! `typeid::of::<T>()` produces the TypeId of `&'static dyn for<'a> Trait<'a,
//! 'static>`.
//!
//! It should be obvious that unlike with the standard library's TypeId,
//! `typeid::of::<A>() == typeid::of::<B>()` does **not** mean that `A` and `B`
//! are the same type. However, there is a common special case where this
//! behavior is exactly what is needed. If:
//!
//! - `A` is an arbitrary non-'static type parameter, _and_
//! - `B` is 'static, _and_
//! - all types with the same id as `B` are also 'static
//!
//! then `typeid::of::<A>() == typeid::of::<B>()` guarantees that `A` and `B`
//! are the same type.
//!
//! ```
//! use core::any::TypeId;
//! use core::slice;
//!
//! pub fn example<T>(slice: &[T]) {
//!     // T is arbitrary and non-'static.
//!
//!     if typeid::of::<T>() == TypeId::of::<u8>() {
//!         // T is definitely u8
//!         let bytes = unsafe { slice::from_raw_parts(slice.as_ptr().cast(), slice.len()) };
//!         process_bytes(bytes);
//!     } else {
//!         for t in slice {
//!             process(t);
//!         }
//!     }
//! }
//!
//! fn process<T>(_: &T) {/* ... */}
//! fn process_bytes(_: &[u8]) {/* ... */}
//! ```

#![no_std]
#![doc(html_root_url = "https://docs.rs/typeid/1.0.3")]
#![allow(clippy::doc_markdown, clippy::inline_always)]

extern crate self as typeid;

use core::any::TypeId;
#[cfg(not(no_const_type_id))]
use core::cmp::Ordering;
#[cfg(not(no_const_type_id))]
use core::fmt::{self, Debug};
#[cfg(not(no_const_type_id))]
use core::hash::{Hash, Hasher};
use core::marker::PhantomData;
use core::mem;

#[cfg(not(no_const_type_id))]
#[derive(Copy, Clone)]
pub struct ConstTypeId {
    type_id_fn: fn() -> TypeId,
}

#[cfg(not(no_const_type_id))]
impl ConstTypeId {
    #[must_use]
    pub const fn of<T>() -> Self
    where
        T: ?Sized,
    {
        ConstTypeId {
            type_id_fn: typeid::of::<T>,
        }
    }

    #[inline]
    fn get(self) -> TypeId {
        (self.type_id_fn)()
    }
}

#[cfg(not(no_const_type_id))]
impl Debug for ConstTypeId {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(&self.get(), formatter)
    }
}

#[cfg(not(no_const_type_id))]
impl PartialEq for ConstTypeId {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.get() == other.get()
    }
}

#[cfg(not(no_const_type_id))]
impl PartialEq<TypeId> for ConstTypeId {
    fn eq(&self, other: &TypeId) -> bool {
        self.get() == *other
    }
}

#[cfg(not(no_const_type_id))]
impl Eq for ConstTypeId {}

#[cfg(not(no_const_type_id))]
impl PartialOrd for ConstTypeId {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

#[cfg(not(no_const_type_id))]
impl Ord for ConstTypeId {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(&self.get(), &other.get())
    }
}

#[cfg(not(no_const_type_id))]
impl Hash for ConstTypeId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.get().hash(state);
    }
}

#[must_use]
#[inline(always)]
pub fn of<T>() -> TypeId
where
    T: ?Sized,
{
    trait NonStaticAny {
        fn get_type_id(&self) -> TypeId
        where
            Self: 'static;
    }

    impl<T: ?Sized> NonStaticAny for PhantomData<T> {
        #[inline(always)]
        fn get_type_id(&self) -> TypeId
        where
            Self: 'static,
        {
            TypeId::of::<T>()
        }
    }

    let phantom_data = PhantomData::<T>;
    NonStaticAny::get_type_id(unsafe {
        mem::transmute::<&dyn NonStaticAny, &(dyn NonStaticAny + 'static)>(&phantom_data)
    })
}
