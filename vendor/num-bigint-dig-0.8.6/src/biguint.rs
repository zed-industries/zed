#[allow(deprecated, unused_imports)]
use alloc::borrow::Cow;
use alloc::string::String;
use alloc::{vec, vec::Vec};
use core::cmp::Ordering::{self, Equal, Greater, Less};
use core::default::Default;
use core::hash::{Hash, Hasher};
use core::iter::{Product, Sum};
use core::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Neg, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};
use core::str::{self, FromStr};
use core::{cmp, fmt, mem};
use core::{f32, f64};
use core::{u32, u64, u8};

#[cfg(feature = "serde")]
use serde;

#[cfg(feature = "zeroize")]
use zeroize::Zeroize;

#[cfg(feature = "std")]
fn sqrt(a: f64) -> f64 {
    a.sqrt()
}

#[cfg(not(feature = "std"))]
fn sqrt(a: f64) -> f64 {
    libm::sqrt(a)
}

#[cfg(feature = "std")]
fn ln(a: f64) -> f64 {
    a.ln()
}

#[cfg(not(feature = "std"))]
fn ln(a: f64) -> f64 {
    libm::log(a)
}

#[cfg(feature = "std")]
fn cbrt(a: f64) -> f64 {
    a.cbrt()
}

#[cfg(not(feature = "std"))]
fn cbrt(a: f64) -> f64 {
    libm::cbrt(a)
}

#[cfg(feature = "std")]
fn exp(a: f64) -> f64 {
    a.exp()
}

#[cfg(not(feature = "std"))]
fn exp(a: f64) -> f64 {
    libm::exp(a)
}

use crate::integer::{Integer, Roots};
use num_traits::float::FloatCore;
use num_traits::{
    CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, FromPrimitive, Num, One, Pow, ToPrimitive,
    Unsigned, Zero,
};

use crate::BigInt;

use crate::big_digit::{self, BigDigit};

use smallvec::SmallVec;

#[path = "monty.rs"]
mod monty;

use self::monty::monty_modpow;
use super::VEC_SIZE;
use crate::algorithms::{__add2, __sub2rev, add2, sub2, sub2rev};
use crate::algorithms::{biguint_shl, biguint_shr};
use crate::algorithms::{cmp_slice, fls, idiv_ceil, ilog2};
use crate::algorithms::{div_rem, div_rem_digit, mac_with_carry, mul3, scalar_mul};
use crate::algorithms::{extended_gcd, mod_inverse};
use crate::traits::{ExtendedGcd, ModInverse};

use crate::ParseBigIntError;
use crate::UsizePromotion;

/// A big unsigned integer type.
#[derive(Clone, Debug)]
pub struct BigUint {
    pub(crate) data: SmallVec<[BigDigit; VEC_SIZE]>,
}

impl PartialEq for BigUint {
    #[inline]
    fn eq(&self, other: &BigUint) -> bool {
        match self.cmp(other) {
            Equal => true,
            _ => false,
        }
    }
}
impl Eq for BigUint {}

impl Hash for BigUint {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data.hash(state);
    }
}

impl PartialOrd for BigUint {
    #[inline]
    fn partial_cmp(&self, other: &BigUint) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BigUint {
    #[inline]
    fn cmp(&self, other: &BigUint) -> Ordering {
        cmp_slice(&self.data[..], &other.data[..])
    }
}

impl Default for BigUint {
    #[inline]
    fn default() -> BigUint {
        Zero::zero()
    }
}

#[cfg(feature = "zeroize")]
impl Zeroize for BigUint {
    fn zeroize(&mut self) {
        self.data.as_mut().zeroize();
    }
}

impl fmt::Display for BigUint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad_integral(true, "", &self.to_str_radix(10))
    }
}

impl fmt::LowerHex for BigUint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad_integral(true, "0x", &self.to_str_radix(16))
    }
}

impl fmt::UpperHex for BigUint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = self.to_str_radix(16);
        s.make_ascii_uppercase();
        f.pad_integral(true, "0x", &s)
    }
}

impl fmt::Binary for BigUint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad_integral(true, "0b", &self.to_str_radix(2))
    }
}

impl fmt::Octal for BigUint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad_integral(true, "0o", &self.to_str_radix(8))
    }
}

impl FromStr for BigUint {
    type Err = ParseBigIntError;

    #[inline]
    fn from_str(s: &str) -> Result<BigUint, ParseBigIntError> {
        BigUint::from_str_radix(s, 10)
    }
}

// Convert from a power of two radix (bits == ilog2(radix)) where bits evenly divides
// BigDigit::BITS
fn from_bitwise_digits_le(v: &[u8], bits: usize) -> BigUint {
    debug_assert!(!v.is_empty() && bits <= 8 && big_digit::BITS % bits == 0);
    debug_assert!(v.iter().all(|&c| (c as BigDigit) < (1 << bits)));

    let digits_per_big_digit = big_digit::BITS / bits;

    let data = v
        .chunks(digits_per_big_digit)
        .map(|chunk| {
            chunk
                .iter()
                .rev()
                .fold(0, |acc, &c| (acc << bits) | c as BigDigit)
        })
        .collect();

    BigUint::new_native(data)
}

// Convert from a power of two radix (bits == ilog2(radix)) where bits doesn't evenly divide
// BigDigit::BITS
fn from_inexact_bitwise_digits_le(v: &[u8], bits: usize) -> BigUint {
    debug_assert!(!v.is_empty() && bits <= 8 && big_digit::BITS % bits != 0);
    debug_assert!(v.iter().all(|&c| (c as BigDigit) < (1 << bits)));

    let big_digits = (v.len() * bits + big_digit::BITS - 1) / big_digit::BITS;
    let mut data = SmallVec::with_capacity(big_digits);

    let mut d = 0;
    let mut dbits = 0; // number of bits we currently have in d

    // walk v accumululating bits in d; whenever we accumulate big_digit::BITS in d, spit out a
    // big_digit:
    for &c in v {
        d |= (c as BigDigit) << dbits;
        dbits += bits;

        if dbits >= big_digit::BITS {
            data.push(d);
            dbits -= big_digit::BITS;
            // if dbits was > big_digit::BITS, we dropped some of the bits in c (they couldn't fit
            // in d) - grab the bits we lost here:
            d = (c as BigDigit) >> (bits - dbits);
        }
    }

    if dbits > 0 {
        debug_assert!(dbits < big_digit::BITS);
        data.push(d as BigDigit);
    }

    BigUint::new_native(data)
}

// Read little-endian radix digits
fn from_radix_digits_be(v: &[u8], radix: u32) -> BigUint {
    debug_assert!(!v.is_empty() && !radix.is_power_of_two());
    debug_assert!(v.iter().all(|&c| (c as u32) < radix));

    // Estimate how big the result will be, so we can pre-allocate it.
    let bits = ilog2(radix) * v.len();
    let big_digits = idiv_ceil(bits, big_digit::BITS);
    let mut data = SmallVec::with_capacity(big_digits);

    let (base, power) = get_radix_base(radix);
    let radix = radix as BigDigit;

    let r = v.len() % power;
    let i = if r == 0 { power } else { r };
    let (head, tail) = v.split_at(i);

    let first = head.iter().fold(0, |acc, &d| acc * radix + d as BigDigit);
    data.push(first);

    debug_assert!(tail.len() % power == 0);
    for chunk in tail.chunks(power) {
        if data.last() != Some(&0) {
            data.push(0);
        }

        let mut carry = 0;
        for d in data.iter_mut() {
            *d = mac_with_carry(0, *d, base, &mut carry);
        }
        debug_assert!(carry == 0);

        let n = chunk.iter().fold(0, |acc, &d| acc * radix + d as BigDigit);
        add2(&mut data, &[n]);
    }

    BigUint::new_native(data)
}

impl Num for BigUint {
    type FromStrRadixErr = ParseBigIntError;

    /// Creates and initializes a `BigUint`.
    fn from_str_radix(s: &str, radix: u32) -> Result<BigUint, ParseBigIntError> {
        assert!(2 <= radix && radix <= 36, "The radix must be within 2...36");
        let mut s = s;
        if s.starts_with('+') {
            let tail = &s[1..];
            if !tail.starts_with('+') {
                s = tail
            }
        }

        if s.is_empty() {
            return Err(ParseBigIntError::empty());
        }

        if s.starts_with('_') {
            // Must lead with a real digit!
            return Err(ParseBigIntError::invalid());
        }

        // First normalize all characters to plain digit values
        let mut v = Vec::with_capacity(s.len());
        for b in s.bytes() {
            let d = match b {
                b'0'..=b'9' => b - b'0',
                b'a'..=b'z' => b - b'a' + 10,
                b'A'..=b'Z' => b - b'A' + 10,
                b'_' => continue,
                _ => u8::MAX,
            };
            if d < radix as u8 {
                v.push(d);
            } else {
                return Err(ParseBigIntError::invalid());
            }
        }

        let res = if radix.is_power_of_two() {
            // Powers of two can use bitwise masks and shifting instead of multiplication
            let bits = ilog2(radix);
            v.reverse();
            if big_digit::BITS % bits == 0 {
                from_bitwise_digits_le(&v, bits)
            } else {
                from_inexact_bitwise_digits_le(&v, bits)
            }
        } else {
            from_radix_digits_be(&v, radix)
        };
        Ok(res)
    }
}

forward_val_val_binop!(impl BitAnd for BigUint, bitand);
forward_ref_val_binop!(impl BitAnd for BigUint, bitand);

// do not use forward_ref_ref_binop_commutative! for bitand so that we can
// clone the smaller value rather than the larger, avoiding over-allocation
impl<'a, 'b> BitAnd<&'b BigUint> for &'a BigUint {
    type Output = BigUint;

    #[inline]
    fn bitand(self, other: &BigUint) -> BigUint {
        // forward to val-ref, choosing the smaller to clone
        if self.data.len() <= other.data.len() {
            self.clone() & other
        } else {
            other.clone() & self
        }
    }
}

forward_val_assign!(impl BitAndAssign for BigUint, bitand_assign);

impl<'a> BitAnd<&'a BigUint> for BigUint {
    type Output = BigUint;

    #[inline]
    fn bitand(mut self, other: &BigUint) -> BigUint {
        self &= other;
        self
    }
}
impl<'a> BitAndAssign<&'a BigUint> for BigUint {
    #[inline]
    fn bitand_assign(&mut self, other: &BigUint) {
        for (ai, &bi) in self.data.iter_mut().zip(other.data.iter()) {
            *ai &= bi;
        }
        self.data.truncate(other.data.len());
        self.normalize();
    }
}

forward_all_binop_to_val_ref_commutative!(impl BitOr for BigUint, bitor);
forward_val_assign!(impl BitOrAssign for BigUint, bitor_assign);

impl<'a> BitOr<&'a BigUint> for BigUint {
    type Output = BigUint;

    fn bitor(mut self, other: &BigUint) -> BigUint {
        self |= other;
        self
    }
}
impl<'a> BitOrAssign<&'a BigUint> for BigUint {
    #[inline]
    fn bitor_assign(&mut self, other: &BigUint) {
        for (ai, &bi) in self.data.iter_mut().zip(other.data.iter()) {
            *ai |= bi;
        }
        if other.data.len() > self.data.len() {
            let extra = &other.data[self.data.len()..];
            self.data.extend(extra.iter().cloned());
        }
    }
}

forward_all_binop_to_val_ref_commutative!(impl BitXor for BigUint, bitxor);
forward_val_assign!(impl BitXorAssign for BigUint, bitxor_assign);

impl<'a> BitXor<&'a BigUint> for BigUint {
    type Output = BigUint;

    fn bitxor(mut self, other: &BigUint) -> BigUint {
        self ^= other;
        self
    }
}
impl<'a> BitXorAssign<&'a BigUint> for BigUint {
    #[inline]
    fn bitxor_assign(&mut self, other: &BigUint) {
        for (ai, &bi) in self.data.iter_mut().zip(other.data.iter()) {
            *ai ^= bi;
        }
        if other.data.len() > self.data.len() {
            let extra = &other.data[self.data.len()..];
            self.data.extend(extra.iter().cloned());
        }
        self.normalize();
    }
}

impl Shl<usize> for BigUint {
    type Output = BigUint;

    #[inline]
    fn shl(self, rhs: usize) -> BigUint {
        biguint_shl(Cow::Owned(self), rhs)
    }
}
impl<'a> Shl<usize> for &'a BigUint {
    type Output = BigUint;

    #[inline]
    fn shl(self, rhs: usize) -> BigUint {
        biguint_shl(Cow::Borrowed(self), rhs)
    }
}

impl ShlAssign<usize> for BigUint {
    #[inline]
    fn shl_assign(&mut self, rhs: usize) {
        let n = mem::replace(self, BigUint::zero());
        *self = n << rhs;
    }
}

impl Shr<usize> for BigUint {
    type Output = BigUint;

    #[inline]
    fn shr(self, rhs: usize) -> BigUint {
        biguint_shr(Cow::Owned(self), rhs)
    }
}
impl<'a> Shr<usize> for &'a BigUint {
    type Output = BigUint;

    #[inline]
    fn shr(self, rhs: usize) -> BigUint {
        biguint_shr(Cow::Borrowed(self), rhs)
    }
}

impl ShrAssign<usize> for BigUint {
    #[inline]
    fn shr_assign(&mut self, rhs: usize) {
        let n = mem::replace(self, BigUint::zero());
        *self = n >> rhs;
    }
}

impl Zero for BigUint {
    #[inline]
    fn zero() -> BigUint {
        BigUint::new(Vec::new())
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.data.is_empty()
    }
}

