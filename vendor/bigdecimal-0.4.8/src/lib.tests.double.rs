// Test BigDecimal::double

macro_rules! impl_case {
    ($name:ident : $a:literal => $ex:literal ) => {
        paste! {
            #[test]
            fn $name() {
                let value = BigDecimal::from_str($a).unwrap();
                let expected = BigDecimal::from_str($ex).unwrap();
                let result = value.double();
                assert_eq!(result, expected);
                assert_eq!(result.int_val, expected.int_val);
                assert_eq!(result.scale, expected.scale);
            }
        }
    };
}

impl_case!(case_zero : "0" => "0");
impl_case!(case_1 : "1" => "2");
impl_case!(case_100Em2 : "1.00" => "2.00");
impl_case!(case_150Em2 : "1.50" => "3.00");
impl_case!(case_neg150Em2 : "-1.50" => "-3.00");
impl_case!(case_32909E4 : "32909E4" => "6.5818E+8");
impl_case!(case_1_1156024145937225657484 : "1.1156024145937225657484" => "2.2312048291874451314968");
