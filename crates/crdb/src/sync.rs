use crate::{
    btree::{self, Bias},
    digest::{Digest, DigestSequence},
    messages::Operation,
    OperationId,
};
use bromberg_sl2::HashMatrix;
use std::{
    cmp::{self, Ordering},
    iter,
    ops::{Range, RangeBounds},
};

impl btree::Item for Operation {
    type Summary = OperationSummary;

    fn summary(&self) -> Self::Summary {
        OperationSummary {
            max_id: self.id(),
            digest: Digest::from(self),
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
    digest: Digest,
}

impl btree::Summary for OperationSummary {
    type Context = ();

    fn add_summary(&mut self, summary: &Self, _: &()) {
        debug_assert!(self.max_id < summary.max_id);
        self.max_id = summary.max_id;
        Digest::add_summary(&mut self.digest, &summary.digest, &());
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
        *self += summary.digest.count;
    }
}

impl btree::Dimension<'_, OperationSummary> for Digest {
    fn add_summary(&mut self, summary: &'_ OperationSummary, _: &()) {
        Digest::add_summary(self, &summary.digest, &());
    }
}

struct RangeDigest {
    range: Range<usize>,
    digest: HashMatrix,
}

fn request_digests(
    operations: &btree::Sequence<Operation>,
    mut root_range: Range<usize>,
    tree_base: usize,
    tree_depth: u32,
    min_operations: usize,
) -> Vec<Digest> {
    root_range.start = cmp::min(root_range.start, operations.summary().digest.count);
    root_range.end = cmp::min(root_range.end, operations.summary().digest.count);
    subdivide_range(root_range, tree_base, tree_depth, min_operations)
        .map(|range| digest_for_range(operations, range))
        .collect()
}

fn subdivide_range(
    root_range: Range<usize>,
    tree_base: usize,
    tree_depth: u32,
    min_operations: usize,
) -> impl Iterator<Item = Range<usize>> {
    let count = tree_base.pow(tree_depth);
    let subrange_len = cmp::max(min_operations, (root_range.len() + count - 1) / count);

    let mut subrange_start = root_range.start;
    iter::from_fn(move || {
        if subrange_start >= root_range.end {
            return None;
        }
        let subrange = subrange_start..cmp::min(subrange_start + subrange_len, root_range.end);
        subrange_start = subrange.end;
        Some(subrange)
    })
}

fn sync(
    client: &mut btree::Sequence<Operation>,
    server: &mut btree::Sequence<Operation>,
    base: usize,
    depth: u32,
    min_operations: usize,
) {
    let mut server_digests = DigestSequence::new();
    let digests = request_digests(server, 0..usize::MAX, base, depth, min_operations);
    server_digests.splice(0..0, digests.iter().cloned());
    let max_sync_range = 0..(client.summary().digest.count + server_digests.operation_count());
    let mut stack =
        subdivide_range(max_sync_range, base, depth, min_operations).collect::<Vec<_>>();
    stack.reverse();

    let mut synced_end = 0;
    while let Some(mut sync_range) = stack.pop() {
        sync_range.start = cmp::max(sync_range.start, synced_end);
        let server_digest = server_digests.digest(sync_range.clone());
        sync_range.end = cmp::max(sync_range.start + server_digest.count, sync_range.end);
        let client_digest = digest_for_range(client, sync_range.clone());
        if client_digest == server_digest {
            if client_digest.count == 0 {
                break;
            }

            synced_end = sync_range.end;
        } else if sync_range.len() > min_operations {
            let digests = request_digests(server, sync_range.clone(), base, depth, min_operations);
            server_digests.splice(sync_range.clone(), digests.iter().cloned());
            let old_stack_len = stack.len();
            stack.extend(subdivide_range(sync_range, base, depth, min_operations));
            stack[old_stack_len..].reverse();
        } else {
            let mut missed_client_ops = Vec::new();
            let mut missed_server_ops = Vec::new();

            let server_operations = request_operations(server, sync_range.clone());
            server_digests.splice(
                sync_range.clone(),
                server_operations.iter().map(|op| op.into()),
            );

            let mut server_operations = server_operations.into_iter().peekable();
            let mut client_operations = operations_for_range(client, sync_range.clone()).peekable();
            let mut server_ix = sync_range.start;
            for _ in sync_range.clone() {
                match (client_operations.peek(), server_operations.peek()) {
                    (Some(client_operation), Some(server_operation)) => {
                        match client_operation.id().cmp(&server_operation.id()) {
                            Ordering::Less => {
                                let client_operation = client_operations.next().unwrap();
                                missed_server_ops
                                    .push(btree::Edit::Insert(client_operation.clone()));
                                server_digests
                                    .splice(server_ix..server_ix, [client_operation.into()]);
                                server_ix += 1;
                            }
                            Ordering::Equal => {
                                client_operations.next().unwrap();
                                server_operations.next().unwrap();
                                server_ix += 1;
                            }
                            Ordering::Greater => {
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
                        server_digests.splice(server_ix..server_ix, [client_operation.into()]);
                        server_ix += 1;
                    }
                    (None, None) => break,
                }
            }

            drop(client_operations);
            client.edit(missed_client_ops, &());

            // Publish these over the network in real implementation.
            server.edit(missed_server_ops, &());

            synced_end = sync_range.end;
        }
    }
}

fn digest_for_range(operations: &btree::Sequence<Operation>, range: Range<usize>) -> Digest {
    let mut cursor = operations.cursor::<usize>();
    cursor.seek(&range.start, Bias::Right, &());
    cursor.summary(&range.end, Bias::Right, &())
}

fn request_operations<T: RangeBounds<usize>>(
    operations: &btree::Sequence<Operation>,
    range: T,
) -> Vec<Operation> {
    operations_for_range(operations, range).cloned().collect()
}

fn operations_for_range<T: RangeBounds<usize>>(
    operations: &btree::Sequence<Operation>,
    range: T,
) -> impl Iterator<Item = &Operation> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{operations, OperationCount};

    #[test]
    fn test_sync() {
        assert_sync(1..=10, 5..=10);
        assert_sync(1..=10, 4..=10);
        assert_sync(1..=10, 1..=5);
        assert_sync([1, 3, 5, 7, 9], [2, 4, 6, 8, 10]);
        assert_sync([1, 2, 3, 4, 6, 7, 8, 9, 11, 12], [4, 5, 6, 10, 12]);
        assert_sync(1..=10, 5..=14);
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
        for base in 2..16 {
            for depth in 1..=4 {
                for min_operations in 1..16 {
                    println!(
                        "base: {}, depth: {}, min_operations: {}",
                        base, depth, min_operations
                    );
                    let mut client_operations = btree::Sequence::from_iter(client_ops.clone(), &());
                    let mut server_operations = btree::Sequence::from_iter(server_ops.clone(), &());
                    sync(
                        &mut client_operations,
                        &mut server_operations,
                        base,
                        depth,
                        min_operations,
                    );
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
                }
            }
        }
    }

    #[test]
    fn test_request_digests() {
        let operations = btree::Sequence::from_iter((1..=64).map(build_operation), &());

        assert_eq!(
            digest_counts(&request_digests(&operations, 0..64, 2, 0, 0)),
            [64]
        );
        assert_eq!(
            digest_counts(&request_digests(&operations, 0..64, 2, 1, 0)),
            [32, 32]
        );
        assert_eq!(
            digest_counts(&request_digests(&operations, 0..64, 2, 2, 0)),
            [16, 16, 16, 16]
        );
        assert_eq!(
            digest_counts(&request_digests(&operations, 32..48, 2, 2, 0)),
            [4, 4, 4, 4]
        );

        assert_eq!(
            digest_counts(&request_digests(&operations, 0..64, 3, 0, 0)),
            [64]
        );
        assert_eq!(
            digest_counts(&request_digests(&operations, 0..64, 3, 1, 0)),
            [22, 22, 22]
        );
        assert_eq!(
            digest_counts(&request_digests(&operations, 0..64, 3, 2, 0)),
            [8, 8, 8, 8, 8, 8, 8, 8]
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

    fn digest_counts(digests: &[Digest]) -> Vec<usize> {
        digests.iter().map(|d| d.count).collect()
    }
}
