use std::{
    path::{Path, PathBuf},
    str::FromStr,
    sync::Arc,
};

use anyhow::{Result, bail};

use async_trait::async_trait;
use collections::BTreeMap;
use gpui::{
    App, AppContext as _, AsyncApp, Context, Entity, EventEmitter, Subscription, Task, WeakEntity,
};
use language::{LanguageName, LanguageRegistry, LanguageToolchainStore, Toolchain, ToolchainList};
use rpc::{
    AnyProtoClient, TypedEnvelope,
    proto::{self, FromProto, ToProto},
};
use settings::WorktreeId;
use util::ResultExt as _;

use crate::{ProjectEnvironment, ProjectPath, worktree_store::WorktreeStore};

pub struct ToolchainStore(ToolchainStoreInner);
enum ToolchainStoreInner {
    Local(
        Entity<LocalToolchainStore>,
        #[allow(dead_code)] Subscription,
    ),
    Remote(Entity<RemoteToolchainStore>),
}

impl EventEmitter<ToolchainStoreEvent> for ToolchainStore {}
impl ToolchainStore {
    pub fn init(client: &AnyProtoClient) {
        client.add_entity_request_handler(Self::handle_activate_toolchain);
        client.add_entity_request_handler(Self::handle_list_toolchains);
        client.add_entity_request_handler(Self::handle_active_toolchain);
    }

    pub fn local(
        languages: Arc<LanguageRegistry>,
        worktree_store: Entity<WorktreeStore>,
        project_environment: Entity<ProjectEnvironment>,
        cx: &mut Context<Self>,
    ) -> Self {
        let entity = cx.new(|_| LocalToolchainStore {
            languages,
            worktree_store,
            project_environment,
            active_toolchains: Default::default(),
        });
        let subscription = cx.subscribe(&entity, |_, _, e: &ToolchainStoreEvent, cx| {
            cx.emit(e.clone())
        });
        Self(ToolchainStoreInner::Local(entity, subscription))
    }

