use std::{cmp, ops::Range};

use crate::{
    btree::{self, Bias},
    messages::Operation,
};
use bromberg_sl2::HashMatrix;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Digest {
    pub count: usize,
    pub hash: HashMatrix,
}

impl std::fmt::Debug for Digest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Digest")
            .field("count", &self.count)
            .field("hash", &self.hash.to_hex())
            .finish()
    }
}

impl Digest {
    pub fn new(count: usize, hash: HashMatrix) -> Self {
        assert!(count > 0);
        Self { count, hash }
    }
}

impl From<&'_ Operation> for Digest {
    fn from(op: &'_ Operation) -> Self {
        Self::new(1, op.id().digest())
    }
}

impl btree::Item for Digest {
    type Summary = Digest;

    fn summary(&self) -> Self::Summary {
        self.clone()
    }
}

impl btree::Summary for Digest {
    type Context = ();

    fn add_summary(&mut self, summary: &Self, _: &()) {
        self.count += summary.count;
        self.hash = self.hash * summary.hash;
    }
}

impl btree::Dimension<'_, Digest> for usize {
    fn add_summary(&mut self, summary: &'_ Digest, _: &()) {
        *self += summary.count;
    }
}

impl btree::Dimension<'_, Digest> for HashMatrix {
    fn add_summary(&mut self, summary: &'_ Digest, _: &()) {
        *self = *self * summary.hash;
    }
}

pub struct DigestSequence {
    digests: btree::Sequence<Digest>,
}

impl DigestSequence {
    pub fn new() -> Self {
        Self {
            digests: Default::default(),
        }
    }

    pub fn items(&self) -> Vec<Digest> {
        self.digests.items(&())
    }

    pub fn operation_count(&self) -> usize {
        self.digests.summary().count
    }

    pub fn digest(&self, mut range: Range<usize>) -> Digest {
        range.start = cmp::min(range.start, self.digests.summary().count);
        range.end = cmp::min(range.end, self.digests.summary().count);
        let mut cursor = self.digests.cursor::<usize>();
        cursor.seek(&range.start, Bias::Right, &());
        assert_eq!(
            *cursor.start(),
            range.start,
            "start is not at the start of a digest range"
        );
        let mut hash: HashMatrix = cursor.summary(&range.end, Bias::Right, &());
        if range.end > *cursor.start() {
            let digest = cursor.item().unwrap();
            hash = hash * digest.hash;
            cursor.next(&());
        }

        Digest {
            count: cursor.start() - range.start,
            hash,
        }
    }

    pub fn splice(&mut self, mut range: Range<usize>, digests: impl IntoIterator<Item = Digest>) {
        let max_index = self.digests.summary().count;
        if range.start > max_index {
            panic!("range out of bounds");
        }
        range.end = cmp::min(range.end, max_index);

        let mut cursor = self.digests.cursor::<usize>();
        let mut new_digests = cursor.slice(&range.start, Bias::Right, &());
        assert_eq!(*cursor.start(), range.start, "start is nedigest range");
        cursor.seek(&range.end, Bias::Right, &());
        assert_eq!(
            *cursor.start(),
            range.end,
            "end is not at the start of a digest range"
        );
        new_digests.extend(digests, &());
        new_digests.append(cursor.suffix(&()), &());
        drop(cursor);
        self.digests = new_digests;
    }
}
