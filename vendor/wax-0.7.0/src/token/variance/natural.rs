use std::cmp::{self, Ordering};
use std::marker::PhantomData;
use std::num::NonZeroUsize;

use crate::token::variance::invariant::{self, UnitBound, Zero as _};
use crate::token::variance::ops::{self, BinaryOperand, Conjunction, Disjunction, Product};
use crate::token::variance::{Bounded, Boundedness, Unbounded, Variance};

macro_rules! impl_natural_invariant {
    ($name:ident => $term:ident $(,)?) => {
        const _: () = {
            use std::num::NonZeroUsize;

            use $crate::token::variance::invariant::{Invariant, One, Zero};
            use $crate::token::variance::natural::{BoundedVariantRange, VariantRange};
            use $crate::token::variance::ops::{self, Conjunction, Disjunction, Product};
            use $crate::token::variance::{
                Bounded, Boundedness, TokenVariance, Unbounded, Variance,
            };

            impl $name {
                pub const ZERO: Self = $name(0);
                pub const ONE: Self = $name(1);

                pub const fn new(n: usize) -> Self {
                    $name(n)
                }

                pub fn map<F>(self, f: F) -> Self
                where
                    F: FnOnce(usize) -> usize,
                {
                    $name(f(self.0))
                }

                pub fn zip_map<F>(self, rhs: Self, f: F) -> Self
                where
                    F: FnOnce(usize, usize) -> usize,
                {
                    $name(f(self.0, rhs.0))
                }
            }

            impl AsRef<usize> for $name {
                fn as_ref(&self) -> &usize {
                    &self.0
                }
            }

            impl Conjunction for $name {
                type Output = Self;

                fn conjunction(self, rhs: Self) -> Self::Output {
                    self.zip_map(rhs, ops::conjunction)
                }
            }

            impl From<usize> for $name {
                fn from(n: usize) -> $name {
                    $name(n)
                }
            }

            impl From<$name> for usize {
                fn from(size: $name) -> usize {
                    size.0
                }
            }

            impl Invariant for $name {
                type Term = $term<$name>;
                type Bound = BoundedVariantRange;

                fn bound(lhs: Self, rhs: Self) -> Boundedness<Self::Bound> {
                    let [lower, upper] = crate::minmax(lhs, rhs);
                    BoundedVariantRange::try_from_lower_and_upper(lower.0, upper.0)
                        .map_or(Unbounded, Bounded)
                }

                fn into_lower_bound(self) -> Boundedness<Self::Bound> {
                    NonZeroUsize::new(self.0)
                        .map(BoundedVariantRange::Lower)
                        .map(From::from)
                        .unwrap_or(Unbounded)
                }
            }

            impl One for $name {
                fn one() -> Self {
                    $name(1)
                }
            }

            impl PartialEq<usize> for $name {
                fn eq(&self, rhs: &usize) -> bool {
                    self.0 == *rhs
                }
            }

            impl Product<usize> for $name {
                type Output = Self;

                fn product(self, rhs: usize) -> Self::Output {
                    self.map(|lhs| ops::product(lhs, rhs))
                }
            }

            impl Product<VariantRange> for $name {
                type Output = TokenVariance<Self>;

                fn product(self, rhs: VariantRange) -> Self::Output {
                    NonZeroUsize::new(self.0).map_or_else(TokenVariance::<Self>::zero, |lhs| {
                        Variance::Variant(rhs.map_bounded(|rhs| ops::product(rhs, lhs)))
                    })
                }
            }

            impl Zero for $name {
                fn zero() -> Self {
                    $name(0)
                }
            }

            impl Conjunction<$name> for BoundedVariantRange {
                type Output = Self;

                fn conjunction(self, rhs: $name) -> Self::Output {
                    self.translation(rhs.0)
                }
            }

            impl Disjunction<$name> for BoundedVariantRange {
                type Output = VariantRange;

                fn disjunction(self, rhs: $name) -> Self::Output {
                    self.union(rhs.0)
                }
            }
        };
    };
}
pub(crate) use impl_natural_invariant;

