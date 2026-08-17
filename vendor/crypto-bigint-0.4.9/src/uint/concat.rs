// TODO(tarcieri): use `const_evaluatable_checked` when stable to make generic around bits.
macro_rules! impl_concat {
    ($(($name:ident, $bits:expr)),+) => {
        $(
            impl $name {
                /// Concatenate the two values, with `self` as most significant and `rhs`
                /// as the least significant.
                pub const fn concat(&self, rhs: &Self) -> UInt<{nlimbs!($bits) * 2}> {
                    let mut limbs = [Limb::ZERO; nlimbs!($bits) * 2];
                    let mut i = 0;
                    let mut j = 0;

                    while j < nlimbs!($bits) {
                        limbs[i] = rhs.limbs[j];
                        i += 1;
                        j += 1;
                    }

                    j = 0;
                    while j < nlimbs!($bits) {
                        limbs[i] = self.limbs[j];
                        i += 1;
                        j += 1;
                    }

                    UInt { limbs }
                }
            }

            impl Concat for $name {
                type Output = UInt<{nlimbs!($bits) * 2}>;

                fn concat(&self, rhs: &Self) -> Self::Output {
                    self.concat(rhs)
                }
            }

            impl From<($name, $name)> for UInt<{nlimbs!($bits) * 2}> {
                fn from(nums: ($name, $name)) -> UInt<{nlimbs!($bits) * 2}> {
                    nums.0.concat(&nums.1)
                }
            }
        )+
     };
}

#[cfg(test)]
mod tests {
    use crate::{U128, U64};

    #[test]
    fn concat() {
        let hi = U64::from_u64(0x0011223344556677);
        let lo = U64::from_u64(0x8899aabbccddeeff);
        assert_eq!(
            hi.concat(&lo),
            U128::from_be_hex("00112233445566778899aabbccddeeff")
        );
    }
}
