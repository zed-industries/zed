use anyhow::Result;
use async_trait::async_trait;
use dap::{DapLocator, DebugRequest, adapters::DebugAdapterName};
use extension::Extension;
use gpui::SharedString;
use std::sync::Arc;
use task::{DebugScenario, SpawnInTerminal, TaskTemplate};

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
        _build_config: &TaskTemplate,
        _resolved_label: &str,
        _adapter: &DebugAdapterName,
    ) -> Option<DebugScenario> {
        _ = self.extension.clone();
        None
    }

    async fn run(&self, _build_config: SpawnInTerminal) -> Result<DebugRequest> {
        Err(anyhow::anyhow!("Not implemented"))
    }
}
