use std::sync::Arc;

use gpui::{AppContext, Model, Task};
use language::{LanguageName, LanguageRegistry, Toolchain, ToolchainList};
use rpc::AnyProtoClient;

use crate::{worktree_store::WorktreeStore, LspStore};

pub enum ToolchainStore {
    Local(LocalToolchainStore),
    Remote(RemoteToolchainStore),
}

impl ToolchainStore {
    pub fn local(
        languages: Arc<LanguageRegistry>,
        worktree_store: Model<WorktreeStore>,
        lsp_store: Model<LspStore>,
    ) -> Self {
        Self::Local(LocalToolchainStore {
            languages,
            worktree_store,
            lsp_store,
        })
    }
    pub fn remote(client: AnyProtoClient) -> Self {
        Self::Remote(RemoteToolchainStore {})
    }
    pub(crate) fn activate_toolchain(
        &self,
        toolchain: Toolchain,
        cx: &mut AppContext,
    ) -> Task<Option<()>> {
        match self {
            ToolchainStore::Local(local) => local.activate_toolchain(toolchain, cx),
            ToolchainStore::Remote(remote) => remote.activate_toolchain(toolchain, cx),
        }
    }
    pub(crate) fn list_toolchains(
        &self,
        language_name: LanguageName,
        cx: &AppContext,
    ) -> Task<Option<ToolchainList>> {
        match self {
            ToolchainStore::Local(local) => local.list_toolchains(language_name, cx),
            ToolchainStore::Remote(remote) => remote.list_toolchains(language_name, cx),
        }
    }
    pub(crate) fn active_toolchain(&self, cx: &mut AppContext) -> Task<()> {
        match self {
            ToolchainStore::Local(local) => local.active_toolchain(cx),
            ToolchainStore::Remote(remote) => remote.active_toolchain(cx),
        }
    }
}

struct LocalToolchainStore {
    languages: Arc<LanguageRegistry>,
    worktree_store: Model<WorktreeStore>,
    lsp_store: Model<LspStore>,
}

impl LocalToolchainStore {
    pub(crate) fn activate_toolchain(
        &self,
        toolchain: Toolchain,
        cx: &mut AppContext,
    ) -> Task<Option<()>> {
        let registry = self.languages.clone();
        let lsp_store = self.lsp_store.downgrade();
        cx.spawn(move |cx| async move {
            let language = registry
                .language_for_name(&toolchain.language_name.0)
                .await
                .ok()?;
            language.toolchain_lister()?.activate(toolchain).await;
            LspStore::refresh_workspace_configurations(&lsp_store, cx).await;
            Some(())
        })
    }
    pub(crate) fn list_toolchains(
        &self,
        language_name: LanguageName,
        cx: &AppContext,
    ) -> Task<Option<ToolchainList>> {
        let registry = self.languages.clone();
        let Some(root) = self
            .worktree_store
            .read(cx)
            .worktrees()
            .next()
            .map(|worktree| worktree.read(cx).abs_path())
        else {
            return Task::ready(None);
        };
        cx.spawn(|cx| async move {
            let language = registry.language_for_name(&language_name.0).await.ok()?;
            let toolchains = language.toolchain_lister()?.list(root.to_path_buf()).await;
            Some(toolchains)
        })
    }
    pub(crate) fn active_toolchain(&self, cx: &mut AppContext) -> Task<()> {
        Task::ready(())
    }
}
struct RemoteToolchainStore {}

impl RemoteToolchainStore {
    pub(crate) fn activate_toolchain(
        &self,
        toolchain: Toolchain,
        cx: &mut AppContext,
    ) -> Task<Option<()>> {
        Task::ready(None)
    }
    pub(crate) fn list_toolchains(
        &self,
        language_name: LanguageName,
        cx: &AppContext,
    ) -> Task<Option<ToolchainList>> {
        Task::ready(None)
    }
    pub(crate) fn active_toolchain(&self, cx: &mut AppContext) -> Task<()> {
        Task::ready(())
    }
}
