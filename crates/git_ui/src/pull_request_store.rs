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