macro_rules! define_natural_invariant {
    ($(#[$attr:meta])* $name:ident $(,)?) => {
        define_natural_invariant!($(#[$attr])* $name => TokenVariance);
    };
    ($(#[$attr:meta])* $name:ident => $term:ident $(,)?) => {
        $(#[$attr])*
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[repr(transparent)]
        pub struct $name(usize);

        $crate::token::variance::natural::impl_natural_invariant!($name => $term);
    };
}
pub(crate) use define_natural_invariant;

pub trait Cobound: Sized {
    type Bound;

    fn cobound(&self) -> Boundedness<Self::Bound>;

    fn into_lower(self) -> Lower<Self> {
        Lower::new(self)
    }

    fn into_upper(self) -> Upper<Self> {
        Upper::new(self)
    }
}

impl Cobound for NaturalBound {
    type Bound = usize;

    fn cobound(&self) -> Boundedness<Self::Bound> {
        use Variance::{Invariant, Variant};

        match self {
            Variant(Unbounded) => Unbounded,
            Variant(Bounded(bound)) => Bounded(bound.get()),
            Invariant(Zero) => Bounded(0),
        }
    }
}

impl Cobound for NonZeroBound {
    type Bound = NonZeroUsize;

    fn cobound(&self) -> Boundedness<Self::Bound> {
        *self
    }
}

pub trait OpenedUpperBound: Sized {
    fn opened_upper_bound(self) -> Boundedness<Self>;
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LowerUpper<L, U> {
    pub lower: L,
    pub upper: U,
}

pub type LowerUpperBound<B> = LowerUpper<Lower<B>, Upper<B>>;
pub type LowerUpperOperand<B> = LowerUpper<BinaryOperand<Lower<B>>, BinaryOperand<Upper<B>>>;

pub type NaturalBound = Variance<Zero, NonZeroBound>;
pub type NaturalRange = Variance<usize, VariantRange>;

impl Conjunction for NaturalBound {
    type Output = Self;

    fn conjunction(self, rhs: Self) -> Self::Output {
        use Variance::{Invariant, Variant};

        let lhs = self;
        match (lhs, rhs) {
            (Variant(Unbounded), _) | (_, Variant(Unbounded)) => Variant(Unbounded),
            (Invariant(Zero), nonzero) | (nonzero, Invariant(Zero)) => nonzero,
            (Variant(Bounded(lhs)), Variant(Bounded(rhs))) => Variant(Bounded(
                lhs.checked_add(rhs.into())
                    .expect("overflow determining conjunction of natural bound"),
            )),
        }
    }
}

impl From<NonZeroBound> for NaturalBound {
    fn from(bound: NonZeroBound) -> Self {
        Variance::Variant(bound)
    }
}

impl From<usize> for NaturalBound {
    fn from(n: usize) -> Self {
        NonZeroUsize::new(n).map_or_else(NaturalBound::zero, |n| Bounded(n).into())
    }
}

impl Product for NaturalBound {
    type Output = Self;

    fn product(self, rhs: Self) -> Self::Output {
        use Variance::{Invariant, Variant};

        let lhs = self;
        match (lhs, rhs) {
            (Variant(Unbounded), _) | (_, Variant(Unbounded)) => Variant(Unbounded),
            (Invariant(Zero), _) | (_, Invariant(Zero)) => Invariant(Zero),
            (Variant(Bounded(lhs)), Variant(Bounded(rhs))) => Variant(Bounded(
                lhs.checked_mul(rhs)
                    .expect("overflow determining product of natural bound"),
            )),
        }
    }
}

impl NaturalRange {
    // NOTE: There is a somewhat inconsistent behavior between the closed and open bound parameters
    //       here: the closed bound is considered unbounded when zero, but the open bound is only
    //       considered unbounded when `None`.
    // NOTE: Given the naturals X and Y where X < Y, this defines an unconventional meaning for the
    //       range [Y,X] and therefore repetitions like `<_:10,1>`: the bounds are reordered, so
    //       `<_:10,1>` and `<_:1,10>` are the same.
    pub fn from_closed_and_open<T>(closed: usize, open: T) -> Self
    where
        T: Into<Option<usize>>,
    {
        let open = open.into();
        let (lower, upper) = match open {
            Some(open) => match Ord::cmp(&closed, &open) {
                Ordering::Greater => (open, Some(closed)),
                _ => (closed, Some(open)),
            },
            _ => (closed, open),
        };
        match (lower, upper) {
            (0, None) => Variance::Variant(Unbounded),
            (lower, upper) => BoundedVariantRange::try_from_lower_and_upper(lower, upper)
                .map(Bounded)
                .map_or_else(|| Variance::Invariant(lower), Variance::Variant),
        }
    }

    fn by_bound_with<F>(self, rhs: Self, mut f: F) -> Self
    where
        F: FnMut(NaturalBound, NaturalBound) -> NaturalBound,
    {
        let lhs = self;
        let lower = f(lhs.lower().into_bound(), rhs.lower().into_bound()).into_lower();
        let upper = f(lhs.upper().into_bound(), rhs.upper().into_bound()).into_upper();
        Self::from_closed_and_open(lower.into_usize(), upper.into_usize())
    }

    fn by_lower_and_upper_with<F>(self, rhs: Self, mut f: F) -> Self
    where
        F: FnMut(LowerUpperOperand<NaturalBound>) -> LowerUpperBound<NaturalBound>,
    {
        let lhs = self;
        let LowerUpper { lower, upper } = f(LowerUpper {
            lower: BinaryOperand {
                lhs: lhs.lower(),
                rhs: rhs.lower(),
            },
            upper: BinaryOperand {
                lhs: lhs.upper(),
                rhs: rhs.upper(),
            },
        });
        Self::from_closed_and_open(lower.into_usize(), upper.into_usize())
    }

    pub fn lower(&self) -> NaturalLower {
        match self {
            Variance::Invariant(n) => NaturalBound::from(*n).into_lower(),
            Variance::Variant(range) => range.lower().map(Variance::Variant),
        }
    }

    pub fn upper(&self) -> NaturalUpper {
        match self {
            Variance::Invariant(n) => NaturalBound::from(*n).into_upper(),
            Variance::Variant(range) => range.upper().map(Variance::Variant),
        }
    }

    pub fn is_one(&self) -> bool {
        self.as_ref().invariant().is_some_and(|&n| n == 1)
    }

    pub fn is_zero(&self) -> bool {
        self.as_ref().invariant().is_some_and(|&n| n == 0)
    }
}

impl From<BoundedVariantRange> for NaturalRange {
    fn from(range: BoundedVariantRange) -> Self {
        Variance::Variant(range.into())
    }
}

impl From<usize> for NaturalRange {
    fn from(n: usize) -> Self {
        Variance::Invariant(n)
    }
}

impl From<VariantRange> for NaturalRange {
    fn from(range: VariantRange) -> Self {
        Variance::Variant(range)
    }
}

impl<T> From<(usize, T)> for NaturalRange
where
    T: Into<Option<usize>>,
{
    fn from((closed, open): (usize, T)) -> Self {
        Self::from_closed_and_open(closed, open)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Zero;

impl Zero {
    const USIZE: usize = 0;
}

impl From<Zero> for usize {
    fn from(_: Zero) -> Self {
        0
    }
}

impl invariant::Zero for Zero {
    fn zero() -> Self {
        Zero
    }
}

pub type NonZeroBound = Boundedness<NonZeroUsize>;
pub type VariantRange = Boundedness<BoundedVariantRange>;

impl Boundedness<UnitBound> {
    pub const BOUNDED: Self = Bounded(UnitBound);
}

impl<T> Boundedness<T> {
    pub fn map_bounded<U, F>(self, f: F) -> Boundedness<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Bounded(bound) => Bounded(f(bound)),
            _ => Unbounded,
        }
    }

    pub fn bounded(self) -> Option<T> {
        match self {
            Bounded(bound) => Some(bound),
            _ => None,
        }
    }

    pub fn as_ref(&self) -> Boundedness<&T> {
        match self {
            Bounded(bound) => Bounded(bound),
            _ => Unbounded,
        }
    }

    pub fn is_bounded(&self) -> bool {
        matches!(self, Bounded(_))
    }

    pub fn is_unbounded(&self) -> bool {
        matches!(self, Unbounded)
    }
}

impl<T> From<T> for Boundedness<T> {
    fn from(bound: T) -> Boundedness<T> {
        Bounded(bound)
    }
}

impl From<Option<usize>> for NonZeroBound {
    fn from(bound: Option<usize>) -> Self {
        bound.unwrap_or(0).into()
    }
}

impl From<usize> for NonZeroBound {
    fn from(bound: usize) -> Self {
        NonZeroUsize::try_from(bound).map_or(Unbounded, Bounded)
    }
}

impl VariantRange {
    pub fn lower(&self) -> NonZeroLower {
        self.as_ref()
            .bounded()
            .map_or_else(|| Unbounded.into_lower(), BoundedVariantRange::lower)
    }

    pub fn upper(&self) -> NonZeroUpper {
        self.as_ref()
            .bounded()
            .map_or_else(|| Unbounded.into_upper(), BoundedVariantRange::upper)
    }
}

impl Product<NonZeroUsize> for VariantRange {
    type Output = Self;

    fn product(self, rhs: NonZeroUsize) -> Self::Output {
        self.map_bounded(|range| ops::product(range, rhs))
    }
}

mod kind {
    #[derive(Debug)]
    pub enum LowerKind {}
    #[derive(Debug)]
    pub enum UpperKind {}
}
use kind::*;

#[derive(Debug)]
pub struct OrderedBound<K, B> {
    bound: B,
    _phantom: PhantomData<fn() -> K>,
}

impl<K, B> OrderedBound<K, B> {
    fn new(bound: B) -> Self {
        OrderedBound {
            bound,
            _phantom: PhantomData,
        }
    }

    pub fn into_bound(self) -> B {
        self.bound
    }

    pub fn map<U, F>(self, f: F) -> OrderedBound<K, U>
    where
        F: FnOnce(B) -> U,
    {
        OrderedBound::new(f(self.bound))
    }
}

impl<K, B> AsRef<B> for OrderedBound<K, B> {
    fn as_ref(&self) -> &B {
        &self.bound
    }
}

impl<K, B> Clone for OrderedBound<K, B>
where
    B: Clone,
{
    fn clone(&self) -> Self {
        Self::new(self.bound.clone())
    }
}

impl<K, B> Copy for OrderedBound<K, B> where B: Copy {}

impl<K, B> Eq for OrderedBound<K, B>
where
    B: Cobound,
    B::Bound: Eq,
{
}

impl<K, B> PartialEq for OrderedBound<K, B>
where
    B: Cobound,
    B::Bound: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.bound.cobound().eq(&other.bound.cobound())
    }
}

pub type Lower<B> = OrderedBound<LowerKind, B>;
pub type Upper<B> = OrderedBound<UpperKind, B>;

pub type NonZeroLower = Lower<NonZeroBound>;
pub type NonZeroUpper = Upper<NonZeroBound>;

pub type NaturalLower = Lower<NaturalBound>;
pub type NaturalUpper = Upper<NaturalBound>;

impl<B> Ord for Lower<B>
where
    B: Cobound,
    B::Bound: Ord,
{
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.partial_cmp(rhs).unwrap()
    }
}

impl<B> PartialOrd for Lower<B>
where
    B: Cobound,
    B::Bound: PartialOrd,
{
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        use Ordering::{Equal, Greater, Less};

        match (&self.bound.cobound(), &rhs.bound.cobound()) {
            (Unbounded, Bounded(_)) => Some(Less),
            (Bounded(_), Unbounded) => Some(Greater),
            (Bounded(lhs), Bounded(rhs)) => lhs.partial_cmp(rhs),
            _ => Some(Equal),
        }
    }
}

impl NaturalLower {
    pub fn into_usize(self) -> usize {
        match self.into_bound() {
            Variance::Invariant(Zero) => Zero::USIZE,
            Variance::Variant(bound) => bound.into_lower().into_usize(),
        }
    }
}

impl NonZeroLower {
    pub fn into_usize(self) -> usize {
        self.into_bound().bounded().map_or(0, From::from)
    }
}

impl<B> Ord for Upper<B>
where
    B: Cobound,
    B::Bound: Ord,
{
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.partial_cmp(rhs).unwrap()
    }
}

impl<B> PartialOrd for Upper<B>
where
    B: Cobound,
    B::Bound: PartialOrd,
{
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        use Ordering::{Equal, Greater, Less};

        match (&self.bound.cobound(), &rhs.bound.cobound()) {
            (Unbounded, Bounded(_)) => Some(Greater),
            (Bounded(_), Unbounded) => Some(Less),
            (Bounded(lhs), Bounded(rhs)) => lhs.partial_cmp(rhs),
            _ => Some(Equal),
        }
    }
}

