//! Multiplication operator trait implementation
//!

use super::*;
use crate::stdlib::mem::swap;


impl Mul<BigDecimal> for BigDecimal {
    type Output = BigDecimal;

    #[inline]
    fn mul(mut self, rhs: BigDecimal) -> BigDecimal {
        if self.is_one() {
            rhs
        } else if rhs.is_one() {
            self
        } else {
            self.scale += rhs.scale;
            self.int_val *= rhs.int_val;
            self
        }
    }
}

impl<'a> Mul<&'a BigDecimal> for BigDecimal {
    type Output = BigDecimal;

    #[inline]
    fn mul(mut self, rhs: &'a BigDecimal) -> BigDecimal {
        if self.is_one() {
            self.scale = rhs.scale;
            self.int_val.set_zero();
            self.int_val += &rhs.int_val;
        } else if rhs.is_zero() {
            self.scale = 0;
            self.int_val.set_zero();
        } else if !self.is_zero() && !rhs.is_one() {
            self.scale += rhs.scale;
            self.int_val *= &rhs.int_val;
        }
        self
    }
}

impl Mul<BigDecimal> for &BigDecimal {
    type Output = BigDecimal;

    #[inline]
    fn mul(self, rhs: BigDecimal) -> BigDecimal {
        rhs * self
    }
}

impl Mul<&BigDecimal> for &BigDecimal {
    type Output = BigDecimal;

    #[inline]
    fn mul(self, rhs: &BigDecimal) -> BigDecimal {
        if self.is_one() {
            rhs.normalized()
        } else if rhs.is_one() {
            self.normalized()
        } else {
            let scale = self.scale + rhs.scale;
            BigDecimal::new(&self.int_val * &rhs.int_val, scale)
        }
    }
}

impl Mul<BigInt> for BigDecimal {
    type Output = BigDecimal;

    #[inline]
    fn mul(mut self, rhs: BigInt) -> BigDecimal {
        self.int_val *= rhs;
        self
    }
}

impl Mul<&BigInt> for BigDecimal {
    type Output = BigDecimal;

    #[inline]
    fn mul(mut self, rhs: &BigInt) -> BigDecimal {
        self.int_val *= rhs;
        self
    }
}

impl Mul<BigInt> for &BigDecimal {
    type Output = BigDecimal;

    #[inline]
    fn mul(self, mut rhs: BigInt) -> BigDecimal {
        rhs *= &self.int_val;
        BigDecimal::new(rhs, self.scale)
    }
}

impl Mul<&BigInt> for &BigDecimal {
    type Output = BigDecimal;

    #[inline]
    fn mul(self, rhs: &BigInt) -> BigDecimal {
        if rhs.is_one() {
            self.normalized()
        } else if self.is_one() {
            BigDecimal::new(rhs.clone(), 0)
        } else {
            let value = &self.int_val * rhs;
            BigDecimal::new(value, self.scale)
        }
    }
}

impl Mul<BigDecimal> for BigInt {
    type Output = BigDecimal;

    #[inline]
    fn mul(mut self, mut rhs: BigDecimal) -> BigDecimal {
        if rhs.is_one() {
            rhs.scale = 0;
            swap(&mut rhs.int_val, &mut self);
        } else if !self.is_one() {
            rhs.int_val *= self;
        }
        rhs
    }
}

impl Mul<BigDecimal> for &BigInt {
    type Output = BigDecimal;

    #[inline]
    fn mul(self, mut rhs: BigDecimal) -> BigDecimal {
        if self.is_one() {
            rhs.normalized()
        } else if rhs.is_one() {
            rhs.int_val.set_zero();
            rhs.int_val += self;
            rhs.scale = 0;
            rhs
        } else {
            rhs.int_val *= self;
            rhs
        }
    }
}

impl Mul<&BigDecimal> for &BigInt {
    type Output = BigDecimal;

    #[inline]
    fn mul(self, rhs: &BigDecimal) -> BigDecimal {
        if self.is_one() {
            rhs.normalized()
        } else if rhs.is_one() {
            BigDecimal::new(self.clone(), 0)
        } else {
            let value = &rhs.int_val * self;
            BigDecimal::new(value, rhs.scale)
        }
    }
}

impl Mul<&BigDecimal> for BigInt {
    type Output = BigDecimal;

    #[inline]
    fn mul(mut self, rhs: &BigDecimal) -> BigDecimal {
        if self.is_one() {
            rhs.normalized()
        } else if rhs.is_one() {
            BigDecimal::new(self, 0)
        } else {
            self *= &rhs.int_val;
            BigDecimal::new(self, rhs.scale)
        }
    }
}

