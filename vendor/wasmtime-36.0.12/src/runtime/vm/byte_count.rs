use core::fmt;

use super::host_page_size;

/// A number of bytes that's guaranteed to be aligned to the host page size.
///
/// This is used to manage page-aligned memory allocations.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HostAlignedByteCount(
    // Invariant: this is always a multiple of the host page size.
    usize,
);

impl HostAlignedByteCount {
    /// A zero byte count.
    pub const ZERO: Self = Self(0);

    /// Creates a new `HostAlignedByteCount` from an aligned byte count.
    ///
    /// Returns an error if `bytes` is not page-aligned.
    pub fn new(bytes: usize) -> Result<Self, ByteCountNotAligned> {
        let host_page_size = host_page_size();
        if bytes % host_page_size == 0 {
            Ok(Self(bytes))
        } else {
            Err(ByteCountNotAligned(bytes))
        }
    }

    /// Creates a new `HostAlignedByteCount` from an aligned byte count without
    /// checking validity.
    ///
    /// ## Safety
    ///
    /// The caller must ensure that `bytes` is page-aligned.
    pub unsafe fn new_unchecked(bytes: usize) -> Self {
        debug_assert!(
            bytes % host_page_size() == 0,
            "byte count {bytes} is not page-aligned (page size = {})",
            host_page_size(),
        );
        Self(bytes)
    }

    /// Creates a new `HostAlignedByteCount`, rounding up to the nearest page.
    ///
    /// Returns an error if `bytes + page_size - 1` overflows.
    pub fn new_rounded_up(bytes: usize) -> Result<Self, ByteCountOutOfBounds> {
        let page_size = host_page_size();
        debug_assert!(page_size.is_power_of_two());
        match bytes.checked_add(page_size - 1) {
            Some(v) => Ok(Self(v & !(page_size - 1))),
            None => Err(ByteCountOutOfBounds(ByteCountOutOfBoundsKind::RoundUp)),
        }
    }

    /// Creates a new `HostAlignedByteCount` from a `u64`, rounding up to the nearest page.
    ///
    /// Returns an error if the `u64` overflows `usize`, or if `bytes +
    /// page_size - 1` overflows.
    pub fn new_rounded_up_u64(bytes: u64) -> Result<Self, ByteCountOutOfBounds> {
        let bytes = bytes
            .try_into()
            .map_err(|_| ByteCountOutOfBounds(ByteCountOutOfBoundsKind::ConvertU64))?;
        Self::new_rounded_up(bytes)
    }

    /// Returns the host page size.
    pub fn host_page_size() -> HostAlignedByteCount {
        // The host page size is always a multiple of itself.
        HostAlignedByteCount(host_page_size())
    }

    /// Returns true if the page count is zero.
    #[inline]
    pub fn is_zero(self) -> bool {
        self == Self::ZERO
    }

    /// Returns the number of bytes as a `usize`.
    #[inline]
    pub fn byte_count(self) -> usize {
        self.0
    }

    /// Add two aligned byte counts together.
    ///
    /// Returns an error if the result overflows.
    pub fn checked_add(self, bytes: HostAlignedByteCount) -> Result<Self, ByteCountOutOfBounds> {
        // aligned + aligned = aligned
        self.0
            .checked_add(bytes.0)
            .map(Self)
            .ok_or(ByteCountOutOfBounds(ByteCountOutOfBoundsKind::Add))
    }

    // Note: saturating_add should not be naively added! usize::MAX is not a
    // power of 2 so is not aligned.

    /// Compute `self - bytes`.
    ///
    /// Returns an error if the result underflows.
    pub fn checked_sub(self, bytes: HostAlignedByteCount) -> Result<Self, ByteCountOutOfBounds> {
        // aligned - aligned = aligned
        self.0
            .checked_sub(bytes.0)
            .map(Self)
            .ok_or_else(|| ByteCountOutOfBounds(ByteCountOutOfBoundsKind::Sub))
    }

    /// Compute `self - bytes`, returning zero if the result underflows.
    #[inline]
    pub fn saturating_sub(self, bytes: HostAlignedByteCount) -> Self {
        // aligned - aligned = aligned, and 0 is always aligned.
        Self(self.0.saturating_sub(bytes.0))
    }

