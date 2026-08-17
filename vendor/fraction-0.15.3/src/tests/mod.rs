macro_rules! generate_ops_tests {
    (
        Zero => $_0:block;
        One => $_1:block;
        Two => $_2:block;
        Three => $_3:block;
        Four => $_4:block;
    ) => {
        #[test]
        fn op_eq_i() {
            let nil = $_0;
            let one = $_1;
            let two = $_2;
            let thr = $_3;
            let four = $_4;

            assert_eq!(nil, nil);
            assert_eq!(nil, $_0);
            assert_eq!($_0, $_0);

            assert_eq!(one, one);
            assert_eq!(one, $_1);
            assert_eq!($_1, $_1);

            assert_eq!(two, two);
            assert_eq!(two, $_2);
            assert_eq!($_2, $_2);

            assert_eq!(thr, thr);
            assert_eq!(thr, $_3);
            assert_eq!($_3, $_3);

            assert_eq!(four, four);
            assert_eq!(four, $_4);
            assert_eq!($_4, $_4);
        }

        #[test]
        fn op_add_assign_i() {
            {
                let mut v = $_0;
                v += $_0;
                assert_eq!(v, $_0);
            }

            {
                let mut v = $_0;
                v += $_1;
                assert_eq!(v, $_1);
            }

            {
                let mut v = $_1;
                v += $_0;
                assert_eq!(v, $_1);
            }

            {
                let mut v = $_1;
                v += $_1;
                assert_eq!(v, $_2);
            }

            {
                let mut v = $_1;
                v += $_2;
                assert_eq!(v, $_3);
            }

            {
                let mut v = $_2;
                v += $_2;
                assert_eq!(v, $_4);
            }
        }

        #[test]
        fn op_add_assign_refs_i() {
            {
                let mut v = $_0;
                v += &$_0;
                assert_eq!(v, $_0);
            }

            {
                let mut v = $_0;
                v += &$_1;
                assert_eq!(v, $_1);
            }

            {
                let mut v = $_1;
                v += &$_0;
                assert_eq!(v, $_1);
            }

            {
                let mut v = $_1;
                v += &$_1;
                assert_eq!(v, $_2);
            }

            {
                let mut v = $_1;
                v += &$_2;
                assert_eq!(v, $_3);
            }

            {
                let mut v = $_2;
                v += &$_2;
                assert_eq!(v, $_4);
            }
        }

        #[test]
        fn op_checked_add_i() {
            assert_eq!($_0, $_0.checked_add(&$_0).unwrap());

            assert_eq!($_1, $_1.checked_add(&$_0).unwrap());
            assert_eq!($_1, $_0.checked_add(&$_1).unwrap());

            assert_eq!($_2, $_2.checked_add(&$_0).unwrap());
            assert_eq!($_2, $_0.checked_add(&$_2).unwrap());
            assert_eq!($_2, $_1.checked_add(&$_1).unwrap());

            assert_eq!($_3, $_3.checked_add(&$_0).unwrap());
            assert_eq!($_3, $_2.checked_add(&$_1).unwrap());
            assert_eq!($_3, $_1.checked_add(&$_2).unwrap());

            assert_eq!($_4, $_0.checked_add(&$_4).unwrap());
            assert_eq!($_4, $_4.checked_add(&$_0).unwrap());
            assert_eq!($_4, $_2.checked_add(&$_2).unwrap());
            assert_eq!($_4, $_1.checked_add(&$_3).unwrap());
            assert_eq!($_4, $_3.checked_add(&$_1).unwrap());
        }

        #[test]
        fn op_checked_div_i() {
            assert_eq!($_1, $_1.checked_div(&$_1).unwrap());
            assert_eq!($_1, $_2.checked_div(&$_2).unwrap());
            assert_eq!($_1, $_3.checked_div(&$_3).unwrap());
            assert_eq!($_1, $_4.checked_div(&$_4).unwrap());

            assert_eq!($_2, $_2.checked_div(&$_1).unwrap());

            assert_eq!($_3, $_3.checked_div(&$_1).unwrap());
            assert_eq!($_2, $_4.checked_div(&$_2).unwrap());

            assert_eq!($_4, $_4.checked_div(&$_1).unwrap());
        }


        #[test]
        fn op_checked_mul_i() {
            assert_eq!($_0, $_0.checked_mul(&$_0).unwrap());

            assert_eq!($_0, $_1.checked_mul(&$_0).unwrap());
            assert_eq!($_0, $_0.checked_mul(&$_1).unwrap());

            assert_eq!($_0, $_2.checked_mul(&$_0).unwrap());
            assert_eq!($_0, $_0.checked_mul(&$_2).unwrap());
            assert_eq!($_1, $_1.checked_mul(&$_1).unwrap());

            assert_eq!($_0, $_3.checked_mul(&$_0).unwrap());
            assert_eq!($_2, $_2.checked_mul(&$_1).unwrap());
            assert_eq!($_2, $_1.checked_mul(&$_2).unwrap());

            assert_eq!($_0, $_0.checked_mul(&$_4).unwrap());
            assert_eq!($_0, $_4.checked_mul(&$_0).unwrap());
            assert_eq!($_4, $_2.checked_mul(&$_2).unwrap());
            assert_eq!($_3, $_1.checked_mul(&$_3).unwrap());
            assert_eq!($_3, $_3.checked_mul(&$_1).unwrap());
        }


        #[test]
        fn op_checked_sub_i() {
            assert_eq!($_0, $_0.checked_sub(&$_0).unwrap());
            assert_eq!($_0, $_1.checked_sub(&$_1).unwrap());
            assert_eq!($_0, $_2.checked_sub(&$_2).unwrap());
            assert_eq!($_0, $_3.checked_sub(&$_3).unwrap());
            assert_eq!($_0, $_4.checked_sub(&$_4).unwrap());

            assert_eq!($_1, $_1.checked_sub(&$_0).unwrap());
            assert_eq!($_1, $_2.checked_sub(&$_1).unwrap());
            assert_eq!($_1, $_3.checked_sub(&$_2).unwrap());
            assert_eq!($_1, $_4.checked_sub(&$_3).unwrap());

            assert_eq!($_2, $_2.checked_sub(&$_0).unwrap());
            assert_eq!($_2, $_3.checked_sub(&$_1).unwrap());
            assert_eq!($_2, $_4.checked_sub(&$_2).unwrap());

            assert_eq!($_3, $_3.checked_sub(&$_0).unwrap());
            assert_eq!($_3, $_4.checked_sub(&$_1).unwrap());

            assert_eq!($_4, $_4.checked_sub(&$_0).unwrap());
        }


        #[test]
        fn op_add_i() {
            assert_eq!($_0, &$_0 + &$_0);
            assert_eq!($_0, $_0 + $_0);

            assert_eq!($_1, &$_1 + &$_0);
            assert_eq!($_1, $_1 + $_0);
            assert_eq!($_1, &$_0 + &$_1);
            assert_eq!($_1, $_0 + $_1);

            assert_eq!($_2, &$_2 + &$_0);
            assert_eq!($_2, $_2 + $_0);
            assert_eq!($_2, &$_0 + &$_2);
            assert_eq!($_2, $_0 + $_2);
            assert_eq!($_2, &$_1 + &$_1);
            assert_eq!($_2, $_1 + $_1);

            assert_eq!($_3, &$_3 + &$_0);
            assert_eq!($_3, $_3 + $_0);
            assert_eq!($_3, &$_2 + &$_1);
            assert_eq!($_3, $_2 + $_1);
            assert_eq!($_3, &$_1 + &$_2);
            assert_eq!($_3, $_1 + $_2);

            assert_eq!($_4, &$_0 + &$_4);
            assert_eq!($_4, $_0 + $_4);
            assert_eq!($_4, &$_4 + &$_0);
            assert_eq!($_4, $_4 + $_0);
            assert_eq!($_4, &$_2 + &$_2);
            assert_eq!($_4, $_2 + $_2);
            assert_eq!($_4, &$_1 + &$_3);
            assert_eq!($_4, $_1 + $_3);
            assert_eq!($_4, &$_3 + &$_1);
            assert_eq!($_4, $_3 + $_1);
            assert_eq!($_4, $_2 + $_1 + $_1);
            assert_eq!($_4, $_1 + $_1 + $_1 + $_1);
        }

        #[test]
        fn op_sub_assign_i() {
            {
                let mut v = $_0;
                v -= $_0;
                assert_eq!(v, $_0);
            }

            {
                let mut v = $_1;
                v -= $_0;
                assert_eq!(v, $_1);
            }

            {
                let mut v = $_1;
                v -= $_1;
                assert_eq!(v, $_0);
            }

            {
                let mut v = $_2;
                v -= $_2;
                assert_eq!(v, $_0);
            }

            {
                let mut v = $_2;
                v -= $_1;
                assert_eq!(v, $_1);
            }

            {
                let mut v = $_2;
                v -= $_0;
                assert_eq!(v, $_2);
            }

            {
                let mut v = $_3;
                v -= $_2;
                assert_eq!(v, $_1);
            }

            {
                let mut v = $_3;
                v -= $_1;
                assert_eq!(v, $_2);
            }

            {
                let mut v = $_3;
                v -= $_0;
                assert_eq!(v, $_3);
            }

            {
                let mut v = $_4;
                v -= $_4;
                assert_eq!(v, $_0);
            }

            {
                let mut v = $_4;
                v -= $_3;
                assert_eq!(v, $_1);
            }

            {
                let mut v = $_4;
                v -= $_2;
                assert_eq!(v, $_2);
            }

            {
                let mut v = $_4;
                v -= $_1;
                assert_eq!(v, $_3);
            }

            {
                let mut v = $_4;
                v -= $_0;
                assert_eq!(v, $_4);
            }
        }

        #[test]
        fn op_sub_assign_refs_i() {
            {
                let mut v = $_0;
                v -= &$_0;
                assert_eq!(v, $_0);
            }

            {
                let mut v = $_1;
                v -= &$_0;
                assert_eq!(v, $_1);
            }

            {
                let mut v = $_1;
                v -= &$_1;
                assert_eq!(v, $_0);
            }

            {
                let mut v = $_2;
                v -= &$_2;
                assert_eq!(v, $_0);
            }

            {
                let mut v = $_2;
                v -= &$_1;
                assert_eq!(v, $_1);
            }

            {
                let mut v = $_2;
                v -= &$_0;
                assert_eq!(v, $_2);
            }

            {
                let mut v = $_3;
                v -= &$_2;
                assert_eq!(v, $_1);
            }

            {
                let mut v = $_3;
                v -= &$_1;
                assert_eq!(v, $_2);
            }

            {
                let mut v = $_3;
                v -= &$_0;
                assert_eq!(v, $_3);
            }

            {
                let mut v = $_4;
                v -= &$_4;
                assert_eq!(v, $_0);
            }

            {
                let mut v = $_4;
                v -= &$_3;
                assert_eq!(v, $_1);
            }

            {
                let mut v = $_4;
                v -= &$_2;
                assert_eq!(v, $_2);
            }

            {
                let mut v = $_4;
                v -= &$_1;
                assert_eq!(v, $_3);
            }

            {
                let mut v = $_4;
                v -= &$_0;
                assert_eq!(v, $_4);
            }
        }

        #[test]
        fn op_sub_i() {
            assert_eq!($_0, $_0 - $_0);
            assert_eq!($_0, &$_0 - &$_0);
            assert_eq!($_0, $_1 - $_1);
            assert_eq!($_0, &$_1 - &$_1);
            assert_eq!($_0, $_2 - $_2);
            assert_eq!($_0, &$_2 - &$_2);
            assert_eq!($_0, $_3 - $_3);
            assert_eq!($_0, &$_3 - &$_3);
            assert_eq!($_0, $_4 - $_4);
            assert_eq!($_0, &$_4 - &$_4);

            assert_eq!($_1, &$_1 - &$_0);
            assert_eq!($_1, $_1 - $_0);
            assert_eq!($_1, &$_2 - &$_1);
            assert_eq!($_1, $_2 - $_1);
            assert_eq!($_1, &$_3 - &$_2);
            assert_eq!($_1, $_3 - $_2);
            assert_eq!($_1, &$_4 - &$_3);
            assert_eq!($_1, $_4 - $_3);

            assert_eq!($_2, &$_2 - &$_0);
            assert_eq!($_2, $_2 - $_0);
            assert_eq!($_2, &$_3 - &$_1);
            assert_eq!($_2, $_3 - $_1);
            assert_eq!($_2, &$_4 - &$_2);
            assert_eq!($_2, $_4 - $_2);

            assert_eq!($_3, &$_3 - &$_0);
            assert_eq!($_3, $_3 - $_0);
            assert_eq!($_3, &$_4 - &$_1);
            assert_eq!($_3, $_4 - $_1);

            assert_eq!($_4, &$_4 - &$_0);
            assert_eq!($_4, $_4 - $_0);
        }


        #[test]
        fn op_mul_assign_i() {
            {
                let mut v = $_0;
                v *= $_0;
                assert_eq!(v, $_0);
            }

            {
                let mut v = $_0;
                v *= $_1;
                assert_eq!(v, $_0);
            }

            {
                let mut v = $_1;
                v *= $_0;
                assert_eq!(v, $_0);
            }

            {
                let mut v = $_1;
                v *= $_1;
                assert_eq!(v, $_1);
            }

            {
                let mut v = $_1;
                v *= $_2;
                assert_eq!(v, $_2);
            }

            {
                let mut v = $_1;
                v *= $_3;
                assert_eq!(v, $_3);
            }
        }

        #[test]
        fn op_mul_assign_refs_i() {
            {
                let mut v = $_0;
                v *= &$_0;
                assert_eq!(v, $_0);
            }

            {
                let mut v = $_0;
                v *= &$_1;
                assert_eq!(v, $_0);
            }

            {
                let mut v = $_1;
                v *= &$_0;
                assert_eq!(v, $_0);
            }

            {
                let mut v = $_1;
                v *= &$_1;
                assert_eq!(v, $_1);
            }

            {
                let mut v = $_1;
                v *= &$_2;
                assert_eq!(v, $_2);
            }

            {
                let mut v = $_1;
                v *= &$_3;
                assert_eq!(v, $_3);
            }
        }

        #[test]
        fn op_mul_i() {
            assert_eq!($_0, &$_0 * &$_0);
            assert_eq!($_0, $_0 * $_0);

            assert_eq!($_0, &$_1 * &$_0);
            assert_eq!($_0, $_1 * $_0);
            assert_eq!($_0, &$_0 * &$_1);
            assert_eq!($_0, $_0 * $_1);

            assert_eq!($_0, &$_2 * &$_0);
            assert_eq!($_0, $_2 * $_0);
            assert_eq!($_0, &$_0 * &$_2);
            assert_eq!($_0, $_0 * $_2);
            assert_eq!($_1, &$_1 * &$_1);
            assert_eq!($_1, $_1 * $_1);

            assert_eq!($_0, &$_3 * &$_0);
            assert_eq!($_0, $_3 * $_0);
            assert_eq!($_2, &$_2 * &$_1);
            assert_eq!($_2, $_2 * $_1);
            assert_eq!($_2, &$_1 * &$_2);
            assert_eq!($_2, $_1 * $_2);

            assert_eq!($_0, &$_0 * &$_4);
            assert_eq!($_0, $_0 * $_4);
            assert_eq!($_0, &$_4 * &$_0);
            assert_eq!($_0, $_4 * $_0);
            assert_eq!($_4, &$_2 * &$_2);
            assert_eq!($_4, $_2 * $_2);
            assert_eq!($_3, &$_1 * &$_3);
            assert_eq!($_3, $_1 * $_3);
            assert_eq!($_3, &$_3 * &$_1);
            assert_eq!($_3, $_3 * $_1);
            assert_eq!($_2, $_2 * $_1 * $_1);
            assert_eq!($_1, $_1 * $_1 * $_1 * $_1);
        }


        #[test]
        fn op_div_assign_i() {
            {
                let mut v = $_0;
                v /= $_1;
                assert_eq!(v, $_0);
            }

            {
                let mut v = $_1;
                v /= $_1;
                assert_eq!(v, $_1);
            }

            {
                let mut v = $_2;
                v /= $_2;
                assert_eq!(v, $_1);
            }

            {
                let mut v = $_2;
                v /= $_1;
                assert_eq!(v, $_2);
            }

            {
                let mut v = $_0;
                v /= $_2;
                assert_eq!(v, $_0);
            }

            {
                let mut v = $_3;
                v /= $_1;
                assert_eq!(v, $_3);
            }

            {
                let mut v = $_0;
                v /= $_3;
                assert_eq!(v, $_0);
            }

            {
                let mut v = $_4;
                v /= $_4;
                assert_eq!(v, $_1);
            }

            {
                let mut v = $_4;
                v /= $_2;
                assert_eq!(v, $_2);
            }

            {
                let mut v = $_4;
                v /= $_1;
                assert_eq!(v, $_4);
            }

            {
                let mut v = $_0;
                v /= $_4;
                assert_eq!(v, $_0);
            }
        }

        #[test]
        fn op_div_assign_refs_i() {
            {
                let mut v = $_0;
                v /= &$_1;
                assert_eq!(v, $_0);
            }

            {
                let mut v = $_1;
                v /= &$_1;
                assert_eq!(v, $_1);
            }

            {
                let mut v = $_2;
                v /= &$_2;
                assert_eq!(v, $_1);
            }

            {
                let mut v = $_2;
                v /= &$_1;
                assert_eq!(v, $_2);
            }

            {
                let mut v = $_0;
                v /= &$_2;
                assert_eq!(v, $_0);
            }

            {
                let mut v = $_3;
                v /= &$_1;
                assert_eq!(v, $_3);
            }

            {
                let mut v = $_0;
                v /= &$_3;
                assert_eq!(v, $_0);
            }

            {
                let mut v = $_4;
                v /= &$_4;
                assert_eq!(v, $_1);
            }

            {
                let mut v = $_4;
                v /= &$_2;
                assert_eq!(v, $_2);
            }

            {
                let mut v = $_4;
                v /= &$_1;
                assert_eq!(v, $_4);
            }

            {
                let mut v = $_0;
                v /= &$_4;
                assert_eq!(v, $_0);
            }
        }


        #[test]
        fn op_div_i() {
            assert_eq!($_1, $_1 / $_1);
            assert_eq!($_1, &$_1 / &$_1);
            assert_eq!($_1, $_2 / $_2);
            assert_eq!($_1, &$_2 / &$_2);
            assert_eq!($_1, $_3 / $_3);
            assert_eq!($_1, &$_3 / &$_3);
            assert_eq!($_1, $_4 / $_4);
            assert_eq!($_1, &$_4 / &$_4);

            assert_eq!($_2, &$_2 / &$_1);
            assert_eq!($_2, $_2 / $_1);

            assert_eq!($_3, &$_3 / &$_1);
            assert_eq!($_3, $_3 / $_1);
            assert_eq!($_2, &$_4 / &$_2);
            assert_eq!($_2, $_4 / $_2);

            assert_eq!($_4, &$_4 / &$_1);
            assert_eq!($_4, $_4 / $_1);
        }


        #[test]
        fn op_rem_assign_i() {
            {
                let mut v = $_1;
                v %= $_1;
                assert_eq!(v, $_0);
            }

            {
                let mut v = $_2;
                v %= $_2;
                assert_eq!(v, $_0);
            }

            {
                let mut v = $_2;
                v %= $_1;
                assert_eq!(v, $_0);
            }

            {
                let mut v = $_3;
                v %= $_2;
                assert_eq!(v, $_1);
            }

            {
                let mut v = $_3;
                v %= $_1;
                assert_eq!(v, $_0);
            }

            {
                let mut v = $_4;
                v %= $_4;
                assert_eq!(v, $_0);
            }

            {
                let mut v = $_4;
                v %= $_3;
                assert_eq!(v, $_1);
            }

            {
                let mut v = $_4;
                v %= $_2;
                assert_eq!(v, $_0);
            }

            {
                let mut v = $_4;
                v %= $_1;
                assert_eq!(v, $_0);
            }
        }


        #[test]
        fn op_rem_assign_refs_i() {
            {
                let mut v = $_1;
                v %= &$_1;
                assert_eq!(v, $_0);
            }

            {
                let mut v = $_2;
                v %= &$_2;
                assert_eq!(v, $_0);
            }

            {
                let mut v = $_2;
                v %= &$_1;
                assert_eq!(v, $_0);
            }

            {
                let mut v = $_3;
                v %= &$_2;
                assert_eq!(v, $_1);
            }

            {
                let mut v = $_3;
                v %= &$_1;
                assert_eq!(v, $_0);
            }

            {
                let mut v = $_4;
                v %= &$_4;
                assert_eq!(v, $_0);
            }

            {
                let mut v = $_4;
                v %= &$_3;
                assert_eq!(v, $_1);
            }

            {
                let mut v = $_4;
                v %= &$_2;
                assert_eq!(v, $_0);
            }

            {
                let mut v = $_4;
                v %= &$_1;
                assert_eq!(v, $_0);
            }
        }

        #[test]
        fn op_rem_i() {
            assert_eq!($_0, $_1 % $_1);
            assert_eq!($_0, &$_1 % &$_1);
            assert_eq!($_0, $_2 % $_2);
            assert_eq!($_0, &$_2 % &$_2);
            assert_eq!($_0, $_3 % $_3);
            assert_eq!($_0, &$_3 % &$_3);
            assert_eq!($_0, $_4 % $_4);
            assert_eq!($_0, &$_4 % &$_4);

            assert_eq!($_0, &$_2 % &$_1);
            assert_eq!($_0, $_2 % $_1);
            assert_eq!($_1, &$_3 % &$_2);
            assert_eq!($_1, $_3 % $_2);
            assert_eq!($_1, &$_4 % &$_3);
            assert_eq!($_1, $_4 % $_3);

            assert_eq!($_0, &$_3 % &$_1);
            assert_eq!($_0, $_3 % $_1);
            assert_eq!($_0, &$_4 % &$_2);
            assert_eq!($_0, $_4 % $_2);

            assert_eq!($_0, &$_4 % &$_1);
            assert_eq!($_0, $_4 % $_1);
        }
    };

    (
        NaN => $nan:block;
        NegInf => $nin:block;
        PosInf => $pin:block;
        Zero => $_0:block;
        Half => $half:block;
        One => $_1:block;
        Two => $_2:block;
        Three => $_3:block;
        Four => $_4:block;
    ) => {

        generate_ops_tests!(
            Zero => $_0;
            One => $_1;
            Two => $_2;
            Three => $_3;
            Four => $_4;
        );

        #[test]
        fn op_add_assign() {
            let nan = $nan;
            let nin = $nin;
            let pin = $pin;
            let nil = $_0;
            let one = $_1;

            let mut nan_ = $nan;
            nan_ += nan;
            assert_eq!(nan, nan_);

            nan_ += nin;
            assert_eq!(nan, nan_);

            nan_ += pin;
            assert_eq!(nan, nan_);

            nan_ += nil;
            assert_eq!(nan, nan_);

            let mut nil_ = nil;
            nil_ += nil_;
            assert_eq!(nil, nil_);

            nil_ += one;
            assert_eq!(one, nil_);
        }

        #[test]
        fn op_add_assign_refs() {
            let nan = $nan;
            let nin = $nin;
            let pin = $pin;
            let nil = $_0;
            let one = $_1;

            let mut nan_ = $nan;
            nan_ += &nan;
            assert_eq!(nan, nan_);

            nan_ += &nin;
            assert_eq!(nan, nan_);

            nan_ += &pin;
            assert_eq!(nan, nan_);

            nan_ += &nil;
            assert_eq!(nan, nan_);

            let mut nil_ = nil;
            nil_ += &nil;
            assert_eq!(nil, nil_);

            nil_ += &one;
            assert_eq!(one, nil_);
        }


        #[test]
        fn op_add() {
            let nan = $nan;
            let ninf = $nin;
            let pinf = $pin;

            assert_eq!(nan, nan);
            assert_eq!(nan, &nan + &nan);
            assert_eq!(nan, nan + nan);
            assert_eq!(nan, &nan + &ninf);
            assert_eq!(nan, nan + ninf);
            assert_eq!(nan, &nan + &pinf);
            assert_eq!(nan, nan + pinf);
            assert_eq!(nan, &ninf + &nan);
            assert_eq!(nan, ninf + nan);
            assert_eq!(nan, &pinf + &nan);
            assert_eq!(nan, pinf + nan);

            assert_eq!(ninf, &ninf + &ninf);
            assert_eq!(ninf, ninf + ninf);
            assert_eq!(pinf, &pinf + &pinf);
            assert_eq!(pinf, pinf + pinf);
            assert_eq!(nan, &ninf + &pinf);
            assert_eq!(nan, ninf + pinf);
            assert_eq!(nan, &pinf + &ninf);
            assert_eq!(nan, pinf + ninf);

            let nil = $_0;

            assert_eq!(nil, nil);
            assert_eq!(nil, &nil + &nil);
            assert_eq!(nil, nil + nil);
            assert_eq!(nan, &nil + &nan);
            assert_eq!(nan, nil + nan);
            assert_eq!(nan, &nan + &nil);
            assert_eq!(nan, nan + nil);
            assert_eq!(ninf, &nil + &ninf);
            assert_eq!(ninf, nil + ninf);
            assert_eq!(ninf, &ninf + &nil);
            assert_eq!(ninf, ninf + nil);
            assert_eq!(pinf, &nil + &pinf);
            assert_eq!(pinf, nil + pinf);
            assert_eq!(pinf, &pinf + &nil);
            assert_eq!(pinf, pinf + nil);

            let one = $_1;

            assert_eq!(one, one);
            assert_eq!(one, &one + &nil);
            assert_eq!(one, one + nil);
            assert_eq!(one, &nil + &one);
            assert_eq!(one, nil + one);
            assert_eq!(nan, &one + &nan);
            assert_eq!(nan, one + nan);
            assert_eq!(nan, &nan + &one);
            assert_eq!(nan, nan + one);
            assert_eq!(ninf, &one + &ninf);
            assert_eq!(ninf, one + ninf);
            assert_eq!(ninf, &ninf + &one);
            assert_eq!(ninf, ninf + one);
            assert_eq!(pinf, &one + &pinf);
            assert_eq!(pinf, one + pinf);
            assert_eq!(pinf, &pinf + &one);
            assert_eq!(pinf, pinf + one);

            let two = $_2;

            assert_eq!(two, two);
            assert_eq!(two, &two + &nil);
            assert_eq!(two, two + nil);
            assert_eq!(two, &nil + &two);
            assert_eq!(two, nil + two);
            assert_eq!(two, &one + &one);
            assert_eq!(two, one + one);
            assert_eq!(nan, &two + &nan);
            assert_eq!(nan, two + nan);
            assert_eq!(nan, &nan + &two);
            assert_eq!(nan, nan + two);
            assert_eq!(ninf, &two + &ninf);
            assert_eq!(ninf, two + ninf);
            assert_eq!(ninf, &ninf + &two);
            assert_eq!(ninf, ninf + two);
            assert_eq!(pinf, &two + &pinf);
            assert_eq!(pinf, two + pinf);
            assert_eq!(pinf, &pinf + &two);
            assert_eq!(pinf, pinf + two);

            let mnil = -nil;

            assert_eq!(mnil, mnil);
            assert_eq!(mnil, nil);
            assert_eq!(mnil, &nil + &nil);
            assert_eq!(mnil, nil + nil);
            assert_eq!(mnil, &mnil + &mnil);
            assert_eq!(mnil, mnil + mnil);
            assert_eq!(mnil, &mnil + &nil);
            assert_eq!(mnil, mnil + nil);
            assert_eq!(mnil, &nil + &mnil);
            assert_eq!(mnil, nil + mnil);
            assert_eq!(nan, &mnil + &nan);
            assert_eq!(nan, mnil + nan);
            assert_eq!(nan, &nan + &mnil);
            assert_eq!(nan, nan + mnil);
            assert_eq!(ninf, &mnil + &ninf);
            assert_eq!(ninf, mnil + ninf);
            assert_eq!(ninf, &ninf + &mnil);
            assert_eq!(ninf, ninf + mnil);
            assert_eq!(pinf, &mnil + &pinf);
            assert_eq!(pinf, mnil + pinf);
            assert_eq!(pinf, &pinf + &mnil);
            assert_eq!(pinf, pinf + mnil);

            let mone = -one;

            assert_eq!(mone, mone);
            assert_eq!(mone, &mone + &nil);
            assert_eq!(mone, mone + nil);
            assert_eq!(mone, &nil + &mone);
            assert_eq!(mone, nil + mone);
            assert_eq!(nan, &mone + &nan);
            assert_eq!(nan, mone + nan);
            assert_eq!(nan, &nan + &mone);
            assert_eq!(nan, nan + mone);
            assert_eq!(nil, &mone + &one);
            assert_eq!(nil, mone + one);
            assert_eq!(nil, &one + &mone);
            assert_eq!(nil, one + mone);
            assert_eq!(one, &mone + &two);
            assert_eq!(one, mone + two);
            assert_eq!(one, &two + &mone);
            assert_eq!(one, two + mone);
            assert_eq!(nan, &mone + &nan);
            assert_eq!(nan, mone + nan);
            assert_eq!(nan, &nan + &mone);
            assert_eq!(nan, nan + mone);
            assert_eq!(ninf, &mone + &ninf);
            assert_eq!(ninf, mone + ninf);
            assert_eq!(ninf, &ninf + &mone);
            assert_eq!(ninf, ninf + mone);
            assert_eq!(pinf, &mone + &pinf);
            assert_eq!(pinf, mone + pinf);
            assert_eq!(pinf, &pinf + &mone);
            assert_eq!(pinf, pinf + mone);

            let mtwo = -two;

            assert_eq!(mtwo, mtwo);
            assert_eq!(mtwo, &mtwo + &nil);
            assert_eq!(mtwo, mtwo + nil);
            assert_eq!(mtwo, &nil + &mtwo);
            assert_eq!(mtwo, nil + mtwo);
            assert_eq!(mtwo, &mtwo + &mnil);
            assert_eq!(mtwo, mtwo + mnil);
            assert_eq!(mtwo, &mnil + &mtwo);
            assert_eq!(mtwo, mnil + mtwo);
            assert_eq!(mone, &mtwo + &one);
            assert_eq!(mone, mtwo + one);
            assert_eq!(mone, &one + &mtwo);
            assert_eq!(mone, one + mtwo);
            assert_eq!(nil, &mtwo + &two);
            assert_eq!(nil, mtwo + two);
            assert_eq!(nil, &two + &mtwo);
            assert_eq!(nil, two + mtwo);
            assert_eq!(nan, &mtwo + &nan);
            assert_eq!(nan, mtwo + nan);
            assert_eq!(nan, &nan + &mtwo);
            assert_eq!(nan, nan + mtwo);
            assert_eq!(ninf, &mtwo + &ninf);
            assert_eq!(ninf, mtwo + ninf);
            assert_eq!(ninf, &ninf + &mtwo);
            assert_eq!(ninf, ninf + mtwo);
            assert_eq!(pinf, &mtwo + &pinf);
            assert_eq!(pinf, mtwo + pinf);
            assert_eq!(pinf, &pinf + &mtwo);
            assert_eq!(pinf, pinf + mtwo);
        }

        #[test]
        fn op_sub_assign() {
            let nan = $nan;
            let nin = $nin;
            let pin = $pin;
            let nil = $_0;
            let one = $_1;

            let mut nan_ = $nan;
            nan_ -= nan;
            assert_eq!(nan, nan_);

            nan_ -= nin;
            assert_eq!(nan, nan_);

            nan_ -= pin;
            assert_eq!(nan, nan_);

            nan_ -= nil;
            assert_eq!(nan, nan_);

            let mut nil_ = nil;
            nil_ -= nil_;
            assert_eq!(nil, nil_);

            nil_ -= one;
            assert_eq!(-one, nil_);
        }

        #[test]
        fn op_sub_assign_refs() {
            let nan = $nan;
            let nin = $nin;
            let pin = $pin;
            let nil = $_0;
            let one = $_1;

            let mut nan_ = $nan;
            nan_ -= &nan;
            assert_eq!(nan, nan_);

            nan_ -= &nin;
            assert_eq!(nan, nan_);

            nan_ -= &pin;
            assert_eq!(nan, nan_);

            nan_ -= &nil;
            assert_eq!(nan, nan_);

            let mut nil_ = nil;
            nil_ -= &nil;
            assert_eq!(nil, nil_);

            nil_ -= &one;
            assert_eq!(-one, nil_);
        }


        #[test]
        fn op_sub() {
            let nan = $nan;
            let ninf = $nin;
            let pinf = $pin;

            assert_eq!(nan, nan);
            assert_eq!(nan, &nan - &nan);
            assert_eq!(nan, nan - nan);
            assert_eq!(nan, &nan - &ninf);
            assert_eq!(nan, nan - ninf);
            assert_eq!(nan, &nan - &pinf);
            assert_eq!(nan, nan - pinf);
            assert_eq!(nan, &ninf - &nan);
            assert_eq!(nan, ninf - nan);
            assert_eq!(nan, &pinf - &nan);
            assert_eq!(nan, pinf - nan);

            assert_eq!(nan, &ninf - &ninf);
            assert_eq!(nan, ninf - ninf);
            assert_eq!(nan, &pinf - &pinf);
            assert_eq!(nan, pinf - pinf);
            assert_eq!(pinf, &pinf - &ninf);
            assert_eq!(pinf, pinf - ninf);
            assert_eq!(ninf, &ninf - &pinf);
            assert_eq!(ninf, ninf - pinf);

            let nil = $_0;

            assert_eq!(nil, nil);
            assert_eq!(nil, &nil - &nil);
            assert_eq!(nil, nil - nil);
            assert_eq!(nan, &nil - &nan);
            assert_eq!(nan, nil - nan);
            assert_eq!(nan, &nan - &nil);
            assert_eq!(nan, nan - nil);
            assert_eq!(pinf, &nil - &ninf);
            assert_eq!(pinf, nil - ninf);
            assert_eq!(ninf, &ninf - &nil);
            assert_eq!(ninf, ninf - nil);
            assert_eq!(ninf, &nil - &pinf);
            assert_eq!(ninf, nil - pinf);
            assert_eq!(pinf, &pinf - &nil);
            assert_eq!(pinf, pinf - nil);

            let one = $_1;
            let two = $_2;

            let mone = -one;
            let mtwo = -two;

            assert_eq!(one, one);
            assert_eq!(one, &one - &nil);
            assert_eq!(one, one - nil);
            assert_eq!(mone, &nil - &one);
            assert_eq!(mone, nil - one);
            assert_eq!(nan, &one - &nan);
            assert_eq!(nan, one - nan);
            assert_eq!(nan, &nan - &one);
            assert_eq!(nan, nan - one);
            assert_eq!(pinf, &one - &ninf);
            assert_eq!(pinf, one - ninf);
            assert_eq!(ninf, &ninf - &one);
            assert_eq!(ninf, ninf - one);
            assert_eq!(ninf, &one - &pinf);
            assert_eq!(ninf, one - pinf);
            assert_eq!(pinf, &pinf - &one);
            assert_eq!(pinf, pinf - one);
            assert_eq!(two, &one - &mone);
            assert_eq!(two, one - mone);
            assert_eq!(mtwo, &mone - &one);
            assert_eq!(mtwo, mone - one);
        }


        #[test]
        fn op_mul() {
            // let nan: Frac = GenericFraction::NaN;
            // let ninf: Frac = GenericFraction::Infinity(Sign::Minus);
            // let pinf: Frac = GenericFraction::Infinity(Sign::Plus);
            let nan = $nan;
            let ninf = $nin;
            let pinf = $pin;

            assert_eq!(nan, nan);
            assert_eq!(nan, &nan * &nan);
            assert_eq!(nan, nan * nan);
            assert_eq!(nan, &nan * &ninf);
            assert_eq!(nan, nan * ninf);
            assert_eq!(nan, &nan * &pinf);
            assert_eq!(nan, nan * pinf);
            assert_eq!(nan, &ninf * &nan);
            assert_eq!(nan, ninf * nan);
            assert_eq!(nan, &pinf * &nan);
            assert_eq!(nan, pinf * nan);

            assert_eq!(pinf, &ninf * &ninf);
            assert_eq!(pinf, ninf * ninf);
            assert_eq!(pinf, &pinf * &pinf);
            assert_eq!(pinf, pinf * pinf);
            assert_eq!(ninf, &pinf * &ninf);
            assert_eq!(ninf, pinf * ninf);
            assert_eq!(ninf, &ninf * &pinf);
            assert_eq!(ninf, ninf * pinf);

            // let nil = Frac::new(0, 1);
            let nil = $_0;

            assert_eq!(nil, nil);
            assert_eq!(nil, &nil * &nil);
            assert_eq!(nil, nil * nil);
            assert_eq!(nan, &nil * &nan);
            assert_eq!(nan, nil * nan);
            assert_eq!(nan, &nan * &nil);
            assert_eq!(nan, nan * nil);
            assert_eq!(nan, &nil * &ninf);
            assert_eq!(nan, nil * ninf);
            assert_eq!(nan, &ninf * &nil);
            assert_eq!(nan, ninf * nil);
            assert_eq!(nan, &nil * &pinf);
            assert_eq!(nan, nil * pinf);
            assert_eq!(nan, &pinf * &nil);
            assert_eq!(nan, pinf * nil);

            // let one = Frac::new(1, 1);
            let one = $_1;

            assert_eq!(one, one);
            assert_eq!(nil, &one * &nil);
            assert_eq!(nil, one * nil);
            assert_eq!(nil, &nil * &one);
            assert_eq!(nil, nil * one);
            assert_eq!(nan, &one * &nan);
            assert_eq!(nan, one * nan);
            assert_eq!(nan, &nan * &one);
            assert_eq!(nan, nan * one);
            assert_eq!(ninf, &one * &ninf);
            assert_eq!(ninf, one * ninf);
            assert_eq!(ninf, &ninf * &one);
            assert_eq!(ninf, ninf * one);
            assert_eq!(pinf, &one * &pinf);
            assert_eq!(pinf, one * pinf);
            assert_eq!(pinf, &pinf * &one);
            assert_eq!(pinf, pinf * one);

            // let two = Frac::new(2, 1);
            let two = $_2;

            assert_eq!(two, two);
            assert_eq!(nil, &two * &nil);
            assert_eq!(nil, two * nil);
            assert_eq!(nil, &nil * &two);
            assert_eq!(nil, nil * two);
            assert_eq!(one, &one * &one);
            assert_eq!(one, one * one);
            assert_eq!(nan, &two * &nan);
            assert_eq!(nan, two * nan);
            assert_eq!(nan, &nan * &two);
            assert_eq!(nan, nan * two);
            assert_eq!(ninf, &two * &ninf);
            assert_eq!(ninf, two * ninf);
            assert_eq!(ninf, &ninf * &two);
            assert_eq!(ninf, ninf * two);
            assert_eq!(pinf, &two * &pinf);
            assert_eq!(pinf, two * pinf);
            assert_eq!(pinf, &pinf * &two);
            assert_eq!(pinf, pinf * two);

            let mone = -one;
            let mtwo = -two;

            assert_eq!(mone, mone);
            assert_eq!(nil, &mone * &nil);
            assert_eq!(nil, mone * nil);
            assert_eq!(nil, &nil * &mone);
            assert_eq!(nil, nil * mone);
            assert_eq!(nan, &mone * &nan);
            assert_eq!(nan, mone * nan);
            assert_eq!(nan, &nan * &mone);
            assert_eq!(nan, nan * mone);
            assert_eq!(mone, &mone * &one);
            assert_eq!(mone, mone * one);
            assert_eq!(mone, &one * &mone);
            assert_eq!(mone, one * mone);
            assert_eq!(mtwo, &mone * &two);
            assert_eq!(mtwo, mone * two);
            assert_eq!(mtwo, &two * &mone);
            assert_eq!(mtwo, two * mone);
            assert_eq!(nan, &mtwo * &nan);
            assert_eq!(nan, mtwo * nan);
            assert_eq!(nan, &nan * &mtwo);
            assert_eq!(nan, nan * mtwo);
            assert_eq!(pinf, &mone * &ninf);
            assert_eq!(pinf, mone * ninf);
            assert_eq!(pinf, &ninf * &mone);
            assert_eq!(pinf, ninf * mone);
            assert_eq!(ninf, &mone * &pinf);
            assert_eq!(ninf, mone * pinf);
            assert_eq!(ninf, &pinf * &mone);
            assert_eq!(ninf, pinf * mone);
        }

        #[test]
        fn op_mul_assign() {
            let nan = $nan;  // Frac::nan();
            let nin = $nin;  // Frac::neg_infinity();
            let pin = $pin;  // Frac::infinity();
            let nil = $_0;  // Frac::new(0, 1);
            let one = $_1;  // Frac::new(1, 1);
            let two = $_2;  // Frac::new(2, 1);

            let mut nan_ = $nan;  // Frac::nan();
            nan_ *= nan;
            assert_eq!(nan, nan_);

            nan_ *= nin;
            assert_eq!(nan, nan_);

            nan_ *= pin;
            assert_eq!(nan, nan_);

            nan_ *= nil;
            assert_eq!(nan, nan_);

            let mut nil_ = nil;
            nil_ *= nil_;
            assert_eq!(nil, nil_);

            nil_ *= one;
            assert_eq!(nil_, nil);

            let mut one_ = one;
            one_ *= two;
            assert_eq!(one_, two);

            let mut one_ = one;
            one_ *= -two;
            assert_eq!(one_, -two);
        }

        #[test]
        fn op_mul_assign_refs() {
            let nan = $nan;  // Frac::nan();
            let nin = $nin;  // Frac::neg_infinity();
            let pin = $pin;  // Frac::infinity();
            let nil = $_0;  // Frac::new(0, 1);
            let one = $_1;  // Frac::new(1, 1);
            let two = $_2;  // Frac::new(2, 1);

            let mut nan_ = $nan;  // Frac::nan();
            nan_ *= &nan;
            assert_eq!(nan, nan_);

            nan_ *= &nin;
            assert_eq!(nan, nan_);

            nan_ *= &pin;
            assert_eq!(nan, nan_);

            nan_ *= &nil;
            assert_eq!(nan, nan_);

            let mut nil_ = nil;
            nil_ *= &nil;
            assert_eq!(nil, nil_);

            nil_ *= &one;
            assert_eq!(nil_, nil);

            let mut one_ = one;
            one_ *= &two;
            assert_eq!(one_, two);

            let mut one_ = one;
            one_ *= &-two;
            assert_eq!(one_, -two);
        }


        #[test]
        fn op_div() {
            // let nan: Frac = GenericFraction::NaN;
            // let ninf: Frac = GenericFraction::Infinity(Sign::Minus);
            // let pinf: Frac = GenericFraction::Infinity(Sign::Plus);

            let nan = $nan;
            let ninf = $nin;
            let pinf = $pin;

            assert_eq!(nan, nan);
            assert_eq!(nan, &nan / &nan);
            assert_eq!(nan, nan / nan);
            assert_eq!(nan, &nan / &ninf);
            assert_eq!(nan, nan / ninf);
            assert_eq!(nan, &nan / &pinf);
            assert_eq!(nan, nan / pinf);
            assert_eq!(nan, &ninf / &nan);
            assert_eq!(nan, ninf / nan);
            assert_eq!(nan, &pinf / &nan);
            assert_eq!(nan, pinf / nan);

            assert_eq!(nan, &ninf / &ninf);
            assert_eq!(nan, ninf / ninf);
            assert_eq!(nan, &pinf / &pinf);
            assert_eq!(nan, pinf / pinf);
            assert_eq!(nan, &pinf / &ninf);
            assert_eq!(nan, pinf / ninf);
            assert_eq!(nan, &ninf / &pinf);
            assert_eq!(nan, ninf / pinf);

            // let nil = Frac::new(0, 1);
            let nil = $_0;

            assert_eq!(nil, nil);
            assert_eq!(nan, &nil / &nil);
            assert_eq!(nan, nil / nil);
            assert_eq!(nan, &nil / &nan);
            assert_eq!(nan, nil / nan);
            assert_eq!(nan, &nan / &nil);
            assert_eq!(nan, nan / nil);
            assert_eq!(nil, &nil / &ninf);
            assert_eq!(nil, nil / ninf);
            assert_eq!(ninf, &ninf / &nil);
            assert_eq!(ninf, ninf / nil);
            assert_eq!(nil, &nil / &pinf);
            assert_eq!(nil, nil / pinf);
            assert_eq!(pinf, &pinf / &nil);
            assert_eq!(pinf, pinf / nil);

            // let one = Frac::new(1, 1);
            let one = $_1;

            assert_eq!(one, one);
            assert_eq!(one, &one / &one);
            assert_eq!(one, one / one);
            assert_eq!(pinf, &one / &nil);
            assert_eq!(pinf, one / nil);
            assert_eq!(nil, &nil / &one);
            assert_eq!(nil, nil / one);
            assert_eq!(nan, &one / &nan);
            assert_eq!(nan, one / nan);
            assert_eq!(nan, &nan / &one);
            assert_eq!(nan, nan / one);
            assert_eq!(nil, &one / &ninf);
            assert_eq!(nil, one / ninf);
            assert_eq!(ninf, &ninf / &one);
            assert_eq!(ninf, ninf / one);
            assert_eq!(nil, &one / &pinf);
            assert_eq!(nil, one / pinf);
            assert_eq!(pinf, &pinf / &one);
            assert_eq!(pinf, pinf / one);

            // let two = Frac::new(2, 1);
            let two = $_2;

            assert_eq!(two, two);
            assert_eq!(pinf, &two / &nil);
            assert_eq!(pinf, two / nil);
            assert_eq!(nil, &nil / &two);
            assert_eq!(nil, nil / two);
            assert_eq!(nan, &two / &nan);
            assert_eq!(nan, two / nan);
            assert_eq!(nan, &nan / &two);
            assert_eq!(nan, nan / two);
            assert_eq!(nil, &two / &ninf);
            assert_eq!(nil, two / ninf);
            assert_eq!(ninf, &ninf / &two);
            assert_eq!(ninf, ninf / two);
            assert_eq!(nil, &two / &pinf);
            assert_eq!(nil, two / pinf);
            assert_eq!(pinf, &pinf / &two);
            assert_eq!(pinf, pinf / two);

            let mone = -one;
            let mtwo = -two;

            assert_eq!(mone, mone);

            assert_eq!(ninf, &mone / &nil);
            assert_eq!(ninf, mone / nil);
            assert_eq!(nil, &nil / &mone);
            assert_eq!(nil, nil / mone);
            assert_eq!(nan, &mone / &nan);
            assert_eq!(nan, mone / nan);
            assert_eq!(nan, &nan / &mone);
            assert_eq!(nan, nan / mone);
            assert_eq!(mone, &mone / &one);
            assert_eq!(mone, mone / one);
            assert_eq!(mone, &one / &mone);
            assert_eq!(mone, one / mone);
            assert_eq!(format!("{:.1}", -$half), format!("{:.1}", &mone / &two));
            assert_eq!(format!("{:.1}", -$half), format!("{:.1}", mone / two));
            assert_eq!(mtwo, &two / &mone);
            assert_eq!(mtwo, two / mone);
            assert_eq!(nan, &mtwo / &nan);
            assert_eq!(nan, mtwo / nan);
            assert_eq!(nan, &nan / &mtwo);
            assert_eq!(nan, nan / mtwo);
            assert_eq!(nil, &mone / &ninf);
            assert_eq!(nil, mone / ninf);
            assert_eq!(pinf, &ninf / &mone);
            assert_eq!(pinf, ninf / mone);
            assert_eq!(nil, &mone / &pinf);
            assert_eq!(nil, mone / pinf);
            assert_eq!(ninf, &pinf / &mone);
            assert_eq!(ninf, pinf / mone);
        }

        #[test]
        fn op_div_assign() {
            let nan = $nan;  // Frac::nan();
            let nin = $nin;  // Frac::neg_infinity();
            let pin = $pin;  // Frac::infinity();
            let nil = $_0;  // Frac::new(0, 1);
            let one = $_1;  // Frac::new(1, 1);
            let two = $_2;  // Frac::new(2, 1);

            let mut nan_ = $nan;  // Frac::nan();
            nan_ /= nan;
            assert_eq!(nan, nan_);

            nan_ /= nin;
            assert_eq!(nan, nan_);

            nan_ /= pin;
            assert_eq!(nan, nan_);

            nan_ /= nil;
            assert_eq!(nan, nan_);

            let mut nil_ = nil;
            nil_ /= nil_;
            assert_eq!(nan, nil_);

            let mut nil_ = nil;
            nil_ /= one;
            assert_eq!(nil_, nil);

            let mut one_ = one;
            one_ /= two;
            assert_eq!(format!("{:.1}", one_), format!("{:.1}", $half));

            let mut one_ = one;
            one_ /= -two;
            assert_eq!(format!("{:.1}", one_), format!("{:.1}", -$half));
        }


        #[test]
        fn op_div_assign_refs() {
            let nan = $nan;  // Frac::nan();
            let nin = $nin;  // Frac::neg_infinity();
            let pin = $pin;  // Frac::infinity();
            let nil = $_0;  // Frac::new(0, 1);
            let one = $_1;  // Frac::new(1, 1);
            let two = $_2;  // Frac::new(2, 1);

            let mut nan_ = $nan;  // Frac::nan();
            nan_ /= &nan;
            assert_eq!(nan, nan_);

            nan_ /= &nin;
            assert_eq!(nan, nan_);

            nan_ /= &pin;
            assert_eq!(nan, nan_);

            nan_ /= &nil;
            assert_eq!(nan, nan_);

            let mut nil_ = nil;
            nil_ /= &nil;
            assert_eq!(nan, nil_);

            let mut nil_ = nil;
            nil_ /= &one;
            assert_eq!(nil_, nil);

            let mut one_ = one;
            one_ /= &two;
            assert_eq!(format!("{:.1}", one_), format!("{:.1}", $half));

            let mut one_ = one;
            one_ /= &-two;
            assert_eq!(format!("{:.1}", one_), format!("{:.1}", -$half));
        }

        #[test]
        fn op_rem() {
            //let nan: Frac = GenericFraction::NaN;
            //let ninf: Frac = GenericFraction::Infinity(Sign::Minus);
            //let pinf: Frac = GenericFraction::Infinity(Sign::Plus);
            let nan = $nan;
            let ninf = $nin;
            let pinf = $pin;

            assert_eq!(nan, nan);
            assert_eq!(nan, &nan % &nan);
            assert_eq!(nan, nan % nan);
            assert_eq!(nan, &nan % &ninf);
            assert_eq!(nan, nan % ninf);
            assert_eq!(nan, &nan % &pinf);
            assert_eq!(nan, nan % pinf);
            assert_eq!(nan, &ninf % &nan);
            assert_eq!(nan, ninf % nan);
            assert_eq!(nan, &pinf % &nan);
            assert_eq!(nan, pinf % nan);

            assert_eq!(nan, &ninf % &ninf);
            assert_eq!(nan, ninf % ninf);
            assert_eq!(nan, &pinf % &pinf);
            assert_eq!(nan, pinf % pinf);
            assert_eq!(nan, &pinf % &ninf);
            assert_eq!(nan, pinf % ninf);
            assert_eq!(nan, &ninf % &pinf);
            assert_eq!(nan, ninf % pinf);

            // let nil = Frac::new(0, 1);
            let nil = $_0;

            assert_eq!(nil, nil);
            assert_eq!(nan, &nil % &nil);
            assert_eq!(nan, nil % nil);
            assert_eq!(nan, &nil % &nan);
            assert_eq!(nan, nil % nan);
            assert_eq!(nan, &nan % &nil);
            assert_eq!(nan, nan % nil);
            assert_eq!(nil, &nil % &ninf);
            assert_eq!(nil, nil % ninf);
            assert_eq!(nan, &ninf % &nil);
            assert_eq!(nan, ninf % nil);
            assert_eq!(nil, &nil % &pinf);
            assert_eq!(nil, nil % pinf);
            assert_eq!(nan, &pinf % &nil);
            assert_eq!(nan, pinf % nil);

            // let one = Frac::new(1, 1);
            let one = $_1;

            assert_eq!(one, one);
            assert_eq!(nil, &one % &one);
            assert_eq!(nil, one % one);
            assert_eq!(nan, &one % &nil);
            assert_eq!(nan, one % nil);
            assert_eq!(nil, &nil % &one);
            assert_eq!(nil, nil % one);
            assert_eq!(nan, &one % &nan);
            assert_eq!(nan, one % nan);
            assert_eq!(nan, &nan % &one);
            assert_eq!(nan, nan % one);
            assert_eq!(one, &one % &ninf);
            assert_eq!(one, one % ninf);
            assert_eq!(nan, &ninf % &one);
            assert_eq!(nan, ninf % one);
            assert_eq!(one, &one % &pinf);
            assert_eq!(one, one % pinf);
            assert_eq!(nan, &pinf % &one);
            assert_eq!(nan, pinf % one);

            // let two = Frac::new(2, 1);
            let two = $_2;

            assert_eq!(two, two);
            assert_eq!(nan, &two % &nil);
            assert_eq!(nan, two % nil);
            assert_eq!(nil, &nil % &two);
            assert_eq!(nil, nil % two);
            assert_eq!(nan, &two % &nan);
            assert_eq!(nan, two % nan);
            assert_eq!(nan, &nan % &two);
            assert_eq!(nan, nan % two);
            assert_eq!(two, &two % &ninf);
            assert_eq!(two, two % ninf);
            assert_eq!(nan, &ninf % &two);
            assert_eq!(nan, ninf % two);
            assert_eq!(two, &two % &pinf);
            assert_eq!(two, two % pinf);
            assert_eq!(nan, &pinf % &two);
            assert_eq!(nan, pinf % two);

            let mone = -one;
            let mtwo = -two;

            assert_eq!(mone, mone);

            assert_eq!(nan, &mone % &nil);
            assert_eq!(nan, mone % nil);
            assert_eq!(nil, &nil % &mone);
            assert_eq!(nil, nil % mone);
            assert_eq!(nan, &mone % &nan);
            assert_eq!(nan, mone % nan);
            assert_eq!(nan, &nan % &mone);
            assert_eq!(nan, nan % mone);
            assert_eq!(nil, &mone % &one);
            assert_eq!(nil, mone % one);
            assert_eq!(nil, &one % &mone);
            assert_eq!(nil, one % mone);
            assert_eq!(mone, &mone % &two);
            assert_eq!(mone, mone % two);
            assert_eq!(nil, &two % &mone);
            assert_eq!(nil, two % mone);
            assert_eq!(nan, &mtwo % &nan);
            assert_eq!(nan, mtwo % nan);
            assert_eq!(nan, &nan % &mtwo);
            assert_eq!(nan, nan % mtwo);
            assert_eq!(mone, &mone % &ninf);
            assert_eq!(mone, mone % ninf);
            assert_eq!(nan, &ninf % &mone);
            assert_eq!(nan, ninf % mone);
            assert_eq!(mone, &mone % &pinf);
            assert_eq!(mone, mone % pinf);
            assert_eq!(nan, &pinf % &mone);
            assert_eq!(nan, pinf % mone);
        }


        #[test]
        fn op_rem_assign() {
            let nan = $nan;  // Frac::nan();
            let nin = $nin;  // Frac::neg_infinity();
            let pin = $pin;  // Frac::infinity();
            let nil = $_0;  // Frac::new(0, 1);
            let one = $_1;  // Frac::new(1, 1);
            let two = $_2;  // Frac::new(2, 1);
            let thr = $_3;  // Frac::new(3, 1);

            let mut nan_ = $nan;  // Frac::nan();
            nan_ %= nan;
            assert_eq!(nan, nan_);

            nan_ %= nin;
            assert_eq!(nan, nan_);

            nan_ %= pin;
            assert_eq!(nan, nan_);

            nan_ %= nil;
            assert_eq!(nan, nan_);

            let mut nil_ = nil;
            nil_ %= nil_;
            assert_eq!(nan, nil_);

            let mut nil_ = nil;
            nil_ %= one;
            assert_eq!(nil_, nil);

            let mut one_ = one;
            one_ %= two;
            assert_eq!(one_, $_1);

            let mut one_ = one;
            one_ %= -two;
            assert_eq!(one_, $_1);

            let mut two_ = two;
            two_ %= two;
            assert_eq!(two_, nil);

            let mut thr_ = thr;
            thr_ %= two;
            assert_eq!(thr_, one);
        }


        #[test]
        fn op_rem_assign_refs() {
            let nan = $nan;  //Frac::nan();
            let nin = $nin;  //Frac::neg_infinity();
            let pin = $pin;  //Frac::infinity();
            let nil = $_0;  // Frac::new(0, 1);
            let one = $_1;  // Frac::new(1, 1);
            let two = $_2;  //Frac::new(2, 1);
            let thr = $_3;  // Frac::new(3, 1);

            let mut nan_ = $nan;  //Frac::nan();
            nan_ %= &nan;
            assert_eq!(nan, nan_);

            nan_ %= &nin;
            assert_eq!(nan, nan_);

            nan_ %= &pin;
            assert_eq!(nan, nan_);

            nan_ %= &nil;
            assert_eq!(nan, nan_);

            let mut nil_ = nil;
            nil_ %= &nil;
            assert_eq!(nan, nil_);

            let mut nil_ = nil;
            nil_ %= &one;
            assert_eq!(nil_, nil);

            let mut one_ = one;
            one_ %= &two;
            assert_eq!(one_, $_1);

            let mut one_ = one;
            one_ %= &-two;
            assert_eq!(one_, $_1);

            let mut two_ = two;
            two_ %= &two;
            assert_eq!(two_, nil);

            let mut thr_ = thr;
            thr_ %= &two;
            assert_eq!(thr_, one);
        }
    };
}
