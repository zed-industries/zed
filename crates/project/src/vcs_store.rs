//! Read-only external source-control repositories, scoped to opened worktrees.

use std::{collections::BTreeMap, path::Path, sync::Arc, time::Duration};

use anyhow::{Context as _, Result, anyhow, bail, ensure};
use buffer_diff::{BufferDiff, DiffOperations};
use collections::HashMap;
use extension::{Extension, ExtensionHostProxy, ExtensionVcsProviderProxy, WorktreeDelegate};
use fs::Fs;
use gpui::{
    App, AppContext, Context, Entity, EventEmitter, Global, ReadGlobal, SharedString, Subscription,
    Task, TaskExt, WeakEntity,
};
use language::{Buffer, BufferEvent};
use settings::WorktreeId;
use util::{ResultExt, rel_path::RelPath};
use vcs::{
    ChangeStatus, Content, ProviderCommand, ProviderConnection, ProviderNotification,
    RepositoryDescriptor, Snapshot,
};

use crate::{
    Project, ProjectEnvironment, ProjectItem, ProjectPath,
    git_store::GitStore,
    trusted_worktrees::TrustedWorktrees,
    worktree_store::{WorktreeStore, WorktreeStoreEvent},
};

#[derive(Clone)]
struct Registration {
    extension: Arc<dyn Extension>,
    provider_id: Arc<str>,
}

type ProviderKey = (Arc<str>, Arc<str>);
type SessionKey = (WorktreeId, ProviderKey);

#[derive(Default)]
struct Registry {
    registrations: BTreeMap<ProviderKey, Registration>,
}
impl EventEmitter<()> for Registry {}
struct GlobalRegistry(Entity<Registry>);
impl Global for GlobalRegistry {}

struct RegistryProxy(Entity<Registry>);
impl ExtensionVcsProviderProxy for RegistryProxy {
    fn register_vcs_provider(
        &self,
        extension: Arc<dyn Extension>,
        provider_id: Arc<str>,
        cx: &mut App,
    ) {
        let key = (extension.manifest().id.clone(), provider_id.clone());
        log::info!(
            "Registered VCS provider: extension={}, provider={}",
            key.0,
            key.1
        );
        self.0.update(cx, |registry, cx| {
            registry.registrations.insert(
                key,
                Registration {
                    extension,
                    provider_id,
                },
            );
            cx.emit(());
        });
    }

    fn unregister_vcs_provider(&self, extension_id: Arc<str>, provider_id: Arc<str>, cx: &mut App) {
        log::info!("Unregistered VCS provider: extension={extension_id}, provider={provider_id}");
        self.0.update(cx, |registry, cx| {
            registry.registrations.remove(&(extension_id, provider_id));
            cx.emit(());
        });
    }
}