    /// Multiply an aligned byte count by a scalar value.
    ///
    /// Returns an error if the result overflows.
    pub fn checked_mul(self, scalar: usize) -> Result<Self, ByteCountOutOfBounds> {
        // aligned * scalar = aligned
        self.0
            .checked_mul(scalar)
            .map(Self)
            .ok_or_else(|| ByteCountOutOfBounds(ByteCountOutOfBoundsKind::Mul))
    }

    /// Divide an aligned byte count by another aligned byte count, producing a
    /// scalar value.
    ///
    /// Returns an error in case the divisor is zero.
    pub fn checked_div(self, divisor: HostAlignedByteCount) -> Result<usize, ByteCountOutOfBounds> {
        self.0
            .checked_div(divisor.0)
            .ok_or_else(|| ByteCountOutOfBounds(ByteCountOutOfBoundsKind::Div))
    }

    /// Compute the remainder of an aligned byte count divided by another
    /// aligned byte count.
    ///
    /// The remainder is always an aligned byte count itself.
    ///
    /// Returns an error in case the divisor is zero.
    pub fn checked_rem(self, divisor: HostAlignedByteCount) -> Result<Self, ByteCountOutOfBounds> {
        // Why is the remainder an aligned byte count? For example, if the page
        // size is 4KiB, then the remainder of dividing (say) 40KiB by 16KiB is
        // 8KiB, which is a multiple of 4KiB.
        //
        // More generally, for integers n >= 0, m > 0, k > 0:
        //
        //     (n * k) % (m * k) = (n % m) * k
        //
        // which is a multiple of k. Here, k is the host page size, so the
        // remainder is a multiple of the host page size.
        self.0
            .checked_rem(divisor.0)
            .map(Self)
            .ok_or_else(|| ByteCountOutOfBounds(ByteCountOutOfBoundsKind::Rem))
    }

    /// Unchecked multiplication by a scalar value.
    ///
    /// ## Safety
    ///
    /// The result must not overflow.
    #[inline]
    pub unsafe fn unchecked_mul(self, n: usize) -> Self {
        Self(self.0 * n)
    }
}

impl PartialEq<usize> for HostAlignedByteCount {
    #[inline]
    fn eq(&self, other: &usize) -> bool {
        self.0 == *other
    }
}

impl PartialEq<HostAlignedByteCount> for usize {
    #[inline]
    fn eq(&self, other: &HostAlignedByteCount) -> bool {
        *self == other.0
    }
}

struct LowerHexDisplay<T>(T);

impl<T: fmt::LowerHex> fmt::Display for LowerHexDisplay<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Use the LowerHex impl as the Display impl, ensuring that there's
        // always a 0x in the beginning (i.e. that the alternate formatter is
        // used.)
        if f.alternate() {
            fmt::LowerHex::fmt(&self.0, f)
        } else {
            // Unfortunately, fill and alignment aren't respected this way, but
            // it's quite hard to construct a new formatter with mostly the same
            // options but the alternate flag set.
            // https://github.com/rust-lang/rust/pull/118159 would make this
            // easier.
            write!(f, "{:#x}", self.0)
        }
    }
}

impl fmt::Display for HostAlignedByteCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Use the LowerHex impl as the Display impl, ensuring that there's
        // always a 0x in the beginning (i.e. that the alternate formatter is
        // used.)
        fmt::Display::fmt(&LowerHexDisplay(self.0), f)
    }
}

impl fmt::LowerHex for HostAlignedByteCount {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(&self.0, f)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ByteCountNotAligned(usize);

impl fmt::Display for ByteCountNotAligned {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "byte count not page-aligned: {}",
            LowerHexDisplay(self.0)
        )
    }
}

