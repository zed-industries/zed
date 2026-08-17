#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::vec::Vec;

use core::fmt;
use core::ops::Deref;
use core::ops::DerefMut;
#[cfg(feature = "std")]
use std::error::Error;

/// A collection of errors.
#[repr(transparent)]
pub struct AggregateError<E> {
    pub(crate) inner: Vec<E>,
}

impl<E> AggregateError<E> {
    pub(crate) fn new(inner: Vec<E>) -> Self {
        Self { inner }
    }
}

impl<E: fmt::Display> fmt::Debug for AggregateError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{self}:")?;

        for (i, err) in self.inner.iter().enumerate() {
            writeln!(f, "- Error {}: {err}", i + 1)?;
        }

        Ok(())
    }
}

impl<E: fmt::Display> fmt::Display for AggregateError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} errors occurred", self.inner.len())
    }
}

impl<E> Deref for AggregateError<E> {
    type Target = Vec<E>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<E> DerefMut for AggregateError<E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[cfg(feature = "std")]
impl<E: Error> Error for AggregateError<E> {}
