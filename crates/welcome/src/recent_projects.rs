use std::{
    collections::BTreeSet,
    ffi::OsString,
    path::{Path, PathBuf},
    sync::Arc,
    time::{Duration, SystemTime},
};

use fs::Fs;
use serde_json::Value;
use smol::stream::{self, StreamExt};

// don't show projects from editors you haven't used it the last 100 days
const MTIME_CUTOFF: Duration = Duration::from_secs(100 * 24 * 60 * 60);

// filters out old files and sorts the remaining ones by recency
async fn recent_files(paths: Vec<PathBuf>, fs: &dyn Fs) -> Vec<String> {
    let now = SystemTime::now();
    stream::iter(paths.into_iter())
        .flat_map(|path| stream::once_future(async move { (fs.metadata(&path).await, path) }))
        .filter_map(|(metadata, path)| {
            let mtime = metadata.ok()??.mtime;
            if let Ok(duration) = now.duration_since(mtime.timestamp_for_user()) {
                (duration < MTIME_CUTOFF).then_some(path.to_string_lossy().to_string())
            } else {
                None
            }
        })
        .collect()
        .await
}

async fn dir_contains_project(path: &Path, fs: &dyn Fs) -> bool {
    let Ok(mut paths) = fs.read_dir(path).await else {
        return false;
    };
    while let Some(Ok(path)) = paths.next().await {
        // TODO: look for other project files, ".jj", etc
        if path.file_name() == Some(&OsString::from(".git")) {
            return true;
        }
    }
    false
}

// returns a list of project roots. ignores any file paths that aren't inside the user's home directory
async fn projects_for_paths(files: &[PathBuf], fs: &dyn Fs) -> Vec<PathBuf> {
    let mut project_dirs = BTreeSet::new();
    let stop_at = paths::home_dir();
    for mut path in files.iter().map(|p| p.as_path()) {
        while let Some(parent) = path.parent() {
            if !parent.starts_with(stop_at) || project_dirs.contains(parent) {
                break;
            }
            if dir_contains_project(parent, fs).await {
                project_dirs.insert(parent.to_path_buf());
            }
            path = parent;
        }
    }
    project_dirs.into_iter().collect()
}

// jq .backupWorkspaces.folders[].folderUri from Code/User/globalStorage/storage.json
//
// jq -r .folder Code/User/workspaceStorage/*/workspace.json
pub async fn get_vscode_projects(fs: Arc<dyn Fs>) -> Option<Vec<String>> {
    let mut read_dir = fs
        .read_dir(&paths::vscode_data_dir().join("User/workspaceStorage"))
        .await
        .ok()?;
    let mut workspaces = Vec::new();
    while let Some(Ok(dir)) = read_dir.next().await {
        workspaces.push(dir.join("workspace.json"))
    }
    let mut result = Vec::new();
    for path in recent_files(workspaces, fs.as_ref()).await {
        let content = fs.load(&PathBuf::from(path)).await.ok()?;
        let value = serde_json::from_str::<Value>(&content).ok()?;
        if let Some(s) = value.get("folder").and_then(|v| v.as_str()) {
            result.push(s.strip_prefix("file://").unwrap_or(s).to_owned());
        }
    }
    Some(result)
}

// nvim --headless -u NONE +oldfiles +exit
pub async fn get_neovim_projects(fs: Arc<dyn Fs>) -> Option<Vec<String>> {
    const MAX_OLDFILES: usize = 100;
    let output = util::command::new_std_command("nvim")
        .args(["--headless", "-u", "NONE", "+oldfiles", "+exit"])
        .output()
        .ok()?
        .stderr;
    let paths = String::from_utf8_lossy(&output)
        .lines()
        .take(MAX_OLDFILES)
        .filter_map(|s| s.split(": ").last().map(|s| PathBuf::from(s)))
        .collect::<Vec<PathBuf>>();
    let projects = projects_for_paths(&paths, fs.as_ref()).await;
    Some(recent_files(projects, fs.as_ref()).await)
}

// jq -r '.folder_history[]' <Sublime\ Text>/Local/Session.sublime_session
// there's also an "Auto Save Session" file that may be more up to date? ignoring for now
pub async fn get_sublime_projects(fs: Arc<dyn Fs>) -> Option<Vec<String>> {
    let path = paths::sublime_data_dir().join("Local/Session.sublime_session");
    let mtime = fs.metadata(&path).await.ok()??.mtime;
    if let Ok(duration) = SystemTime::now().duration_since(mtime.timestamp_for_user()) {
        if duration > MTIME_CUTOFF {
            return None;
        }
    }

    let content = fs.load(&path).await.ok()?;
    let value = serde_json::from_str::<Value>(&content).ok()?;
    value
        .as_object()?
        .get("folder_history")?
        .as_array()?
        .iter()
        .map(|v| v.as_str().map(|s| s.to_owned()))
        .collect()
}

// rust-rover: ??? JetBrains/RustRover20*/workspace/*.xml
