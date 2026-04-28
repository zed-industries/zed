use std::future::Future;
use std::pin::Pin;

use anyhow::Result;
use gpui::AsyncApp;

/// A provider for credentials.
///
/// Used to abstract over reading and writing credentials to some form of
/// persistence (like the system keychain).
pub trait CredentialsProvider: Send + Sync {
    /// Reads the credentials from the provider.
    fn read_credentials<'a>(
        &'a self,
        url: &'a str,
        cx: &'a AsyncApp,
    ) -> Pin<Box<dyn Future<Output = Result<Option<(String, Vec<u8>)>>> + 'a>>;

    /// Writes the credentials to the provider.
    fn write_credentials<'a>(
        &'a self,
        url: &'a str,
        username: &'a str,
        password: &'a [u8],
        cx: &'a AsyncApp,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + 'a>>;

    /// Deletes the credentials from the provider.
    fn delete_credentials<'a>(
        &'a self,
        url: &'a str,
        cx: &'a AsyncApp,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + 'a>>;
}
