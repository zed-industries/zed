//! [`SecretBox`] wrapper type for more carefully handling secret values
//! (e.g. passwords, cryptographic keys, access tokens or other credentials)
//!
//! # Goals
//!
//! - Make secret access explicit and easy-to-audit via the
//!   [`ExposeSecret`] and [`ExposeSecretMut`] traits.
//! - Prevent accidental leakage of secrets via channels like debug logging
//! - Ensure secrets are wiped from memory on drop securely
//!   (using the [`zeroize`] crate)
//!
//! Presently this crate favors a simple, `no_std`-friendly, safe i.e.
//! `forbid(unsafe_code)`-based implementation and does not provide more advanced
//! memory protection mechanisms e.g. ones based on `mlock(2)`/`mprotect(2)`.
//! We may explore more advanced protection mechanisms in the future.
//! Those who don't mind `std` and `libc` dependencies should consider using
//! the [`secrets`](https://crates.io/crates/secrets) crate.
//!
//! # `serde` support
//!
//! When the `serde` feature of this crate is enabled, the [`SecretBox`] type will
//! receive a [`Deserialize`] impl for all `SecretBox<T>` types where
//! `T: DeserializeOwned`. This allows *loading* secret values from data
//! deserialized from `serde` (be careful to clean up any intermediate secrets
//! when doing this, e.g. the unparsed input!)
//!
//! To prevent exfiltration of secret values via `serde`, by default `SecretBox<T>`
//! does *not* receive a corresponding [`Serialize`] impl. If you would like
//! types of `SecretBox<T>` to be serializable with `serde`, you will need to impl
//! the [`SerializableSecret`] marker trait on `T`.

#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

extern crate alloc;

use alloc::{boxed::Box, string::String, vec::Vec};
use core::{
    any,
    fmt::{self, Debug},
};

use zeroize::{Zeroize, ZeroizeOnDrop};

#[cfg(feature = "serde")]
use serde::{de, ser, Deserialize, Serialize};

pub use zeroize;

/// Wrapper type for values that contains secrets, which attempts to limit
/// accidental exposure and ensure secrets are wiped from memory when dropped.
/// (e.g. passwords, cryptographic keys, access tokens or other credentials)
///
/// Access to the secret inner value occurs through the [`ExposeSecret`]
/// or [`ExposeSecretMut`] traits, which provide methods for accessing the inner secret value.
pub struct SecretBox<S: Zeroize + ?Sized> {
    inner_secret: Box<S>,
}

impl<S: Zeroize + ?Sized> Zeroize for SecretBox<S> {
    fn zeroize(&mut self) {
        self.inner_secret.as_mut().zeroize()
    }
}

impl<S: Zeroize + ?Sized> Drop for SecretBox<S> {
    fn drop(&mut self) {
        self.zeroize()
    }
}

impl<S: Zeroize + ?Sized> ZeroizeOnDrop for SecretBox<S> {}

impl<S: Zeroize + ?Sized> From<Box<S>> for SecretBox<S> {
    fn from(source: Box<S>) -> Self {
        Self::new(source)
    }
}

impl<S: Zeroize + ?Sized> SecretBox<S> {
    /// Create a secret value using a pre-boxed value.
    pub fn new(boxed_secret: Box<S>) -> Self {
        Self {
            inner_secret: boxed_secret,
        }
    }
}

impl<S: Zeroize + Default> SecretBox<S> {
    /// Create a secret value using a function that can initialize the value in-place.
    pub fn init_with_mut(ctr: impl FnOnce(&mut S)) -> Self {
        let mut secret = Self::default();
        ctr(secret.expose_secret_mut());
        secret
    }
}

impl<S: Zeroize + Clone> SecretBox<S> {
    /// Create a secret value using the provided function as a constructor.
    ///
    /// The implementation makes an effort to zeroize the locally constructed value
    /// before it is copied to the heap, and constructing it inside the closure minimizes
    /// the possibility of it being accidentally copied by other code.
    ///
    /// **Note:** using [`Self::new`] or [`Self::init_with_mut`] is preferable when possible,
    /// since this method's safety relies on empiric evidence and may be violated on some targets.
    pub fn init_with(ctr: impl FnOnce() -> S) -> Self {
        let mut data = ctr();
        let secret = Self {
            inner_secret: Box::new(data.clone()),
        };
        data.zeroize();
        secret
    }

    /// Same as [`Self::init_with`], but the constructor can be fallible.
    ///
    ///
    /// **Note:** using [`Self::new`] or [`Self::init_with_mut`] is preferable when possible,
    /// since this method's safety relies on empyric evidence and may be violated on some targets.
    pub fn try_init_with<E>(ctr: impl FnOnce() -> Result<S, E>) -> Result<Self, E> {
        let mut data = ctr()?;
        let secret = Self {
            inner_secret: Box::new(data.clone()),
        };
        data.zeroize();
        Ok(secret)
    }
}

impl<S: Zeroize + Default> Default for SecretBox<S> {
    fn default() -> Self {
        Self {
            inner_secret: Box::<S>::default(),
        }
    }
}

