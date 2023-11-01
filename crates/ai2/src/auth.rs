use async_trait::async_trait;
use gpui2::AppContext;

#[derive(Clone, Debug)]
pub enum ProviderCredential {
    Credentials { api_key: String },
    NoCredentials,
    NotNeeded,
}

pub trait CredentialProvider {
    fn has_credentials(&self) -> bool;
    fn retrieve_credentials(&self, cx: &mut AppContext) -> ProviderCredential;
    fn save_credentials(&self, cx: &mut AppContext, credential: ProviderCredential);
    fn delete_credentials(&self, cx: &mut AppContext);
}