impl One for BigUint {
    #[inline]
    fn one() -> BigUint {
        BigUint::new(vec![1])
    }

    #[inline]
    fn is_one(&self) -> bool {
        self.data[..] == [1]
    }
}

impl Unsigned for BigUint {}

macro_rules! pow_impl {
    ($T:ty) => {
        impl<'a> Pow<$T> for &'a BigUint {
            type Output = BigUint;

            #[inline]
            fn pow(self, mut exp: $T) -> Self::Output {
                if exp == 0 {
                    return BigUint::one();
                }
                let mut base = self.clone();

                while exp & 1 == 0 {
                    base = &base * &base;
                    exp >>= 1;
                }

                if exp == 1 {
                    return base;
                }

                let mut acc = base.clone();
                while exp > 1 {
                    exp >>= 1;
                    base = &base * &base;
                    if exp & 1 == 1 {
                        acc = &acc * &base;
                    }
                }
                acc
            }
        }

        impl<'a, 'b> Pow<&'b $T> for &'a BigUint {
            type Output = BigUint;

            #[inline]
            fn pow(self, exp: &$T) -> Self::Output {
                self.pow(*exp)
            }
        }
    };
}

pow_impl!(u8);
pow_impl!(u16);
pow_impl!(u32);
pow_impl!(u64);
pow_impl!(usize);
#[cfg(has_i128)]
pow_impl!(u128);

forward_all_binop_to_val_ref_commutative!(impl Add for BigUint, add);
forward_val_assign!(impl AddAssign for BigUint, add_assign);

impl<'a> Add<&'a BigUint> for BigUint {
    type Output = BigUint;

    fn add(mut self, other: &BigUint) -> BigUint {
        self += other;
        self
    }
}
impl<'a> AddAssign<&'a BigUint> for BigUint {
    #[inline]
    fn add_assign(&mut self, other: &BigUint) {
        let self_len = self.data.len();
        let carry = if self_len < other.data.len() {
            let lo_carry = __add2(&mut self.data[..], &other.data[..self_len]);
            self.data.extend_from_slice(&other.data[self_len..]);
            __add2(&mut self.data[self_len..], &[lo_carry])
        } else {
            __add2(&mut self.data[..], &other.data[..])
        };
        if carry != 0 {
            self.data.push(carry);
        }
    }
}

promote_unsigned_scalars!(impl Add for BigUint, add);
promote_unsigned_scalars_assign!(impl AddAssign for BigUint, add_assign);
forward_all_scalar_binop_to_val_val_commutative!(impl Add<u32> for BigUint, add);
forward_all_scalar_binop_to_val_val_commutative!(impl Add<u64> for BigUint, add);
#[cfg(has_i128)]
forward_all_scalar_binop_to_val_val_commutative!(impl Add<u128> for BigUint, add);

impl Add<u32> for BigUint {
    type Output = BigUint;

    #[inline]
    fn add(mut self, other: u32) -> BigUint {
        self += other;
        self
    }
}

impl AddAssign<u32> for BigUint {
    #[inline]
    fn add_assign(&mut self, other: u32) {
        if other != 0 {
            if self.data.len() == 0 {
                self.data.push(0);
            }

            let carry = __add2(&mut self.data, &[other as BigDigit]);
            if carry != 0 {
                self.data.push(carry);
            }
        }
    }
}

impl Add<u64> for BigUint {
    type Output = BigUint;

    #[inline]
    fn add(mut self, other: u64) -> BigUint {
        self += other;
        self
    }
}

impl AddAssign<u64> for BigUint {
    #[cfg(not(feature = "u64_digit"))]
    #[inline]
    fn add_assign(&mut self, other: u64) {
        let (hi, lo) = big_digit::from_doublebigdigit(other);
        if hi == 0 {
            *self += lo;
        } else {
            while self.data.len() < 2 {
                self.data.push(0);
            }

            let carry = __add2(&mut self.data, &[lo, hi]);
            if carry != 0 {
                self.data.push(carry);
            }
        }
    }

    #[cfg(feature = "u64_digit")]
    #[inline]
    fn add_assign(&mut self, other: u64) {
        if other != 0 {
            if self.data.len() == 0 {
                self.data.push(0);
            }

            let carry = __add2(&mut self.data, &[other as BigDigit]);
            if carry != 0 {
                self.data.push(carry);
            }
        }
    }
}

#[cfg(has_i128)]
impl Add<u128> for BigUint {
    type Output = BigUint;

    #[inline]
    fn add(mut self, other: u128) -> BigUint {
        self += other;
        self
    }
}

#[cfg(has_i128)]
impl AddAssign<u128> for BigUint {
    #[cfg(not(feature = "u64_digit"))]
    #[inline]
    fn add_assign(&mut self, other: u128) {
        if other <= u64::max_value() as u128 {
            *self += other as u64
        } else {
            let (a, b, c, d) = u32_from_u128(other);
            let carry = if a > 0 {
                while self.data.len() < 4 {
                    self.data.push(0);
                }
                __add2(&mut self.data, &[d, c, b, a])
            } else {
                debug_assert!(b > 0);
                while self.data.len() < 3 {
                    self.data.push(0);
                }
                __add2(&mut self.data, &[d, c, b])
            };

            if carry != 0 {
                self.data.push(carry);
            }
        }
    }

    #[cfg(feature = "u64_digit")]
    #[inline]
    fn add_assign(&mut self, other: u128) {
        let (hi, lo) = big_digit::from_doublebigdigit(other);
        if hi == 0 {
            *self += lo;
        } else {
            while self.data.len() < 2 {
                self.data.push(0);
            }

            let carry = __add2(&mut self.data, &[lo, hi]);
            if carry != 0 {
                self.data.push(carry);
            }
        }
    }
}

forward_val_val_binop!(impl Sub for BigUint, sub);
forward_ref_ref_binop!(impl Sub for BigUint, sub);
forward_val_assign!(impl SubAssign for BigUint, sub_assign);

impl<'a> Sub<&'a BigUint> for BigUint {
    type Output = BigUint;

    fn sub(mut self, other: &BigUint) -> BigUint {
        self -= other;
        self
    }
}
impl<'a> SubAssign<&'a BigUint> for BigUint {
    fn sub_assign(&mut self, other: &'a BigUint) {
        sub2(&mut self.data[..], &other.data[..]);
        self.normalize();
    }
}

impl<'a> Sub<BigUint> for &'a BigUint {
    type Output = BigUint;

    fn sub(self, mut other: BigUint) -> BigUint {
        let other_len = other.data.len();
        if other_len < self.data.len() {
            let lo_borrow = __sub2rev(&self.data[..other_len], &mut other.data);
            other.data.extend_from_slice(&self.data[other_len..]);
            if lo_borrow != 0 {
                sub2(&mut other.data[other_len..], &[1])
            }
        } else {
            sub2rev(&self.data[..], &mut other.data[..]);
        }
        other.normalized()
    }
}

promote_unsigned_scalars!(impl Sub for BigUint, sub);
promote_unsigned_scalars_assign!(impl SubAssign for BigUint, sub_assign);
forward_all_scalar_binop_to_val_val!(impl Sub<u32> for BigUint, sub);
forward_all_scalar_binop_to_val_val!(impl Sub<u64> for BigUint, sub);
#[cfg(has_i128)]
forward_all_scalar_binop_to_val_val!(impl Sub<u128> for BigUint, sub);

impl Sub<u32> for BigUint {
    type Output = BigUint;

    #[inline]
    fn sub(mut self, other: u32) -> BigUint {
        self -= other;
        self
    }
}
impl SubAssign<u32> for BigUint {
    fn sub_assign(&mut self, other: u32) {
        sub2(&mut self.data[..], &[other as BigDigit]);
        self.normalize();
    }
}

impl Sub<BigUint> for u32 {
    type Output = BigUint;

    #[cfg(not(feature = "u64_digit"))]
    #[inline]
    fn sub(self, mut other: BigUint) -> BigUint {
        if other.data.len() == 0 {
            other.data.push(self);
        } else {
            sub2rev(&[self], &mut other.data[..]);
        }
        other.normalized()
    }

    #[cfg(feature = "u64_digit")]
    #[inline]
    fn sub(self, mut other: BigUint) -> BigUint {
        if other.data.len() == 0 {
            other.data.push(self as BigDigit);
        } else {
            sub2rev(&[self as BigDigit], &mut other.data[..]);
        }
        other.normalized()
    }
}

impl Sub<u64> for BigUint {
    type Output = BigUint;

    #[inline]
    fn sub(mut self, other: u64) -> BigUint {
        self -= other;
        self
    }
}

impl SubAssign<u64> for BigUint {
    #[cfg(not(feature = "u64_digit"))]
    #[inline]
    fn sub_assign(&mut self, other: u64) {
        let (hi, lo) = big_digit::from_doublebigdigit(other);
        sub2(&mut self.data[..], &[lo, hi]);
        self.normalize();
    }

    #[cfg(feature = "u64_digit")]
    #[inline]
    fn sub_assign(&mut self, other: u64) {
        sub2(&mut self.data[..], &[other as BigDigit]);
        self.normalize();
    }
}

impl Sub<BigUint> for u64 {
    type Output = BigUint;

    #[cfg(not(feature = "u64_digit"))]
    #[inline]
    fn sub(self, mut other: BigUint) -> BigUint {
        while other.data.len() < 2 {
            other.data.push(0);
        }

        let (hi, lo) = big_digit::from_doublebigdigit(self);
        sub2rev(&[lo, hi], &mut other.data[..]);
        other.normalized()
    }

    #[cfg(feature = "u64_digit")]
    #[inline]
    fn sub(self, mut other: BigUint) -> BigUint {
        if other.data.len() == 0 {
            other.data.push(self);
        } else {
            sub2rev(&[self], &mut other.data[..]);
        }
        other.normalized()
    }
}

#[cfg(has_i128)]
impl Sub<u128> for BigUint {
    type Output = BigUint;

    #[inline]
    fn sub(mut self, other: u128) -> BigUint {
        self -= other;
        self
    }
}
#[cfg(has_i128)]
impl SubAssign<u128> for BigUint {
    #[cfg(not(feature = "u64_digit"))]
    #[inline]
    fn sub_assign(&mut self, other: u128) {
        let (a, b, c, d) = u32_from_u128(other);
        sub2(&mut self.data[..], &[d, c, b, a]);
        self.normalize();
    }

    #[cfg(feature = "u64_digit")]
    #[inline]
    fn sub_assign(&mut self, other: u128) {
        let (hi, lo) = big_digit::from_doublebigdigit(other);
        sub2(&mut self.data[..], &[lo, hi]);
        self.normalize();
    }
}

#[cfg(has_i128)]
impl Sub<BigUint> for u128 {
    type Output = BigUint;

    #[cfg(not(feature = "u64_digit"))]
    #[inline]
    fn sub(self, mut other: BigUint) -> BigUint {
        while other.data.len() < 4 {
            other.data.push(0);
        }

        let (a, b, c, d) = u32_from_u128(self);
        sub2rev(&[d, c, b, a], &mut other.data[..]);
        other.normalized()
    }

    #[cfg(feature = "u64_digit")]
    #[inline]
    fn sub(self, mut other: BigUint) -> BigUint {
        while other.data.len() < 2 {
            other.data.push(0);
        }

        let (hi, lo) = big_digit::from_doublebigdigit(self);
        sub2rev(&[lo, hi], &mut other.data[..]);
        other.normalized()
    }
}

forward_all_binop_to_ref_ref!(impl Mul for BigUint, mul);
forward_val_assign!(impl MulAssign for BigUint, mul_assign);

impl<'a, 'b> Mul<&'b BigUint> for &'a BigUint {
    type Output = BigUint;

    #[inline]
    fn mul(self, other: &BigUint) -> BigUint {
        mul3(&self.data[..], &other.data[..])
    }
}

impl<'a, 'b> Mul<&'a BigInt> for &'b BigUint {
    type Output = BigInt;

    #[inline]
    fn mul(self, other: &BigInt) -> BigInt {
        BigInt {
            data: mul3(&self.data[..], &other.digits()[..]),
            sign: other.sign,
        }
    }
}

impl<'a> MulAssign<&'a BigUint> for BigUint {
    #[inline]
    fn mul_assign(&mut self, other: &'a BigUint) {
        *self = &*self * other
    }
}

promote_unsigned_scalars!(impl Mul for BigUint, mul);
promote_unsigned_scalars_assign!(impl MulAssign for BigUint, mul_assign);
forward_all_scalar_binop_to_val_val_commutative!(impl Mul<u32> for BigUint, mul);
forward_all_scalar_binop_to_val_val_commutative!(impl Mul<u64> for BigUint, mul);
#[cfg(has_i128)]
forward_all_scalar_binop_to_val_val_commutative!(impl Mul<u128> for BigUint, mul);

impl Mul<u32> for BigUint {
    type Output = BigUint;

    #[inline]
    fn mul(mut self, other: u32) -> BigUint {
        self *= other;
        self
    }
}
impl MulAssign<u32> for BigUint {
    #[inline]
    fn mul_assign(&mut self, other: u32) {
        if other == 0 {
            self.data.clear();
        } else {
            let carry = scalar_mul(&mut self.data[..], other as BigDigit);
            if carry != 0 {
                self.data.push(carry);
            }
        }
    }
}

impl Mul<u64> for BigUint {
    type Output = BigUint;

