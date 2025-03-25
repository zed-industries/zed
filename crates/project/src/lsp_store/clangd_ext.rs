use std::sync::Arc;

use ::serde::{Deserialize, Serialize};
use gpui::WeakEntity;
use language::CachedLspAdapter;
use lsp::LanguageServer;
use util::ResultExt as _;

use crate::LspStore;

pub const CLANGD_SERVER_NAME: &str = "clangd";

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InactiveRegionsParams {
    pub text_document: lsp::OptionalVersionedTextDocumentIdentifier,
    pub regions: Vec<lsp::Range>,
}

/// InactiveRegions is a clangd extension that marks regions of inactive code.
pub struct InactiveRegions;

impl lsp::notification::Notification for InactiveRegions {
    type Params = InactiveRegionsParams;
    const METHOD: &'static str = "textDocument/inactiveRegions";
}

pub fn register_notifications(
    lsp_store: WeakEntity<LspStore>,
    language_server: &LanguageServer,
    adapter: Arc<CachedLspAdapter>,
) {
    if language_server.name().0 != CLANGD_SERVER_NAME {
        return;
    }
    let server_id = language_server.server_id();

    // TODO: inactiveRegions support needs do add diagnostics, not replace them as `this.update_diagnostics` call below does
    if true {
        return;
    }
    language_server
        .on_notification::<InactiveRegions, _>({
            let adapter = adapter.clone();
            let this = lsp_store;

            move |params: InactiveRegionsParams, cx| {
                let adapter = adapter.clone();
                this.update(cx, |this, cx| {
                    let diagnostics = params
                        .regions
                        .into_iter()
                        .map(|range| lsp::Diagnostic {
                            range,
                            severity: Some(lsp::DiagnosticSeverity::INFORMATION),
                            source: Some(CLANGD_SERVER_NAME.to_string()),
                            message: "inactive region".to_string(),
                            tags: Some(vec![lsp::DiagnosticTag::UNNECESSARY]),
                            ..Default::default()
                        })
                        .collect();
                    let mapped_diagnostics = lsp::PublishDiagnosticsParams {
                        uri: params.text_document.uri,
                        version: params.text_document.version,
                        diagnostics,
                    };
                    this.update_diagnostics(
                        server_id,
                        mapped_diagnostics,
                        &adapter.disk_based_diagnostic_sources,
                        cx,
                    )
                    .log_err();
                })
                .ok();
            }
        })
        .detach();
}
