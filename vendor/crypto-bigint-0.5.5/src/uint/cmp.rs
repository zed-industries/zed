//! [`Uint`] comparisons.
//!
//! By default these are all constant-time and use the `subtle` crate.

use super::Uint;
use crate::{CtChoice, Limb};
use core::cmp::Ordering;
use subtle::{Choice, ConstantTimeEq, ConstantTimeGreater, ConstantTimeLess};

impl<const LIMBS: usize> Uint<LIMBS> {
    /// Return `b` if `c` is truthy, otherwise return `a`.
    #[inline]
    pub(crate) const fn ct_select(a: &Self, b: &Self, c: CtChoice) -> Self {
        let mut limbs = [Limb::ZERO; LIMBS];

        let mut i = 0;
        while i < LIMBS {
            limbs[i] = Limb::ct_select(a.limbs[i], b.limbs[i], c);
            i += 1;
        }

        Uint { limbs }
    }

    #[inline]
    pub(crate) const fn ct_swap(a: &Self, b: &Self, c: CtChoice) -> (Self, Self) {
        let new_a = Self::ct_select(a, b, c);
        let new_b = Self::ct_select(b, a, c);

        (new_a, new_b)
    }

    /// Returns the truthy value if `self`!=0 or the falsy value otherwise.
    #[inline]
    pub(crate) const fn ct_is_nonzero(&self) -> CtChoice {
        let mut b = 0;
        let mut i = 0;
        while i < LIMBS {
            b |= self.limbs[i].0;
            i += 1;
        }
        Limb(b).ct_is_nonzero()
    }

    /// Returns the truthy value if `self` is odd or the falsy value otherwise.
    pub(crate) const fn ct_is_odd(&self) -> CtChoice {
        CtChoice::from_lsb(self.limbs[0].0 & 1)
    }

    /// Returns the truthy value if `self == rhs` or the falsy value otherwise.
    #[inline]
    pub(crate) const fn ct_eq(lhs: &Self, rhs: &Self) -> CtChoice {
        let mut acc = 0;
        let mut i = 0;

        while i < LIMBS {
            acc |= lhs.limbs[i].0 ^ rhs.limbs[i].0;
            i += 1;
        }

        // acc == 0 if and only if self == rhs
        Limb(acc).ct_is_nonzero().not()
    }

    /// Returns the truthy value if `self <= rhs` and the falsy value otherwise.
    #[inline]
    pub(crate) const fn ct_lt(lhs: &Self, rhs: &Self) -> CtChoice {
        // We could use the same approach as in Limb::ct_lt(),
        // but since we have to use Uint::wrapping_sub(), which calls `sbb()`,
        // there are no savings compared to just calling `sbb()` directly.
        let (_res, borrow) = lhs.sbb(rhs, Limb::ZERO);
        CtChoice::from_mask(borrow.0)
    }

    /// Returns the truthy value if `self >= rhs` and the falsy value otherwise.
    #[inline]
    pub(crate) const fn ct_gt(lhs: &Self, rhs: &Self) -> CtChoice {
        let (_res, borrow) = rhs.sbb(lhs, Limb::ZERO);
        CtChoice::from_mask(borrow.0)
    }

    /// Returns the ordering between `self` and `rhs` as an i8.
    /// Values correspond to the Ordering enum:
    ///   -1 is Less
    ///   0 is Equal
    ///   1 is Greater
    #[inline]
    pub(crate) const fn ct_cmp(lhs: &Self, rhs: &Self) -> i8 {
        let mut i = 0;
        let mut borrow = Limb::ZERO;
        let mut diff = Limb::ZERO;

        while i < LIMBS {
            let (w, b) = rhs.limbs[i].sbb(lhs.limbs[i], borrow);
            diff = diff.bitor(w);
            borrow = b;
            i += 1;
        }
        let sgn = ((borrow.0 & 2) as i8) - 1;
        (diff.ct_is_nonzero().to_u8() as i8) * sgn
    }

    /// Returns the Ordering between `self` and `rhs` in variable time.
    pub const fn cmp_vartime(&self, rhs: &Self) -> Ordering {
        let mut i = LIMBS - 1;
        loop {
            let (val, borrow) = self.limbs[i].sbb(rhs.limbs[i], Limb::ZERO);
            if val.0 != 0 {
                return if borrow.0 != 0 {
                    Ordering::Less
                } else {
                    Ordering::Greater
                };
            }
            if i == 0 {
                return Ordering::Equal;
            }
            i -= 1;
        }
    }
}

impl<const LIMBS: usize> ConstantTimeEq for Uint<LIMBS> {
    #[inline]
    fn ct_eq(&self, other: &Self) -> Choice {
        Uint::ct_eq(self, other).into()
    }
}

impl<const LIMBS: usize> ConstantTimeGreater for Uint<LIMBS> {
    #[inline]
    fn ct_gt(&self, other: &Self) -> Choice {
        Uint::ct_gt(self, other).into()
    }
}

