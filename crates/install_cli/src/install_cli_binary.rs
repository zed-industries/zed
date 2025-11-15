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

async fn is_user_admin() -> bool {
    let output = smol::process::Command::new("groups").output().await.ok();
    if let Some(output) = output {
        if output.status.success() {
            let groups = String::from_utf8_lossy(&output.stdout);
            return groups.split_whitespace().any(|g| g == "admin");
        }
    }
    false
}

async fn install_to_local_bin(cx: &AsyncApp) -> Result<PathBuf> {
    let cli_path = cx.update(|cx| cx.path_for_auxiliary_executable("cli"))??;
    let home_dir = std::env::var("HOME").context("Failed to get HOME environment variable")?;
    let local_bin = PathBuf::from(home_dir).join(".local/bin");
    let link_path = local_bin.join("zed");

    // Ensure ~/.local/bin directory exists
    smol::fs::create_dir_all(&local_bin)
        .await
        .context("Failed to create ~/.local/bin directory")?;

    // Don't re-create symlink if it points to the same CLI binary.
    if smol::fs::read_link(&link_path).await.ok().as_ref() == Some(&cli_path) {
        return Ok(link_path);
    }

    // Remove old symlink if exists
    smol::fs::remove_file(&link_path).await.log_err();

    // Create new symlink
    smol::fs::unix::symlink(&cli_path, &link_path)
        .await
        .context("Failed to create symlink to ~/.local/bin/zed")?;

    Ok(link_path)
}

async fn install_to_system_bin(cx: &AsyncApp) -> Result<PathBuf> {
    let cli_path = cx.update(|cx| cx.path_for_auxiliary_executable("cli"))??;
    let link_path = Path::new("/usr/local/bin/zed");
    let bin_dir_path = link_path.parent().unwrap();

    // Don't re-create symlink if it points to the same CLI binary.
    if smol::fs::read_link(link_path).await.ok().as_ref() == Some(&cli_path) {
        return Ok(link_path.into());
    }

    // If the symlink is not there or is outdated, first try replacing it
    // without escalating.
    smol::fs::remove_file(link_path).await.log_err();
    if smol::fs::unix::symlink(&cli_path, link_path)
        .await
        .log_err()
        .is_some()
    {
        return Ok(link_path.into());
    }

    // The symlink could not be created, so use osascript with admin privileges
    // to create it.
    let status = smol::process::Command::new("/usr/bin/osascript")
        .args([
            "-e",
            &format!(
                "do shell script \" \
                    mkdir -p \'{}\' && \
                    ln -sf \'{}\' \'{}\' \
                \" with administrator privileges",
                bin_dir_path.to_string_lossy(),
                cli_path.to_string_lossy(),
                link_path.to_string_lossy(),
            ),
        ])
        .stdout(smol::process::Stdio::inherit())
        .stderr(smol::process::Stdio::inherit())
        .output()
        .await?
        .status;

    anyhow::ensure!(status.success(), "error running osascript");
    Ok(link_path.into())
}

async fn install_script(cx: &AsyncApp) -> Result<PathBuf> {
    let has_admin = is_user_admin().await;

    if !has_admin {
        return install_to_local_bin(cx).await;
    }

    match install_to_system_bin(cx).await {
        Ok(path) => Ok(path),
        Err(err) => {
            eprintln!(
                "Failed to install to system bin: {}. Falling back to local bin.",
                err
            );
            install_to_local_bin(cx).await
        }
    }
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

        workspace.update_in(cx, |workspace, _, cx| {
            struct InstalledZedCli;

            let message = if path.starts_with("/usr/local/bin") {
                format!(
                    "Installed `zed` to {}. You can launch {} from your terminal.",
                    path.to_string_lossy(),
                    ReleaseChannel::global(cx).display_name()
                )
            } else {
                format!(
                    "Installed `zed` to {}. Make sure {} is in your PATH to launch from your terminal.",
                    path.to_string_lossy(),
                    path.parent().unwrap().to_string_lossy()
                )
            };


            workspace.show_toast(
                Toast::new(
                    NotificationId::unique::<InstalledZedCli>(),
                    message,
                ),
                cx,
            )
        })?;
        register_zed_scheme(cx).await.log_err();
        Ok(())
    })
    .detach_and_prompt_err("Error installing zed cli", window, cx, |_, _, _| None);
}
