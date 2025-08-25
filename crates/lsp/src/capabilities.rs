use super::DynamicCapabilities;
use lsp_types::{
    ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind,
    TextDocumentSyncSaveOptions,
};

pub mod cap {
    pub struct DidChangeTextDocument;
    pub struct DidSaveTextDocument;
}

pub trait EffectiveCapability {
    type Value;
    fn compute(static_caps: &ServerCapabilities, dynamic_caps: &DynamicCapabilities)
    -> Self::Value;
}

impl EffectiveCapability for cap::DidChangeTextDocument {
    type Value = Option<TextDocumentSyncKind>;

    fn compute(
        static_caps: &ServerCapabilities,
        dynamic_caps: &DynamicCapabilities,
    ) -> Self::Value {
        dynamic_caps
            .text_document_sync_did_change
            .as_ref()
            .and_then(|id_to_sync_kind_map| {
                if id_to_sync_kind_map.is_empty() {
                    None
                } else {
                    let mut best: Option<TextDocumentSyncKind> = None;
                    for kind in id_to_sync_kind_map.values() {
                        best = Some(match (best, kind) {
                            (None, kind) => *kind,
                            (
                                Some(TextDocumentSyncKind::FULL),
                                &TextDocumentSyncKind::INCREMENTAL,
                            ) => TextDocumentSyncKind::INCREMENTAL,
                            (Some(kind), _) => kind,
                        });
                    }
                    best
                }
            })
            .or_else(|| {
                static_caps
                    .text_document_sync
                    .as_ref()
                    .and_then(|sync| match sync {
                        TextDocumentSyncCapability::Kind(kind) => Some(*kind),
                        TextDocumentSyncCapability::Options(opts) => opts.change,
                    })
            })
    }
}

impl EffectiveCapability for cap::DidSaveTextDocument {
    type Value = Option<bool>;

    fn compute(
        static_caps: &ServerCapabilities,
        dynamic_caps: &DynamicCapabilities,
    ) -> Self::Value {
        dynamic_caps
            .text_document_sync_did_save
            .as_ref()
            .and_then(|id_to_save_options_map| {
                if id_to_save_options_map.is_empty() {
                    None
                } else {
                    Some(
                        id_to_save_options_map
                            .values()
                            .any(|opts| opts.include_text.unwrap_or(false)),
                    )
                }
            })
            .or_else(|| match static_caps.text_document_sync.as_ref()? {
                TextDocumentSyncCapability::Options(opts) => match opts.save.as_ref()? {
                    TextDocumentSyncSaveOptions::Supported(true) => Some(false),
                    TextDocumentSyncSaveOptions::Supported(false) => None,
                    TextDocumentSyncSaveOptions::SaveOptions(save_opts) => {
                        Some(save_opts.include_text.unwrap_or(false))
                    }
                },
                TextDocumentSyncCapability::Kind(_) => None,
            })
    }
}
