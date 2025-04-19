use anyhow::Result;
use async_trait::async_trait;
use task::DebugTaskDefinition;

pub(crate) mod cargo;

#[async_trait]
pub(super) trait DapLocator: Send + Sync {
    async fn run_locator(&self, debug_config: DebugTaskDefinition) -> Result<DebugTaskDefinition>;
}
