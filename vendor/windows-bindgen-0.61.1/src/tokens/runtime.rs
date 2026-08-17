use super::{Delimiter, ToTokens, TokenStream};
use core::ops::BitOr;

pub struct HasIterator; // True
pub struct ThereIsNoIteratorInRepetition; // False

impl BitOr<ThereIsNoIteratorInRepetition> for ThereIsNoIteratorInRepetition {
    type Output = ThereIsNoIteratorInRepetition;
    fn bitor(self, _rhs: ThereIsNoIteratorInRepetition) -> ThereIsNoIteratorInRepetition {
        ThereIsNoIteratorInRepetition
    }
}

impl BitOr<ThereIsNoIteratorInRepetition> for HasIterator {
    type Output = HasIterator;
    fn bitor(self, _rhs: ThereIsNoIteratorInRepetition) -> HasIterator {
        HasIterator
    }
}

impl BitOr<HasIterator> for ThereIsNoIteratorInRepetition {
    type Output = HasIterator;
    fn bitor(self, _rhs: HasIterator) -> HasIterator {
        HasIterator
    }
}

impl BitOr<HasIterator> for HasIterator {
    type Output = HasIterator;
    fn bitor(self, _rhs: HasIterator) -> HasIterator {
        HasIterator
    }
}

/// Extension traits used by the implementation of `quote!`. These are defined
/// in separate traits, rather than as a single trait due to ambiguity issues.
///
/// These traits expose a `quote_into_iter` method which should allow calling
/// whichever impl happens to be applicable. Calling that method repeatedly on
/// the returned value should be idempotent.
pub mod ext {
    use super::RepInterp;
    use super::ToTokens;
    use super::{HasIterator as HasIter, ThereIsNoIteratorInRepetition as DoesNotHaveIter};
    use core::slice;
    use std::collections::btree_set::{self, BTreeSet};

    /// Extension trait providing the `quote_into_iter` method on iterators.
    pub trait RepIteratorExt: Iterator + Sized {
        fn quote_into_iter(self) -> (Self, HasIter) {
            (self, HasIter)
        }
    }

    impl<T: Iterator> RepIteratorExt for T {}

    /// Extension trait providing the `quote_into_iter` method for
    /// non-iterable types. These types interpolate the same value in each
    /// iteration of the repetition.
    pub trait RepToTokensExt {
        /// Pretend to be an iterator for the purposes of `quote_into_iter`.
        /// This allows repeated calls to `quote_into_iter` to continue
        /// correctly returning DoesNotHaveIter.
        fn next(&self) -> Option<&Self> {
            Some(self)
        }

        fn quote_into_iter(&self) -> (&Self, DoesNotHaveIter) {
            (self, DoesNotHaveIter)
        }
    }

    impl<T: ToTokens + ?Sized> RepToTokensExt for T {}

    /// Extension trait providing the `quote_into_iter` method for types that
    /// can be referenced as an iterator.
    pub trait RepAsIteratorExt<'q> {
        type Iter: Iterator;

        fn quote_into_iter(&'q self) -> (Self::Iter, HasIter);
    }

    impl<'q, T: RepAsIteratorExt<'q> + ?Sized> RepAsIteratorExt<'q> for &T {
        type Iter = T::Iter;

        fn quote_into_iter(&'q self) -> (Self::Iter, HasIter) {
            <T as RepAsIteratorExt>::quote_into_iter(*self)
        }
    }

    impl<'q, T: RepAsIteratorExt<'q> + ?Sized> RepAsIteratorExt<'q> for &mut T {
        type Iter = T::Iter;

        fn quote_into_iter(&'q self) -> (Self::Iter, HasIter) {
            <T as RepAsIteratorExt>::quote_into_iter(*self)
        }
    }

    impl<'q, T: 'q> RepAsIteratorExt<'q> for [T] {
        type Iter = slice::Iter<'q, T>;

        fn quote_into_iter(&'q self) -> (Self::Iter, HasIter) {
            (self.iter(), HasIter)
        }
    }

    impl<'q, T: 'q> RepAsIteratorExt<'q> for Vec<T> {
        type Iter = slice::Iter<'q, T>;

        fn quote_into_iter(&'q self) -> (Self::Iter, HasIter) {
            (self.iter(), HasIter)
        }
    }

    impl<'q, T: 'q> RepAsIteratorExt<'q> for BTreeSet<T> {
        type Iter = btree_set::Iter<'q, T>;

        fn quote_into_iter(&'q self) -> (Self::Iter, HasIter) {
            (self.iter(), HasIter)
        }
    }

    macro_rules! array_rep_slice {
        ($($l:tt)*) => {
            $(
                impl<'q, T: 'q> RepAsIteratorExt<'q> for [T; $l] {
                    type Iter = slice::Iter<'q, T>;

                    fn quote_into_iter(&'q self) -> (Self::Iter, HasIter) {
                        (self.iter(), HasIter)
                    }
                }
            )*
        }
    }

    array_rep_slice!(
        0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16
        17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32
    );

    impl<'q, T: RepAsIteratorExt<'q>> RepAsIteratorExt<'q> for RepInterp<T> {
        type Iter = T::Iter;

        fn quote_into_iter(&'q self) -> (Self::Iter, HasIter) {
            self.0.quote_into_iter()
        }
    }
}

