//! `From`-like conversions for [`Uint`].

use crate::{ConcatMixed, Limb, Uint, WideWord, Word, U128, U64};

impl<const LIMBS: usize> Uint<LIMBS> {
    /// Create a [`Uint`] from a `u8` (const-friendly)
    // TODO(tarcieri): replace with `const impl From<u8>` when stable
    pub const fn from_u8(n: u8) -> Self {
        assert!(LIMBS >= 1, "number of limbs must be greater than zero");
        let mut limbs = [Limb::ZERO; LIMBS];
        limbs[0].0 = n as Word;
        Self { limbs }
    }

    /// Create a [`Uint`] from a `u16` (const-friendly)
    // TODO(tarcieri): replace with `const impl From<u16>` when stable
    pub const fn from_u16(n: u16) -> Self {
        assert!(LIMBS >= 1, "number of limbs must be greater than zero");
        let mut limbs = [Limb::ZERO; LIMBS];
        limbs[0].0 = n as Word;
        Self { limbs }
    }

    /// Create a [`Uint`] from a `u32` (const-friendly)
    // TODO(tarcieri): replace with `const impl From<u32>` when stable
    #[allow(trivial_numeric_casts)]
    pub const fn from_u32(n: u32) -> Self {
        assert!(LIMBS >= 1, "number of limbs must be greater than zero");
        let mut limbs = [Limb::ZERO; LIMBS];
        limbs[0].0 = n as Word;
        Self { limbs }
    }

    /// Create a [`Uint`] from a `u64` (const-friendly)
    // TODO(tarcieri): replace with `const impl From<u64>` when stable
    #[cfg(target_pointer_width = "32")]
    pub const fn from_u64(n: u64) -> Self {
        assert!(LIMBS >= 2, "number of limbs must be two or greater");
        let mut limbs = [Limb::ZERO; LIMBS];
        limbs[0].0 = (n & 0xFFFFFFFF) as u32;
        limbs[1].0 = (n >> 32) as u32;
        Self { limbs }
    }

    /// Create a [`Uint`] from a `u64` (const-friendly)
    // TODO(tarcieri): replace with `const impl From<u64>` when stable
    #[cfg(target_pointer_width = "64")]
    pub const fn from_u64(n: u64) -> Self {
        assert!(LIMBS >= 1, "number of limbs must be greater than zero");
        let mut limbs = [Limb::ZERO; LIMBS];
        limbs[0].0 = n;
        Self { limbs }
    }

    /// Create a [`Uint`] from a `u128` (const-friendly)
    // TODO(tarcieri): replace with `const impl From<u128>` when stable
    pub const fn from_u128(n: u128) -> Self {
        assert!(
            LIMBS >= (128 / Limb::BITS),
            "number of limbs must be greater than zero"
        );

        let lo = U64::from_u64((n & 0xffff_ffff_ffff_ffff) as u64);
        let hi = U64::from_u64((n >> 64) as u64);

        let mut limbs = [Limb::ZERO; LIMBS];

        let mut i = 0;
        while i < lo.limbs.len() {
            limbs[i] = lo.limbs[i];
            i += 1;
        }

        let mut j = 0;
        while j < hi.limbs.len() {
            limbs[i + j] = hi.limbs[j];
            j += 1;
        }

        Self { limbs }
    }

    /// Create a [`Uint`] from a `Word` (const-friendly)
    // TODO(tarcieri): replace with `const impl From<Word>` when stable
    pub const fn from_word(n: Word) -> Self {
        assert!(LIMBS >= 1, "number of limbs must be greater than zero");
        let mut limbs = [Limb::ZERO; LIMBS];
        limbs[0].0 = n;
        Self { limbs }
    }

    /// Create a [`Uint`] from a `WideWord` (const-friendly)
    // TODO(tarcieri): replace with `const impl From<WideWord>` when stable
    pub const fn from_wide_word(n: WideWord) -> Self {
        assert!(LIMBS >= 2, "number of limbs must be two or greater");
        let mut limbs = [Limb::ZERO; LIMBS];
        limbs[0].0 = n as Word;
        limbs[1].0 = (n >> Limb::BITS) as Word;
        Self { limbs }
    }
}

impl<const LIMBS: usize> From<u8> for Uint<LIMBS> {
    fn from(n: u8) -> Self {
        // TODO(tarcieri): const where clause when possible
        debug_assert!(LIMBS > 0, "limbs must be non-zero");
        Self::from_u8(n)
    }
}

impl<const LIMBS: usize> From<u16> for Uint<LIMBS> {
    fn from(n: u16) -> Self {
        // TODO(tarcieri): const where clause when possible
        debug_assert!(LIMBS > 0, "limbs must be non-zero");
        Self::from_u16(n)
    }
}

impl<const LIMBS: usize> From<u32> for Uint<LIMBS> {
    fn from(n: u32) -> Self {
        // TODO(tarcieri): const where clause when possible
        debug_assert!(LIMBS > 0, "limbs must be non-zero");
        Self::from_u32(n)
    }
}

