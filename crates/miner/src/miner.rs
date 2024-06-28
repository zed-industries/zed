mod language_model;

use futures::Stream;
use gpui::Model;
use project::Project;
use std::sync::Arc;

pub struct Miner {
    project: Model<Project>,
    language_model: Arc<dyn LanguageModel>,
}

impl Miner {
    pub fn new(project: Model<Project>, language_model: Arc<dyn LanguageModel>) -> Self {
        Self {
            project,
            language_model,
        }
    }
}

pub trait LanguageModel: Send + Sync {
    fn generate(&self) -> Box<dyn Stream<Item = String> + Send + Unpin>;
}
