use std::{path::PathBuf, str::FromStr, sync::Arc};

use anyhow::{Context as _, Result, bail};

use async_trait::async_trait;
use collections::{BTreeMap, IndexSet};
use fs::Fs;
use gpui::{
    App, AppContext as _, AsyncApp, Context, Entity, EventEmitter, Subscription, Task, WeakEntity,
};
use language::{
    LanguageName, LanguageRegistry, LanguageToolchainStore, ManifestDelegate, Toolchain,
    ToolchainList, ToolchainScope,
};
use rpc::{
    AnyProtoClient, TypedEnvelope,
    proto::{
        self, ResolveToolchainResponse,
        resolve_toolchain_response::Response as ResolveResponsePayload,
    },
};
use settings::WorktreeId;
use task::Shell;
use util::{ResultExt as _, rel_path::RelPath};

use crate::{
    ProjectEnvironment, ProjectPath,
    manifest_tree::{ManifestQueryDelegate, ManifestTree},
    worktree_store::WorktreeStore,
};

pub struct ToolchainStore {
    mode: ToolchainStoreInner,
    user_toolchains: BTreeMap<ToolchainScope, IndexSet<Toolchain>>,
    worktree_store: Entity<WorktreeStore>,
    _sub: Subscription,
}

enum ToolchainStoreInner {
    Local(Entity<LocalToolchainStore>),
    Remote(Entity<RemoteToolchainStore>),
}

pub struct Toolchains {
    /// Auto-detected toolchains.
    pub toolchains: ToolchainList,
    /// Path of the project root at which we ran the automatic toolchain detection.
    pub root_path: Arc<RelPath>,
    pub user_toolchains: BTreeMap<ToolchainScope, IndexSet<Toolchain>>,
}
impl EventEmitter<ToolchainStoreEvent> for ToolchainStore {}
impl ToolchainStore {
    pub fn init(client: &AnyProtoClient) {
        client.add_entity_request_handler(Self::handle_activate_toolchain);
        client.add_entity_request_handler(Self::handle_list_toolchains);
        client.add_entity_request_handler(Self::handle_active_toolchain);
        client.add_entity_request_handler(Self::handle_resolve_toolchain);
    }

    pub fn local(
        languages: Arc<LanguageRegistry>,
        worktree_store: Entity<WorktreeStore>,
        project_environment: Entity<ProjectEnvironment>,
        manifest_tree: Entity<ManifestTree>,
        fs: Arc<dyn Fs>,
        cx: &mut Context<Self>,
    ) -> Self {
        let entity = cx.new(|_| LocalToolchainStore {
            languages,
            worktree_store: worktree_store.clone(),
            project_environment,
            active_toolchains: Default::default(),
            manifest_tree,
            fs,
        });
        let _sub = cx.subscribe(&entity, |_, _, e: &ToolchainStoreEvent, cx| {
            cx.emit(e.clone())
        });
        Self {
            mode: ToolchainStoreInner::Local(entity),
            worktree_store,
            user_toolchains: Default::default(),
            _sub,
        }
    }

    pub(super) fn remote(
        project_id: u64,
        worktree_store: Entity<WorktreeStore>,
        client: AnyProtoClient,
        cx: &mut Context<Self>,
    ) -> Self {
        let entity = cx.new(|_| RemoteToolchainStore { client, project_id });
        let _sub = cx.subscribe(&entity, |_, _, e: &ToolchainStoreEvent, cx| {
            cx.emit(e.clone())
        });
        Self {
            mode: ToolchainStoreInner::Remote(entity),
            user_toolchains: Default::default(),
            worktree_store,
            _sub,
        }
    }
    pub(crate) fn activate_toolchain(
        &self,
        path: ProjectPath,
        toolchain: Toolchain,
        cx: &mut App,
    ) -> Task<Option<()>> {
        match &self.mode {
            ToolchainStoreInner::Local(local) => {
                local.update(cx, |this, cx| this.activate_toolchain(path, toolchain, cx))
            }
            ToolchainStoreInner::Remote(remote) => {
                remote.update(cx, |this, cx| this.activate_toolchain(path, toolchain, cx))
            }
        }
    }

