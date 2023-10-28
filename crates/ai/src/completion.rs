use anyhow::Result;
use futures::{future::BoxFuture, stream::BoxStream};
use gpui::AppContext;

use crate::{
    auth::{CredentialProvider, ProviderCredential},
    models::LanguageModel,
};

pub trait CompletionRequest: Send + Sync {
    fn data(&self) -> serde_json::Result<String>;
}

pub trait CompletionProvider {
    fn base_model(&self) -> Box<dyn LanguageModel>;
    fn credential_provider(&self) -> Box<dyn CredentialProvider>;
    fn retrieve_credentials(&self, cx: &AppContext) -> ProviderCredential {
        self.credential_provider().retrieve_credentials(cx)
    }
    fn complete(
        &self,
        prompt: Box<dyn CompletionRequest>,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<String>>>>;
}
