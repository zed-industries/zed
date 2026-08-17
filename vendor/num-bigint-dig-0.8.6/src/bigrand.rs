//! Randomization of big integers

use rand::Rng;
use rand::distributions::uniform::{SampleBorrow, SampleUniform, UniformSampler};
use rand::prelude::*;

use crate::BigInt;
use crate::BigUint;
use crate::Sign::*;

use crate::big_digit::BigDigit;
use crate::bigint::{into_magnitude, magnitude};
use crate::integer::Integer;
#[cfg(feature = "prime")]
use num_iter::range_step;
use num_traits::Zero;
#[cfg(feature = "prime")]
use num_traits::{FromPrimitive, ToPrimitive};

#[cfg(feature = "prime")]
use crate::prime::probably_prime;

pub trait RandBigInt {
    /// Generate a random `BigUint` of the given bit size.
    fn gen_biguint(&mut self, bit_size: usize) -> BigUint;

    /// Generate a random BigInt of the given bit size.
    fn gen_bigint(&mut self, bit_size: usize) -> BigInt;

    /// Generate a random `BigUint` less than the given bound. Fails
    /// when the bound is zero.
    fn gen_biguint_below(&mut self, bound: &BigUint) -> BigUint;

    /// Generate a random `BigUint` within the given range. The lower
    /// bound is inclusive; the upper bound is exclusive. Fails when
    /// the upper bound is not greater than the lower bound.
    fn gen_biguint_range(&mut self, lbound: &BigUint, ubound: &BigUint) -> BigUint;

    /// Generate a random `BigInt` within the given range. The lower
    /// bound is inclusive; the upper bound is exclusive. Fails when
    /// the upper bound is not greater than the lower bound.
    fn gen_bigint_range(&mut self, lbound: &BigInt, ubound: &BigInt) -> BigInt;
}

impl<R: Rng + ?Sized> RandBigInt for R {
    fn gen_biguint(&mut self, bit_size: usize) -> BigUint {
        use super::big_digit::BITS;
        let (digits, rem) = bit_size.div_rem(&BITS);
        let mut data = smallvec![BigDigit::default(); digits + (rem > 0) as usize];

        // `fill` is faster than many `gen::<u32>` calls
        // Internally this calls `SeedableRng` where implementors are responsible for adjusting endianness for reproducable values.
        self.fill(data.as_mut_slice());

        if rem > 0 {
            data[digits] >>= BITS - rem;
        }
        BigUint::new_native(data)
    }

    fn gen_bigint(&mut self, bit_size: usize) -> BigInt {
        loop {
            // Generate a random BigUint...
            let biguint = self.gen_biguint(bit_size);
            // ...and then randomly assign it a Sign...
            let sign = if biguint.is_zero() {
                // ...except that if the BigUint is zero, we need to try
                // again with probability 0.5. This is because otherwise,
                // the probability of generating a zero BigInt would be
                // double that of any other number.
                if self.r#gen() {
                    continue;
                } else {
                    NoSign
                }
            } else if self.r#gen() {
                Plus
            } else {
                Minus
            };
            return BigInt::from_biguint(sign, biguint);
        }
    }

    fn gen_biguint_below(&mut self, bound: &BigUint) -> BigUint {
        assert!(!bound.is_zero());
        let bits = bound.bits();
        loop {
            let n = self.gen_biguint(bits);
            if n < *bound {
                return n;
            }
        }
    }

    fn gen_biguint_range(&mut self, lbound: &BigUint, ubound: &BigUint) -> BigUint {
        assert!(*lbound < *ubound);
        if lbound.is_zero() {
            self.gen_biguint_below(ubound)
        } else {
            lbound + self.gen_biguint_below(&(ubound - lbound))
        }
    }

    fn gen_bigint_range(&mut self, lbound: &BigInt, ubound: &BigInt) -> BigInt {
        assert!(*lbound < *ubound);
        if lbound.is_zero() {
            BigInt::from(self.gen_biguint_below(magnitude(&ubound)))
        } else if ubound.is_zero() {
            lbound + BigInt::from(self.gen_biguint_below(magnitude(&lbound)))
        } else {
            let delta = ubound - lbound;
            lbound + BigInt::from(self.gen_biguint_below(magnitude(&delta)))
        }
    }
}

/// The back-end implementing rand's `UniformSampler` for `BigUint`.
#[derive(Clone, Debug)]
pub struct UniformBigUint {
    base: BigUint,
    len: BigUint,
}

impl UniformSampler for UniformBigUint {
    type X = BigUint;

    #[inline]
    fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = low_b.borrow();
        let high = high_b.borrow();

        assert!(low < high);

        UniformBigUint {
            len: high - low,
            base: low.clone(),
        }
    }

    #[inline]
    fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        Self::new(low_b, high_b.borrow() + 1u32)
    }

    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
        &self.base + rng.gen_biguint_below(&self.len)
    }

    #[inline]
    fn sample_single<R: Rng + ?Sized, B1, B2>(low_b: B1, high_b: B2, rng: &mut R) -> Self::X
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = low_b.borrow();
        let high = high_b.borrow();

        rng.gen_biguint_range(low, high)
    }
}

impl SampleUniform for BigUint {
    type Sampler = UniformBigUint;
}

