use anyhow::Result;
use collections::HashMap;
use git::{GitHostingProviderRegistry, PullRequestComment, parse_git_remote_url};
use gpui::{
    App, AppContext as _, Context, Entity, EntityId, Global, Subscription, Task, WeakEntity,
};
use project::{
    Project,
    git_store::{GitStoreEvent, Repository, RepositoryEvent},
    project_settings::ProjectSettings,
};
use settings::{Settings as _, SettingsStore};

#[derive(Clone, PartialEq, Eq)]
struct FetchKey {
    repository: EntityId,
    branch: String,
    head_sha: Option<String>,
}

pub struct PullRequestStore {
    project: WeakEntity<Project>,
    active_repository: Option<Entity<Repository>>,
    comments_by_path: HashMap<String, Vec<PullRequestComment>>,
    fetched_key: Option<FetchKey>,
    _fetch_task: Option<Task<()>>,
    _subscriptions: Vec<Subscription>,
}

#[derive(Default)]
struct GlobalPullRequestStores(HashMap<EntityId, WeakEntity<PullRequestStore>>);

impl Global for GlobalPullRequestStores {}

impl PullRequestStore {
    pub fn for_project(project: &Entity<Project>, cx: &mut App) -> Entity<Self> {
        let project_id = project.entity_id();
        if let Some(store) = cx
            .try_global::<GlobalPullRequestStores>()
            .and_then(|stores| stores.0.get(&project_id))
            .and_then(WeakEntity::upgrade)
        {
            return store;
        }

        let store = cx.new(|cx| Self::new(project.clone(), cx));
        cx.default_global::<GlobalPullRequestStores>()
            .0
            .insert(project_id, store.downgrade());
        store
    }

    fn new(project: Entity<Project>, cx: &mut Context<Self>) -> Self {
        let git_store = project.read(cx).git_store().clone();
        let git_store_subscription =
            cx.subscribe(&git_store, |this, _git_store, event, cx| match event {
                GitStoreEvent::RepositoryUpdated(_, RepositoryEvent::HeadChanged, _)
                | GitStoreEvent::ActiveRepositoryChanged(_) => this.refresh(cx),
                _ => {}
            });
        let settings_subscription = cx.observe_global::<SettingsStore>(|this, cx| this.refresh(cx));

        let mut this = Self {
            project: project.downgrade(),
            active_repository: None,
            comments_by_path: HashMap::default(),
            fetched_key: None,
            _fetch_task: None,
            _subscriptions: vec![git_store_subscription, settings_subscription],
        };
        this.refresh(cx);
        this
    }

    pub fn active_repository(&self) -> Option<&Entity<Repository>> {
        self.active_repository.as_ref()
    }

    pub fn comments_for_file(&self, file_path: &str) -> &[PullRequestComment] {
        self.comments_by_path
            .get(file_path)
            .map_or(&[], Vec::as_slice)
    }

