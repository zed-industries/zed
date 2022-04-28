use crate::{
    diagnostic_set::DiagnosticEntry, CodeAction, CodeLabel, Completion, Diagnostic, Language,
    Operation,
};
use anyhow::{anyhow, Result};
use clock::ReplicaId;
use lsp::DiagnosticSeverity;
use rpc::proto;
use std::{ops::Range, sync::Arc};
use text::*;

pub use proto::{Buffer, BufferState, SelectionSet};

pub fn serialize_operation(operation: &Operation) -> proto::Operation {
    proto::Operation {
        variant: Some(match operation {
            Operation::Buffer(text::Operation::Edit(edit)) => {
                proto::operation::Variant::Edit(serialize_edit_operation(edit))
            }
            Operation::Buffer(text::Operation::Undo {
                undo,
                lamport_timestamp,
            }) => proto::operation::Variant::Undo(proto::operation::Undo {
                replica_id: undo.id.replica_id as u32,
                local_timestamp: undo.id.value,
                lamport_timestamp: lamport_timestamp.value,
                version: serialize_version(&undo.version),
                transaction_ranges: undo
                    .transaction_ranges
                    .iter()
                    .map(serialize_range)
                    .collect(),
                transaction_version: serialize_version(&undo.transaction_version),
                counts: undo
                    .counts
                    .iter()
                    .map(|(edit_id, count)| proto::UndoCount {
                        replica_id: edit_id.replica_id as u32,
                        local_timestamp: edit_id.value,
                        count: *count,
                    })
                    .collect(),
            }),
            Operation::UpdateSelections {
                selections,
                lamport_timestamp,
            } => proto::operation::Variant::UpdateSelections(proto::operation::UpdateSelections {
                replica_id: lamport_timestamp.replica_id as u32,
                lamport_timestamp: lamport_timestamp.value,
                selections: serialize_selections(selections),
            }),
            Operation::UpdateDiagnostics {
                diagnostics,
                lamport_timestamp,
            } => proto::operation::Variant::UpdateDiagnostics(proto::UpdateDiagnostics {
                replica_id: lamport_timestamp.replica_id as u32,
                lamport_timestamp: lamport_timestamp.value,
                diagnostics: serialize_diagnostics(diagnostics.iter()),
            }),
            Operation::UpdateCompletionTriggers {
                triggers,
                lamport_timestamp,
            } => proto::operation::Variant::UpdateCompletionTriggers(
                proto::operation::UpdateCompletionTriggers {
                    replica_id: lamport_timestamp.replica_id as u32,
                    lamport_timestamp: lamport_timestamp.value,
                    triggers: triggers.clone(),
                },
            ),
        }),
    }
}

pub fn serialize_edit_operation(operation: &EditOperation) -> proto::operation::Edit {
    proto::operation::Edit {
        replica_id: operation.timestamp.replica_id as u32,
        local_timestamp: operation.timestamp.local,
        lamport_timestamp: operation.timestamp.lamport,
        version: serialize_version(&operation.version),
        ranges: operation.ranges.iter().map(serialize_range).collect(),
        new_text: operation
            .new_text
            .iter()
            .map(|text| text.to_string())
            .collect(),
    }
}

pub fn serialize_undo_map_entry(
    (edit_id, counts): (&clock::Local, &[(clock::Local, u32)]),
) -> proto::UndoMapEntry {
    proto::UndoMapEntry {
        replica_id: edit_id.replica_id as u32,
        local_timestamp: edit_id.value,
        counts: counts
            .iter()
            .map(|(undo_id, count)| proto::UndoCount {
                replica_id: undo_id.replica_id as u32,
                local_timestamp: undo_id.value,
                count: *count,
            })
            .collect(),
    }
}

pub fn serialize_selections(selections: &Arc<[Selection<Anchor>]>) -> Vec<proto::Selection> {
    selections.iter().map(serialize_selection).collect()
}

pub fn serialize_selection(selection: &Selection<Anchor>) -> proto::Selection {
    proto::Selection {
        id: selection.id as u64,
        start: Some(serialize_anchor(&selection.start)),
        end: Some(serialize_anchor(&selection.end)),
        reversed: selection.reversed,
    }
}

