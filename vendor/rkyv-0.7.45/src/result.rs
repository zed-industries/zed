//! An archived version of `Result`.

use core::{
    cmp::{Ord, Ordering, PartialOrd},
    hash, mem,
    ops::{Deref, DerefMut},
};

/// An archived [`Result`] that represents either success ([`Ok`](ArchivedResult::Ok)) or failure
/// ([`Err`](ArchivedResult::Err)).
#[derive(Debug)]
#[cfg_attr(feature = "validation", derive(bytecheck::CheckBytes))]
#[repr(u8)]
pub enum ArchivedResult<T, E> {
    /// Contains the success value
    Ok(T),
    /// Contains the error value
    Err(E),
}

impl<T, E> ArchivedResult<T, E> {
    /// Converts from `ArchivedResult<T, E>` to `Option<T>`.
    pub fn ok(self) -> Option<T> {
        match self {
            ArchivedResult::Ok(value) => Some(value),
            ArchivedResult::Err(_) => None,
        }
    }
    /// Returns the contained [`Ok`](ArchivedResult::Ok) value, consuming the `self` value.
    pub fn unwrap(self) -> T {
        match self {
            ArchivedResult::Ok(value) => value,
            ArchivedResult::Err(_) => panic!("called `ArchivedResult::unwrap()` on an `Err` value"),
        }
    }
    /// Returns the contained `Ok` value or computes it from a closure.
    pub fn unwrap_or_else<F>(self, op: F) -> T
    where
        F: FnOnce(E) -> T,
    {
        match self {
            ArchivedResult::Ok(t) => t,
            ArchivedResult::Err(e) => op(e),
        }
    }
    /// Returns `true` if the result is [`Ok`](ArchivedResult::Ok).
    #[inline]
    pub const fn is_ok(&self) -> bool {
        matches!(self, ArchivedResult::Ok(_))
    }

    /// Returns `true` if the result is [`Err`](ArchivedResult::Err).
    #[inline]
    pub const fn is_err(&self) -> bool {
        matches!(self, ArchivedResult::Err(_))
    }

    /// Returns a `Result` containing the success and error values of this `ArchivedResult`.
    #[inline]
    pub fn as_ref(&self) -> Result<&T, &E> {
        match self {
            ArchivedResult::Ok(value) => Ok(value),
            ArchivedResult::Err(err) => Err(err),
        }
    }

    /// Converts from `&mut ArchivedResult<T, E>` to `Result<&mut T, &mut E>`.
    #[inline]
    pub fn as_mut(&mut self) -> Result<&mut T, &mut E> {
        match self {
            ArchivedResult::Ok(value) => Ok(value),
            ArchivedResult::Err(err) => Err(err),
        }
    }

    /// Returns an iterator over the possibly contained value.
    ///
    /// The iterator yields one value if the result is `ArchivedResult::Ok`, otherwise none.
    #[inline]
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            inner: self.as_ref().ok(),
        }
    }

    /// Returns a mutable iterator over the possibly contained value.
    ///
    /// The iterator yields one value if the result is `ArchivedResult::Ok`, otherwise none.
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            inner: self.as_mut().ok(),
        }
    }
}

impl<T: Deref, E> ArchivedResult<T, E> {
    /// Converts from `&ArchivedResult<T, E>` to `Result<&<T as Deref>::Target, &E>`.
    ///
    /// Coerces the `Ok` variant of the original `ArchivedResult` via `Deref` and returns the new
    /// `Result`.
    #[inline]
    pub fn as_deref(&self) -> Result<&<T as Deref>::Target, &E> {
        match self {
            ArchivedResult::Ok(value) => Ok(value.deref()),
            ArchivedResult::Err(err) => Err(err),
        }
    }
}

impl<T: DerefMut, E> ArchivedResult<T, E> {
    /// Converts from `&mut ArchivedResult<T, E>` to `Result<&mut <T as Deref>::Target, &mut E>`.
    ///
    /// Coerces the `Ok` variant of the original `ArchivedResult` via `DerefMut` and returns the new
    /// `Result`.
    #[inline]
    pub fn as_deref_mut(&mut self) -> Result<&mut <T as Deref>::Target, &mut E> {
        match self {
            ArchivedResult::Ok(value) => Ok(value.deref_mut()),
            ArchivedResult::Err(err) => Err(err),
        }
    }
}

/// An iterator over a reference to the `Ok` variant of an [`ArchivedResult`].
///
/// The iterator yields one value if the result is `Ok`, otherwise none.
///
/// Created by [`ArchivedResult::iter`].
pub struct Iter<'a, T> {
    inner: Option<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = None;
        mem::swap(&mut self.inner, &mut result);
        result
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

/// An iterator over a mutable reference to the `Ok` variant of an [`ArchivedResult`].
///
/// The iterator yields one value if the result is `Ok`, otherwise none.
///
/// Created by [`ArchivedResult::iter_mut`].
pub struct IterMut<'a, T> {
    inner: Option<&'a mut T>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = None;
        mem::swap(&mut self.inner, &mut result);
        result
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

impl<T: Eq, E: Eq> Eq for ArchivedResult<T, E> {}

impl<T: hash::Hash, E: hash::Hash> hash::Hash for ArchivedResult<T, E> {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.as_ref().hash(state)
    }
}

impl<T: Ord, E: Ord> Ord for ArchivedResult<T, E> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_ref().cmp(&other.as_ref())
    }
}

impl<T: PartialEq, E: PartialEq> PartialEq for ArchivedResult<T, E> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.as_ref().eq(&other.as_ref())
    }
}

impl<T: PartialOrd, E: PartialOrd> PartialOrd for ArchivedResult<T, E> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_ref().partial_cmp(&other.as_ref())
    }
}

impl<T, U: PartialEq<T>, E, F: PartialEq<E>> PartialEq<Result<T, E>> for ArchivedResult<U, F> {
    #[inline]
    fn eq(&self, other: &Result<T, E>) -> bool {
        match self {
            ArchivedResult::Ok(self_value) => {
                if let Ok(other_value) = other {
                    self_value.eq(other_value)
                } else {
                    false
                }
            }
            ArchivedResult::Err(self_err) => {
                if let Err(other_err) = other {
                    self_err.eq(other_err)
                } else {
                    false
                }
            }
        }
    }
}

impl<T: PartialEq<U>, U, E: PartialEq<F>, F> PartialEq<ArchivedResult<T, E>> for Result<U, F> {
    #[inline]
    fn eq(&self, other: &ArchivedResult<T, E>) -> bool {
        other.eq(self)
    }
}
