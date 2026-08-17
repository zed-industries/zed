#![cfg(feature = "rand")]

extern crate num_bigint_dig as num_bigint;
extern crate num_traits;
extern crate rand;

use crate::num_bigint::RandBigInt;
use num_traits::Zero;
use rand::prelude::*;

fn test_mul_divide_torture_count(count: usize) {
    let bits_max = 1 << 12;
    #[cfg(target_pointer_width = "32")]
    let seed = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    #[cfg(target_pointer_width = "64")]
    let seed = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31, 32,
    ];
    let mut rng = rand::rngs::SmallRng::from_seed(seed);

    for _ in 0..count {
        // Test with numbers of random sizes:
        let xbits = rng.gen_range(0..bits_max);
        let ybits = rng.gen_range(0..bits_max);

        let x = rng.gen_biguint(xbits);
        let y = rng.gen_biguint(ybits);

        if x.is_zero() || y.is_zero() {
            continue;
        }

        let prod = &x * &y;
        assert_eq!(&prod / &x, y);
        assert_eq!(&prod / &y, x);
    }
}

#[test]
fn test_mul_divide_torture() {
    test_mul_divide_torture_count(1000);
}

#[test]
#[ignore]
fn test_mul_divide_torture_long() {
    test_mul_divide_torture_count(1000000);
}
