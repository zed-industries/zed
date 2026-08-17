//! Limb encoding

use super::{Limb, Word};
use crate::Encoding;

impl Encoding for Limb {
    #[cfg(target_pointer_width = "32")]
    type Repr = [u8; 4];
    #[cfg(target_pointer_width = "64")]
    type Repr = [u8; 8];

    #[inline]
    fn from_be_bytes(bytes: Self::Repr) -> Self {
        Limb(Word::from_be_bytes(bytes))
    }

    #[inline]
    fn from_le_bytes(bytes: Self::Repr) -> Self {
        Limb(Word::from_le_bytes(bytes))
    }

    #[inline]
    fn to_be_bytes(&self) -> Self::Repr {
        self.0.to_be_bytes()
    }

    #[inline]
    fn to_le_bytes(&self) -> Self::Repr {
        self.0.to_le_bytes()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;

    prop_compose! {
        fn limb()(inner in any::<Word>()) -> Limb {
            Limb(inner)
        }
    }

    proptest! {
        #[test]
        fn roundtrip(a in limb()) {
            assert_eq!(a, Limb::from_be_bytes(a.to_be_bytes()));
            assert_eq!(a, Limb::from_le_bytes(a.to_le_bytes()));
        }
    }

    proptest! {
        #[test]
        fn reverse(a in limb()) {
            let mut bytes = a.to_be_bytes();
            bytes.reverse();
            assert_eq!(a, Limb::from_le_bytes(bytes));

            let mut bytes = a.to_le_bytes();
            bytes.reverse();
            assert_eq!(a, Limb::from_be_bytes(bytes));
        }
    }
}