    fn refresh(&mut self, cx: &mut Context<Self>) {
        let Some(project) = self.project.upgrade() else {
            return;
        };

        if !ProjectSettings::get_global(cx)
            .git
            .pull_request
            .enable_inline_comments
        {
            self.active_repository = None;
            self._fetch_task = None;
            self.clear(cx);
            return;
        }

        let active_repository = project.read(cx).active_repository(cx);
        self.active_repository = active_repository.clone();

        let Some(repository) = active_repository else {
            self.clear(cx);
            return;
        };

        let repository_id = repository.entity_id();
        let repo = repository.read(cx);
        let branch_name = repo.branch.as_ref().map(|branch| branch.name().to_string());
        let head_sha = repo
            .head_commit
            .as_ref()
            .map(|commit| commit.sha.to_string());
        let remote_url = repo.default_remote_url();

        let (Some(branch_name), Some(remote_url)) = (branch_name, remote_url) else {
            self.clear(cx);
            return;
        };

        let key = FetchKey {
            repository: repository_id,
            branch: branch_name.clone(),
            head_sha,
        };
        if self.fetched_key.as_ref() == Some(&key) {
            return;
        }

        let Some((provider, remote)) =
            parse_git_remote_url(GitHostingProviderRegistry::global(cx), &remote_url)
        else {
            self.clear(cx);
            return;
        };

        let http_client = cx.http_client();
        self.fetched_key = Some(key);

        self._fetch_task = Some(cx.spawn(async move |this, cx| {
            let result: Result<Vec<PullRequestComment>> = async {
                let pull_request = provider
                    .pull_request_for_branch(
                        remote.owner.as_ref(),
                        remote.repo.as_ref(),
                        &branch_name,
                        http_client.clone(),
                    )
                    .await?;
                let Some(pull_request) = pull_request else {
                    return Ok(Vec::new());
                };
                provider
                    .pull_request_comments(
                        remote.owner.as_ref(),
                        remote.repo.as_ref(),
                        &pull_request.number.to_string(),
                        http_client,
                    )
                    .await
            }
            .await;

            this.update(cx, |this, cx| {
                match result {
                    Ok(comments) => {
                        this.comments_by_path.clear();
                        for comment in comments {
                            this.comments_by_path
                                .entry(comment.file_path.clone())
                                .or_default()
                                .push(comment);
                        }
                    }
                    Err(error) => {
                        log::error!("failed to fetch pull request comments: {error:#}");
                        this.fetched_key = None;
                        this.comments_by_path.clear();
                    }
                }
                cx.notify();
            })
            .ok();
        }));
    }

    fn clear(&mut self, cx: &mut Context<Self>) {
        let had_key = self.fetched_key.take().is_some();
        let had_comments = !self.comments_by_path.is_empty();
        self.comments_by_path.clear();
        if had_key || had_comments {
            cx.notify();
        }
    }
}

/// Test helpers shared with the editor-block tests in `inline_pull_request_comments`.
#[cfg(test)]
pub(crate) mod test_support {
    use anyhow::Result;
    use async_trait::async_trait;
    use fs::FakeFs;
    use git::{
        BuildCommitPermalinkParams, BuildPermalinkParams, GitHostingProvider,
        GitHostingProviderRegistry, ParsedGitRemote, PullRequest, PullRequestComment,
    };
    use gpui::{Entity, TestAppContext, UpdateGlobal as _, http_client::HttpClient};
    use project::Project;
    use serde_json::json;
    use settings::SettingsStore;
    use std::{
        path::Path,
        sync::{
            Arc,
            atomic::{AtomicUsize, Ordering},
        },
    };
    use time::OffsetDateTime;
    use url::Url;

    pub(crate) const REMOTE_URL: &str = "https://fake-git.test/zed-industries/zed.git";

    struct FakeGitHostingProvider {
        pull_request: Option<PullRequest>,
        comments: Vec<PullRequestComment>,
        pull_request_for_branch_calls: Arc<AtomicUsize>,
        pull_request_comments_calls: Arc<AtomicUsize>,
    }

    #[async_trait]
    impl GitHostingProvider for FakeGitHostingProvider {
        fn name(&self) -> String {
            "Fake".to_string()
        }

        fn base_url(&self) -> Url {
            Url::parse("https://fake-git.test").unwrap()
        }

        fn build_commit_permalink(
            &self,
            _remote: &ParsedGitRemote,
            _params: BuildCommitPermalinkParams,
        ) -> Url {
            self.base_url()
        }

        fn build_permalink(&self, _remote: ParsedGitRemote, _params: BuildPermalinkParams) -> Url {
            self.base_url()
        }

        fn supports_avatars(&self) -> bool {
            false
        }

        fn format_line_number(&self, line: u32) -> String {
            format!("L{line}")
        }

        fn format_line_numbers(&self, start_line: u32, end_line: u32) -> String {
            format!("L{start_line}-L{end_line}")
        }

        fn parse_remote_url(&self, url: &str) -> Option<ParsedGitRemote> {
            let path = url.strip_prefix("https://fake-git.test/")?;
            let path = path.strip_suffix(".git").unwrap_or(path);
            let (owner, repo) = path.split_once('/')?;
            Some(ParsedGitRemote {
                owner: owner.into(),
                repo: repo.into(),
            })
        }