impl NaturalUpper {
    pub fn into_usize(self) -> Option<usize> {
        match self.into_bound() {
            Variance::Invariant(Zero) => Some(0),
            Variance::Variant(bound) => bound.into_upper().into_usize(),
        }
    }
}

impl NonZeroUpper {
    pub fn into_usize(self) -> Option<usize> {
        self.into_bound().bounded().map(From::from)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum BoundedVariantRange {
    Lower(NonZeroUsize),
    Upper(NonZeroUsize),
    Both {
        lower: NonZeroUsize,
        // The extent must also be non-zero, as that represents an invariant range.
        extent: NonZeroUsize,
    },
}

impl BoundedVariantRange {
    pub fn try_from_lower_and_upper(lower: usize, upper: impl Into<Option<usize>>) -> Option<Self> {
        use BoundedVariantRange::{Both, Lower, Upper};

        // SAFETY: The invariant that the value used to construct a `NonZeroUsize` is not zero is
        //         explicitly checked here.
        unsafe {
            match (lower, upper.into().unwrap_or(0)) {
                (0, 0) => None,
                (lower, 0) => Some(Lower(NonZeroUsize::new_unchecked(lower))),
                (0, upper) => Some(Upper(NonZeroUsize::new_unchecked(upper))),
                (lower, upper) if lower < upper => Some(Both {
                    lower: NonZeroUsize::new_unchecked(lower),
                    extent: NonZeroUsize::new_unchecked(upper - lower),
                }),
                _ => None,
            }
        }
    }

    pub fn union(self, other: impl Into<NaturalRange>) -> VariantRange {
        match NaturalRange::by_lower_and_upper_with(
            self.into(),
            other.into(),
            |LowerUpper { lower, upper }| LowerUpper {
                lower: cmp::min(lower.lhs, lower.rhs),
                upper: cmp::max(upper.lhs, upper.rhs),
            },
        ) {
            Variance::Variant(range) => range,
            _ => unreachable!(),
        }
    }

    pub fn translation(self, vector: usize) -> Self {
        use BoundedVariantRange::{Both, Lower, Upper};

        let expect_add = move |m: NonZeroUsize| {
            m.checked_add(vector)
                .expect("overflow determining translation of range")
        };
        match self {
            Both { lower, extent } => Both {
                lower: expect_add(lower),
                extent,
            },
            Lower(lower) => Lower(expect_add(lower)),
            Upper(upper) => Upper(expect_add(upper)),
        }
    }

    pub fn opened_lower_bound(self) -> VariantRange {
        use BoundedVariantRange::{Both, Lower, Upper};

        match &self {
            Both { .. } => Upper(
                self.upper()
                    .into_bound()
                    .bounded()
                    .expect("closed range has no upper bound"),
            )
            .into(),
            Lower(_) => VariantRange::Unbounded,
            _ => self.into(),
        }
    }

    pub fn lower(&self) -> NonZeroLower {
        use BoundedVariantRange::{Both, Lower, Upper};

        match self {
            Lower(lower) | Both { lower, .. } => Bounded(*lower),
            Upper(_) => Unbounded,
        }
        .into_lower()
    }

    pub fn upper(&self) -> NonZeroUpper {
        use BoundedVariantRange::{Both, Lower, Upper};

        match self {
            Both { lower, extent } => Bounded(Self::upper_from_lower_extent(*lower, *extent)),
            Lower(_) => Unbounded,
            Upper(upper) => Bounded(*upper),
        }
        .into_upper()
    }

    fn upper_from_lower_extent(lower: NonZeroUsize, extent: NonZeroUsize) -> NonZeroUsize {
        lower
            .checked_add(extent.get())
            .expect("overflow determining upper bound of range")
    }
}

impl Conjunction for BoundedVariantRange {
    type Output = Self;

    fn conjunction(self, rhs: Self) -> Self::Output {
        match NaturalRange::by_bound_with(self.into(), rhs.into(), ops::conjunction) {
            Variance::Variant(Bounded(range)) => range,
            _ => unreachable!(),
        }
    }
}

impl Disjunction for BoundedVariantRange {
    type Output = VariantRange;

    fn disjunction(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl OpenedUpperBound for BoundedVariantRange {
    fn opened_upper_bound(self) -> Boundedness<Self> {
        use BoundedVariantRange::{Both, Lower, Upper};

        match &self {
            Both { .. } => Lower(
                self.lower()
                    .into_bound()
                    .bounded()
                    .expect("closed range has no lower bound"),
            )
            .into(),
            Upper(_) => VariantRange::Unbounded,
            _ => self.into(),
        }
    }
}

impl Product for BoundedVariantRange {
    type Output = VariantRange;

    fn product(self, rhs: Self) -> Self::Output {
        match NaturalRange::by_bound_with(self.into(), rhs.into(), ops::product) {
            Variance::Variant(range) => range,
            _ => unreachable!(),
        }
    }
}

impl Product<NonZeroUsize> for BoundedVariantRange {
    type Output = Self;

    fn product(self, rhs: NonZeroUsize) -> Self::Output {
        use Variance::{Invariant, Variant};

        match NaturalRange::by_bound_with(self.into(), Invariant(rhs.into()), ops::product) {
            Variant(Bounded(range)) => range,
            _ => unreachable!(),
        }
    }
}
