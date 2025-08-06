use std::sync::Arc;

use ::serde::{Deserialize, Serialize};
use gpui::WeakEntity;
use language::{CachedLspAdapter, Diagnostic, DiagnosticSourceKind};
use lsp::{LanguageServer, LanguageServerName};
use util::ResultExt as _;

use crate::LspStore;

pub const CLANGD_SERVER_NAME: LanguageServerName = LanguageServerName::new_static("clangd");
const INACTIVE_REGION_MESSAGE: &str = "inactive region";
const INACTIVE_DIAGNOSTIC_SEVERITY: lsp::DiagnosticSeverity = lsp::DiagnosticSeverity::INFORMATION;

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

pub fn is_inactive_region(diag: &Diagnostic) -> bool {
    diag.is_unnecessary
        && diag.severity == INACTIVE_DIAGNOSTIC_SEVERITY
        && diag.message == INACTIVE_REGION_MESSAGE
        && diag
            .source
            .as_ref()
            .is_some_and(|v| v == &CLANGD_SERVER_NAME.0)
}

pub fn is_lsp_inactive_region(diag: &lsp::Diagnostic) -> bool {
    diag.severity == Some(INACTIVE_DIAGNOSTIC_SEVERITY)
        && diag.message == INACTIVE_REGION_MESSAGE
        && diag
            .source
            .as_ref()
            .is_some_and(|v| v == &CLANGD_SERVER_NAME.0)
}

pub fn register_notifications(
    lsp_store: WeakEntity<LspStore>,
    language_server: &LanguageServer,
    adapter: Arc<CachedLspAdapter>,
) {
    if language_server.name() != CLANGD_SERVER_NAME {
        return;
    }
    let server_id = language_server.server_id();

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
                            severity: Some(INACTIVE_DIAGNOSTIC_SEVERITY),
                            source: Some(CLANGD_SERVER_NAME.to_string()),
                            message: INACTIVE_REGION_MESSAGE.to_string(),
                            tags: Some(vec![lsp::DiagnosticTag::UNNECESSARY]),
                            ..lsp::Diagnostic::default()
                        })
                        .collect();
                    let mapped_diagnostics = lsp::PublishDiagnosticsParams {
                        uri: params.text_document.uri,
                        version: params.text_document.version,
                        diagnostics,
                    };
                    this.merge_diagnostics(
                        server_id,
                        mapped_diagnostics,
                        None,
                        DiagnosticSourceKind::Pushed,
                        &adapter.disk_based_diagnostic_sources,
                        |_, diag, _| !is_inactive_region(diag),
                        cx,
                    )
                    .log_err();
                })
                .ok();
            }
        })
        .detach();
}
