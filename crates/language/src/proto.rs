//! Handles conversions of `language` items to and from the [`rpc`] protocol.

use crate::{CursorShape, Diagnostic, DiagnosticSourceKind, diagnostic_set::DiagnosticEntry};
use anyhow::{Context as _, Result};
use clock::ReplicaId;
use lsp::{DiagnosticSeverity, LanguageServerId};
use rpc::proto;
use serde_json::Value;
use std::{ops::Range, str::FromStr, sync::Arc};
use text::*;

pub use proto::{BufferState, File, Operation};

use super::{point_from_lsp, point_to_lsp};

/// Deserializes a `[text::LineEnding]` from the RPC representation.
pub fn deserialize_line_ending(message: proto::LineEnding) -> text::LineEnding {
    match message {
        proto::LineEnding::Unix => text::LineEnding::Unix,
        proto::LineEnding::Windows => text::LineEnding::Windows,
    }
}

/// Serializes a [`text::LineEnding`] to be sent over RPC.
pub fn serialize_line_ending(message: text::LineEnding) -> proto::LineEnding {
    match message {
        text::LineEnding::Unix => proto::LineEnding::Unix,
        text::LineEnding::Windows => proto::LineEnding::Windows,
    }
}

/// Serializes a [`crate::Operation`] to be sent over RPC.
pub fn serialize_operation(operation: &crate::Operation) -> proto::Operation {
    proto::Operation {
        variant: Some(match operation {
            crate::Operation::Buffer(text::Operation::Edit(edit)) => {
                proto::operation::Variant::Edit(serialize_edit_operation(edit))
            }

            crate::Operation::Buffer(text::Operation::Undo(undo)) => {
                proto::operation::Variant::Undo(proto::operation::Undo {
                    replica_id: undo.timestamp.replica_id as u32,
                    lamport_timestamp: undo.timestamp.value,
                    version: serialize_version(&undo.version),
                    counts: undo
                        .counts
                        .iter()
                        .map(|(edit_id, count)| proto::UndoCount {
                            replica_id: edit_id.replica_id as u32,
                            lamport_timestamp: edit_id.value,
                            count: *count,
                        })
                        .collect(),
                })
            }

            crate::Operation::UpdateSelections {
                selections,
                line_mode,
                lamport_timestamp,
                cursor_shape,
            } => proto::operation::Variant::UpdateSelections(proto::operation::UpdateSelections {
                replica_id: lamport_timestamp.replica_id as u32,
                lamport_timestamp: lamport_timestamp.value,
                selections: serialize_selections(selections),
                line_mode: *line_mode,
                cursor_shape: serialize_cursor_shape(cursor_shape) as i32,
            }),

            crate::Operation::UpdateDiagnostics {
                lamport_timestamp,
                server_id,
                diagnostics,
            } => proto::operation::Variant::UpdateDiagnostics(proto::UpdateDiagnostics {
                replica_id: lamport_timestamp.replica_id as u32,
                lamport_timestamp: lamport_timestamp.value,
                server_id: server_id.0 as u64,
                diagnostics: serialize_diagnostics(diagnostics.iter()),
            }),

            crate::Operation::UpdateCompletionTriggers {
                triggers,
                lamport_timestamp,
                server_id,
            } => proto::operation::Variant::UpdateCompletionTriggers(
                proto::operation::UpdateCompletionTriggers {
                    replica_id: lamport_timestamp.replica_id as u32,
                    lamport_timestamp: lamport_timestamp.value,
                    triggers: triggers.clone(),
                    language_server_id: server_id.to_proto(),
                },
            ),
        }),
    }
}

/// Serializes an [`EditOperation`] to be sent over RPC.
pub fn serialize_edit_operation(operation: &EditOperation) -> proto::operation::Edit {
    proto::operation::Edit {
        replica_id: operation.timestamp.replica_id as u32,
        lamport_timestamp: operation.timestamp.value,
        version: serialize_version(&operation.version),
        ranges: operation.ranges.iter().map(serialize_range).collect(),
        new_text: operation
            .new_text
            .iter()
            .map(|text| text.to_string())
            .collect(),
    }
}

