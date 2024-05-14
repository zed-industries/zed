use zed::{
    lsp::{Completion, CompletionKind, Symbol, SymbolKind},
    CodeLabel, CodeLabelSpan,
};
use zed_extension_api::{self as zed, Result};

pub struct RubyLsp {}

impl RubyLsp {
    pub const LANGUAGE_SERVER_ID: &'static str = "ruby-lsp";

    pub fn new() -> Self {
        Self {}
    }

    pub fn server_script_path(&mut self, worktree: &zed::Worktree) -> Result<String> {
        let path = worktree.which("ruby-lsp").ok_or_else(|| {
            "ruby-lsp must be installed manually. Install it with `gem install ruby-lsp`."
                .to_string()
        })?;

        Ok(path)
    }

    pub fn label_for_completion(&self, completion: Completion) -> Option<CodeLabel> {
        let highlight_name = match completion.kind? {
            CompletionKind::Class | CompletionKind::Module => "type",
            CompletionKind::Constant => "constant",
            CompletionKind::Method => "function.method",
            CompletionKind::Reference => "function.method",
            CompletionKind::Keyword => "keyword",
            _ => return None,
        };

        let len = completion.label.len();
        let name_span = CodeLabelSpan::literal(completion.label, Some(highlight_name.to_string()));

        Some(CodeLabel {
            code: Default::default(),
            spans: vec![name_span],
            filter_range: (0..len).into(),
        })
    }

    pub fn label_for_symbol(&self, symbol: Symbol) -> Option<CodeLabel> {
        let name = &symbol.name;

        return match symbol.kind {
            SymbolKind::Method => {
                let code = format!("def {name}; end");
                let filter_range = 0..name.len();
                let display_range = 4..4 + name.len();

                Some(CodeLabel {
                    code,
                    spans: vec![CodeLabelSpan::code_range(display_range)],
                    filter_range: filter_range.into(),
                })
            }
            SymbolKind::Class | SymbolKind::Module => {
                let code = format!("class {name}; end");
                let filter_range = 0..name.len();
                let display_range = 6..6 + name.len();

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
