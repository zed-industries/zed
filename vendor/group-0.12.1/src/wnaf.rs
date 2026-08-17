use alloc::vec::Vec;
use core::iter;
use core::marker::PhantomData;
use core::ops::Mul;

use ff::PrimeField;

use super::Group;

/// Extension trait on a [`Group`] that provides helpers used by [`Wnaf`].
pub trait WnafGroup: Group {
    /// Recommends a wNAF window size given the number of scalars you intend to multiply
    /// a base by. Always returns a number between 2 and 22, inclusive.
    fn recommended_wnaf_for_num_scalars(num_scalars: usize) -> usize;
}

/// Replaces the contents of `table` with a w-NAF window table for the given window size.
pub(crate) fn wnaf_table<G: Group>(table: &mut Vec<G>, mut base: G, window: usize) {
    table.truncate(0);
    table.reserve(1 << (window - 1));

    let dbl = base.double();

    for _ in 0..(1 << (window - 1)) {
        table.push(base);
        base.add_assign(&dbl);
    }
}

/// This struct represents a view of a sequence of bytes as a sequence of
/// `u64` limbs in little-endian byte order. It maintains a current index, and
/// allows access to the limb at that index and the one following it. Bytes
/// beyond the end of the original buffer are treated as zero.
struct LimbBuffer<'a> {
    buf: &'a [u8],
    cur_idx: usize,
    cur_limb: u64,
    next_limb: u64,
}

impl<'a> LimbBuffer<'a> {
    fn new(buf: &'a [u8]) -> Self {
        let mut ret = Self {
            buf,
            cur_idx: 0,
            cur_limb: 0,
            next_limb: 0,
        };

        // Initialise the limb buffers.
        ret.increment_limb();
        ret.increment_limb();
        ret.cur_idx = 0usize;

        ret
    }

    fn increment_limb(&mut self) {
        self.cur_idx += 1;
        self.cur_limb = self.next_limb;
        match self.buf.len() {
            // There are no more bytes in the buffer; zero-extend.
            0 => self.next_limb = 0,

            // There are fewer bytes in the buffer than a u64 limb; zero-extend.
            x @ 1..=7 => {
                let mut next_limb = [0; 8];
                next_limb[..x].copy_from_slice(self.buf);
                self.next_limb = u64::from_le_bytes(next_limb);
                self.buf = &[];
            }

            // There are at least eight bytes in the buffer; read the next u64 limb.
            _ => {
                let (next_limb, rest) = self.buf.split_at(8);
                self.next_limb = u64::from_le_bytes(next_limb.try_into().unwrap());
                self.buf = rest;
            }
        }
    }

    fn get(&mut self, idx: usize) -> (u64, u64) {
        assert!([self.cur_idx, self.cur_idx + 1].contains(&idx));
        if idx > self.cur_idx {
            self.increment_limb();
        }
        (self.cur_limb, self.next_limb)
    }
}

/// Replaces the contents of `wnaf` with the w-NAF representation of a little-endian
/// scalar.
pub(crate) fn wnaf_form<S: AsRef<[u8]>>(wnaf: &mut Vec<i64>, c: S, window: usize) {
    // Required by the NAF definition
    debug_assert!(window >= 2);
    // Required so that the NAF digits fit in i64
    debug_assert!(window <= 64);

    let bit_len = c.as_ref().len() * 8;

    wnaf.truncate(0);
    wnaf.reserve(bit_len);

    // Initialise the current and next limb buffers.
    let mut limbs = LimbBuffer::new(c.as_ref());

    let width = 1u64 << window;
    let window_mask = width - 1;

    let mut pos = 0;
    let mut carry = 0;
    while pos < bit_len {
        // Construct a buffer of bits of the scalar, starting at bit `pos`
        let u64_idx = pos / 64;
        let bit_idx = pos % 64;
        let (cur_u64, next_u64) = limbs.get(u64_idx);
        let bit_buf = if bit_idx + window < 64 {
            // This window's bits are contained in a single u64
            cur_u64 >> bit_idx
        } else {
            // Combine the current u64's bits with the bits from the next u64
            (cur_u64 >> bit_idx) | (next_u64 << (64 - bit_idx))
        };

        // Add the carry into the current window
        let window_val = carry + (bit_buf & window_mask);

        if window_val & 1 == 0 {
            // If the window value is even, preserve the carry and emit 0.
            // Why is the carry preserved?
            // If carry == 0 and window_val & 1 == 0, then the next carry should be 0
            // If carry == 1 and window_val & 1 == 0, then bit_buf & 1 == 1 so the next carry should be 1
            wnaf.push(0);
            pos += 1;
        } else {
            wnaf.push(if window_val < width / 2 {
                carry = 0;
                window_val as i64
            } else {
                carry = 1;
                (window_val as i64).wrapping_sub(width as i64)
            });
            wnaf.extend(iter::repeat(0).take(window - 1));
            pos += window;
        }
    }
}

