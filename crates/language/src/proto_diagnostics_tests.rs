use crate::{
    Diagnostic, DiagnosticMessage,
    diagnostic_set::DiagnosticEntry,
    proto::{deserialize_diagnostics, serialize_diagnostics},
};
use pretty_assertions::assert_eq;
use rpc::proto;
use text::{Anchor, BufferId};

#[test]
fn test_markup_diagnostics_round_trip() {
    let untrimmed_entry = markup_entry(lsp::MarkupKind::Markdown, "\n**message**\n");
    let trimmed_entry = markup_entry(lsp::MarkupKind::Markdown, "**message**");
    let plain_text_entry = markup_entry(lsp::MarkupKind::PlainText, "  plain  ");

    let serialized = serialize_diagnostics([&untrimmed_entry, &trimmed_entry, &plain_text_entry]);
    assert_eq!(serialized[0].message, "**message**");
    assert_eq!(serialized[0].markdown, None);
    assert_eq!(
        serialized[0].markup_message_kind,
        Some(proto::MarkupKind::Markdown as i32)
    );
    assert_eq!(
        serialized[0].untrimmed_markup_message,
        Some("\n**message**\n".to_string())
    );
    assert_eq!(serialized[1].message, "**message**");
    assert_eq!(serialized[1].untrimmed_markup_message, None);
    assert_eq!(
        serialized[1].markup_message_kind,
        Some(proto::MarkupKind::Markdown as i32)
    );
    assert_eq!(serialized[2].message, "plain");
    assert_eq!(serialized[2].markdown, None);
    assert_eq!(
        serialized[2].markup_message_kind,
        Some(proto::MarkupKind::PlainText as i32)
    );
    assert_eq!(
        serialized[2].untrimmed_markup_message,
        Some("  plain  ".to_string())
    );

    let deserialized = deserialize_diagnostics(serialized);
    assert_eq!(
        deserialized.as_ref(),
        [untrimmed_entry, trimmed_entry, plain_text_entry]
    );
}

#[test]
fn test_plain_and_adapter_markdown_diagnostics_round_trip() {
    let plain_entry = entry(DiagnosticMessage::plain("plain message"));
    let adapter_entry = entry(DiagnosticMessage::plain_with_adapter_markdown(
        "message",
        Some("**message**".into()),
    ));

    let serialized = serialize_diagnostics([&plain_entry, &adapter_entry]);
    assert_eq!(serialized[0].message, "plain message");
    assert_eq!(serialized[0].markdown, None);
    assert_eq!(serialized[0].markup_message_kind, None);
    assert_eq!(serialized[0].untrimmed_markup_message, None);
    assert_eq!(serialized[1].message, "message");
    assert_eq!(serialized[1].markdown, Some("**message**".to_string()));
    assert_eq!(serialized[1].markup_message_kind, None);
    assert_eq!(serialized[1].untrimmed_markup_message, None);

    let deserialized = deserialize_diagnostics(serialized);
    assert_eq!(deserialized.as_ref(), [plain_entry, adapter_entry]);
}

#[test]
fn test_unknown_markup_kind_degrades_to_plain_message() {
    let entry = markup_entry(lsp::MarkupKind::Markdown, "**message**");
    let mut serialized = serialize_diagnostics([&entry]);
    serialized[0].markup_message_kind = Some(42);

    let deserialized = deserialize_diagnostics(serialized);
    assert_eq!(deserialized.len(), 1);
    assert_eq!(
        deserialized[0].diagnostic.message,
        DiagnosticMessage::plain("**message**")
    );
}

fn markup_entry(kind: lsp::MarkupKind, value: &str) -> DiagnosticEntry<Anchor> {
    entry(DiagnosticMessage::from_lsp_markup(&lsp::MarkupContent {
        kind,
        value: value.to_string(),
    }))
}

fn entry(message: DiagnosticMessage) -> DiagnosticEntry<Anchor> {
    let buffer_id = BufferId::new(1).unwrap();
    DiagnosticEntry::new(
        Anchor::min_max_range_for_buffer(buffer_id),
        Diagnostic {
            message,
            ..Diagnostic::default()
        },
    )
}
