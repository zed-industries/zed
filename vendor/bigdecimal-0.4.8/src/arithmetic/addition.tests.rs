mod add_bigdecimals {
    use super::*;
    use paste::paste;

    macro_rules! impl_case {
        ( $name:ident: $a:literal + $b:literal = $c:literal ) => {

            #[test]
            fn $name() {
                let lhs: BigDecimal = $a.parse().unwrap();
                let rhs: BigDecimal = $b.parse().unwrap();

                let l_plus_r = add_bigdecimals(lhs.clone(), rhs.clone());
                let r_plus_l = add_bigdecimals(rhs, lhs);

                let expected: BigDecimal = $c.parse().unwrap();
                assert_eq!(expected.int_val, l_plus_r.int_val);
                assert_eq!(expected.scale, l_plus_r.scale);

                assert_eq!(expected.int_val, r_plus_l.int_val);
                assert_eq!(expected.scale, r_plus_l.scale);
            }

            paste! {
                #[test]
                fn [< $name _refs >]() {
                    let lhs: BigDecimal = $a.parse().unwrap();
                    let rhs: BigDecimal = $b.parse().unwrap();

                    let l_plus_r = add_bigdecimal_refs(&lhs, &rhs, None);
                    let r_plus_l = add_bigdecimal_refs(&rhs, &lhs, None);

                    let expected: BigDecimal = $c.parse().unwrap();
                    assert_eq!(expected.int_val, l_plus_r.int_val);
                    assert_eq!(expected.scale, l_plus_r.scale);

                    assert_eq!(expected.int_val, r_plus_l.int_val);
                    assert_eq!(expected.scale, r_plus_l.scale);
                }
            }
        };
    }

    impl_case!(case_1d2345_123d45: "1.2345" + "123.45" = "124.6845");
    impl_case!(case_123d43e5_1d2345: "123.43e5" + "1.2345" = "12343001.2345");

    impl_case!(case_0_0: "0" + "0" = "0");
    impl_case!(case_0_0d00: "0" + "0.00" = "0.00");
    impl_case!(case_10_0d00: "10" + "0.00" = "10.00");
    impl_case!(case_22132e2_0d0000: "22132e2" + "0.0000" = "2213200.0000");
    impl_case!(case_n316d79_0en6: "-316.79" + "0e-6" = "-316.790000");
}
