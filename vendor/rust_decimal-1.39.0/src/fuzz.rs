use crate::Decimal;

use arbitrary::{Arbitrary, Result as ArbitraryResult, Unstructured};

impl Arbitrary<'_> for crate::Decimal {
    fn arbitrary(u: &mut Unstructured<'_>) -> ArbitraryResult<Self> {
        let lo = u32::arbitrary(u)?;
        let mid = u32::arbitrary(u)?;
        let hi = u32::arbitrary(u)?;
        let negative = bool::arbitrary(u)?;
        let scale = u32::arbitrary(u)? % (Self::MAX_SCALE + 1);
        Ok(Decimal::from_parts(lo, mid, hi, negative, scale))
    }
}
