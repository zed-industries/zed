use crate::{
    btree::{self, Bias},
    messages::{Operation, PublishOperations},
    OperationId,
};
use bromberg_sl2::HashMatrix;
use std::{
    cmp::Ordering,
    iter,
    ops::{Range, RangeBounds},
};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Digest {
    count: usize,
    hash: HashMatrix,
}

impl btree::Item for Operation {
    type Summary = OperationSummary;

    fn summary(&self) -> Self::Summary {
        OperationSummary {
            max_id: self.id(),
            digest: Digest {
                count: 1,
                hash: bromberg_sl2::hash_strict(&self.id().to_be_bytes()),
            },
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
        self.digest.count += summary.digest.count;
        self.digest.hash = self.digest.hash * summary.digest.hash;
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
        self.count += summary.digest.count;
        self.hash = self.hash * summary.digest.hash;
    }
}

struct SyncRequest {
    digests: Vec<Digest>,
}

struct SyncResponse {
    shared_prefix_end: usize,
    operations: Vec<Operation>,
}

struct SyncStats {
    server_operations: usize,
    client_operations: usize,
}

fn sync_server(
    operations: &mut btree::Sequence<Operation>,
    sync_request: SyncRequest,
) -> SyncResponse {
    for client_digest in sync_request.digests {
        let server_digest = digest_for_range(operations, 0..client_digest.count);
        if server_digest == client_digest {
            return SyncResponse {
                shared_prefix_end: server_digest.count,
                operations: operations_for_range(operations, server_digest.count..)
                    .cloned()
                    .collect(),
            };
        }
    }

    SyncResponse {
        shared_prefix_end: 0,
        operations: operations.iter().cloned().collect(),
    }
}

fn publish_operations(
    server_operations: &mut btree::Sequence<Operation>,
    request: PublishOperations,
) {
    server_operations.edit(
        request
            .operations
            .into_iter()
            .map(btree::Edit::Insert)
            .collect(),
        &(),
    );
}

fn sync_client(
    client_operations: &mut btree::Sequence<Operation>,
    server_operations: &mut btree::Sequence<Operation>,
    min_shared_prefix_end: usize,
    max_digest_count: usize,
) -> SyncStats {
    let mut digests = Vec::new();
    let mut digest_end_ix = client_operations.summary().digest.count;
    // We will multiply by some some factor less than 1 to produce digests
    // over ever smaller digest ranges. The following formula ensures that
    // we will produce `max_digest_count` digests, and that the last digest
    // will go from `0` to `min_shared_prefix_end`.
    // op_count * factor^max_digest_count = min_shared_prefix_end
    // factor^max_digest_count = min_shared_prefix_end/op_count
    // max_digest_count * log_2(factor) = log_2(min_shared_prefix_end/op_count)
    // log_2(factor) = log_2(min_shared_prefix_end/op_count)/max_digest_count
    // factor = 2^(log_2(min_shared_prefix_end/op_count)/max_digest_count)
    let factor = 2f64.powf(
        (min_shared_prefix_end as f64 / digest_end_ix as f64).log2() / max_digest_count as f64,
    );
    for _ in 0..max_digest_count {
        if digest_end_ix <= min_shared_prefix_end {
            break;
        }

        digests.push(digest_for_range(client_operations, 0..digest_end_ix));
        digest_end_ix = (digest_end_ix as f64 * factor).ceil() as usize; // 🪬
    }

    let server_response = sync_server(server_operations, SyncRequest { digests });
    let new_ops_from_client = {
        let mut new_ops_from_client = Vec::new();
        let mut client_cursor = client_operations.cursor::<usize>();
        let mut new_client_operations =
            client_cursor.slice(&server_response.shared_prefix_end, Bias::Right, &());

        let mut server_operations = server_response.operations.iter().peekable();
        let mut new_ops_from_server = Vec::new();
        while let Some(server_op) = server_operations.peek() {
            match client_cursor.item() {
                Some(client_operation) => {
                    let comparison = server_op.id().cmp(&client_operation.id());
                    match comparison {
                        Ordering::Less => {
                            new_ops_from_server.push(server_operations.next().unwrap().clone());
                        }
                        _ => {
                            new_client_operations.extend(new_ops_from_server.drain(..), &());
                            new_client_operations.push(client_operation.clone(), &());
                            client_cursor.next(&());
                            if comparison == Ordering::Equal {
                                server_operations.next();
                            } else {
                                new_ops_from_client.push(client_operation.clone());
                            }
                        }
                    }
                }
                None => {
                    new_ops_from_server.push(server_operations.next().unwrap().clone());
                }
            }
        }
        new_client_operations.extend(new_ops_from_server, &());

        let client_suffix = client_cursor.suffix(&());
        new_client_operations.append(client_suffix.clone(), &());
        drop(client_cursor);
        *client_operations = new_client_operations;

        new_ops_from_client.extend(client_suffix.iter().cloned());
        new_ops_from_client
    };

    let sync_stats = SyncStats {
        server_operations: server_response.operations.len(),
        client_operations: new_ops_from_client.len(),
    };
    publish_operations(
        server_operations,
        PublishOperations {
            repo_id: Default::default(),
            operations: new_ops_from_client,
        },
    );

    sync_stats
}

fn digest_for_range(operations: &btree::Sequence<Operation>, range: Range<usize>) -> Digest {
    let mut cursor = operations.cursor::<usize>();
    cursor.seek(&range.start, Bias::Right, &());
    cursor.summary(&range.end, Bias::Right, &())
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
        assert_sync(1..=10, 5..=10, 0, 16);
        assert_sync(1..=10, 4..=10, 0, 16);
        assert_sync(1..=10, 1..=5, 0, 16);
        assert_sync([1, 3, 5, 7, 9], [2, 4, 6, 8, 10], 0, 16);
        assert_sync([1, 2, 3, 4, 6, 7, 8, 9, 11, 12], [4, 5, 6, 10, 12], 0, 16);
        assert_sync(1..=10, 5..=14, 0, 16);
        assert_sync(1..=80, (1..=70).chain(90..=100), 0, 16);
        assert_sync(1..=1910, (1..=1900).chain(1910..=2000), 0, 16);
    }

