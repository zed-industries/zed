use std::ops::Range;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use url::Url;

use crate::hosting_provider::GitHostingProvider;
use crate::hosting_providers::{Bitbucket, Codeberg, Gitee, Github, Gitlab, Sourcehut};

pub struct BuildPermalinkParams<'a> {
    pub remote_url: &'a str,
    pub sha: &'a str,
    pub path: &'a str,
    pub selection: Option<Range<u32>>,
}

pub fn build_permalink(params: BuildPermalinkParams) -> Result<Url> {
    let (provider, remote) = parse_git_remote_url(params.remote_url)
        .ok_or_else(|| anyhow!("failed to parse Git remote URL"))?;
    let line_fragment = params
        .selection
        .clone()
        .map(|selection| provider.line_fragment(&selection));

    let mut permalink = provider.build_permalink(remote, params);
    permalink.set_fragment(line_fragment.as_deref());
    Ok(permalink)
}

#[derive(Debug)]
pub struct ParsedGitRemote<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
}

pub struct BuildCommitPermalinkParams<'a> {
    pub remote: &'a ParsedGitRemote<'a>,
    pub sha: &'a str,
}

pub fn parse_git_remote_url(
    url: &str,
) -> Option<(
    Arc<dyn GitHostingProvider + Send + Sync + 'static>,
    ParsedGitRemote,
)> {
    let providers: Vec<Arc<dyn GitHostingProvider + Send + Sync + 'static>> = vec![
        Arc::new(Github),
        Arc::new(Gitlab),
        Arc::new(Bitbucket),
        Arc::new(Codeberg),
        Arc::new(Gitee),
        Arc::new(Sourcehut),
    ];

    providers.into_iter().find_map(|provider| {
        provider
            .parse_remote_url(&url)
            .map(|parsed_remote| (provider, parsed_remote))
    })
}
