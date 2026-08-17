//! This module contains the parallel iterator types for linked lists
//! (`LinkedList<T>`). You will rarely need to interact with it directly
//! unless you have need to name one of the iterator types.

use std::collections::LinkedList;

use crate::iter::plumbing::*;
use crate::iter::*;

use crate::vec;

/// Parallel iterator over a linked list
#[derive(Debug, Clone)]
pub struct IntoIter<T> {
    inner: vec::IntoIter<T>,
}

into_par_vec! {
    LinkedList<T> => IntoIter<T>,
    impl<T: Send>
}

delegate_iterator! {
    IntoIter<T> => T,
    impl<T: Send>
}

/// Parallel iterator over an immutable reference to a linked list
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
    &'a LinkedList<T> => Iter<'a, T>,
    impl<'a, T: Sync>
}

delegate_iterator! {
    Iter<'a, T> => &'a T,
    impl<'a, T: Sync>
}

/// Parallel iterator over a mutable reference to a linked list
#[derive(Debug)]
pub struct IterMut<'a, T> {
    inner: vec::IntoIter<&'a mut T>,
}

into_par_vec! {
    &'a mut LinkedList<T> => IterMut<'a, T>,
    impl<'a, T: Send>
}

delegate_iterator! {
    IterMut<'a, T> => &'a mut T,
    impl<'a, T: Send>
}