pub fn init(cx: &mut App) {
    if cx.try_global::<GlobalRegistry>().is_none() {
        let registry = cx.new(|_| Registry::default());
        ExtensionHostProxy::default_global(cx)
            .register_vcs_provider_proxy(RegistryProxy(registry.clone()));
        cx.set_global(GlobalRegistry(registry));
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct VcsRepositoryId {
    pub worktree_id: WorktreeId,
    pub extension_id: Arc<str>,
    pub provider_id: Arc<str>,
    pub repository_id: String,
}

#[derive(Clone, Debug)]
pub struct VcsRepositorySnapshot {
    pub key: VcsRepositoryId,
    pub descriptor: RepositoryDescriptor,
    pub selected_baseline: String,
    pub snapshot: Option<Arc<Snapshot>>,
    pub loading: bool,
    pub error: Option<SharedString>,
}

#[derive(Clone, Debug)]
pub enum VcsStoreEvent {
    Updated,
    DiffChanged { worktree_id: WorktreeId },
}

pub struct VcsDiff {
    pub buffer: Entity<Buffer>,
    pub diff: Entity<BufferDiff>,
}

struct Repository {
    view: VcsRepositorySnapshot,
    connection: Arc<ProviderConnection>,
    generation: u64,
    refresh_task: Option<Task<()>>,
}

struct Session {
    generation: u64,
    claims_scope: bool,
    connection: Option<Arc<ProviderConnection>>,
    _start_task: Task<()>,
    notification_task: Option<Task<()>>,
}

struct DiffState {
    repository: VcsRepositoryId,
    path: String,
    buffer: WeakEntity<Buffer>,
    diff: Entity<BufferDiff>,
    base: Option<Arc<str>>,
    base_loaded: bool,
    generation: u64,
    update_task: Option<Task<()>>,
    _subscription: Subscription,
}

struct ReadOnlyDiffOperations;
impl DiffOperations for ReadOnlyDiffOperations {
    fn supports_staging(&self) -> bool {
        false
    }
    fn supports_unstaging(&self) -> bool {
        false
    }
    fn supports_restore(&self) -> bool {
        false
    }
}

pub struct VcsStore {
    project: WeakEntity<Project>,
    worktrees: Entity<WorktreeStore>,
    git: Entity<GitStore>,
    environment: Entity<ProjectEnvironment>,
    fs: Arc<dyn Fs>,
    sessions: HashMap<SessionKey, Session>,
    repositories: HashMap<VcsRepositoryId, Repository>,
    selected_baselines: HashMap<VcsRepositoryId, String>,
    diffs: HashMap<text::BufferId, DiffState>,
    generation: u64,
    discovery_task: Option<Task<()>>,
    poll_task: Option<Task<()>>,
    errors: HashMap<SessionKey, SharedString>,
    _subscriptions: Vec<Subscription>,
}

impl EventEmitter<VcsStoreEvent> for VcsStore {}

impl VcsStore {
    pub fn new(
        project: WeakEntity<Project>,
        worktrees: Entity<WorktreeStore>,
        git: Entity<GitStore>,
        environment: Entity<ProjectEnvironment>,
        fs: Arc<dyn Fs>,
        cx: &mut Context<Self>,
    ) -> Self {
        init(cx);
        let registry = GlobalRegistry::global(cx).0.clone();
        let mut subscriptions = vec![
            cx.subscribe(&registry, |this, _, _, cx| {
                this.stop_all(cx);
                this.schedule_discovery(cx);
            }),
            cx.subscribe(&worktrees, |this, _, event, cx| match event {
                WorktreeStoreEvent::WorktreeAdded(_)
                | WorktreeStoreEvent::WorktreeUpdatedGitRepositories(_, _) => {
                    this.schedule_discovery(cx)
                }
                WorktreeStoreEvent::WorktreeRemoved(_, worktree_id)
                | WorktreeStoreEvent::WorktreeReleased(_, worktree_id) => {
                    this.selected_baselines
                        .retain(|key, _| key.worktree_id != *worktree_id);
                    this.schedule_discovery(cx);
                }
                WorktreeStoreEvent::WorktreeUpdatedEntries(worktree_id, _) => {
                    let keys: Vec<_> = this
                        .repositories
                        .keys()
                        .filter(|key| key.worktree_id == *worktree_id)
                        .cloned()
                        .collect();
                    for key in keys {
                        this.refresh(&key, cx);
                    }
                    if !this
                        .repositories
                        .keys()
                        .any(|key| key.worktree_id == *worktree_id)
                    {
                        this.schedule_discovery(cx);
                    }
                }
                _ => {}
            }),
        ];
        if let Some(trust) = TrustedWorktrees::try_get_global(cx) {
            subscriptions.push(cx.subscribe(&trust, |this, _, _, cx| this.schedule_discovery(cx)));
        }
        let mut store = Self {
            project,
            worktrees,
            git,
            environment,
            fs,
            sessions: HashMap::default(),
            repositories: HashMap::default(),
            selected_baselines: HashMap::default(),
            diffs: HashMap::default(),
            generation: 0,
            discovery_task: None,
            poll_task: None,
            errors: HashMap::default(),
            _subscriptions: subscriptions,
        };
        store.schedule_discovery(cx);
        store
    }

    pub fn repositories(&self) -> Vec<VcsRepositorySnapshot> {
        let mut repositories: Vec<_> = self
            .repositories
            .values()
            .map(|repository| repository.view.clone())
            .collect();
        repositories.sort_by(|left, right| {
            left.descriptor
                .scope_uri
                .cmp(&right.descriptor.scope_uri)
                .then_with(|| {
                    left.descriptor
                        .display_name
                        .cmp(&right.descriptor.display_name)
                })
        });
        repositories
    }

    pub fn errors(&self) -> Vec<SharedString> {
        self.errors.values().cloned().collect()
    }

    pub fn diagnostics(&self) -> Vec<SharedString> {
        self.sessions
            .values()
            .filter_map(|session| session.connection.as_ref())
            .map(|connection| connection.diagnostics())
            .filter(|message| !message.is_empty())
            .map(Into::into)
            .collect()
    }

    fn notify(&self, cx: &mut Context<Self>) {
        cx.emit(VcsStoreEvent::Updated);
        cx.notify();
    }

    fn schedule_discovery(&mut self, cx: &mut Context<Self>) {
        self.discovery_task = Some(cx.spawn(async move |this, cx| {
            cx.background_executor()
                .timer(Duration::from_millis(100))
                .await;
            this.update(cx, |this, cx| this.discover(cx)).log_err();
        }));
    }

    fn discover(&mut self, cx: &mut Context<Self>) {
        let registrations = GlobalRegistry::global(cx).0.read(cx).registrations.clone();
        let roots: Vec<_> = self
            .worktrees
            .read(cx)
            .visible_worktrees(cx)
            .map(|worktree| {
                let worktree = worktree.read(cx);
                (worktree.id(), worktree.abs_path())
            })
            .collect();
        let mut desired = std::collections::HashSet::new();
        for (worktree_id, root) in roots {
            let root_path = ProjectPath {
                worktree_id,
                path: Arc::from(RelPath::empty()),
            };
            if self
                .git
                .read(cx)
                .repository_and_path_for_project_path(&root_path, cx)
                .is_some()
                || !self.trusted(worktree_id, cx)
            {
                continue;
            }
            for (provider, registration) in &registrations {
                let key = (worktree_id, provider.clone());
                desired.insert(key.clone());
                if !self.sessions.contains_key(&key) {
                    self.start_session(key, root.clone(), registration.clone(), cx);
                }
            }
        }
        let obsolete: Vec<_> = self
            .sessions
            .keys()
            .filter(|key| !desired.contains(*key))
            .cloned()
            .collect();
        for key in obsolete {
            self.stop_session(&key, cx);
        }
    }

    fn trusted(&self, worktree_id: WorktreeId, cx: &mut App) -> bool {
        TrustedWorktrees::try_get_global(cx).is_some_and(|trust| {
            trust.update(cx, |trust, cx| {
                trust.can_trust(&self.worktrees, worktree_id, cx)
            })
        })
    }

    fn start_session(
        &mut self,
        key: SessionKey,
        root: Arc<Path>,
        registration: Registration,
        cx: &mut Context<Self>,
    ) {
        self.generation += 1;
        let generation = self.generation;
        let fs = self.fs.clone();
        let environment = self.environment.clone();
        let task_key = key.clone();
        let task = cx.spawn(async move |this, cx| {
            let result: Result<Option<(Arc<ProviderConnection>, Vec<RepositoryDescriptor>)>> =
                async {
                    let markers = registration
                        .extension
                        .manifest()
                        .vcs_providers
                        .get(&registration.provider_id)
                        .context("VCS provider was unloaded")?
                        .root_markers
                        .clone();
                    if !matches_markers(&root, &markers, &fs).await? {
                        return Ok(None);
                    }
                    this.update(cx, |this, _| {
                        if let Some(session) = this.sessions.get_mut(&task_key) {
                            session.claims_scope = true;
                        }
                    })?;
                    let shell_env = environment
                        .update(cx, |environment, cx| {
                            environment.directory_environment(root.clone(), cx)
                        })
                        .await
                        .unwrap_or_default();
                    ensure!(
                        this.update(cx, |this, cx| this.trusted(task_key.0, cx))?,
                        "VCS worktree is not trusted"
                    );
                    let delegate = Arc::new(ProviderWorktree {
                        id: task_key.0,
                        root: root.clone(),
                        fs,
                        environment: shell_env.clone(),
                    });
                    let command = registration
                        .extension
                        .vcs_provider_command(registration.provider_id, delegate)
                        .await?;
                    ensure!(
                        this.update(cx, |this, cx| this.trusted(task_key.0, cx))?,
                        "VCS worktree trust was revoked"
                    );
                    let mut environment: HashMap<_, _> = shell_env.into_iter().collect();
                    environment.extend(command.env);
                    let command = ProviderCommand {
                        path: registration.extension.path_from_extension(&command.command),
                        arguments: command.args.into_iter().map(Into::into).collect(),
                        env: environment,
                    };
                    let connection = ProviderConnection::start(
                        command,
                        &root,
                        cx.background_executor().clone(),
                    )?;
                    connection.initialize().await?.validate()?;
                    let scope = url::Url::from_directory_path(&root)
                        .map_err(|()| anyhow!("invalid VCS scope"))?
                        .to_string();
                    let repositories = connection.discover(scope.clone()).await?;
                    for repository in &repositories {
                        repository.validate(&scope)?;
                    }
                    ensure!(
                        repositories.len() <= 1,
                        "multiple VCS repositories claim the same opened scope"
                    );
                    Ok(Some((connection, repositories)))
                }
                .await;
            this.update(cx, |this, cx| {
                if this
                    .sessions
                    .get(&task_key)
                    .is_none_or(|session| session.generation != generation)
                {
                    return;
                }
                match result {
                    Ok(Some((connection, repositories))) => {
                        this.install_session(task_key.clone(), connection, repositories, cx)
                    }
                    Ok(None) => {
                        this.sessions.remove(&task_key);
                    }
                    Err(error) => {
                        this.errors
                            .insert(task_key.clone(), format!("VCS provider: {error:#}").into());
                        this.notify(cx);
                    }
                }
            })
            .log_err();
        });
        self.sessions.insert(
            key,
            Session {
                generation,
                claims_scope: false,
                connection: None,
                _start_task: task,
                notification_task: None,
            },
        );
    }

    fn install_session(
        &mut self,
        key: SessionKey,
        connection: Arc<ProviderConnection>,
        descriptors: Vec<RepositoryDescriptor>,
        cx: &mut Context<Self>,
    ) {
        log::info!(
            "Connected VCS provider: extension={}, provider={}, repositories={}",
            key.1.0,
            key.1.1,
            descriptors.len()
        );
        let notifications = connection.notifications();
        let notification_key = key.clone();
        let generation = self
            .sessions
            .get(&key)
            .map_or(0, |session| session.generation);
        let notification_task = cx.spawn(async move |this, cx| {
            let mut reported_exit = false;
            while let Ok(notification) = notifications.recv().await {
                let exited = matches!(&notification, ProviderNotification::Exited { .. });
                if this
                    .update(cx, |this, cx| match notification {
                        ProviderNotification::DidChange { repository_id } => {
                            let keys: Vec<_> = this
                                .repositories
                                .keys()
                                .filter(|repository| {
                                    repository.worktree_id == notification_key.0
                                        && repository.extension_id == notification_key.1.0
                                        && repository.provider_id == notification_key.1.1
                                        && repository.repository_id == repository_id
                                })
                                .cloned()
                                .collect();
                            for key in keys {
                                this.refresh(&key, cx);
                            }
                        }
                        ProviderNotification::Exited { message } => {
                            this.disconnected(&notification_key, generation, message, cx);
                        }
                    })
                    .is_err()
                {
                    break;
                }
                if exited {
                    reported_exit = true;
                    break;
                }
            }
            if !reported_exit {
                this.update(cx, |this, cx| {
                    this.disconnected(
                        &notification_key,
                        generation,
                        "VCS provider notification stream closed; refresh to reconnect".into(),
                        cx,
                    )
                })
                .log_err();
            }
        });
        if let Some(session) = self.sessions.get_mut(&key) {
            session.connection = Some(connection.clone());
            session.notification_task = Some(notification_task);
        }
        self.errors.remove(&key);
        for descriptor in descriptors {
            let Some(baseline) = descriptor.baselines.first() else {
                continue;
            };
            let id = VcsRepositoryId {
                worktree_id: key.0,
                extension_id: key.1.0.clone(),
                provider_id: key.1.1.clone(),
                repository_id: descriptor.id.clone(),
            };
            let selected_baseline = self
                .selected_baselines
                .get(&id)
                .filter(|selected| {
                    descriptor
                        .baselines
                        .iter()
                        .any(|baseline| &baseline.id == *selected)
                })
                .cloned()
                .unwrap_or_else(|| baseline.id.clone());
            self.repositories.insert(
                id.clone(),
                Repository {
                    view: VcsRepositorySnapshot {
                        key: id.clone(),
                        selected_baseline,
                        descriptor,
                        snapshot: None,
                        loading: false,
                        error: None,
                    },
                    connection: connection.clone(),
                    generation: 0,
                    refresh_task: None,
                },
            );
            self.refresh(&id, cx);
        }
        let owners: Vec<_> = self
            .repositories
            .keys()
            .filter(|repository| repository.worktree_id == key.0)
            .cloned()
            .collect();
        if owners.len() > 1 {
            for owner in owners {
                if let Some(repository) = self.repositories.get_mut(&owner) {
                    repository.generation += 1;
                    repository.refresh_task = None;
                    repository.view.snapshot = None;
                    repository.view.loading = false;
                    repository.view.error = Some(
                        "Multiple VCS providers claim this folder; disable one provider".into(),
                    );
                }
                self.invalidate_diffs(&owner, cx);
            }
        }
        self.ensure_polling(cx);
        self.notify(cx);
        cx.emit(VcsStoreEvent::DiffChanged { worktree_id: key.0 });
    }

    fn disconnected(
        &mut self,
        key: &SessionKey,
        generation: u64,
        message: String,
        cx: &mut Context<Self>,
    ) {
        if self
            .sessions
            .get(key)
            .is_none_or(|session| session.generation != generation)
        {
            return;
        }
        log::warn!(
            "VCS provider disconnected: extension={}, provider={}; see Source Control diagnostics",
            key.1.0,
            key.1.1
        );
        self.errors.insert(key.clone(), message.clone().into());
        let repositories: Vec<_> = self
            .repositories
            .keys()
            .filter(|repository| {
                repository.worktree_id == key.0
                    && repository.extension_id == key.1.0
                    && repository.provider_id == key.1.1
            })
            .cloned()
            .collect();
        for repository in repositories {
            if let Some(state) = self.repositories.get_mut(&repository) {
                state.generation += 1;
                state.view.loading = false;
                state.view.error = Some(message.clone().into());
                state.view.snapshot = None;
                state.refresh_task = None;
            }
            self.invalidate_diffs(&repository, cx);
        }
        self.notify(cx);
    }

    fn stop_session(&mut self, key: &SessionKey, cx: &mut Context<Self>) {
        if let Some(session) = self.sessions.remove(key) {
            if let Some(connection) = session.connection {
                cx.background_spawn(async move {
                    connection.shutdown().await.log_err();
                })
                .detach();
            }
        }
        self.errors.remove(key);
        let repositories: Vec<_> = self
            .repositories
            .keys()
            .filter(|repository| {
                repository.worktree_id == key.0
                    && repository.extension_id == key.1.0
                    && repository.provider_id == key.1.1
            })
            .cloned()
            .collect();
        for repository in repositories {
            self.invalidate_diffs(&repository, cx);
            self.repositories.remove(&repository);
        }
        if self.repositories.is_empty() {
            self.poll_task = None;
        }
        self.notify(cx);
        cx.emit(VcsStoreEvent::DiffChanged { worktree_id: key.0 });
    }

    fn stop_all(&mut self, cx: &mut Context<Self>) {
        let keys: Vec<_> = self.sessions.keys().cloned().collect();
        for key in keys {
            self.stop_session(&key, cx);
        }
    }

    pub fn refresh_all(&mut self, cx: &mut Context<Self>) {
        let failed: Vec<_> = self
            .sessions
            .iter()
            .filter(|(key, session)| {
                session
                    .connection
                    .as_ref()
                    .is_some_and(|connection| !connection.is_alive())
                    || self.errors.contains_key(*key)
            })
            .map(|(key, _)| key.clone())
            .collect();
        for key in &failed {
            self.stop_session(key, cx);
        }
        let healthy: Vec<_> = self.repositories.keys().cloned().collect();
        for key in healthy {
            self.refresh(&key, cx);
        }
        if !failed.is_empty() || self.repositories.is_empty() {
            self.schedule_discovery(cx);
        }
    }

    pub fn set_baseline(
        &mut self,
        key: &VcsRepositoryId,
        baseline: String,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        ensure!(
            self.repositories
                .keys()
                .filter(|candidate| candidate.worktree_id == key.worktree_id)
                .count()
                == 1,
            "VCS ownership is ambiguous"
        );
        let repository = self
            .repositories
            .get_mut(key)
            .context("VCS repository is unavailable")?;
        ensure!(
            repository
                .view
                .descriptor
                .baselines
                .iter()
                .any(|entry| entry.id == baseline),
            "unknown VCS baseline"
        );
        repository.view.selected_baseline = baseline.clone();
        self.selected_baselines.insert(key.clone(), baseline);
        repository.view.snapshot = None;
        self.invalidate_diffs(key, cx);
        self.refresh(key, cx);
        Ok(())
    }

    pub fn refresh(&mut self, key: &VcsRepositoryId, cx: &mut Context<Self>) {
        if self
            .repositories
            .keys()
            .filter(|candidate| candidate.worktree_id == key.worktree_id)
            .count()
            != 1
        {
            return;
        }
        let Some(repository) = self.repositories.get_mut(key) else {
            return;
        };
        if !repository.connection.is_alive() {
            self.refresh_all(cx);
            return;
        }
        repository.generation += 1;
        let generation = repository.generation;
        let baseline = repository.view.selected_baseline.clone();
        let connection = repository.connection.clone();
        let key = key.clone();
        repository.view.loading = true;
        repository.refresh_task = Some(cx.spawn(async move |this, cx| {
            cx.background_executor()
                .timer(Duration::from_millis(150))
                .await;
            let result = connection
                .snapshot(key.repository_id.clone(), baseline.clone())
                .await;
            this.update(cx, |this, cx| {
                let Some(repository) = this.repositories.get_mut(&key) else {
                    return;
                };
                if repository.generation != generation
                    || repository.view.selected_baseline != baseline
                {
                    return;
                }
                repository.view.loading = false;
                let diff_changed = match result.and_then(|snapshot| {
                    snapshot.validate(&key.repository_id, &baseline)?;
                    Ok(snapshot)
                }) {
                    Ok(snapshot) => {
                        // A new snapshot token need not change immutable baseline content.
                        let changed = repository.view.error.is_some()
                            || repository.view.snapshot.as_ref().is_none_or(|previous| {
                                previous.baseline_id != snapshot.baseline_id
                                    || previous.revision_id != snapshot.revision_id
                                    || previous.changes != snapshot.changes
                            });
                        repository.view.snapshot = Some(Arc::new(snapshot));
                        repository.view.error = None;
                        if changed {
                            let buffers: Vec<_> = this
                                .diffs
                                .iter()
                                .filter(|(_, state)| state.repository == key)
                                .filter_map(|(_, state)| state.buffer.upgrade())
                                .collect();
                            for buffer in buffers {
                                this.reload_diff(buffer, cx).detach_and_log_err(cx);
                            }
                        }
                        changed
                    }
                    Err(error) => {
                        repository.view.error = Some(format!("{error:#}").into());
                        this.invalidate_diffs(&key, cx);
                        true
                    }
                };
                this.notify(cx);
                if diff_changed {
                    cx.emit(VcsStoreEvent::DiffChanged {
                        worktree_id: key.worktree_id,
                    });
                }
            })
            .log_err();
        }));
        self.notify(cx);
    }

    fn ensure_polling(&mut self, cx: &mut Context<Self>) {
        if self.poll_task.is_some() || self.repositories.is_empty() {
            return;
        }
        self.poll_task = Some(cx.spawn(async move |this, cx| {
            loop {
                cx.background_executor().timer(Duration::from_secs(5)).await;
                if this.update(cx, |this, cx| this.poll(cx)).is_err() {
                    break;
                }
            }
        }));
    }

    fn poll(&mut self, cx: &mut Context<Self>) {
        let keys: Vec<_> = self
            .repositories
            .iter()
            .filter(|(_, repository)| !repository.view.loading && repository.connection.is_alive())
            .map(|(key, _)| key.clone())
            .collect();
        for key in keys {
            self.refresh(&key, cx);
        }
    }

    pub fn repository_for_path(&self, path: &ProjectPath, cx: &App) -> Option<VcsRepositoryId> {
        if self
            .git
            .read(cx)
            .repository_and_path_for_project_path(path, cx)
            .is_some()
        {
            return None;
        }
        let mut matches = self
            .repositories
            .keys()
            .filter(|key| key.worktree_id == path.worktree_id);
        let key = matches.next()?.clone();
        if matches.next().is_some() {
            return None;
        }
        Some(key)
    }

    pub fn owns_project_path(&self, path: &ProjectPath, cx: &App) -> bool {
        self.repository_for_path(path, cx).is_some()
    }

    pub fn changes(&self, key: &VcsRepositoryId, cx: &App) -> Vec<vcs::Change> {
        self.repositories
            .get(key)
            .and_then(|repository| repository.view.snapshot.as_ref())
            .into_iter()
            .flat_map(|snapshot| snapshot.changes.iter())
            .filter(|change| {
                RelPath::from_unix_str(&change.path)
                    .ok()
                    .is_some_and(|path| {
                        self.repository_for_path(
                            &ProjectPath {
                                worktree_id: key.worktree_id,
                                path: Arc::from(path),
                            },
                            cx,
                        )
                        .as_ref()
                            == Some(key)
                    })
            })
            .cloned()
            .collect()
    }

    pub fn owns_absolute_path(&self, path: &Path, cx: &App) -> bool {
        self.sessions
            .iter()
            .filter(|(_, session)| session.claims_scope)
            .any(|((worktree_id, _), _)| {
                self.worktrees
                    .read(cx)
                    .worktree_for_id(*worktree_id, cx)
                    .is_some_and(|worktree| {
                        let root = worktree.read(cx).abs_path();
                        let Ok(relative) = path.strip_prefix(&root) else {
                            return false;
                        };
                        let Ok(relative) = RelPath::new(relative, worktree.read(cx).path_style())
                        else {
                            return false;
                        };
                        self.git
                            .read(cx)
                            .repository_and_path_for_project_path(
                                &ProjectPath {
                                    worktree_id: *worktree_id,
                                    path: relative.into(),
                                },
                                cx,
                            )
                            .is_none()
                    })
            })
    }

    pub fn check_git_initialization(&self, path: Arc<Path>, cx: &App) -> Task<Result<()>> {
        let markers: Vec<Vec<String>> = GlobalRegistry::global(cx)
            .0
            .read(cx)
            .registrations
            .values()
            .filter_map(|registration| {
                registration
                    .extension
                    .manifest()
                    .vcs_providers
                    .get(&registration.provider_id)
                    .map(|provider| provider.root_markers.clone())
            })
            .collect();
        if markers.is_empty() {
            return Task::ready(Ok(()));
        }
        let fs = self.fs.clone();
        cx.background_executor().spawn(async move {
            for ancestor in path.ancestors() {
                if fs.metadata(&ancestor.join(".git")).await?.is_some() { return Ok(()); }
                for provider_markers in &markers {
                    for marker in provider_markers {
                        ensure!(!marker.is_empty() && marker != "." && marker != ".." && !marker.contains(['/', '\\', '\0']), "invalid VCS repository marker");
                        if fs.metadata(&ancestor.join(marker)).await?.is_some() {
                            bail!("The folder has an external VCS repository marker; Git initialization is unavailable");
                        }
                    }
                }
            }
            Ok(())
        })
    }

    /// The Git-shaped summary is only a paint adapter for the existing tree.
    /// It does not create an index, Git repository, or mutation capability.
    pub fn summary_for_path(
        &self,
        path: &ProjectPath,
        cx: &App,
    ) -> Option<git::status::GitSummary> {
        let key = self.repository_for_path(path, cx)?;
        let repository = self.repositories.get(&key)?;
        let snapshot = repository.view.snapshot.as_ref()?;
        if repository.view.error.is_some() {
            return None;
        }
        let prefix = path.path.as_unix_str();
        let mut summary = git::status::GitSummary::default();
        for change in &snapshot.changes {
            if !prefix.is_empty()
                && change.path != prefix
                && !change
                    .path
                    .strip_prefix(prefix)
                    .is_some_and(|suffix| suffix.starts_with('/'))
            {
                continue;
            }
            let Ok(relative) = RelPath::from_unix_str(&change.path) else {
                continue;
            };
            let change_path = ProjectPath {
                worktree_id: key.worktree_id,
                path: Arc::from(relative),
            };
            if self
                .git
                .read(cx)
                .repository_and_path_for_project_path(&change_path, cx)
                .is_some()
            {
                continue;
            }
            summary.count += 1;
            match change.status {
                ChangeStatus::Added | ChangeStatus::Untracked => summary.untracked += 1,
                ChangeStatus::Deleted => summary.worktree.deleted += 1,
                ChangeStatus::Modified | ChangeStatus::Renamed => summary.worktree.modified += 1,
            }
        }
        Some(summary)
    }

    pub fn open_buffer_diff(
        &mut self,
        buffer: Entity<Buffer>,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<BufferDiff>>> {
        let Some(path) = buffer.read(cx).project_path(cx) else {
            return Task::ready(Err(anyhow!("buffer has no VCS path")));
        };
        let Some(key) = self.repository_for_path(&path, cx) else {
            return Task::ready(Err(anyhow!("buffer has no unambiguous VCS provider")));
        };
        let relative = path.path.as_unix_str().to_owned();
        self.ensure_diff(&key, &relative, &buffer, cx);
        self.load_diff_if_needed(buffer, cx)
    }

    pub fn open_diff(
        &mut self,
        key: VcsRepositoryId,
        path: String,
        cx: &mut Context<Self>,
    ) -> Task<Result<VcsDiff>> {
        let result = (|| {
            vcs::validate_path(&path)?;
            let project_path = ProjectPath {
                worktree_id: key.worktree_id,
                path: Arc::from(RelPath::from_unix_str(&path)?),
            };
            ensure!(
                self.repository_for_path(&project_path, cx).as_ref() == Some(&key),
                "file belongs to another source-control provider"
            );
            let root = self
                .worktrees
                .read(cx)
                .worktree_for_id(key.worktree_id, cx)
                .context("VCS worktree closed")?
                .read(cx)
                .abs_path();
            Ok((project_path, root))
        })();
        let (project_path, root) = match result {
            Ok(result) => result,
            Err(error) => return Task::ready(Err(error)),
        };
        let project = self.project.clone();
        let fs = self.fs.clone();
        cx.spawn(async move |this, cx| {
            let buffer = match scoped_metadata(&root, &path, &fs).await? {
                Some(metadata) => {
                    ensure!(!metadata.is_dir && !metadata.is_symlink, "binary, directory and symlink changes are displayed in the changes list without a text diff");
                    project.update(cx, |project, cx| project.open_buffer(project_path, cx))?.await?
                }
                None => project.update(cx, |project, cx| {
                    let buffer = project.create_local_buffer("", None, false, cx);
                    buffer.update(cx, |buffer, cx| buffer.set_capability(language::Capability::ReadOnly, cx));
                    buffer
                })?,
            };
            let diff_task = this.update(cx, |this, cx| {
                this.ensure_diff(&key, &path, &buffer, cx);
                this.load_diff_if_needed(buffer.clone(), cx)
            })?;
            let diff = diff_task.await?;
            Ok(VcsDiff { buffer, diff })
        })
    }

    fn ensure_diff(
        &mut self,
        key: &VcsRepositoryId,
        path: &str,
        buffer: &Entity<Buffer>,
        cx: &mut Context<Self>,
    ) {
        let buffer_id = buffer.read(cx).remote_id();
        if self
            .diffs
            .get(&buffer_id)
            .is_some_and(|state| state.repository == *key && state.path == path)
        {
            return;
        }
        let snapshot = buffer.read(cx).snapshot();
        let language_registry = buffer.read(cx).language_registry();
        let diff = cx.new(|cx| {
            let mut diff = BufferDiff::new(
                &snapshot.text,
                snapshot.language().cloned(),
                language_registry,
                cx,
            );
            diff.set_operations(Arc::new(ReadOnlyDiffOperations));
            diff
        });
        let subscription = cx.subscribe(buffer, |this, buffer, event, cx| {
            if matches!(
                event,
                BufferEvent::Edited { .. } | BufferEvent::Reloaded | BufferEvent::FileHandleChanged
            ) {
                let buffer_id = buffer.read(cx).remote_id();
                if this
                    .diffs
                    .get(&buffer_id)
                    .is_some_and(|state| !state.base_loaded)
                {
                    this.reload_diff(buffer, cx).detach_and_log_err(cx);
                    return;
                }
                if let Some(state) = this.diffs.get_mut(&buffer_id) {
                    state.generation += 1;
                    state.update_task = None;
                    let base = state.base.clone();
                    let snapshot = buffer.read(cx).text_snapshot();
                    let task = state
                        .diff
                        .update(cx, |diff, cx| diff.set_base_text(base, snapshot, cx));
                    state.update_task = Some(task);
                }
            }
        });
        self.diffs.insert(
            buffer_id,
            DiffState {
                repository: key.clone(),
                path: path.to_owned(),
                buffer: buffer.downgrade(),
                diff,
                base: None,
                base_loaded: false,
                generation: 0,
                update_task: None,
                _subscription: subscription,
            },
        );
    }

    fn load_diff_if_needed(
        &mut self,
        buffer: Entity<Buffer>,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<BufferDiff>>> {
        let buffer_id = buffer.read(cx).remote_id();
        if let Some(state) = self.diffs.get(&buffer_id)
            && state.base_loaded
        {
            return Task::ready(Ok(state.diff.clone()));
        }
        self.reload_diff(buffer, cx)
    }

    fn reload_diff(
        &mut self,
        buffer: Entity<Buffer>,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<BufferDiff>>> {
        let buffer_id = buffer.read(cx).remote_id();
        let Some(state) = self.diffs.get_mut(&buffer_id) else {
            return Task::ready(Err(anyhow!("VCS diff is unavailable")));
        };
        let Some(repository) = self.repositories.get(&state.repository) else {
            return Task::ready(Err(anyhow!("VCS repository closed")));
        };
        let Some(snapshot) = repository.view.snapshot.clone() else {
            return Task::ready(Err(anyhow!("VCS snapshot is loading")));
        };
        if let Some(error) = &repository.view.error {
            return Task::ready(Err(anyhow!("{error}")));
        }
        let connection = repository.connection.clone();
        let root = match vcs::local_path(&repository.view.descriptor.scope_uri) {
            Ok(root) => root,
            Err(error) => return Task::ready(Err(error)),
        };
        let fs = self.fs.clone();
        let path = state.path.clone();
        state.base_loaded = false;
        state.generation += 1;
        let generation = state.generation;
        state.update_task = None;
        let (sender, receiver) = futures::channel::oneshot::channel();
        state.update_task = Some(cx.spawn(async move |this, cx| {
            let result: Result<Entity<BufferDiff>> = async {
                scoped_metadata(&root, &path, &fs).await?;
                let content = connection
                    .read_content(
                        snapshot.repository_id.clone(),
                        snapshot.id.clone(),
                        snapshot.baseline_id.clone(),
                        path,
                    )
                    .await?;
                let base = match content {
                    Content::Absent => None,
                    Content::Text { mut text } => {
                        text::LineEnding::normalize(&mut text);
                        Some(Arc::<str>::from(text))
                    }
                    Content::Binary => {
                        bail!("Binary file: content is available only through its VCS provider")
                    }
                    Content::Symlink { .. } => {
                        bail!("Symlink: target changes are displayed in the changes list")
                    }
                };
                let (diff, update) = this.update(cx, |this, cx| -> Result<_> {
                    let state = this.diffs.get_mut(&buffer_id).context("VCS diff closed")?;
                    ensure!(
                        state.generation == generation,
                        "VCS diff request was superseded"
                    );
                    let repository = this
                        .repositories
                        .get(&state.repository)
                        .context("VCS repository closed")?;
                    ensure!(
                        repository
                            .view
                            .snapshot
                            .as_ref()
                            .is_some_and(|current| current.revision_id == snapshot.revision_id
                                && current.baseline_id == snapshot.baseline_id),
                        "VCS snapshot changed"
                    );
                    state.base = base.clone();
                    state.base_loaded = true;
                    let current = buffer.read(cx).text_snapshot();
                    let diff = state.diff.clone();
                    let update = diff.update(cx, |diff, cx| diff.set_base_text(base, current, cx));
                    Ok((diff, update))
                })??;
                update.await;
                Ok(diff)
            }
            .await;
            if sender.send(result).is_err() {
                log::debug!("VCS diff consumer closed");
            }
        }));
        cx.background_spawn(async move { receiver.await.context("VCS diff request cancelled")? })
    }

    fn invalidate_diffs(&mut self, repository: &VcsRepositoryId, cx: &mut Context<Self>) {
        for state in self
            .diffs
            .values_mut()
            .filter(|state| state.repository == *repository)
        {
            state.generation += 1;
            state.update_task = None;
            state.base_loaded = false;
            if let Some(buffer) = state.buffer.upgrade() {
                let snapshot = buffer.read(cx).text_snapshot();
                let current: Arc<str> = snapshot.text().into();
                state.base = Some(current.clone());
                state.update_task = Some(state.diff.update(cx, |diff, cx| {
                    diff.set_base_text(Some(current), snapshot, cx)
                }));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[gpui::test]
    async fn marker_discovery_is_ancestor_scoped_and_rejects_paths(cx: &mut gpui::TestAppContext) {
        let fs = fs::FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            "/repository",
            serde_json::json!({
                ".example-vcs": "",
                "project": {"source": {}},
                "unrelated": {".nested-provider": ""}
            }),
        )
        .await;
        let fs: Arc<dyn Fs> = fs;
        assert!(
            matches_markers(
                Path::new("/repository/project/source"),
                &[".example-vcs".into()],
                &fs
            )
            .await
            .unwrap()
        );
        assert!(
            !matches_markers(
                Path::new("/repository/project"),
                &[".nested-provider".into()],
                &fs
            )
            .await
            .unwrap()
        );
        assert!(
            matches_markers(
                Path::new("/repository/project"),
                &["../.example-vcs".into()],
                &fs
            )
            .await
            .is_err()
        );
    }

    #[gpui::test]
    async fn external_diff_uses_live_buffer_and_denies_restore(cx: &mut gpui::TestAppContext) {
        let buffer = cx.new(|cx| Buffer::local("saved\n", cx));
        let diff = cx.new(|cx| {
            let snapshot = buffer.read(cx).snapshot();
            let mut diff = BufferDiff::new(&snapshot.text, None, None, cx);
            diff.set_operations(Arc::new(ReadOnlyDiffOperations));
            diff
        });
        let initial = buffer.read_with(cx, |buffer, _| buffer.text_snapshot());
        diff.update(cx, |diff, cx| {
            diff.set_base_text(Some("saved\n".into()), initial, cx)
        })
        .await;
        assert!(diff.read_with(cx, |diff, cx| diff.snapshot(cx).is_empty()));
        buffer.update(cx, |buffer, cx| {
            buffer.edit([(0..5, "unsaved")], None, cx);
        });
        let edited = buffer.read_with(cx, |buffer, _| buffer.text_snapshot());
        diff.update(cx, |diff, cx| {
            diff.set_base_text(Some("saved\n".into()), edited, cx)
        })
        .await;
        diff.read_with(cx, |diff, cx| {
            assert!(!diff.snapshot(cx).is_empty());
            let operations = diff.operations().unwrap();
            assert!(!operations.supports_staging());
            assert!(!operations.supports_unstaging());
            assert!(!operations.supports_restore());
        });
    }

    #[gpui::test]
    async fn text_diff_rejects_symlink_parents_and_accepts_deleted_files(
        cx: &mut gpui::TestAppContext,
    ) {
        let fs = fs::FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            "/scope",
            serde_json::json!({"ordinary": {"file.txt": "text"}}),
        )
        .await;
        fs.insert_tree("/outside", serde_json::json!({"private.txt": "outside"}))
            .await;
        fs.insert_symlink("/scope/link", Path::new("/outside").to_path_buf())
            .await;
        let fs: Arc<dyn Fs> = fs;
        assert!(
            scoped_metadata(Path::new("/scope"), "link/private.txt", &fs)
                .await
                .is_err()
        );
        assert!(
            scoped_metadata(Path::new("/scope"), "ordinary/deleted.txt", &fs)
                .await
                .unwrap()
                .is_none()
        );
        assert!(
            scoped_metadata(Path::new("/scope"), "ordinary/file.txt", &fs)
                .await
                .unwrap()
                .is_some()
        );
    }
}

#[cfg(all(test, unix, feature = "test-support"))]
#[path = "vcs_store_tests.rs"]
mod integration_tests;

async fn matches_markers(root: &Path, markers: &[String], fs: &Arc<dyn Fs>) -> Result<bool> {
    ensure!(
        !markers.is_empty(),
        "VCS provider must declare a repository marker"
    );
    for marker in markers {
        ensure!(
            !marker.is_empty()
                && marker != "."
                && marker != ".."
                && !marker.contains(['/', '\\', '\0']),
            "invalid VCS repository marker"
        );
    }
    for ancestor in root.ancestors() {
        for marker in markers {
            if fs.metadata(&ancestor.join(marker)).await?.is_some() {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

async fn scoped_metadata(
    root: &Path,
    path: &str,
    fs: &Arc<dyn Fs>,
) -> Result<Option<fs::Metadata>> {
    vcs::validate_path(path)?;
    let mut cursor = root.to_path_buf();
    let mut metadata = fs.metadata(&cursor).await?;
    ensure!(
        metadata.as_ref().is_some_and(|metadata| metadata.is_dir),
        "VCS scope no longer exists"
    );
    ensure!(
        metadata
            .as_ref()
            .is_none_or(|metadata| !metadata.is_symlink),
        "VCS text diffs do not follow symlink scopes"
    );
    for component in path.split('/') {
        if metadata.is_none() {
            return Ok(None);
        }
        cursor.push(component);
        metadata = fs.metadata(&cursor).await?;
        ensure!(
            metadata
                .as_ref()
                .is_none_or(|metadata| !metadata.is_symlink),
            "VCS text diffs do not follow symlinks or symlink parents"
        );
    }
    Ok(metadata)
}

struct ProviderWorktree {
    id: WorktreeId,
    root: Arc<Path>,
    fs: Arc<dyn Fs>,
    environment: HashMap<String, String>,
}

#[async_trait::async_trait]
impl WorktreeDelegate for ProviderWorktree {
    fn id(&self) -> u64 {
        self.id.to_proto()
    }
    fn root_path(&self) -> String {
        self.root.to_string_lossy().into_owned()
    }
    async fn read_text_file(&self, path: &RelPath) -> Result<String> {
        scoped_metadata(&self.root, path.as_unix_str(), &self.fs).await?;
        self.fs.load(&self.root.join(path.as_std_path())).await
    }
    async fn which(&self, binary_name: String) -> Option<String> {
        which::which_in(binary_name, self.environment.get("PATH"), &self.root)
            .ok()
            .map(|path| path.to_string_lossy().into_owned())
    }
    async fn shell_env(&self) -> Vec<(String, String)> {
        self.environment.clone().into_iter().collect()
    }
}
