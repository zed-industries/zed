//! A framework for Rust wrappers over C APIs.
//!
//! Ownership is as important in C as it is in Rust, but the semantics are often implicit. In
//! particular, pointer-to-value is commonly used to pass C values both when transferring ownership
//! or a borrow.
//!
//! This crate provides a framework to define a Rust wrapper over these kinds of raw C APIs in a way
//! that allows ownership semantics to be expressed in an ergonomic manner. The framework takes a
//! dual-type approach similar to APIs in the standard library such as `PathBuf`/`Path` or `String`/
//! `str`. One type represents an owned value and references to the other represent borrowed
//! values.
//!
//! # Examples
//!
//! ```
//! use foreign_types::{ForeignType, ForeignTypeRef, Opaque};
//! use std::ops::{Deref, DerefMut};
//! use std::ptr::NonNull;
//!
//! mod foo_sys {
//!     pub enum FOO {}
//!
//!     extern {
//!         pub fn FOO_free(foo: *mut FOO);
//!     }
//! }
//!
//! // The borrowed type is a newtype wrapper around an `Opaque` value.
//! //
//! // `FooRef` values never exist; we instead create references to `FooRef`s
//! // from raw C pointers.
//! pub struct FooRef(Opaque);
//!
//! unsafe impl ForeignTypeRef for FooRef {
//!     type CType = foo_sys::FOO;
//! }
//!
//! // The owned type is simply a newtype wrapper around the raw C type.
//! //
//! // It dereferences to `FooRef`, so methods that do not require ownership
//! // should be defined there.
//! pub struct Foo(NonNull<foo_sys::FOO>);
//!
//! unsafe impl Sync for FooRef {}
//! unsafe impl Send for FooRef {}
//!
//! unsafe impl Sync for Foo {}
//! unsafe impl Send for Foo {}
//!
//! impl Drop for Foo {
//!     fn drop(&mut self) {
//!         unsafe { foo_sys::FOO_free(self.as_ptr()) }
//!     }
//! }
//!
//! unsafe impl ForeignType for Foo {
//!     type CType = foo_sys::FOO;
//!     type Ref = FooRef;
//!
//!     unsafe fn from_ptr(ptr: *mut foo_sys::FOO) -> Foo {
//!         Foo(NonNull::new_unchecked(ptr))
//!     }
//!
//!     fn as_ptr(&self) -> *mut foo_sys::FOO {
//!         self.0.as_ptr()
//!     }
//!
//!     fn into_ptr(self) -> *mut foo_sys::FOO {
//!         let inner = self.as_ptr();
//!         ::core::mem::forget(self);
//!         inner
//!     }
//! }
//!
//! impl Deref for Foo {
//!     type Target = FooRef;
//!
//!     fn deref(&self) -> &FooRef {
//!         unsafe { FooRef::from_ptr(self.as_ptr()) }
//!     }
//! }
//!
//! impl DerefMut for Foo {
//!     fn deref_mut(&mut self) -> &mut FooRef {
//!         unsafe { FooRef::from_ptr_mut(self.as_ptr()) }
//!     }
//! }
//!
//! // add in Borrow, BorrowMut, AsRef, AsRefMut, Clone, ToOwned...
//! ```
//!
//! The `foreign_type!` macro can generate this boilerplate for you:
//!
//! ```
//! use foreign_types::foreign_type;
//!
//! mod foo_sys {
//!     pub enum FOO {}
//!
//!     extern {
//!         pub fn FOO_free(foo: *mut FOO);
//!         pub fn FOO_duplicate(foo: *mut FOO) -> *mut FOO; // optional
//!     }
//! }
//!
//! foreign_type! {
//!     /// A Foo.
//!     pub unsafe type Foo
//!         : Sync + Send // optional
//!     {
//!         type CType = foo_sys::FOO;
//!         fn drop = foo_sys::FOO_free;
//!         fn clone = foo_sys::FOO_duplicate; // optional
//!     }
//!
//!     /// A Foo with generic parameters.
//!     pub unsafe type GenericFoo<T> {
//!         type CType = foo_sys::FOO;
//!         // This type is added as a `PhantomData` field to handle variance
//!         // of the parameters. However, it has no impact on trait impls:
//!         // `GenericFoo<T>` is always `Clone`, even if `T` is not.
//!         type PhantomData = T;
//!         fn drop = foo_sys::FOO_free;
//!         fn clone = foo_sys::FOO_duplicate;
//!     }
//! }
//!
//! # fn main() {}
//! ```
//!
//! If `fn clone` is specified, then it must take `CType` as an argument and return a copy of it as `CType`.
//! It will be used to implement `Clone`, and if the `std` Cargo feature is enabled, `ToOwned`.
//!
//! Say we then have a separate type in our C API that contains a `FOO`:
//!
//! ```
//! mod foo_sys {
//!     pub enum FOO {}
//!     pub enum BAR {}
//!
//!     extern {
//!         pub fn FOO_free(foo: *mut FOO);
//!         pub fn BAR_free(bar: *mut BAR);
//!         pub fn BAR_get_foo(bar: *mut BAR) -> *mut FOO;
//!     }
//! }
//! ```
//!
//! The documentation for the C library states that `BAR_get_foo` returns a reference into the `BAR`
//! passed to it, which translates into a reference in Rust. It also says that we're allowed to
//! modify the `FOO`, so we'll define a pair of accessor methods, one immutable and one mutable:
//!
//! ```
//! use foreign_types::{ForeignTypeRef, foreign_type};
//!
//! mod foo_sys {
//!     pub enum FOO {}
//!     pub enum BAR {}
//!
//!     extern {
//!         pub fn FOO_free(foo: *mut FOO);
//!         pub fn BAR_free(bar: *mut BAR);
//!         pub fn BAR_get_foo(bar: *mut BAR) -> *mut FOO;
//!     }
//! }
//!
//! foreign_type! {
//!     /// A Foo.
//!     pub unsafe type Foo: Sync + Send {
//!         type CType = foo_sys::FOO;
//!         fn drop = foo_sys::FOO_free;
//!     }
//!
//!     /// A Bar.
//!     pub unsafe type Bar: Sync + Send {
//!         type CType = foo_sys::BAR;
//!         fn drop = foo_sys::BAR_free;
//!     }
//! }
//!
//! impl BarRef {
//!     fn foo(&self) -> &FooRef {
//!         unsafe { FooRef::from_ptr(foo_sys::BAR_get_foo(self.as_ptr())) }
//!     }
//!
//!     fn foo_mut(&mut self) -> &mut FooRef {
//!         unsafe { FooRef::from_ptr_mut(foo_sys::BAR_get_foo(self.as_ptr())) }
//!     }
//! }
//!
//! # fn main() {}
//! ```
#![no_std]
#![warn(missing_docs)]
#![doc(html_root_url = "https://docs.rs/foreign-types/0.5")]

