use std::borrow::Cow;
use std::collections::VecDeque;
use std::num::NonZeroUsize;

use crate::PATHS_ARE_CASE_INSENSITIVE;
use crate::encode;
use crate::token::variance::invariant::{Invariant, UnitBound, Zero};
use crate::token::variance::natural::VariantRange;
use crate::token::variance::ops::{self, Conjunction, Disjunction, Product};
use crate::token::variance::{Boundedness, TokenVariance, Variance};

use Fragment::{Nominal, Structural};

pub trait IntoNominalText<'t> {
    fn into_nominal_text(self) -> Text<'t>;
}

impl<'t> IntoNominalText<'t> for Cow<'t, str> {
    fn into_nominal_text(self) -> Text<'t> {
        Nominal(self).into()
    }
}

impl<'t> IntoNominalText<'t> for &'t str {
    fn into_nominal_text(self) -> Text<'t> {
        Nominal(self.into()).into()
    }
}

impl IntoNominalText<'static> for String {
    fn into_nominal_text(self) -> Text<'static> {
        Nominal(self.into()).into()
    }
}

pub trait IntoStructuralText<'t> {
    fn into_structural_text(self) -> Text<'t>;
}

impl<'t> IntoStructuralText<'t> for Cow<'t, str> {
    fn into_structural_text(self) -> Text<'t> {
        Structural(self).into()
    }
}

impl<'t> IntoStructuralText<'t> for &'t str {
    fn into_structural_text(self) -> Text<'t> {
        Structural(self.into()).into()
    }
}

impl IntoStructuralText<'static> for String {
    fn into_structural_text(self) -> Text<'static> {
        Structural(self.into()).into()
    }
}

// TODO: The derived `PartialEq` implementation is incomplete and does not detect contiguous like
//       fragments that are equivalent to an aggregated fragment. This works, but relies on
//       constructing `InvariantText` by consistently appending fragments from tokens.
//       `Text::from_components` is sensitive to this.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Text<'t> {
    fragments: VecDeque<Fragment<'t>>,
}

impl<'t> Text<'t> {
    pub fn new() -> Self {
        Text {
            fragments: VecDeque::new(),
        }
    }

    #[cfg(test)]
    pub(crate) fn from_components<I>(components: I) -> Option<Self>
    where
        I: IntoIterator,
        I::Item: Into<Cow<'t, str>>,
    {
        use itertools::Itertools;

        use crate::token::Separator;

        Itertools::intersperse(
            components
                .into_iter()
                .map(Into::into)
                .map(IntoNominalText::into_nominal_text),
            Separator::INVARIANT_TEXT.into_structural_text(),
        )
        .reduce(Conjunction::conjunction)
    }

    pub fn into_owned(self) -> Text<'static> {
        let Text { fragments } = self;
        Text {
            fragments: fragments.into_iter().map(Fragment::into_owned).collect(),
        }
    }

    pub fn to_string(&self) -> Cow<'t, str> {
        self.fragments
            .iter()
            .map(|fragment| fragment.as_string().clone())
            .reduce(|text, fragment| text + fragment)
            .unwrap_or(Cow::Borrowed(""))
    }

    fn repeated(self, n: NonZeroUsize) -> Self {
        let Text { fragments } = self;
        let n = (usize::from(n) - 1)
            .checked_mul(fragments.len())
            .expect("overflow determining invariant text");
        let first = fragments.clone();
        Text {
            fragments: first
                .into_iter()
                .chain(fragments.into_iter().cycle().take(n))
                .collect(),
        }
    }
}

impl<'t> Conjunction for Text<'t> {
    type Output = Self;

    fn conjunction(self, other: Self) -> Self::Output {
        let Text {
            fragments: mut left,
        } = self;
        let Text {
            fragments: mut right,
        } = other;
        let end = left.pop_back();
        let start = right.pop_front();
        let Text { fragments: middle } = match (end, start) {
            (Some(left), Some(right)) => ops::conjunction(left, right),
            (Some(middle), None) | (None, Some(middle)) => middle.into(),
            (None, None) => Text::new(),
        };
        Text {
            fragments: left.into_iter().chain(middle).chain(right).collect(),
        }
    }
}

impl<'t> Conjunction<Fragment<'t>> for Text<'t> {
    type Output = Self;

    fn conjunction(self, fragment: Fragment<'t>) -> Self::Output {
        self.conjunction(Self::from(fragment))
    }
}

impl<'t> Default for Text<'t> {
    fn default() -> Self {
        Text::new()
    }
}

impl<'t> From<Fragment<'t>> for Text<'t> {
    fn from(fragment: Fragment<'t>) -> Self {
        Text {
            fragments: [fragment].into_iter().collect(),
        }
    }
}

impl<'t> Invariant for Text<'t> {
    type Term = TokenVariance<Self>;
    type Bound = UnitBound;

    fn bound(_: Self, _: Self) -> Boundedness<Self::Bound> {
        UnitBound.into()
    }

    fn into_lower_bound(self) -> Boundedness<Self::Bound> {
        UnitBound.into()
    }
}

impl<'t> Product<usize> for Text<'t> {
    type Output = Self;

    fn product(self, rhs: usize) -> Self::Output {
        NonZeroUsize::new(rhs).map_or_else(Self::zero, |rhs| self.repeated(rhs))
    }
}

impl<'t> Product<VariantRange> for Text<'t> {
    type Output = TokenVariance<Self>;

    fn product(self, rhs: VariantRange) -> Self::Output {
        Variance::Variant(rhs.map_bounded(|_| UnitBound))
    }
}

impl<'t> Zero for Text<'t> {
    fn zero() -> Self {
        Text::new()
    }
}

impl<'t> Disjunction<Text<'t>> for UnitBound {
    type Output = Boundedness<Self>;

    fn disjunction(self, _: Text<'t>) -> Self::Output {
        self.into()
    }
}

#[derive(Clone, Debug, Eq)]
enum Fragment<'t> {
    Nominal(Cow<'t, str>),
    Structural(Cow<'t, str>),
}

impl<'t> Fragment<'t> {
    pub fn into_owned(self) -> Fragment<'static> {
        match self {
            Nominal(text) => Nominal(text.into_owned().into()),
            Structural(text) => Structural(text.into_owned().into()),
        }
    }

    pub fn as_string(&self) -> &Cow<'t, str> {
        match self {
            Nominal(text) | Structural(text) => text,
        }
    }
}

impl<'t> Conjunction for Fragment<'t> {
    type Output = Text<'t>;

    fn conjunction(self, other: Self) -> Self::Output {
        match (self, other) {
            (Nominal(left), Nominal(right)) => Text {
                fragments: [Nominal(left + right)].into_iter().collect(),
            },
            (Structural(left), Structural(right)) => Text {
                fragments: [Structural(left + right)].into_iter().collect(),
            },
            (left, right) => Text {
                fragments: [left, right].into_iter().collect(),
            },
        }
    }
}

impl<'t> PartialEq for Fragment<'t> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Nominal(left), Nominal(right)) => {
                if PATHS_ARE_CASE_INSENSITIVE {
                    // This comparison uses Unicode simple case folding. It would be better to use
                    // full case folding (and better still to use case folding appropriate for the
                    // language of the text), but this approach is used to have consistent results
                    // with the regular expression encoding of compiled globs. A more comprehensive
                    // alternative would be to use something like the `focaccia` crate. See also
                    // `CharExt::has_casing`.
                    encode::case_folded_eq(left.as_ref(), right.as_ref())
                }
                else {
                    left == right
                }
            },
            (Structural(left), Structural(right)) => left == right,
            _ => false,
        }
    }
}
