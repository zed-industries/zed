
#[allow(non_snake_case)]
mod test_round_pair {
    use paste::paste;
    use super::*;

    macro_rules! impl_test {
        ( $($mode:ident),+ => $expected:literal) => {
            $(
                paste! {
                    #[test]
                    fn [< mode_ $mode >]() {
                        let (pair, sign, trailing_zeros) = test_input();
                        let mode = self::RoundingMode::$mode;
                        let result = mode.round_pair(sign, pair, trailing_zeros);
                        assert_eq!(result, $expected);
                    }
                }
            )*
        }
    }

    macro_rules! define_test_input {
        ( - $lhs:literal . $rhs:literal $($t:tt)* ) => {
            define_test_input!(sign=Sign::Minus, pair=($lhs, $rhs), $($t)*);
        };
        ( $lhs:literal . $rhs:literal $($t:tt)*) => {
            define_test_input!(sign=Sign::Plus, pair=($lhs, $rhs), $($t)*);
        };
        ( sign=$sign:expr, pair=$pair:expr, ) => {
            define_test_input!(sign=$sign, pair=$pair, trailing_zeros=true);
        };
        ( sign=$sign:expr, pair=$pair:expr, 000x ) => {
            define_test_input!(sign=$sign, pair=$pair, trailing_zeros=false);
        };
        ( sign=$sign:expr, pair=$pair:expr, trailing_zeros=$trailing_zeros:literal ) => {
            fn test_input() -> ((u8, u8), Sign, bool) { ($pair, $sign, $trailing_zeros) }
        };
    }

    mod case_0_1 {
        use super::*;

        define_test_input!(0 . 1);

        impl_test!(Up, Ceiling => 1);
        impl_test!(Down, Floor, HalfUp, HalfDown, HalfEven => 0);
    }

    mod case_neg_0_1 {
        use super::*;

        define_test_input!(-0 . 1);

        impl_test!(Up, Floor => 1);
        impl_test!(Down, Ceiling, HalfUp, HalfDown, HalfEven => 0);
    }

    mod case_0_5 {
        use super::*;

        define_test_input!( 0 . 5 );

        impl_test!(Up, Ceiling, HalfUp => 1);
        impl_test!(Down, Floor, HalfDown, HalfEven => 0);
    }

    mod case_neg_0_5 {
        use super::*;

        define_test_input!(-0 . 5);

        impl_test!(Up, Floor, HalfUp => 1);
        impl_test!(Down, Ceiling, HalfDown, HalfEven => 0);
    }

    mod case_0_5_000x {
        use super::*;

        // ...000x indicates a non-zero trailing digit; affects behavior of rounding N.0 and N.5
        define_test_input!(0 . 5 000x);

        impl_test!(Up, Ceiling, HalfUp, HalfDown, HalfEven => 1);
        impl_test!(Down, Floor => 0);
    }

    mod case_neg_0_5_000x {
        use super::*;

        define_test_input!(-0 . 5 000x);

        impl_test!(Up, Floor, HalfUp, HalfDown, HalfEven => 1);
        impl_test!(Down, Ceiling => 0);
    }

    mod case_0_7 {
        use super::*;

        define_test_input!(0 . 7);

        impl_test!(Up, Ceiling, HalfUp, HalfDown, HalfEven => 1);
        impl_test!(Down, Floor => 0);
    }

    mod case_neg_0_7 {
        use super::*;

        define_test_input!(-0 . 7);

        impl_test!(Up, Floor, HalfUp, HalfDown, HalfEven => 1);
        impl_test!(Down, Ceiling => 0);
    }

    mod case_neg_4_3_000x {
        use super::*;

        define_test_input!(-4 . 3 000x);

        impl_test!(Up, Floor => 5);
        impl_test!(Down, Ceiling, HalfUp, HalfDown, HalfEven => 4);
    }


    mod case_9_5_000x {
        use super::*;

        define_test_input!(9 . 5 000x);

        impl_test!(Up, Ceiling, HalfDown, HalfUp, HalfEven => 10);
        impl_test!(Down, Floor => 9);
    }

    mod case_9_5 {
        use super::*;

        define_test_input!(9 . 5);

