use std::sync::Arc;

use crate::{executor::Executor, Config, Result};

pub struct LlmState {
    pub config: Config,
    pub executor: Executor,
}

impl LlmState {
    pub async fn new(config: Config, executor: Executor) -> Result<Arc<Self>> {
        let this = Self { config, executor };

        Ok(Arc::new(this))
    }
}
