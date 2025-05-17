use std::sync::Arc;

use gpui::prelude::*;

use crate::JjRepository;

pub struct JjStore {
    repository: Arc<dyn JjRepository>,
}

impl JjStore {
    pub fn new(repository: Arc<dyn JjRepository>, cx: &mut Context<Self>) -> Self {
        Self { repository }
    }

    pub fn repository(&self) -> &Arc<dyn JjRepository> {
        &self.repository
    }
}
