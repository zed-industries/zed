use std::sync::Arc;

use anyhow::Result;
use git::GitHostingProviderRegistry;
use gpui::App;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsStore};
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
    let settings = GitHostingProviderSettings::get_global(cx);
    let provider_registry = GitHostingProviderRegistry::global(cx);

    for provider in settings.git_hosting_providers.iter() {
        let Some(url) = Url::parse(&provider.base_url).log_err() else {
            continue;
        };

        let provider = match provider.provider {
            GitHostingProviderKind::Bitbucket => Arc::new(Bitbucket::new(&provider.name, url)) as _,
            GitHostingProviderKind::Github => Arc::new(Github::new(&provider.name, url)) as _,
            GitHostingProviderKind::Gitlab => Arc::new(Gitlab::new(&provider.name, url)) as _,
        };

        provider_registry.register_hosting_provider(provider);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum GitHostingProviderKind {
    Github,
    Gitlab,
    Bitbucket,
}

/// A custom Git hosting provider.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHostingProviderConfig {
    /// The type of the provider.
    ///
    /// Must be one of `github`, `gitlab`, or `bitbucket`.
    pub provider: GitHostingProviderKind,

    /// The base URL for the provider (e.g., "https://code.corp.big.com").
    pub base_url: String,

    /// The display name for the provider (e.g., "BigCorp GitHub").
    pub name: String,
}

#[derive(Default, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHostingProviderSettings {
    /// The list of custom Git hosting providers.
    #[serde(default)]
    pub git_hosting_providers: Vec<GitHostingProviderConfig>,
}

impl Settings for GitHostingProviderSettings {
    const KEY: Option<&'static str> = None;

    type FileContent = Self;

    fn load(sources: settings::SettingsSources<Self::FileContent>, _: &mut App) -> Result<Self> {
        sources.json_merge()
    }

    fn import_from_vscode(_vscode: &settings::VsCodeSettings, _old: &mut Self::FileContent) {}
}
