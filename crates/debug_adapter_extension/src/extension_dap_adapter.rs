use std::{
    path::{Path, PathBuf},
    str::FromStr,
    sync::Arc,
};

use anyhow::{Context, Result};
use async_trait::async_trait;
use collections::HashMap;
use dap::{
    StartDebuggingRequestArgumentsRequest,
    adapters::{
        DapDelegate, DebugAdapter, DebugAdapterBinary, DebugAdapterName, DebugTaskDefinition,
    },
};
use extension::{Extension, WorktreeDelegate};
use gpui::AsyncApp;
use task::{DebugScenario, ZedDebugConfig};
use util::rel_path::RelPath;

pub(crate) struct ExtensionDapAdapter {
    extension: Arc<dyn Extension>,
    debug_adapter_name: Arc<str>,
    schema: serde_json::Value,
}

impl ExtensionDapAdapter {
    pub(crate) fn new(
        extension: Arc<dyn extension::Extension>,
        debug_adapter_name: Arc<str>,
        schema_path: &Path,
    ) -> Result<Self> {
        let schema = std::fs::read_to_string(&schema_path).with_context(|| {
            format!(
                "Failed to read debug adapter schema for {debug_adapter_name} (from path: `{schema_path:?}`)"
            )
        })?;
        let schema = serde_json::Value::from_str(&schema).with_context(|| {
            format!("Debug adapter schema for {debug_adapter_name} is not a valid JSON")
        })?;
        Ok(Self {
            extension,
            debug_adapter_name,
            schema,
        })
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
        self.0.worktree_root_path().to_string_lossy().into_owned()
    }

    async fn read_text_file(&self, path: &RelPath) -> Result<String> {
        self.0.read_text_file(path).await
    }

    async fn which(&self, binary_name: String) -> Option<String> {
        self.0
            .which(binary_name.as_ref())
            .await
            .map(|path| path.to_string_lossy().into_owned())
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

    fn dap_schema(&self) -> serde_json::Value {
        self.schema.clone()
    }

    async fn get_binary(
        &self,
        delegate: &Arc<dyn DapDelegate>,
        config: &DebugTaskDefinition,
        user_installed_path: Option<PathBuf>,
        // TODO support user args in the extension API
        _user_args: Option<Vec<String>>,
        // TODO support user env in the extension API
        _user_env: Option<HashMap<String, String>>,
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

    async fn config_from_zed_format(&self, zed_scenario: ZedDebugConfig) -> Result<DebugScenario> {
        self.extension.dap_config_to_scenario(zed_scenario).await
    }

    async fn request_kind(
        &self,
        config: &serde_json::Value,
    ) -> Result<StartDebuggingRequestArgumentsRequest> {
        self.extension
            .dap_request_kind(self.debug_adapter_name.clone(), config.clone())
            .await
    }
}