    pub(crate) fn user_toolchains(&self) -> BTreeMap<ToolchainScope, IndexSet<Toolchain>> {
        self.user_toolchains.clone()
    }
    pub(crate) fn add_toolchain(
        &mut self,
        toolchain: Toolchain,
        scope: ToolchainScope,
        cx: &mut Context<Self>,
    ) {
        let did_insert = self
            .user_toolchains
            .entry(scope)
            .or_default()
            .insert(toolchain);
        if did_insert {
            cx.emit(ToolchainStoreEvent::CustomToolchainsModified);
        }
    }

    pub(crate) fn remove_toolchain(
        &mut self,
        toolchain: Toolchain,
        scope: ToolchainScope,
        cx: &mut Context<Self>,
    ) {
        let mut did_remove = false;
        self.user_toolchains
            .entry(scope)
            .and_modify(|toolchains| did_remove = toolchains.shift_remove(&toolchain));
        if did_remove {
            cx.emit(ToolchainStoreEvent::CustomToolchainsModified);
        }
    }

    pub(crate) fn resolve_toolchain(
        &self,
        abs_path: PathBuf,
        language_name: LanguageName,
        cx: &mut Context<Self>,
    ) -> Task<Result<Toolchain>> {
        debug_assert!(abs_path.is_absolute());
        match &self.mode {
            ToolchainStoreInner::Local(local) => local.update(cx, |this, cx| {
                this.resolve_toolchain(abs_path, language_name, cx)
            }),
            ToolchainStoreInner::Remote(remote) => remote.update(cx, |this, cx| {
                this.resolve_toolchain(abs_path, language_name, cx)
            }),
        }
    }
    pub(crate) fn list_toolchains(
        &self,
        path: ProjectPath,
        language_name: LanguageName,
        cx: &mut Context<Self>,
    ) -> Task<Option<Toolchains>> {
        let Some(worktree) = self
            .worktree_store
            .read(cx)
            .worktree_for_id(path.worktree_id, cx)
        else {
            return Task::ready(None);
        };
        let target_root_path = worktree.read_with(cx, |this, _| this.abs_path());

        let user_toolchains = self
            .user_toolchains
            .iter()
            .filter(|(scope, _)| {
                if let ToolchainScope::Subproject(subproject_root_path, relative_path) = scope {
                    target_root_path == *subproject_root_path
                        && relative_path.starts_with(&path.path)
                } else {
                    true
                }
            })
            .map(|(scope, toolchains)| {
                (
                    scope.clone(),
                    toolchains
                        .iter()
                        .filter(|toolchain| toolchain.language_name == language_name)
                        .cloned()
                        .collect::<IndexSet<_>>(),
                )
            })
            .collect::<BTreeMap<_, _>>();
        let task = match &self.mode {
            ToolchainStoreInner::Local(local) => {
                local.update(cx, |this, cx| this.list_toolchains(path, language_name, cx))
            }
            ToolchainStoreInner::Remote(remote) => {
                remote.read(cx).list_toolchains(path, language_name, cx)
            }
        };
        cx.spawn(async move |_, _| {
            let (mut toolchains, root_path) = task.await?;
            toolchains.toolchains.retain(|toolchain| {
                !user_toolchains
                    .values()
                    .any(|toolchains| toolchains.contains(toolchain))
            });

            Some(Toolchains {
                toolchains,
                root_path,
                user_toolchains,
            })
        })
    }

