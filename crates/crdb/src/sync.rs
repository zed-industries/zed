use crate::{
    btree::{self, Bias},
    digest::{Digest, DigestSequence},
    messages::Operation,
    OperationId,
};
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

fn request_digests(
    operations: &btree::Sequence<Operation>,
    mut root_range: Range<usize>,
    count: usize,
    min_operations: usize,
) -> Vec<Digest> {
    root_range.start = cmp::min(root_range.start, operations.summary().digest.count);
    root_range.end = cmp::min(root_range.end, operations.summary().digest.count);
    subdivide_range(root_range, count, min_operations)
        .map(|range| digest_for_range(operations, range))
        .collect()
}

fn subdivide_range(
    root_range: Range<usize>,
    count: usize,
    min_operations: usize,
) -> impl Iterator<Item = Range<usize>> {
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
    max_digests: usize,
    min_operations: usize,
) -> usize {
    let mut server_digests = DigestSequence::new();
    let mut roundtrips = 1;
    let digests = request_digests(server, 0..usize::MAX, max_digests, min_operations);
    server_digests.splice(0..0, digests.iter().cloned());
    let server_operation_count = server_digests.operation_count();
    let max_sync_range = 0..(client.summary().digest.count + server_operation_count);
    let mut stack =
        subdivide_range(max_sync_range, max_digests, min_operations).collect::<Vec<_>>();
    stack.reverse();

    let mut missed_server_ops = Vec::new();
    let mut server_end = 0;
    let mut synced_end = 0;
    while let Some(mut sync_range) = stack.pop() {
        sync_range.start = cmp::max(sync_range.start, synced_end);
        if sync_range.start >= client.summary().digest.count || server_end >= server_operation_count
        {
            // We've exhausted all operations from either the client or the server, so we
            // can fast track to publishing anything the server hasn't seen and requesting
            // anything the client hasn't seen.
            break;
        } else if sync_range.end < synced_end {
            // This range has already been synced, so we can skip it.
            continue;
        }

        let server_digest = server_digests.digest(sync_range.clone());
        sync_range.end = cmp::max(sync_range.start + server_digest.count, sync_range.end);
        let server_range = server_end..server_end + sync_range.len();

        let client_digest = digest_for_range(client, sync_range.clone());
        if client_digest == server_digest {
            log::debug!("skipping {:?}", sync_range);
            synced_end = sync_range.end;
            server_end += server_digest.count;
        } else if sync_range.len() > min_operations {
            log::debug!("descending into {:?}", sync_range);
            roundtrips += 1;
            let digests =
                request_digests(server, server_range.clone(), max_digests, min_operations);
            server_digests.splice(sync_range.clone(), digests.iter().cloned());
            let old_stack_len = stack.len();

            stack.extend(subdivide_range(sync_range, max_digests, min_operations));
            stack[old_stack_len..].reverse();
        } else {
            log::debug!("exchanging operations for {:?}", sync_range);
            roundtrips += 1;
            let server_operations = request_operations(server, server_range.clone());
            debug_assert!(server_operations.len() > 0);
            server_digests.splice(
                sync_range.clone(),
                server_operations.iter().map(|op| op.into()),
            );

            let mut missed_client_ops = Vec::new();
            let mut server_operations = server_operations.into_iter().peekable();
            let mut client_operations = operations_for_range(client, sync_range.clone()).peekable();
            for _ in sync_range {
                match (client_operations.peek(), server_operations.peek()) {
                    (Some(client_operation), Some(server_operation)) => {
                        match client_operation.id().cmp(&server_operation.id()) {
                            Ordering::Less => {
                                let client_operation = client_operations.next().unwrap();
                                missed_server_ops
                                    .push(btree::Edit::Insert(client_operation.clone()));
                                server_digests
                                    .splice(synced_end..synced_end, [client_operation.into()]);
                            }
                            Ordering::Equal => {
                                client_operations.next().unwrap();
                                server_operations.next().unwrap();
                                server_end += 1;
                            }
                            Ordering::Greater => {
                                let server_operation = server_operations.next().unwrap();
                                missed_client_ops.push(btree::Edit::Insert(server_operation));
                                server_end += 1;
                            }
                        }
                    }
                    (None, Some(_)) => {
                        let server_operation = server_operations.next().unwrap();
                        missed_client_ops.push(btree::Edit::Insert(server_operation));
                        server_end += 1;
                    }
                    (Some(_), None) => {
                        let client_operation = client_operations.next().unwrap();
                        missed_server_ops.push(btree::Edit::Insert(client_operation.clone()));
                        server_digests.splice(synced_end..synced_end, [client_operation.into()]);
                    }
                    (None, None) => break,
                }

                synced_end += 1;
            }

            drop(client_operations);
            client.edit(missed_client_ops, &());
        }
    }

    // Fetch and publish the remaining suffixes.
    if synced_end < client.summary().digest.count || server_end < server_operation_count {
        log::debug!("exchanging operations for {:?}..", synced_end);
        roundtrips += 1;

        let remaining_client_ops = operations_for_range(client, synced_end..);
        missed_server_ops.extend(remaining_client_ops.cloned().map(btree::Edit::Insert));

        let remaining_server_ops = request_operations(server, server_end..);
        client.edit(
            remaining_server_ops
                .into_iter()
                .map(btree::Edit::Insert)
                .collect(),
            &(),
        );
    }

    server.edit(missed_server_ops, &());
    roundtrips
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
    use rand::prelude::*;
    use std::env;

    #[test]
    fn test_sync() {
        assert_sync(1..=10, 5..=10);
        assert_sync(1..=10, 4..=10);
        assert_sync(1..=10, 1..=5);
        assert_sync([1, 3, 5, 7, 9], [2, 4, 6, 8, 10]);
        assert_sync([1, 2, 3, 4, 6, 7, 8, 9, 11, 12], [4, 5, 6, 10, 12]);
        assert_sync(1..=10, 5..=14);
        assert_sync(1..=80, (1..=70).chain(90..=100));
        assert_sync(1..=1910, (1..=1900).chain(1910..=2000));
    }

    #[gpui::test(iterations = 100)]
    fn test_random(mut rng: StdRng) {
        let max_operations = env::var("OPERATIONS")
            .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
            .unwrap_or(10);

        let mut connected = true;
        let mut connection_ixs = (1..=max_operations).choose_multiple(&mut rng, max_operations / 4);
        connection_ixs.reverse();
        let mut client_ops = Vec::new();
        let mut server_ops = Vec::new();
        for ix in 1..=max_operations {
            if connection_ixs.last() == Some(&ix) {
                connected = !connected;
                connection_ixs.pop();
            }

            if connected {
                client_ops.push(ix);
                server_ops.push(ix);
            } else if rng.gen() {
                server_ops.push(ix);
            } else {
                client_ops.push(ix);
            }
        }

        // println!("client operations: {:?}", client_ops);
        // println!("server operations: {:?}", server_ops);
        assert_sync_with_config(
            client_ops,
            server_ops,
            [1, 2, 4, 8, 16, 32, 64, 128, 256, 512]
                .choose(&mut rng)
                .unwrap()
                .clone(),
            [1, 2, 4, 8, 16, 32, 64, 128, 256, 512]
                .choose(&mut rng)
                .unwrap()
                .clone(),
        );
    }

    fn assert_sync(
        client_ops: impl IntoIterator<Item = usize>,
        server_ops: impl IntoIterator<Item = usize>,
    ) {
        let client_ops = client_ops.into_iter().collect::<Vec<_>>();
        let server_ops = server_ops.into_iter().collect::<Vec<_>>();
        for max_digests in [1, 2, 4, 8, 16, 32, 64, 128, 256] {
            for min_operations in [1, 2, 4, 8] {
                assert_sync_with_config(
                    client_ops.clone(),
                    server_ops.clone(),
                    max_digests,
                    min_operations,
                );
            }
        }
    }

    fn assert_sync_with_config(
        client_ops: impl IntoIterator<Item = usize>,
        server_ops: impl IntoIterator<Item = usize>,
        max_digests: usize,
        min_operations: usize,
    ) {
        println!(
            "max digests: {}, min_operations: {}",
            max_digests, min_operations
        );
        let client_ops = client_ops
            .into_iter()
            .map(build_operation)
            .collect::<Vec<_>>();
        let server_ops = server_ops
            .into_iter()
            .map(build_operation)
            .collect::<Vec<_>>();
        let mut client_operations = btree::Sequence::from_iter(client_ops, &());
        let mut server_operations = btree::Sequence::from_iter(server_ops, &());
        let roundtrips = sync(
            &mut client_operations,
            &mut server_operations,
            max_digests,
            min_operations,
        );
        dbg!(roundtrips);

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
