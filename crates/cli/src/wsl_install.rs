use anyhow::{Context as _, Result, anyhow, bail};
use std::path::Path;
use util::command::new_std_command;

pub(crate) fn install_wsl(app_path: &Path, distro: Option<&str>) -> Result<()> {
    if !app_path.exists() {
        bail!("Zed executable not found at {}", app_path.display());
    }

    let distros = list_distros().context("failed to list WSL distributions")?;
    if distros.is_empty() {
        bail!("No WSL distributions were found. Install WSL and a Linux distro first.");
    }

    let target_distros = if let Some(distro) = distro {
        if distros.iter().any(|item| item == distro) {
            vec![distro.to_string()]
        } else {
            bail!("WSL distribution '{distro}' not found");
        }
    } else {
        distros
    };

    for distro_name in target_distros {
        install_for_distro(app_path, &distro_name)?;
        println!("Installed `zed` for WSL distro {distro_name}");
    }

    println!("Restart your shell or run `source ~/.profile` to pick up PATH changes.");
    Ok(())
}

pub(crate) fn clear_wsl_cache() -> Result<()> {
    let distros = list_distros().context("Failed to list WSL distributions")?;
    if distros.is_empty() {
        bail!("No WSL distributions found. Install WSL and a distro first.");
    }

    let cache_dir = paths::remote_wsl_server_dir_relative()
        .display(util::paths::PathStyle::Posix)
        .to_string();
    let script = format!("rm -rf \"$HOME/{cache_dir}\"");

    let mut failures = Vec::new();
    for distro in distros {
        match run_wsl_script(&distro, &script) {
            Ok(()) => println!("Cleared WSL cache for distro: {distro}"),
            Err(error) => failures.push(format!("{distro}: {error}")),
        }
    }

    if failures.is_empty() {
        Ok(())
    } else {
        Err(anyhow!(
            "Failed to clear cache for some distros:\n{}",
            failures.join("\n")
        ))
    }
}

fn list_distros() -> Result<Vec<String>> {
    let output = new_std_command("wsl.exe")
        .args(["--list", "--quiet"])
        .output()
        .context("failed to execute wsl.exe")?;

    if !output.status.success() {
        bail!(
            "wsl.exe returned non-zero exit status: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }

    Ok(String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(String::from)
        .collect())
}

fn install_for_distro(app_path: &Path, distro: &str) -> Result<()> {
    let wsl_path = wsl_path_for_windows_path(app_path, distro)?;
    let script = build_wsl_shim(&wsl_path);
    let install_script = build_install_script(&script);

    let output = new_std_command("wsl.exe")
        .arg("--distribution")
        .arg(distro)
        .arg("--exec")
        .arg("sh")
        .arg("-c")
        .arg(install_script)
        .output()
        .with_context(|| format!("failed to install zed shim into {distro}"))?;

    if !output.status.success() {
        return Err(anyhow!(
            "install failed for {distro}: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }

    Ok(())
}

fn run_wsl_script(distro: &str, script: &str) -> Result<()> {
    let output = new_std_command("wsl.exe")
        .arg("--distribution")
        .arg(distro)
        .arg("--exec")
        .arg("sh")
        .arg("-c")
        .arg(script)
        .output()
        .with_context(|| format!("Failed to run script in {distro}"))?;
    if !output.status.success() {
        return Err(anyhow!(
            "script failed in {distro}: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    Ok(())
}

fn wsl_path_for_windows_path(app_path: &Path, distro: &str) -> Result<String> {
    let output = new_std_command("wsl.exe")
        .arg("--distribution")
        .arg(distro)
        .arg("--exec")
        .arg("wslpath")
        .arg("-u")
        .arg(app_path)
        .output()
        .with_context(|| format!("failed to resolve wsl path for {distro}"))?;

    if !output.status.success() {
        return Err(anyhow!(
            "wslpath failed for {distro}: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }

    let wsl_path = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if wsl_path.is_empty() {
        bail!("wslpath returned an empty path for {distro}");
    }
    Ok(wsl_path)
}

fn build_wsl_shim(wsl_zed_path: &str) -> String {
    let escaped_path = wsl_zed_path.replace('"', "\\\"");
    format!(
        r#"#!/usr/bin/env sh

if [ "${{ZED_WSL_DEBUG_INFO:-}}" = "true" ]; then
    set -x
fi

if [ -z "${{WSL_DISTRO_NAME:-}}" ]; then
    echo "This command must be run inside WSL." >&2
    exit 1
fi

ZED_WINDOWS_PATH="{escaped_path}"

if [ ! -x "$ZED_WINDOWS_PATH" ]; then
    echo "Zed for Windows not found at $ZED_WINDOWS_PATH." >&2
    exit 1
fi

if [ ! -r /proc/sys/fs/binfmt_misc/WSLInterop ]; then
    echo "WSL interop is disabled. Enable it to launch Windows apps." >&2
    exit 1
fi

if ! grep -qi enabled /proc/sys/fs/binfmt_misc/WSLInterop; then
    echo "WSL interop is disabled. Enable it to launch Windows apps." >&2
    exit 1
fi

WSL_USER="${{USER:-$USERNAME}}"
exec "$ZED_WINDOWS_PATH" --wsl "${{WSL_USER}}@${{WSL_DISTRO_NAME}}" "$@"
"#
    )
}

fn build_install_script(zed_shim: &str) -> String {
    let zed_shim = zed_shim.replace('\r', "");
    let export_path = r#"export PATH="$HOME/.local/bin:$PATH""#;
    let fish_path = r#"set -gx PATH $HOME/.local/bin $PATH"#;

    format!(
        r#"set -e
mkdir -p "$HOME/.local/bin"
cat <<'ZED_EOF' > "$HOME/.local/bin/zed"
{zed_shim}
ZED_EOF
chmod +x "$HOME/.local/bin/zed"

ensure_path_line() {{
    file="$1"
    line="$2"
    if [ ! -f "$file" ]; then
        touch "$file"
    fi
    if ! grep -Fxq "$line" "$file"; then
        printf '\n%s\n' "$line" >> "$file"
    fi
}}

ensure_path_line "$HOME/.profile" '{export_path}'
ensure_path_line "$HOME/.bashrc" '{export_path}'
ensure_path_line "$HOME/.zshrc" '{export_path}'
mkdir -p "$HOME/.config/fish"
ensure_path_line "$HOME/.config/fish/config.fish" '{fish_path}'
"#
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_install_script_contains_paths() {
        let shim = build_wsl_shim("/mnt/c/Program Files/Zed/Zed.exe");
        let install_script = build_install_script(&shim);

        assert!(install_script.contains("/mnt/c/Program Files/Zed/Zed.exe"));
        assert!(install_script.contains("export PATH"));
        assert!(install_script.contains("config.fish"));
    }
}
