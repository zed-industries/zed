// TODO(tarcieri): use `const_evaluatable_checked` when stable to make generic around bits.
macro_rules! impl_split {
    ($(($name:ident, $bits:expr)),+) => {
        $(
            impl $name {
                /// Split this number in half, returning its high and low components
                /// respectively.
                pub const fn split(&self) -> (UInt<{nlimbs!($bits) / 2}>, UInt<{nlimbs!($bits) / 2}>) {
                    let mut lo = [Limb::ZERO; nlimbs!($bits) / 2];
                    let mut hi = [Limb::ZERO; nlimbs!($bits) / 2];
                    let mut i = 0;
                    let mut j = 0;

                    while j < (nlimbs!($bits) / 2) {
                        lo[j] = self.limbs[i];
                        i += 1;
                        j += 1;
                    }

                    j = 0;
                    while j < (nlimbs!($bits) / 2) {
                        hi[j] = self.limbs[i];
                        i += 1;
                        j += 1;
                    }

                    (UInt { limbs: hi }, UInt { limbs: lo })
                }
            }

            impl Split for $name {
                type Output = UInt<{nlimbs!($bits) / 2}>;

                fn split(&self) -> (Self::Output, Self::Output) {
                    self.split()
                }
            }

            impl From<$name> for (UInt<{nlimbs!($bits) / 2}>, UInt<{nlimbs!($bits) / 2}>) {
                fn from(num: $name) -> (UInt<{nlimbs!($bits) / 2}>, UInt<{nlimbs!($bits) / 2}>) {
                    num.split()
                }
            }
        )+
     };
}

#[cfg(test)]
mod tests {
    use crate::{U128, U64};

    #[test]
    fn split() {
        let (hi, lo) = U128::from_be_hex("00112233445566778899aabbccddeeff").split();
        assert_eq!(hi, U64::from_u64(0x0011223344556677));
        assert_eq!(lo, U64::from_u64(0x8899aabbccddeeff));
    }
}