        async fn pull_request_for_branch(
            &self,
            _repo_owner: &str,
            _repo: &str,
            _branch: &str,
            _client: Arc<dyn HttpClient>,
        ) -> Result<Option<PullRequest>> {
            self.pull_request_for_branch_calls
                .fetch_add(1, Ordering::SeqCst);
            Ok(self.pull_request.clone())
        }

        async fn pull_request_comments(
            &self,
            _repo_owner: &str,
            _repo: &str,
            _pull_request_id: &str,
            _client: Arc<dyn HttpClient>,
        ) -> Result<Vec<PullRequestComment>> {
            self.pull_request_comments_calls
                .fetch_add(1, Ordering::SeqCst);
            Ok(self.comments.clone())
        }
    }

    pub(crate) struct FakeProviderHandle {
        pub(crate) pull_request_for_branch_calls: Arc<AtomicUsize>,
        pub(crate) pull_request_comments_calls: Arc<AtomicUsize>,
    }

    pub(crate) fn comment(file_path: &str, line: Option<u32>) -> PullRequestComment {
        PullRequestComment {
            author_name: "octocat".to_string(),
            body: "looks good".to_string(),
            created_at: OffsetDateTime::UNIX_EPOCH,
            file_path: file_path.to_string(),
            line,
        }
    }