    #[inline]
    fn mul(mut self, other: u64) -> BigUint {
        self *= other;
        self
    }
}
impl MulAssign<u64> for BigUint {
    #[cfg(not(feature = "u64_digit"))]
    #[inline]
    fn mul_assign(&mut self, other: u64) {
        if other == 0 {
            self.data.clear();
        } else if other <= BigDigit::max_value() as u64 {
            *self *= other as BigDigit
        } else {
            let (hi, lo) = big_digit::from_doublebigdigit(other);
            *self = mul3(&self.data[..], &[lo, hi])
        }
    }

    #[cfg(feature = "u64_digit")]
    #[inline]
    fn mul_assign(&mut self, other: u64) {
        if other == 0 {
            self.data.clear();
        } else {
            let carry = scalar_mul(&mut self.data[..], other as BigDigit);
            if carry != 0 {
                self.data.push(carry);
            }
        }
    }
}

#[cfg(has_i128)]
impl Mul<u128> for BigUint {
    type Output = BigUint;

    #[inline]
    fn mul(mut self, other: u128) -> BigUint {
        self *= other;
        self
    }
}
#[cfg(has_i128)]
impl MulAssign<u128> for BigUint {
    #[cfg(not(feature = "u64_digit"))]
    #[inline]
    fn mul_assign(&mut self, other: u128) {
        if other == 0 {
            self.data.clear();
        } else if other <= BigDigit::max_value() as u128 {
            *self *= other as BigDigit
        } else {
            let (a, b, c, d) = u32_from_u128(other);
            *self = mul3(&self.data[..], &[d, c, b, a])
        }
    }

    #[cfg(feature = "u64_digit")]
    #[inline]
    fn mul_assign(&mut self, other: u128) {
        if other == 0 {
            self.data.clear();
        } else if other <= BigDigit::max_value() as u128 {
            *self *= other as BigDigit
        } else {
            let (hi, lo) = big_digit::from_doublebigdigit(other);
            *self = mul3(&self.data[..], &[lo, hi])
        }
    }
}

forward_all_binop_to_ref_ref!(impl Div for BigUint, div);
forward_val_assign!(impl DivAssign for BigUint, div_assign);

impl<'a, 'b> Div<&'b BigUint> for &'a BigUint {
    type Output = BigUint;

    #[inline]
    fn div(self, other: &BigUint) -> BigUint {
        let (q, _) = self.div_rem(other);
        q
    }
}
impl<'a> DivAssign<&'a BigUint> for BigUint {
    #[inline]
    fn div_assign(&mut self, other: &'a BigUint) {
        *self = &*self / other;
    }
}

promote_unsigned_scalars!(impl Div for BigUint, div);
promote_unsigned_scalars_assign!(impl DivAssign for BigUint, div_assign);
forward_all_scalar_binop_to_val_val!(impl Div<u32> for BigUint, div);
forward_all_scalar_binop_to_val_val!(impl Div<u64> for BigUint, div);
#[cfg(has_i128)]
forward_all_scalar_binop_to_val_val!(impl Div<u128> for BigUint, div);

impl Div<u32> for BigUint {
    type Output = BigUint;

    #[inline]
    fn div(self, other: u32) -> BigUint {
        let (q, _) = div_rem_digit(self, other as BigDigit);
        q
    }
}
impl DivAssign<u32> for BigUint {
    #[inline]
    fn div_assign(&mut self, other: u32) {
        *self = &*self / other;
    }
}

impl Div<BigUint> for u32 {
    type Output = BigUint;

    #[inline]
    fn div(self, other: BigUint) -> BigUint {
        match other.data.len() {
            0 => panic!(),
            1 => From::from(self as BigDigit / other.data[0]),
            _ => Zero::zero(),
        }
    }
}

impl Div<u64> for BigUint {
    type Output = BigUint;

    #[inline]
    fn div(self, other: u64) -> BigUint {
        let (q, _) = self.div_rem(&From::from(other));
        q
    }
}
impl DivAssign<u64> for BigUint {
    #[inline]
    fn div_assign(&mut self, other: u64) {
        *self = &*self / other;
    }
}

impl Div<BigUint> for u64 {
    type Output = BigUint;

    #[cfg(not(feature = "u64_digit"))]
    #[inline]
    fn div(self, other: BigUint) -> BigUint {
        match other.data.len() {
            0 => panic!(),
            1 => From::from(self / other.data[0] as u64),
            2 => From::from(self / big_digit::to_doublebigdigit(other.data[1], other.data[0])),
            _ => Zero::zero(),
        }
    }

    #[cfg(feature = "u64_digit")]
    #[inline]
    fn div(self, other: BigUint) -> BigUint {
        match other.data.len() {
            0 => panic!(),
            1 => From::from(self / other.data[0]),
            _ => Zero::zero(),
        }
    }
}

#[cfg(has_i128)]
impl Div<u128> for BigUint {
    type Output = BigUint;

    #[inline]
    fn div(self, other: u128) -> BigUint {
        let (q, _) = self.div_rem(&From::from(other));
        q
    }
}

#[cfg(has_i128)]
impl DivAssign<u128> for BigUint {
    #[inline]
    fn div_assign(&mut self, other: u128) {
        *self = &*self / other;
    }
}

#[cfg(has_i128)]
impl Div<BigUint> for u128 {
    type Output = BigUint;

    #[cfg(not(feature = "u64_digit"))]
    #[inline]
    fn div(self, other: BigUint) -> BigUint {
        match other.data.len() {
            0 => panic!(),
            1 => From::from(self / other.data[0] as u128),
            2 => From::from(
                self / big_digit::to_doublebigdigit(other.data[1], other.data[0]) as u128,
            ),
            3 => From::from(self / u32_to_u128(0, other.data[2], other.data[1], other.data[0])),
            4 => From::from(
                self / u32_to_u128(other.data[3], other.data[2], other.data[1], other.data[0]),
            ),
            _ => Zero::zero(),
        }
    }

    #[cfg(feature = "u64_digit")]
    #[inline]
    fn div(self, other: BigUint) -> BigUint {
        match other.data.len() {
            0 => panic!(),
            1 => From::from(self / other.data[0] as u128),
            2 => From::from(self / big_digit::to_doublebigdigit(other.data[1], other.data[0])),
            _ => Zero::zero(),
        }
    }
}

forward_all_binop_to_ref_ref!(impl Rem for BigUint, rem);
forward_val_assign!(impl RemAssign for BigUint, rem_assign);

impl<'a, 'b> Rem<&'b BigUint> for &'a BigUint {
    type Output = BigUint;

    #[inline]
    fn rem(self, other: &BigUint) -> BigUint {
        let (_, r) = self.div_rem(other);
        r
    }
}
impl<'a> RemAssign<&'a BigUint> for BigUint {
    #[inline]
    fn rem_assign(&mut self, other: &BigUint) {
        *self = &*self % other;
    }
}

promote_unsigned_scalars!(impl Rem for BigUint, rem);
promote_unsigned_scalars_assign!(impl RemAssign for BigUint, rem_assign);
forward_all_scalar_binop_to_val_val!(impl Rem<u32> for BigUint, rem);
forward_all_scalar_binop_to_val_val!(impl Rem<u64> for BigUint, rem);
#[cfg(has_i128)]
forward_all_scalar_binop_to_val_val!(impl Rem<u128> for BigUint, rem);

impl Rem<u32> for BigUint {
    type Output = BigUint;

    #[inline]
    fn rem(self, other: u32) -> BigUint {
        let (_, r) = div_rem_digit(self, other as BigDigit);
        From::from(r)
    }
}
impl RemAssign<u32> for BigUint {
    #[inline]
    fn rem_assign(&mut self, other: u32) {
        *self = &*self % other;
    }
}

impl Rem<BigUint> for u32 {
    type Output = BigUint;

    #[inline]
    fn rem(mut self, other: BigUint) -> BigUint {
        self %= other;
        From::from(self)
    }
}

macro_rules! impl_rem_assign_scalar {
    ($scalar:ty, $to_scalar:ident) => {
        forward_val_assign_scalar!(impl RemAssign for BigUint, $scalar, rem_assign);
        impl<'a> RemAssign<&'a BigUint> for $scalar {
            #[inline]
            fn rem_assign(&mut self, other: &BigUint) {
                *self = match other.$to_scalar() {
                    None => *self,
                    Some(0) => panic!(),
                    Some(v) => *self % v
                };
            }
        }
    }
}
// we can scalar %= BigUint for any scalar, including signed types
#[cfg(has_i128)]
impl_rem_assign_scalar!(u128, to_u128);
impl_rem_assign_scalar!(usize, to_usize);
impl_rem_assign_scalar!(u64, to_u64);
impl_rem_assign_scalar!(u32, to_u32);
impl_rem_assign_scalar!(u16, to_u16);
impl_rem_assign_scalar!(u8, to_u8);
#[cfg(has_i128)]
impl_rem_assign_scalar!(i128, to_i128);
impl_rem_assign_scalar!(isize, to_isize);
impl_rem_assign_scalar!(i64, to_i64);
impl_rem_assign_scalar!(i32, to_i32);
impl_rem_assign_scalar!(i16, to_i16);
impl_rem_assign_scalar!(i8, to_i8);

impl Rem<u64> for BigUint {
    type Output = BigUint;

    #[inline]
    fn rem(self, other: u64) -> BigUint {
        let (_, r) = self.div_rem(&From::from(other));
        r
    }
}
impl RemAssign<u64> for BigUint {
    #[inline]
    fn rem_assign(&mut self, other: u64) {
        *self = &*self % other;
    }
}

impl Rem<BigUint> for u64 {
    type Output = BigUint;

    #[inline]
    fn rem(mut self, other: BigUint) -> BigUint {
        self %= other;
        From::from(self)
    }
}

#[cfg(has_i128)]
impl Rem<u128> for BigUint {
    type Output = BigUint;

    #[inline]
    fn rem(self, other: u128) -> BigUint {
        let (_, r) = self.div_rem(&From::from(other));
        r
    }
}
#[cfg(has_i128)]
impl RemAssign<u128> for BigUint {
    #[inline]
    fn rem_assign(&mut self, other: u128) {
        *self = &*self % other;
    }
}

#[cfg(has_i128)]
impl Rem<BigUint> for u128 {
    type Output = BigUint;

    #[inline]
    fn rem(mut self, other: BigUint) -> BigUint {
        self %= other;
        From::from(self)
    }
}

impl Neg for BigUint {
    type Output = BigUint;

    #[inline]
    fn neg(self) -> BigUint {
        panic!()
    }
}

impl<'a> Neg for &'a BigUint {
    type Output = BigUint;

    #[inline]
    fn neg(self) -> BigUint {
        panic!()
    }
}

impl CheckedAdd for BigUint {
    #[inline]
    fn checked_add(&self, v: &BigUint) -> Option<BigUint> {
        Some(self.add(v))
    }
}

impl CheckedSub for BigUint {
    #[inline]
    fn checked_sub(&self, v: &BigUint) -> Option<BigUint> {
        match self.cmp(v) {
            Less => None,
            Equal => Some(Zero::zero()),
            Greater => Some(self.sub(v)),
        }
    }
}

impl CheckedMul for BigUint {
    #[inline]
    fn checked_mul(&self, v: &BigUint) -> Option<BigUint> {
        Some(self.mul(v))
    }
}

impl CheckedDiv for BigUint {
    #[inline]
    fn checked_div(&self, v: &BigUint) -> Option<BigUint> {
        if v.is_zero() {
            None
        } else {
            Some(self.div(v))
        }
    }
}

impl Integer for BigUint {
    #[inline]
    fn div_rem(&self, other: &BigUint) -> (BigUint, BigUint) {
        div_rem(self, other)
    }

    #[inline]
    fn div_floor(&self, other: &BigUint) -> BigUint {
        let (d, _) = div_rem(self, other);
        d
    }

    #[inline]
    fn mod_floor(&self, other: &BigUint) -> BigUint {
        let (_, m) = div_rem(self, other);
        m
    }

    #[inline]
    fn div_mod_floor(&self, other: &BigUint) -> (BigUint, BigUint) {
        div_rem(self, other)
    }

    /// Calculates the Greatest Common Divisor (GCD) of the number and `other`.
    ///
    /// The result is always positive.
    #[inline]
    fn gcd(&self, other: &Self) -> Self {
        let (res, _, _) = extended_gcd(Cow::Borrowed(self), Cow::Borrowed(other), false);
        res.into_biguint().unwrap()
    }

    /// Calculates the Lowest Common Multiple (LCM) of the number and `other`.
    #[inline]
    fn lcm(&self, other: &BigUint) -> BigUint {
        self / self.gcd(other) * other
    }

    /// Deprecated, use `is_multiple_of` instead.
    #[inline]
    fn divides(&self, other: &BigUint) -> bool {
        self.is_multiple_of(other)
    }

    /// Returns `true` if the number is a multiple of `other`.
    #[inline]
    fn is_multiple_of(&self, other: &BigUint) -> bool {
        (self % other).is_zero()
    }

    /// Returns `true` if the number is divisible by `2`.
    #[inline]
    fn is_even(&self) -> bool {
        // Considering only the last digit.
        match self.data.first() {
            Some(x) => x.is_even(),
            None => true,
        }
    }

    /// Returns `true` if the number is not divisible by `2`.
    #[inline]
    fn is_odd(&self) -> bool {
        !self.is_even()
    }
}

#[inline]
fn fixpoint<F>(mut x: BigUint, max_bits: usize, f: F) -> BigUint
where
    F: Fn(&BigUint) -> BigUint,
{
    let mut xn = f(&x);

    // If the value increased, then the initial guess must have been low.
    // Repeat until we reverse course.
    while x < xn {
        // Sometimes an increase will go way too far, especially with large
        // powers, and then take a long time to walk back.  We know an upper
        // bound based on bit size, so saturate on that.
        x = if xn.bits() > max_bits {
            BigUint::one() << max_bits
        } else {
            xn
        };
        xn = f(&x);
    }

    // Now keep repeating while the estimate is decreasing.
    while x > xn {
        x = xn;
        xn = f(&x);
    }
    x
}

