//! Heap-allocated big unsigned integers.

mod add;
mod cmp;

use crate::{Limb, Word};
use alloc::{vec, vec::Vec};
use core::fmt;

#[cfg(feature = "zeroize")]
use zeroize::Zeroize;

/// Fixed-precision heap-allocated big unsigned integer.
///
/// Alternative to the stack-allocated [`Uint`][`crate::Uint`] but with a
/// fixed precision chosen at runtime instead of compile time.
///
/// Unlike many other heap-allocated big integer libraries, this type is not
/// arbitrary precision and will wrap at its fixed-precision rather than
/// automatically growing.
#[derive(Clone, Default)]
pub struct BoxedUint {
    /// Inner limb vector. Stored from least significant to most significant.
    limbs: Vec<Limb>,
}

impl BoxedUint {
    /// Get the value `0`, represented as succinctly as possible.
    pub fn zero() -> Self {
        Self::default()
    }

    /// Get the value `1`, represented as succinctly as possible.
    pub fn one() -> Self {
        Self {
            limbs: vec![Limb::ONE; 1],
        }
    }

    /// Create a new [`BoxedUint`] with the given number of bits of precision.
    ///
    /// Returns `None` if the number of bits is not a multiple of the
    /// [`Limb`] size.
    pub fn new(bits_precision: usize) -> Option<Self> {
        if bits_precision == 0 || bits_precision % Limb::BITS != 0 {
            return None;
        }

        let nlimbs = bits_precision / Limb::BITS;

        Some(Self {
            limbs: vec![Limb::ZERO; nlimbs],
        })
    }

    /// Get the maximum value for a given number of bits of precision.
    ///
    /// Returns `None` if the number of bits is not a multiple of the
    /// [`Limb`] size.
    pub fn max(bits_precision: usize) -> Option<Self> {
        let mut ret = Self::new(bits_precision)?;

        for limb in &mut ret.limbs {
            *limb = Limb::MAX;
        }

        Some(ret)
    }

    /// Create a [`BoxedUint`] from an array of [`Word`]s (i.e. word-sized unsigned
    /// integers).
    #[inline]
    pub fn from_words(words: &[Word]) -> Self {
        Self {
            limbs: words.iter().copied().map(Into::into).collect(),
        }
    }

    /// Create an array of [`Word`]s (i.e. word-sized unsigned integers) from
    /// a [`BoxedUint`].
    #[inline]
    pub fn to_words(&self) -> Vec<Word> {
        self.limbs.iter().copied().map(Into::into).collect()
    }

    /// Borrow the inner limbs as a slice of [`Word`]s.
    pub fn as_words(&self) -> &[Word] {
        // SAFETY: `Limb` is a `repr(transparent)` newtype for `Word`
        #[allow(trivial_casts, unsafe_code)]
        unsafe {
            &*((self.limbs.as_slice() as *const _) as *const [Word])
        }
    }

    /// Borrow the inner limbs as a mutable array of [`Word`]s.
    pub fn as_words_mut(&mut self) -> &mut [Word] {
        // SAFETY: `Limb` is a `repr(transparent)` newtype for `Word`
        #[allow(trivial_casts, unsafe_code)]
        unsafe {
            &mut *((self.limbs.as_mut_slice() as *mut _) as *mut [Word])
        }
    }

    /// Borrow the limbs of this [`BoxedUint`].
    pub fn as_limbs(&self) -> &[Limb] {
        self.limbs.as_ref()
    }

    /// Borrow the limbs of this [`BoxedUint`] mutably.
    pub fn as_limbs_mut(&mut self) -> &mut [Limb] {
        self.limbs.as_mut()
    }

    /// Convert this [`BoxedUint`] into its inner limbs.
    pub fn to_limbs(&self) -> Vec<Limb> {
        self.limbs.clone()
    }

    /// Convert this [`BoxedUint`] into its inner limbs.
    pub fn into_limbs(self) -> Vec<Limb> {
        self.limbs
    }

    /// Get the precision of this [`BoxedUint`] in bits.
    pub fn bits(&self) -> usize {
        self.limbs.len() * Limb::BITS
    }

    /// Sort two [`BoxedUint`]s by precision, returning a tuple of the shorter
    /// followed by the longer, or the original order if their precision is
    /// equal.
    fn sort_by_precision<'a>(a: &'a Self, b: &'a Self) -> (&'a Self, &'a Self) {
        if a.limbs.len() <= b.limbs.len() {
            (a, b)
        } else {
            (b, a)
        }
    }

    /// Perform a carry chain-like operation over the limbs of the inputs,
    /// constructing a result from the returned limbs and carry.
    ///
    /// If one of the two values has fewer limbs than the other, passes
    /// [`Limb::ZERO`] as the value for that limb.
    fn chain<F>(a: &Self, b: &Self, mut carry: Limb, f: F) -> (Self, Limb)
    where
        F: Fn(Limb, Limb, Limb) -> (Limb, Limb),
    {
        let (shorter, longer) = Self::sort_by_precision(a, b);
        let mut limbs = Vec::with_capacity(longer.limbs.len());

        for i in 0..longer.limbs.len() {
            let &a = shorter.limbs.get(i).unwrap_or(&Limb::ZERO);
            let &b = longer.limbs.get(i).unwrap_or(&Limb::ZERO);
            let (limb, c) = f(a, b, carry);
            limbs.push(limb);
            carry = c;
        }

        (Self { limbs }, carry)
    }
}

impl AsRef<[Word]> for BoxedUint {
    fn as_ref(&self) -> &[Word] {
        self.as_words()
    }
}

impl AsMut<[Word]> for BoxedUint {
    fn as_mut(&mut self) -> &mut [Word] {
        self.as_words_mut()
    }
}

impl AsRef<[Limb]> for BoxedUint {
    fn as_ref(&self) -> &[Limb] {
        self.as_limbs()
    }
}

impl AsMut<[Limb]> for BoxedUint {
    fn as_mut(&mut self) -> &mut [Limb] {
        self.as_limbs_mut()
    }
}

impl fmt::Debug for BoxedUint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BoxedUint(0x{self:X})")
    }
}

impl fmt::Display for BoxedUint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::UpperHex::fmt(self, f)
    }
}

impl fmt::LowerHex for BoxedUint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.limbs.is_empty() {
            return fmt::LowerHex::fmt(&Limb::ZERO, f);
        }

        for limb in self.limbs.iter().rev() {
            fmt::LowerHex::fmt(limb, f)?;
        }
        Ok(())
    }
}

impl fmt::UpperHex for BoxedUint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.limbs.is_empty() {
            return fmt::LowerHex::fmt(&Limb::ZERO, f);
        }

        for limb in self.limbs.iter().rev() {
            fmt::UpperHex::fmt(limb, f)?;
        }
        Ok(())
    }
}

#[cfg(feature = "zeroize")]
impl Zeroize for BoxedUint {
    fn zeroize(&mut self) {
        self.limbs.zeroize();
    }
}
