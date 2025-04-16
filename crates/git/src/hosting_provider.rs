use std::{ops::Range, sync::Arc};

use anyhow::Result;
use async_trait::async_trait;
use collections::BTreeMap;
use derive_more::{Deref, DerefMut};
use gpui::{App, Global, SharedString};
use http_client::HttpClient;
use parking_lot::RwLock;
use url::Url;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PullRequest {
    pub number: u32,
    pub url: Url,
}

#[derive(Clone)]
pub struct GitRemote {
    pub host: Arc<dyn GitHostingProvider + Send + Sync + 'static>,
    pub owner: String,
    pub repo: String,
}

impl std::fmt::Debug for GitRemote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GitRemote")
            .field("host", &self.host.name())
            .field("owner", &self.owner)
            .field("repo", &self.repo)
            .finish()
    }
}

impl GitRemote {
    pub fn host_supports_avatars(&self) -> bool {
        self.host.supports_avatars()
    }

    pub async fn avatar_url(
        &self,
        commit: SharedString,
        client: Arc<dyn HttpClient>,
    ) -> Option<Url> {
        self.host
            .commit_author_avatar_url(&self.owner, &self.repo, commit, client)
            .await
            .ok()
            .flatten()
    }
}

pub struct BuildCommitPermalinkParams<'a> {
    pub sha: &'a str,
}

pub struct BuildPermalinkParams<'a> {
    pub sha: &'a str,
    pub path: &'a str,
    pub selection: Option<Range<u32>>,
}

/// A Git hosting provider.
#[async_trait]
pub trait GitHostingProvider {
    /// Returns the name of the provider.
    fn name(&self) -> String;

    /// Returns the base URL of the provider.
    fn base_url(&self) -> Url;

    /// Returns a permalink to a Git commit on this hosting provider.
    fn build_commit_permalink(
        &self,
        remote: &ParsedGitRemote,
        params: BuildCommitPermalinkParams,
    ) -> Url;

    /// Returns a permalink to a file and/or selection on this hosting provider.
    fn build_permalink(&self, remote: ParsedGitRemote, params: BuildPermalinkParams) -> Url;

    /// Returns whether this provider supports avatars.
    fn supports_avatars(&self) -> bool;

    /// Returns a URL fragment to the given line selection.
    fn line_fragment(&self, selection: &Range<u32>) -> String {
        if selection.start == selection.end {
            let line = selection.start + 1;

            self.format_line_number(line)
        } else {
            let start_line = selection.start + 1;
            let end_line = selection.end + 1;

            self.format_line_numbers(start_line, end_line)
        }
    }

    /// Returns a formatted line number to be placed in a permalink URL.
    fn format_line_number(&self, line: u32) -> String;

    /// Returns a formatted range of line numbers to be placed in a permalink URL.
    fn format_line_numbers(&self, start_line: u32, end_line: u32) -> String;

    fn parse_remote_url(&self, url: &str) -> Option<ParsedGitRemote>;

    fn extract_pull_request(
        &self,
        _remote: &ParsedGitRemote,
        _message: &str,
    ) -> Option<PullRequest> {
        None
    }

    async fn commit_author_avatar_url(
        &self,
        _repo_owner: &str,
        _repo: &str,
        _commit: SharedString,
        _http_client: Arc<dyn HttpClient>,
    ) -> Result<Option<Url>> {
        Ok(None)
    }
}

#[derive(Default, Deref, DerefMut)]
struct GlobalGitHostingProviderRegistry(Arc<GitHostingProviderRegistry>);

impl Global for GlobalGitHostingProviderRegistry {}

#[derive(Default)]
struct GitHostingProviderRegistryState {
    providers: BTreeMap<String, Arc<dyn GitHostingProvider + Send + Sync + 'static>>,
}

#[derive(Default)]
pub struct GitHostingProviderRegistry {
    state: RwLock<GitHostingProviderRegistryState>,
}

impl GitHostingProviderRegistry {
    /// Returns the global [`GitHostingProviderRegistry`].
    pub fn global(cx: &App) -> Arc<Self> {
        cx.global::<GlobalGitHostingProviderRegistry>().0.clone()
    }

    /// Returns the global [`GitHostingProviderRegistry`], if one is set.
    pub fn try_global(cx: &App) -> Option<Arc<Self>> {
        cx.try_global::<GlobalGitHostingProviderRegistry>()
            .map(|registry| registry.0.clone())
    }

    /// Returns the global [`GitHostingProviderRegistry`].
    ///
    /// Inserts a default [`GitHostingProviderRegistry`] if one does not yet exist.
    pub fn default_global(cx: &mut App) -> Arc<Self> {
        cx.default_global::<GlobalGitHostingProviderRegistry>()
            .0
            .clone()
    }

    /// Sets the global [`GitHostingProviderRegistry`].
    pub fn set_global(registry: Arc<GitHostingProviderRegistry>, cx: &mut App) {
        cx.set_global(GlobalGitHostingProviderRegistry(registry));
    }

    /// Returns a new [`GitHostingProviderRegistry`].
    pub fn new() -> Self {
        Self {
            state: RwLock::new(GitHostingProviderRegistryState {
                providers: BTreeMap::default(),
            }),
        }
    }

    /// Returns the list of all [`GitHostingProvider`]s in the registry.
    pub fn list_hosting_providers(
        &self,
    ) -> Vec<Arc<dyn GitHostingProvider + Send + Sync + 'static>> {
        self.state.read().providers.values().cloned().collect()
    }

    /// Adds the provided [`GitHostingProvider`] to the registry.
    pub fn register_hosting_provider(
        &self,
        provider: Arc<dyn GitHostingProvider + Send + Sync + 'static>,
    ) {
        self.state
            .write()
            .providers
            .insert(provider.name(), provider);
    }
}

#[derive(Debug, PartialEq)]
pub struct ParsedGitRemote {
    pub owner: Arc<str>,
    pub repo: Arc<str>,
}

pub fn parse_git_remote_url(
    provider_registry: Arc<GitHostingProviderRegistry>,
    url: &str,
) -> Option<(
    Arc<dyn GitHostingProvider + Send + Sync + 'static>,
    ParsedGitRemote,
)> {
    provider_registry
        .list_hosting_providers()
        .into_iter()
        .find_map(|provider| {
            provider
                .parse_remote_url(url)
                .map(|parsed_remote| (provider, parsed_remote))
        })
}
