use anyhow::Result;
use futures::{future::BoxFuture, stream::BoxStream};

use crate::auth::CredentialProvider;

pub trait CompletionRequest: Send + Sync {
    fn data(&self) -> serde_json::Result<String>;
}

pub trait CompletionProvider: CredentialProvider {
    fn complete(
        &self,
        prompt: Box<dyn CompletionRequest>,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<String>>>>;
}
