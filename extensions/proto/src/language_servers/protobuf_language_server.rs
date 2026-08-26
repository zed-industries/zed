use zed_extension_api::{self as zed, Result, settings::LspSettings};

pub(crate) struct ProtobufLanguageServer {
    cached_binary_path: Option<String>,
}

impl ProtobufLanguageServer {
    pub(crate) const SERVER_NAME: &str = "protobuf-language-server";

    pub(crate) fn new() -> Self {
        ProtobufLanguageServer {
            cached_binary_path: None,
        }
    }

    pub(crate) fn language_server_binary(
        &mut self,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let binary_settings = LspSettings::for_worktree(Self::SERVER_NAME, worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.binary);

        let args = binary_settings
            .as_ref()
            .and_then(|binary_settings| binary_settings.arguments.clone())
            .unwrap_or_else(|| vec!["-logs".into(), "".into()]);

        if let Some(path) = binary_settings.and_then(|binary_settings| binary_settings.path) {
            Ok(zed::Command {
                command: path,
                args,
                env: Default::default(),
            })
        } else if let Some(path) = self.cached_binary_path.clone() {
            Ok(zed::Command {
                command: path,
                args,
                env: Default::default(),
            })
        } else if let Some(path) = worktree.which(Self::SERVER_NAME) {
            self.cached_binary_path = Some(path.clone());
            Ok(zed::Command {
                command: path,
                args,
                env: Default::default(),
            })
        } else {
            Err(format!("{} not found in PATH", Self::SERVER_NAME))
        }
    }
}