impl Roots for BigUint {
    // nth_root, sqrt and cbrt use Newton's method to compute
    // principal root of a given degree for a given integer.

    // Reference:
    // Brent & Zimmermann, Modern Computer Arithmetic, v0.5.9, Algorithm 1.14
    fn nth_root(&self, n: u32) -> Self {
        assert!(n > 0, "root degree n must be at least 1");

        if self.is_zero() || self.is_one() {
            return self.clone();
        }

        match n {
            // Optimize for small n
            1 => return self.clone(),
            2 => return self.sqrt(),
            3 => return self.cbrt(),
            _ => (),
        }

        // The root of non-zero values less than 2ⁿ can only be 1.
        let bits = self.bits();
        if bits <= n as usize {
            return BigUint::one();
        }

        // If we fit in `u64`, compute the root that way.
        if let Some(x) = self.to_u64() {
            return x.nth_root(n).into();
        }

        let max_bits = bits / n as usize + 1;

        let guess = if let Some(f) = self.to_f64() {
            // We fit in `f64` (lossy), so get a better initial guess from that.
            BigUint::from_f64(exp(ln(f) / f64::from(n))).unwrap()
        } else {
            // Try to guess by scaling down such that it does fit in `f64`.
            // With some (x * 2ⁿᵏ), its nth root ≈ (ⁿ√x * 2ᵏ)
            let nsz = n as usize;
            let extra_bits = bits - (f64::MAX_EXP as usize - 1);
            let root_scale = (extra_bits + (nsz - 1)) / nsz;
            let scale = root_scale * nsz;
            if scale < bits && bits - scale > nsz {
                (self >> scale).nth_root(n) << root_scale
            } else {
                BigUint::one() << max_bits
            }
        };

        let n_min_1 = n - 1;
        fixpoint(guess, max_bits, move |s| {
            let q = self / s.pow(n_min_1);
            let t = n_min_1 * s + q;
            t / n
        })
    }

    // Reference:
    // Brent & Zimmermann, Modern Computer Arithmetic, v0.5.9, Algorithm 1.13
    fn sqrt(&self) -> Self {
        if self.is_zero() || self.is_one() {
            return self.clone();
        }

        // If we fit in `u64`, compute the root that way.
        if let Some(x) = self.to_u64() {
            return x.sqrt().into();
        }

        let bits = self.bits();
        let max_bits = bits / 2 as usize + 1;

        let guess = if let Some(f) = self.to_f64() {
            // We fit in `f64` (lossy), so get a better initial guess from that.
            BigUint::from_f64(sqrt(f)).unwrap()
        } else {
            // Try to guess by scaling down such that it does fit in `f64`.
            // With some (x * 2²ᵏ), its sqrt ≈ (√x * 2ᵏ)
            let extra_bits = bits - (f64::MAX_EXP as usize - 1);
            let root_scale = (extra_bits + 1) / 2;
            let scale = root_scale * 2;
            (self >> scale).sqrt() << root_scale
        };

        fixpoint(guess, max_bits, move |s| {
            let q = self / s;
            let t = s + q;
            t >> 1
        })
    }

    fn cbrt(&self) -> Self {
        if self.is_zero() || self.is_one() {
            return self.clone();
        }

        // If we fit in `u64`, compute the root that way.
        if let Some(x) = self.to_u64() {
            return x.cbrt().into();
        }

        let bits = self.bits();
        let max_bits = bits / 3 as usize + 1;

        let guess = if let Some(f) = self.to_f64() {
            // We fit in `f64` (lossy), so get a better initial guess from that.
            BigUint::from_f64(cbrt(f)).unwrap()
        } else {
            // Try to guess by scaling down such that it does fit in `f64`.
            // With some (x * 2³ᵏ), its cbrt ≈ (∛x * 2ᵏ)
            let extra_bits = bits - (f64::MAX_EXP as usize - 1);
            let root_scale = (extra_bits + 2) / 3;
            let scale = root_scale * 3;
            (self >> scale).cbrt() << root_scale
        };

        fixpoint(guess, max_bits, move |s| {
            let q = self / (s * s);
            let t = (s << 1) + q;
            t / 3u32
        })
    }
}

fn high_bits_to_u64(v: &BigUint) -> u64 {
    match v.data.len() {
        0 => 0,
        1 => v.data[0] as u64,
        _ => {
            let mut bits = v.bits();
            let mut ret = 0u64;
            let mut ret_bits = 0;

            for d in v.data.iter().rev() {
                let digit_bits = (bits - 1) % big_digit::BITS + 1;
                let bits_want = cmp::min(64 - ret_bits, digit_bits);

                if bits_want != 64 {
                    ret <<= bits_want;
                }
                ret |= *d as u64 >> (digit_bits - bits_want);
                ret_bits += bits_want;
                bits -= bits_want;

                if ret_bits == 64 {
                    break;
                }
            }

            ret
        }
    }
}

impl ToPrimitive for BigUint {
    #[inline]
    fn to_i64(&self) -> Option<i64> {
        self.to_u64().as_ref().and_then(u64::to_i64)
    }

    #[inline]
    #[cfg(has_i128)]
    fn to_i128(&self) -> Option<i128> {
        self.to_u128().as_ref().and_then(u128::to_i128)
    }

    #[inline]
    fn to_u64(&self) -> Option<u64> {
        let mut ret: u64 = 0;
        let mut bits = 0;

        for i in self.data.iter() {
            if bits >= 64 {
                return None;
            }

            ret += (*i as u64) << bits;
            bits += big_digit::BITS;
        }

        Some(ret)
    }

    #[inline]
    #[cfg(has_i128)]
    fn to_u128(&self) -> Option<u128> {
        let mut ret: u128 = 0;
        let mut bits = 0;

        for i in self.data.iter() {
            if bits >= 128 {
                return None;
            }

            ret |= (*i as u128) << bits;
            bits += big_digit::BITS;
        }

        Some(ret)
    }

    #[inline]
    fn to_f32(&self) -> Option<f32> {
        let mantissa = high_bits_to_u64(self);
        let exponent = self.bits() - fls(mantissa);

        if exponent > f32::MAX_EXP as usize {
            None
        } else {
            let ret = (mantissa as f32) * 2.0f32.powi(exponent as i32);
            if ret.is_infinite() {
                None
            } else {
                Some(ret)
            }
        }
    }

    #[inline]
    fn to_f64(&self) -> Option<f64> {
        let mantissa = high_bits_to_u64(self);
        let exponent = self.bits() - fls(mantissa);

        if exponent > f64::MAX_EXP as usize {
            None
        } else {
            let ret = (mantissa as f64) * 2.0f64.powi(exponent as i32);
            if ret.is_infinite() {
                None
            } else {
                Some(ret)
            }
        }
    }
}

impl FromPrimitive for BigUint {
    #[inline]
    fn from_i64(n: i64) -> Option<BigUint> {
        if n >= 0 {
            Some(BigUint::from(n as u64))
        } else {
            None
        }
    }

    #[inline]
    #[cfg(has_i128)]
    fn from_i128(n: i128) -> Option<BigUint> {
        if n >= 0 {
            Some(BigUint::from(n as u128))
        } else {
            None
        }
    }

    #[inline]
    fn from_u64(n: u64) -> Option<BigUint> {
        Some(BigUint::from(n))
    }

    #[inline]
    #[cfg(has_i128)]
    fn from_u128(n: u128) -> Option<BigUint> {
        Some(BigUint::from(n))
    }

    #[inline]
    fn from_f64(mut n: f64) -> Option<BigUint> {
        // handle NAN, INFINITY, NEG_INFINITY
        if !n.is_finite() {
            return None;
        }

        // match the rounding of casting from float to int
        n = FloatCore::trunc(n);

        // handle 0.x, -0.x
        if n.is_zero() {
            return Some(BigUint::zero());
        }

        let (mantissa, exponent, sign) = FloatCore::integer_decode(n);

        if sign == -1 {
            return None;
        }

        let mut ret = BigUint::from(mantissa);
        if exponent > 0 {
            ret = ret << exponent as usize;
        } else if exponent < 0 {
            ret = ret >> (-exponent) as usize;
        }
        Some(ret)
    }
}

#[cfg(not(feature = "u64_digit"))]
impl From<u64> for BigUint {
    #[inline]
    fn from(mut n: u64) -> Self {
        let mut ret: BigUint = Zero::zero();

        while n != 0 {
            ret.data.push(n as BigDigit);
            // don't overflow if BITS is 64:
            n = (n >> 1) >> (big_digit::BITS - 1);
        }

        ret
    }
}

#[cfg(feature = "u64_digit")]
impl From<u64> for BigUint {
    #[inline]
    fn from(n: u64) -> Self {
        BigUint::new_native(smallvec![n])
    }
}

#[cfg(has_i128)]
impl From<u128> for BigUint {
    #[inline]
    fn from(mut n: u128) -> Self {
        let mut ret: BigUint = Zero::zero();

        while n != 0 {
            ret.data.push(n as BigDigit);
            n >>= big_digit::BITS;
        }

        ret
    }
}

macro_rules! impl_biguint_from_uint {
    ($T:ty) => {
        impl From<$T> for BigUint {
            #[inline]
            fn from(n: $T) -> Self {
                BigUint::from(n as u64)
            }
        }
    };
}

impl_biguint_from_uint!(u8);
impl_biguint_from_uint!(u16);
impl_biguint_from_uint!(u32);
impl_biguint_from_uint!(usize);

/// A generic trait for converting a value to a `BigUint`.
pub trait ToBigUint {
    /// Converts the value of `self` to a `BigUint`.
    fn to_biguint(&self) -> Option<BigUint>;
}

impl ToBigUint for BigUint {
    #[inline]
    fn to_biguint(&self) -> Option<BigUint> {
        Some(self.clone())
    }
}

/// A generic trait for converting a value to a `BigUint`, and consuming the value.
pub trait IntoBigUint {
    /// Converts the value of `self` to a `BigUint`.
    fn into_biguint(self) -> Option<BigUint>;
}

impl IntoBigUint for BigUint {
    #[inline]
    fn into_biguint(self) -> Option<BigUint> {
        Some(self)
    }
}

macro_rules! impl_to_biguint {
    ($T:ty, $from_ty:path) => {
        impl ToBigUint for $T {
            #[inline]
            fn to_biguint(&self) -> Option<BigUint> {
                $from_ty(*self)
            }
        }

        impl IntoBigUint for $T {
            #[inline]
            fn into_biguint(self) -> Option<BigUint> {
                $from_ty(self)
            }
        }
    };
}

impl_to_biguint!(isize, FromPrimitive::from_isize);
impl_to_biguint!(i8, FromPrimitive::from_i8);
impl_to_biguint!(i16, FromPrimitive::from_i16);
impl_to_biguint!(i32, FromPrimitive::from_i32);
impl_to_biguint!(i64, FromPrimitive::from_i64);
#[cfg(has_i128)]
impl_to_biguint!(i128, FromPrimitive::from_i128);

impl_to_biguint!(usize, FromPrimitive::from_usize);
impl_to_biguint!(u8, FromPrimitive::from_u8);
impl_to_biguint!(u16, FromPrimitive::from_u16);
impl_to_biguint!(u32, FromPrimitive::from_u32);
impl_to_biguint!(u64, FromPrimitive::from_u64);
#[cfg(has_i128)]
impl_to_biguint!(u128, FromPrimitive::from_u128);

impl_to_biguint!(f32, FromPrimitive::from_f32);
impl_to_biguint!(f64, FromPrimitive::from_f64);

// Extract bitwise digits that evenly divide BigDigit
fn to_bitwise_digits_le(u: &BigUint, bits: usize) -> Vec<u8> {
    debug_assert!(!u.is_zero() && bits <= 8 && big_digit::BITS % bits == 0);

    let last_i = u.data.len() - 1;
    let mask: BigDigit = (1 << bits) - 1;
    let digits_per_big_digit = big_digit::BITS / bits;
    let digits = (u.bits() + bits - 1) / bits;
    let mut res = Vec::with_capacity(digits);

    for mut r in u.data[..last_i].iter().cloned() {
        for _ in 0..digits_per_big_digit {
            res.push((r & mask) as u8);
            r >>= bits;
        }
    }

    let mut r = u.data[last_i];
    while r != 0 {
        res.push((r & mask) as u8);
        r >>= bits;
    }

    res
}

// Extract bitwise digits that don't evenly divide BigDigit
fn to_inexact_bitwise_digits_le(u: &BigUint, bits: usize) -> Vec<u8> {
    debug_assert!(!u.is_zero() && bits <= 8 && big_digit::BITS % bits != 0);

    let mask: BigDigit = (1 << bits) - 1;
    let digits = (u.bits() + bits - 1) / bits;
    let mut res = Vec::with_capacity(digits);

    let mut r = 0;
    let mut rbits = 0;

    for c in &u.data {
        r |= *c << rbits;
        rbits += big_digit::BITS;

        while rbits >= bits {
            res.push((r & mask) as u8);
            r >>= bits;

            // r had more bits than it could fit - grab the bits we lost
            if rbits > big_digit::BITS {
                r = *c >> (big_digit::BITS - (rbits - bits));
            }

            rbits -= bits;
        }
    }

    if rbits != 0 {
        res.push(r as u8);
    }

    while let Some(&0) = res.last() {
        res.pop();
    }

    res
}