pub fn serialize_diagnostics<'a>(
    diagnostics: impl IntoIterator<Item = &'a DiagnosticEntry<Anchor>>,
) -> Vec<proto::Diagnostic> {
    diagnostics
        .into_iter()
        .map(|entry| proto::Diagnostic {
            start: Some(serialize_anchor(&entry.range.start)),
            end: Some(serialize_anchor(&entry.range.end)),
            message: entry.diagnostic.message.clone(),
            severity: match entry.diagnostic.severity {
                DiagnosticSeverity::ERROR => proto::diagnostic::Severity::Error,
                DiagnosticSeverity::WARNING => proto::diagnostic::Severity::Warning,
                DiagnosticSeverity::INFORMATION => proto::diagnostic::Severity::Information,
                DiagnosticSeverity::HINT => proto::diagnostic::Severity::Hint,
                _ => proto::diagnostic::Severity::None,
            } as i32,
            group_id: entry.diagnostic.group_id as u64,
            is_primary: entry.diagnostic.is_primary,
            is_valid: entry.diagnostic.is_valid,
            code: entry.diagnostic.code.clone(),
            is_disk_based: entry.diagnostic.is_disk_based,
            is_unnecessary: entry.diagnostic.is_unnecessary,
        })
        .collect()
}

pub fn serialize_anchor(anchor: &Anchor) -> proto::Anchor {
    proto::Anchor {
        replica_id: anchor.timestamp.replica_id as u32,
        local_timestamp: anchor.timestamp.value,
        offset: anchor.offset as u64,
        bias: match anchor.bias {
            Bias::Left => proto::Bias::Left as i32,
            Bias::Right => proto::Bias::Right as i32,
        },
    }
}

