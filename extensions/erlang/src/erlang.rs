use zed_extension_api::{self as zed, Result};

struct ErlangExtension;

impl zed::Extension for ErlangExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        config: zed::LanguageServerConfig,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        match config.name.as_str() {
            "erlang-ls" => erlang_ls_server_command(worktree),
            "elp" => elp_server_command(worktree),
            _ => Err(format!("Unsupported language server: {}", config.name)),
        }
    }
}

fn erlang_ls_server_command(worktree: &zed::Worktree) -> Result<zed::Command> {
    let path = worktree
        .which("erlang_ls")
        .ok_or_else(|| "erlang_ls must be installed and available on your $PATH".to_string())?;

    Ok(zed::Command {
        command: path,
        args: Vec::new(),
        env: Default::default(),
    })
}

fn elp_server_command(worktree: &zed::Worktree) -> Result<zed::Command> {
    let path = worktree
        .which("elp")
        .ok_or_else(|| "elp must be installed and available on your $PATH".to_string())?;

    Ok(zed::Command {
        command: path,
        args: vec!["server".to_string()],
        env: Default::default(),
    })
}

zed::register_extension!(ErlangExtension);
