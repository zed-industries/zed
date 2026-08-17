// Test BigDecimal::with_scale_round

macro_rules! impl_test {
    ( name=$($name:expr)*; $scale:literal : $mode:ident => $ex:literal ) => {
        paste! {
            #[test]
            fn [< $($name)* _rounding_ $mode >]() {
                let bigdecimal = test_input();
                let result = bigdecimal.with_scale_round($scale as i64, RoundingMode::$mode);
                let expected = BigDecimal::from_str($ex).unwrap();
                assert_eq!(result, expected);
                assert_eq!(result.int_val, expected.int_val);
                assert_eq!(result.scale, $scale);
            }
        }
    };
    ( -$scale:literal $( : $($modes:ident),+ => $ex:literal )+ ) => {
        $( $( impl_test!(name=scale_neg_ $scale; -$scale : $modes => $ex); )* )*
    };
    ( $scale:literal $( : $($modes:ident),+ => $ex:literal )+ ) => {
        $( $( impl_test!(name=scale_ $scale; $scale : $modes => $ex); )* )*
    };
}


mod case_3009788271450eNeg9 {
    use super::*;

    fn test_input() -> BigDecimal {
        BigDecimal::from_str("3009.788271450").unwrap()
    }

    impl_test!(10 : Up, Down => "3009.7882714500");
    impl_test!(9 : Up, Down => "3009.788271450");
    impl_test!(8 : Up, Down, HalfEven => "3009.78827145");

    impl_test!(7 : Up, Ceiling, HalfUp => "3009.7882715"
                 : Down, Floor, HalfDown, HalfEven => "3009.7882714");

    impl_test!(4 : Up, Ceiling, HalfUp, HalfDown, HalfEven => "3009.7883"
                 : Down, Floor => "3009.7882");

    impl_test!(2 : Up => "3009.79"
                 : Down => "3009.78");

    impl_test!(1 : Up => "3009.8"
                 : Down => "3009.7");

    impl_test!(0 : Up => "3010"
                 : Down => "3009");

    impl_test!( -1 : Up => "301e1");
    impl_test!( -2 : Up => "31e2");
    impl_test!( -3 : Up => "4e3");
    impl_test!( -4 : Up => "1e4" );
    impl_test!( -5 : Up => "1e5" : Down => "0");
    impl_test!( -20 : Up => "1e20" : Down => "0");
}

mod case_neg_636652287787259 {
    use super::*;

    fn test_input() -> BigDecimal {
        BigDecimal::from_str("-636652287787259").unwrap()
    }

    impl_test!(1 : Up, Down => "-636652287787259.0");
    impl_test!(0 : Up, Down => "-636652287787259");
    impl_test!(-1 : Up => "-63665228778726e1"
                  : Down => "-63665228778725e1");
    impl_test!(-12 : Up => "-637e12"
                  : Down => "-636e12");
}

mod case_99999999999999999999999eNeg4 {
    use super::*;

    fn test_input() -> BigDecimal {
        BigDecimal::from_str("99999999999999999999999e-4").unwrap()
    }

    impl_test!(4 : Up => "9999999999999999999.9999");
    impl_test!(3 : Up => "10000000000000000000.000"
                 : Down => "9999999999999999999.999");
    impl_test!(-3 : Up => "10000000000000000e3"
                  : Down => "9999999999999999e3");
}

mod case_369708962060657eNeg30 {
    use super::*;

    fn test_input() -> BigDecimal {
        BigDecimal::from_str("369708962060657E-30").unwrap()
    }

    impl_test!(4 : Up => "1e-4");
    impl_test!(20 : Up => "36971e-20"
                  : Down => "36970e-20");
}

mod case_682829560896740e30 {
    use super::*;

    fn test_input() -> BigDecimal {
        BigDecimal::from_str("682829560896740e30").unwrap()
    }

    impl_test!(4 : Up => "682829560896740000000000000000000000000000000.0000");
    impl_test!(0 : Up => "682829560896740000000000000000000000000000000");
    impl_test!(-35 : Up => "6828295609e35");
    impl_test!(-36 : Up => "682829561e36");
    impl_test!(-100 : Up => "1e100");
}