// Extract little-endian radix digits
#[inline(always)] // forced inline to get const-prop for radix=10
fn to_radix_digits_le(u: &BigUint, radix: u32) -> Vec<u8> {
    debug_assert!(!u.is_zero() && !radix.is_power_of_two());

    // Estimate how big the result will be, so we can pre-allocate it.
    let bits = ilog2(radix);
    let radix_digits = idiv_ceil(u.bits(), bits);
    let mut res = Vec::with_capacity(radix_digits as usize);
    let mut digits = u.clone();

    let (base, power) = get_radix_base(radix);
    let radix = radix as BigDigit;

    while digits.data.len() > 1 {
        let (q, mut r) = div_rem_digit(digits, base);
        for _ in 0..power {
            res.push((r % radix) as u8);
            r /= radix;
        }
        digits = q;
    }

    let mut r = digits.data[0];
    while r != 0 {
        res.push((r % radix) as u8);
        r /= radix;
    }

    res
}

pub fn to_radix_le(u: &BigUint, radix: u32) -> Vec<u8> {
    if u.is_zero() {
        vec![0]
    } else if radix.is_power_of_two() {
        // Powers of two can use bitwise masks and shifting instead of division
        let bits = ilog2(radix);
        if big_digit::BITS % bits == 0 {
            to_bitwise_digits_le(u, bits)
        } else {
            to_inexact_bitwise_digits_le(u, bits)
        }
    } else if radix == 10 {
        // 10 is so common that it's worth separating out for const-propagation.
        // Optimizers can often turn constant division into a faster multiplication.
        to_radix_digits_le(u, 10)
    } else {
        to_radix_digits_le(u, radix)
    }
}

pub fn to_str_radix_reversed(u: &BigUint, radix: u32) -> Vec<u8> {
    assert!(2 <= radix && radix <= 36, "The radix must be within 2...36");

    if u.is_zero() {
        return vec![b'0'];
    }

    let mut res = to_radix_le(u, radix);

    // Now convert everything to ASCII digits.
    for r in &mut res {
        debug_assert!((*r as u32) < radix);
        if *r < 10 {
            *r += b'0';
        } else {
            *r += b'a' - 10;
        }
    }
    res
}

#[cfg(not(feature = "u64_digit"))]
#[inline]
fn ensure_big_digit(raw: Vec<u32>) -> SmallVec<[BigDigit; VEC_SIZE]> {
    raw.into()
}

#[cfg(feature = "u64_digit")]
#[inline]
fn ensure_big_digit(raw: Vec<u32>) -> SmallVec<[BigDigit; VEC_SIZE]> {
    ensure_big_digit_slice(&raw)
}

#[cfg(feature = "u64_digit")]
#[inline]
fn ensure_big_digit_slice(raw: &[u32]) -> SmallVec<[BigDigit; VEC_SIZE]> {
    raw.chunks(2)
        .map(|chunk| {
            // raw could have odd length
            if chunk.len() < 2 {
                chunk[0] as BigDigit
            } else {
                BigDigit::from(chunk[0]) | (BigDigit::from(chunk[1]) << 32)
            }
        })
        .collect()
}

impl BigUint {
    /// Creates and initializes a `BigUint`.
    ///
    /// The digits are in little-endian base 2<sup>32</sup>.
    #[inline]
    pub fn new(digits: Vec<u32>) -> BigUint {
        Self::new_native(ensure_big_digit(digits))
    }

    /// Creates and initializes a `BigUint`.
    ///
    /// The digits are in little-endian base matching `BigDigit`.
    #[inline]
    pub fn new_native(digits: SmallVec<[BigDigit; VEC_SIZE]>) -> BigUint {
        BigUint { data: digits }.normalized()
    }

    /// Creates and initializes a `BigUint`.
    ///
    /// The digits are in little-endian base 2<sup>32</sup>.
    #[inline]
    pub fn from_slice(slice: &[u32]) -> BigUint {
        BigUint::new(slice.to_vec())
    }

    /// Creates and initializes a `BigUint`.
    ///
    /// The digits are in little-endian base matching `BigDigit`
    #[inline]
    pub fn from_slice_native(slice: &[BigDigit]) -> BigUint {
        BigUint::new_native(slice.into())
    }

    pub fn get_limb(&self, i: usize) -> BigDigit {
        self.data[i]
    }

    /// Assign a value to a `BigUint`.
    ///
    /// The digits are in little-endian base 2<sup>32</sup>.
    #[cfg(not(feature = "u64_digit"))]
    #[inline]
    pub fn assign_from_slice(&mut self, slice: &[u32]) {
        self.assign_from_slice_native(slice);
    }
    #[cfg(feature = "u64_digit")]
    #[inline]
    pub fn assign_from_slice(&mut self, slice: &[u32]) {
        let slice_digits = ensure_big_digit_slice(slice);
        self.assign_from_slice_native(&slice_digits);
    }

    /// Assign a value to a `BigUint`.
    ///
    /// The digits are in little-endian with the base matching `BigDigit`.
    #[inline]
    pub fn assign_from_slice_native(&mut self, slice: &[BigDigit]) {
        self.data.resize(slice.len(), 0);
        self.data.clone_from_slice(slice);
        self.normalize();
    }

    /// Creates and initializes a `BigUint`.
    ///
    /// The bytes are in big-endian byte order.
    ///
    /// # Examples
    ///
    /// ```
    /// use num_bigint_dig::BigUint;
    ///
    /// assert_eq!(BigUint::from_bytes_be(b"A"),
    ///            BigUint::parse_bytes(b"65", 10).unwrap());
    /// assert_eq!(BigUint::from_bytes_be(b"AA"),
    ///            BigUint::parse_bytes(b"16705", 10).unwrap());
    /// assert_eq!(BigUint::from_bytes_be(b"AB"),
    ///            BigUint::parse_bytes(b"16706", 10).unwrap());
    /// assert_eq!(BigUint::from_bytes_be(b"Hello world!"),
    ///            BigUint::parse_bytes(b"22405534230753963835153736737", 10).unwrap());
    /// ```
    #[inline]
    pub fn from_bytes_be(bytes: &[u8]) -> BigUint {
        if bytes.is_empty() {
            Zero::zero()
        } else {
            let mut v = bytes.to_vec();
            v.reverse();
            BigUint::from_bytes_le(&*v)
        }
    }

    /// Creates and initializes a `BigUint`.
    ///
    /// The bytes are in little-endian byte order.
    #[inline]
    pub fn from_bytes_le(bytes: &[u8]) -> BigUint {
        if bytes.is_empty() {
            Zero::zero()
        } else {
            from_bitwise_digits_le(bytes, 8)
        }
    }

    /// Creates and initializes a `BigUint`. The input slice must contain
    /// ascii/utf8 characters in [0-9a-zA-Z].
    /// `radix` must be in the range `2...36`.
    ///
    /// The function `from_str_radix` from the `Num` trait provides the same logic
    /// for `&str` buffers.
    ///
    /// # Examples
    ///
    /// ```
    /// use num_bigint_dig::{BigUint, ToBigUint};
    ///
    /// assert_eq!(BigUint::parse_bytes(b"1234", 10), ToBigUint::to_biguint(&1234));
    /// assert_eq!(BigUint::parse_bytes(b"ABCD", 16), ToBigUint::to_biguint(&0xABCD));
    /// assert_eq!(BigUint::parse_bytes(b"G", 16), None);
    /// ```
    #[inline]
    pub fn parse_bytes(buf: &[u8], radix: u32) -> Option<BigUint> {
        str::from_utf8(buf)
            .ok()
            .and_then(|s| BigUint::from_str_radix(s, radix).ok())
    }

    /// Creates and initializes a `BigUint`. Each u8 of the input slice is
    /// interpreted as one digit of the number
    /// and must therefore be less than `radix`.
    ///
    /// The bytes are in big-endian byte order.
    /// `radix` must be in the range `2...256`.
    ///
    /// # Examples
    ///
    /// ```
    /// use num_bigint_dig::{BigUint};
    ///
    /// let inbase190 = &[15, 33, 125, 12, 14];
    /// let a = BigUint::from_radix_be(inbase190, 190).unwrap();
    /// assert_eq!(a.to_radix_be(190), inbase190);
    /// ```
    pub fn from_radix_be(buf: &[u8], radix: u32) -> Option<BigUint> {
        assert!(
            2 <= radix && radix <= 256,
            "The radix must be within 2...256"
        );

        if radix != 256 && buf.iter().any(|&b| b >= radix as u8) {
            return None;
        }

        let res = if radix.is_power_of_two() {
            // Powers of two can use bitwise masks and shifting instead of multiplication
            let bits = ilog2(radix);
            let mut v = Vec::from(buf);
            v.reverse();
            if big_digit::BITS % bits == 0 {
                from_bitwise_digits_le(&v, bits)
            } else {
                from_inexact_bitwise_digits_le(&v, bits)
            }
        } else {
            from_radix_digits_be(buf, radix)
        };

        Some(res)
    }

    /// Creates and initializes a `BigUint`. Each u8 of the input slice is
    /// interpreted as one digit of the number
    /// and must therefore be less than `radix`.
    ///
    /// The bytes are in little-endian byte order.
    /// `radix` must be in the range `2...256`.
    ///
    /// # Examples
    ///
    /// ```
    /// use num_bigint_dig::{BigUint};
    ///
    /// let inbase190 = &[14, 12, 125, 33, 15];
    /// let a = BigUint::from_radix_be(inbase190, 190).unwrap();
    /// assert_eq!(a.to_radix_be(190), inbase190);
    /// ```
    pub fn from_radix_le(buf: &[u8], radix: u32) -> Option<BigUint> {
        assert!(
            2 <= radix && radix <= 256,
            "The radix must be within 2...256"
        );

        if radix != 256 && buf.iter().any(|&b| b >= radix as u8) {
            return None;
        }

        let res = if radix.is_power_of_two() {
            // Powers of two can use bitwise masks and shifting instead of multiplication
            let bits = ilog2(radix);
            if big_digit::BITS % bits == 0 {
                from_bitwise_digits_le(buf, bits)
            } else {
                from_inexact_bitwise_digits_le(buf, bits)
            }
        } else {
            let mut v = Vec::from(buf);
            v.reverse();
            from_radix_digits_be(&v, radix)
        };

        Some(res)
    }

    /// Returns the byte representation of the `BigUint` in big-endian byte order.
    ///
    /// # Examples
    ///
    /// ```
    /// use num_bigint_dig::BigUint;
    ///
    /// let i = BigUint::parse_bytes(b"1125", 10).unwrap();
    /// assert_eq!(i.to_bytes_be(), vec![4, 101]);
    /// ```
    #[inline]
    pub fn to_bytes_be(&self) -> Vec<u8> {
        let mut v = self.to_bytes_le();
        v.reverse();
        v
    }

    /// Returns the byte representation of the `BigUint` in little-endian byte order.
    ///
    /// # Examples
    ///
    /// ```
    /// use num_bigint_dig::BigUint;
    ///
    /// let i = BigUint::parse_bytes(b"1125", 10).unwrap();
    /// assert_eq!(i.to_bytes_le(), vec![101, 4]);
    /// ```
    #[inline]
    pub fn to_bytes_le(&self) -> Vec<u8> {
        if self.is_zero() {
            vec![0]
        } else {
            to_bitwise_digits_le(self, 8)
        }
    }

    /// Returns the integer formatted as a string in the given radix.
    /// `radix` must be in the range `2...36`.
    ///
    /// # Examples
    ///
    /// ```
    /// use num_bigint_dig::BigUint;
    ///
    /// let i = BigUint::parse_bytes(b"ff", 16).unwrap();
    /// assert_eq!(i.to_str_radix(16), "ff");
    /// ```
    #[inline]
    pub fn to_str_radix(&self, radix: u32) -> String {
        let mut v = to_str_radix_reversed(self, radix);
        v.reverse();
        unsafe { String::from_utf8_unchecked(v) }
    }

    /// Returns the integer in the requested base in big-endian digit order.
    /// The output is not given in a human readable alphabet but as a zero
    /// based u8 number.
    /// `radix` must be in the range `2...256`.
    ///
    /// # Examples
    ///
    /// ```
    /// use num_bigint_dig::BigUint;
    ///
    /// assert_eq!(BigUint::from(0xFFFFu64).to_radix_be(159),
    ///            vec![2, 94, 27]);
    /// // 0xFFFF = 65535 = 2*(159^2) + 94*159 + 27
    /// ```
    #[inline]
    pub fn to_radix_be(&self, radix: u32) -> Vec<u8> {
        let mut v = to_radix_le(self, radix);
        v.reverse();
        v
    }

    /// Returns the integer in the requested base in little-endian digit order.
    /// The output is not given in a human readable alphabet but as a zero
    /// based u8 number.
    /// `radix` must be in the range `2...256`.
    ///
    /// # Examples
    ///
    /// ```
    /// use num_bigint_dig::BigUint;
    ///
    /// assert_eq!(BigUint::from(0xFFFFu64).to_radix_le(159),
    ///            vec![27, 94, 2]);
    /// // 0xFFFF = 65535 = 27 + 94*159 + 2*(159^2)
    /// ```
    #[inline]
    pub fn to_radix_le(&self, radix: u32) -> Vec<u8> {
        to_radix_le(self, radix)
    }

    /// Determines the fewest bits necessary to express the `BigUint`.
    #[inline]
    pub fn bits(&self) -> usize {
        if self.is_zero() {
            return 0;
        }
        let zeros = self.data.last().unwrap().leading_zeros();
        self.data.len() * big_digit::BITS - zeros as usize
    }