/// Performs w-NAF exponentiation with the provided window table and w-NAF form scalar.
///
/// This function must be provided a `table` and `wnaf` that were constructed with
/// the same window size; otherwise, it may panic or produce invalid results.
pub(crate) fn wnaf_exp<G: Group>(table: &[G], wnaf: &[i64]) -> G {
    let mut result = G::identity();

    let mut found_one = false;

    for n in wnaf.iter().rev() {
        if found_one {
            result = result.double();
        }

        if *n != 0 {
            found_one = true;

            if *n > 0 {
                result += &table[(n / 2) as usize];
            } else {
                result -= &table[((-n) / 2) as usize];
            }
        }
    }

    result
}

/// A "w-ary non-adjacent form" scalar multiplication (also known as exponentiation)
/// context.
///
/// # Examples
///
/// This struct can be used to implement several patterns:
///
/// ## One base, one scalar
///
/// For this pattern, you can use a transient `Wnaf` context:
///
/// ```ignore
/// use group::Wnaf;
///
/// let result = Wnaf::new().scalar(&scalar).base(base);
/// ```
///
/// ## Many bases, one scalar
///
/// For this pattern, you create a `Wnaf` context, load the scalar into it, and then
/// process each base in turn:
///
/// ```ignore
/// use group::Wnaf;
///
/// let mut wnaf = Wnaf::new();
/// let mut wnaf_scalar = wnaf.scalar(&scalar);
/// let results: Vec<_> = bases
///     .into_iter()
///     .map(|base| wnaf_scalar.base(base))
///     .collect();
/// ```
///
/// ## One base, many scalars
///
/// For this pattern, you create a `Wnaf` context, load the base into it, and then process
/// each scalar in turn:
///
/// ```ignore
/// use group::Wnaf;
///
/// let mut wnaf = Wnaf::new();
/// let mut wnaf_base = wnaf.base(base, scalars.len());
/// let results: Vec<_> = scalars
///     .iter()
///     .map(|scalar| wnaf_base.scalar(scalar))
///     .collect();
/// ```
///
/// ## Many bases, many scalars
///
/// Say you have `n` bases and `m` scalars, and want to produce `n * m` results. For this
/// pattern, you need to cache the w-NAF tables for the bases and then compute the w-NAF
/// form of the scalars on the fly for every base, or vice versa:
///
/// ```ignore
/// use group::Wnaf;
///
/// let mut wnaf_contexts: Vec<_> = (0..bases.len()).map(|_| Wnaf::new()).collect();
/// let mut wnaf_bases: Vec<_> = wnaf_contexts
///     .iter_mut()
///     .zip(bases)
///     .map(|(wnaf, base)| wnaf.base(base, scalars.len()))
///     .collect();
/// let results: Vec<_> = wnaf_bases
///     .iter()
///     .flat_map(|wnaf_base| scalars.iter().map(|scalar| wnaf_base.scalar(scalar)))
///     .collect();
/// ```
///
/// Alternatively, use the [`WnafBase`] and [`WnafScalar`] types, which enable the various
/// tables and w-NAF forms to be cached individually per base and scalar. These types can
/// then be directly multiplied without any additional runtime work, at the cost of fixing
/// a specific window size (rather than choosing the window size dynamically).
#[derive(Debug)]
pub struct Wnaf<W, B, S> {
    base: B,
    scalar: S,
    window_size: W,
}

impl<G: Group> Wnaf<(), Vec<G>, Vec<i64>> {
    /// Construct a new wNAF context without allocating.
    pub fn new() -> Self {
        Wnaf {
            base: vec![],
            scalar: vec![],
            window_size: (),
        }
    }
}

#[cfg(feature = "wnaf-memuse")]
impl<G: Group + memuse::DynamicUsage> memuse::DynamicUsage for Wnaf<(), Vec<G>, Vec<i64>> {
    fn dynamic_usage(&self) -> usize {
        self.base.dynamic_usage() + self.scalar.dynamic_usage()
    }