    pub(super) fn remote(project_id: u64, client: AnyProtoClient, cx: &mut App) -> Self {
        Self(ToolchainStoreInner::Remote(
            cx.new(|_| RemoteToolchainStore { client, project_id }),
        ))
    }
    pub(crate) fn activate_toolchain(
        &self,
        path: ProjectPath,
        toolchain: Toolchain,
        cx: &mut App,
    ) -> Task<Option<()>> {
        match &self.0 {
            ToolchainStoreInner::Local(local, _) => {
                local.update(cx, |this, cx| this.activate_toolchain(path, toolchain, cx))
            }
            ToolchainStoreInner::Remote(remote) => {
                remote.read(cx).activate_toolchain(path, toolchain, cx)
            }
        }
    }
    pub(crate) fn list_toolchains(
        &self,
        path: ProjectPath,
        language_name: LanguageName,
        cx: &App,
    ) -> Task<Option<ToolchainList>> {
        match &self.0 {
            ToolchainStoreInner::Local(local, _) => {
                local.read(cx).list_toolchains(path, language_name, cx)
            }
            ToolchainStoreInner::Remote(remote) => {
                remote.read(cx).list_toolchains(path, language_name, cx)
            }
        }
    }
    pub(crate) fn active_toolchain(
        &self,
        path: ProjectPath,
        language_name: LanguageName,
        cx: &App,
    ) -> Task<Option<Toolchain>> {
        match &self.0 {
            ToolchainStoreInner::Local(local, _) => {
                local.read(cx).active_toolchain(path, language_name, cx)
            }
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
                path: PathBuf::from(toolchain.path).to_proto().into(),
                as_json: serde_json::Value::from_str(&toolchain.raw_json)?,
                language_name,
            };
            let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
            let path: Arc<Path> = if let Some(path) = envelope.payload.path {
                Arc::from(path.as_ref())
            } else {
                Arc::from("".as_ref())
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
        let toolchain = this
            .update(&mut cx, |this, cx| {
                let language_name = LanguageName::from_proto(envelope.payload.language_name);
                let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
                this.active_toolchain(
                    ProjectPath {
                        worktree_id,
                        path: Arc::from(envelope.payload.path.as_deref().unwrap_or("").as_ref()),
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
                    path: path.to_proto(),
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
                let path = Arc::from(envelope.payload.path.as_deref().unwrap_or("").as_ref());
                this.list_toolchains(ProjectPath { worktree_id, path }, language_name, cx)
            })?
            .await;
        let has_values = toolchains.is_some();
        let groups = if let Some(toolchains) = &toolchains {
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
        let toolchains = if let Some(toolchains) = toolchains {
            toolchains
                .toolchains
                .into_iter()
                .map(|toolchain| {
                    let path = PathBuf::from(toolchain.path.to_string());
                    proto::Toolchain {
                        name: toolchain.name.to_string(),
                        path: path.to_proto(),
                        raw_json: toolchain.as_json.to_string(),
                    }
                })
                .collect::<Vec<_>>()
        } else {
            vec![]
        };

        Ok(proto::ListToolchainsResponse {
            has_values,
            toolchains,
            groups,
        })
    }
    pub fn as_language_toolchain_store(&self) -> Arc<dyn LanguageToolchainStore> {
        match &self.0 {
            ToolchainStoreInner::Local(local, _) => Arc::new(LocalStore(local.downgrade())),
            ToolchainStoreInner::Remote(remote) => Arc::new(RemoteStore(remote.downgrade())),
        }
    }
}

struct LocalToolchainStore {
    languages: Arc<LanguageRegistry>,
    worktree_store: Entity<WorktreeStore>,
    project_environment: Entity<ProjectEnvironment>,
    active_toolchains: BTreeMap<(WorktreeId, LanguageName), BTreeMap<Arc<Path>, Toolchain>>,
}

#[async_trait(?Send)]
impl language::LanguageToolchainStore for LocalStore {
    async fn active_toolchain(
        self: Arc<Self>,
        worktree_id: WorktreeId,
        path: Arc<Path>,
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

#[async_trait(?Send)]
impl language::LanguageToolchainStore for RemoteStore {
    async fn active_toolchain(
        self: Arc<Self>,
        worktree_id: WorktreeId,
        path: Arc<Path>,
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

pub(crate) struct EmptyToolchainStore;
#[async_trait(?Send)]
impl language::LanguageToolchainStore for EmptyToolchainStore {
    async fn active_toolchain(
        self: Arc<Self>,
        _: WorktreeId,
        _: Arc<Path>,
        _: LanguageName,
        _: &mut AsyncApp,
    ) -> Option<Toolchain> {
        None
    }
}
struct LocalStore(WeakEntity<LocalToolchainStore>);
struct RemoteStore(WeakEntity<RemoteToolchainStore>);

#[derive(Clone)]
pub enum ToolchainStoreEvent {
    ToolchainActivated,
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
        &self,
        path: ProjectPath,
        language_name: LanguageName,
        cx: &App,
    ) -> Task<Option<ToolchainList>> {
        let registry = self.languages.clone();
        let Some(abs_path) = self.worktree_store.read(cx).absolutize(&path, cx) else {
            return Task::ready(None);
        };
        let environment = self.project_environment.clone();
        cx.spawn(async move |cx| {
            let project_env = environment
                .update(cx, |environment, cx| {
                    environment.get_directory_environment(abs_path.as_path().into(), cx)
                })
                .ok()?
                .await;

            cx.background_spawn(async move {
                let language = registry
                    .language_for_name(language_name.as_ref())
                    .await
                    .ok()?;
                let toolchains = language.toolchain_lister()?;
                Some(toolchains.list(abs_path.to_path_buf(), project_env).await)
            })
            .await
        })
    }
    pub(crate) fn active_toolchain(
        &self,
        path: ProjectPath,
        language_name: LanguageName,
        _: &App,
    ) -> Task<Option<Toolchain>> {
        let ancestors = path.path.ancestors();
        Task::ready(
            self.active_toolchains
                .get(&(path.worktree_id, language_name))
                .and_then(|paths| {
                    ancestors
                        .into_iter()
                        .find_map(|root_path| paths.get(root_path))
                })
                .cloned(),
        )
    }
}
struct RemoteToolchainStore {
    client: AnyProtoClient,
    project_id: u64,
}

impl RemoteToolchainStore {
    pub(crate) fn activate_toolchain(
        &self,
        project_path: ProjectPath,
        toolchain: Toolchain,
        cx: &App,
    ) -> Task<Option<()>> {
        let project_id = self.project_id;
        let client = self.client.clone();
        cx.background_spawn(async move {
            let path = PathBuf::from(toolchain.path.to_string());
            let _ = client
                .request(proto::ActivateToolchain {
                    project_id,
                    worktree_id: project_path.worktree_id.to_proto(),
                    language_name: toolchain.language_name.into(),
                    toolchain: Some(proto::Toolchain {
                        name: toolchain.name.into(),
                        path: path.to_proto(),
                        raw_json: toolchain.as_json.to_string(),
                    }),
                    path: Some(project_path.path.to_string_lossy().into_owned()),
                })
                .await
                .log_err()?;
            Some(())
        })
    }

    pub(crate) fn list_toolchains(
        &self,
        path: ProjectPath,
        language_name: LanguageName,
        cx: &App,
    ) -> Task<Option<ToolchainList>> {
        let project_id = self.project_id;
        let client = self.client.clone();
        cx.background_spawn(async move {
            let response = client
                .request(proto::ListToolchains {
                    project_id,
                    worktree_id: path.worktree_id.to_proto(),
                    language_name: language_name.clone().into(),
                    path: Some(path.path.to_string_lossy().into_owned()),
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
                        // todo(windows)
                        // Do we need to convert path to native string?
                        path: PathBuf::from_proto(toolchain.path)
                            .to_string_lossy()
                            .to_string()
                            .into(),
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
            Some(ToolchainList {
                toolchains,
                default: None,
                groups,
            })
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
                    path: Some(path.path.to_string_lossy().into_owned()),
                })
                .await
                .log_err()?;

            response.toolchain.and_then(|toolchain| {
                Some(Toolchain {
                    language_name: language_name.clone(),
                    name: toolchain.name.into(),
                    // todo(windows)
                    // Do we need to convert path to native string?
                    path: PathBuf::from_proto(toolchain.path)
                        .to_string_lossy()
                        .to_string()
                        .into(),
                    as_json: serde_json::Value::from_str(&toolchain.raw_json).ok()?,
                })
            })
        })
    }
}
