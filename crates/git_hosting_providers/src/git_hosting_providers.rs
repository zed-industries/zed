mod providers;
mod settings;

use std::sync::Arc;

use anyhow::{Result, anyhow};
use git::GitHostingProviderRegistry;
use git::repository::GitRepository;
use gpui::App;
use url::Url;
use util::maybe;

pub use crate::providers::*;
pub use crate::settings::*;

/// Initializes the Git hosting providers.
pub fn init(cx: &mut App) {
    crate::settings::init(cx);

    let provider_registry = GitHostingProviderRegistry::global(cx);
    provider_registry.register_hosting_provider(Arc::new(Bitbucket::public_instance()));
    provider_registry.register_hosting_provider(Arc::new(Chromium));
    provider_registry.register_hosting_provider(Arc::new(Codeberg));
    provider_registry.register_hosting_provider(Arc::new(Gitee));
    provider_registry.register_hosting_provider(Arc::new(Github::public_instance()));
    provider_registry.register_hosting_provider(Arc::new(Gitlab::public_instance()));
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
    use super::get_host_from_git_remote_url;
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
}