impl<const LIMBS: usize> From<u64> for Uint<LIMBS> {
    fn from(n: u64) -> Self {
        // TODO(tarcieri): const where clause when possible
        debug_assert!(LIMBS >= (64 / Limb::BITS), "not enough limbs");
        Self::from_u64(n)
    }
}

impl<const LIMBS: usize> From<u128> for Uint<LIMBS> {
    fn from(n: u128) -> Self {
        // TODO(tarcieri): const where clause when possible
        debug_assert!(LIMBS >= (128 / Limb::BITS), "not enough limbs");
        Self::from_u128(n)
    }
}

#[cfg(target_pointer_width = "32")]
impl From<U64> for u64 {
    fn from(n: U64) -> u64 {
        (n.limbs[0].0 as u64) | ((n.limbs[1].0 as u64) << 32)
    }
}

#[cfg(target_pointer_width = "64")]
impl From<U64> for u64 {
    fn from(n: U64) -> u64 {
        n.limbs[0].into()
    }
}

impl From<U128> for u128 {
    fn from(n: U128) -> u128 {
        let mut i = U128::LIMBS - 1;
        let mut res = n.limbs[i].0 as u128;
        while i > 0 {
            i -= 1;
            res = (res << Limb::BITS) | (n.limbs[i].0 as u128);
        }
        res
    }
}

impl<const LIMBS: usize> From<[Word; LIMBS]> for Uint<LIMBS> {
    fn from(arr: [Word; LIMBS]) -> Self {
        Self::from_words(arr)
    }
}

impl<const LIMBS: usize> From<Uint<LIMBS>> for [Word; LIMBS] {
    fn from(n: Uint<LIMBS>) -> [Word; LIMBS] {
        *n.as_ref()
    }
}

impl<const LIMBS: usize> From<[Limb; LIMBS]> for Uint<LIMBS> {
    fn from(limbs: [Limb; LIMBS]) -> Self {
        Self { limbs }
    }
}

impl<const LIMBS: usize> From<Uint<LIMBS>> for [Limb; LIMBS] {
    fn from(n: Uint<LIMBS>) -> [Limb; LIMBS] {
        n.limbs
    }
}

impl<const LIMBS: usize> From<Limb> for Uint<LIMBS> {
    fn from(limb: Limb) -> Self {
        limb.0.into()
    }
}

impl<const L: usize, const H: usize, const LIMBS: usize> From<(Uint<L>, Uint<H>)> for Uint<LIMBS>
where
    Uint<H>: ConcatMixed<Uint<L>, MixedOutput = Uint<LIMBS>>,
{
    fn from(nums: (Uint<L>, Uint<H>)) -> Uint<LIMBS> {
        nums.1.concat_mixed(&nums.0)
    }
}

impl<const L: usize, const H: usize, const LIMBS: usize> From<&(Uint<L>, Uint<H>)> for Uint<LIMBS>
where
    Uint<H>: ConcatMixed<Uint<L>, MixedOutput = Uint<LIMBS>>,
{
    fn from(nums: &(Uint<L>, Uint<H>)) -> Uint<LIMBS> {
        nums.1.concat_mixed(&nums.0)
    }
}

impl<const L: usize, const H: usize, const LIMBS: usize> From<Uint<LIMBS>> for (Uint<L>, Uint<H>) {
    fn from(num: Uint<LIMBS>) -> (Uint<L>, Uint<H>) {
        crate::uint::split::split_mixed(&num)
    }
}

impl<const LIMBS: usize, const LIMBS2: usize> From<&Uint<LIMBS>> for Uint<LIMBS2> {
    fn from(num: &Uint<LIMBS>) -> Uint<LIMBS2> {
        num.resize()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Limb, Word, U128};

    #[cfg(target_pointer_width = "32")]
    use crate::U64 as UintEx;

    #[cfg(target_pointer_width = "64")]
    use crate::U128 as UintEx;

    #[test]
    fn from_u8() {
        let n = UintEx::from(42u8);
        assert_eq!(n.as_limbs(), &[Limb(42), Limb(0)]);
    }

    #[test]
    fn from_u16() {
        let n = UintEx::from(42u16);
        assert_eq!(n.as_limbs(), &[Limb(42), Limb(0)]);
    }

    #[test]
    fn from_u64() {
        let n = UintEx::from(42u64);
        assert_eq!(n.as_limbs(), &[Limb(42), Limb(0)]);
    }

    #[test]
    fn from_u128() {
        let n = U128::from(42u128);
        assert_eq!(&n.as_limbs()[..2], &[Limb(42), Limb(0)]);
        assert_eq!(u128::from(n), 42u128);
    }

    #[test]
    fn array_round_trip() {
        let arr1 = [1, 2];
        let n = UintEx::from(arr1);
        let arr2: [Word; 2] = n.into();
        assert_eq!(arr1, arr2);
    }
}