/// The back-end implementing rand's `UniformSampler` for `BigInt`.
#[derive(Clone, Debug)]
pub struct UniformBigInt {
    base: BigInt,
    len: BigUint,
}

impl UniformSampler for UniformBigInt {
    type X = BigInt;

    #[inline]
    fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = low_b.borrow();
        let high = high_b.borrow();

        assert!(low < high);
        UniformBigInt {
            len: into_magnitude(high - low),
            base: low.clone(),
        }
    }

    #[inline]
    fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = low_b.borrow();
        let high = high_b.borrow();

        assert!(low <= high);
        Self::new(low, high + 1u32)
    }

    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
        &self.base + BigInt::from(rng.gen_biguint_below(&self.len))
    }

    #[inline]
    fn sample_single<R: Rng + ?Sized, B1, B2>(low_b: B1, high_b: B2, rng: &mut R) -> Self::X
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = low_b.borrow();
        let high = high_b.borrow();

        rng.gen_bigint_range(low, high)
    }
}

impl SampleUniform for BigInt {
    type Sampler = UniformBigInt;
}

/// A random distribution for `BigUint` and `BigInt` values of a particular bit size.
#[derive(Clone, Copy, Debug)]
pub struct RandomBits {
    bits: usize,
}

impl RandomBits {
    #[inline]
    pub fn new(bits: usize) -> RandomBits {
        RandomBits { bits }
    }
}

impl Distribution<BigUint> for RandomBits {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BigUint {
        rng.gen_biguint(self.bits)
    }
}

impl Distribution<BigInt> for RandomBits {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BigInt {
        rng.gen_bigint(self.bits)
    }
}

/// A generic trait for generating random primes.
///
/// *Warning*: This is highly dependend on the provided random number generator,
/// to provide actually random primes.
///
/// # Example
#[cfg_attr(feature = "std", doc = " ```")]
#[cfg_attr(not(feature = "std"), doc = " ```ignore")]
/// extern crate rand;
/// extern crate num_bigint_dig as num_bigint;
///
/// use rand::thread_rng;
/// use num_bigint::RandPrime;
///
/// let mut rng = thread_rng();
/// let p = rng.gen_prime(1024);
/// assert_eq!(p.bits(), 1024);
/// ```
///
#[cfg(feature = "prime")]
pub trait RandPrime {
    /// Generate a random prime number with as many bits as given.
    fn gen_prime(&mut self, bits: usize) -> BigUint;
}

/// A list of small, prime numbers that allows us to rapidly
/// exclude some fraction of composite candidates when searching for a random
/// prime. This list is truncated at the point where smallPrimesProduct exceeds
/// a u64. It does not include two because we ensure that the candidates are
/// odd by construction.
#[cfg(feature = "prime")]
const SMALL_PRIMES: [u8; 15] = [3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53];

#[cfg(feature = "prime")]
lazy_static! {
    /// The product of the values in SMALL_PRIMES and allows us
    /// to reduce a candidate prime by this number and then determine whether it's
    /// coprime to all the elements of SMALL_PRIMES without further BigUint
    /// operations.
    static ref SMALL_PRIMES_PRODUCT: BigUint = BigUint::from_u64(16_294_579_238_595_022_365).unwrap();
}

#[cfg(feature = "prime")]
impl<R: Rng + ?Sized> RandPrime for R {
    fn gen_prime(&mut self, bit_size: usize) -> BigUint {
        if bit_size < 2 {
            panic!("prime size must be at least 2-bit");
        }

        let mut b = bit_size % 8;
        if b == 0 {
            b = 8;
        }

        let bytes_len = (bit_size + 7) / 8;
        let mut bytes = alloc::vec![0u8; bytes_len];

        loop {
            self.fill_bytes(&mut bytes);
            // Clear bits in the first byte to make sure the candidate has a size <= bits.
            bytes[0] &= ((1u32 << (b as u32)) - 1) as u8;

            // Don't let the value be too small, i.e, set the most significant two bits.
            // Setting the top two bits, rather than just the top bit,
            // means that when two of these values are multiplied together,
            // the result isn't ever one bit short.
            if b >= 2 {
                bytes[0] |= 3u8.wrapping_shl(b as u32 - 2);
            } else {
                // Here b==1, because b cannot be zero.
                bytes[0] |= 1;
                if bytes_len > 1 {
                    bytes[1] |= 0x80;
                }
            }

            // Make the value odd since an even number this large certainly isn't prime.
            bytes[bytes_len - 1] |= 1u8;

            let mut p = BigUint::from_bytes_be(&bytes);
            // must always be a u64, as the SMALL_PRIMES_PRODUCT is a u64
            let rem = (&p % &*SMALL_PRIMES_PRODUCT).to_u64().unwrap();

            'next: for delta in range_step(0, 1 << 20, 2) {
                let m = rem + delta;

                for prime in &SMALL_PRIMES {
                    if m % u64::from(*prime) == 0 && (bit_size > 6 || m != u64::from(*prime)) {
                        continue 'next;
                    }
                }

                if delta > 0 {
                    p += BigUint::from_u64(delta).unwrap();
                }

                break;
            }

            // There is a tiny possibility that, by adding delta, we caused
            // the number to be one bit too long. Thus we check bit length here.
            if p.bits() == bit_size && probably_prime(&p, 20) {
                return p;
            }
        }
    }
}
