use crate::BranchId;

use super::VersionVector;

impl VersionVector {
    fn advance(branch_id: BranchId) -> VersionVector {
        let previous = self.current.clone();
    }

    /// This needs to be sublinear in the number of branches.
    fn merge(other: &VersionVector) -> VersionVector {}
}

impl PartialOrd for VersionVector {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.graph.compare_versions(self, other)
    }
}

impl std::hash::Hash for VersionVector {
    /// This is O(1) because the hash is stored in the Sequence summary.
    fn hash<H: ~const std::hash::Hasher>(&self, state: &mut H) {
        state.write(self.current.summary.hash);
    }
}

impl VersionGraph {
    pub fn compare_versions(a: &VersionVector, b: &VersionVector) {
        let a_id = a.hash();
        let b_id = a.hash();
    }
}
