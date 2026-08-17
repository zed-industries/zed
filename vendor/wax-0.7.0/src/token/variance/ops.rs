use std::num::NonZeroUsize;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BinaryOperand<T> {
    pub lhs: T,
    pub rhs: T,
}

pub trait Conjunction<T = Self> {
    type Output;

    fn conjunction(self, rhs: T) -> Self::Output;
}

impl Conjunction for NonZeroUsize {
    type Output = Self;

    fn conjunction(self, rhs: Self) -> Self::Output {
        self.checked_add(rhs.into())
            .expect("overflow determining conjunction of unsigned word")
    }
}

impl Conjunction for usize {
    type Output = Self;

    fn conjunction(self, rhs: Self) -> Self::Output {
        self.checked_add(rhs)
            .expect("overflow determining conjunction of unsigned word")
    }
}

pub trait Disjunction<T = Self> {
    type Output;

    fn disjunction(self, rhs: T) -> Self::Output;
}

pub trait Product<T = Self> {
    type Output;

    fn product(self, rhs: T) -> Self::Output;
}

impl Product for NonZeroUsize {
    type Output = Self;

    fn product(self, rhs: Self) -> Self::Output {
        self.checked_mul(rhs)
            .expect("overflow determining product of unsigned word")
    }
}

impl Product for usize {
    type Output = Self;

    fn product(self, rhs: Self) -> Self::Output {
        self.checked_mul(rhs)
            .expect("overflow determining product of unsigned word")
    }
}

pub fn conjunction<T, U>(lhs: T, rhs: U) -> T::Output
where
    T: Conjunction<U>,
{
    lhs.conjunction(rhs)
}

pub fn disjunction<T, U>(lhs: T, rhs: U) -> T::Output
where
    T: Disjunction<U>,
{
    lhs.disjunction(rhs)
}

pub fn product<T, U>(lhs: T, rhs: U) -> T::Output
where
    T: Product<U>,
{
    lhs.product(rhs)
}
