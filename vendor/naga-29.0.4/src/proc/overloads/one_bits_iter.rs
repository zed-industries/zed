//! An iterator over bitmasks.

/// An iterator that produces the set bits in the given `u64`.
///
/// `OneBitsIter(n)` is an [`Iterator`] that produces each of the set bits in
/// `n`, as a bitmask, in order of increasing value. In other words, it produces
/// the unique sequence of distinct powers of two that adds up to `n`.
///
/// For example, iterating over `OneBitsIter(21)` produces the values `1`, `4`,
/// and `16`, in that order, because `21` is `0xb10101`.
///
/// When `n` is the bits of a bitmask, this iterates over the set bits in the
/// bitmask, in order of increasing bit value. `bitflags` does define an `iter`
/// method, but it's not well-specified or well-implemented.
///
/// The values produced are masks, not bit numbers. Use `u64::trailing_zeros` if
/// you need bit numbers.
pub struct OneBitsIter(u64);

impl OneBitsIter {
    pub const fn new(bits: u64) -> Self {
        Self(bits)
    }
}

impl Iterator for OneBitsIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }

        // Subtracting one from a value in binary clears the lowest `1` bit
        // (call it `B`), and sets all the bits below that.
        let mask = self.0 - 1;

        // Complementing that means that we've instead *set* `B`, *cleared*
        // everything below it, and *complemented* everything above it.
        //
        // Masking with the original value clears everything above and below
        // `B`, leaving only `B` set. This is the value we produce.
        let item = self.0 & !mask;

        // Now that we've produced this bit, remove it from `self.0`.
        self.0 &= mask;

        Some(item)
    }
}

#[test]
fn empty() {
    assert_eq!(OneBitsIter(0).next(), None);
}

#[test]
fn all() {
    let mut obi = OneBitsIter(!0);
    for bit in 0..64 {
        assert_eq!(obi.next(), Some(1 << bit));
    }
    assert_eq!(obi.next(), None);
}

#[test]
fn first() {
    let mut obi = OneBitsIter(1);
    assert_eq!(obi.next(), Some(1));
    assert_eq!(obi.next(), None);
}

#[test]
fn last() {
    let mut obi = OneBitsIter(1 << 63);
    assert_eq!(obi.next(), Some(1 << 63));
    assert_eq!(obi.next(), None);
}

#[test]
fn in_order() {
    let mut obi = OneBitsIter(0b11011000001);
    assert_eq!(obi.next(), Some(1));
    assert_eq!(obi.next(), Some(64));
    assert_eq!(obi.next(), Some(128));
    assert_eq!(obi.next(), Some(512));
    assert_eq!(obi.next(), Some(1024));
    assert_eq!(obi.next(), None);
}
