use zed_extension_api::{
    self as zed,
    lsp::{Completion, CompletionKind},
    CodeLabel, CodeLabelSpan, LanguageServerId, Result,
};

struct MoveExtension {
    cached_binary_path: Option<String>,
}

impl MoveExtension {
    fn language_server_command(
        &mut self,
        _server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        worktree
            .which("move-analyzer")
            .ok_or("move-analyzer not found in PATH, falling back to bundled binary".to_string())
    }

    fn label_for_completion(
        &self,
        _server_id: &LanguageServerId,
        completion: Completion,
    ) -> Option<CodeLabel> {
        match completion.kind? {
            CompletionKind::Function => {
                let func = "func ";
                let mut return_type = String::new();

                if let Some(detail) = completion.detail {
                    if !detail.is_empty() {
                        return_type = format!(" -> {detail}");
                    }
                }

                let before_braces = format!("{func}{}{return_type}", completion.label);
                let code = format!("{before_braces} {{}}");

                Some(CodeLabel {
                    code,
                    spans: vec![CodeLabelSpan::code_range(func.len()..before_braces.len())],
                    filter_range: (0..completion.label.find('(')?).into(),
                })
            }
            CompletionKind::Variable => {
                let var = "var ";
                let code = format!("{var}{}: {}", completion.label, completion.detail?);

                Some(CodeLabel {
                    spans: vec![CodeLabelSpan::code_range(var.len()..code.len())],
                    code,
                    filter_range: (0..completion.label.len()).into(),
                })
            }
            CompletionKind::Value => {
                let mut r#type = String::new();

                if let Some(detail) = completion.detail {
                    if !detail.is_empty() {
                        r#type = format!(": {detail}");
                    }
                }

                let var = format!("var variable{type} = ");
                let code = format!("{var}{}", completion.label);

                Some(CodeLabel {
                    spans: vec![CodeLabelSpan::code_range(var.len()..code.len())],
                    code,
                    filter_range: (0..completion.label.len()).into(),
                })
            }
            CompletionKind::Class
            | CompletionKind::Interface
            | CompletionKind::Module
            | CompletionKind::Enum
            | CompletionKind::Keyword
            | CompletionKind::Struct => {
                let highlight_name = match completion.kind? {
                    CompletionKind::Class
                    | CompletionKind::Interface
                    | CompletionKind::Enum
                    | CompletionKind::Struct => Some("type".to_string()),
                    CompletionKind::Keyword => Some("keyword".to_string()),
                    _ => None,
                };

                Some(CodeLabel {
                    code: Default::default(),
                    filter_range: (0..completion.label.len()).into(),
                    spans: vec![CodeLabelSpan::literal(completion.label, highlight_name)],
                })
            }
            CompletionKind::EnumMember => {
                let start = "enum Enum { case ";
                let code = format!("{start}{} }}", completion.label);

                Some(CodeLabel {
                    code,
                    spans: vec![CodeLabelSpan::code_range(
                        start.len()..start.len() + completion.label.len(),
                    )],
                    filter_range: (0..completion.label.find('(').unwrap_or(completion.label.len()))
                        .into(),
                })
            }
            CompletionKind::TypeParameter => {
                let typealias = "typealias ";
                let code = format!("{typealias}{} = {}", completion.label, completion.detail?);

                Some(CodeLabel {
                    spans: vec![CodeLabelSpan::code_range(typealias.len()..code.len())],
                    code,
                    filter_range: (0..completion.label.len()).into(),
                })
            }
            _ => None,
        }
    }
}

impl zed::Extension for MoveExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        Ok(zed::Command {
            command: self.language_server_command(server_id, worktree)?,
            args: vec![],
            env: Default::default(),
        })
    }

    fn label_for_completion(
        &self,
        server_id: &LanguageServerId,
        completion: Completion,
    ) -> Option<CodeLabel> {
        self.label_for_completion(server_id, completion)
    }
}

zed::register_extension!(MoveExtension);
