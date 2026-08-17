//! "Fallible" iterators.
//!
//! The iterator APIs in the Rust standard library do not support iteration
//! that can fail in a first class manner. These iterators are typically modeled
//! as iterating over `Result<T, E>` values; for example, the `Lines` iterator
//! returns `io::Result<String>`s. When simply iterating over these types, the
//! value being iterated over must be unwrapped in some way before it can be
//! used:
//!
//! ```ignore
//! for line in reader.lines() {
//!     let line = line?;
//!     // work with line
//! }
//! ```
//!
//! In addition, many of the additional methods on the `Iterator` trait will
//! not behave properly in the presence of errors when working with these kinds
//! of iterators. For example, if one wanted to count the number of lines of
//! text in a `Read`er, this might be a way to go about it:
//!
//! ```ignore
//! let count = reader.lines().count();
//! ```
//!
//! This will return the proper value when the reader operates successfully, but
//! if it encounters an IO error, the result will either be slightly higher than
//! expected if the error is transient, or it may run forever if the error is
//! returned repeatedly!
//!
//! In contrast, a fallible iterator is built around the concept that a call to
//! `next` can fail. The trait has an additional `Error` associated type in
//! addition to the `Item` type, and `next` returns `Result<Option<Self::Item>,
//! Self::Error>` rather than `Option<Self::Item>`. Methods like `count` return
//! `Result`s as well.
//!
//! This does mean that fallible iterators are incompatible with Rust's `for`
//! loop syntax, but `while let` loops offer a similar level of ergonomics:
//!
//! ```ignore
//! while let Some(item) = iter.next()? {
//!     // work with item
//! }
//! ```
//!
//! ## Fallible closure arguments
//!
//! Like `Iterator`, many `FallibleIterator` methods take closures as arguments.
//! These use the same signatures as their `Iterator` counterparts, except that
//! `FallibleIterator` expects the closures to be fallible: they return
//! `Result<T, Self::Error>` instead of simply `T`.
//!
//! For example, the standard library's `Iterator::filter` adapter method
//! filters the underlying iterator according to a predicate provided by the
//! user, whose return type is `bool`. In `FallibleIterator::filter`, however,
//! the predicate returns `Result<bool, Self::Error>`:
//!
//! ```
//! # use std::error::Error;
//! # use std::str::FromStr;
//! # use fallible_iterator::{convert, FallibleIterator};
//! let numbers = convert("100\n200\nfern\n400".lines().map(Ok::<&str, Box<Error>>));
//! let big_numbers = numbers.filter(|n| Ok(u64::from_str(n)? > 100));
//! assert!(big_numbers.count().is_err());
//! ```
#![doc(html_root_url = "https://docs.rs/fallible-iterator/0.2")]
#![warn(missing_docs)]
#![no_std]

use core::cmp::{self, Ordering};
use core::convert::Infallible;
use core::iter;
use core::marker::PhantomData;

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

#[cfg(all(test, feature = "alloc"))]
mod test;

enum FoldStop<T, E> {
    Break(T),
    Err(E),
}

impl<T, E> From<E> for FoldStop<T, E> {
    #[inline]
    fn from(e: E) -> FoldStop<T, E> {
        FoldStop::Err(e)
    }
}

trait ResultExt<T, E> {
    fn unpack_fold(self) -> Result<T, E>;
}

impl<T, E> ResultExt<T, E> for Result<T, FoldStop<T, E>> {
    #[inline]
    fn unpack_fold(self) -> Result<T, E> {
        match self {
            Ok(v) => Ok(v),
            Err(FoldStop::Break(v)) => Ok(v),
            Err(FoldStop::Err(e)) => Err(e),
        }
    }
}

/// An `Iterator`-like trait that allows for calculation of items to fail.
pub trait FallibleIterator {
    /// The type being iterated over.
    type Item;

    /// The error type.
    type Error;

    /// Advances the iterator and returns the next value.
    ///
    /// Returns `Ok(None)` when iteration is finished.
    ///
    /// The behavior of calling this method after a previous call has returned
    /// `Ok(None)` or `Err` is implementation defined.
    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error>;

