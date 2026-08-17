//! square root implementation

use crate::*;


pub(crate) fn impl_sqrt(n: &BigUint, scale: i64, ctx: &Context) -> BigDecimal {
    // Calculate the number of digits and the difference compared to the scale
    let num_digits = count_decimal_digits_uint(n);
    let scale_diff = BigInt::from(num_digits) - scale;

    // Calculate the number of wanted digits and the exponent we need to raise the original value to
    // We want twice as many digits as the precision because sqrt halves the number of digits
    // We add an extra one for rounding purposes
    let prec = ctx.precision().get();
    let extra_rounding_digit_count = 5;
    let wanted_digits = 2 * (prec + extra_rounding_digit_count);
    let exponent = wanted_digits.saturating_sub(num_digits) + u64::from(scale_diff.is_odd());
    let sqrt_digits = (n * ten_to_the_uint(exponent)).sqrt();

    // Calculate the scale of the result
    let result_scale_digits = 2 * (2 * prec - scale_diff) - 1;
    let result_scale_decimal: BigDecimal = BigDecimal::new(result_scale_digits, 0) / 4.0;
    let mut result_scale = result_scale_decimal.with_scale_round(0, RoundingMode::HalfEven).int_val;

    // Round the value so it has the correct precision requested
    result_scale += count_decimal_digits_uint(&sqrt_digits).saturating_sub(prec);
    let unrounded_result = BigDecimal::new(sqrt_digits.into(), result_scale.to_i64().unwrap());
    unrounded_result.with_precision_round(ctx.precision(), ctx.rounding_mode())
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! impl_case {
        ($name:ident; $input:literal => $expected:literal) => {
            #[test]
            fn $name() {
                let n: BigDecimal = $input.parse().unwrap();
                let value = n.sqrt().unwrap();

                let expected = $expected.parse().unwrap();
                assert_eq!(value, expected);
                // assert_eq!(value.scale, expected.scale);
            }
        };
        ($name:ident; prec=$prec:literal; round=$round:ident; $input:literal => $expected:literal) => {
            #[test]
            fn $name() {
                let ctx = Context::default()
                                .with_prec($prec).unwrap()
                                .with_rounding_mode(RoundingMode::$round);
                let n: BigDecimal = $input.parse().unwrap();
                let value = n.sqrt_with_context(&ctx).unwrap();

                let expected = $expected.parse().unwrap();
                assert_eq!(value, expected);
                // assert_eq!(value.scale, expected.scale);
            }
        };
    }

    impl_case!(case_0d000; "0.000" => "0");
    impl_case!(case_1en232; "1e-232" => "1e-116");
    impl_case!(case_1d00; "1.00" => "1.00");
    impl_case!(case_1d001; "1.001" => "1.000499875062460964823258287700109753027590031219780479551442971840836093890879944856933288426795152");
    impl_case!(case_100d0; "100" => "10.0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000");
    impl_case!(case_49; "49" => "7.0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000");
    impl_case!(case_d25; ".25" => ".5");
    impl_case!(case_0d0152399025; "0.0152399025" => ".12345");
    impl_case!(case_0d00400; "0.00400" => "0.06324555320336758663997787088865437067439110278650433653715009705585188877278476442688496216758600590");
    impl_case!(case_0d1; "0.1" => "0.3162277660168379331998893544432718533719555139325216826857504852792594438639238221344248108379300295");
    impl_case!(case_152399025; "152399025" => "12345");
    impl_case!(case_2; "2" => "1.414213562373095048801688724209698078569671875376948073176679737990732478462107038850387534327641573");
    impl_case!(case_125348; "125348" => "354.0451948551201563108487193176101314241016013304294520812832530590100407318465590778759640828114535");
    impl_case!(case_121d000242000121; "121.000242000121000000" => "11.000011000");
    impl_case!(case_0d01234567901234567901234567901234567901234567901234567901234567901234567901234567901234567901234567901; "0.01234567901234567901234567901234567901234567901234567901234567901234567901234567901234567901234567901" => "0.1111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111");
    impl_case!(case_2e70; "2e70" => "141421356237309504880168872420969807.8569671875376948073176679737990732478462107038850387534327641573");
    impl_case!(case_8d9793115997963468544185161590576171875en11; "8.9793115997963468544185161590576171875e-11" => "0.000009475922962855041517561783740144225422359796851494316346796373337470068631250135521161989831460407155");
    impl_case!(case_18446744073709551616d1099511; "18446744073709551616.1099511" => "4294967296.000000000012799992691725492477397918722952224079252026972356303360555051219312462698703293");

    impl_case!(case_3d1415926; "3.141592653589793115997963468544185161590576171875" => "1.772453850905515992751519103139248439290428205003682302442979619028063165921408635567477284443197875");
    impl_case!(case_0d71777001; "0.7177700109762963922745342343167413624881759290454997218753321040760896053150388903350654937434826216803814031987652326749140535150336357405672040727695124057298138872112244784753994931999476811850580200000000000000000000000000000000" => "0.8472130847527653667042980517799020703921106560594525833177762276594388966885185567535692987624493813");
    impl_case!(case_0d110889ddd444; "0.1108890000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000444" => "0.3330000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000667");

    impl_case!(case_3e170; "3e170" => "17320508075688772935274463415058723669428052538103806280558069794519330169088000370811.46186757248576");
    impl_case!(case_9e199; "9e199" => "9486832980505137995996680633298155601158665417975650480572514558377783315917714664032744325137900886");
    impl_case!(case_7e200; "7e200" => "2645751311064590590501615753639260425710259183082450180368334459201068823230283627760392886474543611e1");
    impl_case!(case_777e204; "777e204" => "2.787471972953270789531596912111625325974789615194854615319795902911796043681078997362635440358922503E+103");
    impl_case!(case_777e600; "7e600" => "2.645751311064590590501615753639260425710259183082450180368334459201068823230283627760392886474543611E+300");
    impl_case!(case_2e900; "2e900" => "1.414213562373095048801688724209698078569671875376948073176679737990732478462107038850387534327641573E+450");
    impl_case!(case_7e999; "7e999" => "8.366600265340755479781720257851874893928153692986721998111915430804187725943170098308147119649515362E+499");
    impl_case!(case_74908163946345982392040522594123773796e999; "74908163946345982392040522594123773796e999" => "2.736935584670307552030924971360722787091742391079630976117950955395149091570790266754718322365663909E+518");
    impl_case!(case_20e1024; "20e1024" => "4.472135954999579392818347337462552470881236719223051448541794490821041851275609798828828816757564550E512");
    impl_case!(case_3en1025; "3e-1025" => "5.477225575051661134569697828008021339527446949979832542268944497324932771227227338008584361638706258E-513");

    impl_case!(case_3242053850483855en13_prec11_round_down; prec=11; round=Down; "324.2053850483855" => "18.005704236");
    impl_case!(case_3242053850483855en13_prec11_round_up; prec=11; round=Up; "324.2053850483855" => "18.005704237");
    impl_case!(case_3242053850483855en13_prec31_round_up; prec=31; round=Up; "324.2053850483855" => "18.00570423639090823994825477228");

    impl_case!(case_5d085019992340351en10_prec25_round_down; prec=25; round=Down; "5.085019992340351e-10" => "0.00002254998889653906459324292");

    impl_case!(case_3025d13579652399025_prec3_round_up; prec=3; round=Up; "3025.13579652399025" => "55.1");

    impl_case!(case_3025d13579652399025_prec9_round_down; prec=9; round=Down; "3025.13579652399025" => "55.0012345");
    impl_case!(case_3025d13579652399025_prec9_round_up; prec=9; round=Up; "3025.13579652399025" => "55.0012345");

    impl_case!(case_3025d13579652399025_prec8_round_halfdown; prec=8; round=HalfDown; "3025.13579652399025" => "55.001234");
    impl_case!(case_3025d13579652399025_prec8_round_halfeven; prec=8; round=HalfEven; "3025.13579652399025" => "55.001234");
    impl_case!(case_3025d13579652399025_prec8_round_halfup; prec=8; round=HalfUp; "3025.13579652399025" => "55.001235");

    #[test]
    fn test_sqrt_rounding() {
        let vals = vec![
            // sqrt(1.21) = 1.1, [Ceiling, Up] should round up
            ("1.21", "2", "1", "1", "1", "1", "1", "2"),
            // sqrt(2.25) = 1.5, [Ceiling, HalfEven, HalfUp, Up] should round up
            ("2.25", "2", "1", "1", "1", "2", "2", "2"),
            // sqrt(6.25) = 2.5, [Ceiling, HalfUp, Up] should round up
            ("6.25", "3", "2", "2", "2", "2", "3", "3"),
            // sqrt(8.41) = 2.9, [Ceiling, HalfDown, HalfEven, HalfUp, Up] should round up
            ("8.41", "3", "2", "2", "3", "3", "3", "3"),
        ];
        for &(val, ceiling, down, floor, half_down, half_even, half_up, up) in vals.iter() {
            let val = BigDecimal::from_str(val).unwrap();
            let ceiling = BigDecimal::from_str(ceiling).unwrap();
            let down = BigDecimal::from_str(down).unwrap();
            let floor = BigDecimal::from_str(floor).unwrap();
            let half_down = BigDecimal::from_str(half_down).unwrap();
            let half_even = BigDecimal::from_str(half_even).unwrap();
            let half_up = BigDecimal::from_str(half_up).unwrap();
            let up = BigDecimal::from_str(up).unwrap();
            let ctx = Context::default().with_prec(1).unwrap();
            assert_eq!(val.sqrt_with_context(&ctx.with_rounding_mode(RoundingMode::Ceiling)).unwrap(), ceiling);
            assert_eq!(val.sqrt_with_context(&ctx.with_rounding_mode(RoundingMode::Down)).unwrap(), down);
            assert_eq!(val.sqrt_with_context(&ctx.with_rounding_mode(RoundingMode::Floor)).unwrap(), floor);
            assert_eq!(val.sqrt_with_context(&ctx.with_rounding_mode(RoundingMode::HalfDown)).unwrap(), half_down);
            assert_eq!(val.sqrt_with_context(&ctx.with_rounding_mode(RoundingMode::HalfEven)).unwrap(), half_even);
            assert_eq!(val.sqrt_with_context(&ctx.with_rounding_mode(RoundingMode::HalfUp)).unwrap(), half_up);
            assert_eq!(val.sqrt_with_context(&ctx.with_rounding_mode(RoundingMode::Up)).unwrap(), up);
        }
    }

    #[cfg(property_tests)]
    mod prop {
        use super::*;
        use proptest::*;
        use num_traits::FromPrimitive;

        proptest! {
            #[test]
            fn sqrt_of_square_is_self(f: f64, prec in 15..50u64) {
                // ignore non-normal numbers
                prop_assume!(f.is_normal());

                let n = BigDecimal::from_f64(f.abs()).unwrap().with_prec(prec);
                let n_squared = n.square();
                let x = n_squared.sqrt().unwrap();
                prop_assert_eq!(x, n);
            }
        }
    }
}
