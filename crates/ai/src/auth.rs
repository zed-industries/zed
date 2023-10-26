use gpui::AppContext;

#[derive(Clone, Debug)]
pub enum ProviderCredential {
    Credentials { api_key: String },
    NoCredentials,
    NotNeeded,
}

pub trait CredentialProvider: Send + Sync {
    fn retrieve_credentials(&self, cx: &AppContext) -> ProviderCredential;
}

#[derive(Clone)]
pub struct NullCredentialProvider;
impl CredentialProvider for NullCredentialProvider {
    fn retrieve_credentials(&self, _cx: &AppContext) -> ProviderCredential {
        ProviderCredential::NotNeeded
    }
}
