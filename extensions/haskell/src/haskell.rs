use zed::lsp::{Symbol, SymbolKind};
use zed::{CodeLabel, CodeLabelSpan};
use zed_extension_api::{self as zed, Result};

struct HaskellExtension;

impl zed::Extension for HaskellExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let path = worktree
            .which("haskell-language-server-wrapper")
            .ok_or_else(|| "hls must be installed via ghcup".to_string())?;

        Ok(zed::Command {
            command: path,
            args: vec!["lsp".to_string()],
            env: worktree.shell_env(),
        })
    }

    fn label_for_symbol(
        &self,
        _language_server_id: &zed::LanguageServerId,
        symbol: Symbol,
    ) -> Option<CodeLabel> {
        let name = &symbol.name;

        let (code, display_range, filter_range) = match symbol.kind {
            SymbolKind::Struct => {
                let data_decl = "data ";
                let code = format!("{data_decl}{name} = A");
                let display_range = 0..data_decl.len() + name.len();
                let filter_range = data_decl.len()..display_range.end;
                (code, display_range, filter_range)
            }
            SymbolKind::Constructor => {
                let data_decl = "data A = ";
                let code = format!("{data_decl}{name}");
                let display_range = data_decl.len()..data_decl.len() + name.len();
                let filter_range = 0..name.len();
                (code, display_range, filter_range)
            }
            SymbolKind::Variable => {
                let code = format!("{name} :: T");
                let display_range = 0..name.len();
                let filter_range = 0..name.len();
                (code, display_range, filter_range)
            }
            _ => return None,
        };

        Some(CodeLabel {
            spans: vec![CodeLabelSpan::code_range(display_range)],
            filter_range: filter_range.into(),
            code,
        })
    }
}

zed::register_extension!(HaskellExtension);
