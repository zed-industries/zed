// Adds common vector trait implementations.
// The traits here should be supported for all types of $t and all sizes of vector.
macro_rules! impl_matn_common_traits {
    ($t:ty, $matn:ident, $vecn:ident) => {
        impl Default for $matn {
            #[inline(always)]
            fn default() -> Self {
                Self::IDENTITY
            }
        }

        impl Add<$matn> for $matn {
            type Output = Self;
            #[inline(always)]
            fn add(self, other: Self) -> Self::Output {
                Self(self.0.add_matrix(&other.0))
            }
        }

        impl AddAssign<$matn> for $matn {
            #[inline(always)]
            fn add_assign(&mut self, other: Self) {
                self.0 = self.0.add_matrix(&other.0);
            }
        }

        impl Sub<$matn> for $matn {
            type Output = Self;
            #[inline(always)]
            fn sub(self, other: Self) -> Self::Output {
                Self(self.0.sub_matrix(&other.0))
            }
        }

        impl SubAssign<$matn> for $matn {
            #[inline(always)]
            fn sub_assign(&mut self, other: Self) {
                self.0 = self.0.sub_matrix(&other.0);
            }
        }

        impl Neg for $matn {
            type Output = Self;
            #[inline(always)]
            fn neg(self) -> Self::Output {
                Self(self.0.neg_matrix())
            }
        }

        impl Mul<$matn> for $matn {
            type Output = Self;
            #[inline(always)]
            fn mul(self, other: Self) -> Self::Output {
                Self(self.0.mul_matrix(&other.0))
            }
        }

        impl MulAssign<$matn> for $matn {
            #[inline(always)]
            fn mul_assign(&mut self, other: Self) {
                self.0 = self.0.mul_matrix(&other.0);
            }
        }

        impl Mul<$vecn> for $matn {
            type Output = $vecn;
            #[inline(always)]
            fn mul(self, other: $vecn) -> Self::Output {
                $vecn(self.0.mul_vector(other.0))
            }
        }

        impl Mul<$matn> for $t {
            type Output = $matn;
            #[inline(always)]
            fn mul(self, other: $matn) -> Self::Output {
                $matn(other.0.mul_scalar(self))
            }
        }

        impl Mul<$t> for $matn {
            type Output = Self;
            #[inline(always)]
            fn mul(self, other: $t) -> Self::Output {
                Self(self.0.mul_scalar(other))
            }
        }

        impl MulAssign<$t> for $matn {
            #[inline(always)]
            fn mul_assign(&mut self, other: $t) {
                self.0 = self.0.mul_scalar(other);
            }
        }

        impl<'a> Sum<&'a Self> for $matn {
            fn sum<I>(iter: I) -> Self
            where
                I: Iterator<Item = &'a Self>,
            {
                iter.fold(Self::ZERO, |a, &b| Self::add(a, b))
            }
        }

        impl<'a> Product<&'a Self> for $matn {
            fn product<I>(iter: I) -> Self
            where
                I: Iterator<Item = &'a Self>,
            {
                iter.fold(Self::IDENTITY, |a, &b| Self::mul(a, b))
            }
        }
    };
}
