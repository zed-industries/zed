use alloc::borrow::Cow;

use num_traits::Zero;
use smallvec::SmallVec;

use crate::big_digit::{BigDigit, BITS};
use crate::BigUint;
use crate::VEC_SIZE;

#[inline]
pub fn biguint_shr(n: Cow<BigUint>, bits: usize) -> BigUint {
    let n_unit = bits / BITS;
    if n_unit >= n.data.len() {
        return Zero::zero();
    }
    let mut data: SmallVec<[BigDigit; VEC_SIZE]> = match n {
        Cow::Borrowed(n) => n.data[n_unit..].into(),
        Cow::Owned(n) => n.data[n_unit..].into(),
    };

    let n_bits = bits % BITS;
    if n_bits > 0 {
        let mut borrow = 0;
        for elem in data.iter_mut().rev() {
            let new_borrow = *elem << (BITS - n_bits);
            *elem = (*elem >> n_bits) | borrow;
            borrow = new_borrow;
        }
    }

    BigUint::new_native(data)
}