/// Serializes an entry in the undo map to be sent over RPC.
pub fn serialize_undo_map_entry(
    (edit_id, counts): (&clock::Lamport, &[(clock::Lamport, u32)]),
) -> proto::UndoMapEntry {
    proto::UndoMapEntry {
        replica_id: edit_id.replica_id as u32,
        local_timestamp: edit_id.value,
        counts: counts
            .iter()
            .map(|(undo_id, count)| proto::UndoCount {
                replica_id: undo_id.replica_id as u32,
                lamport_timestamp: undo_id.value,
                count: *count,
            })
            .collect(),
    }
}

/// Splits the given list of operations into chunks.
pub fn split_operations(
    mut operations: Vec<proto::Operation>,
) -> impl Iterator<Item = Vec<proto::Operation>> {
    #[cfg(any(test, feature = "test-support"))]
    const CHUNK_SIZE: usize = 5;

    #[cfg(not(any(test, feature = "test-support")))]
    const CHUNK_SIZE: usize = 100;

    let mut done = false;
    std::iter::from_fn(move || {
        if done {
            return None;
        }

        let operations = operations
            .drain(..std::cmp::min(CHUNK_SIZE, operations.len()))
            .collect::<Vec<_>>();
        if operations.is_empty() {
            done = true;
        }
        Some(operations)
    })
}

/// Serializes selections to be sent over RPC.
pub fn serialize_selections(selections: &Arc<[Selection<Anchor>]>) -> Vec<proto::Selection> {
    selections.iter().map(serialize_selection).collect()
}

/// Serializes a [`Selection`] to be sent over RPC.
pub fn serialize_selection(selection: &Selection<Anchor>) -> proto::Selection {
    proto::Selection {
        id: selection.id as u64,
        start: Some(proto::EditorAnchor {
            anchor: Some(serialize_anchor(&selection.start)),
            excerpt_id: 0,
        }),
        end: Some(proto::EditorAnchor {
            anchor: Some(serialize_anchor(&selection.end)),
            excerpt_id: 0,
        }),
        reversed: selection.reversed,
    }
}

/// Serializes a [`CursorShape`] to be sent over RPC.
pub fn serialize_cursor_shape(cursor_shape: &CursorShape) -> proto::CursorShape {
    match cursor_shape {
        CursorShape::Bar => proto::CursorShape::CursorBar,
        CursorShape::Block => proto::CursorShape::CursorBlock,
        CursorShape::Underline => proto::CursorShape::CursorUnderscore,
        CursorShape::Hollow => proto::CursorShape::CursorHollow,
    }
}

/// Deserializes a [`CursorShape`] from the RPC representation.
pub fn deserialize_cursor_shape(cursor_shape: proto::CursorShape) -> CursorShape {
    match cursor_shape {
        proto::CursorShape::CursorBar => CursorShape::Bar,
        proto::CursorShape::CursorBlock => CursorShape::Block,
        proto::CursorShape::CursorUnderscore => CursorShape::Underline,
        proto::CursorShape::CursorHollow => CursorShape::Hollow,
    }
}

/// Serializes a list of diagnostics to be sent over RPC.
pub fn serialize_diagnostics<'a>(
    diagnostics: impl IntoIterator<Item = &'a DiagnosticEntry<Anchor>>,
) -> Vec<proto::Diagnostic> {
    diagnostics
        .into_iter()
        .map(|entry| proto::Diagnostic {
            source: entry.diagnostic.source.clone(),
            source_kind: match entry.diagnostic.source_kind {
                DiagnosticSourceKind::Pulled => proto::diagnostic::SourceKind::Pulled,
                DiagnosticSourceKind::Pushed => proto::diagnostic::SourceKind::Pushed,
                DiagnosticSourceKind::Other => proto::diagnostic::SourceKind::Other,
            } as i32,
            start: Some(serialize_anchor(&entry.range.start)),
            end: Some(serialize_anchor(&entry.range.end)),
            message: entry.diagnostic.message.clone(),
            markdown: entry.diagnostic.markdown.clone(),
            severity: match entry.diagnostic.severity {
                DiagnosticSeverity::ERROR => proto::diagnostic::Severity::Error,
                DiagnosticSeverity::WARNING => proto::diagnostic::Severity::Warning,
                DiagnosticSeverity::INFORMATION => proto::diagnostic::Severity::Information,
                DiagnosticSeverity::HINT => proto::diagnostic::Severity::Hint,
                _ => proto::diagnostic::Severity::None,
            } as i32,
            group_id: entry.diagnostic.group_id as u64,
            is_primary: entry.diagnostic.is_primary,
            underline: entry.diagnostic.underline,
            code: entry.diagnostic.code.as_ref().map(|s| s.to_string()),
            code_description: entry
                .diagnostic
                .code_description
                .as_ref()
                .map(|s| s.to_string()),
            is_disk_based: entry.diagnostic.is_disk_based,
            is_unnecessary: entry.diagnostic.is_unnecessary,
            data: entry.diagnostic.data.as_ref().map(|data| data.to_string()),
        })
        .collect()
}

