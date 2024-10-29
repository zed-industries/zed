use std::sync::Arc;

use anyhow::{bail, Result};

use async_trait::async_trait;
use collections::BTreeMap;
use gpui::{
    AppContext, AsyncAppContext, Context, EventEmitter, Model, ModelContext, Subscription, Task,
    WeakModel,
};
use language::{LanguageName, LanguageRegistry, LanguageToolchainStore, Toolchain, ToolchainList};
use rpc::{proto, AnyProtoClient, TypedEnvelope};
use settings::WorktreeId;
use util::ResultExt as _;

use crate::worktree_store::WorktreeStore;

pub struct ToolchainStore(ToolchainStoreInner);
enum ToolchainStoreInner {
    Local(Model<LocalToolchainStore>, #[allow(dead_code)] Subscription),
    Remote(Model<RemoteToolchainStore>),
}

impl EventEmitter<ToolchainStoreEvent> for ToolchainStore {}
impl ToolchainStore {
    pub fn init(client: &AnyProtoClient) {
        client.add_model_request_handler(Self::handle_activate_toolchain);
        client.add_model_request_handler(Self::handle_list_toolchains);
        client.add_model_request_handler(Self::handle_active_toolchain);
    }

    pub fn local(
        languages: Arc<LanguageRegistry>,
        worktree_store: Model<WorktreeStore>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        let model = cx.new_model(|_| LocalToolchainStore {
            languages,
            worktree_store,
            active_toolchains: Default::default(),
        });
        let subscription = cx.subscribe(&model, |_, _, e: &ToolchainStoreEvent, cx| {
            cx.emit(e.clone())
        });
        Self(ToolchainStoreInner::Local(model, subscription))
    }
    pub(super) fn remote(project_id: u64, client: AnyProtoClient, cx: &mut AppContext) -> Self {
        Self(ToolchainStoreInner::Remote(
            cx.new_model(|_| RemoteToolchainStore { client, project_id }),
        ))
    }
    pub(crate) fn activate_toolchain(
        &self,
        worktree_id: WorktreeId,
        toolchain: Toolchain,
        cx: &mut AppContext,
    ) -> Task<Option<()>> {
        match &self.0 {
            ToolchainStoreInner::Local(local, _) => local.update(cx, |this, cx| {
                this.activate_toolchain(worktree_id, toolchain, cx)
            }),
            ToolchainStoreInner::Remote(remote) => {
                remote
                    .read(cx)
                    .activate_toolchain(worktree_id, toolchain, cx)
            }
        }
    }
    pub(crate) fn list_toolchains(
        &self,
        worktree_id: WorktreeId,
        language_name: LanguageName,
        cx: &AppContext,
    ) -> Task<Option<ToolchainList>> {
        match &self.0 {
            ToolchainStoreInner::Local(local, _) => {
                local
                    .read(cx)
                    .list_toolchains(worktree_id, language_name, cx)
            }
            ToolchainStoreInner::Remote(remote) => {
                remote
                    .read(cx)
                    .list_toolchains(worktree_id, language_name, cx)
            }
        }
    }
    pub(crate) fn active_toolchain(
        &self,
        worktree_id: WorktreeId,
        language_name: LanguageName,
        cx: &AppContext,
    ) -> Task<Option<Toolchain>> {
        match &self.0 {
            ToolchainStoreInner::Local(local, _) => {
                local
                    .read(cx)
                    .active_toolchain(worktree_id, language_name, cx)
            }
            ToolchainStoreInner::Remote(remote) => {
                remote
                    .read(cx)
                    .active_toolchain(worktree_id, language_name, cx)
            }
        }
    }
    async fn handle_activate_toolchain(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::ActivateToolchain>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::Ack> {
        this.update(&mut cx, |this, cx| {
            let language_name = LanguageName::from_proto(envelope.payload.language_name);
            let Some(toolchain) = envelope.payload.toolchain else {
                bail!("Missing `toolchain` in payload");
            };
            let toolchain = Toolchain {
                name: toolchain.name.into(),
                path: toolchain.path.into(),
                language_name,
            };
            let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
            Ok(this.activate_toolchain(worktree_id, toolchain, cx))
        })??
        .await;
        Ok(proto::Ack {})
    }
    async fn handle_active_toolchain(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::ActiveToolchain>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::ActiveToolchainResponse> {
        let toolchain = this
            .update(&mut cx, |this, cx| {
                let language_name = LanguageName::from_proto(envelope.payload.language_name);
                let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
                this.active_toolchain(worktree_id, language_name, cx)
            })?
            .await;

        Ok(proto::ActiveToolchainResponse {
            toolchain: toolchain.map(|toolchain| proto::Toolchain {
                name: toolchain.name.into(),
                path: toolchain.path.into(),
            }),
        })
    }

    async fn handle_list_toolchains(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::ListToolchains>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::ListToolchainsResponse> {
        let toolchains = this
            .update(&mut cx, |this, cx| {
                let language_name = LanguageName::from_proto(envelope.payload.language_name);
                let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
                this.list_toolchains(worktree_id, language_name, cx)
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
                .map(|toolchain| proto::Toolchain {
                    name: toolchain.name.to_string(),
                    path: toolchain.path.to_string(),
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
    pub(crate) fn as_language_toolchain_store(&self) -> Arc<dyn LanguageToolchainStore> {
        match &self.0 {
            ToolchainStoreInner::Local(local, _) => Arc::new(LocalStore(local.downgrade())),
            ToolchainStoreInner::Remote(remote) => Arc::new(RemoteStore(remote.downgrade())),
        }
    }
}

struct LocalToolchainStore {
    languages: Arc<LanguageRegistry>,
    worktree_store: Model<WorktreeStore>,
    active_toolchains: BTreeMap<(WorktreeId, LanguageName), Toolchain>,
}

#[async_trait(?Send)]
impl language::LanguageToolchainStore for LocalStore {
    async fn active_toolchain(
        self: Arc<Self>,
        worktree_id: WorktreeId,
        language_name: LanguageName,
        cx: &mut AsyncAppContext,
    ) -> Option<Toolchain> {
        self.0
            .update(cx, |this, cx| {
                this.active_toolchain(worktree_id, language_name, cx)
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
        language_name: LanguageName,
        cx: &mut AsyncAppContext,
    ) -> Option<Toolchain> {
        self.0
            .update(cx, |this, cx| {
                this.active_toolchain(worktree_id, language_name, cx)
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
        _: LanguageName,
        _: &mut AsyncAppContext,
    ) -> Option<Toolchain> {
        None
    }
}
struct LocalStore(WeakModel<LocalToolchainStore>);
struct RemoteStore(WeakModel<RemoteToolchainStore>);

#[derive(Clone)]
pub(crate) enum ToolchainStoreEvent {
    ToolchainActivated,
}

impl EventEmitter<ToolchainStoreEvent> for LocalToolchainStore {}

impl LocalToolchainStore {
    pub(crate) fn activate_toolchain(
        &self,
        worktree_id: WorktreeId,
        toolchain: Toolchain,
        cx: &mut ModelContext<Self>,
    ) -> Task<Option<()>> {
        cx.spawn(move |this, mut cx| async move {
            this.update(&mut cx, |this, cx| {
                this.active_toolchains.insert(
                    (worktree_id, toolchain.language_name.clone()),
                    toolchain.clone(),
                );
                cx.emit(ToolchainStoreEvent::ToolchainActivated);
            })
            .ok();
            Some(())
        })
    }
    pub(crate) fn list_toolchains(
        &self,
        worktree_id: WorktreeId,
        language_name: LanguageName,
        cx: &AppContext,
    ) -> Task<Option<ToolchainList>> {
        let registry = self.languages.clone();
        let Some(root) = self
            .worktree_store
            .read(cx)
            .worktree_for_id(worktree_id, cx)
            .map(|worktree| worktree.read(cx).abs_path())
        else {
            return Task::ready(None);
        };
        cx.spawn(|_| async move {
            let language = registry.language_for_name(&language_name.0).await.ok()?;
            let toolchains = language.toolchain_lister()?.list(root.to_path_buf()).await;
            Some(toolchains)
        })
    }
    pub(crate) fn active_toolchain(
        &self,
        worktree_id: WorktreeId,
        language_name: LanguageName,
        _: &AppContext,
    ) -> Task<Option<Toolchain>> {
        Task::ready(
            self.active_toolchains
                .get(&(worktree_id, language_name))
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
        worktree_id: WorktreeId,
        toolchain: Toolchain,
        cx: &AppContext,
    ) -> Task<Option<()>> {
        let project_id = self.project_id;
        let client = self.client.clone();
        cx.spawn(move |_| async move {
            let _ = client
                .request(proto::ActivateToolchain {
                    project_id,
                    worktree_id: worktree_id.to_proto(),
                    language_name: toolchain.language_name.into(),
                    toolchain: Some(proto::Toolchain {
                        name: toolchain.name.into(),
                        path: toolchain.path.into(),
                    }),
                })
                .await
                .log_err()?;
            Some(())
        })
    }
    pub(crate) fn list_toolchains(
        &self,
        worktree_id: WorktreeId,
        language_name: LanguageName,
        cx: &AppContext,
    ) -> Task<Option<ToolchainList>> {
        let project_id = self.project_id;
        let client = self.client.clone();
        cx.spawn(move |_| async move {
            let response = client
                .request(proto::ListToolchains {
                    project_id,
                    worktree_id: worktree_id.to_proto(),
                    language_name: language_name.clone().into(),
                })
                .await
                .log_err()?;
            if !response.has_values {
                return None;
            }
            let toolchains = response
                .toolchains
                .into_iter()
                .map(|toolchain| Toolchain {
                    language_name: language_name.clone(),
                    name: toolchain.name.into(),
                    path: toolchain.path.into(),
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
        worktree_id: WorktreeId,
        language_name: LanguageName,
        cx: &AppContext,
    ) -> Task<Option<Toolchain>> {
        let project_id = self.project_id;
        let client = self.client.clone();
        cx.spawn(move |_| async move {
            let response = client
                .request(proto::ActiveToolchain {
                    project_id,
                    worktree_id: worktree_id.to_proto(),
                    language_name: language_name.clone().into(),
                })
                .await
                .log_err()?;

            response.toolchain.map(|toolchain| Toolchain {
                language_name: language_name.clone(),
                name: toolchain.name.into(),
                path: toolchain.path.into(),
            })
        })
    }
}
