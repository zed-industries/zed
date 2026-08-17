//! Equivalence tests between `num-bigint` and `crypto-bigint`

use crypto_bigint::{Encoding, U256};
use num_bigint::BigUint;
use num_traits::identities::Zero;
use proptest::prelude::*;
use std::mem;

/// Example prime number (NIST P-256 curve order)
const P: U256 =
    U256::from_be_hex("ffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632551");

fn to_biguint(uint: &U256) -> BigUint {
    BigUint::from_bytes_le(uint.to_le_bytes().as_ref())
}

fn to_uint(big_uint: BigUint) -> U256 {
    let mut input = [0u8; U256::BYTE_SIZE];
    let encoded = big_uint.to_bytes_le();
    let l = encoded.len().min(U256::BYTE_SIZE);
    input[..l].copy_from_slice(&encoded[..l]);

    U256::from_le_slice(&input)
}

prop_compose! {
    fn uint()(bytes in any::<[u8; 32]>()) -> U256 {
        U256::from_le_slice(&bytes)
    }
}
prop_compose! {
    fn uint_mod_p(p: U256)(a in uint()) -> U256 {
        a.wrapping_rem(&p)
    }
}

proptest! {
    #[test]
    fn roundtrip(a in uint()) {
        assert_eq!(a, to_uint(to_biguint(&a)));
    }

    #[test]
    fn wrapping_add(a in uint(), b in uint()) {
        let a_bi = to_biguint(&a);
        let b_bi = to_biguint(&b);

        let expected = to_uint(a_bi + b_bi);
        let actual = a.wrapping_add(&b);

        assert_eq!(expected, actual);
    }

    #[test]
    fn add_mod_nist_p256(a in uint_mod_p(P), b in uint_mod_p(P)) {
        assert!(a < P);
        assert!(b < P);

        let a_bi = to_biguint(&a);
        let b_bi = to_biguint(&b);
        let p_bi = to_biguint(&P);

        let expected = to_uint((a_bi + b_bi) % p_bi);
        let actual = a.add_mod(&b, &P);

        assert!(expected < P);
        assert!(actual < P);

        assert_eq!(expected, actual);
    }

    #[test]
    fn sub_mod_nist_p256(mut a in uint_mod_p(P), mut b in uint_mod_p(P)) {
        if b > a {
            mem::swap(&mut a, &mut b);
        }

        assert!(a < P);
        assert!(b < P);

        let a_bi = to_biguint(&a);
        let b_bi = to_biguint(&b);
        let p_bi = to_biguint(&P);

        let expected = to_uint((a_bi - b_bi) % p_bi);
        let actual = a.sub_mod(&b, &P);

        assert!(expected < P);
        assert!(actual < P);

        assert_eq!(expected, actual);
    }

    #[test]
    fn wrapping_sub(mut a in uint(), mut b in uint()) {
        if b > a {
            mem::swap(&mut a, &mut b);
        }

        let a_bi = to_biguint(&a);
        let b_bi = to_biguint(&b);

        let expected = to_uint(a_bi - b_bi);
        let actual = a.wrapping_sub(&b);

        assert_eq!(expected, actual);
    }

    #[test]
    fn wrapping_mul(a in uint(), b in uint()) {
        let a_bi = to_biguint(&a);
        let b_bi = to_biguint(&b);

        let expected = to_uint(a_bi * b_bi);
        let actual = a.wrapping_mul(&b);

        assert_eq!(expected, actual);
    }

    #[test]
    fn wrapping_div(a in uint(), b in uint()) {
        let a_bi = to_biguint(&a);
        let b_bi = to_biguint(&b);

        if !b_bi.is_zero() {
            let expected = to_uint(a_bi / b_bi);
            let actual = a.wrapping_div(&b);

            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn wrapping_rem(a in uint(), b in uint()) {
        let a_bi = to_biguint(&a);
        let b_bi = to_biguint(&b);

        if b_bi.is_zero() {
            let expected = to_uint(a_bi % b_bi);
            let actual = a.wrapping_rem(&b);

            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn wrapping_sqrt(a in uint()) {
        let a_bi = to_biguint(&a);
        let expected = to_uint(a_bi.sqrt());
        let actual = a.wrapping_sqrt();

        assert_eq!(expected, actual);
    }

    #[test]
    fn wrapping_or(a in uint(), b in uint()) {
        let a_bi = to_biguint(&a);
        let b_bi = to_biguint(&b);

        if !b_bi.is_zero() {
            let expected = to_uint(a_bi | b_bi);
            let actual = a.wrapping_or(&b);

            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn wrapping_and(a in uint(), b in uint()) {
        let a_bi = to_biguint(&a);
        let b_bi = to_biguint(&b);

        if !b_bi.is_zero() {
            let expected = to_uint(a_bi & b_bi);
            let actual = a.wrapping_and(&b);

            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn wrapping_xor(a in uint(), b in uint()) {
        let a_bi = to_biguint(&a);
        let b_bi = to_biguint(&b);
        if !b_bi.is_zero() {
            let expected = to_uint(a_bi ^ b_bi);
            let actual = a.wrapping_xor(&b);

            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn encoding(a in uint()) {
        assert_eq!(a, U256::from_be_bytes(a.to_be_bytes()));
        assert_eq!(a, U256::from_le_bytes(a.to_le_bytes()));
    }

    #[test]
    fn encoding_reverse(a in uint()) {
        let mut bytes = a.to_be_bytes();
        bytes.reverse();
        assert_eq!(a, U256::from_le_bytes(bytes));

        let mut bytes = a.to_le_bytes();
        bytes.reverse();
        assert_eq!(a, U256::from_be_bytes(bytes));
}
}