/// Serializes an [`Anchor`] to be sent over RPC.
pub fn serialize_anchor(anchor: &Anchor) -> proto::Anchor {
    proto::Anchor {
        replica_id: anchor.timestamp.replica_id as u32,
        timestamp: anchor.timestamp.value,
        offset: anchor.offset as u64,
        bias: match anchor.bias {
            Bias::Left => proto::Bias::Left as i32,
            Bias::Right => proto::Bias::Right as i32,
        },
        buffer_id: anchor.buffer_id.map(Into::into),
    }
}

pub fn serialize_anchor_range(range: Range<Anchor>) -> proto::AnchorRange {
    proto::AnchorRange {
        start: Some(serialize_anchor(&range.start)),
        end: Some(serialize_anchor(&range.end)),
    }
}

/// Deserializes an [`Range<Anchor>`] from the RPC representation.
pub fn deserialize_anchor_range(range: proto::AnchorRange) -> Result<Range<Anchor>> {
    Ok(
        deserialize_anchor(range.start.context("invalid anchor")?).context("invalid anchor")?
            ..deserialize_anchor(range.end.context("invalid anchor")?).context("invalid anchor")?,
    )
}

// This behavior is currently copied in the collab database, for snapshotting channel notes
/// Deserializes an [`crate::Operation`] from the RPC representation.
pub fn deserialize_operation(message: proto::Operation) -> Result<crate::Operation> {
    Ok(
        match message.variant.context("missing operation variant")? {
            proto::operation::Variant::Edit(edit) => {
                crate::Operation::Buffer(text::Operation::Edit(deserialize_edit_operation(edit)))
            }
            proto::operation::Variant::Undo(undo) => {
                crate::Operation::Buffer(text::Operation::Undo(UndoOperation {
                    timestamp: clock::Lamport {
                        replica_id: undo.replica_id as ReplicaId,
                        value: undo.lamport_timestamp,
                    },
                    version: deserialize_version(&undo.version),
                    counts: undo
                        .counts
                        .into_iter()
                        .map(|c| {
                            (
                                clock::Lamport {
                                    replica_id: c.replica_id as ReplicaId,
                                    value: c.lamport_timestamp,
                                },
                                c.count,
                            )
                        })
                        .collect(),
                }))
            }
            proto::operation::Variant::UpdateSelections(message) => {
                let selections = message
                    .selections
                    .into_iter()
                    .filter_map(|selection| {
                        Some(Selection {
                            id: selection.id as usize,
                            start: deserialize_anchor(selection.start?.anchor?)?,
                            end: deserialize_anchor(selection.end?.anchor?)?,
                            reversed: selection.reversed,
                            goal: SelectionGoal::None,
                        })
                    })
                    .collect::<Vec<_>>();

                crate::Operation::UpdateSelections {
                    lamport_timestamp: clock::Lamport {
                        replica_id: message.replica_id as ReplicaId,
                        value: message.lamport_timestamp,
                    },
                    selections: Arc::from(selections),
                    line_mode: message.line_mode,
                    cursor_shape: deserialize_cursor_shape(
                        proto::CursorShape::from_i32(message.cursor_shape)
                            .context("Missing cursor shape")?,
                    ),
                }
            }
            proto::operation::Variant::UpdateDiagnostics(message) => {
                crate::Operation::UpdateDiagnostics {
                    lamport_timestamp: clock::Lamport {
                        replica_id: message.replica_id as ReplicaId,
                        value: message.lamport_timestamp,
                    },
                    server_id: LanguageServerId(message.server_id as usize),
                    diagnostics: deserialize_diagnostics(message.diagnostics),
                }
            }
            proto::operation::Variant::UpdateCompletionTriggers(message) => {
                crate::Operation::UpdateCompletionTriggers {
                    triggers: message.triggers,
                    lamport_timestamp: clock::Lamport {
                        replica_id: message.replica_id as ReplicaId,
                        value: message.lamport_timestamp,
                    },
                    server_id: LanguageServerId::from_proto(message.language_server_id),
                }
            }
        },
    )
}

