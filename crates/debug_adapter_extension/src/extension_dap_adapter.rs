use std::{path::PathBuf, sync::Arc};

use anyhow::Result;
use async_trait::async_trait;
use dap::adapters::{
    DapDelegate, DebugAdapter, DebugAdapterBinary, DebugAdapterName, DebugTaskDefinition,
};
use extension::{Extension, WorktreeDelegate};
use gpui::AsyncApp;

pub(crate) struct ExtensionDapAdapter {
    extension: Arc<dyn Extension>,
    debug_adapter_name: Arc<str>,
}

impl ExtensionDapAdapter {
    pub(crate) fn new(
        extension: Arc<dyn extension::Extension>,
        debug_adapter_name: Arc<str>,
    ) -> Self {
        Self {
            extension,
            debug_adapter_name,
        }
    }
}

/// An adapter that allows an [`dap::adapters::DapDelegate`] to be used as a [`WorktreeDelegate`].
struct WorktreeDelegateAdapter(pub Arc<dyn DapDelegate>);

#[async_trait]
impl WorktreeDelegate for WorktreeDelegateAdapter {
    fn id(&self) -> u64 {
        self.0.worktree_id().to_proto()
    }

    fn root_path(&self) -> String {
        self.0.worktree_root_path().to_string_lossy().to_string()
    }

    async fn read_text_file(&self, path: PathBuf) -> Result<String> {
        self.0.read_text_file(path).await
    }

    async fn which(&self, binary_name: String) -> Option<String> {
        self.0
            .which(binary_name.as_ref())
            .await
            .map(|path| path.to_string_lossy().to_string())
    }

    async fn shell_env(&self) -> Vec<(String, String)> {
        self.0.shell_env().await.into_iter().collect()
    }
}

#[async_trait(?Send)]
impl DebugAdapter for ExtensionDapAdapter {
    fn name(&self) -> DebugAdapterName {
        self.debug_adapter_name.as_ref().into()
    }

    async fn get_binary(
        &self,
        delegate: &Arc<dyn DapDelegate>,
        config: &DebugTaskDefinition,
        user_installed_path: Option<PathBuf>,
        _cx: &mut AsyncApp,
    ) -> Result<DebugAdapterBinary> {
        self.extension
            .get_dap_binary(
                self.debug_adapter_name.clone(),
                config.clone(),
                user_installed_path,
                Arc::new(WorktreeDelegateAdapter(delegate.clone())),
            )
            .await
    }
}