#[cfg(feature = "std")]
extern crate std;

#[doc(hidden)]
pub use foreign_types_macros::foreign_type_impl;
#[doc(inline)]
pub use foreign_types_shared::{ForeignType, ForeignTypeRef, Opaque};

#[doc(hidden)]
pub mod export {
    pub use core::borrow::{Borrow, BorrowMut};
    pub use core::clone::Clone;
    pub use core::convert::{AsMut, AsRef};
    pub use core::marker::{PhantomData, Send, Sync};
    pub use core::ops::{Deref, DerefMut, Drop};
    pub use core::ptr::NonNull;

    #[cfg(feature = "std")]
    pub use std::borrow::ToOwned;
}

/// A macro to easily define wrappers for foreign types.
///
/// # Examples
///
/// ```
/// use foreign_types::foreign_type;
///
/// # mod openssl_sys { pub type SSL = (); pub unsafe fn SSL_free(_: *mut SSL) {} pub unsafe fn SSL_dup(x: *mut SSL) -> *mut SSL {x} }
/// # mod foo_sys { pub type THING = (); pub unsafe fn THING_free(_: *mut THING) {} }
/// foreign_type! {
///     /// Documentation for the owned type.
///     pub unsafe type Ssl: Sync + Send {
///         type CType = openssl_sys::SSL;
///         fn drop = openssl_sys::SSL_free;
///         fn clone = openssl_sys::SSL_dup;
///     }
///
///     /// This type immutably borrows other data and has a limited lifetime!
///     pub unsafe type Thing<'a>: Send {
///         type CType = foo_sys::THING;
///         type PhantomData = &'a ();
///         fn drop = foo_sys::THING_free;
///     }
/// }
///
/// # fn main() {}
/// ```
#[macro_export(local_inner_macros)]
macro_rules! foreign_type {
    ($($t:tt)*) => {
        $crate::foreign_type_impl!($crate $($t)*);
    };
}