forward_val_assignop!(impl MulAssign for BigDecimal, mul_assign);

impl MulAssign<&BigDecimal> for BigDecimal {
    #[inline]
    fn mul_assign(&mut self, rhs: &BigDecimal) {
        if rhs.is_one() {
            return;
        }
        self.scale += rhs.scale;
        self.int_val = &self.int_val * &rhs.int_val;
    }
}

impl MulAssign<&BigInt> for BigDecimal {
    #[inline]
    fn mul_assign(&mut self, rhs: &BigInt) {
        if rhs.is_one() {
            return;
        }
        self.int_val *= rhs;
    }
}

impl MulAssign<BigInt> for BigDecimal {
    #[inline]
    fn mul_assign(&mut self, rhs: BigInt) {
        *self *= &rhs
    }
}


#[cfg(test)]
#[allow(non_snake_case)]
mod bigdecimal_tests {
    use super::*;
    use num_traits::{ToPrimitive, FromPrimitive, Signed, Zero, One};
    use num_bigint;
    use paste::paste;

    macro_rules! impl_test {
        ($name:ident; $a:literal * $b:literal => $expected:literal) => {
            #[test]
            fn $name() {
                let mut a: BigDecimal = $a.parse().unwrap();
                let b: BigDecimal = $b.parse().unwrap();
                let expected = $expected.parse().unwrap();

                let prod = a.clone() * b.clone();
                assert_eq!(prod, expected);
                assert_eq!(prod.scale, expected.scale);

                assert_eq!(a.clone() * &b, expected);
                assert_eq!(&a * b.clone(), expected);
                assert_eq!(&a * &b, expected);

                a *= b;
                assert_eq!(a, expected);
                assert_eq!(a.scale, expected.scale);
            }
        };
        ($name:ident; $bigt:ty; $a:literal * $b:literal => $expected:literal) => {
            #[test]
            fn $name() {
                let a: BigDecimal = $a.parse().unwrap();
                let b: $bigt = $b.parse().unwrap();
                let c = $expected.parse().unwrap();

                let prod = a.clone() * b.clone();
                assert_eq!(prod, c);
                assert_eq!(prod.scale, c.scale);

                assert_eq!(b.clone() * a.clone(), c);
                assert_eq!(a.clone() * &b, c);
                assert_eq!(b.clone() * &a, c);
                assert_eq!(&a * b.clone(), c);
                assert_eq!(&b * a.clone(), c);
                assert_eq!(&a * &b, c);
                assert_eq!(&b * &a, c);
            }
        };
    }

    impl_test!(case_2_1; "2" * "1" => "2");
    impl_test!(case_12d34_1d234; "12.34" * "1.234" => "15.22756");
    impl_test!(case_2e1_1; "2e1" * "1" => "2e1");
    impl_test!(case_3_d333333; "3" * ".333333" => "0.999999");
    impl_test!(case_2389472934723_209481029831; "2389472934723" * "209481029831" => "500549251119075878721813");
    impl_test!(case_1ed450_1e500; "1e-450" * "1e500" => "0.1e51");
    impl_test!(case_n995052931ddd_4d523087321; "-995052931372975485719.533153137" * "4.523087321" => "-4500711297616988541501.836966993116075977");
    impl_test!(case_995052931ddd_n4d523087321; "995052931372975485719.533153137" * "-4.523087321" => "-4500711297616988541501.836966993116075977");
    impl_test!(case_n8d37664968_n4d523087321; "-8.37664968" * "-1.9086963714056968482094712882596748" => "15.988480848752691653730876239769592670324064");
    impl_test!(case_n8d37664968_0; "-8.37664968" * "0" => "0.00000000");

    impl_test!(case_8d561_10; BigInt; "8.561" * "10" => "85.610");

    // Test multiplication between big decimal and big integer
    impl_test!(case_10000_638655273892892437; BigInt; "10000" * "638655273892892437" => "6386552738928924370000");
    impl_test!(case_1en10_n9056180052657301; BigInt; "1e-10" * "-9056180052657301" => "-905618.0052657301");
    impl_test!(case_n9en1_n368408638655273892892437473; BigInt; "-9e-1" * "-368408638655273892892437473" => "331567774789746503603193725.7");
    impl_test!(case_n1d175470587012343730098_577575785; BigInt; "-1.175470587012343730098" * "577575785" => "-678923347.038065234601180476930");
}
