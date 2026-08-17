//! Implementation of cube-root algorithm

use crate::*;
use num_bigint::BigUint;
use rounding::NonDigitRoundingData;
use stdlib::num::NonZeroU64;


pub(crate) fn impl_cbrt_int_scale(n: &BigInt, scale: i64, ctx: &Context) -> BigDecimal {
    let rounding_data = NonDigitRoundingData {
        sign: n.sign(),
        mode: ctx.rounding_mode(),
    };

    impl_cbrt_uint_scale((n.magnitude(), scale).into(), ctx.precision(), rounding_data)
}

/// implementation of cuberoot - always positive
pub(crate) fn impl_cbrt_uint_scale(
    n: WithScale<&BigUint>,
    precision: NonZeroU64,
    // contains sign and rounding mode
    rounding_data: NonDigitRoundingData,
) -> BigDecimal {
    if n.is_zero() {
        let biguint = BigInt::from_biguint(Sign::Plus, n.value.clone());
        return BigDecimal::new(biguint, n.scale / 3);
    }

    // count number of digits in the decimal
    let integer_digit_count = count_decimal_digits_uint(n.value);

    // extra digits to use for rounding
    let extra_rounding_digit_count = 4;

    // required number of digits for precision and rounding
    let required_precision = precision.get() + extra_rounding_digit_count;
    let required_precision = 3 * required_precision;

    // number of extra zeros to add to end of integer_digits
    let mut exp_shift = required_precision.saturating_sub(integer_digit_count);

    // effective scale after multiplying by 10^exp_shift
    // (we've added that many trailing zeros after)
    let shifted_scale = n.scale + exp_shift as i64;

    let (mut new_scale, remainder) = shifted_scale.div_rem(&3);

    match remainder.cmp(&0) {
        Ordering::Greater => {
            new_scale += 1;
            exp_shift += (3 - remainder) as u64;
        }
        Ordering::Less => {
            exp_shift += remainder.neg() as u64;
        }
        Ordering::Equal => {
        }
    }

    // clone-on-write copy of digits
    let mut integer_digits = stdlib::borrow::Cow::Borrowed(n.value);

    // add required trailing zeros to integer_digits
    if exp_shift > 0 {
        arithmetic::multiply_by_ten_to_the_uint(
            integer_digits.to_mut(), exp_shift
        );
    }

    let result_digits = integer_digits.nth_root(3);
    let result_digits_count = count_decimal_digits_uint(&result_digits);
    debug_assert!(result_digits_count > precision.get());

    let digits_to_trim = result_digits_count - precision.get();
    debug_assert_ne!(digits_to_trim, 0);
    debug_assert!((result_digits_count as i64 - count_decimal_digits_uint(&integer_digits) as i64 / 3).abs() < 2);

    new_scale -= digits_to_trim as i64;

    let divisor = ten_to_the_uint(digits_to_trim);
    let (mut result_digits, remainder) = result_digits.div_rem(&divisor);

    let remainder_digits = remainder.to_radix_le(10);
    let insig_digit0;
    let trailing_digits;
    if remainder_digits.len() < digits_to_trim as usize {
        // leading zeros
        insig_digit0 = 0;
        trailing_digits = remainder_digits.as_slice();
    } else {
        let (&d, rest) = remainder_digits.split_last().unwrap();
        insig_digit0 = d;
        trailing_digits = rest;
    }

    let insig_data = rounding::InsigData::from_digit_and_lazy_trailing_zeros(
        rounding_data, insig_digit0, || { trailing_digits.iter().all(Zero::is_zero) }
    );

    // lowest digit to round
    let sig_digit = (&result_digits % 10u8).to_u8().unwrap();
    let rounded_digit = insig_data.round_digit(sig_digit);

    let rounding_term = rounded_digit - sig_digit;
    result_digits += rounding_term;

    let result = BigInt::from_biguint(rounding_data.sign, result_digits);

    BigDecimal::new(result, new_scale)
}


#[cfg(test)]
mod test {
    use super::*;
    use stdlib::num::NonZeroU64;

    macro_rules! impl_test {
        ($name:ident; $input:literal => $expected:literal) => {
            #[test]
            fn $name() {
                let n: BigDecimal = $input.parse().unwrap();
                let value = n.cbrt();

                let expected = $expected.parse().unwrap();
                assert_eq!(value, expected);
            }
        };
        ($name:ident; prec=$prec:literal; round=$round:ident; $input:literal => $expected:literal) => {
            #[test]
            fn $name() {
                let ctx = Context::new(NonZeroU64::new($prec).unwrap(), RoundingMode::$round);
                let n: BigDecimal = $input.parse().unwrap();
                let value = n.cbrt_with_context(&ctx);

                let expected = $expected.parse().unwrap();
                assert_eq!(value, expected);
            }
        };
    }

