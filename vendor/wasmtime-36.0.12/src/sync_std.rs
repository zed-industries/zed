//! Synchronization primitives for Wasmtime.
//!
//! This is a small set of primitives split between std and no_std with "dummy"
//! implementation on no_std. The no_std implementations live in
//! `sync_nostd.rs`.

use once_cell::sync::OnceCell;
use std::ops::{Deref, DerefMut};

/// This type is intended to mirror, and one day be implemented by, the
/// `std::sync::OnceLock` type. At this time
/// `std::sync::OnceLock::get_or_try_init` is not stable so for now this is
/// implemented with the `once_cell` crate instead.
pub struct OnceLock<T>(OnceCell<T>);

impl<T> OnceLock<T> {
    #[inline]
    pub const fn new() -> OnceLock<T> {
        OnceLock(OnceCell::new())
    }

    #[inline]
    pub fn get_or_init(&self, f: impl FnOnce() -> T) -> &T {
        self.0.get_or_init(f)
    }

    #[inline]
    pub fn get_or_try_init<E>(&self, f: impl FnOnce() -> Result<T, E>) -> Result<&T, E> {
        self.0.get_or_try_init(f)
    }
}

impl<T> Default for OnceLock<T> {
    fn default() -> OnceLock<T> {
        OnceLock::new()
    }
}

/// Small wrapper around `std::sync::RwLock` which undoes poisoning.
#[derive(Debug, Default)]
pub struct RwLock<T>(std::sync::RwLock<T>);

impl<T> RwLock<T> {
    #[inline]
    pub const fn new(val: T) -> RwLock<T> {
        RwLock(std::sync::RwLock::new(val))
    }

    #[inline]
    pub fn read(&self) -> impl Deref<Target = T> + '_ {
        self.0.read().unwrap()
    }

    #[inline]
    pub fn write(&self) -> impl DerefMut<Target = T> + '_ {
        self.0.write().unwrap()
    }
}
