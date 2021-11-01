use std::sync::Arc;

use anyhow::{anyhow, Result};
use buffer::*;
use clock::ReplicaId;
use rpc::proto;

pub use proto::Buffer;

pub fn serialize_operation(operation: &Operation) -> proto::Operation {
    proto::Operation {
        variant: Some(match operation {
            Operation::Edit(edit) => {
                proto::operation::Variant::Edit(serialize_edit_operation(edit))
            }
            Operation::Undo {
                undo,
                lamport_timestamp,
            } => proto::operation::Variant::Undo(proto::operation::Undo {
                replica_id: undo.id.replica_id as u32,
                local_timestamp: undo.id.value,
                lamport_timestamp: lamport_timestamp.value,
                ranges: undo
                    .ranges
                    .iter()
                    .map(|r| proto::Range {
                        start: r.start.0 as u64,
                        end: r.end.0 as u64,
                    })
                    .collect(),
                counts: undo
                    .counts
                    .iter()
                    .map(|(edit_id, count)| proto::operation::UndoCount {
                        replica_id: edit_id.replica_id as u32,
                        local_timestamp: edit_id.value,
                        count: *count,
                    })
                    .collect(),
                version: From::from(&undo.version),
            }),
            Operation::UpdateSelections {
                set_id,
                selections,
                lamport_timestamp,
            } => proto::operation::Variant::UpdateSelections(proto::operation::UpdateSelections {
                replica_id: set_id.replica_id as u32,
                local_timestamp: set_id.value,
                lamport_timestamp: lamport_timestamp.value,
                version: selections.version().into(),
                selections: selections
                    .raw_entries()
                    .iter()
                    .map(|(range, state)| proto::Selection {
                        id: state.id as u64,
                        start: range.start.0 .0 as u64,
                        end: range.end.0 .0 as u64,
                        reversed: state.reversed,
                    })
                    .collect(),
            }),
            Operation::RemoveSelections {
                set_id,
                lamport_timestamp,
            } => proto::operation::Variant::RemoveSelections(proto::operation::RemoveSelections {
                replica_id: set_id.replica_id as u32,
                local_timestamp: set_id.value,
                lamport_timestamp: lamport_timestamp.value,
            }),
            Operation::SetActiveSelections {
                set_id,
                lamport_timestamp,
            } => proto::operation::Variant::SetActiveSelections(
                proto::operation::SetActiveSelections {
                    replica_id: lamport_timestamp.replica_id as u32,
                    local_timestamp: set_id.map(|set_id| set_id.value),
                    lamport_timestamp: lamport_timestamp.value,
                },
            ),
        }),
    }
}

pub fn serialize_edit_operation(operation: &EditOperation) -> proto::operation::Edit {
    let ranges = operation
        .ranges
        .iter()
        .map(|range| proto::Range {
            start: range.start.0 as u64,
            end: range.end.0 as u64,
        })
        .collect();
    proto::operation::Edit {
        replica_id: operation.timestamp.replica_id as u32,
        local_timestamp: operation.timestamp.local,
        lamport_timestamp: operation.timestamp.lamport,
        version: From::from(&operation.version),
        ranges,
        new_text: operation.new_text.clone(),
    }
}

pub fn serialize_selection_set(set: &SelectionSet) -> proto::SelectionSet {
    let version = set.selections.version();
    let entries = set.selections.raw_entries();
    proto::SelectionSet {
        replica_id: set.id.replica_id as u32,
        lamport_timestamp: set.id.value as u32,
        is_active: set.active,
        version: version.into(),
        selections: entries
            .iter()
            .map(|(range, state)| proto::Selection {
                id: state.id as u64,
                start: range.start.0 .0 as u64,
                end: range.end.0 .0 as u64,
                reversed: state.reversed,
            })
            .collect(),
    }
}

