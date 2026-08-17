//! This module contains the parallel iterator types for B-Tree sets
//! (`BTreeSet<T>`). You will rarely need to interact with it directly
//! unless you have need to name one of the iterator types.

use std::collections::BTreeSet;

use crate::iter::plumbing::*;
use crate::iter::*;

use crate::vec;

/// Parallel iterator over a B-Tree set
#[derive(Debug)] // std doesn't Clone
pub struct IntoIter<T> {
    inner: vec::IntoIter<T>,
}

into_par_vec! {
    BTreeSet<T> => IntoIter<T>,
    impl<T: Send>
}

delegate_iterator! {
    IntoIter<T> => T,
    impl<T: Send>
}

/// Parallel iterator over an immutable reference to a B-Tree set
#[derive(Debug)]
pub struct Iter<'a, T> {
    inner: vec::IntoIter<&'a T>,
}

impl<T> Clone for Iter<'_, T> {
    fn clone(&self) -> Self {
        Iter {
            inner: self.inner.clone(),
        }
    }
}

into_par_vec! {
    &'a BTreeSet<T> => Iter<'a, T>,
    impl<'a, T: Sync>
}

delegate_iterator! {
    Iter<'a, T> => &'a T,
    impl<'a, T: Sync + 'a>
}

// `BTreeSet` doesn't have a mutable `Iterator`
