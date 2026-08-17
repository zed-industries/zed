use crate::BigInt;

/// Generic trait to implement modular inverse.
pub trait ModInverse<R: Sized>: Sized {
    type Output: Sized;

    /// Function to calculate the [modular multiplicative
    /// inverse](https://en.wikipedia.org/wiki/Modular_multiplicative_inverse) of an integer *a* modulo *m*.
    ///
    /// TODO: references
    /// Returns the modular inverse of `self`.
    /// If none exists it returns `None`.
    fn mod_inverse(self, m: R) -> Option<Self::Output>;
}

/// Generic trait to implement extended GCD.
/// Calculates the extended eucledian algorithm.
/// See https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm for details.
/// The returned values are
/// - greatest common divisor (1)
/// - Bezout coefficients (2)
pub trait ExtendedGcd<R: Sized>: Sized {
    fn extended_gcd(self, other: R) -> (BigInt, BigInt, BigInt);
}
