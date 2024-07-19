use zed::lsp::{Completion, CompletionKind, Symbol, SymbolKind};
use zed::{CodeLabel, CodeLabelSpan};
use zed_extension_api::settings::LspSettings;
use zed_extension_api::{self as zed, serde_json, Result};

pub trait CommonPythonLsp {
    fn get_language_server_id(&self) -> &'static str;

    fn label_for_completion(&self, completion: Completion) -> Option<CodeLabel> {
        let label = &completion.label;
        let highlight_name = match completion.kind? {
            CompletionKind::Class => "type",
            CompletionKind::Constant => "constant",
            CompletionKind::Method => "function.method",
            CompletionKind::Function => "function",
            _ => return None,
        };

        let name_span = CodeLabelSpan::literal(label, Some(highlight_name.to_string()));

        Some(CodeLabel {
            code: label.clone(),
            spans: if let Some(detail) = completion.detail {
                vec![
                    name_span,
                    CodeLabelSpan::literal(" ", None),
                    CodeLabelSpan::literal(detail, None),
                ]
            } else {
                vec![name_span]
            },
            filter_range: (0..label.len()).into(),
        })
    }

    fn label_for_symbol(&self, symbol: Symbol) -> Option<CodeLabel> {
        let name = &symbol.name;

        return match symbol.kind {
            SymbolKind::Method | SymbolKind::Function => {
                let def = "def ";
                let code = format!("{def}{name}():\n");
                let filter_range = 0..name.len();
                let display_range = def.len()..def.len() + name.len();

                Some(CodeLabel {
                    code,
                    spans: vec![CodeLabelSpan::code_range(display_range)],
                    filter_range: filter_range.into(),
                })
            }
            SymbolKind::Class => {
                let class = "class ";
                let code = format!("{class}{name}");
                let filter_range = 0..name.len();
                let display_range = class.len()..class.len() + name.len();

                Some(CodeLabel {
                    code,
                    spans: vec![CodeLabelSpan::code_range(display_range)],
                    filter_range: filter_range.into(),
                })
            }
            SymbolKind::Constant => {
                let code = name.to_uppercase().to_string();
                let filter_range = 0..name.len();
                let display_range = 0..name.len();

                Some(CodeLabel {
                    code,
                    spans: vec![CodeLabelSpan::code_range(display_range)],
                    filter_range: filter_range.into(),
                })
            }
            _ => None,
        };
    }

    fn workspace_configuration(
        &mut self,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let settings = LspSettings::for_worktree(self.get_language_server_id(), worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone())
            .unwrap_or_default();

        Ok(Some(serde_json::json!({
            self.get_language_server_id(): settings
        })))
    }
}
