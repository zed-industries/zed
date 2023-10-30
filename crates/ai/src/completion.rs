use anyhow::Result;
use futures::{future::BoxFuture, stream::BoxStream};

use crate::{auth::CredentialProvider, models::LanguageModel};

pub trait CompletionRequest: Send + Sync {
    fn data(&self) -> serde_json::Result<String>;
}

pub trait CompletionProvider: CredentialProvider {
    fn base_model(&self) -> Box<dyn LanguageModel>;
    fn complete(
        &self,
        prompt: Box<dyn CompletionRequest>,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<String>>>>;
    fn box_clone(&self) -> Box<dyn CompletionProvider>;
}

impl Clone for Box<dyn CompletionProvider> {
    fn clone(&self) -> Box<dyn CompletionProvider> {
        self.box_clone()
    }
}
