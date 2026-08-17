use core::cell;
#[cfg(feature = "num-complex")]
use num_complex::Complex;

/// Equality that is defined using the absolute difference of two numbers.
pub trait AbsDiffEq<Rhs = Self>: PartialEq<Rhs>
where
    Rhs: ?Sized,
{
    /// Used for specifying relative comparisons.
    type Epsilon;

    /// The default tolerance to use when testing values that are close together.
    ///
    /// This is used when no `epsilon` value is supplied to the [`abs_diff_eq!`], [`relative_eq!`],
    /// or [`ulps_eq!`] macros.
    fn default_epsilon() -> Self::Epsilon;

    /// A test for equality that uses the absolute difference to compute the approximate
    /// equality of two numbers.
    fn abs_diff_eq(&self, other: &Rhs, epsilon: Self::Epsilon) -> bool;

    /// The inverse of [`AbsDiffEq::abs_diff_eq`].
    fn abs_diff_ne(&self, other: &Rhs, epsilon: Self::Epsilon) -> bool {
        !Self::abs_diff_eq(self, other, epsilon)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Base implementations
///////////////////////////////////////////////////////////////////////////////////////////////////

macro_rules! impl_unsigned_abs_diff_eq {
    ($T:ident, $default_epsilon:expr) => {
        impl AbsDiffEq for $T {
            type Epsilon = $T;

            #[inline]
            fn default_epsilon() -> $T {
                $default_epsilon
            }

            #[inline]
            fn abs_diff_eq(&self, other: &$T, epsilon: $T) -> bool {
                (if self > other {
                    self - other
                } else {
                    other - self
                }) <= epsilon
            }
        }
    };
}

impl_unsigned_abs_diff_eq!(u8, 0);
impl_unsigned_abs_diff_eq!(u16, 0);
impl_unsigned_abs_diff_eq!(u32, 0);
impl_unsigned_abs_diff_eq!(u64, 0);
impl_unsigned_abs_diff_eq!(usize, 0);

macro_rules! impl_signed_abs_diff_eq {
    ($T:ident, $default_epsilon:expr) => {
        impl AbsDiffEq for $T {
            type Epsilon = $T;

            #[inline]
            fn default_epsilon() -> $T {
                $default_epsilon
            }

            #[inline]
            #[allow(unused_imports)]
            fn abs_diff_eq(&self, other: &$T, epsilon: $T) -> bool {
                use num_traits::float::FloatCore;
                $T::abs(self - other) <= epsilon
            }
        }
    };
}

impl_signed_abs_diff_eq!(i8, 0);
impl_signed_abs_diff_eq!(i16, 0);
impl_signed_abs_diff_eq!(i32, 0);
impl_signed_abs_diff_eq!(i64, 0);
impl_signed_abs_diff_eq!(isize, 0);
impl_signed_abs_diff_eq!(f32, core::f32::EPSILON);
impl_signed_abs_diff_eq!(f64, core::f64::EPSILON);

///////////////////////////////////////////////////////////////////////////////////////////////////
// Derived implementations
///////////////////////////////////////////////////////////////////////////////////////////////////

impl<'a, T: AbsDiffEq + ?Sized> AbsDiffEq for &'a T {
    type Epsilon = T::Epsilon;

    #[inline]
    fn default_epsilon() -> T::Epsilon {
        T::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &&'a T, epsilon: T::Epsilon) -> bool {
        T::abs_diff_eq(*self, *other, epsilon)
    }
}

impl<'a, T: AbsDiffEq + ?Sized> AbsDiffEq for &'a mut T {
    type Epsilon = T::Epsilon;

    #[inline]
    fn default_epsilon() -> T::Epsilon {
        T::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &&'a mut T, epsilon: T::Epsilon) -> bool {
        T::abs_diff_eq(*self, *other, epsilon)
    }
}

impl<T: AbsDiffEq + Copy> AbsDiffEq for cell::Cell<T> {
    type Epsilon = T::Epsilon;

    #[inline]
    fn default_epsilon() -> T::Epsilon {
        T::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &cell::Cell<T>, epsilon: T::Epsilon) -> bool {
        T::abs_diff_eq(&self.get(), &other.get(), epsilon)
    }
}

impl<T: AbsDiffEq + ?Sized> AbsDiffEq for cell::RefCell<T> {
    type Epsilon = T::Epsilon;

    #[inline]
    fn default_epsilon() -> T::Epsilon {
        T::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &cell::RefCell<T>, epsilon: T::Epsilon) -> bool {
        T::abs_diff_eq(&self.borrow(), &other.borrow(), epsilon)
    }
}

impl<A, B> AbsDiffEq<[B]> for [A]
where
    A: AbsDiffEq<B>,
    A::Epsilon: Clone,
{
    type Epsilon = A::Epsilon;

    #[inline]
    fn default_epsilon() -> A::Epsilon {
        A::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &[B], epsilon: A::Epsilon) -> bool {
        self.len() == other.len()
            && Iterator::zip(self.iter(), other).all(|(x, y)| A::abs_diff_eq(x, y, epsilon.clone()))
    }
}

#[cfg(feature = "num-complex")]
impl<T: AbsDiffEq> AbsDiffEq for Complex<T>
where
    T::Epsilon: Clone,
{
    type Epsilon = T::Epsilon;

    #[inline]
    fn default_epsilon() -> T::Epsilon {
        T::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &Complex<T>, epsilon: T::Epsilon) -> bool {
        T::abs_diff_eq(&self.re, &other.re, epsilon.clone())
            && T::abs_diff_eq(&self.im, &other.im, epsilon)
    }
}