    mod default {
        use super::*;

        impl_test!(case_0; "0.00" => "0");
        impl_test!(case_1; "1.00" => "1");
        impl_test!(case_1d001; "1.001" => "1.000333222283909495175449559955220102010284758197360454054345461242739715702641939155238095670636841");
        impl_test!(case_10; "10" => "2.154434690031883721759293566519350495259344942192108582489235506346411106648340800185441503543243276");
        impl_test!(case_13409d179789484375; "13409.179789484375" => "23.7575");
        impl_test!(case_n59283293e25; "-59283293e25" => "-84006090355.84281237113712383191213626687332139035750444925827809487776780721673264524620270275301685");
        impl_test!(case_94213372931en127; "94213372931e-127" => "2.112049945275324414051072540210070583697242797173805198575907094646677475250362108901530353886613160E-39");
    }

    impl_test!(case_prec15_down_10; prec=15; round=Down; "10" => "2.15443469003188");
    impl_test!(case_prec6_up_0d979970546636727; prec=6; round=Up; "0.979970546636727" => "0.993279");

    impl_test!(case_1037d495615705321421375_full; "1037.495615705321421375" => "10.123455");
    impl_test!(case_1037d495615705321421375_prec7_halfdown; prec=7; round=HalfDown; "1037.495615705321421375" => "10.12345");
    impl_test!(case_1037d495615705321421375_prec7_halfeven; prec=7; round=HalfEven; "1037.495615705321421375" => "10.12346");
    impl_test!(case_1037d495615705321421375_prec7_halfup; prec=7; round=HalfUp; "1037.495615705321421375" => "10.12346");

    impl_test!(case_0d014313506928855520728400001_full; "0.014313506928855520728400001" => "0.242800001");
    impl_test!(case_0d014313506928855520728400001_prec6_down; prec=6; round=Down; "0.014313506928855520728400001" => "0.242800");
    impl_test!(case_0d014313506928855520728400001_prec6_up; prec=6; round=Up; "0.014313506928855520728400001" => "0.242801");

    impl_test!(case_4151902e20_prec16_halfup; prec=16; round=HalfUp; "4151902e20" => "746017527.6855992");
    impl_test!(case_4151902e20_prec16_up; prec=16; round=Up; "4151902e20" => "746017527.6855993");
    impl_test!(case_4151902e20_prec17_up; prec=17; round=Up; "4151902e20" => "746017527.68559921");
    impl_test!(case_4151902e20_prec18_up; prec=18; round=Up; "4151902e20" => "746017527.685599209");
    // impl_test!(case_4151902e20_prec18_up; prec=18; round=Up; "4151902e20" => "746017527.685599209");

    impl_test!(case_1850846e201_prec14_up; prec=16; round=Up; "1850846e201" => "1.227788123885769e69");

    impl_test!(case_6d3797558642427987505823530913e85_prec16_up; prec=160; round=Up; "6.3797558642427987505823530913E+85" => "3995778017e19");

    impl_test!(case_88573536600476899341824_prec20_up; prec=20; round=Up; "88573536600476899341824" => "44576024");
    impl_test!(case_88573536600476899341824_prec7_up; prec=7; round=Up; "88573536600476899341824" => "4457603e1");

    impl_test!(case_833636d150970875_prec5_up; prec=5; round=Up; "833636.150970875" => "94.115000");
    impl_test!(case_833636d150970875_prec5_halfup; prec=5; round=HalfUp; "833636.150970875" => "94.115");
    impl_test!(case_833636d150970875_prec4_halfup; prec=4; round=HalfUp; "833636.150970875" => "94.12");
    impl_test!(case_833636d150970875_prec20_up; prec=20; round=Up; "833636.150970875" => "94.115000");

    #[cfg(property_tests)]
    mod prop {
        use super::*;
        use proptest::*;
        use num_traits::FromPrimitive;

        proptest! {
            #[test]
            fn cbrt_of_cube_is_self(f: f64, prec in 15..50u64) {
                // ignore non-normal numbers
                prop_assume!(f.is_normal());

                let n = BigDecimal::from_f64(f).unwrap().with_prec(prec);
                let n_cubed = n.cube();
                let x = n_cubed.cbrt();
                prop_assert_eq!(x, n);
            }
        }
    }
}
