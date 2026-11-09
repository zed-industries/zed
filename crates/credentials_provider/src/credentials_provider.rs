#[cfg(feature = "test-support")]
pub mod test_support;

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

    /// Writes credentials only if none exist for the URL.
    ///
    /// Returns `true` if written, or `false` if credentials already exist and
    /// were left unchanged. The default reads and then writes, so a concurrent
    /// writer can still be overwritten. Providers override this when their
    /// storage can create atomically.
    ///
    /// # Errors
    ///
    /// Returns an error if the read or write fails. A failed read never writes.
    fn write_credentials_if_not_exists<'a>(
        &'a self,
        url: &'a str,
        username: &'a str,
        password: &'a [u8],
        cx: &'a AsyncApp,
    ) -> Pin<Box<dyn Future<Output = Result<bool>> + 'a>> {
        Box::pin(async move {
            if self.read_credentials(url, cx).await?.is_some() {
                return Ok(false);
            }
            self.write_credentials(url, username, password, cx).await?;
            Ok(true)
        })
    }

    /// Deletes the credentials from the provider.
    fn delete_credentials<'a>(
        &'a self,
        url: &'a str,
        cx: &'a AsyncApp,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + 'a>>;
}
