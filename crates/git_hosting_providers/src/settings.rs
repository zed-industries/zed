use anyhow::Result;
use gpui::App;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::Settings;

/// Configuration for a custom Git hosting provider
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct GitProviderConfig {
    /// Domain name for the provider (e.g., "code.corp.big.com")
    #[serde(default)]
    pub domain: String,

    /// The type of provider to use (must match a provider_type value)
    /// Examples: "github", "gitlab", "bitbucket"
    #[serde(rename = "type", default)]
    pub provider_type: String,

    /// Display name for the provider (e.g., "Corporate GitHub")
    #[serde(default)]
    pub name: String,
}

impl GitProviderConfig {
    /// Validates that all required fields are present
    pub fn is_valid(&self) -> bool {
        !self.domain.is_empty() && !self.provider_type.is_empty() && !self.name.is_empty()
    }
}

#[derive(Default, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitProviderSettings {
    /// List of custom Git providers
    #[serde(default)]
    pub providers: Option<Vec<GitProviderConfig>>,
}

impl Settings for GitProviderSettings {
    const KEY: Option<&'static str> = Some("git_providers");
    type FileContent = Self;

    fn load(sources: settings::SettingsSources<Self::FileContent>, _: &mut App) -> Result<Self> {
        let settings: Self = sources.json_merge()?;
        let default_providers: Vec<GitProviderConfig> = vec![];
        Ok(Self {
            providers: Some(
                settings
                    .providers
                    .ok_or_else(|| Some(default_providers))
                    .unwrap(),
            ),
        })
    }
}
