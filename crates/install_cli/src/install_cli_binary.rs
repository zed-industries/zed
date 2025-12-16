use super::register_zed_scheme;
use anyhow::{Context as _, Result};
use gpui::{AppContext as _, AsyncApp, Context, PromptLevel, Window, actions};
use release_channel::ReleaseChannel;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use util::ResultExt;
use workspace::notifications::{DetachAndPromptErr, NotificationId};
use workspace::{Toast, Workspace};

actions!(
    cli,
    [
        /// Installs the Zed CLI tool to the system PATH.
        InstallCliBinary,
    ]
);

async fn add_path_to_zshrc() -> Result<bool> {
    let home_dir = std::env::var("HOME").context("Failed to get HOME environment variable")?;
    let zshrc_path = PathBuf::from(&home_dir).join(".zshrc");
    let path_export = r#"export PATH="$HOME/.local/bin:$PATH""#;

    // Read existing .zshrc content if it exists
    let existing_content = smol::fs::read_to_string(&zshrc_path)
        .await
        .unwrap_or_default();

    // Check if the PATH export already exists
    if existing_content.contains(path_export) || existing_content.contains("$HOME/.local/bin") {
        return Ok(false);
    }

    // Append the PATH export to .zshrc
    let new_content = if existing_content.is_empty() {
        format!("{}\n", path_export)
    } else if existing_content.ends_with('\n') {
        format!("{}{}\n", existing_content, path_export)
    } else {
        format!("{}\n{}\n", existing_content, path_export)
    };

    smol::fs::write(&zshrc_path, new_content)
        .await
        .context("Failed to write to ~/.zshrc")?;

    Ok(true)
}

async fn install_script(cx: &AsyncApp) -> Result<PathBuf> {
    let cli_path = cx.update(|cx| cx.path_for_auxiliary_executable("cli"))??;
    let home_dir = std::env::var("HOME").context("Failed to get HOME environment variable")?;
    let link_path = Path::new(&home_dir).join(".local/bin/zed");
    let bin_dir_path = link_path.parent().unwrap();

    // Ensure ~/.local/bin directory exists
    smol::fs::create_dir_all(&bin_dir_path)
        .await
        .context("Failed to create ~/.local/bin directory")?;

    // Don't re-create symlink if it points to the same CLI binary.
    if smol::fs::read_link(&link_path).await.ok().as_ref() == Some(&cli_path) {
        return Ok(link_path);
    }

    // If the symlink is not there or is outdated, first try replacing it.
    smol::fs::remove_file(&link_path).await.log_err();
    smol::fs::unix::symlink(&cli_path, &link_path)
        .await
        .context("Failed to create symlink to ~/.local/bin/zed")?;

    Ok(link_path)
}

pub fn install_cli_binary(window: &mut Window, cx: &mut Context<Workspace>) {
    const LINUX_PROMPT_DETAIL: &str = "If you installed Zed from our official release add ~/.local/bin to your PATH.\n\nIf you installed Zed from a different source like your package manager, then you may need to create an alias/symlink manually.\n\nDepending on your package manager, the CLI might be named zeditor, zedit, zed-editor or something else.";

    cx.spawn_in(window, async move |workspace, cx| {
        if cfg!(any(target_os = "linux", target_os = "freebsd")) {
            let prompt = cx.prompt(
                PromptLevel::Warning,
                "CLI should already be installed",
                Some(LINUX_PROMPT_DETAIL),
                &["Ok"],
            );
            cx.background_spawn(prompt).detach();
            return Ok(());
        }
        let path = install_script(cx.deref())
            .await
            .context("error creating CLI symlink")?;

        // Add PATH to ~/.zshrc
        let added_to_zshrc = add_path_to_zshrc().await.unwrap_or(false);

        workspace.update_in(cx, |workspace, _, cx| {
            struct InstalledZedCli;

            let shell_note = if added_to_zshrc {
                "\n\nAdded ~/.local/bin to PATH in ~/.zshrc. If you use a different shell (bash, fish, etc.), please add the following to your shell config:\nexport PATH=\"$HOME/.local/bin:$PATH\""
            } else {
                "\n\nIf you use a shell other than zsh, please add the following to your shell config:\nexport PATH=\"$HOME/.local/bin:$PATH\""
            };

            workspace.show_toast(
                Toast::new(
                    NotificationId::unique::<InstalledZedCli>(),
                    format!(
                        "Installed `zed` to {}. You can launch {} from your terminal.{}",
                        path.to_string_lossy(),
                        ReleaseChannel::global(cx).display_name(),
                        shell_note
                    ),
                ),
                cx,
            )
        })?;
        register_zed_scheme(cx).await.log_err();
        Ok(())
    })
    .detach_and_prompt_err("Error installing zed cli", window, cx, |_, _, _| None);
}
