use std::{
    collections::BTreeSet,
    path::{Path, PathBuf},
    str::FromStr,
    sync::Arc,
    time::{Duration, SystemTime},
};

use fs::Fs;
use serde_json::Value;
use smol::stream::{self, StreamExt};

// don't show projects you haven't used it the last 100 days
const PROJECT_MTIME_CUTOFF: Duration = Duration::from_secs(100 * 24 * 60 * 60);

async fn filter_old_files(paths: Vec<PathBuf>, fs: &dyn Fs) -> Option<Vec<PathBuf>> {
    let now = SystemTime::now();
    let with_mtimes: Option<Vec<(SystemTime, PathBuf)>> = stream::iter(paths.into_iter())
        .filter_map(async |path| {
            (
                fs.metadata(&path).await.ok()?.mtime.timestamp_for_user(),
                path,
            )
        })
        .collect();
    // paths.into_iter()
    // .filter_map(|path| std::fs::metadata(&path).ok()?.mtime())

    with_mtimes
        .into_iter()
        .filter(|(mtime, path)| {
            if let Ok(dur) = now.duration_since(mtime) {
                dur < PROJECT_MTIME_CUTOFF
            } else {
                false
            }
        })
        .map(|(mtime, path)| path)
        .collect()

    // TODO: use this for the ones that aren't ordered
}

async fn dir_contains_project(path: &Path, fs: &dyn Fs) -> bool {
    let Ok(mut paths) = fs.read_dir(path).await else {
        return false;
    };
    while let Some(Ok(path)) = paths.next().await {
        if Some(path) == PathBuf::from_str(".git").ok() {
            return true;
        }
    }
    false
}

// returns a list of project roots. ignores any file paths that aren't inside the user's home directory
async fn projects_for_paths(files: &[PathBuf], fs: &dyn Fs) -> Vec<PathBuf> {
    let mut known_roots = BTreeSet::new();
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
    known_roots.into_iter().collect()
}

// jq -r .folder Code/User/workspaceStorage/*/workspace.json
// or maybe .backupWorkspaces.folders[].folderUri from Code/User/globalStorage/storage.json
pub async fn get_vscode_projects(fs: Arc<dyn Fs>) -> Option<Vec<PathBuf>> {
    let path = paths::vscode_data_dir().join("User/globalStorage/storage.json");
    let content = fs.load(&path).await.ok()?;
    let value = serde_json::from_str::<Value>(&content).ok()?;
    let projects = util::json_get_path(&value, "backupWorkspaces.folders")?
        .as_array()?
        .iter()
        .map(|v| {
            Some(
                PathBuf::from_str(
                    v.as_object()?
                        .get("folderUri")?
                        .as_str()?
                        .strip_prefix("file://")?,
                )
                .ok()?,
            )
        })
        .collect::<Option<Vec<_>>>()?;
    filter_old_files(projects, fs.as_ref()).await
}

// nvim --headless +oldfiles +exit
pub async fn get_neovim_projects(fs: Arc<dyn Fs>) -> Option<Vec<PathBuf>> {
    const MAX_OLDFILES: usize = 100;
    let output = util::command::new_std_command("nvim")
        .args(["--headless", "-u", "NONE", "+oldfiles", "+exit"])
        .output()
        .ok()?
        .stderr;
    let paths = String::from_utf8(output)
        .ok()?
        .lines()
        .take(MAX_OLDFILES)
        .map(|s| s.split(": ").last().and_then(|s| PathBuf::from_str(s).ok()))
        .collect::<Option<Vec<PathBuf>>>()?;
    let projects = projects_for_paths(&paths, fs.as_ref()).await;
    filter_old_files(projects, fs.as_ref()).await
}

// sublime: jq -r .folder_history <Sublime\ Text/Local/Auto\ Save\ Session.sublime_session
// rust-rover: ??? JetBrains/RustRover20*/workspace/*.xml