    /// Strips off trailing zero bigdigits - comparisons require the last element in the vector to
    /// be nonzero.
    #[inline]
    pub(crate) fn normalize(&mut self) {
        while let Some(&0) = self.data.last() {
            self.data.pop();
        }
    }

    /// Returns a normalized `BigUint`.
    #[inline]
    pub(crate) fn normalized(mut self) -> BigUint {
        self.normalize();
        self
    }

    /// Returns `(self ^ exponent) % modulus`.
    ///
    /// Panics if the modulus is zero.
    pub fn modpow(&self, exponent: &Self, modulus: &Self) -> Self {
        assert!(!modulus.is_zero(), "divide by zero!");

        // For an odd modulus, we can use Montgomery multiplication in base 2^32.
        if modulus.is_odd() {
            return monty_modpow(self, exponent, modulus);
        }

        // Otherwise do basically the same as `num::pow`, but with a modulus.
        let one = BigUint::one();
        if exponent.is_zero() {
            return one;
        }

        let mut base = self % modulus;
        let mut exp = exponent.clone();
        while exp.is_even() {
            base = &base * &base % modulus;
            exp >>= 1;
        }
        if exp == one {
            return base;
        }

        let mut acc = base.clone();
        while exp > one {
            exp >>= 1;
            base = &base * &base % modulus;
            if exp.is_odd() {
                acc = acc * &base % modulus;
            }
        }
        acc
    }

    /// Returns the truncated principal square root of `self` --
    /// see [Roots::sqrt](https://docs.rs/num-integer/0.1/num_integer/trait.Roots.html#method.sqrt)
    pub fn sqrt(&self) -> Self {
        Roots::sqrt(self)
    }

    /// Returns the truncated principal cube root of `self` --
    /// see [Roots::cbrt](https://docs.rs/num-integer/0.1/num_integer/trait.Roots.html#method.cbrt).
    pub fn cbrt(&self) -> Self {
        Roots::cbrt(self)
    }

    /// Returns the truncated principal `n`th root of `self` --
    /// see [Roots::nth_root](https://docs.rs/num-integer/0.1/num_integer/trait.Roots.html#tymethod.nth_root).
    pub fn nth_root(&self, n: u32) -> Self {
        Roots::nth_root(self, n)
    }

    pub fn trailing_zeros(&self) -> Option<usize> {
        trailing_zeros(self)
    }

    /// Sets the value to the provided digit, reusing internal storage.
    pub fn set_digit(&mut self, digit: BigDigit) {
        if self.is_zero() {
            self.data.resize(1, digit);
        } else {
            self.data.resize(1, 0);
            self.data[0] = digit;
        }
    }
}

/// Returns the number of least-significant bits that are zero,
/// or `None` if the entire number is zero.
pub fn trailing_zeros(u: &BigUint) -> Option<usize> {
    u.data
        .iter()
        .enumerate()
        .find(|&(_, &digit)| digit != 0)
        .map(|(i, digit)| i * big_digit::BITS + digit.trailing_zeros() as usize)
}

impl_sum_iter_type!(BigUint);
impl_product_iter_type!(BigUint);

pub trait IntDigits {
    fn digits(&self) -> &[BigDigit];
    fn digits_mut(&mut self) -> &mut SmallVec<[BigDigit; VEC_SIZE]>;
    fn normalize(&mut self);
    fn capacity(&self) -> usize;
    fn len(&self) -> usize;
}

impl IntDigits for BigUint {
    #[inline]
    fn digits(&self) -> &[BigDigit] {
        &self.data
    }
    #[inline]
    fn digits_mut(&mut self) -> &mut SmallVec<[BigDigit; VEC_SIZE]> {
        &mut self.data
    }
    #[inline]
    fn normalize(&mut self) {
        self.normalize();
    }
    #[inline]
    fn capacity(&self) -> usize {
        self.data.capacity()
    }
    #[inline]
    fn len(&self) -> usize {
        self.data.len()
    }
}

/// Combine four `u32`s into a single `u128`.
#[cfg(has_i128)]
#[inline]
#[allow(dead_code)]
fn u32_to_u128(a: u32, b: u32, c: u32, d: u32) -> u128 {
    u128::from(d) | (u128::from(c) << 32) | (u128::from(b) << 64) | (u128::from(a) << 96)
}

/// Split a single `u128` into four `u32`.
#[cfg(has_i128)]
#[inline]
#[allow(dead_code)]
fn u32_from_u128(n: u128) -> (u32, u32, u32, u32) {
    (
        (n >> 96) as u32,
        (n >> 64) as u32,
        (n >> 32) as u32,
        n as u32,
    )
}

#[cfg(feature = "serde")]
#[cfg(not(feature = "u64_digit"))]
impl serde::Serialize for BigUint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Note: do not change the serialization format, or it may break forward
        // and backward compatibility of serialized data!  If we ever change the
        // internal representation, we should still serialize in base-`u32`.
        let data: &[u32] = &self.data.as_slice();
        data.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
#[cfg(feature = "u64_digit")]
impl serde::Serialize for BigUint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let last = if self.data.is_empty() {
            0
        } else {
            self.data.len() - 1
        };
        let data: Vec<u32> = self
            .data
            .iter()
            .enumerate()
            .flat_map(|(i, n)| {
                if i == last && n < &(u32::MAX as u64) {
                    vec![*n as u32]
                } else {
                    vec![*n as u32, (n >> 32) as u32]
                }
            })
            .collect();
        data.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for BigUint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let data: Vec<u32> = Vec::deserialize(deserializer)?;
        Ok(BigUint::new(data))
    }
}

