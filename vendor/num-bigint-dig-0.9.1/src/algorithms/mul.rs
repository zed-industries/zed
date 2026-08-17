use crate::algorithms::mac3;
use crate::big_digit::{BigDigit, DoubleBigDigit, BITS};
use crate::BigUint;

#[inline]
pub fn mul_with_carry(a: BigDigit, b: BigDigit, acc: &mut DoubleBigDigit) -> BigDigit {
    *acc += (a as DoubleBigDigit) * (b as DoubleBigDigit);
    let lo = *acc as BigDigit;
    *acc >>= BITS;
    lo
}

pub fn mul3(x: &[BigDigit], y: &[BigDigit]) -> BigUint {
    let len = x.len() + y.len() + 1;
    let mut prod = BigUint {
        data: smallvec![0; len],
    };

    mac3(&mut prod.data[..], x, y);
    prod.normalized()
}

pub fn scalar_mul(a: &mut [BigDigit], b: BigDigit) -> BigDigit {
    let mut carry = 0;
    for a in a.iter_mut() {
        *a = mul_with_carry(*a, b, &mut carry);
    }
    carry as BigDigit
}
