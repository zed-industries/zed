use std::sync::Arc;

use anyhow::{bail, Result};

use async_trait::async_trait;
use collections::BTreeMap;
use gpui::{AppContext, AsyncAppContext, Context, Model, ModelContext, Task, WeakModel};
use language::{LanguageName, LanguageRegistry, Toolchain, ToolchainList, ToolchainLister};
use rpc::{proto, AnyProtoClient, TypedEnvelope};
use settings::WorktreeId;

use crate::{worktree_store::WorktreeStore, LspStore};

pub(crate) enum ToolchainStore {
    Local(Model<LocalToolchainStore>),
    Remote(Model<RemoteToolchainStore>),
}

impl ToolchainStore {
    pub(super) fn init(client: &AnyProtoClient) {
        client.add_model_request_handler(Self::handle_activate_toolchain);
    }

    pub(super) fn local(
        languages: Arc<LanguageRegistry>,
        worktree_store: Model<WorktreeStore>,
        lsp_store: WeakModel<LspStore>,
        cx: &mut AppContext,
    ) -> Self {
        Self::Local(cx.new_model(|_| LocalToolchainStore {
            languages,
            worktree_store,
            lsp_store,
            active_toolchains: Default::default(),
        }))
    }
    pub(super) fn remote(client: AnyProtoClient, cx: &mut AppContext) -> Self {
        Self::Remote(cx.new_model(|_| RemoteToolchainStore { client }))
    }
    pub(crate) fn activate_toolchain(
        &self,
        worktree_id: WorktreeId,
        toolchain: Toolchain,
        cx: &mut AppContext,
    ) -> Task<Option<()>> {
        match self {
            ToolchainStore::Local(local) => local.update(cx, |this, cx| {
                this.activate_toolchain(worktree_id, toolchain, cx)
            }),
            ToolchainStore::Remote(remote) => {
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
        match self {
            ToolchainStore::Local(local) => {
                local
                    .read(cx)
                    .list_toolchains(worktree_id, language_name, cx)
            }
            ToolchainStore::Remote(remote) => {
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
        match self {
            ToolchainStore::Local(local) => {
                local
                    .read(cx)
                    .active_toolchain(worktree_id, language_name, cx)
            }
            ToolchainStore::Remote(remote) => {
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
                label: toolchain.name.into(),
                path: toolchain.path.into(),
                language_name,
            };
            let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
            Ok(this.activate_toolchain(worktree_id, toolchain, cx))
        })??
        .await;
        Ok(proto::Ack {})
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
        let toolchains = if let Some(toolchains) = toolchains {
            toolchains
                .toolchains
                .into_iter()
                .map(|toolchain| proto::Toolchain {
                    name: toolchain.label.to_string(),
                    path: toolchain.path.to_string(),
                })
                .collect::<Vec<_>>()
        } else {
            vec![]
        };
        Ok(proto::ListToolchainsResponse {
            has_values,
            toolchains,
        })
    }
}

struct LocalToolchainStore {
    languages: Arc<LanguageRegistry>,
    worktree_store: Model<WorktreeStore>,
    lsp_store: WeakModel<LspStore>,
    active_toolchains: BTreeMap<(WorktreeId, LanguageName), Toolchain>,
}

#[async_trait(?Send)]
impl language::ToolchainStore for LocalStore {
    async fn active_toolchain(
        self: Arc<Self>,
        worktree_id: WorktreeId,
        language_name: LanguageName,
        cx: &mut AppContext,
    ) -> Option<Toolchain> {
        self.0
            .update(cx, |this, cx| {
                this.active_toolchain(worktree_id, language_name, cx)
            })
            .ok()?
            .await
    }
}
struct LocalStore(WeakModel<LocalToolchainStore>);

impl LocalToolchainStore {
    pub(crate) fn activate_toolchain(
        &self,
        worktree_id: WorktreeId,
        toolchain: Toolchain,
        cx: &mut ModelContext<Self>,
    ) -> Task<Option<()>> {
        let lsp_store = self.lsp_store.clone();
        cx.spawn(move |this, mut cx| async move {
            this.update(&mut cx, |this, _| {
                this.active_toolchains
                    .insert((worktree_id, toolchain.language_name.clone()), toolchain)
            });
            LspStore::refresh_workspace_configurations(&lsp_store, Arc::new(LocalStore(this)), cx)
                .await;
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
        cx: &AppContext,
    ) -> Task<Option<Toolchain>> {
        Task::ready(None)
    }
}
struct RemoteToolchainStore {
    client: AnyProtoClient,
}

impl RemoteToolchainStore {
    pub(crate) fn activate_toolchain(
        &self,
        worktree_id: WorktreeId,
        toolchain: Toolchain,
        cx: &AppContext,
    ) -> Task<Option<()>> {
        Task::ready(None)
    }
    pub(crate) fn list_toolchains(
        &self,
        worktree_id: WorktreeId,
        language_name: LanguageName,
        cx: &AppContext,
    ) -> Task<Option<ToolchainList>> {
        Task::ready(None)
    }
    pub(crate) fn active_toolchain(
        &self,
        worktree_id: WorktreeId,
        language_name: LanguageName,
        cx: &AppContext,
    ) -> Task<Option<Toolchain>> {
        Task::ready(None)
    }
}