pub fn deserialize_operation(message: proto::Operation) -> Result<Operation> {
    Ok(
        match message
            .variant
            .ok_or_else(|| anyhow!("missing operation variant"))?
        {
            proto::operation::Variant::Edit(edit) => {
                Operation::Edit(deserialize_edit_operation(edit))
            }
            proto::operation::Variant::Undo(undo) => Operation::Undo {
                lamport_timestamp: clock::Lamport {
                    replica_id: undo.replica_id as ReplicaId,
                    value: undo.lamport_timestamp,
                },
                undo: UndoOperation {
                    id: clock::Local {
                        replica_id: undo.replica_id as ReplicaId,
                        value: undo.local_timestamp,
                    },
                    counts: undo
                        .counts
                        .into_iter()
                        .map(|c| {
                            (
                                clock::Local {
                                    replica_id: c.replica_id as ReplicaId,
                                    value: c.local_timestamp,
                                },
                                c.count,
                            )
                        })
                        .collect(),
                    ranges: undo
                        .ranges
                        .into_iter()
                        .map(|r| FullOffset(r.start as usize)..FullOffset(r.end as usize))
                        .collect(),
                    version: undo.version.into(),
                },
            },
            proto::operation::Variant::UpdateSelections(message) => {
                let version = message.version.into();
                let entries = message
                    .selections
                    .iter()
                    .map(|selection| {
                        let range = (FullOffset(selection.start as usize), Bias::Left)
                            ..(FullOffset(selection.end as usize), Bias::Right);
                        let state = SelectionState {
                            id: selection.id as usize,
                            reversed: selection.reversed,
                            goal: SelectionGoal::None,
                        };
                        (range, state)
                    })
                    .collect();
                let selections = AnchorRangeMap::from_raw(version, entries);

                Operation::UpdateSelections {
                    set_id: clock::Lamport {
                        replica_id: message.replica_id as ReplicaId,
                        value: message.local_timestamp,
                    },
                    lamport_timestamp: clock::Lamport {
                        replica_id: message.replica_id as ReplicaId,
                        value: message.lamport_timestamp,
                    },
                    selections: Arc::from(selections),
                }
            }
            proto::operation::Variant::RemoveSelections(message) => Operation::RemoveSelections {
                set_id: clock::Lamport {
                    replica_id: message.replica_id as ReplicaId,
                    value: message.local_timestamp,
                },
                lamport_timestamp: clock::Lamport {
                    replica_id: message.replica_id as ReplicaId,
                    value: message.lamport_timestamp,
                },
            },
            proto::operation::Variant::SetActiveSelections(message) => {
                Operation::SetActiveSelections {
                    set_id: message.local_timestamp.map(|value| clock::Lamport {
                        replica_id: message.replica_id as ReplicaId,
                        value,
                    }),
                    lamport_timestamp: clock::Lamport {
                        replica_id: message.replica_id as ReplicaId,
                        value: message.lamport_timestamp,
                    },
                }
            }
        },
    )
}

pub fn deserialize_edit_operation(edit: proto::operation::Edit) -> EditOperation {
    let ranges = edit
        .ranges
        .into_iter()
        .map(|range| FullOffset(range.start as usize)..FullOffset(range.end as usize))
        .collect();
    EditOperation {
        timestamp: InsertionTimestamp {
            replica_id: edit.replica_id as ReplicaId,
            local: edit.local_timestamp,
            lamport: edit.lamport_timestamp,
        },
        version: edit.version.into(),
        ranges,
        new_text: edit.new_text,
    }
}

pub fn deserialize_selection_set(set: proto::SelectionSet) -> SelectionSet {
    SelectionSet {
        id: clock::Lamport {
            replica_id: set.replica_id as u16,
            value: set.lamport_timestamp,
        },
        active: set.is_active,
        selections: Arc::new(AnchorRangeMap::from_raw(
            set.version.into(),
            set.selections
                .into_iter()
                .map(|selection| {
                    let range = (FullOffset(selection.start as usize), Bias::Left)
                        ..(FullOffset(selection.end as usize), Bias::Right);
                    let state = SelectionState {
                        id: selection.id as usize,
                        reversed: selection.reversed,
                        goal: SelectionGoal::None,
                    };
                    (range, state)
                })
                .collect(),
        )),
    }
}
