use super::InsertionSubrange;

/// Ordered by insertion id, then the start of the subrange.
impl Ord for InsertionSubrange {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.insertion
            .cmp(&other.insertion)
            .then_with(|| self.range.start.cmp(&other.range.start))
    }
}

impl PartialOrd for InsertionSubrange {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
