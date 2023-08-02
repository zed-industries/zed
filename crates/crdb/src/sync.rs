use crate::{
    btree::{self, Bias},
    messages::Operation,
    OperationId,
};
use bromberg_sl2::HashMatrix;
use collections::VecDeque;
use std::ops::Range;

impl btree::Item for Operation {
    type Summary = OperationSummary;

    fn summary(&self) -> Self::Summary {
        OperationSummary {
            max_id: self.id(),
            digest: bromberg_sl2::hash_strict(&self.id().to_be_bytes()),
            count: 1,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct OperationSummary {
    max_id: OperationId,
    digest: bromberg_sl2::HashMatrix,
    count: usize,
}

impl btree::Summary for OperationSummary {
    type Context = ();

    fn add_summary(&mut self, summary: &Self, _: &()) {
        debug_assert!(self.max_id < summary.max_id);
        self.max_id = summary.max_id;
        self.digest = self.digest * summary.digest;
        self.count += summary.count;
    }
}

impl btree::Dimension<'_, OperationSummary> for usize {
    fn add_summary(&mut self, summary: &'_ OperationSummary, _: &()) {
        *self += summary.count;
    }
}

impl btree::Dimension<'_, OperationSummary> for HashMatrix {
    fn add_summary(&mut self, summary: &'_ OperationSummary, _: &()) {
        *self = *self * summary.digest;
    }
}

struct RangeDigest {
    range: Range<usize>,
    digest: HashMatrix,
}

fn range_digests(
    operations: &btree::Sequence<Operation>,
    base: usize,
    max_depth: u32,
    min_range_size: usize,
) -> Vec<RangeDigest> {
    let max_digests = base.pow(max_depth);
    let mut digests = Vec::with_capacity(max_digests);
    let mut cursor = operations.cursor::<usize>();
    let mut queue = VecDeque::new();
    queue.push_back(RangeDigest {
        range: 0..operations.summary().count,
        digest: operations.summary().digest,
    });

    while let Some(next) = queue.pop_front() {
        let range = next.range.clone();
        if range.len() < min_range_size {
            break;
        }

        digests.push(next);
        if digests.len() >= max_digests {
            continue;
        }

        let mut start = range.start;
        let subrange_size = range.len() / base;
        for _ in 0..base {
            let end = start + subrange_size;
            cursor.seek(&start, Bias::Right, &());
            let digest = cursor.summary(&end, Bias::Right, &());
            queue.push_back(RangeDigest {
                range: start..end,
                digest,
            });
            start = end;
        }
    }

    digests
}

/// In memory only exploration
// fn sync(client: btree::Sequence<Operation>, server: btree::Sequence<Operation>) {

//     let mut depth = 0;
//     let mut digests = Vec::new();
//     let mut range = 0..client.summary().count;

//     while depth <= 3 {
//         let mut cursor = client.cursor::<usize>();
//         cursor.seek(&range.start, Bias::Right, &());
//         let digest = cursor.summary(&range.end, Bias::Right, &());
//         digests.push(RangeDigest { range, digest })
//         depth += 1;
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{operations, OperationCount};

    #[test]
    fn test_sync() {
        let operations = btree::Sequence::from_iter(
            (1..=64).map(|ix| {
                build_operation(OperationId {
                    replica_id: Default::default(),
                    operation_count: OperationCount(ix),
                })
            }),
            &(),
        );

        assert_eq!(
            ranges(&range_digests(&operations, 2, 8, 16)),
            [0..64, 0..32, 32..64, 0..16, 16..32, 32..48, 48..64]
        );
    }

    fn build_operation(id: OperationId) -> Operation {
        Operation::CreateBranch(operations::CreateBranch {
            id,
            parent: Default::default(),
            name: "".into(),
        })
    }

    fn ranges(digests: &[RangeDigest]) -> Vec<Range<usize>> {
        digests.iter().map(|d| d.range.clone()).collect()
    }
}
