//! Parallel iterator types for [results]
//!
//! You will rarely need to interact with this module directly unless you need
//! to name one of the iterator types.
//!
//! [results]: std::result

use crate::iter::plumbing::*;
use crate::iter::*;
use std::sync::Mutex;

use crate::option;

/// Parallel iterator over a result
#[derive(Debug, Clone)]
pub struct IntoIter<T> {
    inner: option::IntoIter<T>,
}

impl<T: Send, E> IntoParallelIterator for Result<T, E> {
    type Item = T;
    type Iter = IntoIter<T>;

    fn into_par_iter(self) -> Self::Iter {
        IntoIter {
            inner: self.ok().into_par_iter(),
        }
    }
}

delegate_indexed_iterator! {
    IntoIter<T> => T,
    impl<T: Send>
}

/// Parallel iterator over an immutable reference to a result
#[derive(Debug)]
pub struct Iter<'a, T> {
    inner: option::IntoIter<&'a T>,
}

impl<T> Clone for Iter<'_, T> {
    fn clone(&self) -> Self {
        Iter {
            inner: self.inner.clone(),
        }
    }
}

impl<'a, T: Sync, E> IntoParallelIterator for &'a Result<T, E> {
    type Item = &'a T;
    type Iter = Iter<'a, T>;

    fn into_par_iter(self) -> Self::Iter {
        Iter {
            inner: self.as_ref().ok().into_par_iter(),
        }
    }
}

delegate_indexed_iterator! {
    Iter<'a, T> => &'a T,
    impl<'a, T: Sync>
}

/// Parallel iterator over a mutable reference to a result
#[derive(Debug)]
pub struct IterMut<'a, T> {
    inner: option::IntoIter<&'a mut T>,
}

impl<'a, T: Send, E> IntoParallelIterator for &'a mut Result<T, E> {
    type Item = &'a mut T;
    type Iter = IterMut<'a, T>;

    fn into_par_iter(self) -> Self::Iter {
        IterMut {
            inner: self.as_mut().ok().into_par_iter(),
        }
    }
}

delegate_indexed_iterator! {
    IterMut<'a, T> => &'a mut T,
    impl<'a, T: Send>
}

/// Collect an arbitrary `Result`-wrapped collection.
///
/// If any item is `Err`, then all previous `Ok` items collected are
/// discarded, and it returns that error.  If there are multiple errors, the
/// one returned is not deterministic.
impl<C, T, E> FromParallelIterator<Result<T, E>> for Result<C, E>
where
    C: FromParallelIterator<T>,
    T: Send,
    E: Send,
{
    fn from_par_iter<I>(par_iter: I) -> Self
    where
        I: IntoParallelIterator<Item = Result<T, E>>,
    {
        fn ok<T, E>(saved: &Mutex<Option<E>>) -> impl Fn(Result<T, E>) -> Option<T> + '_ {
            move |item| match item {
                Ok(item) => Some(item),
                Err(error) => {
                    // We don't need a blocking `lock()`, as anybody
                    // else holding the lock will also be writing
                    // `Some(error)`, and then ours is irrelevant.
                    if let Ok(mut guard) = saved.try_lock() {
                        if guard.is_none() {
                            *guard = Some(error);
                        }
                    }
                    None
                }
            }
        }

        let saved_error = Mutex::new(None);
        let collection = par_iter
            .into_par_iter()
            .map(ok(&saved_error))
            .while_some()
            .collect();

        match saved_error.into_inner().unwrap() {
            Some(error) => Err(error),
            None => Ok(collection),
        }
    }
}