/// Deserializes an [`EditOperation`] from the RPC representation.
pub fn deserialize_edit_operation(edit: proto::operation::Edit) -> EditOperation {
    EditOperation {
        timestamp: clock::Lamport {
            replica_id: edit.replica_id as ReplicaId,
            value: edit.lamport_timestamp,
        },
        version: deserialize_version(&edit.version),
        ranges: edit.ranges.into_iter().map(deserialize_range).collect(),
        new_text: edit.new_text.into_iter().map(Arc::from).collect(),
    }
}

/// Deserializes an entry in the undo map from the RPC representation.
pub fn deserialize_undo_map_entry(
    entry: proto::UndoMapEntry,
) -> (clock::Lamport, Vec<(clock::Lamport, u32)>) {
    (
        clock::Lamport {
            replica_id: entry.replica_id as u16,
            value: entry.local_timestamp,
        },
        entry
            .counts
            .into_iter()
            .map(|undo_count| {
                (
                    clock::Lamport {
                        replica_id: undo_count.replica_id as u16,
                        value: undo_count.lamport_timestamp,
                    },
                    undo_count.count,
                )
            })
            .collect(),
    )
}

/// Deserializes selections from the RPC representation.
pub fn deserialize_selections(selections: Vec<proto::Selection>) -> Arc<[Selection<Anchor>]> {
    selections
        .into_iter()
        .filter_map(deserialize_selection)
        .collect()
}

/// Deserializes a [`Selection`] from the RPC representation.
pub fn deserialize_selection(selection: proto::Selection) -> Option<Selection<Anchor>> {
    Some(Selection {
        id: selection.id as usize,
        start: deserialize_anchor(selection.start?.anchor?)?,
        end: deserialize_anchor(selection.end?.anchor?)?,
        reversed: selection.reversed,
        goal: SelectionGoal::None,
    })
}

/// Deserializes a list of diagnostics from the RPC representation.
pub fn deserialize_diagnostics(
    diagnostics: Vec<proto::Diagnostic>,
) -> Arc<[DiagnosticEntry<Anchor>]> {
    diagnostics
        .into_iter()
        .filter_map(|diagnostic| {
            let data = if let Some(data) = diagnostic.data {
                Some(Value::from_str(&data).ok()?)
            } else {
                None
            };
            Some(DiagnosticEntry {
                range: deserialize_anchor(diagnostic.start?)?..deserialize_anchor(diagnostic.end?)?,
                diagnostic: Diagnostic {
                    source: diagnostic.source,
                    severity: match proto::diagnostic::Severity::from_i32(diagnostic.severity)? {
                        proto::diagnostic::Severity::Error => DiagnosticSeverity::ERROR,
                        proto::diagnostic::Severity::Warning => DiagnosticSeverity::WARNING,
                        proto::diagnostic::Severity::Information => DiagnosticSeverity::INFORMATION,
                        proto::diagnostic::Severity::Hint => DiagnosticSeverity::HINT,
                        proto::diagnostic::Severity::None => return None,
                    },
                    message: diagnostic.message,
                    markdown: diagnostic.markdown,
                    group_id: diagnostic.group_id as usize,
                    code: diagnostic.code.map(lsp::NumberOrString::from_string),
                    code_description: diagnostic
                        .code_description
                        .and_then(|s| lsp::Url::parse(&s).ok()),
                    is_primary: diagnostic.is_primary,
                    is_disk_based: diagnostic.is_disk_based,
                    is_unnecessary: diagnostic.is_unnecessary,
                    underline: diagnostic.underline,
                    source_kind: match proto::diagnostic::SourceKind::from_i32(
                        diagnostic.source_kind,
                    )? {
                        proto::diagnostic::SourceKind::Pulled => DiagnosticSourceKind::Pulled,
                        proto::diagnostic::SourceKind::Pushed => DiagnosticSourceKind::Pushed,
                        proto::diagnostic::SourceKind::Other => DiagnosticSourceKind::Other,
                    },
                    data,
                },
            })
        })
        .collect()
}