    pub(crate) fn active_toolchain(
        &self,
        path: ProjectPath,
        language_name: LanguageName,
        cx: &App,
    ) -> Task<Option<Toolchain>> {
        match &self.mode {
            ToolchainStoreInner::Local(local) => Task::ready(local.read(cx).active_toolchain(
                path.worktree_id,
                &path.path,
                language_name,
            )),
            ToolchainStoreInner::Remote(remote) => {
                remote.read(cx).active_toolchain(path, language_name, cx)
            }
        }
    }
    async fn handle_activate_toolchain(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::ActivateToolchain>,
        mut cx: AsyncApp,
    ) -> Result<proto::Ack> {
        this.update(&mut cx, |this, cx| {
            let language_name = LanguageName::from_proto(envelope.payload.language_name);
            let Some(toolchain) = envelope.payload.toolchain else {
                bail!("Missing `toolchain` in payload");
            };
            let toolchain = Toolchain {
                name: toolchain.name.into(),
                // todo(windows)
                // Do we need to convert path to native string?
                path: toolchain.path.into(),
                as_json: serde_json::Value::from_str(&toolchain.raw_json)?,
                language_name,
            };
            let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
            let path = if let Some(path) = envelope.payload.path {
                RelPath::from_proto(&path)?
            } else {
                RelPath::empty().into()
            };
            Ok(this.activate_toolchain(ProjectPath { worktree_id, path }, toolchain, cx))
        })??
        .await;
        Ok(proto::Ack {})
    }
    async fn handle_active_toolchain(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::ActiveToolchain>,
        mut cx: AsyncApp,
    ) -> Result<proto::ActiveToolchainResponse> {
        let path = RelPath::unix(envelope.payload.path.as_deref().unwrap_or(""))?;
        let toolchain = this
            .update(&mut cx, |this, cx| {
                let language_name = LanguageName::from_proto(envelope.payload.language_name);
                let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
                this.active_toolchain(
                    ProjectPath {
                        worktree_id,
                        path: Arc::from(path),
                    },
                    language_name,
                    cx,
                )
            })?
            .await;

        Ok(proto::ActiveToolchainResponse {
            toolchain: toolchain.map(|toolchain| {
                let path = PathBuf::from(toolchain.path.to_string());
                proto::Toolchain {
                    name: toolchain.name.into(),
                    path: path.to_string_lossy().into_owned(),
                    raw_json: toolchain.as_json.to_string(),
                }
            }),
        })
    }

    async fn handle_list_toolchains(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::ListToolchains>,
        mut cx: AsyncApp,
    ) -> Result<proto::ListToolchainsResponse> {
        let toolchains = this
            .update(&mut cx, |this, cx| {
                let language_name = LanguageName::from_proto(envelope.payload.language_name);
                let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
                let path = RelPath::from_proto(envelope.payload.path.as_deref().unwrap_or(""))?;
                anyhow::Ok(this.list_toolchains(
                    ProjectPath { worktree_id, path },
                    language_name,
                    cx,
                ))
            })??
            .await;
        let has_values = toolchains.is_some();
        let groups = if let Some(Toolchains { toolchains, .. }) = &toolchains {
            toolchains
                .groups
                .iter()
                .filter_map(|group| {
                    Some(proto::ToolchainGroup {
                        start_index: u64::try_from(group.0).ok()?,
                        name: String::from(group.1.as_ref()),
                    })
                })
                .collect()
        } else {
            vec![]
        };
        let (toolchains, relative_path) = if let Some(Toolchains {
            toolchains,
            root_path: relative_path,
            ..
        }) = toolchains
        {
            let toolchains = toolchains
                .toolchains
                .into_iter()
                .map(|toolchain| {
                    let path = PathBuf::from(toolchain.path.to_string());
                    proto::Toolchain {
                        name: toolchain.name.to_string(),
                        path: path.to_string_lossy().into_owned(),
                        raw_json: toolchain.as_json.to_string(),
                    }
                })
                .collect::<Vec<_>>();
            (toolchains, relative_path)
        } else {
            (vec![], Arc::from(RelPath::empty()))
        };

        Ok(proto::ListToolchainsResponse {
            has_values,
            toolchains,
            groups,
            relative_worktree_path: Some(relative_path.to_proto()),
        })
    }

    async fn handle_resolve_toolchain(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::ResolveToolchain>,
        mut cx: AsyncApp,
    ) -> Result<proto::ResolveToolchainResponse> {
        let toolchain = this
            .update(&mut cx, |this, cx| {
                let language_name = LanguageName::from_proto(envelope.payload.language_name);
                let path = PathBuf::from(envelope.payload.abs_path);
                this.resolve_toolchain(path, language_name, cx)
            })?
            .await;
        let response = match toolchain {
            Ok(toolchain) => {
                let toolchain = proto::Toolchain {
                    name: toolchain.name.to_string(),
                    path: toolchain.path.to_string(),
                    raw_json: toolchain.as_json.to_string(),
                };
                ResolveResponsePayload::Toolchain(toolchain)
            }
            Err(e) => ResolveResponsePayload::Error(e.to_string()),
        };
        Ok(ResolveToolchainResponse {
            response: Some(response),
        })
    }

