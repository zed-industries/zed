use anyhow::Result;
use async_trait::async_trait;
use task::DebugTaskDefinition;

#[async_trait]
pub(super) trait DapLocator: Send + Sync {
    async fn run_locator(&self, debug_config: &mut DebugTaskDefinition) -> Result<()>;
}
