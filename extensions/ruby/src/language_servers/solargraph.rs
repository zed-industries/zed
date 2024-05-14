use zed::lsp::{Completion, CompletionKind, Symbol, SymbolKind};
use zed::{CodeLabel, CodeLabelSpan};
use zed_extension_api::{self as zed, Result};

pub struct Solargraph {}

impl Solargraph {
    pub const LANGUAGE_SERVER_ID: &'static str = "solargraph";

    pub fn new() -> Self {
        Self {}
    }

    pub fn server_script_path(&mut self, worktree: &zed::Worktree) -> Result<String> {
        let path = worktree
            .which("solargraph")
            .ok_or_else(|| "solargraph must be installed manually".to_string())?;

        Ok(path)
    }

    pub fn label_for_completion(&self, completion: Completion) -> Option<CodeLabel> {
        let highlight_name = match completion.kind? {
            CompletionKind::Class | CompletionKind::Module => "type",
            CompletionKind::Constant => "constant",
            CompletionKind::Method => "function.method",
            CompletionKind::Keyword => {
                if completion.label.starts_with(':') {
                    "string.special.symbol"
                } else {
                    "keyword"
                }
            }
            CompletionKind::Variable => {
                if completion.label.starts_with('@') {
                    "property"
                } else {
                    return None;
                }
            }
            _ => return None,
        };

        let len = completion.label.len();
        let name_span = CodeLabelSpan::literal(completion.label, Some(highlight_name.to_string()));

        Some(CodeLabel {
            code: Default::default(),
            spans: if let Some(detail) = completion.detail {
                vec![
                    name_span,
                    CodeLabelSpan::literal(" ", None),
                    CodeLabelSpan::literal(detail, None),
                ]
            } else {
                vec![name_span]
            },
            filter_range: (0..len).into(),
        })
    }

    pub fn label_for_symbol(&self, symbol: Symbol) -> Option<CodeLabel> {
        let name = &symbol.name;

        return match symbol.kind {
            SymbolKind::Method => {
                let mut parts = name.split('#');
                let container_name = parts.next()?;
                let method_name = parts.next()?;

                if parts.next().is_some() {
                    return None;
                }

                let filter_range = 0..name.len();

                let spans = vec![
                    CodeLabelSpan::literal(container_name, Some("type".to_string())),
                    CodeLabelSpan::literal("#", None),
                    CodeLabelSpan::literal(method_name, Some("function.method".to_string())),
                ];

                Some(CodeLabel {
                    code: name.to_string(),
                    spans,
                    filter_range: filter_range.into(),
                })
            }
            SymbolKind::Class | SymbolKind::Module => {
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
}
