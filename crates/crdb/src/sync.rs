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
    root_range: Range<usize>,
    base: usize,
    target_depth: u32,
) -> Vec<Digest> {
    let digest_count = base.pow(target_depth);
    let subrange_len = (root_range.len() + digest_count - 1) / digest_count;

    let mut digests = Vec::with_capacity(digest_count);
    let mut subrange_start = root_range.start;
    while subrange_start < root_range.end {
        let subrange_end = cmp::min(subrange_start + subrange_len, root_range.end);
        digests.push(digest_for_range(operations, subrange_start..subrange_end));
        subrange_start = subrange_end;
    }
    digests
}

fn leaf_ranges(root_range: Range<usize>, tree_base: usize, tree_depth: u32) -> Vec<Range<usize>> {
    let count = tree_base.pow(tree_depth);
    let subrange_len = (root_range.len() + count - 1) / count;

    let mut subranges = Vec::with_capacity(count);
    let mut subrange_start = root_range.start;
    while subrange_start < root_range.end {
        let subrange_end = cmp::min(subrange_start + subrange_len, root_range.end);
        subranges.push(subrange_start..subrange_end);
        subrange_start = subrange_end;
    }
    subranges
}

fn sync(client: &mut btree::Sequence<Operation>, server: &mut btree::Sequence<Operation>) {
    const BASE: usize = 2;
    const DEPTH: u32 = 2;
    const MIN_OPERATIONS: usize = 5;
    let max_sync_range = 0..(client.summary().digest.count + server.summary().digest.count);
    let mut server_digests = DigestSequence::new();
    let digests = request_digests(server, max_sync_range.clone(), BASE, DEPTH);
    server_digests.splice(0..0, digests.iter().cloned());
    let mut stack = leaf_ranges(max_sync_range, BASE, DEPTH);
    stack.reverse();

    let mut synced_end = 0;
    while let Some(mut sync_range) = stack.pop() {
        sync_range.start = cmp::max(sync_range.start, synced_end);
        println!("visiting {:?}", sync_range);
        server_digests.debug();
        let server_digest = server_digests.digest(sync_range.clone());
        sync_range.end = cmp::max(sync_range.start + server_digest.count, sync_range.end);
        println!("server digest range was {:?}", sync_range);
        let client_digest = digest_for_range(client, sync_range.clone());
        if client_digest == server_digest {
            println!("digests are the same in {:?}", sync_range);
            synced_end = sync_range.end;
            continue;
        } else if sync_range.len() > MIN_OPERATIONS {
            println!("digests are not the same, recursing");
            let digests = request_digests(server, sync_range.clone(), BASE, DEPTH);
            server_digests.splice(sync_range.clone(), digests.iter().cloned());
            let old_stack_len = stack.len();
            stack.extend(leaf_ranges(sync_range, BASE, DEPTH));
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
                                println!("server missed {:?}", client_operation.id());
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
                                println!("client missed {:?}", server_operation.id());
                                missed_client_ops.push(btree::Edit::Insert(server_operation));
                            }
                        }
                    }
                    (None, Some(_)) => {
                        let server_operation = server_operations.next().unwrap();
                        println!("client missed {:?}", server_operation.id());
                        missed_client_ops.push(btree::Edit::Insert(server_operation));
                    }
                    (Some(_), None) => {
                        let client_operation = client_operations.next().unwrap();
                        println!("server missed {:?}", client_operation.id());
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

// fn sync(client: &mut btree::Sequence<Operation>, server: &mut btree::Sequence<Operation>) {
//     const BASE: usize = 2;
//     const DEPTH: u32 = 3;
//     let count = 2 * cmp::max(client.summary().count, server.summary().count);

//     let mut server_digests = DigestSequence::new();
//     let mut queue = Vec::new();
//     queue.push(0..count);

//     while let Some(mut range) = queue.pop() {
//         server_digests.debug();
//         let server_digest = server_digests.digest(range.clone());
//         let server_digest_range = range.start..range.start + server_digest.count;
//         let client_digest = digest_in_range(&client, left_digest_range.clone());
//         if client_right_digest == right_server_digest.hash {
//             continue;
//         } else {
//             if client_left_digest == left_server_digest.hash {
//                 range = left_digest_range.end..right_digest_range.end;
//             } else {
//                 range = right_digest_range;
//             }

//             if range.len() > BASE.pow(DEPTH) {
//                 let mut digests = Vec::new();
//                 println!("roundtrip for digests in range {:?}", range);
//                 for range in request_digests(server, range.clone(), BASE, DEPTH)
//                     .into_iter()
//                     .rev()
//                 {
//                     queue.push(range.range.clone());
//                     digests.push(range);
//                 }

//                 println!("before");
//                 server_digests.debug();
//                 server_digests.splice(
//                     range,
//                     digests.into_iter().rev().filter_map(|digest_range| {
//                         let start = cmp::min(digest_range.range.start, server.summary().count);
//                         let end = cmp::min(digest_range.range.end, server.summary().count);
//                         if start < end {
//                             Some(crate::digest::Digest::new(end - start, digest_range.digest))
//                         } else {
//                             None
//                         }
//                     }),
//                 );
//                 println!("after");
//                 server_digests.debug();
//                 continue;
//             }
//             dbg!(&range);

//             println!("roundtrip for operations {:?}", range);
//             let client_operations = operations_in_range(&client, range.clone());
//             let server_operations = operations_in_range(&server, range.clone())
//                 .cloned()
//                 .collect::<Vec<_>>();
//             server_digests.splice(range.clone(), server_operations.iter().map(|op| op.into()));

//             let mut server_ix = range.start;
//             let mut client_operations = client_operations.peekable();
//             let mut server_operations = server_operations.into_iter().peekable();
//             let mut missed_server_ops = Vec::new();
//             let mut missed_client_ops = Vec::new();
//             for _ in range.clone() {
//                 match (client_operations.peek(), server_operations.peek()) {
//                     (Some(client_operation), Some(server_operation)) => {
//                         match client_operation.id().cmp(&server_operation.id()) {
//                             cmp::Ordering::Less => {
//                                 let client_operation = client_operations.next().unwrap();
//                                 println!("server missed {:?}", client_operation.id());
//                                 missed_server_ops
//                                     .push(btree::Edit::Insert(client_operation.clone()));
//                                 server_digests
//                                     .splice(server_ix..server_ix, [client_operation.into()]);
//                                 server_ix += 1;
//                             }
//                             cmp::Ordering::Equal => {
//                                 client_operations.next().unwrap();
//                                 server_operations.next().unwrap();
//                                 server_ix += 1;
//                             }
//                             cmp::Ordering::Greater => {
//                                 let server_operation = server_operations.next().unwrap();
//                                 println!("client missed {:?}", server_operation.id());
//                                 missed_client_ops.push(btree::Edit::Insert(server_operation));
//                             }
//                         }
//                     }
//                     (None, Some(_)) => {
//                         let server_operation = server_operations.next().unwrap();
//                         println!("client missed {:?}", server_operation.id());
//                         missed_client_ops.push(btree::Edit::Insert(server_operation));
//                     }
//                     (Some(_), None) => {
//                         let client_operation = client_operations.next().unwrap();
//                         println!("server missed {:?}", client_operation.id());
//                         missed_server_ops.push(btree::Edit::Insert(client_operation.clone()));
//                         server_digests.splice(server_ix..server_ix, [client_operation.into()]);
//                         server_ix += 1;
//                     }
//                     (None, None) => break,
//                 }
//             }

//             drop(client_operations);
//             client.edit(missed_client_ops, &());
//             server.edit(missed_server_ops, &());
//         }
//     }
// }

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
        assert_sync([1, 3, 5, 7, 9], [2, 4, 6, 8, 10]);
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
    fn test_request_digests() {
        let operations = btree::Sequence::from_iter((1..=64).map(build_operation), &());

        assert_eq!(
            digest_counts(&request_digests(&operations, 0..64, 2, 0,)),
            [64]
        );
        assert_eq!(
            digest_counts(&request_digests(&operations, 0..64, 2, 1)),
            [32, 32]
        );
        assert_eq!(
            digest_counts(&request_digests(&operations, 0..64, 2, 2)),
            [16, 16, 16, 16]
        );
        assert_eq!(
            digest_counts(&request_digests(&operations, 32..48, 2, 2)),
            [4, 4, 4, 4]
        );

        assert_eq!(
            digest_counts(&request_digests(&operations, 0..64, 3, 0)),
            [64]
        );
        assert_eq!(
            digest_counts(&request_digests(&operations, 0..64, 3, 1)),
            [22, 22, 22]
        );
        assert_eq!(
            digest_counts(&request_digests(&operations, 0..64, 3, 2)),
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
