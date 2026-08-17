#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

use rustls::{client::WantsClientCert, ClientConfig, ConfigBuilder, WantsVerifier};
use std::sync::Arc;

mod verification;
pub use verification::Verifier;

// Build the Android module when generating docs so that
// the Android-specific functions are included regardless of
// the host.
#[cfg(any(all(doc, docsrs), target_os = "android"))]
#[cfg_attr(docsrs, doc(cfg(target_os = "android")))]
pub mod android;

/// Fixures and data to support testing the server
/// certificate verifier.
#[cfg(any(test, feature = "ffi-testing"))]
mod tests;

// Re-export any exported functions that are required for
// tests to run in a platform-native environment.
#[cfg(feature = "ffi-testing")]
#[cfg_attr(feature = "ffi-testing", allow(unused_imports))]
pub use tests::ffi::*;

/// Creates and returns a `rustls` configuration that verifies TLS
/// certificates in the best way for the underlying OS platform, using
/// safe defaults for the `rustls` configuration.
///
/// # Example
///
/// This example shows how to use the custom verifier with the `reqwest` crate:
/// ```ignore
/// # use reqwest::ClientBuilder;
/// #[tokio::main]
/// use rustls_platform_verifier::ConfigVerifierExt;
///
/// async fn main() {
///     let client = ClientBuilder::new()
///         .use_preconfigured_tls(ClientConfig::with_platform_verifier())
///         .build()
///         .expect("nothing should fail");
///
///     let _response = client.get("https://example.com").send().await;
/// }
/// ```
///
/// **Important:** You must ensure that your `reqwest` version is using the same Rustls
/// version as this crate or it will panic when downcasting the `&dyn Any` verifier.
///
/// If you require more control over the rustls [`ClientConfig`], you can import the
/// [`BuilderVerifierExt`] trait and call `.with_platform_verifier()` on the [`ConfigBuilder`].
///
/// Refer to the crate level documentation to see what platforms
/// are currently supported.
#[deprecated(since = "0.4.0", note = "use the `ConfigVerifierExt` instead")]
pub fn tls_config() -> ClientConfig {
    ClientConfig::with_platform_verifier()
}

/// Attempts to construct a `rustls` configuration that verifies TLS certificates in the best way
/// for the underlying OS platform, using the provided
/// [`CryptoProvider`][rustls::crypto::CryptoProvider].
///
/// See [`tls_config`] for further documentation.
///
/// # Errors
///
/// Propagates any error returned by [`rustls::ConfigBuilder::with_safe_default_protocol_versions`].
#[deprecated(since = "0.4.0", note = "use the `BuilderVerifierExt` instead")]
pub fn tls_config_with_provider(
    provider: Arc<rustls::crypto::CryptoProvider>,
) -> Result<ClientConfig, rustls::Error> {
    Ok(ClientConfig::builder_with_provider(provider)
        .with_safe_default_protocol_versions()?
        .with_platform_verifier()
        .with_no_client_auth())
}

/// Exposed for debugging certificate issues with standalone tools.
///
/// This is not intended for production use, you should use [tls_config] instead.
#[cfg(feature = "dbg")]
pub fn verifier_for_dbg(root: &[u8]) -> Arc<dyn rustls::client::danger::ServerCertVerifier> {
    Arc::new(Verifier::new_with_fake_root(root))
}

/// Extension trait to help configure [`ClientConfig`]s with the platform verifier.
pub trait BuilderVerifierExt {
    /// Configures the `ClientConfig` with the platform verifier.
    ///
    /// ```rust
    /// use rustls::ClientConfig;
    /// use rustls_platform_verifier::BuilderVerifierExt;
    /// let config = ClientConfig::builder()
    ///     .with_platform_verifier()
    ///     .with_no_client_auth();
    /// ```
    fn with_platform_verifier(self) -> ConfigBuilder<ClientConfig, WantsClientCert>;
}

impl BuilderVerifierExt for ConfigBuilder<ClientConfig, WantsVerifier> {
    fn with_platform_verifier(self) -> ConfigBuilder<ClientConfig, WantsClientCert> {
        let provider = self.crypto_provider().clone();
        self.dangerous()
            .with_custom_certificate_verifier(Arc::new(Verifier::new().with_provider(provider)))
    }
}

/// Extension trait to help build a [`ClientConfig`] with the platform verifier.
pub trait ConfigVerifierExt {
    /// Build a [`ClientConfig`] with the platform verifier and the default `CryptoProvider`.
    ///
    /// ```rust
    /// use rustls::ClientConfig;
    /// use rustls_platform_verifier::ConfigVerifierExt;
    /// let config = ClientConfig::with_platform_verifier();
    /// ```
    fn with_platform_verifier() -> ClientConfig;
}

impl ConfigVerifierExt for ClientConfig {
    fn with_platform_verifier() -> ClientConfig {
        ClientConfig::builder()
            .with_platform_verifier()
            .with_no_client_auth()
    }
}
