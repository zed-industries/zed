use crate::language_servers::commons::CommonPythonLsp;
use zed::lsp::{Completion, Symbol};
use zed::CodeLabel;
use zed_extension_api::{self as zed, serde_json, Result};

pub struct BasedPyright;

impl CommonPythonLsp for BasedPyright {
    fn get_language_server_id(&self) -> &'static str {
        Self::LANGUAGE_SERVER_ID
    }
}

impl BasedPyright {
    pub const LANGUAGE_SERVER_ID: &'static str = "basedpyright";

    pub fn new() -> Self {
        Self {}
    }

    pub fn server_script_path(&mut self, worktree: &zed::Worktree) -> Result<String> {
        let path = worktree.which("basedpyright-langserver").ok_or_else(|| {
            "basedpyright must be installed manually. Install it with `pip install basedpyright` or specify the 'binary' path to it via local settings.".to_string()
        })?;

        Ok(path)
    }

    pub fn workspace_configuration(
        &mut self,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        CommonPythonLsp::workspace_configuration(self, worktree)
    }

    pub fn label_for_completion(&self, completion: Completion) -> Option<CodeLabel> {
        CommonPythonLsp::label_for_completion(self, completion)
    }

    pub fn label_for_symbol(&self, symbol: Symbol) -> Option<CodeLabel> {
        CommonPythonLsp::label_for_symbol(self, symbol)
    }
}
