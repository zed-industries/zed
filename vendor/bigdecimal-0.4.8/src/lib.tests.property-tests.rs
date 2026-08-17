// Property tests to be included by lib.rs (if enabled)


mod arithmetic {
    use super::*;

    macro_rules! impl_test {
        ($t:ty) => {
            paste! { proptest! {
                #[test]
                fn [< add_ref $t >](n: $t, m: i128, e: i8) {
                    let d = BigDecimal::new(m.into(), e as i64);
                    let sum = n + &d;

                    let s1 = &d + n;
                    let s2 = d.clone() + n;

                    prop_assert_eq!(&sum, &s1);
                    prop_assert_eq!(&sum, &s2);

                    let mut s = d;
                    s += n;
                    prop_assert_eq!(sum, s);
                }

                #[test]
                fn [< sub_ $t >](n: $t, m: i128, e: i8) {
                    let d = BigDecimal::new(m.into(), e as i64);
                    let diff_n_d = n - &d;
                    let diff_d_n = d.clone() - n;
                    prop_assert_eq!(&diff_n_d, &diff_d_n.neg());

                    let mut a = d.clone();
                    a -= n;
                    prop_assert_eq!(&a, &diff_n_d.neg());
                }

                #[test]
                fn [< mul_ $t >](n: $t, m: i128, e: i8) {
                    let d = BigDecimal::new(m.into(), e as i64);
                    let prod_n_d = n * &d;
                    let prod_d_n = d.clone() * n;
                    prop_assert_eq!(&prod_n_d, &prod_d_n);

                    let mut r = d.clone();
                    r *= n;
                    prop_assert_eq!(&prod_n_d, &r);

                    let r = d.neg() * n;
                    prop_assert_eq!(prod_n_d.neg(), r);
                }

                #[test]
                fn [< div_ $t >](n: $t, m: i128, e: i8) {
                    prop_assume!(m != 0);

                    let d = BigDecimal::new(m.into(), e as i64);
                    let quotient_n_ref_d = n / &d;
                    let quotient_n_d = n / d.clone();
                    prop_assert_eq!(&quotient_n_ref_d, &quotient_n_d);

                    let prod = quotient_n_d * &d;
                    let diff = n - &prod;
                    // prop_assert!(dbg!(diff.scale) > 99);
                    prop_assert!(diff.abs() < BigDecimal::new(1.into(), 60));
                }
            } }
        };
        (float-div $t:ty) => {
            paste! { proptest! {
                #[test]
                fn [< div_ $t >](n: $t, m: i128, e: i8) {
                    prop_assume!(m != 0);

                    let d = BigDecimal::new(m.into(), e as i64);
                    let quotient_n_ref_d = n / &d;
                    let quotient_n_d = n / d.clone();
                    prop_assert_eq!(&quotient_n_ref_d, &quotient_n_d);

                    let d = BigDecimal::new(m.into(), e as i64);
                    let quotient_ref_d_n = &d / n;
                    let quotient_d_n = d.clone() / n;
                    prop_assert_eq!(&quotient_ref_d_n, &quotient_d_n);

                    let mut q = d.clone();
                    q /= n;
                    prop_assert_eq!(&q, &quotient_d_n);
                }
            } }
        };
    }

    impl_test!(u8);
    impl_test!(u16);
    impl_test!(u32);
    impl_test!(u64);
    impl_test!(u128);

    impl_test!(i8);
    impl_test!(i16);
    impl_test!(i32);
    impl_test!(i64);
    impl_test!(i128);

    impl_test!(float-div f32);
    impl_test!(float-div f64);

    proptest! {
        #[test]
        fn square(f: f32) {
            // ignore non-normal numbers
            prop_assume!(f.is_normal());

            let n: BigDecimal = BigDecimal::from_f32(f).unwrap();
            let n_times_n = &n * &n;

            prop_assert_eq!(n_times_n, n.square())
        }
    }
}
