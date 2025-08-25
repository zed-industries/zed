use lsp_types::{
    ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind,
    TextDocumentSyncSaveOptions,
};

use super::DynamicCapabilities;

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
                    return None;
                }
                let mut has_incremental = false;
                for data in id_to_sync_kind_map.values() {
                    let sync_kind = match data.sync_kind {
                        0 => Some(TextDocumentSyncKind::NONE),
                        1 => Some(TextDocumentSyncKind::FULL),
                        2 => Some(TextDocumentSyncKind::INCREMENTAL),
                        _ => None,
                    };
                    if sync_kind == Some(TextDocumentSyncKind::FULL) {
                        return Some(TextDocumentSyncKind::FULL);
                    }
                    if sync_kind == Some(TextDocumentSyncKind::INCREMENTAL) {
                        has_incremental = true;
                    }
                }
                if has_incremental {
                    Some(TextDocumentSyncKind::INCREMENTAL)
                } else {
                    None
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
                            .any(|data| data.include_text.unwrap_or(false)),
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