// Helper type used within interpolations to allow for repeated binding names.
// Implements the relevant traits, and exports a dummy `next()` method.
#[derive(Copy, Clone)]
pub struct RepInterp<T>(pub T);

impl<T> RepInterp<T> {
    // This method is intended to look like `Iterator::next`, and is called when
    // a name is bound multiple times, as the previous binding will shadow the
    // original `Iterator` object. This allows us to avoid advancing the
    // iterator multiple times per iteration.
    pub fn next(self) -> Option<T> {
        Some(self.0)
    }
}

impl<T: Iterator> Iterator for RepInterp<T> {
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<T: ToTokens> ToTokens for RepInterp<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens);
    }
}

pub fn push_group(tokens: &mut TokenStream, delimiter: Delimiter, inner: TokenStream) {
    tokens.push_space();
    tokens.push(delimiter.open());
    tokens.combine(&inner);
    tokens.push_space();
    tokens.push(delimiter.close());
}

pub fn parse(tokens: &mut TokenStream, s: &str) {
    tokens.push_space();
    tokens.push_str(s);
}

pub fn push_ident(tokens: &mut TokenStream, s: &str) {
    match tokens.0.chars().last() {
        None | Some(':') => {}
        _ => tokens.0.push(' '),
    }
    tokens.push_str(s);
}

pub fn push_colon2(tokens: &mut TokenStream) {
    match tokens.0.chars().last() {
        Some(':') => tokens.push_str(" ::"),
        _ => tokens.push_str("::"),
    }
}

macro_rules! push_punct {
    ($name:ident $char1:tt) => {
        pub fn $name(tokens: &mut TokenStream) {
            tokens.push_space();
            tokens.push($char1);
        }
    };
    ($name:ident $char1:tt $char2:tt) => {
        pub fn $name(tokens: &mut TokenStream) {
            tokens.push_space();
            tokens.push($char1);
            tokens.push($char2);
        }
    };
    ($name:ident $char1:tt $char2:tt $char3:tt) => {
        pub fn $name(tokens: &mut TokenStream) {
            tokens.push(' ');
            tokens.push($char1);
            tokens.push($char2);
            tokens.push($char3);
        }
    };
}

push_punct!(push_add '+');
push_punct!(push_add_eq '+' '=');
push_punct!(push_and '&');
push_punct!(push_and_and '&' '&');
push_punct!(push_and_eq '&' '=');
push_punct!(push_at '@');
push_punct!(push_bang '!');
push_punct!(push_caret '^');
push_punct!(push_caret_eq '^' '=');
push_punct!(push_colon ':');
push_punct!(push_comma ',');
push_punct!(push_div '/');
push_punct!(push_div_eq '/' '=');
push_punct!(push_dot '.');
push_punct!(push_dot2 '.' '.');
push_punct!(push_dot3 '.' '.' '.');
push_punct!(push_dot_dot_eq '.' '.' '=');
push_punct!(push_eq '=');
push_punct!(push_eq_eq '=' '=');
push_punct!(push_ge '>' '=');
push_punct!(push_gt '>');
push_punct!(push_le '<' '=');
push_punct!(push_lt '<');
push_punct!(push_mul_eq '*' '=');
push_punct!(push_ne '!' '=');
push_punct!(push_or '|');
push_punct!(push_or_eq '|' '=');
push_punct!(push_or_or '|' '|');
push_punct!(push_pound '#');
push_punct!(push_question '?');
push_punct!(push_rarrow '-' '>');
push_punct!(push_larrow '<' '-');
push_punct!(push_rem '%');
push_punct!(push_rem_eq '%' '=');
push_punct!(push_fat_arrow '=' '>');
push_punct!(push_semi ';');
push_punct!(push_shl '<' '<');
push_punct!(push_shl_eq '<' '<' '=');
push_punct!(push_shr '>' '>');
push_punct!(push_shr_eq '>' '>' '=');
push_punct!(push_star '*');
push_punct!(push_sub '-');
push_punct!(push_sub_eq '-' '=');
