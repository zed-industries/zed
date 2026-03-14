use anyhow::Context as _;
use serde::{Deserialize, Serialize};
use std::ops::Range;
use std::sync::Arc;
use sum_tree::TreeMap;

use super::{EditOperation, FullOffset, HistoryEntry, Operation, Transaction, UndoOperation};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct SerializedLamport {
    replica_id: u16,
    value: u32,
}

impl From<clock::Lamport> for SerializedLamport {
    fn from(lamport: clock::Lamport) -> Self {
        SerializedLamport {
            replica_id: lamport.replica_id.as_u16(),
            value: lamport.value,
        }
    }
}

impl From<SerializedLamport> for clock::Lamport {
    fn from(serialized: SerializedLamport) -> Self {
        clock::Lamport {
            replica_id: clock::ReplicaId::new(serialized.replica_id),
            value: serialized.value,
        }
    }
}

fn serialize_global(version: &clock::Global) -> Vec<u32> {
    let mut values = Vec::new();
    for lamport in version.iter() {
        let idx = lamport.replica_id.as_u16() as usize;
        if values.len() <= idx {
            values.resize(idx + 1, 0);
        }
        values[idx] = lamport.value;
    }
    values
}

fn deserialize_global(values: &[u32]) -> clock::Global {
    values
        .iter()
        .enumerate()
        .filter(|(_, value)| **value > 0)
        .map(|(replica_idx, &value)| clock::Lamport {
            replica_id: clock::ReplicaId::new(replica_idx as u16),
            value,
        })
        .collect()
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct SerializedHistoryEntry {
    id: SerializedLamport,
    edit_ids: Vec<SerializedLamport>,
    start: Vec<u32>,
    suppress_grouping: bool,
}

impl From<&HistoryEntry> for SerializedHistoryEntry {
    fn from(entry: &HistoryEntry) -> Self {
        let transaction = entry.transaction();
        SerializedHistoryEntry {
            id: SerializedLamport::from(transaction.id),
            edit_ids: transaction
                .edit_ids
                .iter()
                .copied()
                .map(SerializedLamport::from)
                .collect(),
            start: serialize_global(&transaction.start),
            suppress_grouping: entry.suppress_grouping(),
        }
    }
}

impl From<SerializedHistoryEntry> for Transaction {
    fn from(entry: SerializedHistoryEntry) -> Self {
        Transaction {
            id: clock::Lamport::from(entry.id),
            edit_ids: entry
                .edit_ids
                .into_iter()
                .map(clock::Lamport::from)
                .collect(),
            start: deserialize_global(&entry.start),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct SerializedEditOperation {
    timestamp: SerializedLamport,
    version: Vec<u32>,
    ranges: Vec<(u64, u64)>,
    new_text: Vec<Arc<str>>,
}

impl From<&EditOperation> for SerializedEditOperation {
    fn from(op: &EditOperation) -> Self {
        SerializedEditOperation {
            timestamp: SerializedLamport::from(op.timestamp),
            version: serialize_global(&op.version),
            ranges: op
                .ranges
                .iter()
                .map(|range| (range.start.0 as u64, range.end.0 as u64))
                .collect(),
            new_text: op.new_text.clone(),
        }
    }
}

impl From<SerializedEditOperation> for EditOperation {
    fn from(op: SerializedEditOperation) -> Self {
        EditOperation {
            timestamp: clock::Lamport::from(op.timestamp),
            version: deserialize_global(&op.version),
            ranges: op
                .ranges
                .into_iter()
                .map(|(start, end)| Range {
                    start: FullOffset(start as usize),
                    end: FullOffset(end as usize),
                })
                .collect(),
            new_text: op.new_text,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct SerializedUndoOperation {
    timestamp: SerializedLamport,
    version: Vec<u32>,
    counts: Vec<(SerializedLamport, u32)>,
}

impl From<&UndoOperation> for SerializedUndoOperation {
    fn from(op: &UndoOperation) -> Self {
        let mut counts: Vec<(SerializedLamport, u32)> = op
            .counts
            .iter()
            .map(|(key, &value)| (SerializedLamport::from(*key), value))
            .collect();
        counts.sort_by_key(|(key, _)| (key.value, key.replica_id));
        SerializedUndoOperation {
            timestamp: SerializedLamport::from(op.timestamp),
            version: serialize_global(&op.version),
            counts,
        }
    }
}

impl From<SerializedUndoOperation> for UndoOperation {
    fn from(op: SerializedUndoOperation) -> Self {
        UndoOperation {
            timestamp: clock::Lamport::from(op.timestamp),
            version: deserialize_global(&op.version),
            counts: op
                .counts
                .into_iter()
                .map(|(key, value)| (clock::Lamport::from(key), value))
                .collect(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
enum SerializedOperation {
    Edit(SerializedEditOperation),
    Undo(SerializedUndoOperation),
}

impl From<&Operation> for SerializedOperation {
    fn from(op: &Operation) -> Self {
        match op {
            Operation::Edit(edit) => SerializedOperation::Edit(SerializedEditOperation::from(edit)),
            Operation::Undo(undo) => {
                SerializedOperation::Undo(SerializedUndoOperation::from(undo))
            }
        }
    }
}

impl From<SerializedOperation> for Operation {
    fn from(op: SerializedOperation) -> Self {
        match op {
            SerializedOperation::Edit(edit) => Operation::Edit(EditOperation::from(edit)),
            SerializedOperation::Undo(undo) => Operation::Undo(UndoOperation::from(undo)),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct SerializedUndoHistory {
    undo_stack: Vec<SerializedHistoryEntry>,
    redo_stack: Vec<SerializedHistoryEntry>,
    operations: Vec<SerializedOperation>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct SerializedUndoHistoryV2 {
    base_text: String,
    undo_stack: Vec<SerializedHistoryEntry>,
    redo_stack: Vec<SerializedHistoryEntry>,
    operations: Vec<SerializedOperation>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) enum HistoryBlob {
    V1(SerializedUndoHistory),
    V2(SerializedUndoHistoryV2),
}

pub fn encode_history(
    base_text: &rope::Rope,
    undo_stack: &[HistoryEntry],
    redo_stack: &[HistoryEntry],
    operations: &TreeMap<clock::Lamport, Operation>,
) -> anyhow::Result<Vec<u8>> {
    // Include ALL operations, not just those referenced by the kept undo/redo
    // entries. CRDT edit operations depend on each other (later inserts reference
    // positions created by earlier ones), so dropping any edit breaks the rebuild.
    let serialized_ops: Vec<SerializedOperation> =
        operations.values().map(SerializedOperation::from).collect();

    let history = SerializedUndoHistoryV2 {
        base_text: base_text.to_string(),
        undo_stack: undo_stack
            .iter()
            .map(SerializedHistoryEntry::from)
            .collect(),
        redo_stack: redo_stack
            .iter()
            .map(SerializedHistoryEntry::from)
            .collect(),
        operations: serialized_ops,
    };

    let blob = HistoryBlob::V2(history);
    postcard::to_allocvec(&blob).context("failed to encode history blob")
}

pub struct DecodedHistory {
    pub base_text: String,
    pub undo_stack: Vec<Transaction>,
    pub redo_stack: Vec<Transaction>,
    pub operations: Vec<Operation>,
}

pub fn decode_history(bytes: &[u8]) -> anyhow::Result<DecodedHistory> {
    let blob: HistoryBlob =
        postcard::from_bytes(bytes).context("failed to decode history blob")?;
    match blob {
        HistoryBlob::V1(_) => {
            anyhow::bail!("V1 history format is no longer supported");
        }
        HistoryBlob::V2(history) => {
            let undo_stack = history
                .undo_stack
                .into_iter()
                .map(Transaction::from)
                .collect();
            let redo_stack = history
                .redo_stack
                .into_iter()
                .map(Transaction::from)
                .collect();
            let operations = history
                .operations
                .into_iter()
                .map(Operation::from)
                .collect();
            Ok(DecodedHistory {
                base_text: history.base_text,
                undo_stack,
                redo_stack,
                operations,
            })
        }
    }
}
