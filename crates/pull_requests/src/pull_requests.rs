pub mod api;
pub mod auth;
pub mod models;
pub mod store;

use anyhow::Result;
use git::ParsedGitRemote;
use gpui::{App, Context, Entity, EventEmitter};
use project::Project;
use std::sync::Arc;

pub use api::{GithubPrClient, PullRequestApi};
pub use auth::{GithubAuth, GithubAuthDialog, GithubSettings};
pub use models::{
    CheckStatus, CreatePullRequest, PullRequest, PullRequestComment, PullRequestReview, PullRequestState,
    ReviewComment, ReviewState,
};
pub use store::PullRequestStore;

pub fn init(cx: &mut App) {
    auth::GithubAuth::init(cx);
    store::init(cx);
}

fn parse_github_remote(url: &str) -> Option<ParsedGitRemote> {
    // Handle both SSH and HTTPS GitHub URLs
    // SSH: git@github.com:owner/repo.git
    // HTTPS: https://github.com/owner/repo.git or https://github.com/owner/repo

    if url.starts_with("git@github.com:") {
        // SSH format
        let path = url.strip_prefix("git@github.com:")?;
        let path = path.strip_suffix(".git").unwrap_or(path);
        let parts: Vec<&str> = path.split('/').collect();
        if parts.len() == 2 {
            return Some(ParsedGitRemote {
                owner: parts[0].into(),
                repo: parts[1].into(),
            });
        }
    } else if url.contains("github.com/") {
        // HTTPS format
        let start = url.find("github.com/")? + "github.com/".len();
        let path = &url[start..];
        let path = path.strip_suffix(".git").unwrap_or(path);
        let parts: Vec<&str> = path.split('/').collect();
        if parts.len() >= 2 {
            return Some(ParsedGitRemote {
                owner: parts[0].into(),
                repo: parts[1].into(),
            });
        }
    }

    None
}

pub struct PullRequestManager {
    project: Entity<Project>,
    store: Entity<PullRequestStore>,
    api_client: Arc<dyn PullRequestApi>,
    _subscriptions: Vec<gpui::Subscription>,
}

impl PullRequestManager {
    pub fn new_with_store(
        project: Entity<Project>,
        store: Entity<PullRequestStore>,
        api_client: Arc<dyn PullRequestApi>,
        cx: &mut Context<Self>,
    ) -> Self {
        let subscriptions = vec![cx.subscribe(&store, Self::on_store_event)];

        Self {
            project,
            store,
            api_client,
            _subscriptions: subscriptions,
        }
    }

    pub fn store(&self) -> &Entity<PullRequestStore> {
        &self.store
    }

    pub fn refresh_pull_requests(&mut self, cx: &mut Context<Self>) -> anyhow::Result<()> {
        let remote = self.get_current_remote(cx)?;
        self.store
            .update(cx, |store, cx| store.fetch_pull_requests(remote, cx))
    }

    pub fn checkout_pull_request(
        &mut self,
        pr_number: u32,
        cx: &mut Context<Self>,
    ) -> anyhow::Result<()> {
        self.store
            .update(cx, |store, cx| store.checkout_pull_request(pr_number, cx))
    }

    pub fn update_api_client(&mut self, api_client: Arc<dyn PullRequestApi>, cx: &mut Context<Self>) {
        self.api_client = api_client.clone();
        self.store
            .update(cx, |store, _cx| store.update_api_client(api_client));
    }

    pub fn api_client(&self) -> Arc<dyn PullRequestApi> {
        self.api_client.clone()
    }

    pub fn get_current_remote(&self, cx: &App) -> Result<ParsedGitRemote> {
        // Get the git store from the project
        let git_store = self.project.read(cx).git_store();
        let repositories = git_store.read(cx).repositories();

        // Get the first repository with a remote URL
        for (_id, repo) in repositories {
            let snapshot = repo.read(cx).snapshot();
            if let Some(remote_url) = &snapshot.remote_origin_url {
                // Parse the remote URL to extract owner and repo
                if let Some(parsed) = parse_github_remote(remote_url) {
                    return Ok(parsed);
                }
            }
        }

        // Fallback to Zed repo if no git remote found
        Ok(ParsedGitRemote {
            owner: "zed-industries".into(),
            repo: "zed".into(),
        })
    }

    fn on_store_event(
        &mut self,
        _store: Entity<PullRequestStore>,
        event: &PullRequestStoreEvent,
        cx: &mut Context<Self>,
    ) {
        match event {
            PullRequestStoreEvent::PullRequestsUpdated => {
                cx.notify();
            }
            PullRequestStoreEvent::ActivePullRequestChanged => {
                cx.notify();
            }
        }
    }
}

impl EventEmitter<PullRequestManagerEvent> for PullRequestManager {}

pub enum PullRequestManagerEvent {
    PullRequestsRefreshed,
    PullRequestCheckedOut(u32),
}

pub enum PullRequestStoreEvent {
    PullRequestsUpdated,
    ActivePullRequestChanged,
}
