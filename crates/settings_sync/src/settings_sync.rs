use credentials_provider::CredentialsProvider;
use gpui::App;
use schemars::JsonSchema;
use serde::Deserialize;
use settings::Settings;
use zed_actions::{PullSettingsFromGit, SyncSettingsToGit};

mod sync_engine;

const CREDENTIALS_URL: &str = "https://zed.dev/settings-sync";

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

    cx.on_action(|_: &SyncSettingsToGit, cx: &mut App| {
        let settings = SyncSettings::get_global(cx);
        if let Some(repo_url) = settings.git.repo_url.clone() {
            let branch = settings.git.branch.clone();
            
            let credentials_provider = <dyn CredentialsProvider>::global(cx);
            cx.spawn(|cx: &mut gpui::AsyncApp| {
                let cx = cx.clone();
                async move {
                    let token = if let Ok(Some((_, password))) = credentials_provider.read_credentials(CREDENTIALS_URL, &cx).await {
                        Some(String::from_utf8_lossy(&password).to_string())
                    } else {
                        log::warn!("No token found in keychain. We will try anonymous access.");
                        None
                    };

                    cx.background_executor()
                        .spawn(async move {
                            log::info!("Starting SyncSettingsToGit to {} on branch {}", repo_url, branch);
                            let engine = sync_engine::SyncEngine::new();
                            if let Err(e) = engine.push(&repo_url, &branch, token.as_deref()) {
                                log::error!("Failed to push settings to git: {:?}", e);
                            } else {
                                log::info!("Settings successfully pushed to git");
                            }
                        })
                        .await;
                }
            })
            .detach();
        } else {
            log::warn!("No repo_url configured for settings sync. Please configure sync.git.repo_url in settings.json");
        }
    });

    cx.on_action(|_: &PullSettingsFromGit, cx: &mut App| {
        let settings = SyncSettings::get_global(cx);
        if let Some(repo_url) = settings.git.repo_url.clone() {
            let branch = settings.git.branch.clone();
            let credentials_provider = <dyn CredentialsProvider>::global(cx);
            
            cx.spawn(|cx: &mut gpui::AsyncApp| {
                let cx = cx.clone();
                async move {
                    let token = if let Ok(Some((_, password))) = credentials_provider.read_credentials(CREDENTIALS_URL, &cx).await {
                        Some(String::from_utf8_lossy(&password).to_string())
                    } else {
                        None
                    };

                    cx.background_executor()
                        .spawn(async move {
                            log::info!("Starting PullSettingsFromGit from {} on branch {}", repo_url, branch);
                            let engine = sync_engine::SyncEngine::new();
                            if let Err(e) = engine.pull(&repo_url, &branch, token.as_deref()) {
                                log::error!("Failed to pull settings from git: {:?}", e);
                            } else {
                                log::info!("Settings successfully pulled from git");
                            }
                        })
                        .await;
                }
            })
            .detach();
        } else {
            log::warn!("No repo_url configured for settings sync. Please configure sync.git.repo_url in settings.json");
        }
    });
}