    pub fn as_language_toolchain_store(&self) -> Arc<dyn LanguageToolchainStore> {
        match &self.mode {
            ToolchainStoreInner::Local(local) => Arc::new(LocalStore(local.downgrade())),
            ToolchainStoreInner::Remote(remote) => Arc::new(RemoteStore(remote.downgrade())),
        }
    }
    pub fn as_local_store(&self) -> Option<&Entity<LocalToolchainStore>> {
        match &self.mode {
            ToolchainStoreInner::Local(local) => Some(local),
            ToolchainStoreInner::Remote(_) => None,
        }
    }
}

pub struct LocalToolchainStore {
    languages: Arc<LanguageRegistry>,
    worktree_store: Entity<WorktreeStore>,
    project_environment: Entity<ProjectEnvironment>,
    active_toolchains: BTreeMap<(WorktreeId, LanguageName), BTreeMap<Arc<RelPath>, Toolchain>>,
    manifest_tree: Entity<ManifestTree>,
    fs: Arc<dyn Fs>,
}

#[async_trait(?Send)]
impl language::LocalLanguageToolchainStore for LocalStore {
    fn active_toolchain(
        self: Arc<Self>,
        worktree_id: WorktreeId,
        path: &Arc<RelPath>,
        language_name: LanguageName,
        cx: &mut AsyncApp,
    ) -> Option<Toolchain> {
        self.0
            .update(cx, |this, _| {
                this.active_toolchain(worktree_id, path, language_name)
            })
            .ok()?
    }
}

#[async_trait(?Send)]
impl language::LanguageToolchainStore for RemoteStore {
    async fn active_toolchain(
        self: Arc<Self>,
        worktree_id: WorktreeId,
        path: Arc<RelPath>,
        language_name: LanguageName,
        cx: &mut AsyncApp,
    ) -> Option<Toolchain> {
        self.0
            .update(cx, |this, cx| {
                this.active_toolchain(ProjectPath { worktree_id, path }, language_name, cx)
            })
            .ok()?
            .await
    }
}

pub struct EmptyToolchainStore;
impl language::LocalLanguageToolchainStore for EmptyToolchainStore {
    fn active_toolchain(
        self: Arc<Self>,
        _: WorktreeId,
        _: &Arc<RelPath>,
        _: LanguageName,
        _: &mut AsyncApp,
    ) -> Option<Toolchain> {
        None
    }
}
pub(crate) struct LocalStore(WeakEntity<LocalToolchainStore>);
struct RemoteStore(WeakEntity<RemoteToolchainStore>);

#[derive(Clone)]
pub enum ToolchainStoreEvent {
    ToolchainActivated,
    CustomToolchainsModified,
}

impl EventEmitter<ToolchainStoreEvent> for LocalToolchainStore {}

