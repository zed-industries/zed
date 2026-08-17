use crate::error::Error;
use crate::kx::{SupportedKxGroup, ALL_KX_GROUPS};
use crate::suites::{SupportedCipherSuite, DEFAULT_CIPHER_SUITES};
use crate::versions;

use std::fmt;
use std::marker::PhantomData;

/// Building a [`ServerConfig`] or [`ClientConfig`] in a linker-friendly and
/// complete way.
///
/// Linker-friendly: meaning unused cipher suites, protocol
/// versions, key exchange mechanisms, etc. can be discarded
/// by the linker as they'll be unreferenced.
///
/// Complete: the type system ensures all decisions required to run a
/// server or client have been made by the time the process finishes.
///
/// Example, to make a [`ServerConfig`]:
///
/// ```no_run
/// # use rustls::ServerConfig;
/// # let certs = vec![];
/// # let private_key = rustls::PrivateKey(vec![]);
/// ServerConfig::builder()
///     .with_safe_default_cipher_suites()
///     .with_safe_default_kx_groups()
///     .with_safe_default_protocol_versions()
///     .unwrap()
///     .with_no_client_auth()
///     .with_single_cert(certs, private_key)
///     .expect("bad certificate/key");
/// ```
///
/// This may be shortened to:
///
/// ```no_run
/// # use rustls::ServerConfig;
/// # let certs = vec![];
/// # let private_key = rustls::PrivateKey(vec![]);
/// ServerConfig::builder()
///     .with_safe_defaults()
///     .with_no_client_auth()
///     .with_single_cert(certs, private_key)
///     .expect("bad certificate/key");
/// ```
///
/// To make a [`ClientConfig`]:
///
/// ```no_run
/// # use rustls::ClientConfig;
/// # let root_certs = rustls::RootCertStore::empty();
/// # let certs = vec![];
/// # let private_key = rustls::PrivateKey(vec![]);
/// ClientConfig::builder()
///     .with_safe_default_cipher_suites()
///     .with_safe_default_kx_groups()
///     .with_safe_default_protocol_versions()
///     .unwrap()
///     .with_root_certificates(root_certs)
///     .with_client_auth_cert(certs, private_key)
///     .expect("bad certificate/key");
/// ```
///
/// This may be shortened to:
///
/// ```
/// # use rustls::ClientConfig;
/// # let root_certs = rustls::RootCertStore::empty();
/// ClientConfig::builder()
///     .with_safe_defaults()
///     .with_root_certificates(root_certs)
///     .with_no_client_auth();
/// ```
///
/// The types used here fit together like this:
///
/// 1. Call [`ClientConfig::builder()`] or [`ServerConfig::builder()`] to initialize a builder.
/// 1. You must make a decision on which cipher suites to use, typically
///    by calling [`ConfigBuilder<S, WantsCipherSuites>::with_safe_default_cipher_suites()`].
/// 2. Now you must make a decision
///    on key exchange groups: typically by calling
///    [`ConfigBuilder<S, WantsKxGroups>::with_safe_default_kx_groups()`].
/// 3. Now you must make
///    a decision on which protocol versions to support, typically by calling
///    [`ConfigBuilder<S, WantsVersions>::with_safe_default_protocol_versions()`].
/// 5. Now see [`ConfigBuilder<ClientConfig, WantsVerifier>`] or
///    [`ConfigBuilder<ServerConfig, WantsVerifier>`] for further steps.
///
/// [`ServerConfig`]: crate::ServerConfig
/// [`ClientConfig`]: crate::ClientConfig
/// [`ClientConfig::builder()`]: crate::ClientConfig::builder()
/// [`ServerConfig::builder()`]: crate::ServerConfig::builder()
/// [`ConfigBuilder<ClientConfig, WantsVerifier>`]: struct.ConfigBuilder.html#impl-3
/// [`ConfigBuilder<ServerConfig, WantsVerifier>`]: struct.ConfigBuilder.html#impl-6
#[derive(Clone)]
pub struct ConfigBuilder<Side: ConfigSide, State> {
    pub(crate) state: State,
    pub(crate) side: PhantomData<Side>,
}

impl<Side: ConfigSide, State: fmt::Debug> fmt::Debug for ConfigBuilder<Side, State> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let side_name = std::any::type_name::<Side>();
        let side_name = side_name
            .split("::")
            .last()
            .unwrap_or(side_name);
        f.debug_struct(&format!("ConfigBuilder<{}, _>", side_name))
            .field("state", &self.state)
            .finish()
    }
}

/// Config builder state where the caller must supply cipher suites.
///
/// For more information, see the [`ConfigBuilder`] documentation.
#[derive(Clone, Debug)]
pub struct WantsCipherSuites(pub(crate) ());

