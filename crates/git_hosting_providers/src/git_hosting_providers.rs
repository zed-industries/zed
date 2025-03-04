mod providers;

use std::sync::Arc;

use git::repository::GitRepository;
use git::GitHostingProviderRegistry;
use gpui::App;

pub use crate::providers::*;

/// Initializes the Git hosting providers.
pub fn init(cx: &App) {
    let provider_registry = GitHostingProviderRegistry::global(cx);
    provider_registry.register_hosting_provider(Arc::new(Bitbucket));
    provider_registry.register_hosting_provider(Arc::new(Chromium));
    provider_registry.register_hosting_provider(Arc::new(Codeberg));
    provider_registry.register_hosting_provider(Arc::new(Gitee));
    provider_registry.register_hosting_provider(Arc::new(Github));
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

    if let Ok(gitlab_self_hosted) = Gitlab::from_remote_url(&origin_url) {
        provider_registry.register_hosting_provider(Arc::new(gitlab_self_hosted));
    }
}
