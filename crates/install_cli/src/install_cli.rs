use anyhow::{anyhow, Result};
use gpui::{actions, AsyncAppContext};
use std::path::Path;
use util::ResultExt;

actions!(cli, [Install]);

#[cfg(not(windows))]
pub async fn install_cli(cx: &AsyncAppContext) -> Result<()> {
    let cli_path = cx.update(|cx| cx.path_for_auxiliary_executable("cli"))??;
    let link_path = Path::new("/usr/local/bin/zed");
    let bin_dir_path = link_path.parent().unwrap();

    // Don't re-create symlink if it points to the same CLI binary.
    if smol::fs::read_link(link_path).await.ok().as_ref() == Some(&cli_path) {
        return Ok(());
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
            return Ok(());
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
        Ok(())
    } else {
        Err(anyhow!("error running osascript"))
    }
}

#[cfg(windows)]
pub async fn install_cli(cx: &AsyncAppContext) -> Result<()> {
    use winreg::enums::*;
    use winreg::RegKey;

    let cli_path = cx.update(|cx| cx.path_for_auxiliary_executable("cli"))??;
    let cli_file_name = cli_path.file_name().expect("missing cli file name");
    let cli_parent_path = cli_path.parent();
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    // https://learn.microsoft.com/en-us/windows/win32/shell/app-registration
    let app_path_reg_path =
        Path::new("Software\\Microsoft\\Windows\\CurrentVersion\\App Paths").join(cli_file_name);
    let (app_path_reg_key, _) = hkcu.create_subkey(&app_path_reg_path)?;
    app_path_reg_key.set_value("", &cli_path.to_owned().into_os_string())?;

    if let Some(cli_parent_path) = cli_parent_path {
        let env_path_reg_path = Path::new("Environment");
        let (env_path_reg_key, _) = hkcu.create_subkey(&env_path_reg_path)?;
        let env_path_value: String = env_path_reg_key.get_value("Path")?;
        let mut env_path_values: Vec<String> = env_path_value
            .split(';')
            .map(|v| v.to_string())
            .into_iter()
            .filter(|v| v.trim().len() > 0)
            .filter(|v| !Path::new(v).join(cli_file_name).exists())
            .collect();

        env_path_values.push(cli_parent_path.to_string_lossy().to_string());

        let env_path_value: String = env_path_values.join(";");
        env_path_reg_key.set_value("Path", &env_path_value)?;
    }

    Ok(())
}
