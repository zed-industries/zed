mod providers;

use std::sync::Arc;

use anyhow::{anyhow, Result};
use git::repository::GitRepository;
use git::GitHostingProviderRegistry;
use gpui::App;
use settings::Settings;
use url::Url;
use util::maybe;

pub use crate::providers::*;

/// Initializes the Git hosting providers.
pub fn init(cx: &mut App) {
    let provider_registry = GitHostingProviderRegistry::global(cx);
    provider_registry.register_hosting_provider(Arc::new(Bitbucket::public_instance()));
    provider_registry.register_hosting_provider(Arc::new(Chromium));
    provider_registry.register_hosting_provider(Arc::new(Codeberg));
    provider_registry.register_hosting_provider(Arc::new(Gitee));
    provider_registry.register_hosting_provider(Arc::new(Github::public_instance()));
    provider_registry.register_hosting_provider(Arc::new(Gitlab::public_instance()));
    provider_registry.register_hosting_provider(Arc::new(Sourcehut));

    GitProviderSettings::register(cx);

    let settings = GitProviderSettings::get_global(cx);

    settings
        .providers
        .clone()
        .unwrap_or(vec![])
        .iter()
        .for_each(|custom_provider_config| {
            if let Some(provider) =
                provider_registry.find_provider_by_type(&custom_provider_config.provider_type)
            {
                // TODO: Don't `unwrap`.
                match provider.provider_type() {
                    "bitbucket" => {
                        provider_registry.register_hosting_provider(Arc::new(Bitbucket::new(
                            &custom_provider_config.name,
                            Url::parse(&custom_provider_config.domain).unwrap(),
                        )));
                    }
                    "github" => {
                        provider_registry.register_hosting_provider(Arc::new(Github::new(
                            &custom_provider_config.name,
                            Url::parse(&custom_provider_config.domain).unwrap(),
                        )));
                    }
                    "gitlab" => {
                        provider_registry.register_hosting_provider(Arc::new(Gitlab::new(
                            &custom_provider_config.name,
                            Url::parse(&custom_provider_config.domain).unwrap(),
                        )));
                    }
                    _ => {}
                }
            }
        });
}

/// Registers additional Git hosting providers.
///
/// These require information from the Git repository to construct, so their
/// registration is deferred until we have a Git repository initialized.
pub fn register_additional_providers(
    provider_registry: Arc<GitHostingProviderRegistry>,
    repository: Arc<dyn GitRepository>,
) {
    let Some(origin_url) = repository.remote_url("origin") else {
        return;
    };

    if let Ok(gitlab_self_hosted) = Gitlab::from_remote_url(&origin_url) {
        provider_registry.register_hosting_provider(Arc::new(gitlab_self_hosted));
    } else if let Ok(github_self_hosted) = Github::from_remote_url(&origin_url) {
        provider_registry.register_hosting_provider(Arc::new(github_self_hosted));
    }
}

pub fn get_host_from_git_remote_url(remote_url: &str) -> Result<String> {
    maybe!({
        if let Some(remote_url) = remote_url.strip_prefix("git@") {
            if let Some((host, _)) = remote_url.trim_start_matches("git@").split_once(':') {
                return Some(host.to_string());
            }
        }

        Url::parse(&remote_url)
            .ok()
            .and_then(|remote_url| remote_url.host_str().map(|host| host.to_string()))
    })
    .ok_or_else(|| anyhow!("URL has no host"))
}

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

#[derive(
    Default, Clone, serde_derive::Serialize, serde_derive::Deserialize, schemars::JsonSchema,
)]
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

#[cfg(test)]
mod tests {

    use super::get_host_from_git_remote_url;
    use crate::GitProviderConfig;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_get_host_from_git_remote_url() {
        let tests = [
            (
                "https://jlannister@github.com/some-org/some-repo.git",
                Some("github.com".to_string()),
            ),
            (
                "git@github.com:zed-industries/zed.git",
                Some("github.com".to_string()),
            ),
            (
                "git@my.super.long.subdomain.com:zed-industries/zed.git",
                Some("my.super.long.subdomain.com".to_string()),
            ),
        ];

        for (remote_url, expected_host) in tests {
            let host = get_host_from_git_remote_url(remote_url).ok();
            assert_eq!(host, expected_host);
        }
    }

    #[test]
    fn test_git_provider_config_is_valid() {
        let config = GitProviderConfig {
            domain: "code.corp.big.com".to_string(),
            provider_type: "github".to_string(),
            name: "Corporate GitHub".to_string(),
        };
        assert!(config.is_valid());
    }
}
