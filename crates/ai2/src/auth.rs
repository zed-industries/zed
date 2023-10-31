use async_trait::async_trait;
use gpui2::AppContext;

#[derive(Clone, Debug)]
pub enum ProviderCredential {
    Credentials { api_key: String },
    NoCredentials,
    NotNeeded,
}

#[async_trait]
pub trait CredentialProvider: Send + Sync {
    fn has_credentials(&self) -> bool;
    async fn retrieve_credentials(&self, cx: &mut AppContext) -> ProviderCredential;
    async fn save_credentials(&self, cx: &mut AppContext, credential: ProviderCredential);
    async fn delete_credentials(&self, cx: &mut AppContext);
}
