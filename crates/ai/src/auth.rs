use futures::future::BoxFuture;
use gpui::AppContext;

#[derive(Clone, Debug)]
pub enum ProviderCredential {
    Credentials { api_key: String },
    NoCredentials,
    NotNeeded,
}

pub trait CredentialProvider: Send + Sync {
    fn has_credentials(&self) -> bool;
    #[must_use]
    fn retrieve_credentials(&self, cx: &mut AppContext) -> BoxFuture<ProviderCredential>;
    #[must_use]
    fn save_credentials(
        &self,
        cx: &mut AppContext,
        credential: ProviderCredential,
    ) -> BoxFuture<()>;
    #[must_use]
    fn delete_credentials(&self, cx: &mut AppContext) -> BoxFuture<()>;
}