    /// Returns bounds on the remaining length of the iterator.
    ///
    /// Specifically, the first half of the returned tuple is a lower bound and
    /// the second half is an upper bound.
    ///
    /// For the upper bound, `None` indicates that the upper bound is either
    /// unknown or larger than can be represented as a `usize`.
    ///
    /// Both bounds assume that all remaining calls to `next` succeed. That is,
    /// `next` could return an `Err` in fewer calls than specified by the lower
    /// bound.
    ///
    /// The default implementation returns `(0, None)`, which is correct for
    /// any iterator.
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }

    /// Consumes the iterator, returning the number of remaining items.
    #[inline]
    fn count(self) -> Result<usize, Self::Error>
    where
        Self: Sized,
    {
        self.fold(0, |n, _| Ok(n + 1))
    }

    /// Returns the last element of the iterator.
    #[inline]
    fn last(self) -> Result<Option<Self::Item>, Self::Error>
    where
        Self: Sized,
    {
        self.fold(None, |_, v| Ok(Some(v)))
    }

    /// Returns the `n`th element of the iterator.
    #[inline]
    fn nth(&mut self, mut n: usize) -> Result<Option<Self::Item>, Self::Error> {
        while let Some(e) = self.next()? {
            if n == 0 {
                return Ok(Some(e));
            }
            n -= 1;
        }
        Ok(None)
    }

    /// Returns an iterator starting at the same point, but stepping by the given amount at each iteration.
    ///
    /// # Panics
    ///
    /// Panics if `step` is 0.
    #[inline]
    fn step_by(self, step: usize) -> StepBy<Self>
    where
        Self: Sized,
    {
        assert!(step != 0);
        StepBy {
            it: self,
            step: step - 1,
            first_take: true,
        }
    }

    /// Returns an iterator which yields the elements of this iterator followed
    /// by another.
    #[inline]
    fn chain<I>(self, it: I) -> Chain<Self, I>
    where
        I: IntoFallibleIterator<Item = Self::Item, Error = Self::Error>,
        Self: Sized,
    {
        Chain {
            front: self,
            back: it,
            state: ChainState::Both,
        }
    }

    /// Returns an iterator that yields pairs of this iterator's and another
    /// iterator's values.
    #[inline]
    fn zip<I>(self, o: I) -> Zip<Self, I::IntoFallibleIter>
    where
        Self: Sized,
        I: IntoFallibleIterator<Error = Self::Error>,
    {
        Zip(self, o.into_fallible_iter())
    }

    /// Returns an iterator which applies a fallible transform to the elements
    /// of the underlying iterator.
    #[inline]
    fn map<F, B>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> Result<B, Self::Error>,
    {
        Map { it: self, f }
    }

    /// Calls a fallible closure on each element of an iterator.
    #[inline]
    fn for_each<F>(self, mut f: F) -> Result<(), Self::Error>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> Result<(), Self::Error>,
    {
        self.fold((), move |(), item| f(item))
    }

    /// Returns an iterator which uses a predicate to determine which values
    /// should be yielded. The predicate may fail; such failures are passed to
    /// the caller.
    #[inline]
    fn filter<F>(self, f: F) -> Filter<Self, F>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> Result<bool, Self::Error>,
    {
        Filter { it: self, f }
    }

    /// Returns an iterator which both filters and maps. The closure may fail;
    /// such failures are passed along to the consumer.
    #[inline]
    fn filter_map<B, F>(self, f: F) -> FilterMap<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> Result<Option<B>, Self::Error>,
    {
        FilterMap { it: self, f }
    }

    /// Returns an iterator which yields the current iteration count as well
    /// as the value.
    #[inline]
    fn enumerate(self) -> Enumerate<Self>
    where
        Self: Sized,
    {
        Enumerate { it: self, n: 0 }
    }

    /// Returns an iterator that can peek at the next element without consuming
    /// it.
    #[inline]
    fn peekable(self) -> Peekable<Self>
    where
        Self: Sized,
    {
        Peekable {
            it: self,
            next: None,
        }
    }

    /// Returns an iterator that skips elements based on a predicate.
    #[inline]
    fn skip_while<P>(self, predicate: P) -> SkipWhile<Self, P>
    where
        Self: Sized,
        P: FnMut(&Self::Item) -> Result<bool, Self::Error>,
    {
        SkipWhile {
            it: self,
            flag: false,
            predicate,
        }
    }

    /// Returns an iterator that yields elements based on a predicate.
    #[inline]
    fn take_while<P>(self, predicate: P) -> TakeWhile<Self, P>
    where
        Self: Sized,
        P: FnMut(&Self::Item) -> Result<bool, Self::Error>,
    {
        TakeWhile {
            it: self,
            flag: false,
            predicate,
        }
    }

    /// Returns an iterator which skips the first `n` values of this iterator.
    #[inline]
    fn skip(self, n: usize) -> Skip<Self>
    where
        Self: Sized,
    {
        Skip { it: self, n }
    }

    /// Returns an iterator that yields only the first `n` values of this
    /// iterator.
    #[inline]
    fn take(self, n: usize) -> Take<Self>
    where
        Self: Sized,
    {
        Take {
            it: self,
            remaining: n,
        }
    }

    /// Returns an iterator which applies a stateful map to values of this
    /// iterator.
    #[inline]
    fn scan<St, B, F>(self, initial_state: St, f: F) -> Scan<Self, St, F>
    where
        Self: Sized,
        F: FnMut(&mut St, Self::Item) -> Result<Option<B>, Self::Error>,
    {
        Scan {
            it: self,
            f,
            state: initial_state,
        }
    }

    /// Returns an iterator which maps this iterator's elements to iterators, yielding those iterators' values.
    #[inline]
    fn flat_map<U, F>(self, f: F) -> FlatMap<Self, U, F>
    where
        Self: Sized,
        U: IntoFallibleIterator<Error = Self::Error>,
        F: FnMut(Self::Item) -> Result<U, Self::Error>,
    {
        FlatMap {
            it: self.map(f),
            cur: None,
        }
    }

    /// Returns an iterator which flattens an iterator of iterators, yielding those iterators' values.
    #[inline]
    fn flatten(self) -> Flatten<Self>
    where
        Self: Sized,
        Self::Item: IntoFallibleIterator<Error = Self::Error>,
    {
        Flatten {
            it: self,
            cur: None,
        }
    }

    /// Returns an iterator which yields this iterator's elements and ends after
    /// the first `Ok(None)`.
    ///
    /// The behavior of calling `next` after it has previously returned
    /// `Ok(None)` is normally unspecified. The iterator returned by this method
    /// guarantees that `Ok(None)` will always be returned.
    #[inline]
    fn fuse(self) -> Fuse<Self>
    where
        Self: Sized,
    {
        Fuse {
            it: self,
            done: false,
        }
    }

    /// Returns an iterator which passes each element to a closure before returning it.
    #[inline]
    fn inspect<F>(self, f: F) -> Inspect<Self, F>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> Result<(), Self::Error>,
    {
        Inspect { it: self, f }
    }

    /// Borrow an iterator rather than consuming it.
    ///
    /// This is useful to allow the use of iterator adaptors that would
    /// otherwise consume the value.
    #[inline]
    fn by_ref(&mut self) -> &mut Self
    where
        Self: Sized,
    {
        self
    }

    /// Transforms the iterator into a collection.
    ///
    /// An `Err` will be returned if any invocation of `next` returns `Err`.
    #[inline]
    fn collect<T>(self) -> Result<T, Self::Error>
    where
        T: iter::FromIterator<Self::Item>,
        Self: Sized,
    {
        self.iterator().collect()
    }

    /// Transforms the iterator into two collections, partitioning elements by a closure.
    #[inline]
    fn partition<B, F>(self, mut f: F) -> Result<(B, B), Self::Error>
    where
        Self: Sized,
        B: Default + Extend<Self::Item>,
        F: FnMut(&Self::Item) -> Result<bool, Self::Error>,
    {
        let mut a = B::default();
        let mut b = B::default();

        self.for_each(|i| {
            if f(&i)? {
                a.extend(Some(i));
            } else {
                b.extend(Some(i));
            }
            Ok(())
        })?;

        Ok((a, b))
    }

    /// Applies a function over the elements of the iterator, producing a single
    /// final value.
    #[inline]
    fn fold<B, F>(mut self, init: B, f: F) -> Result<B, Self::Error>
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> Result<B, Self::Error>,
    {
        self.try_fold(init, f)
    }

    /// Applies a function over the elements of the iterator, producing a single final value.
    ///
    /// This is used as the "base" of many methods on `FallibleIterator`.
    #[inline]
    fn try_fold<B, E, F>(&mut self, mut init: B, mut f: F) -> Result<B, E>
    where
        Self: Sized,
        E: From<Self::Error>,
        F: FnMut(B, Self::Item) -> Result<B, E>,
    {
        while let Some(v) = self.next()? {
            init = f(init, v)?;
        }
        Ok(init)
    }

    /// Determines if all elements of this iterator match a predicate.
    #[inline]
    fn all<F>(&mut self, mut f: F) -> Result<bool, Self::Error>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> Result<bool, Self::Error>,
    {
        self.try_fold((), |(), v| {
            if !f(v)? {
                return Err(FoldStop::Break(false));
            }
            Ok(())
        })
        .map(|()| true)
        .unpack_fold()
    }

    /// Determines if any element of this iterator matches a predicate.
    #[inline]
    fn any<F>(&mut self, mut f: F) -> Result<bool, Self::Error>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> Result<bool, Self::Error>,
    {
        self.try_fold((), |(), v| {
            if f(v)? {
                return Err(FoldStop::Break(true));
            }
            Ok(())
        })
        .map(|()| false)
        .unpack_fold()
    }

    /// Returns the first element of the iterator that matches a predicate.
    #[inline]
    fn find<F>(&mut self, mut f: F) -> Result<Option<Self::Item>, Self::Error>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> Result<bool, Self::Error>,
    {
        self.try_fold((), |(), v| {
            if f(&v)? {
                return Err(FoldStop::Break(Some(v)));
            }
            Ok(())
        })
        .map(|()| None)
        .unpack_fold()
    }

    /// Applies a function to the elements of the iterator, returning the first non-`None` result.
    #[inline]
    fn find_map<B, F>(&mut self, f: F) -> Result<Option<B>, Self::Error>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> Result<Option<B>, Self::Error>,
    {
        self.filter_map(f).next()
    }

    /// Returns the position of the first element of this iterator that matches
    /// a predicate. The predicate may fail; such failures are returned to the
    /// caller.
    #[inline]
    fn position<F>(&mut self, mut f: F) -> Result<Option<usize>, Self::Error>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> Result<bool, Self::Error>,
    {
        self.try_fold(0, |n, v| {
            if f(v)? {
                return Err(FoldStop::Break(Some(n)));
            }
            Ok(n + 1)
        })
        .map(|_| None)
        .unpack_fold()
    }

    /// Returns the maximal element of the iterator.
    #[inline]
    fn max(self) -> Result<Option<Self::Item>, Self::Error>
    where
        Self: Sized,
        Self::Item: Ord,
    {
        self.max_by(|a, b| Ok(a.cmp(b)))
    }

    /// Returns the element of the iterator which gives the maximum value from
    /// the function.
    #[inline]
    fn max_by_key<B, F>(mut self, mut f: F) -> Result<Option<Self::Item>, Self::Error>
    where
        Self: Sized,
        B: Ord,
        F: FnMut(&Self::Item) -> Result<B, Self::Error>,
    {
        let max = match self.next()? {
            Some(v) => (f(&v)?, v),
            None => return Ok(None),
        };

        self.fold(max, |(key, max), v| {
            let new_key = f(&v)?;
            if key > new_key {
                Ok((key, max))
            } else {
                Ok((new_key, v))
            }
        })
        .map(|v| Some(v.1))
    }

    /// Returns the element that gives the maximum value with respect to the function.
    #[inline]
    fn max_by<F>(mut self, mut f: F) -> Result<Option<Self::Item>, Self::Error>
    where
        Self: Sized,
        F: FnMut(&Self::Item, &Self::Item) -> Result<Ordering, Self::Error>,
    {
        let max = match self.next()? {
            Some(v) => v,
            None => return Ok(None),
        };

        self.fold(max, |max, v| {
            if f(&max, &v)? == Ordering::Greater {
                Ok(max)
            } else {
                Ok(v)
            }
        })
        .map(Some)
    }

    /// Returns the minimal element of the iterator.
    #[inline]
    fn min(self) -> Result<Option<Self::Item>, Self::Error>
    where
        Self: Sized,
        Self::Item: Ord,
    {
        self.min_by(|a, b| Ok(a.cmp(b)))
    }

    /// Returns the element of the iterator which gives the minimum value from
    /// the function.
    #[inline]
    fn min_by_key<B, F>(mut self, mut f: F) -> Result<Option<Self::Item>, Self::Error>
    where
        Self: Sized,
        B: Ord,
        F: FnMut(&Self::Item) -> Result<B, Self::Error>,
    {
        let min = match self.next()? {
            Some(v) => (f(&v)?, v),
            None => return Ok(None),
        };

        self.fold(min, |(key, min), v| {
            let new_key = f(&v)?;
            if key < new_key {
                Ok((key, min))
            } else {
                Ok((new_key, v))
            }
        })
        .map(|v| Some(v.1))
    }

    /// Returns the element that gives the minimum value with respect to the function.
    #[inline]
    fn min_by<F>(mut self, mut f: F) -> Result<Option<Self::Item>, Self::Error>
    where
        Self: Sized,
        F: FnMut(&Self::Item, &Self::Item) -> Result<Ordering, Self::Error>,
    {
        let min = match self.next()? {
            Some(v) => v,
            None => return Ok(None),
        };

        self.fold(min, |min, v| {
            if f(&min, &v)? == Ordering::Less {
                Ok(min)
            } else {
                Ok(v)
            }
        })
        .map(Some)
    }

    /// Returns an iterator that yields this iterator's items in the opposite
    /// order.
    #[inline]
    fn rev(self) -> Rev<Self>
    where
        Self: Sized + DoubleEndedFallibleIterator,
    {
        Rev(self)
    }

    /// Converts an iterator of pairs into a pair of containers.
    #[inline]
    fn unzip<A, B, FromA, FromB>(self) -> Result<(FromA, FromB), Self::Error>
    where
        Self: Sized + FallibleIterator<Item = (A, B)>,
        FromA: Default + Extend<A>,
        FromB: Default + Extend<B>,
    {
        let mut from_a = FromA::default();
        let mut from_b = FromB::default();

        self.for_each(|(a, b)| {
            from_a.extend(Some(a));
            from_b.extend(Some(b));
            Ok(())
        })?;

        Ok((from_a, from_b))
    }

    /// Returns an iterator which clones all of its elements.
    #[inline]
    fn cloned<'a, T>(self) -> Cloned<Self>
    where
        Self: Sized + FallibleIterator<Item = &'a T>,
        T: 'a + Clone,
    {
        Cloned(self)
    }

    /// Returns an iterator which repeats this iterator endlessly.
    #[inline]
    fn cycle(self) -> Cycle<Self>
    where
        Self: Sized + Clone,
    {
        Cycle {
            it: self.clone(),
            cur: self,
        }
    }

    /// Lexicographically compares the elements of this iterator to that of
    /// another.
    #[inline]
    fn cmp<I>(mut self, other: I) -> Result<Ordering, Self::Error>
    where
        Self: Sized,
        I: IntoFallibleIterator<Item = Self::Item, Error = Self::Error>,
        Self::Item: Ord,
    {
        let mut other = other.into_fallible_iter();

        loop {
            match (self.next()?, other.next()?) {
                (None, None) => return Ok(Ordering::Equal),
                (None, _) => return Ok(Ordering::Less),
                (_, None) => return Ok(Ordering::Greater),
                (Some(x), Some(y)) => match x.cmp(&y) {
                    Ordering::Equal => {}
                    o => return Ok(o),
                },
            }
        }
    }

    /// Lexicographically compares the elements of this iterator to that of
    /// another.
    #[inline]
    fn partial_cmp<I>(mut self, other: I) -> Result<Option<Ordering>, Self::Error>
    where
        Self: Sized,
        I: IntoFallibleIterator<Error = Self::Error>,
        Self::Item: PartialOrd<I::Item>,
    {
        let mut other = other.into_fallible_iter();

        loop {
            match (self.next()?, other.next()?) {
                (None, None) => return Ok(Some(Ordering::Equal)),
                (None, _) => return Ok(Some(Ordering::Less)),
                (_, None) => return Ok(Some(Ordering::Greater)),
                (Some(x), Some(y)) => match x.partial_cmp(&y) {
                    Some(Ordering::Equal) => {}
                    o => return Ok(o),
                },
            }
        }
    }

    /// Determines if the elements of this iterator are equal to those of
    /// another.
    #[inline]
    fn eq<I>(mut self, other: I) -> Result<bool, Self::Error>
    where
        Self: Sized,
        I: IntoFallibleIterator<Error = Self::Error>,
        Self::Item: PartialEq<I::Item>,
    {
        let mut other = other.into_fallible_iter();

        loop {
            match (self.next()?, other.next()?) {
                (None, None) => return Ok(true),
                (None, _) | (_, None) => return Ok(false),
                (Some(x), Some(y)) => {
                    if x != y {
                        return Ok(false);
                    }
                }
            }
        }
    }

    /// Determines if the elements of this iterator are not equal to those of
    /// another.
    #[inline]
    fn ne<I>(mut self, other: I) -> Result<bool, Self::Error>
    where
        Self: Sized,
        I: IntoFallibleIterator<Error = Self::Error>,
        Self::Item: PartialEq<I::Item>,
    {
        let mut other = other.into_fallible_iter();

        loop {
            match (self.next()?, other.next()?) {
                (None, None) => return Ok(false),
                (None, _) | (_, None) => return Ok(true),
                (Some(x), Some(y)) => {
                    if x != y {
                        return Ok(true);
                    }
                }
            }
        }
    }

    /// Determines if the elements of this iterator are lexicographically less
    /// than those of another.
    #[inline]
    fn lt<I>(mut self, other: I) -> Result<bool, Self::Error>
    where
        Self: Sized,
        I: IntoFallibleIterator<Error = Self::Error>,
        Self::Item: PartialOrd<I::Item>,
    {
        let mut other = other.into_fallible_iter();

        loop {
            match (self.next()?, other.next()?) {
                (None, None) => return Ok(false),
                (None, _) => return Ok(true),
                (_, None) => return Ok(false),
                (Some(x), Some(y)) => match x.partial_cmp(&y) {
                    Some(Ordering::Less) => return Ok(true),
                    Some(Ordering::Equal) => {}
                    Some(Ordering::Greater) => return Ok(false),
                    None => return Ok(false),
                },
            }
        }
    }

    /// Determines if the elements of this iterator are lexicographically less
    /// than or equal to those of another.
    #[inline]
    fn le<I>(mut self, other: I) -> Result<bool, Self::Error>
    where
        Self: Sized,
        I: IntoFallibleIterator<Error = Self::Error>,
        Self::Item: PartialOrd<I::Item>,
    {
        let mut other = other.into_fallible_iter();

        loop {
            match (self.next()?, other.next()?) {
                (None, None) => return Ok(true),
                (None, _) => return Ok(true),
                (_, None) => return Ok(false),
                (Some(x), Some(y)) => match x.partial_cmp(&y) {
                    Some(Ordering::Less) => return Ok(true),
                    Some(Ordering::Equal) => {}
                    Some(Ordering::Greater) => return Ok(false),
                    None => return Ok(false),
                },
            }
        }
    }

    /// Determines if the elements of this iterator are lexicographically
    /// greater than those of another.
    #[inline]
    fn gt<I>(mut self, other: I) -> Result<bool, Self::Error>
    where
        Self: Sized,
        I: IntoFallibleIterator<Error = Self::Error>,
        Self::Item: PartialOrd<I::Item>,
    {
        let mut other = other.into_fallible_iter();

        loop {
            match (self.next()?, other.next()?) {
                (None, None) => return Ok(false),
                (None, _) => return Ok(false),
                (_, None) => return Ok(true),
                (Some(x), Some(y)) => match x.partial_cmp(&y) {
                    Some(Ordering::Less) => return Ok(false),
                    Some(Ordering::Equal) => {}
                    Some(Ordering::Greater) => return Ok(true),
                    None => return Ok(false),
                },
            }
        }
    }

    /// Determines if the elements of this iterator are lexicographically
    /// greater than or equal to those of another.
    #[inline]
    fn ge<I>(mut self, other: I) -> Result<bool, Self::Error>
    where
        Self: Sized,
        I: IntoFallibleIterator<Error = Self::Error>,
        Self::Item: PartialOrd<I::Item>,
    {
        let mut other = other.into_fallible_iter();

        loop {
            match (self.next()?, other.next()?) {
                (None, None) => return Ok(true),
                (None, _) => return Ok(false),
                (_, None) => return Ok(true),
                (Some(x), Some(y)) => match x.partial_cmp(&y) {
                    Some(Ordering::Less) => return Ok(false),
                    Some(Ordering::Equal) => {}
                    Some(Ordering::Greater) => return Ok(true),
                    None => return Ok(false),
                },
            }
        }
    }

    /// Returns a normal (non-fallible) iterator over `Result<Item, Error>`.
    #[inline]
    fn iterator(self) -> Iterator<Self>
    where
        Self: Sized,
    {
        Iterator(self)
    }

    /// Returns an iterator which applies a transform to the errors of the
    /// underlying iterator.
    #[inline]
    fn map_err<B, F>(self, f: F) -> MapErr<Self, F>
    where
        F: FnMut(Self::Error) -> B,
        Self: Sized,
    {
        MapErr { it: self, f }
    }

    /// Returns an iterator which unwraps all of its elements.
    #[inline]
    fn unwrap<T>(self) -> Unwrap<Self>
    where
        Self: Sized + FallibleIterator<Item = T>,
        Self::Error: core::fmt::Debug,
    {
        Unwrap(self)
    }
}

