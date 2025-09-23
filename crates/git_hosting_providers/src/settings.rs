use std::sync::Arc;

use git::GitHostingProviderRegistry;
use gpui::App;
use settings::{GitHostingProviderConfig, GitHostingProviderKind, Settings, SettingsStore};
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

#[derive(Debug, Clone)]
pub struct GitHostingProviderSettings {
    pub git_hosting_providers: Vec<GitHostingProviderConfig>,
}

impl Settings for GitHostingProviderSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        Self {
            git_hosting_providers: content
                .project
                .git_hosting_providers
                .clone()
                .unwrap()
                .into(),
        }
    }
}
