use core::{cell, f32, f64};
#[cfg(feature = "num-complex")]
use num_complex::Complex;
use AbsDiffEq;

/// Equality comparisons between two numbers using both the absolute difference and
/// relative based comparisons.
pub trait RelativeEq<Rhs = Self>: AbsDiffEq<Rhs>
where
    Rhs: ?Sized,
{
    /// The default relative tolerance for testing values that are far-apart.
    ///
    /// This is used when no `max_relative` value is supplied to the [`relative_eq`] macro.
    fn default_max_relative() -> Self::Epsilon;

    /// A test for equality that uses a relative comparison if the values are far apart.
    fn relative_eq(&self, other: &Rhs, epsilon: Self::Epsilon, max_relative: Self::Epsilon)
        -> bool;

    /// The inverse of [`RelativeEq::relative_eq`].
    fn relative_ne(
        &self,
        other: &Rhs,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        !Self::relative_eq(self, other, epsilon, max_relative)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Base implementations
///////////////////////////////////////////////////////////////////////////////////////////////////

// Implementation based on: [Comparing Floating Point Numbers, 2012 Edition]
// (https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/)
macro_rules! impl_relative_eq {
    ($T:ident, $U:ident) => {
        impl RelativeEq for $T {
            #[inline]
            fn default_max_relative() -> $T {
                $T::EPSILON
            }

            #[inline]
            #[allow(unused_imports)]
            fn relative_eq(&self, other: &$T, epsilon: $T, max_relative: $T) -> bool {
                use num_traits::float::FloatCore;
                // Handle same infinities
                if self == other {
                    return true;
                }

                // Handle remaining infinities
                if $T::is_infinite(*self) || $T::is_infinite(*other) {
                    return false;
                }

                let abs_diff = $T::abs(self - other);

                // For when the numbers are really close together
                if abs_diff <= epsilon {
                    return true;
                }

                let abs_self = $T::abs(*self);
                let abs_other = $T::abs(*other);

                let largest = if abs_other > abs_self {
                    abs_other
                } else {
                    abs_self
                };

                // Use a relative difference comparison
                abs_diff <= largest * max_relative
            }
        }
    };
}

impl_relative_eq!(f32, i32);
impl_relative_eq!(f64, i64);

///////////////////////////////////////////////////////////////////////////////////////////////////
// Derived implementations
///////////////////////////////////////////////////////////////////////////////////////////////////

impl<'a, T: RelativeEq + ?Sized> RelativeEq for &'a T {
    #[inline]
    fn default_max_relative() -> T::Epsilon {
        T::default_max_relative()
    }

    #[inline]
    fn relative_eq(&self, other: &&'a T, epsilon: T::Epsilon, max_relative: T::Epsilon) -> bool {
        T::relative_eq(*self, *other, epsilon, max_relative)
    }
}

impl<'a, T: RelativeEq + ?Sized> RelativeEq for &'a mut T {
    #[inline]
    fn default_max_relative() -> T::Epsilon {
        T::default_max_relative()
    }

    #[inline]
    fn relative_eq(
        &self,
        other: &&'a mut T,
        epsilon: T::Epsilon,
        max_relative: T::Epsilon,
    ) -> bool {
        T::relative_eq(*self, *other, epsilon, max_relative)
    }
}

impl<T: RelativeEq + Copy> RelativeEq for cell::Cell<T> {
    #[inline]
    fn default_max_relative() -> T::Epsilon {
        T::default_max_relative()
    }

    #[inline]
    fn relative_eq(
        &self,
        other: &cell::Cell<T>,
        epsilon: T::Epsilon,
        max_relative: T::Epsilon,
    ) -> bool {
        T::relative_eq(&self.get(), &other.get(), epsilon, max_relative)
    }
}

impl<T: RelativeEq + ?Sized> RelativeEq for cell::RefCell<T> {
    #[inline]
    fn default_max_relative() -> T::Epsilon {
        T::default_max_relative()
    }

    #[inline]
    fn relative_eq(
        &self,
        other: &cell::RefCell<T>,
        epsilon: T::Epsilon,
        max_relative: T::Epsilon,
    ) -> bool {
        T::relative_eq(&self.borrow(), &other.borrow(), epsilon, max_relative)
    }
}

impl<A, B> RelativeEq<[B]> for [A]
where
    A: RelativeEq<B>,
    A::Epsilon: Clone,
{
    #[inline]
    fn default_max_relative() -> A::Epsilon {
        A::default_max_relative()
    }

    #[inline]
    fn relative_eq(&self, other: &[B], epsilon: A::Epsilon, max_relative: A::Epsilon) -> bool {
        self.len() == other.len()
            && Iterator::zip(self.iter(), other)
                .all(|(x, y)| A::relative_eq(x, y, epsilon.clone(), max_relative.clone()))
    }
}

#[cfg(feature = "num-complex")]
impl<T: RelativeEq> RelativeEq for Complex<T>
where
    T::Epsilon: Clone,
{
    #[inline]
    fn default_max_relative() -> T::Epsilon {
        T::default_max_relative()
    }

    #[inline]
    fn relative_eq(
        &self,
        other: &Complex<T>,
        epsilon: T::Epsilon,
        max_relative: T::Epsilon,
    ) -> bool {
        T::relative_eq(&self.re, &other.re, epsilon.clone(), max_relative.clone())
            && T::relative_eq(&self.im, &other.im, epsilon, max_relative)
    }
}
