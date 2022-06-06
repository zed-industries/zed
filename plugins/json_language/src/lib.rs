use plugin::prelude::*;
use std::fs;

#[import]
fn command(string: String) -> Option<String>;

#[bind]
pub fn name(_: ()) -> &'static str {
    println!("huh, let me see...");
    Command::new("sh")
        .arg("-c")
        .arg("echo hello")
        .output()
        .expect("failed to execute process");
    "vscode-json-languageserver"
}

#[bind]
pub fn server_args(_: ()) -> Vec<String> {
    vec!["--stdio".into()]
}

#[bind]
fn fetch_latest_server_version() -> Option<String> {
    #[derive(Deserialize)]
    struct NpmInfo {
        versions: Vec<String>,
    }

    let output = command("npm info vscode-json-languageserver --json")?;
    if !output.status.success() {
        return None;
    }

    let mut info: NpmInfo = serde_json::from_slice(&output.stdout)?;
    info.versions.pop()
}

#[bind]
pub fn fetch_server_binary(version: String) -> Option<PathBuf> {
    let version_dir = container_dir.join(version.as_str());
    fs::create_dir_all(&version_dir)
        .await
        .context("failed to create version directory")?;
    let binary_path = version_dir.join(Self::BIN_PATH);

    if fs::metadata(&binary_path).await.is_err() {
        let output = smol::process::Command::new("npm")
            .current_dir(&version_dir)
            .arg("install")
            .arg(format!("vscode-json-languageserver@{}", version))
            .output()
            .await
            .context("failed to run npm install")?;
        if !output.status.success() {
            Err(anyhow!("failed to install vscode-json-languageserver"))?;
        }

        if let Some(mut entries) = fs::read_dir(&container_dir).await.log_err() {
            while let Some(entry) = entries.next().await {
                if let Some(entry) = entry.log_err() {
                    let entry_path = entry.path();
                    if entry_path.as_path() != version_dir {
                        fs::remove_dir_all(&entry_path).await.log_err();
                    }
                }
            }
        }
    }

    Ok(binary_path)
}

#[bind]
pub fn cached_server_binary(container_dir: PathBuf) -> Option<PathBuf> {
    let mut last_version_dir = None;
    let mut entries = fs::read_dir(&container_dir).await?;
    while let Some(entry) = entries.next().await {
        let entry = entry?;
        if entry.file_type().await?.is_dir() {
            last_version_dir = Some(entry.path());
        }
    }
    let last_version_dir = last_version_dir.ok_or_else(|| anyhow!("no cached binary"))?;
    let bin_path = last_version_dir.join(Self::BIN_PATH);
    if bin_path.exists() {
        Ok(bin_path)
    } else {
        Err(anyhow!(
            "missing executable in directory {:?}",
            last_version_dir
        ))
    }
}

#[bind]
pub fn initialization_options(_: ()) -> Option<serde_json::Value> {
    Some(json!({
        "provideFormatter": true
    }))
}

#[bind]
fn id_for_language(name: String) -> Option<String> {
    if name == "JSON" {
        Some("jsonc".into())
    } else {
        None
    }
}