    pub(crate) fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            GitHostingProviderRegistry::set_global(Arc::new(GitHostingProviderRegistry::new()), cx);
        });
    }

    pub(crate) fn register_provider(
        pull_request: Option<PullRequest>,
        comments: Vec<PullRequestComment>,
        cx: &mut TestAppContext,
    ) -> FakeProviderHandle {
        let pull_request_for_branch_calls = Arc::new(AtomicUsize::new(0));
        let pull_request_comments_calls = Arc::new(AtomicUsize::new(0));
        let provider = Arc::new(FakeGitHostingProvider {
            pull_request,
            comments,
            pull_request_for_branch_calls: pull_request_for_branch_calls.clone(),
            pull_request_comments_calls: pull_request_comments_calls.clone(),
        });
        cx.update(|cx| {
            GitHostingProviderRegistry::global(cx).register_hosting_provider(provider);
        });
        FakeProviderHandle {
            pull_request_for_branch_calls,
            pull_request_comments_calls,
        }
    }

    pub(crate) fn set_enabled(enabled: bool, cx: &mut TestAppContext) {
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |content| {
                    content
                        .git
                        .get_or_insert_default()
                        .pull_request
                        .get_or_insert_default()
                        .enable_inline_comments = Some(enabled);
                });
            });
        });
    }

    pub(crate) async fn setup_project(cx: &mut TestAppContext) -> Entity<Project> {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            Path::new("/project"),
            json!({
                ".git": {},
                "src": {
                    "main.rs": "fn main() {\n    println!(\"hi\");\n    let x = 42;\n}\n",
                    "lib.rs": "pub fn lib() {}\n",
                },
            }),
        )
        .await;
        fs.set_branch_name(Path::new("/project/.git"), Some("feature-branch"));
        fs.set_remote_for_repo(Path::new("/project/.git"), "origin", REMOTE_URL);

        let project = Project::test(fs.clone(), [Path::new("/project")], cx).await;
        project
            .update(cx, |project, cx| project.git_scans_complete(cx))
            .await;
        cx.run_until_parked();
        project
    }

    pub(crate) fn pull_request() -> PullRequest {
        PullRequest {
            number: 42,
            url: Url::parse("https://fake-git.test/zed-industries/zed/pull/42").unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PullRequestStore;
    use super::test_support::*;
    use gpui::TestAppContext;
    use std::sync::atomic::Ordering;

    #[gpui::test]
    async fn test_groups_comments_by_file_path(cx: &mut TestAppContext) {
        init_test(cx);
        let handle = register_provider(
            Some(pull_request()),
            vec![
                comment("src/main.rs", Some(1)),
                comment("src/main.rs", Some(5)),
                comment("src/lib.rs", Some(1)),
            ],
            cx,
        );
        set_enabled(true, cx);
        let project = setup_project(cx).await;

        let store = cx.update(|cx| PullRequestStore::for_project(&project, cx));
        cx.run_until_parked();

        assert_eq!(
            handle.pull_request_for_branch_calls.load(Ordering::SeqCst),
            1
        );
        assert_eq!(handle.pull_request_comments_calls.load(Ordering::SeqCst), 1);
        store.read_with(cx, |store, _| {
            assert_eq!(store.comments_for_file("src/main.rs").len(), 2);
            assert_eq!(store.comments_for_file("src/lib.rs").len(), 1);
            assert!(store.comments_for_file("src/other.rs").is_empty());
        });
    }

    #[gpui::test]
    async fn test_does_not_fetch_when_disabled(cx: &mut TestAppContext) {
        init_test(cx);
        let handle = register_provider(
            Some(pull_request()),
            vec![comment("src/main.rs", Some(1))],
            cx,
        );
        // Setting is left at its default (disabled).
        let project = setup_project(cx).await;

        let store = cx.update(|cx| PullRequestStore::for_project(&project, cx));
        cx.run_until_parked();

        assert_eq!(
            handle.pull_request_for_branch_calls.load(Ordering::SeqCst),
            0
        );
        assert_eq!(handle.pull_request_comments_calls.load(Ordering::SeqCst), 0);
        store.read_with(cx, |store, _| {
            assert!(store.comments_for_file("src/main.rs").is_empty());
        });
    }

    #[gpui::test]
    async fn test_fetches_when_enabled_after_start(cx: &mut TestAppContext) {
        init_test(cx);
        let handle = register_provider(
            Some(pull_request()),
            vec![comment("src/main.rs", Some(1))],
            cx,
        );
        let project = setup_project(cx).await;

        let store = cx.update(|cx| PullRequestStore::for_project(&project, cx));
        cx.run_until_parked();
        assert_eq!(handle.pull_request_comments_calls.load(Ordering::SeqCst), 0);

        set_enabled(true, cx);
        cx.run_until_parked();

        assert_eq!(handle.pull_request_comments_calls.load(Ordering::SeqCst), 1);
        store.read_with(cx, |store, _| {
            assert_eq!(store.comments_for_file("src/main.rs").len(), 1);
        });
    }

    #[gpui::test]
    async fn test_clears_comments_when_disabled_after_fetch(cx: &mut TestAppContext) {
        init_test(cx);
        register_provider(
            Some(pull_request()),
            vec![comment("src/main.rs", Some(1))],
            cx,
        );
        set_enabled(true, cx);
        let project = setup_project(cx).await;

        let store = cx.update(|cx| PullRequestStore::for_project(&project, cx));
        cx.run_until_parked();
        store.read_with(cx, |store, _| {
            assert_eq!(store.comments_for_file("src/main.rs").len(), 1);
        });

        set_enabled(false, cx);
        cx.run_until_parked();
        store.read_with(cx, |store, _| {
            assert!(store.comments_for_file("src/main.rs").is_empty());
        });
    }

    #[gpui::test]
    async fn test_no_comments_when_no_pull_request(cx: &mut TestAppContext) {
        init_test(cx);
        let handle = register_provider(None, vec![comment("src/main.rs", Some(1))], cx);
        set_enabled(true, cx);
        let project = setup_project(cx).await;

        let store = cx.update(|cx| PullRequestStore::for_project(&project, cx));
        cx.run_until_parked();

        assert_eq!(
            handle.pull_request_for_branch_calls.load(Ordering::SeqCst),
            1
        );
        // With no matching pull request, comments are never requested.
        assert_eq!(handle.pull_request_comments_calls.load(Ordering::SeqCst), 0);
        store.read_with(cx, |store, _| {
            assert!(store.comments_for_file("src/main.rs").is_empty());
        });
    }
}