impl<S: ConfigSide> ConfigBuilder<S, WantsCipherSuites> {
    /// Start side-specific config with defaults for underlying cryptography.
    ///
    /// If used, this will enable all safe supported cipher suites ([`DEFAULT_CIPHER_SUITES`]), all
    /// safe supported key exchange groups ([`ALL_KX_GROUPS`]) and all safe supported protocol
    /// versions ([`DEFAULT_VERSIONS`]).
    ///
    /// These are safe defaults, useful for 99% of applications.
    ///
    /// [`DEFAULT_VERSIONS`]: versions::DEFAULT_VERSIONS
    pub fn with_safe_defaults(self) -> ConfigBuilder<S, WantsVerifier> {
        ConfigBuilder {
            state: WantsVerifier {
                cipher_suites: DEFAULT_CIPHER_SUITES.to_vec(),
                kx_groups: ALL_KX_GROUPS.to_vec(),
                versions: versions::EnabledVersions::new(versions::DEFAULT_VERSIONS),
            },
            side: self.side,
        }
    }

    /// Choose a specific set of cipher suites.
    pub fn with_cipher_suites(
        self,
        cipher_suites: &[SupportedCipherSuite],
    ) -> ConfigBuilder<S, WantsKxGroups> {
        ConfigBuilder {
            state: WantsKxGroups {
                cipher_suites: cipher_suites.to_vec(),
            },
            side: self.side,
        }
    }

    /// Choose the default set of cipher suites ([`DEFAULT_CIPHER_SUITES`]).
    ///
    /// Note that this default provides only high-quality suites: there is no need
    /// to filter out low-, export- or NULL-strength cipher suites: rustls does not
    /// implement these.
    pub fn with_safe_default_cipher_suites(self) -> ConfigBuilder<S, WantsKxGroups> {
        self.with_cipher_suites(DEFAULT_CIPHER_SUITES)
    }
}

/// Config builder state where the caller must supply key exchange groups.
///
/// For more information, see the [`ConfigBuilder`] documentation.
#[derive(Clone, Debug)]
pub struct WantsKxGroups {
    cipher_suites: Vec<SupportedCipherSuite>,
}

impl<S: ConfigSide> ConfigBuilder<S, WantsKxGroups> {
    /// Choose a specific set of key exchange groups.
    pub fn with_kx_groups(
        self,
        kx_groups: &[&'static SupportedKxGroup],
    ) -> ConfigBuilder<S, WantsVersions> {
        ConfigBuilder {
            state: WantsVersions {
                cipher_suites: self.state.cipher_suites,
                kx_groups: kx_groups.to_vec(),
            },
            side: self.side,
        }
    }

    /// Choose the default set of key exchange groups ([`ALL_KX_GROUPS`]).
    ///
    /// This is a safe default: rustls doesn't implement any poor-quality groups.
    pub fn with_safe_default_kx_groups(self) -> ConfigBuilder<S, WantsVersions> {
        self.with_kx_groups(&ALL_KX_GROUPS)
    }
}

/// Config builder state where the caller must supply TLS protocol versions.
///
/// For more information, see the [`ConfigBuilder`] documentation.
#[derive(Clone, Debug)]
pub struct WantsVersions {
    cipher_suites: Vec<SupportedCipherSuite>,
    kx_groups: Vec<&'static SupportedKxGroup>,
}

impl<S: ConfigSide> ConfigBuilder<S, WantsVersions> {
    /// Accept the default protocol versions: both TLS1.2 and TLS1.3 are enabled.
    pub fn with_safe_default_protocol_versions(
        self,
    ) -> Result<ConfigBuilder<S, WantsVerifier>, Error> {
        self.with_protocol_versions(versions::DEFAULT_VERSIONS)
    }

    /// Use a specific set of protocol versions.
    pub fn with_protocol_versions(
        self,
        versions: &[&'static versions::SupportedProtocolVersion],
    ) -> Result<ConfigBuilder<S, WantsVerifier>, Error> {
        let mut any_usable_suite = false;
        for suite in &self.state.cipher_suites {
            if versions.contains(&suite.version()) {
                any_usable_suite = true;
                break;
            }
        }

        if !any_usable_suite {
            return Err(Error::General("no usable cipher suites configured".into()));
        }

        if self.state.kx_groups.is_empty() {
            return Err(Error::General("no kx groups configured".into()));
        }

        Ok(ConfigBuilder {
            state: WantsVerifier {
                cipher_suites: self.state.cipher_suites,
                kx_groups: self.state.kx_groups,
                versions: versions::EnabledVersions::new(versions),
            },
            side: self.side,
        })
    }
}

/// Config builder state where the caller must supply a verifier.
///
/// For more information, see the [`ConfigBuilder`] documentation.
#[derive(Clone, Debug)]
pub struct WantsVerifier {
    pub(crate) cipher_suites: Vec<SupportedCipherSuite>,
    pub(crate) kx_groups: Vec<&'static SupportedKxGroup>,
    pub(crate) versions: versions::EnabledVersions,
}

/// Helper trait to abstract [`ConfigBuilder`] over building a [`ClientConfig`] or [`ServerConfig`].
///
/// [`ClientConfig`]: crate::ClientConfig
/// [`ServerConfig`]: crate::ServerConfig
pub trait ConfigSide: sealed::Sealed {}

impl ConfigSide for crate::ClientConfig {}
impl ConfigSide for crate::ServerConfig {}

mod sealed {
    pub trait Sealed {}
    impl Sealed for crate::ClientConfig {}
    impl Sealed for crate::ServerConfig {}
}
