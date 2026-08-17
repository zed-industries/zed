use core::cell;
#[cfg(feature = "num-complex")]
use num_complex::Complex;
use num_traits::Signed;

use AbsDiffEq;

/// Equality comparisons between two numbers using both the absolute difference and ULPs
/// (Units in Last Place) based comparisons.
pub trait UlpsEq<Rhs = Self>: AbsDiffEq<Rhs>
where
    Rhs: ?Sized,
{
    /// The default ULPs to tolerate when testing values that are far-apart.
    ///
    /// This is used when no `max_ulps` value is supplied to the [`ulps_eq`] macro.
    fn default_max_ulps() -> u32;

    /// A test for equality that uses units in the last place (ULP) if the values are far apart.
    fn ulps_eq(&self, other: &Rhs, epsilon: Self::Epsilon, max_ulps: u32) -> bool;

    /// The inverse of [`UlpsEq::ulps_eq`].
    fn ulps_ne(&self, other: &Rhs, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        !Self::ulps_eq(self, other, epsilon, max_ulps)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Base implementations
///////////////////////////////////////////////////////////////////////////////////////////////////

// Implementation based on: [Comparing Floating Point Numbers, 2012 Edition]
// (https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/)
macro_rules! impl_ulps_eq {
    ($T:ident, $U:ident) => {
        impl UlpsEq for $T {
            #[inline]
            fn default_max_ulps() -> u32 {
                4
            }

            #[inline]
            fn ulps_eq(&self, other: &$T, epsilon: $T, max_ulps: u32) -> bool {
                // For when the numbers are really close together
                if $T::abs_diff_eq(self, other, epsilon) {
                    return true;
                }

                // Trivial negative sign check
                if self.signum() != other.signum() {
                    return false;
                }

                // ULPS difference comparison
                let int_self: $U = self.to_bits();
                let int_other: $U = other.to_bits();

                // To be replaced with `abs_sub`, if
                // https://github.com/rust-lang/rust/issues/62111 lands.
                if int_self <= int_other {
                    int_other - int_self <= max_ulps as $U
                } else {
                    int_self - int_other <= max_ulps as $U
                }
            }
        }
    };
}

impl_ulps_eq!(f32, u32);
impl_ulps_eq!(f64, u64);

///////////////////////////////////////////////////////////////////////////////////////////////////
// Derived implementations
///////////////////////////////////////////////////////////////////////////////////////////////////

impl<'a, T: UlpsEq + ?Sized> UlpsEq for &'a T {
    #[inline]
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    #[inline]
    fn ulps_eq(&self, other: &&'a T, epsilon: T::Epsilon, max_ulps: u32) -> bool {
        T::ulps_eq(*self, *other, epsilon, max_ulps)
    }
}

impl<'a, T: UlpsEq + ?Sized> UlpsEq for &'a mut T {
    #[inline]
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    #[inline]
    fn ulps_eq(&self, other: &&'a mut T, epsilon: T::Epsilon, max_ulps: u32) -> bool {
        T::ulps_eq(*self, *other, epsilon, max_ulps)
    }
}

impl<T: UlpsEq + Copy> UlpsEq for cell::Cell<T> {
    #[inline]
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    #[inline]
    fn ulps_eq(&self, other: &cell::Cell<T>, epsilon: T::Epsilon, max_ulps: u32) -> bool {
        T::ulps_eq(&self.get(), &other.get(), epsilon, max_ulps)
    }
}

impl<T: UlpsEq + ?Sized> UlpsEq for cell::RefCell<T> {
    #[inline]
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    #[inline]
    fn ulps_eq(&self, other: &cell::RefCell<T>, epsilon: T::Epsilon, max_ulps: u32) -> bool {
        T::ulps_eq(&self.borrow(), &other.borrow(), epsilon, max_ulps)
    }
}

impl<A, B> UlpsEq<[B]> for [A]
where
    A: UlpsEq<B>,
    A::Epsilon: Clone,
{
    #[inline]
    fn default_max_ulps() -> u32 {
        A::default_max_ulps()
    }

    #[inline]
    fn ulps_eq(&self, other: &[B], epsilon: A::Epsilon, max_ulps: u32) -> bool {
        self.len() == other.len()
            && Iterator::zip(self.iter(), other)
                .all(|(x, y)| A::ulps_eq(x, y, epsilon.clone(), max_ulps))
    }
}

#[cfg(feature = "num-complex")]
impl<T: UlpsEq> UlpsEq for Complex<T>
where
    T::Epsilon: Clone,
{
    #[inline]
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    #[inline]
    fn ulps_eq(&self, other: &Complex<T>, epsilon: T::Epsilon, max_ulps: u32) -> bool {
        T::ulps_eq(&self.re, &other.re, epsilon.clone(), max_ulps)
            && T::ulps_eq(&self.im, &other.im, epsilon, max_ulps)
    }
}
