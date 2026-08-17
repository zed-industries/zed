use alloc::borrow::Cow;
use core::iter::repeat;

use smallvec::SmallVec;

use crate::big_digit::BITS;
use crate::BigUint;

#[inline]
pub fn biguint_shl(n: Cow<BigUint>, bits: usize) -> BigUint {
    let n_unit = bits / BITS;
    let mut data = match n_unit {
        0 => n.into_owned().data,
        _ => {
            let len = n_unit + n.data.len() + 1;
            let mut data = SmallVec::with_capacity(len);
            data.extend(repeat(0).take(n_unit));
            data.extend(n.data.iter().cloned());
            data
        }
    };

    let n_bits = bits % BITS;
    if n_bits > 0 {
        let mut carry = 0;
        for elem in data[n_unit..].iter_mut() {
            let new_carry = *elem >> (BITS - n_bits);
            *elem = (*elem << n_bits) | carry;
            carry = new_carry;
        }
        if carry != 0 {
            data.push(carry);
        }
    }

    BigUint::new_native(data)
}
