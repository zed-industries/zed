mod providers;

use std::collections::HashMap;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use git::repository::GitRepository;
use git::GitHostingProviderRegistry;
use gpui::{App, AppContext};
use settings::Settings;
use url::Url;
use util::maybe;

pub use crate::providers::*;

/// Configuration for a custom Git hosting provider
#[derive(Debug, Default, Clone, gpui::Serialize, gpui::Deserialize)]
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

/// Settings for Git hosting providers
#[derive(Debug, Default, Clone, gpui::Serialize, gpui::Deserialize)]
pub struct GitProviderSettings {
    /// List of custom Git providers
    #[serde(default)]
    pub providers: Vec<GitProviderConfig>,
}

impl Settings for GitProviderSettings {
    const KEY: Option<&'static str> = Some("git.providers");

    type FileContent = Self;

    fn load(file_content: Self::FileContent, _: &AppContext) -> anyhow::Result<Self> {
        Ok(file_content)
    }
}

/// Initializes the Git hosting providers.
pub fn init(cx: &App) {
    let provider_registry = GitHostingProviderRegistry::global(cx);
    provider_registry.register_hosting_provider(Arc::new(Bitbucket::new()));
    provider_registry.register_hosting_provider(Arc::new(Chromium));
    provider_registry.register_hosting_provider(Arc::new(Codeberg));
    provider_registry.register_hosting_provider(Arc::new(Gitee));
    provider_registry.register_hosting_provider(Arc::new(Github::new()));
    provider_registry.register_hosting_provider(Arc::new(Gitlab::new()));
    provider_registry.register_hosting_provider(Arc::new(Sourcehut));
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

    let Ok(host) = get_host_from_git_remote_url(&origin_url) else {
        log::error!("failed to parse Git remote URL: {}", origin_url);
        return;
    };

    // Check if we have custom provider configuration in settings
    if let Some(app_context) = provider_registry.app_context() {
        if let Ok(settings) = GitProviderSettings::get_global(app_context) {
            // Check for a custom provider configuration matching this domain
            if let Some(config) = settings.providers.iter().find(|p| p.domain == host) {
                // Validate that all required fields are present
                if !config.is_valid() {
                    log::warn!(
                        "Invalid provider configuration for {}: missing required fields. All of domain, type, and name must be specified.", 
                        host
                    );
                    // Continue to fallback logic
                } else {
                    log::info!(
                        "Using custom provider configuration for {}: type={}, name={}", 
                        host, 
                        config.provider_type,
                        config.name
                    );
                    
                    // Try to create a self-hosted instance based on the provider type
                    if let Ok(Some(provider)) = provider_registry.create_self_hosted_instance(
                        &config.provider_type, 
                        &host
                    ) {
                        provider_registry.register_hosting_provider(provider);
                        return;
                    } else {
                        log::warn!(
                            "Failed to create self-hosted instance for {}. Provider type '{}' may be invalid.",
                            host,
                            config.provider_type
                        );
                    }
                }
            }
        }
    }

    // Fall back to the existing provider detection logic
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::Arc;

    use super::{get_host_from_git_remote_url, GitProviderConfig, GitProviderSettings};
    use anyhow::Result;
    use git::{parse_git_remote_url, GitHostingProvider, GitHostingProviderRegistry};
    use gpui::{AppContext, TestAppContext, platform::TestDispatcher};
    use pretty_assertions::assert_eq;
    use rand::rngs::StdRng;
    use settings::{Settings, SettingsStore};

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
    fn test_custom_domain_provider_mapping() {
        // Create a test app context with a dispatcher
        let dispatcher = gpui::platform::TestDispatcher::new(rand::rngs::StdRng::seed_from_u64(0));
        let mut cx = TestAppContext::new(dispatcher, Some("test_custom_domain_provider_mapping"));

        // Initialize settings
        SettingsStore::test(&mut cx);

        // Create a provider registry
        let provider_registry = GitHostingProviderRegistry::new();
        let registry_arc = Arc::new(provider_registry);

        // Register standard providers
        registry_arc.register_hosting_provider(Arc::new(super::Github::new()));
        registry_arc.register_hosting_provider(Arc::new(super::Gitlab::new()));

        // Create and register our settings with a custom provider
        let settings = GitProviderSettings {
            providers: vec![
                GitProviderConfig {
                    domain: "code.corp.big.com".into(),
                    provider_type: "github".into(),
                    name: "Corporate GitHub".into(),
                }
            ],
        };

        cx.update(|cx| {
            // Set the registry as global
            GitHostingProviderRegistry::set_global(registry_arc.clone(), cx);

            // Set our settings as global
            settings.set_global(cx);
        });

        // Test that custom domain mapping works
        let custom_url = "https://code.corp.big.com/fungroup/risk-service.git";

        // Verify that standard parsing fails for custom domain
        let result = parse_git_remote_url(registry_arc.clone(), custom_url);
        assert!(
            result.is_none(),
            "Standard parsing should not work with custom domain"
        );

        // Manually create a self-hosted provider based on our settings
        let host = get_host_from_git_remote_url(custom_url).unwrap();
        let config = settings.providers.iter().find(|p| p.domain == host).unwrap();
        
        let provider_result = registry_arc.create_self_hosted_instance(&config.provider_type, &host);
        assert!(provider_result.is_ok());
        
        if let Some(provider) = provider_result.unwrap() {
            registry_arc.register_hosting_provider(provider);
        }

        // Check that a provider is now registered that can handle our custom URL
        let result = parse_git_remote_url(registry_arc.clone(), custom_url);
        assert!(
            result.is_some(),
            "Custom domain mapping should allow parsing the URL"
        );

        if let Some((provider, parsed_remote)) = result {
            assert_eq!(provider.provider_type(), "github");
            assert_eq!(parsed_remote.owner.to_string(), "fungroup");
            assert_eq!(parsed_remote.repo.to_string(), "risk-service");
        }
    }

    /// Test for SSH-style git URLs with custom domains
    #[test]
    fn test_custom_domain_ssh_style_urls() {
        // Create a test app context with a dispatcher
        let dispatcher = gpui::platform::TestDispatcher::new(rand::rngs::StdRng::seed_from_u64(0));
        let mut cx = TestAppContext::new(dispatcher, Some("test_custom_domain_ssh_style_urls"));
        
        // Initialize settings
        SettingsStore::test(&mut cx);
        
        // Create a provider registry
        let provider_registry = GitHostingProviderRegistry::new();
        let registry_arc = Arc::new(provider_registry);
        
        // Register standard providers
        registry_arc.register_hosting_provider(Arc::new(super::Github::new()));
        registry_arc.register_hosting_provider(Arc::new(super::Gitlab::new()));
        
        // Create and register our settings with a custom provider
        let settings = GitProviderSettings {
            providers: vec![
                GitProviderConfig {
                    domain: "code.corp.big.com".into(),
                    provider_type: "github".into(),
                    name: "Corporate GitHub".into(),
                }
            ],
        };
        
        cx.update(|cx| {
            // Set the registry as global
            GitHostingProviderRegistry::set_global(registry_arc.clone(), cx);
            
            // Set our settings as global
            settings.set_global(cx);
        });
        
        // Test SSH-style custom domain URL
        let ssh_style_url = "git@code.corp.big.com:fungroup/risk-service.git";
        
        // First verify we can extract the host correctly
        let host_result = get_host_from_git_remote_url(ssh_style_url);
        assert!(host_result.is_ok(), "Should be able to extract host from SSH-style URL");
        assert_eq!(host_result.unwrap(), "code.corp.big.com");
        
        // Check that the URL doesn't parse with standard providers
        let result = parse_git_remote_url(registry_arc.clone(), ssh_style_url);
        assert!(result.is_none(), "SSH custom domain URL should not parse with standard providers");
        
        // Create the custom provider
        let host = "code.corp.big.com";
        let config = settings.providers.iter().find(|p| p.domain == host).unwrap();
        let provider_result = registry_arc.create_self_hosted_instance(&config.provider_type, host);
        assert!(provider_result.is_ok());
        
        if let Some(provider) = provider_result.unwrap() {
            registry_arc.register_hosting_provider(provider);
        }
        
        // Now check that the URL parses correctly with our custom provider
        let result = parse_git_remote_url(registry_arc.clone(), ssh_style_url);
        assert!(result.is_some(), "SSH custom domain URL should parse with custom provider");
        
        if let Some((provider, parsed_remote)) = result {
            assert_eq!(provider.provider_type(), "github");
            assert_eq!(parsed_remote.owner.to_string(), "fungroup");
            assert_eq!(parsed_remote.repo.to_string(), "risk-service");
        }
    }
    
    /// Test for validating provider configuration
    #[test]
    fn test_provider_config_validation() {
        // Valid configuration with all fields
        let valid_config = GitProviderConfig {
            domain: "code.corp.big.com".to_string(),
            provider_type: "github".to_string(),
            name: "Corporate GitHub".to_string(),
        };
        assert!(valid_config.is_valid(), "Valid config should pass validation");
        
        // Invalid configurations with missing fields
        let missing_domain = GitProviderConfig {
            domain: "".to_string(),
            provider_type: "github".to_string(),
            name: "Corporate GitHub".to_string(),
        };
        assert!(!missing_domain.is_valid(), "Config with missing domain should fail validation");
        
        let missing_type = GitProviderConfig {
            domain: "code.corp.big.com".to_string(),
            provider_type: "".to_string(),
            name: "Corporate GitHub".to_string(),
        };
        assert!(!missing_type.is_valid(), "Config with missing type should fail validation");
        
        let missing_name = GitProviderConfig {
            domain: "code.corp.big.com".to_string(),
            provider_type: "github".to_string(),
            name: "".to_string(),
        };
        assert!(!missing_name.is_valid(), "Config with missing name should fail validation");
        
        // Default-constructed config should be invalid
        let default_config = GitProviderConfig::default();
        assert!(!default_config.is_valid(), "Default config should be invalid");
    }
    
    /// Test for deserializing git.providers with invalid settings
    #[test]
    fn test_git_providers_invalid_settings() {
        use serde_json::json;
        use settings::SettingsStore;
        
        // Create a test app context with a dispatcher
        let dispatcher = gpui::platform::TestDispatcher::new(rand::rngs::StdRng::seed_from_u64(0));
        let mut cx = TestAppContext::new(dispatcher, Some("test_git_providers_invalid_settings"));
        
        // Initialize settings
        let store = SettingsStore::test(&mut cx);
        
        // Create a JSON config with invalid git.providers (missing required fields)
        let config = json!({
            "git": {
                "providers": [
                    {
                        "domain": "code.corp.big.com",
                        // Missing "type" field
                        "name": "Corporate GitHub"
                    },
                    {
                        // Missing "domain" field
                        "type": "gitlab",
                        "name": "Internal GitLab"
                    },
                    {
                        "domain": "missing.name.example.com",
                        "type": "github",
                        // Missing "name" field
                    }
                ]
            }
        });
        
        // Set up test settings
        store.update_user_settings(config.to_string());
        
        // Get the GitProviderSettings
        cx.update(|cx| {
            // Check that we can parse the settings (even with invalid entries)
            let settings = GitProviderSettings::get_global(cx).expect("Failed to get settings");
            
            // We should have 3 entries (all were deserialized, even if invalid)
            assert_eq!(settings.providers.len(), 3, "Should have 3 provider entries");
            
            // Check validation on each entry
            let invalid_providers = settings.providers.iter()
                .filter(|p| !p.is_valid())
                .count();
                
            assert_eq!(invalid_providers, 3, "All providers should be invalid");
            
            // Find one specific entry and verify its partial data
            let corp_github = settings.providers.iter()
                .find(|p| p.domain == "code.corp.big.com")
                .expect("Missing code.corp.big.com provider");
                
            assert_eq!(corp_github.name, "Corporate GitHub");
            assert!(corp_github.provider_type.is_empty(), "type should be empty");
            assert!(!corp_github.is_valid(), "Should be invalid due to missing type");
        });
    }
    
    /// Test for deserializing git.providers settings
    #[test]
    fn test_git_providers_settings_deserialization() {
        use serde_json::json;
        use settings::SettingsStore;
        use std::path::PathBuf;
        
        // Create a test app context with a dispatcher
        let dispatcher = gpui::platform::TestDispatcher::new(rand::rngs::StdRng::seed_from_u64(0));
        let mut cx = TestAppContext::new(dispatcher, Some("test_git_providers_settings_deserialization"));
        
        // Initialize settings
        let store = SettingsStore::test(&mut cx);
        
        // Create a JSON config with git.providers
        let config = json!({
            "git": {
                "providers": [
                    {
                        "domain": "code.corp.big.com",
                        "type": "github",
                        "name": "Corporate GitHub"
                    },
                    {
                        "domain": "git.internal.org",
                        "type": "gitlab",
                        "name": "Internal GitLab"
                    }
                ]
            }
        });
        
        // Set up test settings
        store.update_user_settings(config.to_string());
        
        // Get the GitProviderSettings
        cx.update(|cx| {
            // Check that we can properly deserialize our settings
            let settings = GitProviderSettings::get_global(cx).expect("Failed to get settings");
            
            // Verify the first provider entry
            let corp_github = settings.providers.iter()
                .find(|p| p.domain == "code.corp.big.com")
                .expect("Missing code.corp.big.com provider");
                
            assert_eq!(corp_github.provider_type, "github");
            assert_eq!(corp_github.name, "Corporate GitHub");
            
            // Verify the second provider entry
            let internal_gitlab = settings.providers.iter()
                .find(|p| p.domain == "git.internal.org")
                .expect("Missing git.internal.org provider");
                
            assert_eq!(internal_gitlab.provider_type, "gitlab");
            assert_eq!(internal_gitlab.name, "Internal GitLab");
            
            // Verify the total number of providers
            assert_eq!(settings.providers.len(), 2, "Wrong number of providers");
        });
    }
    
    /// Test specifically for the "failed to parse Git remote URL" error
    #[test]
    fn test_custom_domain_avoids_parse_error() {
        // Create a test app context with a dispatcher
        let dispatcher = gpui::platform::TestDispatcher::new(rand::rngs::StdRng::seed_from_u64(0));
        let mut cx = TestAppContext::new(dispatcher, Some("test_custom_domain_avoids_parse_error"));

        // Initialize settings
        SettingsStore::test(&mut cx);

        // Create a provider registry
        let provider_registry = GitHostingProviderRegistry::new();
        let registry_arc = Arc::new(provider_registry);

        // Register standard providers
        registry_arc.register_hosting_provider(Arc::new(super::Github::new()));
        registry_arc.register_hosting_provider(Arc::new(super::Gitlab::new()));

        // Setup test URLs
        let standard_github_url = "https://github.com/zed-industries/zed.git";
        let custom_domain_url = "https://code.corp.big.com/fungroup/risk-service.git";

        // Verify we can parse a standard URL
        let result = get_host_from_git_remote_url(standard_github_url);
        assert!(result.is_ok(), "Should parse standard GitHub URL");
        assert_eq!(result.unwrap(), "github.com");

        // Verify we can also parse a custom domain URL
        let result = get_host_from_git_remote_url(custom_domain_url);
        assert!(result.is_ok(), "Should parse custom domain URL");
        assert_eq!(result.unwrap(), "code.corp.big.com");

        // Test with settings enabled
        let settings = GitProviderSettings {
            providers: vec![
                GitProviderConfig {
                    domain: "code.corp.big.com".into(),
                    provider_type: "github".into(),
                    name: "Corporate GitHub".into(),
                }
            ],
        };

        cx.update(|cx| {
            // Set the registry as global
            GitHostingProviderRegistry::set_global(registry_arc.clone(), cx);

            // Set our settings as global
            settings.set_global(cx);
        });

        // First simulate the problem case without our custom domain provider
        {
            // Test direct URL parsing - should fail with standard providers
            let result = parse_git_remote_url(registry_arc.clone(), custom_domain_url);
            assert!(result.is_none(), "Before our fix, parsing should fail for custom domain");
            
            // Extract the host from the URL (this part works fine - it's provider matching that fails)
            let host_result = get_host_from_git_remote_url(custom_domain_url);
            assert!(host_result.is_ok(), "Host extraction should succeed");
            let host = host_result.unwrap();
            assert_eq!(host, "code.corp.big.com");
            
            // Verify no standard provider can handle this URL
            let found_provider = registry_arc
                .list_hosting_providers()
                .into_iter()
                .find(|provider| provider.parse_remote_url(custom_domain_url).is_some());
                
            assert!(
                found_provider.is_none(),
                "Before our fix, no provider would handle this domain"
            );
        }
        
        // Now test with our fix - create a self-hosted provider using our config
        {
            // Get the configuration from settings
            let host = "code.corp.big.com";
            let config = settings.providers.iter().find(|p| p.domain == host).unwrap();
            
            // Create a self-hosted provider based on the configuration
            let custom_provider_result = registry_arc.create_self_hosted_instance(
                &config.provider_type, 
                &host
            );
            assert!(custom_provider_result.is_ok(), "Should be able to create custom provider");
            
            let custom_provider = custom_provider_result.unwrap();
            assert!(custom_provider.is_some(), "Should get a valid provider instance");
            
            // Register the provider
            if let Some(provider) = custom_provider {
                registry_arc.register_hosting_provider(provider);
            }
            
            // Now try to parse the URL - this should succeed with our custom provider
            let result = parse_git_remote_url(registry_arc.clone(), custom_domain_url);
            assert!(result.is_some(), "With our fix, URL parsing should succeed");
            
            // Check that the resulting provider has the right type and parsing is correct
            if let Some((provider, parsed_remote)) = result {
                assert_eq!(provider.provider_type(), "github");
                assert_eq!(parsed_remote.owner.to_string(), "fungroup");
                assert_eq!(parsed_remote.repo.to_string(), "risk-service");
            }
        }
    }
}