impl<I: FallibleIterator + ?Sized> FallibleIterator for &mut I {
    type Item = I::Item;
    type Error = I::Error;

    #[inline]
    fn next(&mut self) -> Result<Option<I::Item>, I::Error> {
        (**self).next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (**self).size_hint()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Result<Option<I::Item>, I::Error> {
        (**self).nth(n)
    }
}

impl<I: DoubleEndedFallibleIterator + ?Sized> DoubleEndedFallibleIterator for &mut I {
    #[inline]
    fn next_back(&mut self) -> Result<Option<I::Item>, I::Error> {
        (**self).next_back()
    }
}

#[cfg(feature = "alloc")]
impl<I: FallibleIterator + ?Sized> FallibleIterator for Box<I> {
    type Item = I::Item;
    type Error = I::Error;

    #[inline]
    fn next(&mut self) -> Result<Option<I::Item>, I::Error> {
        (**self).next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (**self).size_hint()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Result<Option<I::Item>, I::Error> {
        (**self).nth(n)
    }
}

#[cfg(feature = "alloc")]
impl<I: DoubleEndedFallibleIterator + ?Sized> DoubleEndedFallibleIterator for Box<I> {
    #[inline]
    fn next_back(&mut self) -> Result<Option<I::Item>, I::Error> {
        (**self).next_back()
    }
}

/// A fallible iterator able to yield elements from both ends.
pub trait DoubleEndedFallibleIterator: FallibleIterator {
    /// Advances the end of the iterator, returning the last value.
    fn next_back(&mut self) -> Result<Option<Self::Item>, Self::Error>;

