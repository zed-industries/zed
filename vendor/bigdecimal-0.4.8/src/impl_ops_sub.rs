//!
//! Subtraction operator trait implementation
//!

use crate::*;


impl Sub<BigDecimal> for BigDecimal {
    type Output = BigDecimal;

    #[inline]
    fn sub(self, rhs: BigDecimal) -> BigDecimal {
        if rhs.is_zero() {
            return self;
        }

        if self.is_zero() {
            return rhs.neg();
        }

        let mut lhs = self;
        match lhs.scale.cmp(&rhs.scale) {
            Ordering::Equal => {
                lhs.int_val -= rhs.int_val;
                lhs
            }
            Ordering::Less => {
                lhs.take_and_scale(rhs.scale) - rhs
            }
            Ordering::Greater => {
                let rhs = rhs.take_and_scale(lhs.scale);
                lhs - rhs
            },
        }
    }
}

impl Sub<BigDecimal> for &'_ BigDecimal {
    type Output = BigDecimal;

    #[inline]
    fn sub(self, rhs: BigDecimal) -> BigDecimal {
        self.to_ref() - rhs
    }
}

impl Sub<BigDecimal> for BigDecimalRef<'_> {
    type Output = BigDecimal;

    #[inline]
    fn sub(self, rhs: BigDecimal) -> BigDecimal {
        (rhs - self).neg()
    }
}

impl<'a, T: Into<BigDecimalRef<'a>>> Sub<T> for BigDecimal {
    type Output = BigDecimal;

    fn sub(mut self, rhs: T) -> BigDecimal {
        self.sub_assign(rhs);
        self
    }
}

impl<'a, T: Into<BigDecimalRef<'a>>> Sub<T> for &'_ BigDecimal {
    type Output = BigDecimal;

    fn sub(self, rhs: T) -> BigDecimal {
        let rhs = rhs.into();

        match self.scale.cmp(&rhs.scale) {
            Ordering::Equal => {
                self.clone() - rhs
            }
            Ordering::Less => {
                self.with_scale(rhs.scale) - rhs
            }
            Ordering::Greater => {
                self - rhs.to_owned_with_scale(self.scale)
            }
        }
    }
}

impl<'a, T: Into<BigDecimalRef<'a>>> Sub<T> for BigDecimalRef<'_> {
    type Output = BigDecimal;

    fn sub(self, rhs: T) -> BigDecimal {
        let rhs = rhs.into();

        match self.scale.cmp(&rhs.scale) {
            Ordering::Equal => self.to_owned() - rhs,
            Ordering::Less => self.to_owned_with_scale(rhs.scale) - rhs,
            Ordering::Greater => self - rhs.to_owned_with_scale(self.scale),
        }
    }
}

impl Sub<BigInt> for BigDecimal {
    type Output = BigDecimal;

    fn sub(mut self, rhs: BigInt) -> BigDecimal {
        self.sub_assign(rhs);
        self
    }
}


impl Sub<BigInt> for &'_ BigDecimal {
    type Output = BigDecimal;

    #[inline]
    fn sub(self, rhs: BigInt) -> BigDecimal {
        self.to_ref() - rhs
    }
}

impl Sub<BigInt> for BigDecimalRef<'_> {
    type Output = BigDecimal;

    #[inline]
    fn sub(self, rhs: BigInt) -> BigDecimal {
        self - BigDecimal::from(rhs)
    }
}

impl Sub<BigDecimal> for BigInt {
    type Output = BigDecimal;

    #[inline]
    fn sub(self, rhs: BigDecimal) -> BigDecimal {
        (rhs - self).neg()
    }
}

impl Sub<BigDecimal> for &BigInt {
    type Output = BigDecimal;

    #[inline]
    fn sub(self, rhs: BigDecimal) -> BigDecimal {
        (rhs - self).neg()
    }
}

impl<'a> Sub<BigDecimalRef<'a>> for BigInt {
    type Output = BigDecimal;

    #[inline]
    fn sub(self, rhs: BigDecimalRef<'a>) -> BigDecimal {
        (rhs - &self).neg()
    }
}


impl<'a> Sub<BigDecimalRef<'a>> for &BigInt {
    type Output = BigDecimal;

    #[inline]
    fn sub(self, rhs: BigDecimalRef<'a>) -> BigDecimal {
        (rhs - self).neg()
    }
}


impl SubAssign<BigDecimal> for BigDecimal {
    #[inline]
    fn sub_assign(&mut self, rhs: BigDecimal) {
        if rhs.is_zero() {
            return;
        }
        if self.is_zero() {
            *self = rhs.neg();
            return;
        }
        match self.scale.cmp(&rhs.scale) {
            Ordering::Equal => {
                self.int_val -= rhs.int_val;
            }
            Ordering::Less => {
                self.int_val *= ten_to_the((rhs.scale - self.scale) as u64);
                self.int_val -= rhs.int_val;
                self.scale = rhs.scale;
            }
            Ordering::Greater => {
                let mut rhs_int_val = rhs.int_val;
                rhs_int_val *= ten_to_the((self.scale - rhs.scale) as u64);
                self.int_val -= rhs_int_val;
            }
        }
    }
}

