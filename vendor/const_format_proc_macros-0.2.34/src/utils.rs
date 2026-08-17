use proc_macro2::Span;

#[cfg(feature = "derive")]
use quote::ToTokens;

use std::{
    collections::VecDeque,
    iter::Fuse,
    mem,
    ops::{Deref, DerefMut},
};

pub(crate) fn dummy_ident() -> proc_macro2::Ident {
    proc_macro2::Ident::new("__dummy__", Span::mixed_site())
}

////////////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "derive")]
pub fn spanned_err(tokens: &dyn ToTokens, display: &dyn std::fmt::Display) -> crate::Error {
    use syn::spanned::Spanned;
    crate::Error::new(tokens.span(), display)
}

////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub struct Peekable2<I: Iterator> {
    iter: Fuse<I>,
    queue: VecDeque<I::Item>,
}

impl Peekable2<std::ops::Range<u8>> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<I: IntoIterator>(iter: I) -> Peekable2<I::IntoIter> {
        Peekable2 {
            iter: iter.into_iter().fuse(),
            queue: VecDeque::new(),
        }
    }
}

impl<I: Iterator> Peekable2<I> {
    pub fn is_empty(&mut self) -> bool {
        self.peek().is_none()
    }

    pub fn peek(&mut self) -> Option<&I::Item> {
        if self.queue.is_empty() {
            self.queue.push_back(self.iter.next()?);
        }
        Some(&self.queue[0])
    }
    pub fn peek2(&mut self) -> Option<&I::Item> {
        while self.queue.len() < 2 {
            self.queue.push_back(self.iter.next()?);
        }
        Some(&self.queue[1])
    }
}

impl<I> Iterator for Peekable2<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        if let opt @ Some(_) = self.queue.pop_front() {
            opt
        } else {
            self.iter.next()
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (low, high) = self.iter.size_hint();
        let len = self.queue.len();
        (low + len, high.map(|x| x.saturating_add(len)))
    }
}

////////////////////////////////////////////////////////////////////////////////////

/// A result wrapper which panics if it's the error variant is not handled,
/// by calling `.into_result()`.
#[derive(Debug, Clone)]
pub struct LinearResult {
    errors: Result<(), crate::Error>,
}

impl Drop for LinearResult {
    fn drop(&mut self) {
        mem::replace(&mut self.errors, Ok(())).expect("Expected LinearResult to be handled");
    }
}

impl LinearResult {
    #[inline]
    pub fn new(res: Result<(), crate::Error>) -> Self {
        Self { errors: res }
    }

    #[inline]
    pub fn ok() -> Self {
        Self::new(Ok(()))
    }
}

impl From<Result<(), crate::Error>> for LinearResult {
    #[inline]
    fn from(res: Result<(), crate::Error>) -> Self {
        Self::new(res)
    }
}

impl Deref for LinearResult {
    type Target = Result<(), crate::Error>;

    fn deref(&self) -> &Result<(), crate::Error> {
        &self.errors
    }
}

impl DerefMut for LinearResult {
    fn deref_mut(&mut self) -> &mut Result<(), crate::Error> {
        &mut self.errors
    }
}

#[allow(dead_code)]
impl LinearResult {
    #[inline]
    pub fn into_result(mut self) -> Result<(), crate::Error> {
        mem::replace(&mut self.errors, Ok(()))
    }

    #[inline]
    pub fn take(&mut self) -> Result<(), crate::Error> {
        self.replace(Ok(()))
    }

    #[inline]
    pub fn replace(&mut self, other: Result<(), crate::Error>) -> Result<(), crate::Error> {
        mem::replace(&mut self.errors, other)
    }

    #[inline]
    pub fn push_err<E>(&mut self, err: E)
    where
        E: Into<crate::Error>,
    {
        let err = err.into();
        match &mut self.errors {
            this @ Ok(_) => *this = Err(err),
            Err(e) => e.combine(err),
        }
    }

    #[inline]
    pub fn combine_err<E>(&mut self, res: Result<(), E>)
    where
        E: Into<crate::Error>,
    {
        if let Err(err) = res {
            let err = err.into();
            self.push_err(err);
        }
    }
}