pub fn deserialize_operation(message: proto::Operation) -> Result<Operation> {
    Ok(
        match message
            .variant
            .ok_or_else(|| anyhow!("missing operation variant"))?
        {
            proto::operation::Variant::Edit(edit) => {
                Operation::Buffer(text::Operation::Edit(deserialize_edit_operation(edit)))
            }
            proto::operation::Variant::Undo(undo) => Operation::Buffer(text::Operation::Undo {
                lamport_timestamp: clock::Lamport {
                    replica_id: undo.replica_id as ReplicaId,
                    value: undo.lamport_timestamp,
                },
                undo: UndoOperation {
                    id: clock::Local {
                        replica_id: undo.replica_id as ReplicaId,
                        value: undo.local_timestamp,
                    },
                    version: deserialize_version(undo.version),
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
                    transaction_ranges: undo
                        .transaction_ranges
                        .into_iter()
                        .map(deserialize_range)
                        .collect(),
                    transaction_version: deserialize_version(undo.transaction_version),
                },
            }),
            proto::operation::Variant::UpdateSelections(message) => {
                let selections = message
                    .selections
                    .into_iter()
                    .filter_map(|selection| {
                        Some(Selection {
                            id: selection.id as usize,
                            start: deserialize_anchor(selection.start?)?,
                            end: deserialize_anchor(selection.end?)?,
                            reversed: selection.reversed,
                            goal: SelectionGoal::None,
                        })
                    })
                    .collect::<Vec<_>>();

                Operation::UpdateSelections {
                    lamport_timestamp: clock::Lamport {
                        replica_id: message.replica_id as ReplicaId,
                        value: message.lamport_timestamp,
                    },
                    selections: Arc::from(selections),
                }
            }
            proto::operation::Variant::UpdateDiagnostics(message) => Operation::UpdateDiagnostics {
                diagnostics: deserialize_diagnostics(message.diagnostics),
                lamport_timestamp: clock::Lamport {
                    replica_id: message.replica_id as ReplicaId,
                    value: message.lamport_timestamp,
                },
            },
            proto::operation::Variant::UpdateCompletionTriggers(message) => {
                Operation::UpdateCompletionTriggers {
                    triggers: message.triggers,
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
    EditOperation {
        timestamp: InsertionTimestamp {
            replica_id: edit.replica_id as ReplicaId,
            local: edit.local_timestamp,
            lamport: edit.lamport_timestamp,
        },
        version: deserialize_version(edit.version),
        ranges: edit.ranges.into_iter().map(deserialize_range).collect(),
        new_text: edit.new_text.into_iter().map(Arc::from).collect(),
    }
}

pub fn deserialize_undo_map_entry(
    entry: proto::UndoMapEntry,
) -> (clock::Local, Vec<(clock::Local, u32)>) {
    (
        clock::Local {
            replica_id: entry.replica_id as u16,
            value: entry.local_timestamp,
        },
        entry
            .counts
            .into_iter()
            .map(|undo_count| {
                (
                    clock::Local {
                        replica_id: undo_count.replica_id as u16,
                        value: undo_count.local_timestamp,
                    },
                    undo_count.count,
                )
            })
            .collect(),
    )
}

pub fn deserialize_selections(selections: Vec<proto::Selection>) -> Arc<[Selection<Anchor>]> {
    Arc::from(
        selections
            .into_iter()
            .filter_map(deserialize_selection)
            .collect::<Vec<_>>(),
    )
}

pub fn deserialize_selection(selection: proto::Selection) -> Option<Selection<Anchor>> {
    Some(Selection {
        id: selection.id as usize,
        start: deserialize_anchor(selection.start?)?,
        end: deserialize_anchor(selection.end?)?,
        reversed: selection.reversed,
        goal: SelectionGoal::None,
    })
}

pub fn deserialize_diagnostics(
    diagnostics: Vec<proto::Diagnostic>,
) -> Arc<[DiagnosticEntry<Anchor>]> {
    diagnostics
        .into_iter()
        .filter_map(|diagnostic| {
            Some(DiagnosticEntry {
                range: deserialize_anchor(diagnostic.start?)?..deserialize_anchor(diagnostic.end?)?,
                diagnostic: Diagnostic {
                    severity: match proto::diagnostic::Severity::from_i32(diagnostic.severity)? {
                        proto::diagnostic::Severity::Error => DiagnosticSeverity::ERROR,
                        proto::diagnostic::Severity::Warning => DiagnosticSeverity::WARNING,
                        proto::diagnostic::Severity::Information => DiagnosticSeverity::INFORMATION,
                        proto::diagnostic::Severity::Hint => DiagnosticSeverity::HINT,
                        proto::diagnostic::Severity::None => return None,
                    },
                    message: diagnostic.message,
                    group_id: diagnostic.group_id as usize,
                    code: diagnostic.code,
                    is_valid: diagnostic.is_valid,
                    is_primary: diagnostic.is_primary,
                    is_disk_based: diagnostic.is_disk_based,
                    is_unnecessary: diagnostic.is_unnecessary,
                },
            })
        })
        .collect()
}

pub fn deserialize_anchor(anchor: proto::Anchor) -> Option<Anchor> {
    Some(Anchor {
        timestamp: clock::Local {
            replica_id: anchor.replica_id as ReplicaId,
            value: anchor.local_timestamp,
        },
        offset: anchor.offset as usize,
        bias: match proto::Bias::from_i32(anchor.bias)? {
            proto::Bias::Left => Bias::Left,
            proto::Bias::Right => Bias::Right,
        },
    })
}

pub fn lamport_timestamp_for_operation(operation: &proto::Operation) -> Option<clock::Lamport> {
    let replica_id;
    let value;
    match operation.variant.as_ref()? {
        proto::operation::Variant::Edit(op) => {
            replica_id = op.replica_id;
            value = op.lamport_timestamp;
        }
        proto::operation::Variant::Undo(op) => {
            replica_id = op.replica_id;
            value = op.lamport_timestamp;
        }
        proto::operation::Variant::UpdateDiagnostics(op) => {
            replica_id = op.replica_id;
            value = op.lamport_timestamp;
        }
        proto::operation::Variant::UpdateSelections(op) => {
            replica_id = op.replica_id;
            value = op.lamport_timestamp;
        }
        proto::operation::Variant::UpdateCompletionTriggers(op) => {
            replica_id = op.replica_id;
            value = op.lamport_timestamp;
        }
    }

    Some(clock::Lamport {
        replica_id: replica_id as ReplicaId,
        value,
    })
}

pub fn serialize_completion(completion: &Completion) -> proto::Completion {
    proto::Completion {
        old_start: Some(serialize_anchor(&completion.old_range.start)),
        old_end: Some(serialize_anchor(&completion.old_range.end)),
        new_text: completion.new_text.clone(),
        lsp_completion: serde_json::to_vec(&completion.lsp_completion).unwrap(),
    }
}

pub fn deserialize_completion(
    completion: proto::Completion,
    language: Option<&Arc<Language>>,
) -> Result<Completion> {
    let old_start = completion
        .old_start
        .and_then(deserialize_anchor)
        .ok_or_else(|| anyhow!("invalid old start"))?;
    let old_end = completion
        .old_end
        .and_then(deserialize_anchor)
        .ok_or_else(|| anyhow!("invalid old end"))?;
    let lsp_completion = serde_json::from_slice(&completion.lsp_completion)?;
    Ok(Completion {
        old_range: old_start..old_end,
        new_text: completion.new_text,
        label: language
            .and_then(|l| l.label_for_completion(&lsp_completion))
            .unwrap_or(CodeLabel::plain(
                lsp_completion.label.clone(),
                lsp_completion.filter_text.as_deref(),
            )),
        lsp_completion,
    })
}

pub fn serialize_code_action(action: &CodeAction) -> proto::CodeAction {
    proto::CodeAction {
        start: Some(serialize_anchor(&action.range.start)),
        end: Some(serialize_anchor(&action.range.end)),
        lsp_action: serde_json::to_vec(&action.lsp_action).unwrap(),
    }
}

pub fn deserialize_code_action(action: proto::CodeAction) -> Result<CodeAction> {
    let start = action
        .start
        .and_then(deserialize_anchor)
        .ok_or_else(|| anyhow!("invalid start"))?;
    let end = action
        .end
        .and_then(deserialize_anchor)
        .ok_or_else(|| anyhow!("invalid end"))?;
    let lsp_action = serde_json::from_slice(&action.lsp_action)?;
    Ok(CodeAction {
        range: start..end,
        lsp_action,
    })
}

pub fn serialize_transaction(transaction: &Transaction) -> proto::Transaction {
    proto::Transaction {
        id: Some(serialize_local_timestamp(transaction.id)),
        edit_ids: transaction
            .edit_ids
            .iter()
            .copied()
            .map(serialize_local_timestamp)
            .collect(),
        start: serialize_version(&transaction.start),
        end: serialize_version(&transaction.end),
        ranges: transaction.ranges.iter().map(serialize_range).collect(),
    }
}

pub fn deserialize_transaction(transaction: proto::Transaction) -> Result<Transaction> {
    Ok(Transaction {
        id: deserialize_local_timestamp(
            transaction
                .id
                .ok_or_else(|| anyhow!("missing transaction id"))?,
        ),
        edit_ids: transaction
            .edit_ids
            .into_iter()
            .map(deserialize_local_timestamp)
            .collect(),
        start: deserialize_version(transaction.start.into()),
        end: deserialize_version(transaction.end),
        ranges: transaction
            .ranges
            .into_iter()
            .map(deserialize_range)
            .collect(),
    })
}

pub fn serialize_local_timestamp(timestamp: clock::Local) -> proto::LocalTimestamp {
    proto::LocalTimestamp {
        replica_id: timestamp.replica_id as u32,
        value: timestamp.value,
    }
}

pub fn deserialize_local_timestamp(timestamp: proto::LocalTimestamp) -> clock::Local {
    clock::Local {
        replica_id: timestamp.replica_id as ReplicaId,
        value: timestamp.value,
    }
}

pub fn serialize_range(range: &Range<FullOffset>) -> proto::Range {
    proto::Range {
        start: range.start.0 as u64,
        end: range.end.0 as u64,
    }
}

pub fn deserialize_range(range: proto::Range) -> Range<FullOffset> {
    FullOffset(range.start as usize)..FullOffset(range.end as usize)
}

pub fn deserialize_version(message: Vec<proto::VectorClockEntry>) -> clock::Global {
    let mut version = clock::Global::new();
    for entry in message {
        version.observe(clock::Local {
            replica_id: entry.replica_id as ReplicaId,
            value: entry.timestamp,
        });
    }
    version
}

pub fn serialize_version(version: &clock::Global) -> Vec<proto::VectorClockEntry> {
    version
        .iter()
        .map(|entry| proto::VectorClockEntry {
            replica_id: entry.replica_id as u32,
            timestamp: entry.value,
        })
        .collect()
}