impl core::error::Error for ByteCountNotAligned {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ByteCountOutOfBounds(ByteCountOutOfBoundsKind);

impl fmt::Display for ByteCountOutOfBounds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl core::error::Error for ByteCountOutOfBounds {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ByteCountOutOfBoundsKind {
    // We don't carry the arguments that errored out to avoid the error type
    // becoming too big.
    RoundUp,
    ConvertU64,
    Add,
    Sub,
    Mul,
    Div,
    Rem,
}

impl fmt::Display for ByteCountOutOfBoundsKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ByteCountOutOfBoundsKind::RoundUp => f.write_str("byte count overflow rounding up"),
            ByteCountOutOfBoundsKind::ConvertU64 => {
                f.write_str("byte count overflow converting u64")
            }
            ByteCountOutOfBoundsKind::Add => f.write_str("byte count overflow during addition"),
            ByteCountOutOfBoundsKind::Sub => f.write_str("byte count underflow during subtraction"),
            ByteCountOutOfBoundsKind::Mul => {
                f.write_str("byte count overflow during multiplication")
            }
            ByteCountOutOfBoundsKind::Div => f.write_str("division by zero"),
            ByteCountOutOfBoundsKind::Rem => f.write_str("remainder by zero"),
        }
    }
}

#[cfg(test)]
mod proptest_impls {
    use super::*;

    use proptest::prelude::*;

    impl Arbitrary for HostAlignedByteCount {
        type Strategy = BoxedStrategy<Self>;
        type Parameters = ();

        fn arbitrary_with(_: ()) -> Self::Strategy {
            // Compute the number of pages that fit in a usize, rounded down.
            // For example, if:
            //
            // * usize::MAX is 2**64 - 1
            // * host_page_size is 2**12 (4KiB)
            //
            // Then page_count = floor(usize::MAX / host_page_size) = 2**52 - 1.
            // The range 0..=page_count, when multiplied by the page size, will
            // produce values in the range 0..=(2**64 - 2**12), in steps of
            // 2**12, uniformly at random. This is the desired uniform
            // distribution of byte counts.
            let page_count = usize::MAX / host_page_size();
            (0..=page_count)
                .prop_map(|n| HostAlignedByteCount::new(n * host_page_size()).unwrap())
                .boxed()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byte_count_display() {
        // Pages should hopefully be 64k or smaller.
        let byte_count = HostAlignedByteCount::new(65536).unwrap();

        assert_eq!(format!("{byte_count}"), "0x10000");
        assert_eq!(format!("{byte_count:x}"), "10000");
        assert_eq!(format!("{byte_count:#x}"), "0x10000");
    }

    #[test]
    fn byte_count_ops() {
        let host_page_size = host_page_size();
        HostAlignedByteCount::new(0).expect("0 is aligned");
        HostAlignedByteCount::new(host_page_size).expect("host_page_size is aligned");
        HostAlignedByteCount::new(host_page_size * 2).expect("host_page_size * 2 is aligned");
        HostAlignedByteCount::new(host_page_size + 1)
            .expect_err("host_page_size + 1 is not aligned");
        HostAlignedByteCount::new(host_page_size / 2)
            .expect_err("host_page_size / 2 is not aligned");

        // Rounding up.
        HostAlignedByteCount::new_rounded_up(usize::MAX).expect_err("usize::MAX overflows");
        assert_eq!(
            HostAlignedByteCount::new_rounded_up(usize::MAX - host_page_size)
                .expect("(usize::MAX - 1 page) is in bounds"),
            HostAlignedByteCount::new((usize::MAX - host_page_size) + 1)
                .expect("usize::MAX is 2**N - 1"),
        );

        // Addition.
        let half_max = HostAlignedByteCount::new((usize::MAX >> 1) + 1)
            .expect("(usize::MAX >> 1) + 1 is aligned");
        half_max
            .checked_add(HostAlignedByteCount::host_page_size())
            .expect("half max + page size is in bounds");
        half_max
            .checked_add(half_max)
            .expect_err("half max + half max is out of bounds");

        // Subtraction.
        let half_max_minus_one = half_max
            .checked_sub(HostAlignedByteCount::host_page_size())
            .expect("(half_max - 1 page) is in bounds");
        assert_eq!(
            half_max.checked_sub(half_max),
            Ok(HostAlignedByteCount::ZERO)
        );
        assert_eq!(
            half_max.checked_sub(half_max_minus_one),
            Ok(HostAlignedByteCount::host_page_size())
        );
        half_max_minus_one
            .checked_sub(half_max)
            .expect_err("(half_max - 1 page) - half_max is out of bounds");

        // Multiplication.
        half_max
            .checked_mul(2)
            .expect_err("half max * 2 is out of bounds");
        half_max_minus_one
            .checked_mul(2)
            .expect("(half max - 1 page) * 2 is in bounds");
    }
}
