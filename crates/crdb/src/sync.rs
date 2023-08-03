use crate::{
    btree::{self, Bias},
    digest::DigestSequence,
    messages::Operation,
    OperationId,
};
use bromberg_sl2::HashMatrix;
use collections::VecDeque;
use std::{
    cmp, iter,
    ops::{Range, RangeBounds},
};

impl btree::Item for Operation {
    type Summary = OperationSummary;

    fn summary(&self) -> Self::Summary {
        OperationSummary {
            max_id: self.id(),
            digest: self.id().digest(),
            count: 1,
        }
    }
}

impl btree::KeyedItem for Operation {
    type Key = OperationId;

    fn key(&self) -> Self::Key {
        self.id()
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

impl btree::Dimension<'_, OperationSummary> for OperationId {
    fn add_summary(&mut self, summary: &'_ OperationSummary, _: &()) {
        debug_assert!(*self < summary.max_id);
        *self = summary.max_id;
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
    root_range: Range<usize>,
    base: usize,
    target_depth: u32,
) -> Vec<RangeDigest> {
    struct Frame {
        digest: RangeDigest,
        depth: u32,
    }

    let mut digests = Vec::new();
    let mut cursor = operations.cursor::<usize>();
    cursor.seek(&root_range.start, Bias::Right, &());
    let root_digest = cursor.summary(&root_range.end, Bias::Right, &());
    let mut queue = VecDeque::new();
    queue.push_back(Frame {
        depth: 0,
        digest: RangeDigest {
            range: root_range,
            digest: root_digest,
        },
    });

    while let Some(Frame { depth, digest }) = queue.pop_front() {
        let range = digest.range.clone();
        if depth == target_depth || range.len() <= base {
            digests.push(digest);
            continue;
        }

        let mut start = range.start;
        let subrange_size = (range.len() + base - 1) / base;
        for _ in 0..base {
            let end = cmp::min(start + subrange_size, range.end);
            cursor.seek(&start, Bias::Right, &());
            let digest = cursor.summary(&end, Bias::Right, &());
            queue.push_back(Frame {
                depth: depth + 1,
                digest: RangeDigest {
                    range: start..end,
                    digest,
                },
            });
            start = end;
        }
    }

    digests
}

fn sync(client: &mut btree::Sequence<Operation>, server: &mut btree::Sequence<Operation>) {
    const BASE: usize = 2;
    const DEPTH: u32 = 2;
    let count = 2 * cmp::max(client.summary().count, server.summary().count);

    let mut server_digests = DigestSequence::new();
    let mut queue = Vec::new();
    for digest_range in range_digests(server, 0..count, BASE, DEPTH)
        .into_iter()
        .rev()
    {
        queue.push(digest_range.range.clone());
        let mut server_range = digest_range.range.clone();
        server_range.start = cmp::min(server_range.start, server.summary().count);
        server_range.end = cmp::min(server_range.end, server.summary().count);
        if !server_range.is_empty() {
            server_digests.insert(
                0,
                crate::digest::Digest::new(server_range.len(), digest_range.digest),
            )
        }
    }

    while let Some(mut range) = queue.pop() {
        let (left_server_digest, right_server_digest) = server_digests.digest(range.clone());
        let left_digest_range = range.start..range.start + left_server_digest.count;
        let right_digest_range =
            range.start..cmp::max(range.end, range.start + right_server_digest.count);

        let client_right_digest = digest_in_range(&client, right_digest_range.clone());
        let client_left_digest = digest_in_range(&client, left_digest_range.clone());
        if client_right_digest == right_server_digest.hash {
            continue;
        } else {
            if client_left_digest == left_server_digest.hash {
                range = left_digest_range.end..right_digest_range.end;
            } else {
                range = right_digest_range;
            }

            if range.len() > 128 {
                let mut digests = Vec::new();
                println!("roundtrip for digests");
                for range in range_digests(server, range.clone(), BASE, DEPTH)
                    .into_iter()
                    .rev()
                {
                    queue.push(range.range.clone());
                    digests.push(range);
                }
                server_digests.fill(
                    range,
                    digests.into_iter().rev().map(|digest_range| {
                        crate::digest::Digest::new(digest_range.range.len(), digest_range.digest)
                    }),
                );
                continue;
            }

            println!("roundtrip for operations {:?}", range);
            let client_operations = operations_in_range(&client, range.clone());
            let server_operations = operations_in_range(&server, range.clone())
                .cloned()
                .collect::<Vec<_>>();
            server_digests.fill(range.clone(), server_operations.iter().map(|op| op.into()));

            let mut server_ix = range.start;
            let mut client_operations = client_operations.peekable();
            let mut server_operations = server_operations.into_iter().peekable();
            let mut missed_server_ops = Vec::new();
            let mut missed_client_ops = Vec::new();
            for _ in range.clone() {
                match (client_operations.peek(), server_operations.peek()) {
                    (Some(client_operation), Some(server_operation)) => {
                        match client_operation.id().cmp(&server_operation.id()) {
                            cmp::Ordering::Less => {
                                let client_operation = client_operations.next().unwrap();
                                missed_server_ops
                                    .push(btree::Edit::Insert(client_operation.clone()));
                                server_digests.insert(server_ix, client_operation.into());
                                server_ix += 1;
                            }
                            cmp::Ordering::Equal => {
                                client_operations.next().unwrap();
                                server_operations.next().unwrap();
                                server_ix += 1;
                            }
                            cmp::Ordering::Greater => {
                                let server_operation = server_operations.next().unwrap();
                                missed_client_ops.push(btree::Edit::Insert(server_operation));
                            }
                        }
                    }
                    (None, Some(_)) => {
                        let server_operation = server_operations.next().unwrap();
                        missed_client_ops.push(btree::Edit::Insert(server_operation));
                    }
                    (Some(_), None) => {
                        let client_operation = client_operations.next().unwrap();
                        missed_server_ops.push(btree::Edit::Insert(client_operation.clone()));
                        server_digests.insert(server_ix, client_operation.into());
                        server_ix += 1;
                    }
                    (None, None) => break,
                }
            }

            drop(client_operations);
            client.edit(missed_client_ops, &());
            server.edit(missed_server_ops, &());
        }
    }
}

fn digest_in_range(operations: &btree::Sequence<Operation>, range: Range<usize>) -> HashMatrix {
    let mut cursor = operations.cursor::<usize>();
    cursor.seek(&range.start, Bias::Right, &());
    cursor.summary(&range.end, Bias::Right, &())
}

fn operations_in_range<T>(
    operations: &btree::Sequence<Operation>,
    range: T,
) -> impl Iterator<Item = &Operation>
where
    T: RangeBounds<usize>,
{
    let mut cursor = operations.cursor::<usize>();
    match range.start_bound() {
        collections::Bound::Included(start) => {
            cursor.seek(start, Bias::Right, &());
        }
        collections::Bound::Excluded(start) => {
            cursor.seek(&(*start + 1), Bias::Right, &());
        }
        collections::Bound::Unbounded => cursor.next(&()),
    }

    iter::from_fn(move || {
        if range.contains(cursor.start()) {
            let operation = cursor.item()?;
            cursor.next(&());
            Some(operation)
        } else {
            None
        }
    })
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
//

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{operations, OperationCount};

    #[test]
    fn test_sync() {
        assert_sync(1..=10, 5..=10);
        assert_sync(1..=10, 4..=10);
        assert_sync(1..=10, 1..=5);
        assert_sync(
            (1..=10).filter(|ix| ix % 2 == 0),
            (1..=10).filter(|ix| ix % 2 == 1),
        );
        assert_sync([1, 2, 3, 4, 6, 7, 8, 9, 11, 12], [4, 5, 6, 10, 12]);
        assert_sync(1..=10, 5..=14);
        assert_sync(1..=10000, 1..=7000);
    }

    fn assert_sync(
        client_ops: impl IntoIterator<Item = usize>,
        server_ops: impl IntoIterator<Item = usize>,
    ) {
        let client_ops = client_ops
            .into_iter()
            .map(build_operation)
            .collect::<Vec<_>>();
        let server_ops = server_ops
            .into_iter()
            .map(build_operation)
            .collect::<Vec<_>>();
        println!("===== syncing =====");
        println!(
            "Client: {:?}",
            client_ops.iter().map(|op| op.id()).collect::<Vec<_>>()
        );
        println!(
            "Server: {:?}",
            server_ops.iter().map(|op| op.id()).collect::<Vec<_>>()
        );
        let mut client_operations = btree::Sequence::from_iter(client_ops, &());
        let mut server_operations = btree::Sequence::from_iter(server_ops, &());
        sync(&mut client_operations, &mut server_operations);
        assert_eq!(
            client_operations
                .iter()
                .map(|op| op.id())
                .collect::<Vec<_>>(),
            server_operations
                .iter()
                .map(|op| op.id())
                .collect::<Vec<_>>()
        );
        println!("===================");
        println!();
    }

    #[test]
    fn test_range_digests() {
        let operations = btree::Sequence::from_iter((1..=64).map(build_operation), &());

        assert_eq!(ranges(&range_digests(&operations, 0..64, 2, 0,)), [0..64]);
        assert_eq!(
            ranges(&range_digests(&operations, 0..64, 2, 1)),
            [0..32, 32..64]
        );
        assert_eq!(
            ranges(&range_digests(&operations, 0..64, 2, 2)),
            [0..16, 16..32, 32..48, 48..64]
        );
        assert_eq!(
            ranges(&range_digests(&operations, 32..48, 2, 2)),
            [32..36, 36..40, 40..44, 44..48]
        );

        assert_eq!(ranges(&range_digests(&operations, 0..64, 3, 0)), [0..64]);
        assert_eq!(
            ranges(&range_digests(&operations, 0..64, 3, 1)),
            [0..22, 22..44, 44..64]
        );
        assert_eq!(
            ranges(&range_digests(&operations, 0..64, 3, 2)),
            [
                0..8,
                8..16,
                16..22,
                22..30,
                30..38,
                38..44,
                44..51,
                51..58,
                58..64
            ]
        );
    }

    fn build_operation(id: usize) -> Operation {
        Operation::CreateBranch(operations::CreateBranch {
            id: OperationId {
                replica_id: Default::default(),
                operation_count: OperationCount(id),
            },
            parent: Default::default(),
            name: "".into(),
        })
    }

    fn ranges(digests: &[RangeDigest]) -> Vec<Range<usize>> {
        digests.iter().map(|d| d.range.clone()).collect()
    }
}
