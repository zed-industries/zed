use std::{
    collections::BTreeSet,
    path::{Path, PathBuf},
    str::FromStr,
    sync::Arc,
    time::{Duration, SystemTime},
};

use fs::Fs;
use serde_json::Value;
use smol::stream::{self,  StreamExt};

// don't show projects you haven't used it the last 100 days
const PROJECT_MTIME_CUTOFF: Duration = Duration::from_secs(100 * 24 * 60 * 60);

// filters out old files and sorts the remaining ones by recency
async fn recent_files(paths: Vec<PathBuf>, fs: &dyn Fs) -> Vec<PathBuf> {
    let now = SystemTime::now();
    stream::iter(paths.into_iter())
        .flat_map(|path| stream::once_future(async move { (fs.metadata(&path).await, path) }))
        .filter_map(|(metadata, path)| {
            let mtime = metadata.ok()??.mtime;
            if let Ok(duration) = now.duration_since(mtime.timestamp_for_user()) {
                (duration < PROJECT_MTIME_CUTOFF).then_some(path)
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

// jq .backupWorkspaces.folders[].folderUri from Code/User/globalStorage/storage.json
//
// jq -r .folder Code/User/workspaceStorage/*/workspace.json
pub async fn get_vscode_projects(fs: Arc<dyn Fs>) -> Option<Vec<PathBuf>> {
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
        let content = fs.load(&path).await.ok()?;
        let value = serde_json::from_str::<Value>(&content).ok()?;
        if let Some(s) = value.get("folder").and_then(|v| v.as_str()) {
            result.push(PathBuf::from(s));
        }
    }
    Some(result)
}

// nvim --headless +oldfiles +exit
pub async fn get_neovim_projects(fs: Arc<dyn Fs>) -> Option<Vec<PathBuf>> {
    dbg!("getting neovim");
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
    dbg!("found", &projects);
    Some(recent_files(projects, fs.as_ref()).await)
}

// sublime: jq -r .folder_history <Sublime\ Text/Local/Auto\ Save\ Session.sublime_session
// rust-rover: ??? JetBrains/RustRover20*/workspace/*.xml
