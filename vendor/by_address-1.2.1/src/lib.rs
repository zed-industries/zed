//! Wrapper type for by-address hashing and comparison.
//!
//! [`ByAddress`] can be used to wrap any pointer type (i.e. any type that implements the Deref
//! trait).  This includes references, raw pointers, smart pointers like `Rc<T>` and `Box<T>`, and
//! specialized pointer-like types such as `Vec<T>` and `String`.
//!
//! Comparison, ordering, and hashing of the wrapped pointer will be based on the address of its
//! contents, rather than their value.
//!
//! ```
//! use by_address::ByAddress;
//! use std::rc::Rc;
//!
//! let rc = Rc::new(5);
//! let x = ByAddress(rc.clone());
//! let y = ByAddress(rc.clone());
//!
//! // x and y are two pointers to the same address:
//! assert_eq!(x, y);
//!
//! let z = ByAddress(Rc::new(5));
//!
//! // *x and *z have the same value, but not the same address:
//! assert_ne!(x, z);
//! ```
//!
//! If `T` is a pointer to an unsized type, then comparison of `ByAddress<T>` uses the
//! entire fat pointer, not just the "thin" data address.  This means that two slice pointers
//! are consider equal only if they have the same starting address *and* length.
//!
//! ```
//! # use by_address::ByAddress;
//! #
//! let v = [1, 2, 3, 4];
//!
//! assert_eq!(ByAddress(&v[0..4]), ByAddress(&v[0..4])); // Same address and length.
//! assert_ne!(ByAddress(&v[0..4]), ByAddress(&v[0..2])); // Same address, different length.
//! ```
//!
//! You can use [`ByThinAddress`] instead if you want to compare slices by starting address only,
//! or trait objects by data pointer only.
//!
//! You can use wrapped pointers as keys in hashed or ordered collections, like BTreeMap/BTreeSet
//! or HashMap/HashSet, even if the target of the pointer doesn't implement hashing or ordering.
//! This even includes pointers to trait objects, which usually don't implement the Eq trait
//! because it is not object-safe.
//!
//! ```
//! # use by_address::ByAddress;
//! # use std::collections::HashSet;
//! #
//! /// Call each item in `callbacks`, skipping any duplicate references.
//! fn call_each_once(callbacks: &[&dyn Fn()]) {
//!     let mut seen: HashSet<ByAddress<&dyn Fn()>> = HashSet::new();
//!     for &f in callbacks {
//!         if seen.insert(ByAddress(f)) {
//!             f();
//!         }
//!     }
//! }
//! ```
//!
//! However, note that comparing fat pointers to trait objects can be unreliable because of
//! [Rust issue #46139](https://github.com/rust-lang/rust/issues/46139).  In some cases,
//! [`ByThinAddress`] may be more useful.
//!
//! This crate does not depend on libstd, so it can be used in [`no_std`] projects.
//!
//! [`no_std`]: https://doc.rust-lang.org/book/first-edition/using-rust-without-the-standard-library.html

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![no_std]

use core::cmp::Ordering;
use core::convert::AsRef;
use core::fmt::{Debug, Display, Formatter};
use core::hash::{Hash, Hasher};
use core::ops::{Deref, DerefMut};
use core::ptr;

/// Wrapper for pointer types that implements by-address comparison.
///
/// See the [crate-level documentation](index.html) for details.
///
/// Equality tests and hashes on fat pointers (`&dyn Trait`, `&[T]`, `&str`, etc)
/// include the attribute of the fat pointer.
///
/// However, note that comparing fat pointers to trait objects can be unreliable because of
/// [Rust issue #46139](https://github.com/rust-lang/rust/issues/46139).  In some cases,
/// [`ByThinAddress`] may be more useful.
#[repr(transparent)]
#[derive(Copy, Clone, Default)]
pub struct ByAddress<T>(pub T)
where
    T: ?Sized + Deref;

impl<T> ByAddress<T>
where
    T: ?Sized + Deref,
{
    /// Convenience method for pointer casts.
    fn addr(&self) -> *const T::Target {
        &*self.0
    }

    /// Convert `&T` to `&ByAddress<T>`.
    pub fn from_ref(r: &T) -> &Self {
        // SAFETY: `struct ByAddress` is `repr(transparent)`.
        unsafe {
            &*(r as *const T as *const Self)
        }
    }
}

struct DebugAdapter<'a, T>(&'a T)
where
    T: ?Sized + Deref + Debug;

impl<'a, T> Debug for DebugAdapter<'a, T>
where
    T: ?Sized + Deref + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)?;
        f.write_str(" @ ")?;
        (self.0.deref() as *const T::Target).fmt(f)?;
        Ok(())
    }
}

impl<T> Debug for ByAddress<T>
where
    T: ?Sized + Deref + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ByAddress")
            .field(&DebugAdapter(&self.0))
            .finish()
    }
}

impl<T> Display for ByAddress<T>
where
    T: ?Sized + Deref + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

/// Raw pointer equality
impl<T> PartialEq for ByAddress<T>
where
    T: ?Sized + Deref,
{
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self.addr(), other.addr())
    }
}
impl<T> Eq for ByAddress<T> where T: ?Sized + Deref {}

