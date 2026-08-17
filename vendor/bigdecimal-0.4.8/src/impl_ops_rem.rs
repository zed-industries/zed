//! Remainder implementations

use super::*;


impl Rem<BigDecimal> for BigDecimal {
    type Output = BigDecimal;

    #[inline]
    fn rem(self, other: BigDecimal) -> BigDecimal {
        let scale = cmp::max(self.scale, other.scale);

        let num = self.take_and_scale(scale).int_val;
        let den = other.take_and_scale(scale).int_val;

        BigDecimal::new(num % den, scale)
    }
}

impl Rem<&BigDecimal> for BigDecimal {
    type Output = BigDecimal;

    #[inline]
    fn rem(self, other: &BigDecimal) -> BigDecimal {
        let scale = cmp::max(self.scale, other.scale);
        let num = self.take_and_scale(scale).int_val;
        let den = &other.int_val;

        let result = if scale == other.scale {
            num % den
        } else {
            num % (den * ten_to_the((scale - other.scale) as u64))
        };
        BigDecimal::new(result, scale)
    }
}

impl Rem<BigDecimal> for &BigDecimal {
    type Output = BigDecimal;

    #[inline]
    fn rem(self, other: BigDecimal) -> BigDecimal {
        let scale = cmp::max(self.scale, other.scale);
        let num = &self.int_val;
        let den = other.take_and_scale(scale).int_val;

        let result = if scale == self.scale {
            num % den
        } else {
            let scaled_num = num * ten_to_the((scale - self.scale) as u64);
            scaled_num % den
        };

        BigDecimal::new(result, scale)
    }
}

impl Rem<&BigDecimal> for &BigDecimal {
    type Output = BigDecimal;

    #[inline]
    fn rem(self, other: &BigDecimal) -> BigDecimal {
        let scale = cmp::max(self.scale, other.scale);
        let num = &self.int_val;
        let den = &other.int_val;

        let result = match self.scale.cmp(&other.scale) {
            Ordering::Equal => num % den,
            Ordering::Less => {
                let scaled_num = num * ten_to_the((scale - self.scale) as u64);
                scaled_num % den
            }
            Ordering::Greater => {
                let scaled_den = den * ten_to_the((scale - other.scale) as u64);
                num % scaled_den
            }
        };
        BigDecimal::new(result, scale)
    }
}

impl RemAssign<&BigDecimal> for BigDecimal {
    fn rem_assign(&mut self, other: &BigDecimal) {
        let rem = (&*self).rem(other);
        *self = rem;
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use paste::paste;

    macro_rules! impl_case {
        ($a:literal % $b:literal => $c:literal ) => {
            paste! {
                impl_case!([< case_ $a _ $b >]: $a % $b => $c);
            }
        };
        ($name:ident: $a:literal % $b:literal => $c:literal ) => {
            #[test]
            fn $name() {
                let mut a: BigDecimal = $a.parse().unwrap();
                let b: BigDecimal = $b.parse().unwrap();
                let c: BigDecimal = $c.parse().unwrap();

                assert_eq!(a.clone() % b.clone(), c);

                assert_eq!(a.clone() % &b, c);
                assert_eq!(&a % b.clone(), c);
                assert_eq!(&a % &b, c);

                a %= &b;
                assert_eq!(a, c);
            }
        };
    }

    impl_case!("100" % "5" => "0");
    impl_case!("2e1" % "1" => "0");
    impl_case!("2" % "1" => "0");
    impl_case!("1" % "3" => "1");
    impl_case!("1" % "5e-1" => "0");
    impl_case!("15e-1" % "1" => "0.5");
    impl_case!("1" % "3e-2" => "1e-2");
    impl_case!("10" % "3e-3" => "0.001");
    impl_case!("3" % "2" => "1");
    impl_case!("1234e-2" % "1233e-3" => "0.01");

    impl_case!(case_neg3_2: "-3" % "2" => "-1");
    impl_case!(case_3_neg2: "3" % "-2" => "1");
    impl_case!(case_neg3_neg2: "3" % "-2" => "1");

    impl_case!(case_neg95eneg1_515eneg2: "-9.5" % "5.15" => "-4.35");


    #[cfg(property_tests)]
    mod prop {
        use super::*;
        use proptest::*;
        use num_traits::FromPrimitive;

        proptest! {
            #[test]
            fn quotient_and_remainder(f: f32, g: f32) {
                // ignore non-normal numbers
                prop_assume!(f.is_normal());
                prop_assume!(g.is_normal());
                prop_assume!(!g.is_zero());

                let (f, g) = if f.abs() > g.abs() {
                    (f, g)
                } else {
                    (g, f)
                };

                let a = BigDecimal::from_f32(f).unwrap();
                let b = BigDecimal::from_f32(g).unwrap();

                let r = &a % &b;
                let q = (&a / &b).with_scale(0);
                assert_eq!(a, q * b + r);
            }
        }
    }
}
