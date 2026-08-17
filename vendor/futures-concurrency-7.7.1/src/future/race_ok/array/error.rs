use core::fmt;
use core::ops::{Deref, DerefMut};
#[cfg(feature = "std")]
use std::error::Error;

/// A collection of errors.
#[repr(transparent)]
pub struct AggregateError<E, const N: usize> {
    inner: [E; N],
}

impl<E, const N: usize> AggregateError<E, N> {
    pub(super) fn new(inner: [E; N]) -> Self {
        Self { inner }
    }
}

impl<E: fmt::Display, const N: usize> fmt::Debug for AggregateError<E, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{self}:")?;

        for (i, err) in self.inner.iter().enumerate() {
            writeln!(f, "- Error {}: {err}", i + 1)?;
        }

        Ok(())
    }
}

impl<E: fmt::Display, const N: usize> fmt::Display for AggregateError<E, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} errors occured", self.inner.len())
    }
}

impl<E, const N: usize> Deref for AggregateError<E, N> {
    type Target = [E; N];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<E, const N: usize> DerefMut for AggregateError<E, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[cfg(feature = "std")]
impl<E: Error, const N: usize> std::error::Error for AggregateError<E, N> {}
