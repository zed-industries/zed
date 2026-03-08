use gpui::App;
use schemars::JsonSchema;
use serde::Deserialize;
use settings::Settings;
use zed_actions::{PullSettingsFromGit, SyncSettingsToGit};

#[derive(Clone, Debug, Default, Deserialize, JsonSchema)]
pub struct SyncSettings {
    pub git: GitSyncSettings,
}

#[derive(Clone, Debug, Default, Deserialize, JsonSchema)]
pub struct GitSyncSettings {
    pub repo_url: Option<String>,
    pub branch: String,
}

impl Settings for SyncSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let mut settings = Self {
            git: GitSyncSettings {
                repo_url: None,
                branch: "main".to_string(),
            },
        };

        if let Some(sync) = &content.sync {
            if let Some(git) = &sync.git {
                if let Some(repo_url) = &git.repo_url {
                    settings.git.repo_url = Some(repo_url.clone());
                }
                if let Some(branch) = &git.branch {
                    settings.git.branch = branch.clone();
                }
            }
        }

        settings
    }
}

pub fn init(cx: &mut App) {
    SyncSettings::register(cx);

    cx.on_action(|_: &SyncSettingsToGit, _cx| {
        log::info!("SyncSettingsToGit action triggered (stub)");
    });

    cx.on_action(|_: &PullSettingsFromGit, _cx| {
        log::info!("PullSettingsFromGit action triggered (stub)");
    });
}