/// Returns the greatest power of the radix <= big_digit::BASE
#[inline]
fn get_radix_base(radix: u32) -> (BigDigit, usize) {
    debug_assert!(
        2 <= radix && radix <= 256,
        "The radix must be within 2...256"
    );
    debug_assert!(!radix.is_power_of_two());

    // To generate this table:
    //    for radix in 2u64..257 {
    //        let mut power = big_digit::BITS / fls(radix as u64);
    //        let mut base = radix.pow(power as u32);
    //
    //        while let Some(b) = base.checked_mul(radix) {
    //            if b > big_digit::MAX {
    //                break;
    //            }
    //            base = b;
    //            power += 1;
    //        }
    //
    //        println!("({:10}, {:2}), // {:2}", base, power, radix);
    //    }
    // and
    //    for radix in 2u64..257 {
    //        let mut power = 64 / fls(radix as u64);
    //        let mut base = radix.pow(power as u32);
    //
    //        while let Some(b) = base.checked_mul(radix) {
    //            base = b;
    //            power += 1;
    //        }
    //
    //        println!("({:20}, {:2}), // {:2}", base, power, radix);
    //    }
    match big_digit::BITS {
        32 => {
            const BASES: [(u32, usize); 257] = [
                (0, 0),
                (0, 0),
                (0, 0),              //  2
                (3_486_784_401, 20), //  3
                (0, 0),              //  4
                (1_220_703_125, 13), //  5
                (2_176_782_336, 12), //  6
                (1_977_326_743, 11), //  7
                (0, 0),              //  8
                (3_486_784_401, 10), //  9
                (1_000_000_000, 9),  // 10
                (2_357_947_691, 9),  // 11
                (429_981_696, 8),    // 12
                (815_730_721, 8),    // 13
                (1_475_789_056, 8),  // 14
                (2_562_890_625, 8),  // 15
                (0, 0),              // 16
                (410_338_673, 7),    // 17
                (612_220_032, 7),    // 18
                (893_871_739, 7),    // 19
                (1_280_000_000, 7),  // 20
                (1_801_088_541, 7),  // 21
                (2_494_357_888, 7),  // 22
                (3_404_825_447, 7),  // 23
                (191_102_976, 6),    // 24
                (244_140_625, 6),    // 25
                (308_915_776, 6),    // 26
                (387_420_489, 6),    // 27
                (481_890_304, 6),    // 28
                (594_823_321, 6),    // 29
                (729_000_000, 6),    // 30
                (887_503_681, 6),    // 31
                (0, 0),              // 32
                (1_291_467_969, 6),  // 33
                (1_544_804_416, 6),  // 34
                (1_838_265_625, 6),  // 35
                (2_176_782_336, 6),  // 36
                (2_565_726_409, 6),  // 37
                (3_010_936_384, 6),  // 38
                (3_518_743_761, 6),  // 39
                (4_096_000_000, 6),  // 40
                (115_856_201, 5),    // 41
                (130_691_232, 5),    // 42
                (147_008_443, 5),    // 43
                (164_916_224, 5),    // 44
                (184_528_125, 5),    // 45
                (205_962_976, 5),    // 46
                (229_345_007, 5),    // 47
                (254_803_968, 5),    // 48
                (282_475_249, 5),    // 49
                (312_500_000, 5),    // 50
                (345_025_251, 5),    // 51
                (380_204_032, 5),    // 52
                (418_195_493, 5),    // 53
                (459_165_024, 5),    // 54
                (503_284_375, 5),    // 55
                (550_731_776, 5),    // 56
                (601_692_057, 5),    // 57
                (656_356_768, 5),    // 58
                (714_924_299, 5),    // 59
                (777_600_000, 5),    // 60
                (844_596_301, 5),    // 61
                (916_132_832, 5),    // 62
                (992_436_543, 5),    // 63
                (0, 0),              // 64
                (1_160_290_625, 5),  // 65
                (1_252_332_576, 5),  // 66
                (1_350_125_107, 5),  // 67
                (1_453_933_568, 5),  // 68
                (1_564_031_349, 5),  // 69
                (1_680_700_000, 5),  // 70
                (1_804_229_351, 5),  // 71
                (1_934_917_632, 5),  // 72
                (2_073_071_593, 5),  // 73
                (2_219_006_624, 5),  // 74
                (2_373_046_875, 5),  // 75
                (2_535_525_376, 5),  // 76
                (2_706_784_157, 5),  // 77
                (2_887_174_368, 5),  // 78
                (3_077_056_399, 5),  // 79
                (3_276_800_000, 5),  // 80
                (3_486_784_401, 5),  // 81
                (3_707_398_432, 5),  // 82
                (3_939_040_643, 5),  // 83
                (4_182_119_424, 5),  // 84
                (52_200_625, 4),     // 85
                (54_700_816, 4),     // 86
                (57_289_761, 4),     // 87
                (59_969_536, 4),     // 88
                (62_742_241, 4),     // 89
                (65_610_000, 4),     // 90
                (68_574_961, 4),     // 91
                (71_639_296, 4),     // 92
                (74_805_201, 4),     // 93
                (78_074_896, 4),     // 94
                (81_450_625, 4),     // 95
                (84_934_656, 4),     // 96
                (88_529_281, 4),     // 97
                (92_236_816, 4),     // 98
                (96_059_601, 4),     // 99
                (100_000_000, 4),    // 100
                (104_060_401, 4),    // 101
                (108_243_216, 4),    // 102
                (112_550_881, 4),    // 103
                (116_985_856, 4),    // 104
                (121_550_625, 4),    // 105
                (126_247_696, 4),    // 106
                (131_079_601, 4),    // 107
                (136_048_896, 4),    // 108
                (141_158_161, 4),    // 109
                (146_410_000, 4),    // 110
                (151_807_041, 4),    // 111
                (157_351_936, 4),    // 112
                (163_047_361, 4),    // 113
                (168_896_016, 4),    // 114
                (174_900_625, 4),    // 115
                (181_063_936, 4),    // 116
                (187_388_721, 4),    // 117
                (193_877_776, 4),    // 118
                (200_533_921, 4),    // 119
                (207_360_000, 4),    // 120
                (214_358_881, 4),    // 121
                (221_533_456, 4),    // 122
                (228_886_641, 4),    // 123
                (236_421_376, 4),    // 124
                (244_140_625, 4),    // 125
                (252_047_376, 4),    // 126
                (260_144_641, 4),    // 127
                (0, 0),              // 128
                (276_922_881, 4),    // 129
                (285_610_000, 4),    // 130
                (294_499_921, 4),    // 131
                (303_595_776, 4),    // 132
                (312_900_721, 4),    // 133
                (322_417_936, 4),    // 134
                (332_150_625, 4),    // 135
                (342_102_016, 4),    // 136
                (352_275_361, 4),    // 137
                (362_673_936, 4),    // 138
                (373_301_041, 4),    // 139
                (384_160_000, 4),    // 140
                (395_254_161, 4),    // 141
                (406_586_896, 4),    // 142
                (418_161_601, 4),    // 143
                (429_981_696, 4),    // 144
                (442_050_625, 4),    // 145
                (454_371_856, 4),    // 146
                (466_948_881, 4),    // 147
                (479_785_216, 4),    // 148
                (492_884_401, 4),    // 149
                (506_250_000, 4),    // 150
                (519_885_601, 4),    // 151
                (533_794_816, 4),    // 152
                (547_981_281, 4),    // 153
                (562_448_656, 4),    // 154
                (577_200_625, 4),    // 155
                (592_240_896, 4),    // 156
                (607_573_201, 4),    // 157
                (623_201_296, 4),    // 158
                (639_128_961, 4),    // 159
                (655_360_000, 4),    // 160
                (671_898_241, 4),    // 161
                (688_747_536, 4),    // 162
                (705_911_761, 4),    // 163
                (723_394_816, 4),    // 164
                (741_200_625, 4),    // 165
                (759_333_136, 4),    // 166
                (777_796_321, 4),    // 167
                (796_594_176, 4),    // 168
                (815_730_721, 4),    // 169
                (835_210_000, 4),    // 170
                (855_036_081, 4),    // 171
                (875_213_056, 4),    // 172
                (895_745_041, 4),    // 173
                (916_636_176, 4),    // 174
                (937_890_625, 4),    // 175
                (959_512_576, 4),    // 176
                (981_506_241, 4),    // 177
                (1_003_875_856, 4),  // 178
                (1_026_625_681, 4),  // 179
                (1_049_760_000, 4),  // 180
                (1_073_283_121, 4),  // 181
                (1_097_199_376, 4),  // 182
                (1_121_513_121, 4),  // 183
                (1_146_228_736, 4),  // 184
                (1_171_350_625, 4),  // 185
                (1_196_883_216, 4),  // 186
                (1_222_830_961, 4),  // 187
                (1_249_198_336, 4),  // 188
                (1_275_989_841, 4),  // 189
                (1_303_210_000, 4),  // 190
                (1_330_863_361, 4),  // 191
                (1_358_954_496, 4),  // 192
                (1_387_488_001, 4),  // 193
                (1_416_468_496, 4),  // 194
                (1_445_900_625, 4),  // 195
                (1_475_789_056, 4),  // 196
                (1_506_138_481, 4),  // 197
                (1_536_953_616, 4),  // 198
                (1_568_239_201, 4),  // 199
                (1_600_000_000, 4),  // 200
                (1_632_240_801, 4),  // 201
                (1_664_966_416, 4),  // 202
                (1_698_181_681, 4),  // 203
                (1_731_891_456, 4),  // 204
                (1_766_100_625, 4),  // 205
                (1_800_814_096, 4),  // 206
                (1_836_036_801, 4),  // 207
                (1_871_773_696, 4),  // 208
                (1_908_029_761, 4),  // 209
                (1_944_810_000, 4),  // 210
                (1_982_119_441, 4),  // 211
                (2_019_963_136, 4),  // 212
                (2_058_346_161, 4),  // 213
                (2_097_273_616, 4),  // 214
                (2_136_750_625, 4),  // 215
                (2_176_782_336, 4),  // 216
                (2_217_373_921, 4),  // 217
                (2_258_530_576, 4),  // 218
                (2_300_257_521, 4),  // 219
                (2_342_560_000, 4),  // 220
                (2_385_443_281, 4),  // 221
                (2_428_912_656, 4),  // 222
                (2_472_973_441, 4),  // 223
                (2_517_630_976, 4),  // 224
                (2_562_890_625, 4),  // 225
                (2_608_757_776, 4),  // 226
                (2_655_237_841, 4),  // 227
                (2_702_336_256, 4),  // 228
                (2_750_058_481, 4),  // 229
                (2_798_410_000, 4),  // 230
                (2_847_396_321, 4),  // 231
                (2_897_022_976, 4),  // 232
                (2_947_295_521, 4),  // 233
                (2_998_219_536, 4),  // 234
                (3_049_800_625, 4),  // 235
                (3_102_044_416, 4),  // 236
                (3_154_956_561, 4),  // 237
                (3_208_542_736, 4),  // 238
                (3_262_808_641, 4),  // 239
                (3_317_760_000, 4),  // 240
                (3_373_402_561, 4),  // 241
                (3_429_742_096, 4),  // 242
                (3_486_784_401, 4),  // 243
                (3_544_535_296, 4),  // 244
                (3_603_000_625, 4),  // 245
                (3_662_186_256, 4),  // 246
                (3_722_098_081, 4),  // 247
                (3_782_742_016, 4),  // 248
                (3_844_124_001, 4),  // 249
                (3_906_250_000, 4),  // 250
                (3_969_126_001, 4),  // 251
                (4_032_758_016, 4),  // 252
                (4_097_152_081, 4),  // 253
                (4_162_314_256, 4),  // 254
                (4_228_250_625, 4),  // 255
                (0, 0),              // 256
            ];

            let (base, power) = BASES[radix as usize];
            (base as BigDigit, power)
        }
        64 => {
            const BASES: [(u64, usize); 257] = [
                (0, 0),
                (0, 0),
                (9_223_372_036_854_775_808, 63),  //  2
                (12_157_665_459_056_928_801, 40), //  3
                (4_611_686_018_427_387_904, 31),  //  4
                (7_450_580_596_923_828_125, 27),  //  5
                (4_738_381_338_321_616_896, 24),  //  6
                (3_909_821_048_582_988_049, 22),  //  7
                (9_223_372_036_854_775_808, 21),  //  8
                (12_157_665_459_056_928_801, 20), //  9
                (10_000_000_000_000_000_000, 19), // 10
                (5_559_917_313_492_231_481, 18),  // 11
                (2_218_611_106_740_436_992, 17),  // 12
                (8_650_415_919_381_337_933, 17),  // 13
                (2_177_953_337_809_371_136, 16),  // 14
                (6_568_408_355_712_890_625, 16),  // 15
                (1_152_921_504_606_846_976, 15),  // 16
                (2_862_423_051_509_815_793, 15),  // 17
                (6_746_640_616_477_458_432, 15),  // 18
                (15_181_127_029_874_798_299, 15), // 19
                (1_638_400_000_000_000_000, 14),  // 20
                (3_243_919_932_521_508_681, 14),  // 21
                (6_221_821_273_427_820_544, 14),  // 22
                (11_592_836_324_538_749_809, 14), // 23
                (876_488_338_465_357_824, 13),    // 24
                (1_490_116_119_384_765_625, 13),  // 25
                (2_481_152_873_203_736_576, 13),  // 26
                (4_052_555_153_018_976_267, 13),  // 27
                (6_502_111_422_497_947_648, 13),  // 28
                (10_260_628_712_958_602_189, 13), // 29
                (15_943_230_000_000_000_000, 13), // 30
                (787_662_783_788_549_761, 12),    // 31
                (1_152_921_504_606_846_976, 12),  // 32
                (1_667_889_514_952_984_961, 12),  // 33
                (2_386_420_683_693_101_056, 12),  // 34
                (3_379_220_508_056_640_625, 12),  // 35
                (4_738_381_338_321_616_896, 12),  // 36
                (6_582_952_005_840_035_281, 12),  // 37
                (9_065_737_908_494_995_456, 12),  // 38
                (12_381_557_655_576_425_121, 12), // 39
                (16_777_216_000_000_000_000, 12), // 40
                (550_329_031_716_248_441, 11),    // 41
                (717_368_321_110_468_608, 11),    // 42
                (929_293_739_471_222_707, 11),    // 43
                (1_196_683_881_290_399_744, 11),  // 44
                (1_532_278_301_220_703_125, 11),  // 45
                (1_951_354_384_207_722_496, 11),  // 46
                (2_472_159_215_084_012_303, 11),  // 47
                (3_116_402_981_210_161_152, 11),  // 48
                (3_909_821_048_582_988_049, 11),  // 49
                (4_882_812_500_000_000_000, 11),  // 50
                (6_071_163_615_208_263_051, 11),  // 51
                (7_516_865_509_350_965_248, 11),  // 52
                (9_269_035_929_372_191_597, 11),  // 53
                (11_384_956_040_305_711_104, 11), // 54
                (13_931_233_916_552_734_375, 11), // 55
                (16_985_107_389_382_393_856, 11), // 56
                (362_033_331_456_891_249, 10),    // 57
                (430_804_206_899_405_824, 10),    // 58
                (511_116_753_300_641_401, 10),    // 59
                (604_661_760_000_000_000, 10),    // 60
                (713_342_911_662_882_601, 10),    // 61
                (839_299_365_868_340_224, 10),    // 62
                (984_930_291_881_790_849, 10),    // 63
                (1_152_921_504_606_846_976, 10),  // 64
                (1_346_274_334_462_890_625, 10),  // 65
                (1_568_336_880_910_795_776, 10),  // 66
                (1_822_837_804_551_761_449, 10),  // 67
                (2_113_922_820_157_210_624, 10),  // 68
                (2_446_194_060_654_759_801, 10),  // 69
                (2_824_752_490_000_000_000, 10),  // 70
                (3_255_243_551_009_881_201, 10),  // 71
                (3_743_906_242_624_487_424, 10),  // 72
                (4_297_625_829_703_557_649, 10),  // 73
                (4_923_990_397_355_877_376, 10),  // 74
                (5_631_351_470_947_265_625, 10),  // 75
                (6_428_888_932_339_941_376, 10),  // 76
                (7_326_680_472_586_200_649, 10),  // 77
                (8_335_775_831_236_199_424, 10),  // 78
                (9_468_276_082_626_847_201, 10),  // 79
                (10_737_418_240_000_000_000, 10), // 80
                (12_157_665_459_056_928_801, 10), // 81
                (13_744_803_133_596_058_624, 10), // 82
                (15_516_041_187_205_853_449, 10), // 83
                (17_490_122_876_598_091_776, 10), // 84
                (231_616_946_283_203_125, 9),     // 85
                (257_327_417_311_663_616, 9),     // 86
                (285_544_154_243_029_527, 9),     // 87
                (316_478_381_828_866_048, 9),     // 88
                (350_356_403_707_485_209, 9),     // 89
                (387_420_489_000_000_000, 9),     // 90
                (427_929_800_129_788_411, 9),     // 91
                (472_161_363_286_556_672, 9),     // 92
                (520_411_082_988_487_293, 9),     // 93
                (572_994_802_228_616_704, 9),     // 94
                (630_249_409_724_609_375, 9),     // 95
                (692_533_995_824_480_256, 9),     // 96
                (760_231_058_654_565_217, 9),     // 97
                (833_747_762_130_149_888, 9),     // 98
                (913_517_247_483_640_899, 9),     // 99
                (1_000_000_000_000_000_000, 9),   // 100
                (1_093_685_272_684_360_901, 9),   // 101
                (1_195_092_568_622_310_912, 9),   // 102
                (1_304_773_183_829_244_583, 9),   // 103
                (1_423_311_812_421_484_544, 9),   // 104
                (1_551_328_215_978_515_625, 9),   // 105
                (1_689_478_959_002_692_096, 9),   // 106
                (1_838_459_212_420_154_507, 9),   // 107
                (1_999_004_627_104_432_128, 9),   // 108
                (2_171_893_279_442_309_389, 9),   // 109
                (2_357_947_691_000_000_000, 9),   // 110
                (2_558_036_924_386_500_591, 9),   // 111
                (2_773_078_757_450_186_752, 9),   // 112
                (3_004_041_937_984_268_273, 9),   // 113
                (3_251_948_521_156_637_184, 9),   // 114
                (3_517_876_291_919_921_875, 9),   // 115
                (3_802_961_274_698_203_136, 9),   // 116
                (4_108_400_332_687_853_397, 9),   // 117
                (4_435_453_859_151_328_768, 9),   // 118
                (4_785_448_563_124_474_679, 9),   // 119
                (5_159_780_352_000_000_000, 9),   // 120
                (5_559_917_313_492_231_481, 9),   // 121
                (5_987_402_799_531_080_192, 9),   // 122
                (6_443_858_614_676_334_363, 9),   // 123
                (6_930_988_311_686_938_624, 9),   // 124
                (7_450_580_596_923_828_125, 9),   // 125
                (8_004_512_848_309_157_376, 9),   // 126
                (8_594_754_748_609_397_887, 9),   // 127
                (9_223_372_036_854_775_808, 9),   // 128
                (9_892_530_380_752_880_769, 9),   // 129
                (10_604_499_373_000_000_000, 9),  // 130
                (11_361_656_654_439_817_571, 9),  // 131
                (12_166_492_167_065_567_232, 9),  // 132
                (13_021_612_539_908_538_853, 9),  // 133
                (13_929_745_610_903_012_864, 9),  // 134
                (14_893_745_087_865_234_375, 9),  // 135
                (15_916_595_351_771_938_816, 9),  // 136
                (17_001_416_405_572_203_977, 9),  // 137
                (18_151_468_971_815_029_248, 9),  // 138
                (139_353_667_211_683_681, 8),     // 139
                (147_578_905_600_000_000, 8),     // 140
                (156_225_851_787_813_921, 8),     // 141
                (165_312_903_998_914_816, 8),     // 142
                (174_859_124_550_883_201, 8),     // 143
                (184_884_258_895_036_416, 8),     // 144
                (195_408_755_062_890_625, 8),     // 145
                (206_453_783_524_884_736, 8),     // 146
                (218_041_257_467_152_161, 8),     // 147
                (230_193_853_492_166_656, 8),     // 148
                (242_935_032_749_128_801, 8),     // 149
                (256_289_062_500_000_000, 8),     // 150
                (270_281_038_127_131_201, 8),     // 151
                (284_936_905_588_473_856, 8),     // 152
                (300_283_484_326_400_961, 8),     // 153
                (316_348_490_636_206_336, 8),     // 154
                (333_160_561_500_390_625, 8),     // 155
                (350_749_278_894_882_816, 8),     // 156
                (369_145_194_573_386_401, 8),     // 157
                (388_379_855_336_079_616, 8),     // 158
                (408_485_828_788_939_521, 8),     // 159
                (429_496_729_600_000_000, 8),     // 160
                (451_447_246_258_894_081, 8),     // 161
                (474_373_168_346_071_296, 8),     // 162
                (498_311_414_318_121_121, 8),     // 163
                (523_300_059_815_673_856, 8),     // 164
                (549_378_366_500_390_625, 8),     // 165
                (576_586_811_427_594_496, 8),     // 166
                (604_967_116_961_135_041, 8),     // 167
                (634_562_281_237_118_976, 8),     // 168
                (665_416_609_183_179_841, 8),     // 169
                (697_575_744_100_000_000, 8),     // 170
                (731_086_699_811_838_561, 8),     // 171
                (765_997_893_392_859_136, 8),     // 172
                (802_359_178_476_091_681, 8),     // 173
                (840_221_879_151_902_976, 8),     // 174
                (879_638_824_462_890_625, 8),     // 175
                (920_664_383_502_155_776, 8),     // 176
                (963_354_501_121_950_081, 8),     // 177
                (1_007_766_734_259_732_736, 8),   // 178
                (1_053_960_288_888_713_761, 8),   // 179
                (1_101_996_057_600_000_000, 8),   // 180
                (1_151_936_657_823_500_641, 8),   // 181
                (1_203_846_470_694_789_376, 8),   // 182
                (1_257_791_680_575_160_641, 8),   // 183
                (1_313_840_315_232_157_696, 8),   // 184
                (1_372_062_286_687_890_625, 8),   // 185
                (1_432_529_432_742_502_656, 8),   // 186
                (1_495_315_559_180_183_521, 8),   // 187
                (1_560_496_482_665_168_896, 8),   // 188
                (1_628_150_074_335_205_281, 8),   // 189
                (1_698_356_304_100_000_000, 8),   // 190
                (1_771_197_285_652_216_321, 8),   // 191
                (1_846_757_322_198_614_016, 8),   // 192
                (1_925_122_952_918_976_001, 8),   // 193
                (2_006_383_000_160_502_016, 8),   // 194
                (2_090_628_617_375_390_625, 8),   // 195
                (2_177_953_337_809_371_136, 8),   // 196
                (2_268_453_123_948_987_361, 8),   // 197
                (2_362_226_417_735_475_456, 8),   // 198
                (2_459_374_191_553_118_401, 8),   // 199
                (2_560_000_000_000_000_000, 8),   // 200
                (2_664_210_032_449_121_601, 8),   // 201
                (2_772_113_166_407_885_056, 8),   // 202
                (2_883_821_021_683_985_761, 8),   // 203
                (2_999_448_015_365_799_936, 8),   // 204
                (3_119_111_417_625_390_625, 8),   // 205
                (3_242_931_408_352_297_216, 8),   // 206
                (3_371_031_134_626_313_601, 8),   // 207
                (3_503_536_769_037_500_416, 8),   // 208
                (3_640_577_568_861_717_121, 8),   // 209
                (3_782_285_936_100_000_000, 8),   // 210
                (3_928_797_478_390_152_481, 8),   // 211
                (4_080_251_070_798_954_496, 8),   // 212
                (4_236_788_918_503_437_921, 8),   // 213
                (4_398_556_620_369_715_456, 8),   // 214
                (4_565_703_233_437_890_625, 8),   // 215
                (4_738_381_338_321_616_896, 8),   // 216
                (4_916_747_105_530_914_241, 8),   // 217
                (5_100_960_362_726_891_776, 8),   // 218
                (5_291_184_662_917_065_441, 8),   // 219
                (5_487_587_353_600_000_000, 8),   // 220
                (5_690_339_646_868_044_961, 8),   // 221
                (5_899_616_690_476_974_336, 8),   // 222
                (6_115_597_639_891_380_481, 8),   // 223
                (6_338_465_731_314_712_576, 8),   // 224
                (6_568_408_355_712_890_625, 8),   // 225
                (6_805_617_133_840_466_176, 8),   // 226
                (7_050_287_992_278_341_281, 8),   // 227
                (7_302_621_240_492_097_536, 8),   // 228
                (7_562_821_648_920_027_361, 8),   // 229
                (7_831_098_528_100_000_000, 8),   // 230
                (8_107_665_808_844_335_041, 8),   // 231
                (8_392_742_123_471_896_576, 8),   // 232
                (8_686_550_888_106_661_441, 8),   // 233
                (8_989_320_386_052_055_296, 8),   // 234
                (9_301_283_852_250_390_625, 8),   // 235
                (9_622_679_558_836_781_056, 8),   // 236
                (9_953_750_901_796_946_721, 8),   // 237
                (10_294_746_488_738_365_696, 8),  // 238
                (10_645_920_227_784_266_881, 8),  // 239
                (11_007_531_417_600_000_000, 8),  // 240
                (11_379_844_838_561_358_721, 8),  // 241
                (11_763_130_845_074_473_216, 8),  // 242
                (12_157_665_459_056_928_801, 8),  // 243
                (12_563_730_464_589_807_616, 8),  // 244
                (12_981_613_503_750_390_625, 8),  // 245
                (13_411_608_173_635_297_536, 8),  // 246
                (13_854_014_124_583_882_561, 8),  // 247
                (14_309_137_159_611_744_256, 8),  // 248
                (14_777_289_335_064_248_001, 8),  // 249
                (15_258_789_062_500_000_000, 8),  // 250
                (15_753_961_211_814_252_001, 8),  // 251
                (16_263_137_215_612_256_256, 8),  // 252
                (16_786_655_174_842_630_561, 8),  // 253
                (17_324_859_965_700_833_536, 8),  // 254
                (17_878_103_347_812_890_625, 8),  // 255
                (72_057_594_037_927_936, 7),      // 256
            ];

            let (base, power) = BASES[radix as usize];
            (base as BigDigit, power)
        }
        _ => panic!("Invalid bigdigit size"),
    }
}

