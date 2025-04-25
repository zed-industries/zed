use anyhow::Result;
use async_trait::async_trait;
use dap::DebugRequest;
use task::SpawnInTerminal;

pub(crate) mod cargo;

/// Given a user build configuration, locator creates a fill-in debug target ([DebugRequest]) on behalf of the user.
#[async_trait]
pub(super) trait DapLocator: Send + Sync {
    /// Determines whether this locator can generate debug target for given task.
    fn accepts(&self, build_config: &SpawnInTerminal) -> bool;
    async fn run(&self, build_config: SpawnInTerminal) -> Result<DebugRequest>;
}