impl<'rhs, T: Into<BigDecimalRef<'rhs>>> SubAssign<T> for BigDecimal {
    #[inline]
    fn sub_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        if rhs.is_zero() {
            return;
        }
        if self.is_zero() {
            *self = rhs.neg().to_owned();
            return;
        }

        match self.scale.cmp(&rhs.scale) {
            Ordering::Equal => {
                self.int_val -= rhs.to_owned().int_val;
            }
            Ordering::Less => {
                self.int_val *= ten_to_the((rhs.scale - self.scale) as u64);
                self.int_val -= rhs.to_owned().int_val;
                self.scale = rhs.scale;
            }
            Ordering::Greater => {
                *self -= rhs.to_owned_with_scale(self.scale);
            }
        }
    }
}

impl SubAssign<BigInt> for BigDecimal {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: BigInt) {
        *self -= BigDecimal::new(rhs, 0)
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use paste::paste;

    macro_rules! impl_case {
        ($name:ident: $a:literal - $b:literal => $c:literal ) => {
            #[test]
            fn $name() {
                let a: BigDecimal = $a.parse().unwrap();
                let b: BigDecimal = $b.parse().unwrap();
                let c: BigDecimal = $c.parse().unwrap();

                assert_eq!(c, a.clone() - b.clone());

                assert_eq!(c, a.clone() - &b);
                assert_eq!(c, &a - b.clone());
                assert_eq!(c, &a - &b);

                assert_eq!(c, a.to_ref() - &b);
                assert_eq!(c, &a - b.to_ref());
                assert_eq!(c, a.to_ref() - b.to_ref());

                let mut n = a.clone();
                n -= b.to_ref();
                assert_eq!(n, c);

                let mut n = a.clone();
                n -= &b;
                assert_eq!(n, c);

                let mut n = a.clone();
                n -= b.clone();
                assert_eq!(n, c);

                let mut n = a.clone();
                (&mut n).sub_assign(b.clone());
                assert_eq!(n, c);
            }
        };
        ($name:ident: $a:literal - (int) $b:literal => $c:literal ) => {
            #[test]
            fn $name() {
                let a: BigDecimal = $a.parse().unwrap();
                let b: BigInt = $b.parse().unwrap();
                let expected: BigDecimal = $c.parse().unwrap();

                assert_eq!(expected, a.clone() - b.clone());
                assert_eq!(expected, a.clone() - &b);
                assert_eq!(expected, &a - &b);
                assert_eq!(expected, &a - b.clone());
                assert_eq!(expected, a.to_ref() - &b);

                let expected_neg = expected.clone().neg();
                assert_eq!(expected_neg, b.clone() - a.clone());
                assert_eq!(expected_neg, &b - a.to_ref());
                assert_eq!(expected_neg, &b - a.clone());
            }
        };
    }

    impl_case!(case_1234en2_1234en3: "12.34" - "1.234" => "11.106");
    impl_case!(case_1234en2_n1234en3: "12.34" - "-1.234" => "13.574");
    impl_case!(case_1234e6_1234en6: "1234e6" - "1234e-6" => "1233999999.998766");
    impl_case!(case_1234en6_1234e6: "1234e-6" - "1234e6" => "-1233999999.998766");
    impl_case!(case_712911676en6_4856259269250829: "712911676e-6" - "4856259269250829" => "-4856259269250116.088324");
    impl_case!(case_85616001e4_0: "85616001e4" - "0" => "85616001e4");
    impl_case!(case_0_520707672en5: "0" - "5207.07672" => "-520707672e-5");
    impl_case!(case_99291289e5_int0: "99291289e5" - (int)"0" => "99291289e5");
    impl_case!(case_7051277471570131en16_int1: "0.7051277471570131" - (int)"1" => "-0.2948722528429869");
    impl_case!(case_4068603022763836en8_intneg10: "40686030.22763836" - (int)"-10" => "40686040.22763836");

    #[cfg(property_tests)]
    mod prop {
        use super::*;
        use proptest::*;
        use num_traits::FromPrimitive;

        proptest! {
            #[test]
            fn sub_refs_and_owners(f: f32, g: f32) {
                // ignore non-normal numbers
                prop_assume!(f.is_normal());
                prop_assume!(g.is_normal());

                let a = BigDecimal::from_f32(f).unwrap();
                let b = BigDecimal::from_f32(g).unwrap();
                let own_minus_ref = a.clone() - &b;
                let ref_minus_own = &a - b.clone();

                let mut c = a.clone();
                c -= &b;

                let mut d = a.clone();
                d -= b;

                prop_assert_eq!(&own_minus_ref, &ref_minus_own);
                prop_assert_eq!(&c, &ref_minus_own);
                prop_assert_eq!(&d, &ref_minus_own);
            }

            #[test]
            fn subtraction_is_anticommunative(f: f32, g: f32) {
                // ignore non-normal numbers
                prop_assume!(f.is_normal());
                prop_assume!(g.is_normal());

                let a = BigDecimal::from_f32(f).unwrap();
                let b = BigDecimal::from_f32(g).unwrap();
                let a_minus_b = &a - &b;
                let b_minus_a = &b - &a;

                prop_assert_eq!(a_minus_b, -b_minus_a)
            }
        }
    }
}