        impl_test!(Up, Ceiling, HalfUp, HalfEven => 10);
        impl_test!(Down, Floor, HalfDown => 9);
    }

    mod case_8_5 {
        use super::*;

        define_test_input!(8 . 5);

        impl_test!(Up, Ceiling, HalfUp => 9);
        impl_test!(Down, Floor, HalfDown, HalfEven => 8);
    }

    mod case_neg_6_5 {
        use super::*;

        define_test_input!(-6 . 5);

        impl_test!(Up, Floor, HalfUp => 7);
        impl_test!(Down, Ceiling, HalfDown, HalfEven => 6);
    }

    mod case_neg_6_5_000x {
        use super::*;

        define_test_input!(-6 . 5 000x);

        impl_test!(Up, Floor, HalfUp, HalfDown, HalfEven => 7);
        impl_test!(Down, Ceiling => 6);
    }

    mod case_3_0 {
        use super::*;

        define_test_input!(3 . 0);

        impl_test!(Up, Down, Ceiling, Floor, HalfUp, HalfDown, HalfEven => 3);
    }

    mod case_3_0_000x {
        use super::*;

        define_test_input!(3 . 0 000x);

        impl_test!(Up, Ceiling => 4);
        impl_test!(Down, Floor, HalfUp, HalfDown, HalfEven => 3);
    }

    mod case_neg_2_0 {
        use super::*;

        define_test_input!(-2 . 0);

        impl_test!(Up, Down, Ceiling, Floor, HalfUp, HalfDown, HalfEven => 2);
    }

    mod case_neg_2_0_000x {
        use super::*;

        define_test_input!(-2 . 0 000x);

        impl_test!(Up, Floor => 3);
        impl_test!(Down, Ceiling, HalfUp, HalfDown, HalfEven => 2);
    }
}


#[cfg(test)]
#[allow(non_snake_case)]
mod test_round_u32 {
    use paste::paste;
    use super::*;

    macro_rules! impl_test {
        ( $pos:literal :: $($mode:ident),+ => $expected:literal) => {
            $(
                paste! {
                    #[test]
                    fn [< digit_ $pos _mode_ $mode >]() {
                        let (value, sign, trailing_zeros) = test_input();
                        let mode = self::RoundingMode::$mode;
                        let pos = stdlib::num::NonZeroU8::new($pos as u8).unwrap();
                        let result = mode.round_u32(pos, sign, value, trailing_zeros);
                        assert_eq!(result, $expected);
                    }
                }
            )*
        }
    }

    macro_rules! define_test_input {
        ( - $value:literal $($t:tt)* ) => {
            define_test_input!(sign=Sign::Minus, value=$value $($t)*);
        };
        ( $value:literal $($t:tt)* ) => {
            define_test_input!(sign=Sign::Plus, value=$value $($t)*);
        };
        ( sign=$sign:expr, value=$value:literal ...000x ) => {
            define_test_input!(sign=$sign, value=$value, trailing_zeros=false);
        };
        ( sign=$sign:expr, value=$value:literal ) => {
            define_test_input!(sign=$sign, value=$value, trailing_zeros=true);
        };
        ( sign=$sign:expr, value=$value:expr, trailing_zeros=$trailing_zeros:literal ) => {
            fn test_input() -> (u32, Sign, bool) { ($value, $sign, $trailing_zeros) }
        };
    }

    mod case_13950000 {
        use super::*;

        define_test_input!(13950000);

        impl_test!(3 :: Up => 13950000);
        impl_test!(5 :: Up, Ceiling, HalfUp, HalfEven => 14000000);
        impl_test!(5 :: Down, HalfDown => 13900000);
    }

    mod case_neg_35488622_000x {
        use super::*;

        // ...000x indicates non-zero trailing digit
        define_test_input!(-35488622 ...000x);

        impl_test!(1 :: Up => 35488630);
        impl_test!(1 :: Down => 35488620);
        impl_test!(2 :: Up => 35488700);
        impl_test!(2 :: Down => 35488600);
        impl_test!(7 :: Up, Floor => 40000000);
        impl_test!(7 :: Down, Ceiling => 30000000);
        impl_test!(8 :: Up => 100000000);
        impl_test!(8 :: Down => 0);
    }
}