/// Raw pointer ordering
impl<T> Ord for ByAddress<T>
where
    T: ?Sized + Deref,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.addr().cmp(&other.addr())
    }
}

/// Raw pointer comparison
impl<T> PartialOrd for ByAddress<T>
where
    T: ?Sized + Deref,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.addr().cmp(&other.addr()))
    }
}

/// Raw pointer hashing
impl<T> Hash for ByAddress<T>
where
    T: ?Sized + Deref,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.addr().hash(state)
    }
}

// Generic conversion traits:

impl<T> Deref for ByAddress<T>
where
    T: ?Sized + Deref,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for ByAddress<T>
where
    T: ?Sized + Deref,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T, U> AsRef<U> for ByAddress<T>
where
    T: ?Sized + Deref + AsRef<U>,
{
    fn as_ref(&self) -> &U {
        self.0.as_ref()
    }
}

impl<T, U> AsMut<U> for ByAddress<T>
where
    T: ?Sized + Deref + AsMut<U>,
{
    fn as_mut(&mut self) -> &mut U {
        self.0.as_mut()
    }
}

impl<T> From<T> for ByAddress<T>
where
    T: Deref,
{
    fn from(t: T) -> ByAddress<T> {
        ByAddress(t)
    }
}

/// Similar to [`ByAddress`], but omits the attributes of fat pointers.
///
/// This means that two slices with the same starting element but different lengths will be
/// considered equal.
///
/// Two trait objects with the same data pointer but different vtables will also be considered
/// equal.  (In particular, this may happen for traits that are implemented on zero-sized types,
/// including `Fn` and other closure traits.)
#[repr(transparent)]
#[derive(Copy, Clone, Default)]
pub struct ByThinAddress<T>(pub T)
where
    T: ?Sized + Deref;

impl<T> ByThinAddress<T>
where
    T: ?Sized + Deref,
{
    /// Convenience method for pointer casts.
    fn addr(&self) -> *const T::Target {
        &*self.0
    }

    /// Convert `&T` to `&ByThinAddress<T>`.
    pub fn from_ref(r: &T) -> &Self {
        // SAFETY: `struct ByAddress` is `repr(transparent)`.
        unsafe {
            &*(r as *const T as *const Self)
        }
    }
}

impl<T> Debug for ByThinAddress<T>
where
    T: ?Sized + Deref + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ByThinAddress")
            .field(&DebugAdapter(&self.0))
            .finish()
    }
}

impl<T> Display for ByThinAddress<T>
where
    T: ?Sized + Deref + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

/// Raw pointer equality
impl<T> PartialEq for ByThinAddress<T>
where
    T: ?Sized + Deref,
{
    fn eq(&self, other: &Self) -> bool {
        core::ptr::eq(self.addr() as *const (), other.addr() as *const _)
    }
}
impl<T> Eq for ByThinAddress<T> where T: ?Sized + Deref {}

/// Raw pointer ordering
impl<T> Ord for ByThinAddress<T>
where
    T: ?Sized + Deref,
{
    fn cmp(&self, other: &Self) -> Ordering {
        (self.addr() as *const ()).cmp(&(other.addr() as *const ()))
    }
}

/// Raw pointer comparison
impl<T> PartialOrd for ByThinAddress<T>
where
    T: ?Sized + Deref,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some((self.addr() as *const ()).cmp(&(other.addr() as *const ())))
    }
}

/// Raw pointer hashing
impl<T> Hash for ByThinAddress<T>
where
    T: ?Sized + Deref,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.addr() as *const ()).hash(state)
    }
}

// Generic conversion traits:

impl<T> Deref for ByThinAddress<T>
where
    T: ?Sized + Deref,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for ByThinAddress<T>
where
    T: ?Sized + Deref,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T, U> AsRef<U> for ByThinAddress<T>
where
    T: ?Sized + Deref + AsRef<U>,
{
    fn as_ref(&self) -> &U {
        self.0.as_ref()
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use std::format;

    use crate::{ByAddress, ByThinAddress};

    trait A: std::fmt::Debug {
        fn test(&self) {}
    }
    trait B: A {
        fn test2(&self) {}
    }

    #[derive(Debug)]
    struct Test {}
    impl A for Test {}
    impl B for Test {}

    fn force_vtable<O: B>(v: &O) -> &dyn A {
        v
    }

    #[test]
    fn test_thin_ptr_fail() {
        let t = Test {};
        let tr1: &dyn A = &t;
        let tr2: &dyn A = force_vtable(&t);

        let a = ByAddress(tr1);
        let b = ByAddress(tr2);

        assert_ne!(a, b);
    }

    #[test]
    fn test_thin_ptr_success() {
        let t = Test {};
        let tr1: &dyn A = &t;
        let tr2: &dyn A = force_vtable(&t);

        let a = ByThinAddress(tr1);
        let b = ByThinAddress(tr2);

        assert_eq!(a, b);
    }

    #[test]
    fn test_debug() {
        let x = &1;
        let b = ByAddress(x);
        let expected = format!("ByAddress(1 @ {:p})", x);
        let actual = format!("{:?}", b);
        assert_eq!(expected, actual);

        let t = ByThinAddress(x);
        let expected = format!("ByThinAddress(1 @ {:p})", x);
        let actual = format!("{:?}", t);
        assert_eq!(expected, actual);
    }
}
