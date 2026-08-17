extern crate num_bigint_dig as num_bigint;
extern crate num_traits;

use crate::num_bigint::BigInt;
use crate::num_bigint::Sign::Plus;
use num_traits::{Signed, ToPrimitive, Zero};

use std::ops::Neg;

mod consts;
use crate::consts::*;

#[macro_use]
mod macros;

#[test]
fn test_scalar_add() {
    fn check(x: &BigInt, y: &BigInt, z: &BigInt) {
        let (x, y, z) = (x.clone(), y.clone(), z.clone());
        assert_signed_scalar_op!(x + y == z);
    }

    for elm in SUM_TRIPLES.iter() {
        let (a_vec, b_vec, c_vec) = *elm;
        let a = BigInt::from_slice(Plus, a_vec);
        let b = BigInt::from_slice(Plus, b_vec);
        let c = BigInt::from_slice(Plus, c_vec);
        let (na, nb, nc) = (-&a, -&b, -&c);

        check(&a, &b, &c);
        check(&b, &a, &c);
        check(&c, &na, &b);
        check(&c, &nb, &a);
        check(&a, &nc, &nb);
        check(&b, &nc, &na);
        check(&na, &nb, &nc);
        check(&a, &na, &Zero::zero());
    }
}

#[test]
fn test_scalar_sub() {
    fn check(x: &BigInt, y: &BigInt, z: &BigInt) {
        let (x, y, z) = (x.clone(), y.clone(), z.clone());
        assert_signed_scalar_op!(x - y == z);
    }

    for elm in SUM_TRIPLES.iter() {
        let (a_vec, b_vec, c_vec) = *elm;
        let a = BigInt::from_slice(Plus, a_vec);
        let b = BigInt::from_slice(Plus, b_vec);
        let c = BigInt::from_slice(Plus, c_vec);
        let (na, nb, nc) = (-&a, -&b, -&c);

        check(&c, &a, &b);
        check(&c, &b, &a);
        check(&nb, &a, &nc);
        check(&na, &b, &nc);
        check(&b, &na, &c);
        check(&a, &nb, &c);
        check(&nc, &na, &nb);
        check(&a, &a, &Zero::zero());
    }
}

#[test]
fn test_scalar_mul() {
    fn check(x: &BigInt, y: &BigInt, z: &BigInt) {
        let (x, y, z) = (x.clone(), y.clone(), z.clone());
        assert_signed_scalar_op!(x * y == z);
    }

    for elm in MUL_TRIPLES.iter() {
        let (a_vec, b_vec, c_vec) = *elm;
        let a = BigInt::from_slice(Plus, a_vec);
        let b = BigInt::from_slice(Plus, b_vec);
        let c = BigInt::from_slice(Plus, c_vec);
        let (na, nb, nc) = (-&a, -&b, -&c);

        check(&a, &b, &c);
        check(&b, &a, &c);
        check(&na, &nb, &c);

        check(&na, &b, &nc);
        check(&nb, &a, &nc);
    }
}

#[test]
fn test_scalar_div_rem() {
    fn check_sub(a: &BigInt, b: u32, ans_q: &BigInt, ans_r: &BigInt) {
        let (q, r) = (a / b, a % b);
        if !r.is_zero() {
            assert_eq!(r.sign(), a.sign());
        }
        assert!(r.abs() <= From::from(b));
        assert!(*a == b * &q + &r);
        assert!(q == *ans_q);
        assert!(r == *ans_r);

        let (a, b, ans_q, ans_r) = (a.clone(), b.clone(), ans_q.clone(), ans_r.clone());
        assert_op!(a / b == ans_q);
        assert_op!(a % b == ans_r);

        if b <= i32::max_value() as u32 {
            let nb = -(b as i32);
            assert_op!(a / nb == -ans_q.clone());
            assert_op!(a % nb == ans_r);
        }
    }

    fn check(a: &BigInt, b: u32, q: &BigInt, r: &BigInt) {
        check_sub(a, b, q, r);
        check_sub(&a.neg(), b, &q.neg(), &r.neg());
    }

    for elm in MUL_TRIPLES.iter() {
        let (a_vec, b_vec, c_vec) = *elm;
        let a = BigInt::from_slice(Plus, a_vec);
        let b = BigInt::from_slice(Plus, b_vec);
        let c = BigInt::from_slice(Plus, c_vec);

        if a_vec.len() == 1 && a_vec[0] != 0 {
            let a = a_vec[0];
            check(&c, a, &b, &Zero::zero());
        }

        if b_vec.len() == 1 && b_vec[0] != 0 {
            let b = b_vec[0];
            check(&c, b, &a, &Zero::zero());
        }
    }

    for elm in DIV_REM_QUADRUPLES.iter() {
        let (a_vec, b_vec, c_vec, d_vec) = *elm;
        let a = BigInt::from_slice(Plus, a_vec);
        let c = BigInt::from_slice(Plus, c_vec);
        let d = BigInt::from_slice(Plus, d_vec);

        if b_vec.len() == 1 && b_vec[0] != 0 {
            let b = b_vec[0];
            check(&a, b, &c, &d);
        }
    }
}
