use anyhow::Result;
use git::{GitHostingProviderRegistry, PullRequest, PullRequestComment, parse_git_remote_url};
use gpui::{Context, Entity, SharedString, Subscription, Task};
use project::git_store::{Repository, RepositoryEvent};

/// Identifies the branch state a fetch corresponds to, so repeated repository
/// notifications that don't change the branch or head commit don't trigger
/// redundant network requests.
#[derive(Clone, PartialEq, Eq)]
struct FetchKey {
    branch: String,
    head_sha: Option<String>,
}

pub enum PullRequestState {
    /// There is no branch, no remote, or the remote isn't a recognized hosting
    /// provider, so no pull request can be resolved.
    Unavailable,
    Loading,
    Loaded {
        /// `None` when the branch has no associated pull request.
        pull_request: Option<PullRequest>,
        comments: Vec<PullRequestComment>,
    },
    Error(SharedString),
}

/// Tracks the pull request (and its comments) associated with the current
/// branch of a [`Repository`], refetching whenever the branch or head commit
/// changes.
///
/// This deliberately performs the network request locally (via
/// [`gpui::App::http_client`]), mirroring how commit-author avatars are fetched
/// in `commit_tooltip`. It is not proxied to a collab host.
//
// TODO(pr-comments): the underlying provider requests still have known gaps:
//   - auth is limited to the `GITHUB_TOKEN` environment variable,
//   - comment results are not paginated (first page only),
//   - pull requests opened from forks aren't found (the `head` filter assumes
//     the branch lives in the base repo's owner).
pub struct PullRequestStore {
    repository: Entity<Repository>,
    state: PullRequestState,
    fetched_key: Option<FetchKey>,
    _fetch_task: Option<Task<()>>,
    _subscription: Subscription,
}

impl PullRequestStore {
    pub fn new(repository: Entity<Repository>, cx: &mut Context<Self>) -> Self {
        let subscription = Self::subscribe(&repository, cx);
        let mut this = Self {
            repository,
            state: PullRequestState::Unavailable,
            fetched_key: None,
            _fetch_task: None,
            _subscription: subscription,
        };
        this.refresh(cx);
        this
    }

    pub fn state(&self) -> &PullRequestState {
        &self.state
    }

    pub fn repository(&self) -> &Entity<Repository> {
        &self.repository
    }

    /// Binds to a different repository, e.g. when the active repository changes.
    pub fn set_repository(&mut self, repository: Entity<Repository>, cx: &mut Context<Self>) {
        if repository == self.repository {
            return;
        }
        self._subscription = Self::subscribe(&repository, cx);
        self.repository = repository;
        self.fetched_key = None;
        self.refresh(cx);
    }

    fn subscribe(repository: &Entity<Repository>, cx: &mut Context<Self>) -> Subscription {
        cx.subscribe(repository, |this, _repository, event, cx| {
            if matches!(event, RepositoryEvent::HeadChanged) {
                this.refresh(cx);
            }
        })
    }

    pub fn refresh(&mut self, cx: &mut Context<Self>) {
        let repository = self.repository.read(cx);
        let branch_name = repository
            .branch
            .as_ref()
            .map(|branch| branch.name().to_string());
        let head_sha = repository
            .head_commit
            .as_ref()
            .map(|commit| commit.sha.to_string());
        let remote_url = repository.default_remote_url();

        let (Some(branch_name), Some(remote_url)) = (branch_name, remote_url) else {
            self.set_unavailable(cx);
            return;
        };

        let key = FetchKey {
            branch: branch_name.clone(),
            head_sha,
        };
        if self.fetched_key.as_ref() == Some(&key) {
            // We've already loaded (or are loading) this exact branch state.
            return;
        }

        let Some((provider, remote)) =
            parse_git_remote_url(GitHostingProviderRegistry::global(cx), &remote_url)
        else {
            self.set_unavailable(cx);
            return;
        };

        let http_client = cx.http_client();
        self.fetched_key = Some(key);
        self.state = PullRequestState::Loading;
        cx.notify();

        self._fetch_task = Some(cx.spawn(async move |this, cx| {
            let result: Result<(Option<PullRequest>, Vec<PullRequestComment>)> = async {
                let pull_request = provider
                    .pull_request_for_branch(
                        remote.owner.as_ref(),
                        remote.repo.as_ref(),
                        &branch_name,
                        http_client.clone(),
                    )
                    .await?;
                let Some(pull_request) = pull_request else {
                    return Ok((None, Vec::new()));
                };
                let comments = provider
                    .pull_request_comments(
                        remote.owner.as_ref(),
                        remote.repo.as_ref(),
                        &pull_request.number.to_string(),
                        http_client,
                    )
                    .await?;
                Ok((Some(pull_request), comments))
            }
            .await;

            this.update(cx, |this, cx| {
                match result {
                    Ok((pull_request, comments)) => {
                        this.state = PullRequestState::Loaded {
                            pull_request,
                            comments,
                        };
                    }
                    Err(error) => {
                        // Clear the key so a later `HeadChanged` retries instead
                        // of treating the failed branch state as loaded.
                        this.fetched_key = None;
                        this.state = PullRequestState::Error(error.to_string().into());
                    }
                }
                cx.notify();
            })
            .ok();
        }));
    }

    fn set_unavailable(&mut self, cx: &mut Context<Self>) {
        self.fetched_key = None;
        self.state = PullRequestState::Unavailable;
        cx.notify();
    }
}
