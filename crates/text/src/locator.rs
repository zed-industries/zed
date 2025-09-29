use smallvec::SmallVec;
use std::iter;

/// An identifier for a position in a ordered collection.
///
/// Allows prepending and appending without needing to renumber existing locators
/// using `Locator::between(lhs, rhs)`.
///
/// The initial location for a collection should be `Locator::between(Locator::min(), Locator::max())`,
/// leaving room for items to be inserted before and after it.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Locator(SmallVec<[u64; 4]>);

impl Locator {
    pub const fn min() -> Self {
        // SAFETY: 1 is <= 4
        Self(unsafe { SmallVec::from_const_with_len_unchecked([u64::MIN; 4], 1) })
    }

    pub const fn max() -> Self {
        // SAFETY: 1 is <= 4
        Self(unsafe { SmallVec::from_const_with_len_unchecked([u64::MAX; 4], 1) })
    }

    pub const fn min_ref() -> &'static Self {
        const { &Self::min() }
    }

    pub const fn max_ref() -> &'static Self {
        const { &Self::max() }
    }

    pub fn assign(&mut self, other: &Self) {
        self.0.resize(other.0.len(), 0);
        self.0.copy_from_slice(&other.0);
    }

    pub fn between(lhs: &Self, rhs: &Self) -> Self {
        let lhs = lhs.0.iter().copied().chain(iter::repeat(u64::MIN));
        let rhs = rhs.0.iter().copied().chain(iter::repeat(u64::MAX));
        let mut location = SmallVec::new();
        for (lhs, rhs) in lhs.zip(rhs) {
            let mid = lhs + ((rhs.saturating_sub(lhs)) >> 48);
            location.push(mid);
            if mid > lhs {
                break;
            }
        }
        Self(location)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Default for Locator {
    fn default() -> Self {
        Self::min()
    }
}

impl sum_tree::Item for Locator {
    type Summary = Locator;

    fn summary(&self, _cx: ()) -> Self::Summary {
        self.clone()
    }
}

impl sum_tree::KeyedItem for Locator {
    type Key = Locator;

    fn key(&self) -> Self::Key {
        self.clone()
    }
}

impl sum_tree::ContextLessSummary for Locator {
    fn zero() -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &Self) {
        self.assign(summary);
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
                (0..rng.random_range(1..=5))
                    .map(|_| rng.random_range(0..=100))
                    .collect(),
            );
            rhs = Locator(
                (0..rng.random_range(1..=5))
                    .map(|_| rng.random_range(0..=100))
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
