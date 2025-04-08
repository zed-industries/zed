use anyhow::Result;
use async_trait::async_trait;
use dap::DebugAdapterConfig;

#[async_trait]
pub(super) trait DapLocator: Send + Sync {
    async fn run_locator(&self, debug_config: &mut DebugAdapterConfig) -> Result<()>;
}
