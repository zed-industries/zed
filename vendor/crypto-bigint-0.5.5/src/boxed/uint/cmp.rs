//! [`BoxedUint`] comparisons.
//!
//! By default these are all constant-time and use the `subtle` crate.

use super::BoxedUint;
use crate::Limb;
use subtle::{Choice, ConstantTimeEq};

impl ConstantTimeEq for BoxedUint {
    #[inline]
    fn ct_eq(&self, other: &Self) -> Choice {
        let (shorter, longer) = Self::sort_by_precision(self, other);
        let mut ret = Choice::from(1u8);

        for i in 0..longer.limbs.len() {
            let a = shorter.limbs.get(i).unwrap_or(&Limb::ZERO);
            let b = longer.limbs.get(i).unwrap_or(&Limb::ZERO);
            ret &= a.ct_eq(b);
        }

        ret
    }
}

impl Eq for BoxedUint {}
impl PartialEq for BoxedUint {
    fn eq(&self, other: &Self) -> bool {
        self.ct_eq(other).into()
    }
}

#[cfg(test)]
mod tests {
    use super::BoxedUint;
    use subtle::ConstantTimeEq;

    #[test]
    fn ct_eq() {
        let a = BoxedUint::zero();
        let b = BoxedUint::one();

        assert!(bool::from(a.ct_eq(&a)));
        assert!(!bool::from(a.ct_eq(&b)));
        assert!(!bool::from(b.ct_eq(&a)));
        assert!(bool::from(b.ct_eq(&b)));
    }
}