    fn dynamic_usage_bounds(&self) -> (usize, Option<usize>) {
        let (base_lower, base_upper) = self.base.dynamic_usage_bounds();
        let (scalar_lower, scalar_upper) = self.scalar.dynamic_usage_bounds();

        (
            base_lower + scalar_lower,
            base_upper.zip(scalar_upper).map(|(a, b)| a + b),
        )
    }
}

impl<G: WnafGroup> Wnaf<(), Vec<G>, Vec<i64>> {
    /// Given a base and a number of scalars, compute a window table and return a `Wnaf` object that
    /// can perform exponentiations with `.scalar(..)`.
    pub fn base(&mut self, base: G, num_scalars: usize) -> Wnaf<usize, &[G], &mut Vec<i64>> {
        // Compute the appropriate window size based on the number of scalars.
        let window_size = G::recommended_wnaf_for_num_scalars(num_scalars);

        // Compute a wNAF table for the provided base and window size.
        wnaf_table(&mut self.base, base, window_size);

        // Return a Wnaf object that immutably borrows the computed base storage location,
        // but mutably borrows the scalar storage location.
        Wnaf {
            base: &self.base[..],
            scalar: &mut self.scalar,
            window_size,
        }
    }

    /// Given a scalar, compute its wNAF representation and return a `Wnaf` object that can perform
    /// exponentiations with `.base(..)`.
    pub fn scalar(&mut self, scalar: &<G as Group>::Scalar) -> Wnaf<usize, &mut Vec<G>, &[i64]> {
        // We hard-code a window size of 4.
        let window_size = 4;

        // Compute the wNAF form of the scalar.
        wnaf_form(&mut self.scalar, scalar.to_repr(), window_size);

        // Return a Wnaf object that mutably borrows the base storage location, but
        // immutably borrows the computed wNAF form scalar location.
        Wnaf {
            base: &mut self.base,
            scalar: &self.scalar[..],
            window_size,
        }
    }
}

impl<'a, G: Group> Wnaf<usize, &'a [G], &'a mut Vec<i64>> {
    /// Constructs new space for the scalar representation while borrowing
    /// the computed window table, for sending the window table across threads.
    pub fn shared(&self) -> Wnaf<usize, &'a [G], Vec<i64>> {
        Wnaf {
            base: self.base,
            scalar: vec![],
            window_size: self.window_size,
        }
    }
}

#[cfg(feature = "wnaf-memuse")]
impl<'a, G: Group> memuse::DynamicUsage for Wnaf<usize, &'a [G], Vec<i64>> {
    fn dynamic_usage(&self) -> usize {
        // The heap memory for the window table is counted in the parent `Wnaf`.
        self.scalar.dynamic_usage()
    }

    fn dynamic_usage_bounds(&self) -> (usize, Option<usize>) {
        self.scalar.dynamic_usage_bounds()
    }
}

impl<'a, G: Group> Wnaf<usize, &'a mut Vec<G>, &'a [i64]> {
    /// Constructs new space for the window table while borrowing
    /// the computed scalar representation, for sending the scalar representation
    /// across threads.
    pub fn shared(&self) -> Wnaf<usize, Vec<G>, &'a [i64]> {
        Wnaf {
            base: vec![],
            scalar: self.scalar,
            window_size: self.window_size,
        }
    }
}

#[cfg(feature = "wnaf-memuse")]
impl<'a, G: Group + memuse::DynamicUsage> memuse::DynamicUsage for Wnaf<usize, Vec<G>, &'a [i64]> {
    fn dynamic_usage(&self) -> usize {
        // The heap memory for the scalar representation is counted in the parent `Wnaf`.
        self.base.dynamic_usage()
    }

    fn dynamic_usage_bounds(&self) -> (usize, Option<usize>) {
        self.base.dynamic_usage_bounds()
    }
}

impl<B, S: AsRef<[i64]>> Wnaf<usize, B, S> {
    /// Performs exponentiation given a base.
    pub fn base<G: Group>(&mut self, base: G) -> G
    where
        B: AsMut<Vec<G>>,
    {
        wnaf_table(self.base.as_mut(), base, self.window_size);
        wnaf_exp(self.base.as_mut(), self.scalar.as_ref())
    }
}

impl<B, S: AsMut<Vec<i64>>> Wnaf<usize, B, S> {
    /// Performs exponentiation given a scalar.
    pub fn scalar<G: Group>(&mut self, scalar: &<G as Group>::Scalar) -> G
    where
        B: AsRef<[G]>,
    {
        wnaf_form(self.scalar.as_mut(), scalar.to_repr(), self.window_size);
        wnaf_exp(self.base.as_ref(), self.scalar.as_mut())
    }
}

