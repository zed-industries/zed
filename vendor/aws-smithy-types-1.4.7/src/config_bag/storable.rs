/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::config_bag::value::Value;
use crate::config_bag::ItemIter;
use std::fmt::{Debug, Formatter};
use std::iter::Rev;
use std::marker::PhantomData;
use std::slice;

/// Trait defining how types can be stored and loaded from the config bag
pub trait Store: Sized + Send + Sync + 'static {
    /// Denote the returned type when loaded from the config bag
    type ReturnedType<'a>: Send + Sync;
    /// Denote the stored type when stored into the config bag
    type StoredType: Send + Sync + Debug;

    /// Create a returned type from an iterable of items
    fn merge_iter(iter: ItemIter<'_, Self>) -> Self::ReturnedType<'_>;
}

/// Store an item in the config bag by replacing the existing value
///
/// See the [module docs](crate::config_bag) for more documentation.
#[non_exhaustive]
pub struct StoreReplace<U>(PhantomData<U>);

impl<U> Debug for StoreReplace<U> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "StoreReplace")
    }
}

/// Store an item in the config bag by effectively appending it to a list
///
/// See the [module docs](crate::config_bag) for more documentation.
#[non_exhaustive]
pub struct StoreAppend<U>(PhantomData<U>);

impl<U> Debug for StoreAppend<U> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "StoreAppend")
    }
}

/// Trait that marks the implementing types as able to be stored in the config bag
///
/// See the [module docs](crate::config_bag) for more documentation.
pub trait Storable: Send + Sync + Debug + 'static {
    /// Specify how an item is stored in the config bag, e.g. [`StoreReplace`] and [`StoreAppend`]
    type Storer: Store;
}

impl<U: Send + Sync + Debug + 'static> Store for StoreReplace<U> {
    type ReturnedType<'a> = Option<&'a U>;
    type StoredType = Value<U>;

    fn merge_iter(mut iter: ItemIter<'_, Self>) -> Self::ReturnedType<'_> {
        iter.next().and_then(|item| match item {
            Value::Set(item) => Some(item),
            Value::ExplicitlyUnset(_) => None,
        })
    }
}

impl<U: Send + Sync + Debug + 'static> Store for StoreAppend<U> {
    type ReturnedType<'a> = AppendItemIter<'a, U>;
    type StoredType = Value<Vec<U>>;

    fn merge_iter(iter: ItemIter<'_, Self>) -> Self::ReturnedType<'_> {
        AppendItemIter {
            inner: iter,
            cur: None,
        }
    }
}

/// Iterator of items returned by [`StoreAppend`]
pub struct AppendItemIter<'a, U> {
    inner: ItemIter<'a, StoreAppend<U>>,
    cur: Option<Rev<slice::Iter<'a, U>>>,
}

impl<U> Debug for AppendItemIter<'_, U> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "AppendItemIter")
    }
}

impl<'a, U: 'a> Iterator for AppendItemIter<'a, U>
where
    U: Send + Sync + Debug + 'static,
{
    type Item = &'a U;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(buf) = &mut self.cur {
            match buf.next() {
                Some(item) => return Some(item),
                None => self.cur = None,
            }
        }
        match self.inner.next() {
            None => None,
            Some(Value::Set(u)) => {
                self.cur = Some(u.iter().rev());
                self.next()
            }
            Some(Value::ExplicitlyUnset(_)) => None,
        }
    }
}