/// Deserializes an [`Anchor`] from the RPC representation.
pub fn deserialize_anchor(anchor: proto::Anchor) -> Option<Anchor> {
    let buffer_id = if let Some(id) = anchor.buffer_id {
        Some(BufferId::new(id).ok()?)
    } else {
        None
    };
    Some(Anchor {
        timestamp: clock::Lamport {
            replica_id: anchor.replica_id as ReplicaId,
            value: anchor.timestamp,
        },
        offset: anchor.offset as usize,
        bias: match proto::Bias::from_i32(anchor.bias)? {
            proto::Bias::Left => Bias::Left,
            proto::Bias::Right => Bias::Right,
        },
        buffer_id,
    })
}

/// Returns a `[clock::Lamport`] timestamp for the given [`proto::Operation`].
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

/// Serializes a [`Transaction`] to be sent over RPC.
pub fn serialize_transaction(transaction: &Transaction) -> proto::Transaction {
    proto::Transaction {
        id: Some(serialize_timestamp(transaction.id)),
        edit_ids: transaction
            .edit_ids
            .iter()
            .copied()
            .map(serialize_timestamp)
            .collect(),
        start: serialize_version(&transaction.start),
    }
}

/// Deserializes a [`Transaction`] from the RPC representation.
pub fn deserialize_transaction(transaction: proto::Transaction) -> Result<Transaction> {
    Ok(Transaction {
        id: deserialize_timestamp(transaction.id.context("missing transaction id")?),
        edit_ids: transaction
            .edit_ids
            .into_iter()
            .map(deserialize_timestamp)
            .collect(),
        start: deserialize_version(&transaction.start),
    })
}

/// Serializes a [`clock::Lamport`] timestamp to be sent over RPC.
pub fn serialize_timestamp(timestamp: clock::Lamport) -> proto::LamportTimestamp {
    proto::LamportTimestamp {
        replica_id: timestamp.replica_id as u32,
        value: timestamp.value,
    }
}

/// Deserializes a [`clock::Lamport`] timestamp from the RPC representation.
pub fn deserialize_timestamp(timestamp: proto::LamportTimestamp) -> clock::Lamport {
    clock::Lamport {
        replica_id: timestamp.replica_id as ReplicaId,
        value: timestamp.value,
    }
}

/// Serializes a range of [`FullOffset`]s to be sent over RPC.
pub fn serialize_range(range: &Range<FullOffset>) -> proto::Range {
    proto::Range {
        start: range.start.0 as u64,
        end: range.end.0 as u64,
    }
}

/// Deserializes a range of [`FullOffset`]s from the RPC representation.
pub fn deserialize_range(range: proto::Range) -> Range<FullOffset> {
    FullOffset(range.start as usize)..FullOffset(range.end as usize)
}

/// Deserializes a clock version from the RPC representation.
pub fn deserialize_version(message: &[proto::VectorClockEntry]) -> clock::Global {
    let mut version = clock::Global::new();
    for entry in message {
        version.observe(clock::Lamport {
            replica_id: entry.replica_id as ReplicaId,
            value: entry.timestamp,
        });
    }
    version
}

/// Serializes a clock version to be sent over RPC.
pub fn serialize_version(version: &clock::Global) -> Vec<proto::VectorClockEntry> {
    version
        .iter()
        .map(|entry| proto::VectorClockEntry {
            replica_id: entry.replica_id as u32,
            timestamp: entry.value,
        })
        .collect()
}

pub fn serialize_lsp_edit(edit: lsp::TextEdit) -> proto::TextEdit {
    let start = point_from_lsp(edit.range.start).0;
    let end = point_from_lsp(edit.range.end).0;
    proto::TextEdit {
        new_text: edit.new_text,
        lsp_range_start: Some(proto::PointUtf16 {
            row: start.row,
            column: start.column,
        }),
        lsp_range_end: Some(proto::PointUtf16 {
            row: end.row,
            column: end.column,
        }),
    }
}

pub fn deserialize_lsp_edit(edit: proto::TextEdit) -> Option<lsp::TextEdit> {
    let start = edit.lsp_range_start?;
    let start = PointUtf16::new(start.row, start.column);
    let end = edit.lsp_range_end?;
    let end = PointUtf16::new(end.row, end.column);
    Some(lsp::TextEdit {
        range: lsp::Range {
            start: point_to_lsp(start),
            end: point_to_lsp(end),
        },
        new_text: edit.new_text,
    })
}