    #[gpui::test(iterations = 100)]
    fn test_random(mut rng: StdRng) {
        let max_operations = env::var("OPERATIONS")
            .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
            .unwrap_or(10);
        let min_shared_prefix_end = 1024;
        let max_digest_count = 1024;

        let mut connected = true;
        let mut client_ops = btree::Sequence::new();
        let mut server_ops = btree::Sequence::new();
        let mut ideal_server_ops = 0;
        let mut ideal_client_ops = 0;
        let mut next_reconnection = None;
        for ix in 1..=max_operations {
            if connected && rng.gen_bool(0.0005) {
                dbg!(ix);
                connected = false;

                let mut factor = 0.0005;
                while rng.gen() {
                    factor *= 2.0;
                }

                let remaining_operations = max_operations - ix;
                let disconnection_period = (remaining_operations as f64 * factor) as usize;
                next_reconnection = Some(ix + disconnection_period);
                dbg!(disconnection_period);
            }

            if next_reconnection == Some(ix) {
                connected = true;
                next_reconnection = None;
                log::debug!("===============");

                let stats = sync_client(
                    &mut client_ops,
                    &mut server_ops,
                    min_shared_prefix_end,
                    max_digest_count,
                );
                log::debug!(
                    "ideal server ops: {}, actual server ops: {}, abs error: {}, pct error: {:.3}%",
                    ideal_server_ops,
                    stats.server_operations,
                    stats.server_operations - ideal_server_ops,
                    ((stats.server_operations as f64 / ideal_server_ops as f64) - 1.) * 100.
                );
                log::debug!(
                    "ideal client ops: {}, actual client ops: {}, abs error: {}, pct error: {:.3}%",
                    ideal_client_ops,
                    stats.client_operations,
                    stats.client_operations - ideal_client_ops,
                    ((stats.client_operations as f64 / ideal_client_ops as f64) - 1.0) * 100.
                );

                assert_eq!(
                    client_ops.iter().map(|op| op.id()).collect::<Vec<_>>(),
                    server_ops.iter().map(|op| op.id()).collect::<Vec<_>>()
                );
                ideal_client_ops = 0;
                ideal_server_ops = 0;
            }

            if connected {
                client_ops.push(build_operation(ix), &());
                server_ops.push(build_operation(ix), &());
            } else if rng.gen_bool(0.95) {
                ideal_server_ops += 1;
                server_ops.push(build_operation(ix), &());
            } else {
                ideal_client_ops += 1;
                client_ops.push(build_operation(ix), &());
            }
        }

        log::debug!("============");
        let stats = sync_client(
            &mut client_ops,
            &mut server_ops,
            min_shared_prefix_end,
            max_digest_count,
        );
        log::debug!(
            "ideal server ops: {}, actual server ops: {}, abs error: {}, pct error: {:.3}%",
            ideal_server_ops,
            stats.server_operations,
            stats.server_operations - ideal_server_ops,
            ((stats.server_operations as f64 / ideal_server_ops as f64) - 1.) * 100.
        );
        log::debug!(
            "ideal client ops: {}, actual client ops: {}, abs error: {}, pct error: {:.3}%",
            ideal_client_ops,
            stats.client_operations,
            stats.client_operations - ideal_client_ops,
            ((stats.client_operations as f64 / ideal_client_ops as f64) - 1.0) * 100.
        );
        assert_eq!(
            client_ops.iter().map(|op| op.id()).collect::<Vec<_>>(),
            server_ops.iter().map(|op| op.id()).collect::<Vec<_>>()
        );
    }

    fn assert_sync(
        client_ops: impl IntoIterator<Item = usize>,
        server_ops: impl IntoIterator<Item = usize>,
        min_digest_delta: usize,
        max_digest_count: usize,
    ) {
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
        sync_client(
            &mut client_operations,
            &mut server_operations,
            min_digest_delta,
            max_digest_count,
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