/// A "w-ary non-adjacent form" scalar, that uses precomputation to improve the speed of
/// scalar multiplication.
///
/// # Examples
///
/// See [`WnafBase`] for usage examples.
#[derive(Clone, Debug)]
pub struct WnafScalar<F: PrimeField, const WINDOW_SIZE: usize> {
    wnaf: Vec<i64>,
    field: PhantomData<F>,
}

#[cfg(feature = "wnaf-memuse")]
impl<F: PrimeField, const WINDOW_SIZE: usize> memuse::DynamicUsage for WnafScalar<F, WINDOW_SIZE> {
    fn dynamic_usage(&self) -> usize {
        self.wnaf.dynamic_usage()
    }

    fn dynamic_usage_bounds(&self) -> (usize, Option<usize>) {
        self.wnaf.dynamic_usage_bounds()
    }
}

impl<F: PrimeField, const WINDOW_SIZE: usize> WnafScalar<F, WINDOW_SIZE> {
    /// Computes the w-NAF representation of the given scalar with the specified
    /// `WINDOW_SIZE`.
    pub fn new(scalar: &F) -> Self {
        let mut wnaf = vec![];

        // Compute the w-NAF form of the scalar.
        wnaf_form(&mut wnaf, scalar.to_repr(), WINDOW_SIZE);

        WnafScalar {
            wnaf,
            field: PhantomData::default(),
        }
    }
}

/// A fixed window table for a group element, precomputed to improve the speed of scalar
/// multiplication.
///
/// This struct is designed for usage patterns that have long-term cached bases and/or
/// scalars, or [Cartesian products] of bases and scalars. The [`Wnaf`] API enables one or
/// the other to be cached, but requires either the base window tables or the scalar w-NAF
/// forms to be computed repeatedly on the fly, which can become a significant performance
/// issue for some use cases.
///
/// `WnafBase` and [`WnafScalar`] enable an alternative trade-off: by fixing the window
/// size at compile time, the precomputations are guaranteed to only occur once per base
/// and once per scalar. Users should select their window size based on how long the bases
/// are expected to live; a larger window size will consume more memory and take longer to
/// precompute, but result in faster scalar multiplications.
///
/// [Cartesian products]: https://en.wikipedia.org/wiki/Cartesian_product
///
/// # Examples
///
/// ```ignore
/// use group::{WnafBase, WnafScalar};
///
/// let wnaf_bases: Vec<_> = bases.into_iter().map(WnafBase::<_, 4>::new).collect();
/// let wnaf_scalars: Vec<_> = scalars.iter().map(WnafScalar::new).collect();
/// let results: Vec<_> = wnaf_bases
///     .iter()
///     .flat_map(|base| wnaf_scalars.iter().map(|scalar| base * scalar))
///     .collect();
/// ```
///
/// Note that this pattern requires specifying a fixed window size (unlike previous
/// patterns that picked a suitable window size internally). This is necessary to ensure
/// in the type system that the base and scalar `Wnaf`s were computed with the same window
/// size, allowing the result to be computed infallibly.
#[derive(Clone, Debug)]
pub struct WnafBase<G: Group, const WINDOW_SIZE: usize> {
    table: Vec<G>,
}

#[cfg(feature = "wnaf-memuse")]
impl<G: Group + memuse::DynamicUsage, const WINDOW_SIZE: usize> memuse::DynamicUsage
    for WnafBase<G, WINDOW_SIZE>
{
    fn dynamic_usage(&self) -> usize {
        self.table.dynamic_usage()
    }

    fn dynamic_usage_bounds(&self) -> (usize, Option<usize>) {
        self.table.dynamic_usage_bounds()
    }
}

impl<G: Group, const WINDOW_SIZE: usize> WnafBase<G, WINDOW_SIZE> {
    /// Computes a window table for the given base with the specified `WINDOW_SIZE`.
    pub fn new(base: G) -> Self {
        let mut table = vec![];

        // Compute a window table for the provided base and window size.
        wnaf_table(&mut table, base, WINDOW_SIZE);

        WnafBase { table }
    }
}

impl<G: Group, const WINDOW_SIZE: usize> Mul<&WnafScalar<G::Scalar, WINDOW_SIZE>>
    for &WnafBase<G, WINDOW_SIZE>
{
    type Output = G;

    fn mul(self, rhs: &WnafScalar<G::Scalar, WINDOW_SIZE>) -> Self::Output {
        wnaf_exp(&self.table, &rhs.wnaf)
    }
}
