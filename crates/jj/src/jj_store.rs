use std::sync::Arc;

use gpui::prelude::*;

use crate::JujutsuRepository;

pub struct JujutsuStore {
    repository: Arc<dyn JujutsuRepository>,
}

impl JujutsuStore {
    pub fn new(repository: Arc<dyn JujutsuRepository>, _cx: &mut Context<Self>) -> Self {
        Self { repository }
    }

    pub fn repository(&self) -> &Arc<dyn JujutsuRepository> {
        &self.repository
    }
}
