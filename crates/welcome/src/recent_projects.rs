// nvim: nvim --headless +oldfiles +exit
// vscode: jq -r .folder ~/Library/Application\ Support/Code/User/workspaceStorage/*/workspace.json
// or maybe Code/User/globalStorage/storage.json
// sublime: jq -r .folder_history <~/Library/Application\ Support/Sublime\ Text/Local/Auto\ Save\ Session.sublime_session
// rust-rover: ??? ~/Library/Application\ Support/JetBrains/RustRover20*/workspace/*.xml

use std::{path::PathBuf, sync::Arc, time::Duration};

use fs::Fs;

pub struct RecentProject {
    path: PathBuf,
    last_opened_or_changed: Option<Duration>,
}

pub async fn get_vscode_projects(fs: Arc<dyn Fs>) -> Vec<RecentProject> {
    // paths::
}
