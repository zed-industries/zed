//! inverse implementation

use crate::*;

/// Implementation of inverse: (1/n)
pub(crate) fn impl_inverse_uint_scale(n: &BigUint, scale: i64, ctx: &Context) -> BigDecimal {
    let guess = make_inv_guess(n.bits(), scale);
    let max_precision = ctx.precision().get();

    let s = BigDecimal::new(BigInt::from_biguint(Sign::Plus, n.clone()), scale);
    let two = BigDecimal::from(2);

    let next_iteration = move |r: BigDecimal| {
        let tmp = &two - &s * &r;
        r * tmp
    };

    // calculate first iteration
    let mut running_result = next_iteration(guess);
    debug_assert!(!running_result.is_zero(), "Zero detected in inverse calculation of {}e{}", n, -scale);

    let mut prev_result = BigDecimal::one();
    let mut result = BigDecimal::zero();

    // TODO: Prove that we don't need to arbitrarily limit iterations
    // and that convergence can be calculated
    while prev_result != result {
        // store current result to test for convergence
        prev_result = result;

        // calculate next iteration
        running_result = next_iteration(running_result).with_prec(max_precision + 2);

        // 'result' has clipped precision, 'running_result' has full precision
        result = if running_result.digits() > max_precision {
            running_result.with_precision_round(ctx.precision(), ctx.rounding_mode())
        } else {
            running_result.clone()
        };
    }

    return result;
}


/// guess inverse based on the number of bits in the integer and decimal's scale
fn make_inv_guess(bit_count: u64, scale: i64) -> BigDecimal {
    // scale by ln(2)
    let magic_factor = stdlib::f64::consts::LN_2;

    let bit_count = bit_count as f64;
    let initial_guess = magic_factor * exp2(-bit_count);
    if initial_guess.is_finite() && initial_guess != 0.0 {
        if let Ok(mut result) = BigDecimal::try_from(initial_guess) {
            result.scale -= scale;
            return result;
        }
    }

    // backup guess for out-of-range integers

    let approx_scale = bit_count * stdlib::f64::consts::LOG10_2;
    let approx_scale_int = approx_scale.trunc();
    let approx_scale_frac = approx_scale - approx_scale_int;

    let recip = libm::exp10(-approx_scale_frac);
    let mut res = BigDecimal::from_f32((magic_factor * recip) as f32).unwrap();
    res.scale += approx_scale_int as i64;
    res.scale -= scale;
    return res;
}


#[cfg(test)]
mod test_make_inv_guess {
    use super::*;
    use paste::paste;

    macro_rules! impl_case {
        ( $bin_count:literal, -$scale:literal => $expected:literal ) => {
            paste! { impl_case!( [< case_ $bin_count _n $scale >]: $bin_count, -$scale => $expected); }
        };
        ( $bin_count:literal, $scale:literal => $expected:literal ) => {
            paste! { impl_case!( [< case_ $bin_count _ $scale >]: $bin_count, $scale => $expected); }
        };
        ( $name:ident: $bin_count:expr, $scale:expr => $expected:literal ) => {
            impl_case!($name: $bin_count, $scale, prec=5 => $expected);
        };
        ( $name:ident: $bin_count:expr, $scale:expr, prec=$prec:literal => $expected:literal ) => {
            #[test]
            fn $name() {
                let guess = make_inv_guess($bin_count, $scale);
                let expected: BigDecimal = $expected.parse().unwrap();
                assert_eq!(guess.with_prec($prec), expected.with_prec($prec));
            }
        };
    }

    impl_case!(0, 0 => "0.69315");
    impl_case!(1, 0 => "0.34657");
    impl_case!(2, 0 => "0.17329");
    impl_case!(2, 1 => "1.7329");

    // 1 / (2^3 * 10^5) ~
    impl_case!(3, -5 => "8.6643e-07");

    // 2^-20
    impl_case!(20, 0 => "6.6104e-07");
    impl_case!(20, -900 => "6.6104E-907");
    impl_case!(20, 800 => "6.6104E+793");

    impl_case!(40, 10000 => "6.3041E+9987");

