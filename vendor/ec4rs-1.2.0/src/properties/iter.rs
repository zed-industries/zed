#![allow(unused_imports)]
use crate::properties::{Properties, RawValue};

macro_rules! impls {
    ($name:ident, $valuetype:ty) => {
        impl<'a> Iterator for $name<'a> {
            type Item = (&'a str, $valuetype);
            #[cfg(not(feature = "allow-empty-values"))]
            fn next(&mut self) -> Option<Self::Item> {
                loop {
                    let pair = self.0.next()?;
                    if pair.1.is_unset() {
                        continue;
                    }
                    let (ref key, val) = pair;
                    break Some((key, val));
                }
            }
            #[cfg(feature = "allow-empty-values")]
            fn next(&mut self) -> Option<Self::Item> {
                let pair = self.0.next()?;
                let (ref key, val) = pair;
                Some((key, val))
            }

            #[cfg(not(feature = "allow-empty-values"))]
            fn size_hint(&self) -> (usize, Option<usize>) {
                (0, self.0.size_hint().1)
            }
            #[cfg(feature = "allow-empty-values")]
            fn size_hint(&self) -> (usize, Option<usize>) {
                self.0.size_hint()
            }
        }
        impl<'a> DoubleEndedIterator for $name<'a> {
            #[cfg(not(feature = "allow-empty-values"))]
            fn next_back(&mut self) -> Option<Self::Item> {
                loop {
                    let pair = self.0.next_back()?;
                    if pair.1.is_unset() {
                        continue;
                    }
                    let (ref key, val) = pair;
                    break Some((key, val));
                }
            }
            #[cfg(feature = "allow-empty-values")]
            fn next_back(&mut self) -> Option<Self::Item> {
                let pair = self.0.next_back()?;
                let (ref key, val) = pair;
                Some((key, val))
            }
        }
        impl<'a> std::iter::FusedIterator for $name<'a> {}
        //TODO: PartialEq/Eq?
    };
}

/// An iterator over [`Properties`].
#[derive(Clone)]
pub struct Iter<'a>(pub(super) std::slice::Iter<'a, (String, RawValue)>);

impls! {Iter, &'a RawValue}

/// An iterator over [`Properties`] that allows value mutation.
pub struct IterMut<'a>(pub(super) std::slice::IterMut<'a, (String, RawValue)>);

impls! {IterMut, &'a mut RawValue}