    /// Applies a function over the elements of the iterator in reverse order, producing a single final value.
    #[inline]
    fn rfold<B, F>(mut self, init: B, f: F) -> Result<B, Self::Error>
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> Result<B, Self::Error>,
    {
        self.try_rfold(init, f)
    }

    /// Applies a function over the elements of the iterator in reverse, producing a single final value.
    ///
    /// This is used as the "base" of many methods on `DoubleEndedFallibleIterator`.
    #[inline]
    fn try_rfold<B, E, F>(&mut self, mut init: B, mut f: F) -> Result<B, E>
    where
        Self: Sized,
        E: From<Self::Error>,
        F: FnMut(B, Self::Item) -> Result<B, E>,
    {
        while let Some(v) = self.next_back()? {
            init = f(init, v)?;
        }
        Ok(init)
    }
}

/// Conversion into a `FallibleIterator`.
pub trait IntoFallibleIterator {
    /// The elements of the iterator.
    type Item;

    /// The error value of the iterator.
    type Error;

    /// The iterator.
    type IntoFallibleIter: FallibleIterator<Item = Self::Item, Error = Self::Error>;

    /// Creates a fallible iterator from a value.
    fn into_fallible_iter(self) -> Self::IntoFallibleIter;
}

impl<I> IntoFallibleIterator for I
where
    I: FallibleIterator,
{
    type Item = I::Item;
    type Error = I::Error;
    type IntoFallibleIter = I;

    #[inline]
    fn into_fallible_iter(self) -> I {
        self
    }
}

/// An iterator which applies a fallible transform to the elements of the
/// underlying iterator.
#[derive(Clone)]
pub struct Map<T, F> {
    it: T,
    f: F,
}

impl<I: core::fmt::Debug, F> core::fmt::Debug for Map<I, F> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Map").field("iter", &self.it).finish()
    }
}