    impl_case!(70, -5 => "5.8712e-27");
    impl_case!(70, 5 => "5.8712e-17");
    impl_case!(70, 50 => "5.8712e+28");

    impl_case!(888, -300 => "3.3588E-568");
    impl_case!(888, -19 => "3.3588E-287");
    impl_case!(888, 0 => "3.3588E-268");
    impl_case!(888, 270 => "335.88");

    impl_case!(1022, 10 => "1.5423e-298");
    impl_case!(1022, 308 => "1.5423");

    impl_case!(1038, 316 => "2353.4");

    impl_case!(case_31028_n659: 31028, -659 => "3.0347E-10000");
    impl_case!(case_31028_0: 31028, 0 => "3.0347E-9341");
    impl_case!(case_31028_1: 31028, 1 => "3.0347E-9340");
    impl_case!(case_31028_9340: 31028, 9340 => ".30347");
    impl_case!(case_31028_10000: 31028, 10000 => "3.0347E+659");

    // impl_case!(case_max: u64::MAX, 270 => "335.88");
}

#[cfg(test)]
mod test {
    use super::*;
    use stdlib::num::NonZeroU64;

    #[test]
    fn test_inverse_35543972957198043e291() {
        let v = vec![
            0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
            2324389888, 849200558
        ];
        let x = BigInt::new(Sign::Minus, v);
        let d = BigDecimal::from(x);
        let expected = "-2.813416500187520746852694701086705659180043761702417561798711758892800449936819185796527214192677476E-308".parse().unwrap();
        assert_eq!(d.inverse(), expected);

        assert_eq!(d.neg().inverse(), expected.neg());
    }

    macro_rules! impl_case {
        ($name:ident: $prec:literal, $round:ident => $expected:literal) => {
            #[test]
            fn $name() {
                let n = test_input();
                let prec = NonZeroU64::new($prec).unwrap();
                let rounding = RoundingMode::$round;
                let ctx = Context::new(prec, rounding);

                let result = n.inverse_with_context(&ctx);

                let expected = $expected.parse().unwrap();
                assert_eq!(&result, &expected);

                let product = result * &n;
                let epsilon = BigDecimal::new(BigInt::one(), $prec - 1);
                let diff = (BigDecimal::one() - &product).abs();
                assert!(diff < epsilon);
            }
        };
    }

    mod invert_seven {
        use super::*;

        fn test_input() -> BigDecimal {
            BigDecimal::from(7u8)
        }

        impl_case!(case_prec10_round_down: 10, Down => "0.1428571428");
        impl_case!(case_prec10_round_up: 10, Up => "0.1428571429");

        impl_case!(case_prec11_round_ceiling: 11, Ceiling => "0.14285714286");
    }


    #[test]
    fn inv_random_number() {
       let n = BigDecimal::try_from(0.08121970592310568).unwrap();

       let ctx = Context::new(NonZeroU64::new(40).unwrap(), RoundingMode::Down);
       let i = n.inverse_with_context(&ctx);
       assert_eq!(&i, &"12.31228294456944530942557443718279245563".parse().unwrap());

       let product = i * &n;
       assert!(BigDecimal::one() - &product < "1e-39".parse().unwrap());
    }

    #[cfg(property_tests)]
    mod prop {
        use super::*;
        use proptest::*;
        use num_traits::FromPrimitive;

        proptest! {

            #[test]
            fn inverse_multiplies_to_one(f: f64, prec in 1..100u64) {
                // ignore non-normal numbers
                prop_assume!(f.is_normal());
                prop_assume!(f != 0.0);

                let n = BigDecimal::from_f64(f).unwrap();

                let ctx = Context::new(NonZeroU64::new(prec).unwrap(), RoundingMode::Up);
                let i = n.inverse_with_context(&ctx);
                let product = &i * &n;

                // accurate to precision minus one (due to rounding)
                let epsilon = BigDecimal::new(1.into(), prec as i64 - 1);
                let diff_from_one = BigDecimal::one() - &product;

                prop_assert!(diff_from_one.abs() < epsilon, "{} >= {}", diff_from_one.abs(), epsilon);
            }
        }
    }
}
