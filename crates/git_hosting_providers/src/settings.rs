use std::sync::Arc;

use git::GitHostingProviderRegistry;
use gpui::App;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{
    GitHostingProviderConfig, GitHostingProviderKind, Settings, SettingsKey, SettingsStore,
    SettingsUi,
};
use url::Url;
use util::ResultExt as _;

use crate::{Bitbucket, Github, Gitlab};

pub(crate) fn init(cx: &mut App) {
    GitHostingProviderSettings::register(cx);

    init_git_hosting_provider_settings(cx);
}

fn init_git_hosting_provider_settings(cx: &mut App) {
    update_git_hosting_providers_from_settings(cx);

    cx.observe_global::<SettingsStore>(update_git_hosting_providers_from_settings)
        .detach();
}

fn update_git_hosting_providers_from_settings(cx: &mut App) {
    let settings_store = cx.global::<SettingsStore>();
    let settings = GitHostingProviderSettings::get_global(cx);
    let provider_registry = GitHostingProviderRegistry::global(cx);

    let local_values: Vec<GitHostingProviderConfig> = settings_store
        .get_all_locals::<GitHostingProviderSettings>()
        .into_iter()
        .flat_map(|(_, _, providers)| providers.git_hosting_providers.clone())
        .collect();

    let iter = settings
        .git_hosting_providers
        .clone()
        .into_iter()
        .chain(local_values)
        .filter_map(|provider| {
            let url = Url::parse(&provider.base_url).log_err()?;

            Some(match provider.provider {
                GitHostingProviderKind::Bitbucket => {
                    Arc::new(Bitbucket::new(&provider.name, url)) as _
                }
                GitHostingProviderKind::Github => Arc::new(Github::new(&provider.name, url)) as _,
                GitHostingProviderKind::Gitlab => Arc::new(Gitlab::new(&provider.name, url)) as _,
            })
        });

    provider_registry.set_setting_providers(iter);
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, SettingsUi, SettingsKey)]
#[settings_key(None)]
pub struct GitHostingProviderSettings {
    /// The list of custom Git hosting providers.
    #[serde(default)]
    pub git_hosting_providers: Vec<GitHostingProviderConfig>,
}

impl Settings for GitHostingProviderSettings {
    fn from_default(content: &settings::SettingsContent, _cx: &mut App) -> Option<Self> {
        Some(Self {
            git_hosting_providers: content.git_hosting_providers.clone()?,
        })
    }

    fn refine(&mut self, content: &settings::SettingsContent, _: &mut App) {
        if let Some(more) = &content.git_hosting_providers {
            self.git_hosting_providers.extend_from_slice(&more.clone());
        }
    }

    fn import_from_vscode(_: &settings::VsCodeSettings, _: &mut settings::SettingsContent) {}
}