impl LocalToolchainStore {
    pub(crate) fn activate_toolchain(
        &self,
        path: ProjectPath,
        toolchain: Toolchain,
        cx: &mut Context<Self>,
    ) -> Task<Option<()>> {
        cx.spawn(async move |this, cx| {
            this.update(cx, |this, cx| {
                this.active_toolchains
                    .entry((path.worktree_id, toolchain.language_name.clone()))
                    .or_default()
                    .insert(path.path, toolchain.clone());
                cx.emit(ToolchainStoreEvent::ToolchainActivated);
            })
            .ok();
            Some(())
        })
    }
    pub(crate) fn list_toolchains(
        &mut self,
        path: ProjectPath,
        language_name: LanguageName,
        cx: &mut Context<Self>,
    ) -> Task<Option<(ToolchainList, Arc<RelPath>)>> {
        let registry = self.languages.clone();

        let manifest_tree = self.manifest_tree.downgrade();
        let fs = self.fs.clone();

        let environment = self.project_environment.clone();
        cx.spawn(async move |this, cx| {
            let language = cx
                .background_spawn(registry.language_for_name(language_name.as_ref()))
                .await
                .ok()?;
            let toolchains = language.toolchain_lister()?;
            let manifest_name = toolchains.meta().manifest_name;
            let (snapshot, worktree) = this
                .update(cx, |this, cx| {
                    this.worktree_store
                        .read(cx)
                        .worktree_for_id(path.worktree_id, cx)
                        .map(|worktree| (worktree.read(cx).snapshot(), worktree))
                })
                .ok()
                .flatten()?;
            let worktree_id = snapshot.id();
            let worktree_root = snapshot.abs_path().to_path_buf();
            let delegate =
                Arc::from(ManifestQueryDelegate::new(snapshot)) as Arc<dyn ManifestDelegate>;
            let relative_path = manifest_tree
                .update(cx, |this, cx| {
                    this.root_for_path(&path, &manifest_name, &delegate, cx)
                })
                .ok()?
                .unwrap_or_else(|| ProjectPath {
                    path: Arc::from(RelPath::empty()),
                    worktree_id,
                });
            let abs_path = worktree
                .update(cx, |this, _| this.absolutize(&relative_path.path))
                .ok()?;

            let project_env = environment
                .update(cx, |environment, cx| {
                    environment.local_directory_environment(
                        &Shell::System,
                        abs_path.as_path().into(),
                        cx,
                    )
                })
                .ok()?
                .await;

            cx.background_spawn(async move {
                Some((
                    toolchains
                        .list(
                            worktree_root,
                            relative_path.path.clone(),
                            project_env,
                            fs.as_ref(),
                        )
                        .await,
                    relative_path.path,
                ))
            })
            .await
        })
    }
    pub(crate) fn active_toolchain(
        &self,
        worktree_id: WorktreeId,
        relative_path: &Arc<RelPath>,
        language_name: LanguageName,
    ) -> Option<Toolchain> {
        let ancestors = relative_path.ancestors();

        self.active_toolchains
            .get(&(worktree_id, language_name))
            .and_then(|paths| {
                ancestors
                    .into_iter()
                    .find_map(|root_path| paths.get(root_path))
            })
            .cloned()
    }

    fn resolve_toolchain(
        &self,
        path: PathBuf,
        language_name: LanguageName,
        cx: &mut Context<Self>,
    ) -> Task<Result<Toolchain>> {
        let registry = self.languages.clone();
        let environment = self.project_environment.clone();
        let fs = self.fs.clone();
        cx.spawn(async move |_, cx| {
            let language = cx
                .background_spawn(registry.language_for_name(&language_name.0))
                .await
                .with_context(|| format!("Language {} not found", language_name.0))?;
            let toolchain_lister = language.toolchain_lister().with_context(|| {
                format!("Language {} does not support toolchains", language_name.0)
            })?;

            let project_env = environment
                .update(cx, |environment, cx| {
                    environment.local_directory_environment(
                        &Shell::System,
                        path.as_path().into(),
                        cx,
                    )
                })?
                .await;
            cx.background_spawn(async move {
                toolchain_lister
                    .resolve(path, project_env, fs.as_ref())
                    .await
            })
            .await
        })
    }
}

impl EventEmitter<ToolchainStoreEvent> for RemoteToolchainStore {}
struct RemoteToolchainStore {
    client: AnyProtoClient,
    project_id: u64,
}

