use std::path::Path;

use settings::{
    ExtendingVec, GuixConnection, RegisterSetting, Settings,
};

use crate::transport::guix::GuixShellOptions;

#[derive(RegisterSetting)]
pub struct GuixSettings {
    pub guix_connections: ExtendingVec<GuixConnection>,
}

impl GuixSettings {
    pub fn shell_options_for(&self, manifest_path: &Path, project_root: &Path) -> GuixShellOptions {
        let manifest_path = manifest_path.to_string_lossy();
        let project_root = project_root.to_string_lossy();

        self.guix_connections
            .0
            .iter()
            .find(|connection| {
                connection.manifest_path == manifest_path || connection.project_root == project_root
            })
            .map(|connection| connection.options.clone().into())
            .unwrap_or_default()
    }
}

impl Settings for GuixSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let remote = &content.remote;
        Self {
            guix_connections: remote.guix_connections.clone().unwrap_or_default().into(),
        }
    }
}
