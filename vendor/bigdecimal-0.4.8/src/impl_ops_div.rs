//! Implement division

use super::*;

impl Div<BigDecimal> for BigDecimal {
    type Output = BigDecimal;
    #[inline]
    fn div(self, other: BigDecimal) -> BigDecimal {
        if other.is_zero() {
            panic!("Division by zero");
        }
        if self.is_zero() || other.is_one() {
            return self;
        }

        let scale = self.scale - other.scale;

        if self.int_val == other.int_val {
            return BigDecimal {
                int_val: 1.into(),
                scale: scale,
            };
        }

        let max_precision = DEFAULT_PRECISION;

        return impl_division(self.int_val, &other.int_val, scale, max_precision);
    }
}

impl<'a> Div<&'a BigDecimal> for BigDecimal {
    type Output = BigDecimal;
    #[inline]
    fn div(self, other: &'a BigDecimal) -> BigDecimal {
        if other.is_zero() {
            panic!("Division by zero");
        }
        if self.is_zero() || other.is_one() {
            return self;
        }

        let scale = self.scale - other.scale;

        if self.int_val == other.int_val {
            return BigDecimal {
                int_val: 1.into(),
                scale: scale,
            };
        }

        let max_precision = DEFAULT_PRECISION;

        return impl_division(self.int_val, &other.int_val, scale, max_precision);
    }
}

forward_ref_val_binop!(impl Div for BigDecimal, div);

impl Div<&BigDecimal> for &BigDecimal {
    type Output = BigDecimal;

    #[inline]
    fn div(self, other: &BigDecimal) -> BigDecimal {
        if other.is_zero() {
            panic!("Division by zero");
        }
        // TODO: Fix setting scale
        if self.is_zero() || other.is_one() {
            return self.clone();
        }

        let scale = self.scale - other.scale;

        let num_int = &self.int_val;
        let den_int = &other.int_val;

        if num_int == den_int {
            return BigDecimal {
                int_val: 1.into(),
                scale: scale,
            };
        }

        let max_precision = DEFAULT_PRECISION;

        return impl_division(num_int.clone(), den_int, scale, max_precision);
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_div() {
        let vals = vec![
            ("0", "1", "0"),
            ("0", "10", "0"),
            ("2", "1", "2"),
            ("2e1", "1", "2e1"),
            ("10", "10", "1"),
            ("100", "10.0", "1e1"),
            ("20.0", "200", ".1"),
            ("4", "2", "2.0"),
            ("15", "3", "5.0"),
            ("1", "2", "0.5"),
            ("1", "2e-2", "5e1"),
            ("1", "0.2", "5"),
            ("1.0", "0.02", "50"),
            ("1", "0.020", "5e1"),
            ("5.0", "4.00", "1.25"),
            ("5.0", "4.000", "1.25"),
            ("5", "4.000", "1.25"),
            ("5", "4", "125e-2"),
            ("100", "5", "20"),
            ("-50", "5", "-10"),
            ("200", "-5", "-40."),
            ("1", "3", ".3333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333"),
            ("-2", "-3", ".6666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666667"),
            ("-12.34", "1.233", "-10.00811030008110300081103000811030008110300081103000811030008110300081103000811030008110300081103001"),
            ("125348", "352.2283", "355.8714617763535752237966114591019517738921035021887792661748076460636467881768727839301952739175132"),
        ];

        for &(x, y, z) in vals.iter() {

            let a = BigDecimal::from_str(x).unwrap();
            let b = BigDecimal::from_str(y).unwrap();
            let c = BigDecimal::from_str(z).unwrap();

            assert_eq!(a.clone() / b.clone(), c);
            assert_eq!(a.clone() / &b, c);
            assert_eq!(&a / b.clone(), c);
            assert_eq!(&a / &b, c);
            // assert_eq!(q.scale, c.scale);

            // let mut q = a;
            // q /= b;
            // assert_eq!(q, c);
        }
    }

    #[test]
    #[should_panic(expected = "Division by zero")]
    fn test_division_by_zero_panics() {
        let x = BigDecimal::from_str("3.14").unwrap();
        let _r = x / 0;
    }

    #[test]
    #[should_panic(expected = "Division by zero")]
    fn test_division_by_zero_panics_v2() {
        let x = BigDecimal::from_str("3.14").unwrap();
        let _r = x / BigDecimal::zero();
    }

    #[test]
    fn test_division_by_large_number() {
        let n = 1u8;
        let d: BigDecimal = "79437738588056219546528239237352667078".parse().unwrap();

        let quotient_n_ref_d = n / &d;
        let quotient_n_d = n / d.clone();
        assert_eq!(quotient_n_ref_d, quotient_n_d);

        assert_eq!(quotient_n_ref_d, "1.258847517281104957975270408416632052090243053529147458917576143852500316808428812104171430669001064E-38".parse().unwrap());
    }
}