impl RemoteToolchainStore {
    pub(crate) fn activate_toolchain(
        &self,
        project_path: ProjectPath,
        toolchain: Toolchain,
        cx: &mut Context<Self>,
    ) -> Task<Option<()>> {
        let project_id = self.project_id;
        let client = self.client.clone();
        cx.spawn(async move |this, cx| {
            let did_activate = cx
                .background_spawn(async move {
                    let path = PathBuf::from(toolchain.path.to_string());
                    let _ = client
                        .request(proto::ActivateToolchain {
                            project_id,
                            worktree_id: project_path.worktree_id.to_proto(),
                            language_name: toolchain.language_name.into(),
                            toolchain: Some(proto::Toolchain {
                                name: toolchain.name.into(),
                                path: path.to_string_lossy().into_owned(),
                                raw_json: toolchain.as_json.to_string(),
                            }),
                            path: Some(project_path.path.to_proto()),
                        })
                        .await
                        .log_err()?;
                    Some(())
                })
                .await;
            did_activate.and_then(|_| {
                this.update(cx, |_, cx| {
                    cx.emit(ToolchainStoreEvent::ToolchainActivated);
                })
                .ok()
            })
        })
    }

    pub(crate) fn list_toolchains(
        &self,
        path: ProjectPath,
        language_name: LanguageName,
        cx: &App,
    ) -> Task<Option<(ToolchainList, Arc<RelPath>)>> {
        let project_id = self.project_id;
        let client = self.client.clone();
        cx.background_spawn(async move {
            let response = client
                .request(proto::ListToolchains {
                    project_id,
                    worktree_id: path.worktree_id.to_proto(),
                    language_name: language_name.clone().into(),
                    path: Some(path.path.to_proto()),
                })
                .await
                .log_err()?;
            if !response.has_values {
                return None;
            }
            let toolchains = response
                .toolchains
                .into_iter()
                .filter_map(|toolchain| {
                    Some(Toolchain {
                        language_name: language_name.clone(),
                        name: toolchain.name.into(),
                        path: toolchain.path.into(),
                        as_json: serde_json::Value::from_str(&toolchain.raw_json).ok()?,
                    })
                })
                .collect();
            let groups = response
                .groups
                .into_iter()
                .filter_map(|group| {
                    Some((usize::try_from(group.start_index).ok()?, group.name.into()))
                })
                .collect();
            let relative_path = RelPath::from_proto(
                response
                    .relative_worktree_path
                    .as_deref()
                    .unwrap_or_default(),
            )
            .log_err()?;
            Some((
                ToolchainList {
                    toolchains,
                    default: None,
                    groups,
                },
                relative_path,
            ))
        })
    }
    pub(crate) fn active_toolchain(
        &self,
        path: ProjectPath,
        language_name: LanguageName,
        cx: &App,
    ) -> Task<Option<Toolchain>> {
        let project_id = self.project_id;
        let client = self.client.clone();
        cx.background_spawn(async move {
            let response = client
                .request(proto::ActiveToolchain {
                    project_id,
                    worktree_id: path.worktree_id.to_proto(),
                    language_name: language_name.clone().into(),
                    path: Some(path.path.to_proto()),
                })
                .await
                .log_err()?;

            response.toolchain.and_then(|toolchain| {
                Some(Toolchain {
                    language_name: language_name.clone(),
                    name: toolchain.name.into(),
                    path: toolchain.path.into(),
                    as_json: serde_json::Value::from_str(&toolchain.raw_json).ok()?,
                })
            })
        })
    }

    fn resolve_toolchain(
        &self,
        abs_path: PathBuf,
        language_name: LanguageName,
        cx: &mut Context<Self>,
    ) -> Task<Result<Toolchain>> {
        let project_id = self.project_id;
        let client = self.client.clone();
        cx.background_spawn(async move {
            let response: proto::ResolveToolchainResponse = client
                .request(proto::ResolveToolchain {
                    project_id,
                    language_name: language_name.clone().into(),
                    abs_path: abs_path.to_string_lossy().into_owned(),
                })
                .await?;

            let response = response
                .response
                .context("Failed to resolve toolchain via RPC")?;
            use proto::resolve_toolchain_response::Response;
            match response {
                Response::Toolchain(toolchain) => Ok(Toolchain {
                    language_name: language_name.clone(),
                    name: toolchain.name.into(),
                    path: toolchain.path.into(),
                    as_json: serde_json::Value::from_str(&toolchain.raw_json)
                        .context("Deserializing ResolveToolchain LSP response")?,
                }),
                Response::Error(error) => {
                    anyhow::bail!("{error}");
                }
            }
        })
    }
}