impl<const LIMBS: usize> ConstantTimeLess for Uint<LIMBS> {
    #[inline]
    fn ct_lt(&self, other: &Self) -> Choice {
        Uint::ct_lt(self, other).into()
    }
}

impl<const LIMBS: usize> Eq for Uint<LIMBS> {}

impl<const LIMBS: usize> Ord for Uint<LIMBS> {
    fn cmp(&self, other: &Self) -> Ordering {
        let c = Self::ct_cmp(self, other);
        match c {
            -1 => Ordering::Less,
            0 => Ordering::Equal,
            _ => Ordering::Greater,
        }
    }
}

impl<const LIMBS: usize> PartialOrd for Uint<LIMBS> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<const LIMBS: usize> PartialEq for Uint<LIMBS> {
    fn eq(&self, other: &Self) -> bool {
        self.ct_eq(other).into()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Integer, Zero, U128};
    use core::cmp::Ordering;
    use subtle::{ConstantTimeEq, ConstantTimeGreater, ConstantTimeLess};

    #[test]
    fn is_zero() {
        assert!(bool::from(U128::ZERO.is_zero()));
        assert!(!bool::from(U128::ONE.is_zero()));
        assert!(!bool::from(U128::MAX.is_zero()));
    }

    #[test]
    fn is_odd() {
        assert!(!bool::from(U128::ZERO.is_odd()));
        assert!(bool::from(U128::ONE.is_odd()));
        assert!(bool::from(U128::MAX.is_odd()));
    }

    #[test]
    fn ct_eq() {
        let a = U128::ZERO;
        let b = U128::MAX;

        assert!(bool::from(a.ct_eq(&a)));
        assert!(!bool::from(a.ct_eq(&b)));
        assert!(!bool::from(b.ct_eq(&a)));
        assert!(bool::from(b.ct_eq(&b)));
    }

    #[test]
    fn ct_gt() {
        let a = U128::ZERO;
        let b = U128::ONE;
        let c = U128::MAX;

        assert!(bool::from(b.ct_gt(&a)));
        assert!(bool::from(c.ct_gt(&a)));
        assert!(bool::from(c.ct_gt(&b)));

        assert!(!bool::from(a.ct_gt(&a)));
        assert!(!bool::from(b.ct_gt(&b)));
        assert!(!bool::from(c.ct_gt(&c)));

        assert!(!bool::from(a.ct_gt(&b)));
        assert!(!bool::from(a.ct_gt(&c)));
        assert!(!bool::from(b.ct_gt(&c)));
    }

    #[test]
    fn ct_lt() {
        let a = U128::ZERO;
        let b = U128::ONE;
        let c = U128::MAX;

        assert!(bool::from(a.ct_lt(&b)));
        assert!(bool::from(a.ct_lt(&c)));
        assert!(bool::from(b.ct_lt(&c)));

        assert!(!bool::from(a.ct_lt(&a)));
        assert!(!bool::from(b.ct_lt(&b)));
        assert!(!bool::from(c.ct_lt(&c)));

        assert!(!bool::from(b.ct_lt(&a)));
        assert!(!bool::from(c.ct_lt(&a)));
        assert!(!bool::from(c.ct_lt(&b)));
    }

    #[test]
    fn cmp() {
        let a = U128::ZERO;
        let b = U128::ONE;
        let c = U128::MAX;

        assert_eq!(a.cmp(&b), Ordering::Less);
        assert_eq!(a.cmp(&c), Ordering::Less);
        assert_eq!(b.cmp(&c), Ordering::Less);

        assert_eq!(a.cmp(&a), Ordering::Equal);
        assert_eq!(b.cmp(&b), Ordering::Equal);
        assert_eq!(c.cmp(&c), Ordering::Equal);

        assert_eq!(b.cmp(&a), Ordering::Greater);
        assert_eq!(c.cmp(&a), Ordering::Greater);
        assert_eq!(c.cmp(&b), Ordering::Greater);
    }

    #[test]
    fn cmp_vartime() {
        let a = U128::ZERO;
        let b = U128::ONE;
        let c = U128::MAX;

        assert_eq!(a.cmp_vartime(&b), Ordering::Less);
        assert_eq!(a.cmp_vartime(&c), Ordering::Less);
        assert_eq!(b.cmp_vartime(&c), Ordering::Less);

        assert_eq!(a.cmp_vartime(&a), Ordering::Equal);
        assert_eq!(b.cmp_vartime(&b), Ordering::Equal);
        assert_eq!(c.cmp_vartime(&c), Ordering::Equal);

        assert_eq!(b.cmp_vartime(&a), Ordering::Greater);
        assert_eq!(c.cmp_vartime(&a), Ordering::Greater);
        assert_eq!(c.cmp_vartime(&b), Ordering::Greater);
    }
}
