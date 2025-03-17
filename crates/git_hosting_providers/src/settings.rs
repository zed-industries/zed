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

pub fn init_git_hosting_provider_settings(cx: &mut App) {
    update_git_hosting_providers_from_settings(cx);

    cx.observe_global::<SettingsStore>(update_git_hosting_providers_from_settings)
        .detach();
}

fn update_git_hosting_providers_from_settings(cx: &mut App) {
    let settings = GitHostingProviderSettings::get_global(cx);
    let provider_registry = GitHostingProviderRegistry::global(cx);

    for provider in settings.providers.iter() {
        let Some(url) = Url::parse(&provider.domain).log_err() else {
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

/// Configuration for a custom Git hosting provider.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitProviderConfig {
    /// The type of the provider.
    ///
    /// Must be one of `github`, `gitlab`, or `bitbucket`.
    pub provider: GitHostingProviderKind,

    /// The domain name for the provider (e.g., "code.corp.big.com").
    pub domain: String,

    /// The display name for the provider (e.g., "MyCo GitHub").
    pub name: String,
}

#[derive(Default, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHostingProviderSettings {
    /// List of custom Git providers.
    #[serde(default)]
    pub providers: Vec<GitProviderConfig>,
}

impl Settings for GitHostingProviderSettings {
    const KEY: Option<&'static str> = Some("git_providers");

    type FileContent = Self;

    fn load(sources: settings::SettingsSources<Self::FileContent>, _: &mut App) -> Result<Self> {
        sources.json_merge()
    }
}
