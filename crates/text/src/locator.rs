use smallvec::{smallvec, SmallVec};
use std::iter;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Locator(SmallVec<[u32; 4]>);

impl Locator {
    pub fn min() -> Self {
        Self(smallvec![u32::MIN])
    }

    pub fn max() -> Self {
        Self(smallvec![u32::MAX])
    }

    pub fn between(lhs: &Self, rhs: &Self) -> Self {
        let lhs = lhs.0.iter().copied().chain(iter::repeat(u32::MIN));
        let rhs = rhs.0.iter().copied().chain(iter::repeat(u32::MAX));
        let mut location = SmallVec::new();
        for (lhs, rhs) in lhs.zip(rhs) {
            let mid = lhs + (rhs.saturating_sub(lhs)) / 2;
            location.push(mid);
            if mid > lhs {
                break;
            }
        }
        Self(location)
    }
}

impl Default for Locator {
    fn default() -> Self {
        Self::min()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;
    use std::mem;

    #[gpui::test(iterations = 100)]
    fn test_locators(mut rng: StdRng) {
        let mut lhs = Default::default();
        let mut rhs = Default::default();
        while lhs == rhs {
            lhs = Locator(
                (0..rng.gen_range(1..=5))
                    .map(|_| rng.gen_range(0..=100))
                    .collect(),
            );
            rhs = Locator(
                (0..rng.gen_range(1..=5))
                    .map(|_| rng.gen_range(0..=100))
                    .collect(),
            );
        }

        if lhs > rhs {
            mem::swap(&mut lhs, &mut rhs);
        }

        let middle = Locator::between(&lhs, &rhs);
        assert!(middle > lhs);
        assert!(middle < rhs);
        for ix in 0..middle.0.len() - 1 {
            assert!(
                middle.0[ix] == *lhs.0.get(ix).unwrap_or(&0)
                    || middle.0[ix] == *rhs.0.get(ix).unwrap_or(&0)
            );
        }
    }
}
