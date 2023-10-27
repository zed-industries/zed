use anyhow::Result;
use futures::{future::BoxFuture, stream::BoxStream};

use crate::models::LanguageModel;

pub trait CompletionRequest: Send + Sync {
    fn data(&self) -> serde_json::Result<String>;
}

pub trait CompletionProvider {
    fn base_model(&self) -> Box<dyn LanguageModel>;
    fn complete(
        &self,
        prompt: Box<dyn CompletionRequest>,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<String>>>>;
}
