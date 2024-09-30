use zed::lsp::{Completion, CompletionKind, Symbol, SymbolKind};
use zed::{CodeLabel, CodeLabelSpan};
use zed_extension_api::settings::LspSettings;
use zed_extension_api::{self as zed, LanguageServerId, Result};

pub struct SolargraphBinary {
    pub path: String,
    pub args: Option<Vec<String>>,
}

pub struct Solargraph {}

impl Solargraph {
    pub const LANGUAGE_SERVER_ID: &'static str = "solargraph";

    pub fn new() -> Self {
        Self {}
    }

    pub fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let binary = self.language_server_binary(language_server_id, worktree)?;

        Ok(zed::Command {
            command: binary.path,
            args: binary.args.unwrap_or_else(|| vec!["stdio".to_string()]),
            env: worktree.shell_env(),
        })
    }

    fn language_server_binary(
        &self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<SolargraphBinary> {
        let binary_settings = LspSettings::for_worktree("solargraph", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.binary);
        let binary_args = binary_settings
            .as_ref()
            .and_then(|binary_settings| binary_settings.arguments.clone());

        if let Some(path) = binary_settings.and_then(|binary_settings| binary_settings.path) {
            return Ok(SolargraphBinary {
                path,
                args: binary_args,
            });
        }

        if let Some(path) = worktree.which("solargraph") {
            return Ok(SolargraphBinary {
                path,
                args: binary_args,
            });
        }

        Err("solargraph must be installed manually".to_string())
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
