use anyhow::Result;
use async_trait::async_trait;
use task::TaskTemplate;

pub(crate) mod cargo;

#[async_trait]
pub(super) trait DapLocator: Send + Sync {
    async fn run_locator(&self, build_config: Option<TaskTemplate>) -> Result<TaskTemplate>;
}
