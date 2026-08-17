use crate::hb::set_digest::hb_set_digest_t;

pub(crate) struct DigestU32Set(hb_set_digest_t);

impl DigestU32Set {
    pub(crate) fn default() -> Self {
        Self(hb_set_digest_t::new())
    }
    pub(crate) fn insert(&mut self, value: u32) {
        self.0.add(value);
    }
    pub(crate) fn insert_range(&mut self, range: core::ops::RangeInclusive<u32>) {
        self.0.add_range(*range.start(), *range.end());
    }
    pub(crate) fn clear(&mut self) {
        self.0.clear();
    }
    pub(crate) fn extend_unsorted<U: IntoIterator<Item = u32>>(&mut self, iter: U) {
        for value in iter {
            self.0.add(value);
        }
    }
    pub(crate) fn contains(&self, value: u32) -> bool {
        self.0.may_have(value)
    }
    pub(crate) fn intersects_set(&self, other: &Self) -> bool {
        self.0.may_intersect(&other.0)
    }
}
