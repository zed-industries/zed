use gpui::AppContext;

#[derive(Clone, Debug)]
pub enum ProviderCredential {
    Credentials { api_key: String },
    NoCredentials,
    NotNeeded,
}

pub trait CredentialProvider: Send + Sync {
    fn retrieve_credentials(&self, cx: &AppContext) -> ProviderCredential;
    fn save_credentials(&self, cx: &AppContext, credential: ProviderCredential);
    fn delete_credentials(&self, cx: &AppContext);
}

#[derive(Clone)]
pub struct NullCredentialProvider;
impl CredentialProvider for NullCredentialProvider {
    fn retrieve_credentials(&self, _cx: &AppContext) -> ProviderCredential {
        ProviderCredential::NotNeeded
    }
    fn save_credentials(&self, cx: &AppContext, credential: ProviderCredential) {}
    fn delete_credentials(&self, cx: &AppContext) {}
}
