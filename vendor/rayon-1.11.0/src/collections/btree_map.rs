//! This module contains the parallel iterator types for B-Tree maps
//! (`BTreeMap<K, V>`). You will rarely need to interact with it directly
//! unless you have need to name one of the iterator types.

use std::collections::BTreeMap;

use crate::iter::plumbing::*;
use crate::iter::*;

use crate::vec;

/// Parallel iterator over a B-Tree map
#[derive(Debug)] // std doesn't Clone
pub struct IntoIter<K, V> {
    inner: vec::IntoIter<(K, V)>,
}

into_par_vec! {
    BTreeMap<K, V> => IntoIter<K, V>,
    impl<K: Send, V: Send>
}

delegate_iterator! {
    IntoIter<K, V> => (K, V),
    impl<K: Send, V: Send>
}

/// Parallel iterator over an immutable reference to a B-Tree map
#[derive(Debug)]
pub struct Iter<'a, K, V> {
    inner: vec::IntoIter<(&'a K, &'a V)>,
}

impl<K, V> Clone for Iter<'_, K, V> {
    fn clone(&self) -> Self {
        Iter {
            inner: self.inner.clone(),
        }
    }
}

into_par_vec! {
    &'a BTreeMap<K, V> => Iter<'a, K, V>,
    impl<'a, K: Sync, V: Sync>
}

delegate_iterator! {
    Iter<'a, K, V> => (&'a K, &'a V),
    impl<'a, K: Sync + 'a, V: Sync + 'a>
}

/// Parallel iterator over a mutable reference to a B-Tree map
#[derive(Debug)]
pub struct IterMut<'a, K, V> {
    inner: vec::IntoIter<(&'a K, &'a mut V)>,
}

into_par_vec! {
    &'a mut BTreeMap<K, V> => IterMut<'a, K, V>,
    impl<'a, K: Sync, V: Send>
}

delegate_iterator! {
    IterMut<'a, K, V> => (&'a K, &'a mut V),
    impl<'a, K: Sync + 'a, V: Send + 'a>
}