impl<T, F, B> FallibleIterator for Map<T, F>
where
    T: FallibleIterator,
    F: FnMut(T::Item) -> Result<B, T::Error>,
{
    type Item = B;
    type Error = T::Error;

    #[inline]
    fn next(&mut self) -> Result<Option<B>, T::Error> {
        match self.it.next() {
            Ok(Some(v)) => Ok(Some((self.f)(v)?)),
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.it.size_hint()
    }

    #[inline]
    fn try_fold<C, E, G>(&mut self, init: C, mut f: G) -> Result<C, E>
    where
        E: From<T::Error>,
        G: FnMut(C, B) -> Result<C, E>,
    {
        let map = &mut self.f;
        self.it.try_fold(init, |b, v| f(b, map(v)?))
    }
}

impl<B, F, I> DoubleEndedFallibleIterator for Map<I, F>
where
    I: DoubleEndedFallibleIterator,
    F: FnMut(I::Item) -> Result<B, I::Error>,
{
    #[inline]
    fn next_back(&mut self) -> Result<Option<B>, I::Error> {
        match self.it.next_back() {
            Ok(Some(v)) => Ok(Some((self.f)(v)?)),
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    }

    #[inline]
    fn try_rfold<C, E, G>(&mut self, init: C, mut f: G) -> Result<C, E>
    where
        E: From<I::Error>,
        G: FnMut(C, B) -> Result<C, E>,
    {
        let map = &mut self.f;
        self.it.try_rfold(init, |acc, v| f(acc, map(v)?))
    }
}

#[derive(Clone, Debug)]
enum ChainState {
    Both,
    Front,
    Back,
}

/// An iterator which yields the elements of one iterator followed by another.
#[derive(Clone, Debug)]
pub struct Chain<T, U> {
    front: T,
    back: U,
    state: ChainState,
}

impl<T, U> FallibleIterator for Chain<T, U>
where
    T: FallibleIterator,
    U: FallibleIterator<Item = T::Item, Error = T::Error>,
{
    type Item = T::Item;
    type Error = T::Error;

    #[inline]
    fn next(&mut self) -> Result<Option<T::Item>, T::Error> {
        match self.state {
            ChainState::Both => match self.front.next()? {
                Some(e) => Ok(Some(e)),
                None => {
                    self.state = ChainState::Back;
                    self.back.next()
                }
            },
            ChainState::Front => self.front.next(),
            ChainState::Back => self.back.next(),
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let front_hint = self.front.size_hint();
        let back_hint = self.back.size_hint();

        let low = front_hint.0.saturating_add(back_hint.0);
        let high = match (front_hint.1, back_hint.1) {
            (Some(f), Some(b)) => f.checked_add(b),
            _ => None,
        };

        (low, high)
    }

    #[inline]
    fn count(self) -> Result<usize, T::Error> {
        match self.state {
            ChainState::Both => Ok(self.front.count()? + self.back.count()?),
            ChainState::Front => self.front.count(),
            ChainState::Back => self.back.count(),
        }
    }

    #[inline]
    fn try_fold<B, E, F>(&mut self, init: B, mut f: F) -> Result<B, E>
    where
        E: From<T::Error>,
        F: FnMut(B, T::Item) -> Result<B, E>,
    {
        match self.state {
            ChainState::Both => {
                let init = self.front.try_fold(init, &mut f)?;
                self.state = ChainState::Back;
                self.back.try_fold(init, f)
            }
            ChainState::Front => self.front.try_fold(init, f),
            ChainState::Back => self.back.try_fold(init, f),
        }
    }

    #[inline]
    fn find<F>(&mut self, mut f: F) -> Result<Option<T::Item>, T::Error>
    where
        F: FnMut(&T::Item) -> Result<bool, T::Error>,
    {
        match self.state {
            ChainState::Both => match self.front.find(&mut f)? {
                Some(v) => Ok(Some(v)),
                None => {
                    self.state = ChainState::Back;
                    self.back.find(f)
                }
            },
            ChainState::Front => self.front.find(f),
            ChainState::Back => self.back.find(f),
        }
    }

    #[inline]
    fn last(self) -> Result<Option<T::Item>, T::Error> {
        match self.state {
            ChainState::Both => {
                self.front.last()?;
                self.back.last()
            }
            ChainState::Front => self.front.last(),
            ChainState::Back => self.back.last(),
        }
    }
}

impl<T, U> DoubleEndedFallibleIterator for Chain<T, U>
where
    T: DoubleEndedFallibleIterator,
    U: DoubleEndedFallibleIterator<Item = T::Item, Error = T::Error>,
{
    #[inline]
    fn next_back(&mut self) -> Result<Option<T::Item>, T::Error> {
        match self.state {
            ChainState::Both => match self.back.next_back()? {
                Some(e) => Ok(Some(e)),
                None => {
                    self.state = ChainState::Front;
                    self.front.next_back()
                }
            },
            ChainState::Front => self.front.next_back(),
            ChainState::Back => self.back.next_back(),
        }
    }

    #[inline]
    fn try_rfold<B, E, F>(&mut self, init: B, mut f: F) -> Result<B, E>
    where
        E: From<T::Error>,
        F: FnMut(B, T::Item) -> Result<B, E>,
    {
        match self.state {
            ChainState::Both => {
                let init = self.back.try_rfold(init, &mut f)?;
                self.state = ChainState::Front;
                self.front.try_rfold(init, f)
            }
            ChainState::Front => self.front.try_rfold(init, f),
            ChainState::Back => self.back.try_rfold(init, f),
        }
    }
}

/// An iterator which clones the elements of the underlying iterator.
#[derive(Clone, Debug)]
pub struct Cloned<I>(I);

impl<'a, T, I> FallibleIterator for Cloned<I>
where
    I: FallibleIterator<Item = &'a T>,
    T: 'a + Clone,
{
    type Item = T;
    type Error = I::Error;

    #[inline]
    fn next(&mut self) -> Result<Option<T>, I::Error> {
        self.0.next().map(|o| o.cloned())
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }

    #[inline]
    fn try_fold<B, E, F>(&mut self, init: B, mut f: F) -> Result<B, E>
    where
        E: From<I::Error>,
        F: FnMut(B, T) -> Result<B, E>,
    {
        self.0.try_fold(init, |acc, v| f(acc, v.clone()))
    }
}

impl<'a, T, I> DoubleEndedFallibleIterator for Cloned<I>
where
    I: DoubleEndedFallibleIterator<Item = &'a T>,
    T: 'a + Clone,
{
    #[inline]
    fn next_back(&mut self) -> Result<Option<T>, I::Error> {
        self.0.next_back().map(|o| o.cloned())
    }

    #[inline]
    fn try_rfold<B, E, F>(&mut self, init: B, mut f: F) -> Result<B, E>
    where
        E: From<I::Error>,
        F: FnMut(B, T) -> Result<B, E>,
    {
        self.0.try_rfold(init, |acc, v| f(acc, v.clone()))
    }
}

/// Converts an `Iterator<Item = Result<T, E>>` into a `FallibleIterator<Item = T, Error = E>`.
#[inline]
pub fn convert<T, E, I>(it: I) -> Convert<I>
where
    I: iter::Iterator<Item = Result<T, E>>,
{
    Convert(it)
}

/// A fallible iterator that wraps a normal iterator over `Result`s.
#[derive(Clone, Debug)]
pub struct Convert<I>(I);

impl<T, E, I> FallibleIterator for Convert<I>
where
    I: iter::Iterator<Item = Result<T, E>>,
{
    type Item = T;
    type Error = E;

    #[inline]
    fn next(&mut self) -> Result<Option<T>, E> {
        match self.0.next() {
            Some(Ok(i)) => Ok(Some(i)),
            Some(Err(e)) => Err(e),
            None => Ok(None),
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }

    #[inline]
    fn try_fold<B, E2, F>(&mut self, init: B, mut f: F) -> Result<B, E2>
    where
        E2: From<E>,
        F: FnMut(B, T) -> Result<B, E2>,
    {
        self.0.try_fold(init, |acc, v| f(acc, v?))
    }
}

impl<T, E, I> DoubleEndedFallibleIterator for Convert<I>
where
    I: DoubleEndedIterator<Item = Result<T, E>>,
{
    #[inline]
    fn next_back(&mut self) -> Result<Option<T>, E> {
        match self.0.next_back() {
            Some(Ok(i)) => Ok(Some(i)),
            Some(Err(e)) => Err(e),
            None => Ok(None),
        }
    }

    #[inline]
    fn try_rfold<B, E2, F>(&mut self, init: B, mut f: F) -> Result<B, E2>
    where
        E2: From<E>,
        F: FnMut(B, T) -> Result<B, E2>,
    {
        self.0.try_rfold(init, |acc, v| f(acc, v?))
    }
}

/// A fallible iterator that wraps a normal iterator over `Result`s.
#[derive(Clone, Debug)]
pub struct IntoFallible<I>(I);

impl<T, I> FallibleIterator for IntoFallible<I>
where
    I: iter::Iterator<Item = T>,
{
    type Item = T;
    type Error = Infallible;

    #[inline]
    fn next(&mut self) -> Result<Option<T>, Self::Error> {
        Ok(self.0.next())
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }

    #[inline]
    fn try_fold<B, E2, F>(&mut self, init: B, f: F) -> Result<B, E2>
    where
        E2: From<Infallible>,
        F: FnMut(B, T) -> Result<B, E2>,
    {
        self.0.try_fold(init, f)
    }
}

impl<T, I: iter::Iterator<Item = T>> From<I> for IntoFallible<I> {
    fn from(value: I) -> Self {
        Self(value)
    }
}

impl<T, I> DoubleEndedFallibleIterator for IntoFallible<I>
where
    I: DoubleEndedIterator<Item = T>,
{
    #[inline]
    fn next_back(&mut self) -> Result<Option<T>, Infallible> {
        Ok(self.0.next_back())
    }

    #[inline]
    fn try_rfold<B, E2, F>(&mut self, init: B, f: F) -> Result<B, E2>
    where
        E2: From<Infallible>,
        F: FnMut(B, T) -> Result<B, E2>,
    {
        self.0.try_rfold(init, f)
    }
}

/// An iterator that yields the iteration count as well as the values of the
/// underlying iterator.
#[derive(Clone, Debug)]
pub struct Enumerate<I> {
    it: I,
    n: usize,
}

impl<I> FallibleIterator for Enumerate<I>
where
    I: FallibleIterator,
{
    type Item = (usize, I::Item);
    type Error = I::Error;

    #[inline]
    fn next(&mut self) -> Result<Option<(usize, I::Item)>, I::Error> {
        self.it.next().map(|o| {
            o.map(|e| {
                let i = self.n;
                self.n += 1;
                (i, e)
            })
        })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.it.size_hint()
    }

    #[inline]
    fn count(self) -> Result<usize, I::Error> {
        self.it.count()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Result<Option<(usize, I::Item)>, I::Error> {
        match self.it.nth(n)? {
            Some(v) => {
                let i = self.n + n;
                self.n = i + 1;
                Ok(Some((i, v)))
            }
            None => Ok(None),
        }
    }

    #[inline]
    fn try_fold<B, E, F>(&mut self, init: B, mut f: F) -> Result<B, E>
    where
        E: From<I::Error>,
        F: FnMut(B, (usize, I::Item)) -> Result<B, E>,
    {
        let n = &mut self.n;
        self.it.try_fold(init, |acc, v| {
            let i = *n;
            *n += 1;
            f(acc, (i, v))
        })
    }
}

/// An iterator which uses a fallible predicate to determine which values of the
/// underlying iterator should be yielded.
#[derive(Clone, Debug)]
pub struct Filter<I, F> {
    it: I,
    f: F,
}

impl<I, F> FallibleIterator for Filter<I, F>
where
    I: FallibleIterator,
    F: FnMut(&I::Item) -> Result<bool, I::Error>,
{
    type Item = I::Item;
    type Error = I::Error;

    #[inline]
    fn next(&mut self) -> Result<Option<I::Item>, I::Error> {
        let filter = &mut self.f;
        self.it
            .try_fold((), |(), v| {
                if filter(&v)? {
                    return Err(FoldStop::Break(Some(v)));
                }
                Ok(())
            })
            .map(|()| None)
            .unpack_fold()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, self.it.size_hint().1)
    }

    #[inline]
    fn try_fold<B, E, G>(&mut self, init: B, mut f: G) -> Result<B, E>
    where
        E: From<I::Error>,
        G: FnMut(B, I::Item) -> Result<B, E>,
    {
        let predicate = &mut self.f;
        self.it.try_fold(
            init,
            |acc, v| {
                if predicate(&v)? {
                    f(acc, v)
                } else {
                    Ok(acc)
                }
            },
        )
    }
}

impl<I, F> DoubleEndedFallibleIterator for Filter<I, F>
where
    I: DoubleEndedFallibleIterator,
    F: FnMut(&I::Item) -> Result<bool, I::Error>,
{
    #[inline]
    fn next_back(&mut self) -> Result<Option<I::Item>, I::Error> {
        let filter = &mut self.f;
        self.it
            .try_rfold((), |(), v| {
                if filter(&v)? {
                    return Err(FoldStop::Break(Some(v)));
                }
                Ok(())
            })
            .map(|()| None)
            .unpack_fold()
    }

    #[inline]
    fn try_rfold<B, E, G>(&mut self, init: B, mut f: G) -> Result<B, E>
    where
        E: From<I::Error>,
        G: FnMut(B, I::Item) -> Result<B, E>,
    {
        let predicate = &mut self.f;
        self.it.try_rfold(
            init,
            |acc, v| {
                if predicate(&v)? {
                    f(acc, v)
                } else {
                    Ok(acc)
                }
            },
        )
    }
}

/// An iterator which both filters and maps the values of the underlying
/// iterator.
#[derive(Clone, Debug)]
pub struct FilterMap<I, F> {
    it: I,
    f: F,
}

impl<B, I, F> FallibleIterator for FilterMap<I, F>
where
    I: FallibleIterator,
    F: FnMut(I::Item) -> Result<Option<B>, I::Error>,
{
    type Item = B;
    type Error = I::Error;

    #[inline]
    fn next(&mut self) -> Result<Option<B>, I::Error> {
        let map = &mut self.f;
        self.it
            .try_fold((), |(), v| match map(v)? {
                Some(v) => Err(FoldStop::Break(Some(v))),
                None => Ok(()),
            })
            .map(|()| None)
            .unpack_fold()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, self.it.size_hint().1)
    }

    #[inline]
    fn try_fold<C, E, G>(&mut self, init: C, mut f: G) -> Result<C, E>
    where
        E: From<I::Error>,
        G: FnMut(C, B) -> Result<C, E>,
    {
        let map = &mut self.f;
        self.it.try_fold(init, |acc, v| match map(v)? {
            Some(v) => f(acc, v),
            None => Ok(acc),
        })
    }
}

impl<B, I, F> DoubleEndedFallibleIterator for FilterMap<I, F>
where
    I: DoubleEndedFallibleIterator,
    F: FnMut(I::Item) -> Result<Option<B>, I::Error>,
{
    #[inline]
    fn next_back(&mut self) -> Result<Option<B>, I::Error> {
        let map = &mut self.f;
        self.it
            .try_rfold((), |(), v| match map(v)? {
                Some(v) => Err(FoldStop::Break(Some(v))),
                None => Ok(()),
            })
            .map(|()| None)
            .unpack_fold()
    }

    #[inline]
    fn try_rfold<C, E, G>(&mut self, init: C, mut f: G) -> Result<C, E>
    where
        E: From<I::Error>,
        G: FnMut(C, B) -> Result<C, E>,
    {
        let map = &mut self.f;
        self.it.try_rfold(init, |acc, v| match map(v)? {
            Some(v) => f(acc, v),
            None => Ok(acc),
        })
    }
}

/// An iterator which maps each element to another iterator, yielding those iterator's elements.
#[derive(Clone, Debug)]
pub struct FlatMap<I, U, F>
where
    U: IntoFallibleIterator,
{
    it: Map<I, F>,
    cur: Option<U::IntoFallibleIter>,
}

impl<I, U, F> FallibleIterator for FlatMap<I, U, F>
where
    I: FallibleIterator,
    U: IntoFallibleIterator<Error = I::Error>,
    F: FnMut(I::Item) -> Result<U, I::Error>,
{
    type Item = U::Item;
    type Error = U::Error;

    #[inline]
    fn next(&mut self) -> Result<Option<U::Item>, U::Error> {
        loop {
            if let Some(it) = &mut self.cur {
                if let Some(v) = it.next()? {
                    return Ok(Some(v));
                }
            }
            match self.it.next()? {
                Some(it) => self.cur = Some(it.into_fallible_iter()),
                None => return Ok(None),
            }
        }
    }

    #[inline]
    fn try_fold<B, E, G>(&mut self, init: B, mut f: G) -> Result<B, E>
    where
        E: From<U::Error>,
        G: FnMut(B, U::Item) -> Result<B, E>,
    {
        let mut acc = init;
        if let Some(cur) = &mut self.cur {
            acc = cur.try_fold(acc, &mut f)?;
            self.cur = None;
        }

        let cur = &mut self.cur;
        self.it.try_fold(acc, |acc, v| {
            let mut it = v.into_fallible_iter();
            match it.try_fold(acc, &mut f) {
                Ok(acc) => Ok(acc),
                Err(e) => {
                    *cur = Some(it);
                    Err(e)
                }
            }
        })
    }
}

/// An iterator which flattens an iterator of iterators, yielding those iterators' elements.
pub struct Flatten<I>
where
    I: FallibleIterator,
    I::Item: IntoFallibleIterator,
{
    it: I,
    cur: Option<<I::Item as IntoFallibleIterator>::IntoFallibleIter>,
}

impl<I> Clone for Flatten<I>
where
    I: FallibleIterator + Clone,
    I::Item: IntoFallibleIterator,
    <I::Item as IntoFallibleIterator>::IntoFallibleIter: Clone,
{
    #[inline]
    fn clone(&self) -> Flatten<I> {
        Flatten {
            it: self.it.clone(),
            cur: self.cur.clone(),
        }
    }
}

impl<I> FallibleIterator for Flatten<I>
where
    I: FallibleIterator,
    I::Item: IntoFallibleIterator<Error = I::Error>,
{
    type Item = <I::Item as IntoFallibleIterator>::Item;
    type Error = <I::Item as IntoFallibleIterator>::Error;

    #[inline]
    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        loop {
            if let Some(it) = &mut self.cur {
                if let Some(v) = it.next()? {
                    return Ok(Some(v));
                }
            }
            match self.it.next()? {
                Some(it) => self.cur = Some(it.into_fallible_iter()),
                None => return Ok(None),
            }
        }
    }

    #[inline]
    fn try_fold<B, E, G>(&mut self, init: B, mut f: G) -> Result<B, E>
    where
        E: From<Self::Error>,
        G: FnMut(B, Self::Item) -> Result<B, E>,
    {
        let mut acc = init;
        if let Some(cur) = &mut self.cur {
            acc = cur.try_fold(acc, &mut f)?;
            self.cur = None;
        }

        let cur = &mut self.cur;
        self.it.try_fold(acc, |acc, v| {
            let mut it = v.into_fallible_iter();
            match it.try_fold(acc, &mut f) {
                Ok(acc) => Ok(acc),
                Err(e) => {
                    *cur = Some(it);
                    Err(e)
                }
            }
        })
    }
}

/// Creates an iterator from a fallible function generating values.
///
/// ```
/// # use fallible_iterator::{from_fn, FallibleIterator};
/// let mut count = 0;
/// let counter = from_fn(move || {
///     // Increment our count. This is why we started at zero.
///     count += 1;
///
///     // Check to see if we've finished counting or not.
///     if count < 6 {
///         Ok(Some(count))
///     } else if count < 7 {
///         Ok(None)
///     } else {
///         Err(())
///     }
/// });
/// assert_eq!(&counter.collect::<Vec<_>>().unwrap(), &[1, 2, 3, 4, 5]);
/// ```
#[inline]
pub fn from_fn<I, E, F>(fun: F) -> FromFn<F>
where
    F: FnMut() -> Result<Option<I>, E>,
{
    FromFn { fun }
}

/// An iterator using a function to generate new values.
#[derive(Clone, Debug)]
pub struct FromFn<F> {
    fun: F,
}

impl<I, E, F> FallibleIterator for FromFn<F>
where
    F: FnMut() -> Result<Option<I>, E>,
{
    type Item = I;
    type Error = E;

    fn next(&mut self) -> Result<Option<I>, E> {
        (self.fun)()
    }
}

/// An iterator that yields `Ok(None)` forever after the underlying iterator
/// yields `Ok(None)` once.
#[derive(Clone, Debug)]
pub struct Fuse<I> {
    it: I,
    done: bool,
}

impl<I> FallibleIterator for Fuse<I>
where
    I: FallibleIterator,
{
    type Item = I::Item;
    type Error = I::Error;

    #[inline]
    fn next(&mut self) -> Result<Option<I::Item>, I::Error> {
        if self.done {
            return Ok(None);
        }

        match self.it.next()? {
            Some(i) => Ok(Some(i)),
            None => {
                self.done = true;
                Ok(None)
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.done {
            (0, Some(0))
        } else {
            self.it.size_hint()
        }
    }

    #[inline]
    fn count(self) -> Result<usize, I::Error> {
        if self.done {
            Ok(0)
        } else {
            self.it.count()
        }
    }

    #[inline]
    fn last(self) -> Result<Option<I::Item>, I::Error> {
        if self.done {
            Ok(None)
        } else {
            self.it.last()
        }
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Result<Option<I::Item>, I::Error> {
        if self.done {
            Ok(None)
        } else {
            let v = self.it.nth(n)?;
            if v.is_none() {
                self.done = true;
            }
            Ok(v)
        }
    }

    #[inline]
    fn try_fold<B, E, F>(&mut self, init: B, f: F) -> Result<B, E>
    where
        E: From<I::Error>,
        F: FnMut(B, I::Item) -> Result<B, E>,
    {
        if self.done {
            Ok(init)
        } else {
            self.it.try_fold(init, f)
        }
    }
}

/// An iterator which passes each element to a closure before returning it.
#[derive(Clone, Debug)]
pub struct Inspect<I, F> {
    it: I,
    f: F,
}

impl<I, F> FallibleIterator for Inspect<I, F>
where
    I: FallibleIterator,
    F: FnMut(&I::Item) -> Result<(), I::Error>,
{
    type Item = I::Item;
    type Error = I::Error;

    #[inline]
    fn next(&mut self) -> Result<Option<I::Item>, I::Error> {
        match self.it.next()? {
            Some(i) => {
                (self.f)(&i)?;
                Ok(Some(i))
            }
            None => Ok(None),
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.it.size_hint()
    }

    #[inline]
    fn try_fold<B, E, G>(&mut self, init: B, mut f: G) -> Result<B, E>
    where
        E: From<I::Error>,
        G: FnMut(B, I::Item) -> Result<B, E>,
    {
        let inspect = &mut self.f;
        self.it.try_fold(init, |acc, v| {
            inspect(&v)?;
            f(acc, v)
        })
    }
}

impl<I, F> DoubleEndedFallibleIterator for Inspect<I, F>
where
    I: DoubleEndedFallibleIterator,
    F: FnMut(&I::Item) -> Result<(), I::Error>,
{
    #[inline]
    fn next_back(&mut self) -> Result<Option<I::Item>, I::Error> {
        match self.it.next_back()? {
            Some(i) => {
                (self.f)(&i)?;
                Ok(Some(i))
            }
            None => Ok(None),
        }
    }

    #[inline]
    fn try_rfold<B, E, G>(&mut self, init: B, mut f: G) -> Result<B, E>
    where
        E: From<I::Error>,
        G: FnMut(B, I::Item) -> Result<B, E>,
    {
        let inspect = &mut self.f;
        self.it.try_rfold(init, |acc, v| {
            inspect(&v)?;
            f(acc, v)
        })
    }
}

/// A normal (non-fallible) iterator which wraps a fallible iterator.
#[derive(Clone, Debug)]
pub struct Iterator<I>(I);

impl<I> iter::Iterator for Iterator<I>
where
    I: FallibleIterator,
{
    type Item = Result<I::Item, I::Error>;

    #[inline]
    fn next(&mut self) -> Option<Result<I::Item, I::Error>> {
        match self.0.next() {
            Ok(Some(v)) => Some(Ok(v)),
            Ok(None) => None,
            Err(e) => Some(Err(e)),
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<I> DoubleEndedIterator for Iterator<I>
where
    I: DoubleEndedFallibleIterator,
{
    #[inline]
    fn next_back(&mut self) -> Option<Result<I::Item, I::Error>> {
        match self.0.next_back() {
            Ok(Some(v)) => Some(Ok(v)),
            Ok(None) => None,
            Err(e) => Some(Err(e)),
        }
    }
}

/// An iterator which applies a transform to the errors of the underlying
/// iterator.
#[derive(Clone, Debug)]
pub struct MapErr<I, F> {
    it: I,
    f: F,
}

impl<B, F, I> FallibleIterator for MapErr<I, F>
where
    I: FallibleIterator,
    F: FnMut(I::Error) -> B,
{
    type Item = I::Item;
    type Error = B;

    #[inline]
    fn next(&mut self) -> Result<Option<I::Item>, B> {
        self.it.next().map_err(&mut self.f)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.it.size_hint()
    }

    #[inline]
    fn count(mut self) -> Result<usize, B> {
        self.it.count().map_err(&mut self.f)
    }

    #[inline]
    fn last(mut self) -> Result<Option<I::Item>, B> {
        self.it.last().map_err(&mut self.f)
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Result<Option<I::Item>, B> {
        self.it.nth(n).map_err(&mut self.f)
    }

    #[inline]
    fn try_fold<C, E, G>(&mut self, init: C, mut f: G) -> Result<C, E>
    where
        E: From<B>,
        G: FnMut(C, I::Item) -> Result<C, E>,
    {
        self.it
            .try_fold(init, |acc, v| f(acc, v).map_err(MappedErr::Fold))
            .map_err(|e| match e {
                MappedErr::It(e) => (self.f)(e).into(),
                MappedErr::Fold(e) => e,
            })
    }
}

impl<B, F, I> DoubleEndedFallibleIterator for MapErr<I, F>
where
    I: DoubleEndedFallibleIterator,
    F: FnMut(I::Error) -> B,
{
    #[inline]
    fn next_back(&mut self) -> Result<Option<I::Item>, B> {
        self.it.next_back().map_err(&mut self.f)
    }

    #[inline]
    fn try_rfold<C, E, G>(&mut self, init: C, mut f: G) -> Result<C, E>
    where
        E: From<B>,
        G: FnMut(C, I::Item) -> Result<C, E>,
    {
        self.it
            .try_rfold(init, |acc, v| f(acc, v).map_err(MappedErr::Fold))
            .map_err(|e| match e {
                MappedErr::It(e) => (self.f)(e).into(),
                MappedErr::Fold(e) => e,
            })
    }
}

enum MappedErr<T, U> {
    It(T),
    Fold(U),
}

impl<T, U> From<T> for MappedErr<T, U> {
    #[inline]
    fn from(t: T) -> MappedErr<T, U> {
        MappedErr::It(t)
    }
}

/// An iterator which can look at the next element without consuming it.
#[derive(Clone, Debug)]
pub struct Peekable<I: FallibleIterator> {
    it: I,
    next: Option<I::Item>,
}

impl<I> Peekable<I>
where
    I: FallibleIterator,
{
    /// Returns a reference to the next value without advancing the iterator.
    #[inline]
    pub fn peek(&mut self) -> Result<Option<&I::Item>, I::Error> {
        if self.next.is_none() {
            self.next = self.it.next()?;
        }

        Ok(self.next.as_ref())
    }

    /// Consume and return the next value of this iterator if a condition is true.
    ///
    /// If func returns true for the next value of this iterator, consume and return it. Otherwise, return None.
    #[inline]
    pub fn next_if(&mut self, f: impl Fn(&I::Item) -> bool) -> Result<Option<I::Item>, I::Error> {
        match self.peek()? {
            Some(item) if f(item) => self.next(),
            _ => Ok(None),
        }
    }

    /// Consume and return the next item if it is equal to `expected`.
    #[inline]
    pub fn next_if_eq<T>(&mut self, expected: &T) -> Result<Option<I::Item>, I::Error>
    where
        T: ?Sized,
        I::Item: PartialEq<T>,
    {
        self.next_if(|found| found == expected)
    }
}

impl<I> FallibleIterator for Peekable<I>
where
    I: FallibleIterator,
{
    type Item = I::Item;
    type Error = I::Error;

    #[inline]
    fn next(&mut self) -> Result<Option<I::Item>, I::Error> {
        if let Some(next) = self.next.take() {
            return Ok(Some(next));
        }

        self.it.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let mut hint = self.it.size_hint();
        if self.next.is_some() {
            hint.0 = hint.0.saturating_add(1);
            hint.1 = hint.1.and_then(|h| h.checked_add(1));
        }
        hint
    }

    #[inline]
    fn try_fold<B, E, F>(&mut self, init: B, mut f: F) -> Result<B, E>
    where
        E: From<I::Error>,
        F: FnMut(B, I::Item) -> Result<B, E>,
    {
        let mut acc = init;
        if let Some(v) = self.next.take() {
            acc = f(acc, v)?;
        }
        self.it.try_fold(acc, f)
    }
}

/// An iterator which yields elements of the underlying iterator in reverse
/// order.
#[derive(Clone, Debug)]
pub struct Rev<I>(I);

impl<I> FallibleIterator for Rev<I>
where
    I: DoubleEndedFallibleIterator,
{
    type Item = I::Item;
    type Error = I::Error;

    #[inline]
    fn next(&mut self) -> Result<Option<I::Item>, I::Error> {
        self.0.next_back()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }

    #[inline]
    fn count(self) -> Result<usize, I::Error> {
        self.0.count()
    }

    #[inline]
    fn try_fold<B, E, F>(&mut self, init: B, f: F) -> Result<B, E>
    where
        E: From<I::Error>,
        F: FnMut(B, I::Item) -> Result<B, E>,
    {
        self.0.try_rfold(init, f)
    }
}

impl<I> DoubleEndedFallibleIterator for Rev<I>
where
    I: DoubleEndedFallibleIterator,
{
    #[inline]
    fn next_back(&mut self) -> Result<Option<I::Item>, I::Error> {
        self.0.next()
    }

    #[inline]
    fn try_rfold<B, E, F>(&mut self, init: B, f: F) -> Result<B, E>
    where
        E: From<I::Error>,
        F: FnMut(B, I::Item) -> Result<B, E>,
    {
        self.0.try_fold(init, f)
    }
}

/// An iterator which applies a stateful closure.
#[derive(Clone, Debug)]
pub struct Scan<I, St, F> {
    it: I,
    f: F,
    state: St,
}

impl<B, I, St, F> FallibleIterator for Scan<I, St, F>
where
    I: FallibleIterator,
    F: FnMut(&mut St, I::Item) -> Result<Option<B>, I::Error>,
{
    type Item = B;
    type Error = I::Error;

    #[inline]
    fn next(&mut self) -> Result<Option<B>, I::Error> {
        match self.it.next()? {
            Some(v) => (self.f)(&mut self.state, v),
            None => Ok(None),
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let hint = self.it.size_hint();
        (0, hint.1)
    }
}

/// An iterator which skips initial elements.
#[derive(Clone, Debug)]
pub struct Skip<I> {
    it: I,
    n: usize,
}

impl<I> FallibleIterator for Skip<I>
where
    I: FallibleIterator,
{
    type Item = I::Item;
    type Error = I::Error;

    #[inline]
    fn next(&mut self) -> Result<Option<I::Item>, I::Error> {
        if self.n == 0 {
            self.it.next()
        } else {
            let n = self.n;
            self.n = 0;
            self.it.nth(n)
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let hint = self.it.size_hint();

        (
            hint.0.saturating_sub(self.n),
            hint.1.map(|x| x.saturating_sub(self.n)),
        )
    }
}

/// An iterator which skips initial elements based on a predicate.
#[derive(Clone, Debug)]
pub struct SkipWhile<I, P> {
    it: I,
    flag: bool,
    predicate: P,
}

impl<I, P> FallibleIterator for SkipWhile<I, P>
where
    I: FallibleIterator,
    P: FnMut(&I::Item) -> Result<bool, I::Error>,
{
    type Item = I::Item;
    type Error = I::Error;

    #[inline]
    fn next(&mut self) -> Result<Option<I::Item>, I::Error> {
        let flag = &mut self.flag;
        let pred = &mut self.predicate;
        self.it.find(move |x| {
            if *flag || !pred(x)? {
                *flag = true;
                Ok(true)
            } else {
                Ok(false)
            }
        })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let hint = self.it.size_hint();
        if self.flag {
            hint
        } else {
            (0, hint.1)
        }
    }
}

/// An iterator which steps through the elements of the underlying iterator by a certain amount.
#[derive(Clone, Debug)]
pub struct StepBy<I> {
    it: I,
    step: usize,
    first_take: bool,
}

impl<I> FallibleIterator for StepBy<I>
where
    I: FallibleIterator,
{
    type Item = I::Item;
    type Error = I::Error;

    #[inline]
    fn next(&mut self) -> Result<Option<I::Item>, I::Error> {
        if self.first_take {
            self.first_take = false;
            self.it.next()
        } else {
            self.it.nth(self.step)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let inner_hint = self.it.size_hint();

        if self.first_take {
            let f = |n| {
                if n == 0 {
                    0
                } else {
                    1 + (n - 1) / (self.step + 1)
                }
            };
            (f(inner_hint.0), inner_hint.1.map(f))
        } else {
            let f = |n| n / (self.step + 1);
            (f(inner_hint.0), inner_hint.1.map(f))
        }
    }
}

/// An iterator which yields a limited number of elements from the underlying
/// iterator.
#[derive(Clone, Debug)]
pub struct Take<I> {
    it: I,
    remaining: usize,
}

impl<I> FallibleIterator for Take<I>
where
    I: FallibleIterator,
{
    type Item = I::Item;
    type Error = I::Error;

    #[inline]
    fn next(&mut self) -> Result<Option<I::Item>, I::Error> {
        if self.remaining == 0 {
            return Ok(None);
        }

        let next = self.it.next();
        if let Ok(Some(_)) = next {
            self.remaining -= 1;
        }
        next
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let hint = self.it.size_hint();
        (
            cmp::min(hint.0, self.remaining),
            hint.1.map(|n| cmp::min(n, self.remaining)),
        )
    }
}

/// An iterator which yields elements based on a predicate.
#[derive(Clone, Debug)]
pub struct TakeWhile<I, P> {
    it: I,
    flag: bool,
    predicate: P,
}

impl<I, P> FallibleIterator for TakeWhile<I, P>
where
    I: FallibleIterator,
    P: FnMut(&I::Item) -> Result<bool, I::Error>,
{
    type Item = I::Item;
    type Error = I::Error;

    #[inline]
    fn next(&mut self) -> Result<Option<I::Item>, I::Error> {
        if self.flag {
            Ok(None)
        } else {
            match self.it.next()? {
                Some(item) => {
                    if (self.predicate)(&item)? {
                        Ok(Some(item))
                    } else {
                        self.flag = true;
                        Ok(None)
                    }
                }
                None => Ok(None),
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.flag {
            (0, Some(0))
        } else {
            let hint = self.it.size_hint();
            (0, hint.1)
        }
    }
}

/// An iterator which cycles another endlessly.
#[derive(Clone, Debug)]
pub struct Cycle<I> {
    it: I,
    cur: I,
}

impl<I> FallibleIterator for Cycle<I>
where
    I: FallibleIterator + Clone,
{
    type Item = I::Item;
    type Error = I::Error;

    #[inline]
    fn next(&mut self) -> Result<Option<I::Item>, I::Error> {
        match self.cur.next()? {
            None => {
                self.cur = self.it.clone();
                self.cur.next()
            }
            Some(v) => Ok(Some(v)),
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (usize::max_value(), None)
    }
}

/// An iterator that yields pairs of this iterator's and another iterator's
/// values.
#[derive(Clone, Debug)]
pub struct Zip<T, U>(T, U);

impl<T, U> FallibleIterator for Zip<T, U>
where
    T: FallibleIterator,
    U: FallibleIterator<Error = T::Error>,
{
    type Item = (T::Item, U::Item);
    type Error = T::Error;

    #[inline]
    fn next(&mut self) -> Result<Option<(T::Item, U::Item)>, T::Error> {
        match (self.0.next()?, self.1.next()?) {
            (Some(a), Some(b)) => Ok(Some((a, b))),
            _ => Ok(None),
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let a = self.0.size_hint();
        let b = self.1.size_hint();

        let low = cmp::min(a.0, b.0);

        let high = match (a.1, b.1) {
            (Some(a), Some(b)) => Some(cmp::min(a, b)),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        };

        (low, high)
    }
}

/// An iterator that unwraps every element yielded by the underlying
/// FallibleIterator
#[derive(Clone, Debug)]
pub struct Unwrap<T>(T);

impl<T> iter::Iterator for Unwrap<T>
where
    T: FallibleIterator,
    T::Error: core::fmt::Debug,
{
    type Item = T::Item;

    #[inline]
    fn next(&mut self) -> Option<T::Item> {
        self.0.next().unwrap()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_, max) = self.0.size_hint();
        (0, max)
    }
}

fn _is_object_safe(_: &dyn DoubleEndedFallibleIterator<Item = (), Error = ()>) {}

/// An extnsion-trait with set of useful methods to convert [`core::iter::Iterator`]
/// into [`FallibleIterator`]
pub trait IteratorExt {
    /// Convert an iterator of `Result`s into `FallibleIterator` by transposition
    fn transpose_into_fallible<T, E>(self) -> Convert<Self>
    where
        Self: iter::Iterator<Item = Result<T, E>> + Sized;

    /// Convert an iterator of anything into `FallibleIterator` by wrapping
    /// into `Result<T, Infallible>` where `Infallible` is an error that can never actually
    /// happen.
    fn into_fallible<T>(self) -> IntoFallible<Self>
    where
        Self: iter::Iterator<Item = T> + Sized;
}

impl<I> IteratorExt for I
where
    I: iter::Iterator,
{
    /// Convert an iterator of `Result`s into `FallibleIterator` by transposition
    fn transpose_into_fallible<T, E>(self) -> Convert<Self>
    where
        Self: iter::Iterator<Item = Result<T, E>> + Sized,
    {
        Convert(self)
    }

    /// Convert an iterator of anything into `FallibleIterator` by wrapping
    /// into `Result<T, Infallible>` where `Infallible` is an error that can never actually
    /// happen.
    fn into_fallible<T>(self) -> IntoFallible<Self>
    where
        Self: iter::Iterator<Item = T> + Sized,
    {
        IntoFallible(self)
    }
}

/// An iterator that yields nothing.
#[derive(Clone, Debug)]
pub struct Empty<T, E>(PhantomData<T>, PhantomData<E>);

impl<T, E> FallibleIterator for Empty<T, E> {
    type Item = T;
    type Error = E;

    #[inline]
    fn next(&mut self) -> Result<Option<T>, E> {
        Ok(None)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(0))
    }
}

/// Creates an iterator that yields nothing.
pub fn empty<T, E>() -> Empty<T, E> {
    Empty(PhantomData, PhantomData)
}

/// An iterator that yields something exactly once.
#[derive(Clone, Debug)]
pub struct Once<T, E>(Option<T>, PhantomData<E>);

/// Creates an iterator that yields an element exactly once.
pub fn once<T, E>(value: T) -> Once<T, E> {
    Once(Some(value), PhantomData)
}

impl<T, E> FallibleIterator for Once<T, E> {
    type Item = T;
    type Error = E;

    #[inline]
    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        Ok(self.0.take())
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.0 {
            Some(_) => (1, Some(1)),
            None => (0, Some(0)),
        }
    }
}

/// An iterator that fails with a predetermined error exactly once.
#[derive(Clone, Debug)]
pub struct OnceErr<T, E>(PhantomData<T>, Option<E>);

/// Creates an iterator that fails with a predetermined error exactly once.
pub fn once_err<T, E>(value: E) -> OnceErr<T, E> {
    OnceErr(PhantomData, Some(value))
}

impl<T, E> FallibleIterator for OnceErr<T, E> {
    type Item = T;
    type Error = E;

    #[inline]
    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        match self.1.take() {
            Some(value) => Err(value),
            None => Ok(None),
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(0))
    }
}

/// An iterator that endlessly repeats a single element.
#[derive(Clone, Debug)]
pub struct Repeat<T: Clone, E>(T, PhantomData<E>);

/// Creates an iterator that endlessly repeats a single element.
pub fn repeat<T: Clone, E>(value: T) -> Repeat<T, E> {
    Repeat(value, PhantomData)
}

impl<T: Clone, E> FallibleIterator for Repeat<T, E> {
    type Item = T;
    type Error = E;

    #[inline]
    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        Ok(Some(self.0.clone()))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (usize::max_value(), None)
    }
}

/// An iterator that endlessly repeats a single error.
#[derive(Clone, Debug)]
pub struct RepeatErr<T, E: Clone>(PhantomData<T>, E);

/// Creates an iterator that endlessly repeats a single error.
pub fn repeat_err<T, E: Clone>(value: E) -> RepeatErr<T, E> {
    RepeatErr(PhantomData, value)
}

impl<T, E: Clone> FallibleIterator for RepeatErr<T, E> {
    type Item = T;
    type Error = E;

    #[inline]
    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        Err(self.1.clone())
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(0))
    }
}
