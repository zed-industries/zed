// nvim: nvim --headless +oldfiles +exit
// vscode: jq -r .folder Code/User/workspaceStorage/*/workspace.json
// or maybe .backupWorkspaces.folders[].folderUri from Code/User/globalStorage/storage.json
// sublime: jq -r .folder_history <Sublime\ Text/Local/Auto\ Save\ Session.sublime_session
// rust-rover: ??? JetBrains/RustRover20*/workspace/*.xml

use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    sync::Arc,
};

use fs::Fs;
use serde_json::Value;
use time::OffsetDateTime;

pub struct RecentProject {
    path: PathBuf,
    last_opened_or_changed: Option<OffsetDateTime>,
}

async fn mtime_for_project(root: &Path) -> Option<OffsetDateTime> {
    todo!()
}

async fn dir_contains_project(path: &Path, fs: Arc<dyn Fs>) -> bool {
    const ROOT_PROJECT_FILES: [&'static str] = [".git", "Cargo.lock", todo!()]
    fs.read_dir(path).await.any(|s|)
}

// returns a list of project roots. ignores any file paths that aren't inside the user's home directory
async fn projects_for_paths(files: &[PathBuf], fs: Arc<dyn Fs>) -> HashSet<PathBuf> {
    let mut known_roots = HashSet::new();
    let stop_at = paths::home_dir();
    for path in files {
        while let Some(parent) = path.parent() {
            if !parent.starts_with(stop_at) {
                break;
            }
            if known_roots.contains(parent) {
                continue;
            }
            if dir_contains_project(parent, fs).await {
                known_roots.insert(parent.to_path_buf());
            }
        }
    }
    known_roots
}

pub async fn get_vscode_projects(fs: Arc<dyn Fs>) -> Option<Vec<RecentProject>> {
    let path = paths::vscode_data_dir().join("User/globalStorage/storage.json");
    let content = fs.load(paths::vscode_settings_file()).await?;
    let storage = serde_json::from_str::<Value>(&content)?;
    util::json_get_path(storage, "backupWorkspaces.folders")
        .and_then(|v| v.as_array())
        .and_then(|arr| {
            arr.iter()
                .map(|v| v.as_object()?.get("folderUri")?.strip_prefix("file://"))
        })
        .collect()
}

pub async fn get_neovim_projects(fs: Arc<dyn Fs>) -> Option<Vec<RecentProject>> {
    const MAX_OLDFILES: usize = 100;
    let output = util::command::new_std_command("nvim")
        .args(["--headless", "-u", "NONE", "+oldfiles", "+exit"])
        .output()
        .ok()?
        .stderr;
    let files: Vec<PathBuf> = String::from_utf8(output)
        .ok()?
        .lines()
        .take(MAX_OLDFILES)
        .map(|s| s.split(": ").last().into())
        .collect()?;
    Some(
        projects_for_paths(&files, fs)
            .await
            .into_iter()
            .map(|p| RecentProject {
                path: p,
                last_opened_or_changed: mtime_for_project(p).await,
            })
            .collect(),
    )
}
