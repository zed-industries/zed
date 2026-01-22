use std::collections::HashMap;
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::{Arc, LazyLock};

use anyhow::Result;
use futures::FutureExt as _;
use gpui::{App, AsyncApp};
use release_channel::ReleaseChannel;

/// An environment variable whose presence indicates that the system keychain
/// should be used in development.
///
/// By default, running Zed in development uses the development credentials
/// provider. Setting this environment variable allows you to interact with the
/// system keychain (for instance, if you need to test something).
///
/// Only works in development. Setting this environment variable in other
/// release channels is a no-op.
static ZED_DEVELOPMENT_USE_KEYCHAIN: LazyLock<bool> = LazyLock::new(|| {
    std::env::var("ZED_DEVELOPMENT_USE_KEYCHAIN").is_ok_and(|value| !value.is_empty())
});

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

impl dyn CredentialsProvider {
    /// Returns the global [`CredentialsProvider`].
    pub fn global(cx: &App) -> Arc<Self> {
        // The `CredentialsProvider` trait has `Send + Sync` bounds on it, so it
        // seems like this is a false positive from Clippy.
        #[allow(clippy::arc_with_non_send_sync)]
        Self::new(cx)
    }

    fn new(cx: &App) -> Arc<Self> {
        let use_development_provider = match ReleaseChannel::try_global(cx) {
            Some(ReleaseChannel::Dev) => {
                // In development we default to using the development
                // credentials provider to avoid getting spammed by relentless
                // keychain access prompts.
                //
                // However, if the `ZED_DEVELOPMENT_USE_KEYCHAIN` environment
                // variable is set, we will use the actual keychain.
                !*ZED_DEVELOPMENT_USE_KEYCHAIN
            }
            Some(ReleaseChannel::Nightly | ReleaseChannel::Preview | ReleaseChannel::Stable)
            | None => false,
        };

        if use_development_provider {
            Arc::new(DevelopmentCredentialsProvider::new())
        } else {
            Arc::new(KeychainCredentialsProvider)
        }
    }
}

/// A credentials provider that stores credentials in the system keychain.
struct KeychainCredentialsProvider;

impl CredentialsProvider for KeychainCredentialsProvider {
    fn read_credentials<'a>(
        &'a self,
        url: &'a str,
        cx: &'a AsyncApp,
    ) -> Pin<Box<dyn Future<Output = Result<Option<(String, Vec<u8>)>>> + 'a>> {
        async move { cx.update(|cx| cx.read_credentials(url)).await }.boxed_local()
    }

    fn write_credentials<'a>(
        &'a self,
        url: &'a str,
        username: &'a str,
        password: &'a [u8],
        cx: &'a AsyncApp,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + 'a>> {
        async move {
            cx.update(move |cx| cx.write_credentials(url, username, password))
                .await
        }
        .boxed_local()
    }

    fn delete_credentials<'a>(
        &'a self,
        url: &'a str,
        cx: &'a AsyncApp,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + 'a>> {
        async move { cx.update(move |cx| cx.delete_credentials(url)).await }.boxed_local()
    }
}

/// A credentials provider that stores credentials in a local file.
///
/// This MUST only be used in development, as this is not a secure way of storing
/// credentials on user machines.
///
/// Its existence is purely to work around the annoyance of having to constantly
/// re-allow access to the system keychain when developing Zed.
struct DevelopmentCredentialsProvider {
    path: PathBuf,
}

impl DevelopmentCredentialsProvider {
    fn new() -> Self {
        let path = paths::config_dir().join("development_credentials");

        Self { path }
    }

    fn load_credentials(&self) -> Result<HashMap<String, (String, Vec<u8>)>> {
        let json = std::fs::read(&self.path)?;
        let credentials: HashMap<String, (String, Vec<u8>)> = serde_json::from_slice(&json)?;

        Ok(credentials)
    }

    fn save_credentials(&self, credentials: &HashMap<String, (String, Vec<u8>)>) -> Result<()> {
        let json = serde_json::to_string(credentials)?;
        std::fs::write(&self.path, json)?;

        Ok(())
    }
}

impl CredentialsProvider for DevelopmentCredentialsProvider {
    fn read_credentials<'a>(
        &'a self,
        url: &'a str,
        _cx: &'a AsyncApp,
    ) -> Pin<Box<dyn Future<Output = Result<Option<(String, Vec<u8>)>>> + 'a>> {
        async move {
            Ok(self
                .load_credentials()
                .unwrap_or_default()
                .get(url)
                .cloned())
        }
        .boxed_local()
    }

    fn write_credentials<'a>(
        &'a self,
        url: &'a str,
        username: &'a str,
        password: &'a [u8],
        _cx: &'a AsyncApp,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + 'a>> {
        async move {
            let mut credentials = self.load_credentials().unwrap_or_default();
            credentials.insert(url.to_string(), (username.to_string(), password.to_vec()));

            self.save_credentials(&credentials)
        }
        .boxed_local()
    }

    fn delete_credentials<'a>(
        &'a self,
        url: &'a str,
        _cx: &'a AsyncApp,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + 'a>> {
        async move {
            let mut credentials = self.load_credentials()?;
            credentials.remove(url);

            self.save_credentials(&credentials)
        }
        .boxed_local()
    }
}
