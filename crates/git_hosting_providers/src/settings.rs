use anyhow::Result;
use gpui::App;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::Settings;

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
pub struct GitProviderSettings {
    /// List of custom Git providers.
    #[serde(default)]
    pub providers: Vec<GitProviderConfig>,
}

impl Settings for GitProviderSettings {
    const KEY: Option<&'static str> = Some("git_providers");

    type FileContent = Self;

    fn load(sources: settings::SettingsSources<Self::FileContent>, _: &mut App) -> Result<Self> {
        sources.json_merge()
    }
}
