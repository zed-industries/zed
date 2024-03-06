use anyhow::{anyhow, Result};
use gpui::{actions, AsyncAppContext};
use std::path::{Path, PathBuf};
use util::ResultExt;

actions!(cli, [Install, RegisterZedScheme]);

pub async fn install_cli(cx: &AsyncAppContext) -> Result<PathBuf> {
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
    // todo("windows")
    #[cfg(not(windows))]
    {
        if smol::fs::unix::symlink(&cli_path, link_path)
            .await
            .log_err()
            .is_some()
        {
            return Ok(link_path.into());
        }
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
    if status.success() {
        Ok(link_path.into())
    } else {
        Err(anyhow!("error running osascript"))
    }
}