#[cfg(not(feature = "u64_digit"))]
#[test]
fn test_from_slice() {
    fn check(slice: &[u32], data: &[BigDigit]) {
        assert_eq!(&BigUint::from_slice(slice).data[..], data);
    }
    check(&[1], &[1]);
    check(&[0, 0, 0], &[]);
    check(&[1, 2, 0, 0], &[1, 2]);
    check(&[0, 0, 1, 2], &[0, 0, 1, 2]);
    check(&[0, 0, 1, 2, 0, 0], &[0, 0, 1, 2]);
    check(&[-1i32 as u32], &[-1i32 as BigDigit]);
}

#[cfg(feature = "u64_digit")]
#[test]
fn test_from_slice() {
    fn check(slice: &[u32], data: &[BigDigit]) {
        assert_eq!(
            &BigUint::from_slice(slice).data[..],
            data,
            "from {:?}, to {:?}",
            slice,
            data
        );
    }
    check(&[1], &[1]);
    check(&[0, 0, 0], &[]);
    check(&[1, 2], &[8_589_934_593]);
    check(&[1, 2, 0, 0], &[8_589_934_593]);
    check(&[0, 0, 1, 2], &[0, 8_589_934_593]);
    check(&[0, 0, 1, 2, 0, 0], &[0, 8_589_934_593]);
    check(&[-1i32 as u32], &[(-1i32 as u32) as BigDigit]);
}

#[test]
fn test_from_slice_native() {
    fn check(slice: &[BigDigit], data: &[BigDigit]) {
        assert!(&BigUint::from_slice_native(slice).data[..] == data);
    }
    check(&[1], &[1]);
    check(&[0, 0, 0], &[]);
    check(&[1, 2, 0, 0], &[1, 2]);
    check(&[0, 0, 1, 2], &[0, 0, 1, 2]);
    check(&[0, 0, 1, 2, 0, 0], &[0, 0, 1, 2]);
    check(&[-1i32 as BigDigit], &[-1i32 as BigDigit]);
}

#[test]
fn test_assign_from_slice_native() {
    fn check(slice: &[BigDigit], data: &[BigDigit]) {
        let mut p = BigUint::from_slice_native(&[2627, 0, 9182, 42]);
        p.assign_from_slice_native(slice);
        assert!(&p.data[..] == data);
    }
    check(&[1], &[1]);
    check(&[0, 0, 0], &[]);
    check(&[1, 2, 0, 0], &[1, 2]);
    check(&[0, 0, 1, 2], &[0, 0, 1, 2]);
    check(&[0, 0, 1, 2, 0, 0], &[0, 0, 1, 2]);
    check(&[-1i32 as BigDigit], &[-1i32 as BigDigit]);
}

#[cfg(has_i128)]
#[test]
fn test_u32_u128() {
    assert_eq!(u32_from_u128(0u128), (0, 0, 0, 0));
    assert_eq!(
        u32_from_u128(u128::max_value()),
        (
            u32::max_value(),
            u32::max_value(),
            u32::max_value(),
            u32::max_value()
        )
    );

    assert_eq!(
        u32_from_u128(u32::max_value() as u128),
        (0, 0, 0, u32::max_value())
    );

    assert_eq!(
        u32_from_u128(u64::max_value() as u128),
        (0, 0, u32::max_value(), u32::max_value())
    );

    assert_eq!(
        u32_from_u128((u64::max_value() as u128) + u32::max_value() as u128),
        (0, 1, 0, u32::max_value() - 1)
    );

    assert_eq!(u32_from_u128(36_893_488_151_714_070_528), (0, 2, 1, 0));
}

#[cfg(has_i128)]
#[test]
fn test_u128_u32_roundtrip() {
    // roundtrips
    let values = vec![
        0u128,
        1u128,
        u64::max_value() as u128 * 3,
        u32::max_value() as u128,
        u64::max_value() as u128,
        (u64::max_value() as u128) + u32::max_value() as u128,
        u128::max_value(),
    ];

    for val in &values {
        let (a, b, c, d) = u32_from_u128(*val);
        assert_eq!(u32_to_u128(a, b, c, d), *val);
    }
}

// Mod Inverse

impl<'a> ModInverse<&'a BigUint> for BigUint {
    type Output = BigInt;
    fn mod_inverse(self, m: &'a BigUint) -> Option<BigInt> {
        mod_inverse(Cow::Owned(self), Cow::Borrowed(m))
    }
}

impl ModInverse<BigUint> for BigUint {
    type Output = BigInt;
    fn mod_inverse(self, m: BigUint) -> Option<BigInt> {
        mod_inverse(Cow::Owned(self), Cow::Owned(m))
    }
}

impl<'a> ModInverse<&'a BigInt> for BigUint {
    type Output = BigInt;
    fn mod_inverse(self, m: &'a BigInt) -> Option<BigInt> {
        mod_inverse(Cow::Owned(self), Cow::Owned(m.to_biguint().unwrap()))
    }
}
impl ModInverse<BigInt> for BigUint {
    type Output = BigInt;
    fn mod_inverse(self, m: BigInt) -> Option<BigInt> {
        mod_inverse(Cow::Owned(self), Cow::Owned(m.into_biguint().unwrap()))
    }
}

impl<'a, 'b> ModInverse<&'b BigUint> for &'a BigUint {
    type Output = BigInt;

    fn mod_inverse(self, m: &'b BigUint) -> Option<BigInt> {
        mod_inverse(Cow::Borrowed(self), Cow::Borrowed(m))
    }
}

impl<'a> ModInverse<BigUint> for &'a BigUint {
    type Output = BigInt;

    fn mod_inverse(self, m: BigUint) -> Option<BigInt> {
        mod_inverse(Cow::Borrowed(self), Cow::Owned(m))
    }
}

impl<'a, 'b> ModInverse<&'b BigInt> for &'a BigUint {
    type Output = BigInt;

    fn mod_inverse(self, m: &'b BigInt) -> Option<BigInt> {
        mod_inverse(Cow::Borrowed(self), Cow::Owned(m.to_biguint().unwrap()))
    }
}

// Extended GCD

impl<'a> ExtendedGcd<&'a BigUint> for BigUint {
    fn extended_gcd(self, other: &'a BigUint) -> (BigInt, BigInt, BigInt) {
        let (a, b, c) = extended_gcd(Cow::Owned(self), Cow::Borrowed(other), true);
        (a, b.unwrap(), c.unwrap())
    }
}

impl<'a> ExtendedGcd<&'a BigInt> for BigUint {
    fn extended_gcd(self, other: &'a BigInt) -> (BigInt, BigInt, BigInt) {
        let (a, b, c) = extended_gcd(
            Cow::Owned(self),
            Cow::Owned(other.to_biguint().unwrap()),
            true,
        );
        (a, b.unwrap(), c.unwrap())
    }
}

impl<'a, 'b> ExtendedGcd<&'b BigInt> for &'a BigUint {
    fn extended_gcd(self, other: &'b BigInt) -> (BigInt, BigInt, BigInt) {
        let (a, b, c) = extended_gcd(
            Cow::Borrowed(self),
            Cow::Owned(other.to_biguint().unwrap()),
            true,
        );
        (a, b.unwrap(), c.unwrap())
    }
}

impl<'a, 'b> ExtendedGcd<&'b BigUint> for &'a BigUint {
    fn extended_gcd(self, other: &'b BigUint) -> (BigInt, BigInt, BigInt) {
        let (a, b, c) = extended_gcd(Cow::Borrowed(self), Cow::Borrowed(other), true);
        (a, b.unwrap(), c.unwrap())
    }
}

#[test]
fn test_set_digit() {
    let mut a = BigUint::new(vec![3]);
    a.set_digit(4);
    assert_eq!(a.data.len(), 1);
    assert_eq!(a.data[0], 4);

    let mut a = BigUint::new(vec![3, 2]);
    a.set_digit(4);
    assert_eq!(a.data.len(), 1);
    assert_eq!(a.data[0], 4);

    let mut a = BigUint::new(vec![]);
    a.set_digit(4);
    assert_eq!(a.data.len(), 1);
    assert_eq!(a.data[0], 4);
}

// arbitrary support
#[cfg(feature = "fuzz")]
impl arbitrary::Arbitrary<'_> for BigUint {
    fn arbitrary(src: &mut arbitrary::Unstructured<'_>) -> arbitrary::Result<Self> {
        let data = SmallVec::arbitrary(src)?;
        Ok(Self { data })
    }

    fn size_hint(depth: usize) -> (usize, Option<usize>) {
        SmallVec::<[BigDigit; VEC_SIZE]>::size_hint(depth)
    }
}
