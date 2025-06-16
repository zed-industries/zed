use std::{
    path::{Path, PathBuf},
    str::FromStr,
    sync::Arc,
};

use anyhow::{Context, Result};
use async_trait::async_trait;
use dap::{
    DapLocator, DebugRequest,
    adapters::{
        DapDelegate, DebugAdapter, DebugAdapterBinary, DebugAdapterName, DebugTaskDefinition,
    },
};
use extension::{Extension, WorktreeDelegate};
use gpui::{AsyncApp, SharedString};
use task::{DebugScenario, SpawnInTerminal, TaskTemplate, ZedDebugConfig};

pub(crate) struct ExtensionLocatorAdapter {
    extension: Arc<dyn Extension>,
    locator_name: SharedString,
}

impl ExtensionLocatorAdapter {
    pub(crate) fn new(extension: Arc<dyn extension::Extension>, locator_name: Arc<str>) -> Self {
        Self {
            extension,
            locator_name: SharedString::from(locator_name),
        }
    }
}

#[async_trait]
impl DapLocator for ExtensionLocatorAdapter {
    fn name(&self) -> SharedString {
        self.locator_name.clone()
    }
    /// Determines whether this locator can generate debug target for given task.
    async fn create_scenario(
        &self,
        build_config: &TaskTemplate,
        resolved_label: &str,
        adapter: &DebugAdapterName,
    ) -> Option<DebugScenario> {
        None
    }

    async fn run(&self, build_config: SpawnInTerminal) -> Result<DebugRequest> {
        Err(anyhow::anyhow!("Not implemented"))
    }
}