impl<S: Zeroize + ?Sized> Debug for SecretBox<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SecretBox<{}>([REDACTED])", any::type_name::<S>())
    }
}

impl<S> Clone for SecretBox<S>
where
    S: CloneableSecret,
{
    fn clone(&self) -> Self {
        SecretBox {
            inner_secret: self.inner_secret.clone(),
        }
    }
}

impl<S: Zeroize + ?Sized> ExposeSecret<S> for SecretBox<S> {
    fn expose_secret(&self) -> &S {
        self.inner_secret.as_ref()
    }
}

impl<S: Zeroize + ?Sized> ExposeSecretMut<S> for SecretBox<S> {
    fn expose_secret_mut(&mut self) -> &mut S {
        self.inner_secret.as_mut()
    }
}

/// Secret slice type.
///
/// This is a type alias for [`SecretBox<[S]>`] which supports some helpful trait impls.
///
/// Notably it has a [`From<Vec<S>>`] impl which is the preferred method for construction.
pub type SecretSlice<S> = SecretBox<[S]>;

impl<S> From<Vec<S>> for SecretSlice<S>
where
    S: Zeroize,
    [S]: Zeroize,
{
    fn from(vec: Vec<S>) -> Self {
        Self::from(vec.into_boxed_slice())
    }
}

impl<S> Clone for SecretSlice<S>
where
    S: CloneableSecret + Zeroize,
    [S]: Zeroize,
{
    fn clone(&self) -> Self {
        SecretBox {
            inner_secret: Vec::from(&*self.inner_secret).into_boxed_slice(),
        }
    }
}

impl<S> Default for SecretSlice<S>
where
    S: Zeroize,
    [S]: Zeroize,
{
    fn default() -> Self {
        Vec::new().into()
    }
}

/// Secret string type.
///
/// This is a type alias for [`SecretBox<str>`] which supports some helpful trait impls.
///
/// Notably it has a [`From<String>`] impl which is the preferred method for construction.
pub type SecretString = SecretBox<str>;

impl From<String> for SecretString {
    fn from(s: String) -> Self {
        Self::from(s.into_boxed_str())
    }
}

impl From<&str> for SecretString {
    fn from(s: &str) -> Self {
        Self::from(String::from(s))
    }
}

impl Clone for SecretString {
    fn clone(&self) -> Self {
        SecretBox {
            inner_secret: self.inner_secret.clone(),
        }
    }
}

impl Default for SecretString {
    fn default() -> Self {
        String::default().into()
    }
}

/// Marker trait for secrets which are allowed to be cloned
pub trait CloneableSecret: Clone + Zeroize {}

// Mark integer primitives as cloneable secrets

impl CloneableSecret for i8 {}
impl CloneableSecret for i16 {}
impl CloneableSecret for i32 {}
impl CloneableSecret for i64 {}
impl CloneableSecret for i128 {}
impl CloneableSecret for isize {}

impl CloneableSecret for u8 {}
impl CloneableSecret for u16 {}
impl CloneableSecret for u32 {}
impl CloneableSecret for u64 {}
impl CloneableSecret for u128 {}
impl CloneableSecret for usize {}

/// Expose a reference to an inner secret
pub trait ExposeSecret<S: ?Sized> {
    /// Expose secret: this is the only method providing access to a secret.
    fn expose_secret(&self) -> &S;
}

/// Expose a mutable reference to an inner secret
pub trait ExposeSecretMut<S: ?Sized> {
    /// Expose secret: this is the only method providing access to a secret.
    fn expose_secret_mut(&mut self) -> &mut S;
}

/// Marker trait for secret types which can be [`Serialize`]-d by [`serde`].
///
/// When the `serde` feature of this crate is enabled and types are marked with
/// this trait, they receive a [`Serialize` impl][1] for `SecretBox<T>`.
/// (NOTE: all types which impl `DeserializeOwned` receive a [`Deserialize`]
/// impl)
///
/// This is done deliberately to prevent accidental exfiltration of secrets
/// via `serde` serialization.
///
/// If you really want to have `serde` serialize those types, use the
/// [`serialize_with`][2] attribute to specify a serializer that exposes the secret.
///
/// [1]: https://docs.rs/secrecy/latest/secrecy/struct.Secret.html#implementations
/// [2]: https://serde.rs/field-attrs.html#serialize_with
#[cfg(feature = "serde")]
pub trait SerializableSecret: Serialize {}

#[cfg(feature = "serde")]
impl<'de, T> Deserialize<'de> for SecretBox<T>
where
    T: Zeroize + Clone + de::DeserializeOwned + Sized,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        Self::try_init_with(|| T::deserialize(deserializer))
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for SecretString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        String::deserialize(deserializer).map(Into::into)
    }
}

#[cfg(feature = "serde")]
impl<T> Serialize for SecretBox<T>
where
    T: Zeroize + SerializableSecret + Serialize + Sized,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self.expose_secret().serialize(serializer)
    }
}
