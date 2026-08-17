use alloc::borrow::Cow;

use num_traits::{One, Signed};

use crate::algorithms::extended_gcd;
use crate::{BigInt, BigUint};

/// Calculate the modular inverse of `g`.
/// Implementation is based on the naive version from wikipedia.
#[inline]
pub fn mod_inverse(g: Cow<BigUint>, n: Cow<BigUint>) -> Option<BigInt> {
    let (d, x, _) = extended_gcd(g, n.clone(), true);

    if !d.is_one() {
        return None;
    }

    let x = x.unwrap();

    if x.is_negative() {
        Some(x + n.as_ref())
    } else {
        Some(x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use alloc::vec;

    use num_traits::FromPrimitive;

    use crate::integer::Integer;
    use crate::traits::ModInverse;

    #[test]
    fn test_mod_inverse() {
        let tests = [
            ["1234567", "458948883992"],
            [
                "239487239847",
                "2410312426921032588552076022197566074856950548502459942654116941958108831682612228890093858261341614673227141477904012196503648957050582631942730706805009223062734745341073406696246014589361659774041027169249453200378729434170325843778659198143763193776859869524088940195577346119843545301547043747207749969763750084308926339295559968882457872412993810129130294592999947926365264059284647209730384947211681434464714438488520940127459844288859336526896320919633919",
            ],
            ["-10", "13"],
            ["-6193420858199668535", "2881"],
        ];

        for test in &tests {
            let element = BigInt::parse_bytes(test[0].as_bytes(), 10).unwrap();
            let modulus = BigInt::parse_bytes(test[1].as_bytes(), 10).unwrap();

            //println!("{} modinv {}", element, modulus);
            let inverse = element.clone().mod_inverse(&modulus).unwrap();
            //println!("inverse: {}", &inverse);
            let cmp = (inverse * &element).mod_floor(&modulus);

            assert_eq!(
                cmp,
                BigInt::one(),
                "mod_inverse({}, {}) * {} % {} = {}, not 1",
                &element,
                &modulus,
                &element,
                &modulus,
                &cmp
            );
        }

        // exhaustive tests for small numbers
        for n in 2..100 {
            let modulus = BigInt::from_u64(n).unwrap();
            for x in 1..n {
                for sign in vec![1i64, -1i64] {
                    let element = BigInt::from_i64(sign * x as i64).unwrap();
                    let gcd = element.gcd(&modulus);

                    if !gcd.is_one() {
                        continue;
                    }

                    let inverse = element.clone().mod_inverse(&modulus).unwrap();
                    let cmp = (&inverse * &element).mod_floor(&modulus);
                    //println!("inverse: {}", &inverse);
                    assert_eq!(
                        cmp,
                        BigInt::one(),
                        "mod_inverse({}, {}) * {} % {} = {}, not 1",
                        &element,
                        &modulus,
                        &element,
                        &modulus,
                        &cmp
                    );
                }
            }
        }
    }
}
