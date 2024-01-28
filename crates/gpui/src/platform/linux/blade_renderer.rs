use std::sync::Arc;

pub struct BladeRenderer {
    gpu: Arc<blade::Context>,
}

impl BladeRenderer {
    pub fn new(gpu: Arc<blade::Context>) -> Self {
        Self { gpu }
    }
}
