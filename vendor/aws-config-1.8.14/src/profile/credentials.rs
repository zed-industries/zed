/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Profile File Based Credential Providers
//!
//! Profile file based providers combine two pieces:
//!
//! 1. Parsing and resolution of the assume role chain
//! 2. A user-modifiable hashmap of provider name to provider.
//!
//! Profile file based providers first determine the chain of providers that will be used to load
//! credentials. After determining and validating this chain, a `Vec` of providers will be created.
//!
//! Each subsequent provider will provide boostrap providers to the next provider in order to load
//! the final credentials.
//!
//! This module contains two sub modules:
//! - `repr` which contains an abstract representation of a provider chain and the logic to
//!   build it from `~/.aws/credentials` and `~/.aws/config`.
//! - `exec` which contains a chain representation of providers to implement passing bootstrapped credentials
//!   through a series of providers.

use crate::profile::cell::ErrorTakingOnceCell;
#[allow(deprecated)]
use crate::profile::profile_file::ProfileFiles;
use crate::profile::Profile;
use crate::profile::ProfileFileLoadError;
use crate::provider_config::ProviderConfig;
use aws_credential_types::credential_feature::AwsCredentialFeature;
use aws_credential_types::{
    provider::{self, error::CredentialsError, future, ProvideCredentials},
    Credentials,
};
use aws_smithy_types::error::display::DisplayErrorContext;
use std::borrow::Cow;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::sync::Arc;
use tracing::Instrument;

mod exec;
pub(crate) mod repr;

/// AWS Profile based credentials provider
///
/// This credentials provider will load credentials from `~/.aws/config` and `~/.aws/credentials`.
/// The locations of these files are configurable via environment variables, see [below](#location-of-profile-files).
///
/// Generally, this will be constructed via the default provider chain, however, it can be manually
/// constructed with the builder:
/// ```rust,no_run
/// use aws_config::profile::ProfileFileCredentialsProvider;
/// let provider = ProfileFileCredentialsProvider::builder().build();
/// ```
///
/// _Note: Profile providers, when called, will load and parse the profile from the file system
/// only once. Parsed file contents will be cached indefinitely._
///
/// This provider supports several different credentials formats:
/// ### Credentials defined explicitly within the file
/// ```ini
/// [default]
/// aws_access_key_id = 123
/// aws_secret_access_key = 456
/// ```
///
/// ### Assume Role Credentials loaded from a credential source
/// ```ini
/// [default]
/// role_arn = arn:aws:iam::123456789:role/RoleA
/// credential_source = Environment
/// ```
///
/// NOTE: Currently only the `Environment` credential source is supported although it is possible to
/// provide custom sources:
/// ```no_run
/// use aws_credential_types::provider::{self, future, ProvideCredentials};
/// use aws_config::profile::ProfileFileCredentialsProvider;
/// #[derive(Debug)]
/// struct MyCustomProvider;
/// impl MyCustomProvider {
///     async fn load_credentials(&self) -> provider::Result {
///         todo!()
///     }
/// }
///
/// impl ProvideCredentials for MyCustomProvider {
///   fn provide_credentials<'a>(&'a self) -> future::ProvideCredentials where Self: 'a {
///         future::ProvideCredentials::new(self.load_credentials())
///     }
/// }
/// # if cfg!(feature = "default-https-client") {
/// let provider = ProfileFileCredentialsProvider::builder()
///     .with_custom_provider("Custom", MyCustomProvider)
///     .build();
/// }
/// ```
///
/// ### Assume role credentials from a source profile
/// ```ini
/// [default]
/// role_arn = arn:aws:iam::123456789:role/RoleA
/// source_profile = base
///
/// [profile base]
/// aws_access_key_id = 123
/// aws_secret_access_key = 456
/// ```
///
/// Other more complex configurations are possible, consult `test-data/assume-role-tests.json`.
///
/// ### Credentials loaded from an external process
/// ```ini
/// [default]
/// credential_process = /opt/bin/awscreds-custom --username helen
/// ```
///
/// An external process can be used to provide credentials.
///
/// ### Loading Credentials from SSO
/// ```ini
/// [default]
/// sso_start_url = https://example.com/start
/// sso_region = us-east-2
/// sso_account_id = 123456789011
/// sso_role_name = readOnly
/// region = us-west-2
/// ```
///
/// SSO can also be used as a source profile for assume role chains.
///
/// ### Credentials from a console session
///
/// An existing AWS Console session can be used to provide credentials.
///
/// ```ini
/// [default]
/// login_session = arn:aws:iam::0123456789012:user/Admin
/// ```
///
#[doc = include_str!("location_of_profile_files.md")]
#[derive(Debug)]
pub struct ProfileFileCredentialsProvider {
    config: Arc<Config>,
    inner_provider: ErrorTakingOnceCell<ChainProvider, CredentialsError>,
}

#[derive(Debug)]
struct Config {
    factory: exec::named::NamedProviderFactory,
    provider_config: ProviderConfig,
}

impl ProfileFileCredentialsProvider {
    /// Builder for this credentials provider
    pub fn builder() -> Builder {
        Builder::default()
    }

    async fn load_credentials(&self) -> provider::Result {
        // The inner provider needs to be cached across successive calls to load_credentials
        // since the base providers can potentially have information cached in their instances.
        // For example, the SsoCredentialsProvider maintains an in-memory expiring token cache.
        let inner_provider = self
            .inner_provider
            .get_or_init(
                {
                    let config = self.config.clone();
                    move || async move {
                        match build_provider_chain(config.clone()).await {
                            Ok(chain) => Ok(ChainProvider {
                                config: config.clone(),
                                chain: Some(Arc::new(chain)),
                            }),
                            Err(err) => match err {
                                ProfileFileError::NoProfilesDefined
                                | ProfileFileError::ProfileDidNotContainCredentials { .. } => {
                                    Ok(ChainProvider {
                                        config: config.clone(),
                                        chain: None,
                                    })
                                }
                                _ => Err(CredentialsError::invalid_configuration(format!(
                                    "ProfileFile provider could not be built: {}",
                                    &err
                                ))),
                            },
                        }
                    }
                },
                CredentialsError::unhandled(
                    "profile file credentials provider initialization error already taken",
                ),
            )
            .await?;
        inner_provider.provide_credentials().await.map(|mut creds| {
            creds
                .get_property_mut_or_default::<Vec<AwsCredentialFeature>>()
                .push(AwsCredentialFeature::CredentialsProfile);
            creds
        })
    }
}

impl ProvideCredentials for ProfileFileCredentialsProvider {
    fn provide_credentials<'a>(&'a self) -> future::ProvideCredentials<'a>
    where
        Self: 'a,
    {
        future::ProvideCredentials::new(self.load_credentials())
    }
}

/// An Error building a Credential source from an AWS Profile
#[derive(Debug)]
#[non_exhaustive]
pub enum ProfileFileError {
    /// The profile was not a valid AWS profile
    #[non_exhaustive]
    InvalidProfile(ProfileFileLoadError),

    /// No profiles existed (the profile was empty)
    #[non_exhaustive]
    NoProfilesDefined,

    /// The profile did not contain any credential information
    #[non_exhaustive]
    ProfileDidNotContainCredentials {
        /// The name of the profile
        profile: String,
    },

    /// The profile contained an infinite loop of `source_profile` references
    #[non_exhaustive]
    CredentialLoop {
        /// Vec of profiles leading to the loop
        profiles: Vec<String>,
        /// The next profile that caused the loop
        next: String,
    },

    /// The profile was missing a credential source
    #[non_exhaustive]
    MissingCredentialSource {
        /// The name of the profile
        profile: String,
        /// Error message
        message: Cow<'static, str>,
    },
    /// The profile contained an invalid credential source
    #[non_exhaustive]
    InvalidCredentialSource {
        /// The name of the profile
        profile: String,
        /// Error message
        message: Cow<'static, str>,
    },
    /// The profile referred to a another profile by name that was not defined
    #[non_exhaustive]
    MissingProfile {
        /// The name of the profile
        profile: String,
        /// Error message
        message: Cow<'static, str>,
    },
    /// The profile referred to `credential_source` that was not defined
    #[non_exhaustive]
    UnknownProvider {
        /// The name of the provider
        name: String,
    },

    /// Feature not enabled
    #[non_exhaustive]
    FeatureNotEnabled {
        /// The feature or comma delimited list of features that must be enabled
        feature: Cow<'static, str>,
        /// Additional information about the missing feature
        message: Option<Cow<'static, str>>,
    },

    /// Missing sso-session section in config
    #[non_exhaustive]
    MissingSsoSession {
        /// The name of the profile that specified `sso_session`
        profile: String,
        /// SSO session name
        sso_session: String,
    },

    /// Invalid SSO configuration
    #[non_exhaustive]
    InvalidSsoConfig {
        /// The name of the profile that the error originates in
        profile: String,
        /// Error message
        message: Cow<'static, str>,
    },

    /// Profile is intended to be used in the token provider chain rather
    /// than in the credentials chain.
    #[non_exhaustive]
    TokenProviderConfig {},
}

impl ProfileFileError {
    fn missing_field(profile: &Profile, field: &'static str) -> Self {
        ProfileFileError::MissingProfile {
            profile: profile.name().to_string(),
            message: format!("`{field}` was missing").into(),
        }
    }
}

impl Error for ProfileFileError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ProfileFileError::InvalidProfile(err) => Some(err),
            _ => None,
        }
    }
}

impl Display for ProfileFileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProfileFileError::InvalidProfile(err) => {
                write!(f, "invalid profile: {err}")
            }
            ProfileFileError::CredentialLoop { profiles, next } => write!(
                f,
                "profile formed an infinite loop. first we loaded {profiles:?}, \
            then attempted to reload {next}",
            ),
            ProfileFileError::MissingCredentialSource { profile, message } => {
                write!(f, "missing credential source in `{profile}`: {message}")
            }
            ProfileFileError::InvalidCredentialSource { profile, message } => {
                write!(f, "invalid credential source in `{profile}`: {message}")
            }
            ProfileFileError::MissingProfile { profile, message } => {
                write!(f, "profile `{profile}` was not defined: {message}")
            }
            ProfileFileError::UnknownProvider { name } => write!(
                f,
                "profile referenced `{name}` provider but that provider is not supported",
            ),
            ProfileFileError::NoProfilesDefined => write!(f, "No profiles were defined"),
            ProfileFileError::ProfileDidNotContainCredentials { profile } => write!(
                f,
                "profile `{profile}` did not contain credential information"
            ),
            ProfileFileError::FeatureNotEnabled { feature, message } => {
                let message = message.as_deref().unwrap_or_default();
                write!(
                    f,
                    "This behavior requires following cargo feature(s) enabled: {feature}. {message}",
                )
            }
            ProfileFileError::MissingSsoSession {
                profile,
                sso_session,
            } => {
                write!(f, "sso-session named `{sso_session}` (referenced by profile `{profile}`) was not found")
            }
            ProfileFileError::InvalidSsoConfig { profile, message } => {
                write!(f, "profile `{profile}` has invalid SSO config: {message}")
            }
            ProfileFileError::TokenProviderConfig { .. } => {
                write!(
                    f,
                    "selected profile will resolve an access token instead of credentials \
                     since it doesn't have `sso_account_id` and `sso_role_name` set. Specify both \
                     `sso_account_id` and `sso_role_name` to let this profile resolve credentials."
                )
            }
        }
    }
}

/// Builder for [`ProfileFileCredentialsProvider`]
#[derive(Debug, Default)]
pub struct Builder {
    provider_config: Option<ProviderConfig>,
    profile_override: Option<String>,
    #[allow(deprecated)]
    profile_files: Option<ProfileFiles>,
    custom_providers: HashMap<Cow<'static, str>, Arc<dyn ProvideCredentials>>,
}

impl Builder {
    /// Override the configuration for the [`ProfileFileCredentialsProvider`]
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn test() {
    /// use aws_config::profile::ProfileFileCredentialsProvider;
    /// use aws_config::provider_config::ProviderConfig;
    /// let provider = ProfileFileCredentialsProvider::builder()
    ///     .configure(&ProviderConfig::with_default_region().await)
    ///     .build();
    /// # }
    /// ```
    pub fn configure(mut self, provider_config: &ProviderConfig) -> Self {
        self.provider_config = Some(provider_config.clone());
        self
    }

    /// Adds a custom credential source
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use aws_credential_types::provider::{self, future, ProvideCredentials};
    /// use aws_config::profile::ProfileFileCredentialsProvider;
    /// #[derive(Debug)]
    /// struct MyCustomProvider;
    /// impl MyCustomProvider {
    ///     async fn load_credentials(&self) -> provider::Result {
    ///         todo!()
    ///     }
    /// }
    ///
    /// impl ProvideCredentials for MyCustomProvider {
    ///   fn provide_credentials<'a>(&'a self) -> future::ProvideCredentials where Self: 'a {
    ///         future::ProvideCredentials::new(self.load_credentials())
    ///     }
    /// }
    ///
    /// # if cfg!(feature = "rustls") {
    /// let provider = ProfileFileCredentialsProvider::builder()
    ///     .with_custom_provider("Custom", MyCustomProvider)
    ///     .build();
    /// # }
    /// ```
    pub fn with_custom_provider(
        mut self,
        name: impl Into<Cow<'static, str>>,
        provider: impl ProvideCredentials + 'static,
    ) -> Self {
        self.custom_providers
            .insert(name.into(), Arc::new(provider));
        self
    }

    /// Override the profile name used by the [`ProfileFileCredentialsProvider`]
    pub fn profile_name(mut self, profile_name: impl Into<String>) -> Self {
        self.profile_override = Some(profile_name.into());
        self
    }

    /// Set the profile file that should be used by the [`ProfileFileCredentialsProvider`]
    #[allow(deprecated)]
    pub fn profile_files(mut self, profile_files: ProfileFiles) -> Self {
        self.profile_files = Some(profile_files);
        self
    }

    /// Builds a [`ProfileFileCredentialsProvider`]
    pub fn build(self) -> ProfileFileCredentialsProvider {
        let build_span = tracing::debug_span!("build_profile_file_credentials_provider");
        let _enter = build_span.enter();
        let conf = self
            .provider_config
            .unwrap_or_default()
            .with_profile_config(self.profile_files, self.profile_override);
        let mut named_providers = self.custom_providers.clone();
        named_providers
            .entry("Environment".into())
            .or_insert_with(|| {
                Arc::new(crate::environment::credentials::EnvironmentVariableCredentialsProvider::new_with_env(
                    conf.env(),
                ))
            });

        named_providers
            .entry("Ec2InstanceMetadata".into())
            .or_insert_with(|| {
                Arc::new(
                    crate::imds::credentials::ImdsCredentialsProvider::builder()
                        .configure(&conf)
                        .build(),
                )
            });

        named_providers
            .entry("EcsContainer".into())
            .or_insert_with(|| {
                Arc::new(
                    crate::ecs::EcsCredentialsProvider::builder()
                        .configure(&conf)
                        .build(),
                )
            });
        let factory = exec::named::NamedProviderFactory::new(named_providers);

        ProfileFileCredentialsProvider {
            config: Arc::new(Config {
                factory,
                provider_config: conf,
            }),
            inner_provider: ErrorTakingOnceCell::new(),
        }
    }
}

async fn build_provider_chain(
    config: Arc<Config>,
) -> Result<exec::ProviderChain, ProfileFileError> {
    let profile_set = config
        .provider_config
        .try_profile()
        .await
        .map_err(|parse_err| ProfileFileError::InvalidProfile(parse_err.clone()))?;
    let repr = repr::resolve_chain(profile_set)?;
    tracing::info!(chain = ?repr, "constructed abstract provider from config file");
    exec::ProviderChain::from_repr(&config.provider_config, repr, &config.factory)
}

#[derive(Debug)]
struct ChainProvider {
    config: Arc<Config>,
    chain: Option<Arc<exec::ProviderChain>>,
}

impl ChainProvider {
    async fn provide_credentials(&self) -> Result<Credentials, CredentialsError> {
        // Can't borrow `self` across an await point, or else we lose `Send` on the returned future
        let config = self.config.clone();
        let chain = self.chain.clone();

        if let Some(chain) = chain {
            let mut creds = match chain
                .base()
                .provide_credentials()
                .instrument(tracing::debug_span!("load_base_credentials"))
                .await
            {
                Ok(creds) => {
                    tracing::info!(creds = ?creds, "loaded base credentials");
                    creds
                }
                Err(e) => {
                    tracing::warn!(error = %DisplayErrorContext(&e), "failed to load base credentials");
                    return Err(CredentialsError::provider_error(e));
                }
            };

            // we want to create `SdkConfig` _after_ we have resolved the profile or else
            // we won't get things like `service_config()` set appropriately.
            let sdk_config = config.provider_config.client_config();
            for provider in chain.chain().iter() {
                let next_creds = provider
                    .credentials(creds, &sdk_config)
                    .instrument(tracing::debug_span!("load_assume_role", provider = ?provider))
                    .await;
                match next_creds {
                    Ok(next_creds) => {
                        tracing::info!(creds = ?next_creds, "loaded assume role credentials");
                        creds = next_creds
                    }
                    Err(e) => {
                        tracing::warn!(provider = ?provider, "failed to load assume role credentials");
                        return Err(CredentialsError::provider_error(e));
                    }
                }
            }
            Ok(creds)
        } else {
            Err(CredentialsError::not_loaded_no_source())
        }
    }
}

#[cfg(test)]
mod test {
    use crate::profile::credentials::Builder;
    use aws_credential_types::provider::ProvideCredentials;

    macro_rules! make_test {
        ($name: ident) => {
            #[tokio::test]
            async fn $name() {
                let _ = crate::test_case::TestEnvironment::from_dir(
                    concat!("./test-data/profile-provider/", stringify!($name)),
                    crate::test_case::test_credentials_provider(|config| async move {
                        Builder::default()
                            .configure(&config)
                            .build()
                            .provide_credentials()
                            .await
                    }),
                )
                .await
                .unwrap()
                .execute()
                .await;
            }
        };
    }

    make_test!(e2e_assume_role);
    make_test!(e2e_fips_and_dual_stack_sts);
    make_test!(empty_config);
    make_test!(retry_on_error);
    make_test!(invalid_config);
    make_test!(region_override);
    // TODO(https://github.com/awslabs/aws-sdk-rust/issues/1117) This test is disabled on Windows because it uses Unix-style paths
    #[cfg(all(feature = "credentials-process", not(windows)))]
    make_test!(credential_process);
    // TODO(https://github.com/awslabs/aws-sdk-rust/issues/1117) This test is disabled on Windows because it uses Unix-style paths
    #[cfg(all(feature = "credentials-process", not(windows)))]
    make_test!(credential_process_failure);
    // TODO(https://github.com/awslabs/aws-sdk-rust/issues/1117) This test is disabled on Windows because it uses Unix-style paths
    #[cfg(all(feature = "credentials-process", not(windows)))]
    make_test!(credential_process_account_id_fallback);
    #[cfg(feature = "credentials-process")]
    make_test!(credential_process_invalid);
    #[cfg(feature = "sso")]
    make_test!(sso_credentials);
    #[cfg(feature = "sso")]
    make_test!(invalid_sso_credentials_config);
    #[cfg(feature = "sso")]
    make_test!(sso_override_global_env_url);
    #[cfg(feature = "sso")]
    make_test!(sso_token);

    make_test!(assume_role_override_global_env_url);
    make_test!(assume_role_override_service_env_url);
    make_test!(assume_role_override_global_profile_url);
    make_test!(assume_role_override_service_profile_url);
}

#[cfg(all(test, feature = "sso"))]
mod sso_tests {
    use crate::{profile::credentials::Builder, provider_config::ProviderConfig};
    use aws_credential_types::credential_feature::AwsCredentialFeature;
    use aws_credential_types::provider::ProvideCredentials;
    use aws_sdk_sso::config::RuntimeComponents;
    use aws_smithy_runtime_api::client::{
        http::{
            HttpClient, HttpConnector, HttpConnectorFuture, HttpConnectorSettings,
            SharedHttpConnector,
        },
        orchestrator::{HttpRequest, HttpResponse},
    };
    use aws_smithy_types::body::SdkBody;
    use aws_types::os_shim_internal::{Env, Fs};
    use std::collections::HashMap;

    #[derive(Debug)]
    struct ClientInner {
        expected_token: &'static str,
    }
    impl HttpConnector for ClientInner {
        fn call(&self, request: HttpRequest) -> HttpConnectorFuture {
            assert_eq!(
                self.expected_token,
                request.headers().get("x-amz-sso_bearer_token").unwrap()
            );
            HttpConnectorFuture::ready(Ok(HttpResponse::new(
                    200.try_into().unwrap(),
                    SdkBody::from("{\"roleCredentials\":{\"accessKeyId\":\"ASIARTESTID\",\"secretAccessKey\":\"TESTSECRETKEY\",\"sessionToken\":\"TESTSESSIONTOKEN\",\"expiration\": 1651516560000}}"),
                )))
        }
    }
    #[derive(Debug)]
    struct Client {
        inner: SharedHttpConnector,
    }
    impl Client {
        fn new(expected_token: &'static str) -> Self {
            Self {
                inner: SharedHttpConnector::new(ClientInner { expected_token }),
            }
        }
    }
    impl HttpClient for Client {
        fn http_connector(
            &self,
            _settings: &HttpConnectorSettings,
            _components: &RuntimeComponents,
        ) -> SharedHttpConnector {
            self.inner.clone()
        }
    }

    fn create_test_fs() -> Fs {
        Fs::from_map({
            let mut map = HashMap::new();
            map.insert(
                "/home/.aws/config".to_string(),
                br#"
[profile default]
sso_session = dev
sso_account_id = 012345678901
sso_role_name = SampleRole
region = us-east-1

[sso-session dev]
sso_region = us-east-1
sso_start_url = https://d-abc123.awsapps.com/start
                "#
                .to_vec(),
            );
            map.insert(
                "/home/.aws/sso/cache/34c6fceca75e456f25e7e99531e2425c6c1de443.json".to_string(),
                br#"
                {
                    "accessToken": "secret-access-token",
                    "expiresAt": "2199-11-14T04:05:45Z",
                    "refreshToken": "secret-refresh-token",
                    "clientId": "ABCDEFG323242423121312312312312312",
                    "clientSecret": "ABCDE123",
                    "registrationExpiresAt": "2199-03-06T19:53:17Z",
                    "region": "us-east-1",
                    "startUrl": "https://d-abc123.awsapps.com/start"
                }
                "#
                .to_vec(),
            );
            map
        })
    }

    // TODO(https://github.com/awslabs/aws-sdk-rust/issues/1117) This test is ignored on Windows because it uses Unix-style paths
    #[cfg_attr(windows, ignore)]
    // In order to preserve the SSO token cache, the inner provider must only
    // be created once, rather than once per credential resolution.
    #[tokio::test]
    async fn create_inner_provider_exactly_once() {
        let fs = create_test_fs();

        let provider_config = ProviderConfig::empty()
            .with_fs(fs.clone())
            .with_env(Env::from_slice(&[("HOME", "/home")]))
            .with_http_client(Client::new("secret-access-token"));
        let provider = Builder::default().configure(&provider_config).build();

        let first_creds = provider.provide_credentials().await.unwrap();

        // Write to the token cache with an access token that won't match the fake client's
        // expected access token, and thus, won't return SSO credentials.
        fs.write(
            "/home/.aws/sso/cache/34c6fceca75e456f25e7e99531e2425c6c1de443.json",
            r#"
            {
                "accessToken": "NEW!!secret-access-token",
                "expiresAt": "2199-11-14T04:05:45Z",
                "refreshToken": "secret-refresh-token",
                "clientId": "ABCDEFG323242423121312312312312312",
                "clientSecret": "ABCDE123",
                "registrationExpiresAt": "2199-03-06T19:53:17Z",
                "region": "us-east-1",
                "startUrl": "https://d-abc123.awsapps.com/start"
            }
            "#,
        )
        .await
        .unwrap();

        // Loading credentials will still work since the SSOTokenProvider should have only
        // been created once, and thus, the correct token is still in an in-memory cache.
        let second_creds = provider
            .provide_credentials()
            .await
            .expect("used cached token instead of loading from the file system");
        assert_eq!(first_creds, second_creds);

        // Now create a new provider, which should use the new cached token value from the file system
        // since it won't have the in-memory cache. We do this just to verify that the FS mutation above
        // actually worked correctly.
        let provider_config = ProviderConfig::empty()
            .with_fs(fs.clone())
            .with_env(Env::from_slice(&[("HOME", "/home")]))
            .with_http_client(Client::new("NEW!!secret-access-token"));
        let provider = Builder::default().configure(&provider_config).build();
        let third_creds = provider.provide_credentials().await.unwrap();
        assert_eq!(second_creds, third_creds);
    }

    #[cfg_attr(windows, ignore)]
    #[tokio::test]
    async fn credential_feature() {
        let fs = create_test_fs();

        let provider_config = ProviderConfig::empty()
            .with_fs(fs.clone())
            .with_env(Env::from_slice(&[("HOME", "/home")]))
            .with_http_client(Client::new("secret-access-token"));
        let provider = Builder::default().configure(&provider_config).build();

        let creds = provider.provide_credentials().await.unwrap();

        assert_eq!(
            &vec![
                AwsCredentialFeature::CredentialsSso,
                AwsCredentialFeature::CredentialsProfile
            ],
            creds.get_property::<Vec<AwsCredentialFeature>>().unwrap()
        )
    }
}

#[cfg(all(test, feature = "credentials-login"))]
mod login_tests {
    use crate::provider_config::ProviderConfig;
    use aws_credential_types::provider::error::CredentialsError;
    use aws_credential_types::provider::ProvideCredentials;
    use aws_sdk_signin::config::RuntimeComponents;
    use aws_smithy_runtime_api::client::{
        http::{
            HttpClient, HttpConnector, HttpConnectorFuture, HttpConnectorSettings,
            SharedHttpConnector,
        },
        orchestrator::{HttpRequest, HttpResponse},
    };
    use aws_smithy_types::body::SdkBody;
    use aws_types::os_shim_internal::{Env, Fs};
    use std::collections::HashMap;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    #[derive(Debug, Clone)]
    struct TestClientInner {
        call_count: Arc<AtomicUsize>,
        response: Option<&'static str>,
    }

    impl HttpConnector for TestClientInner {
        fn call(&self, _request: HttpRequest) -> HttpConnectorFuture {
            self.call_count.fetch_add(1, Ordering::SeqCst);
            if let Some(response) = self.response {
                HttpConnectorFuture::ready(Ok(HttpResponse::new(
                    200.try_into().unwrap(),
                    SdkBody::from(response),
                )))
            } else {
                HttpConnectorFuture::ready(Ok(HttpResponse::new(
                    500.try_into().unwrap(),
                    SdkBody::from("{\"error\":\"server_error\"}"),
                )))
            }
        }
    }

    #[derive(Debug, Clone)]
    struct TestClient {
        inner: SharedHttpConnector,
        call_count: Arc<AtomicUsize>,
    }

    impl TestClient {
        fn new_success() -> Self {
            let call_count = Arc::new(AtomicUsize::new(0));
            let response = r#"{
                "accessToken": {
                    "accessKeyId": "ASIARTESTID",
                    "secretAccessKey": "TESTSECRETKEY",
                    "sessionToken": "TESTSESSIONTOKEN"
                },
                "expiresIn": 3600,
                "refreshToken": "new-refresh-token"
            }"#;
            let inner = TestClientInner {
                call_count: call_count.clone(),
                response: Some(response),
            };
            Self {
                inner: SharedHttpConnector::new(inner),
                call_count,
            }
        }

        fn new_error() -> Self {
            let call_count = Arc::new(AtomicUsize::new(0));
            let inner = TestClientInner {
                call_count: call_count.clone(),
                response: None,
            };
            Self {
                inner: SharedHttpConnector::new(inner),
                call_count,
            }
        }

        fn call_count(&self) -> usize {
            self.call_count.load(Ordering::SeqCst)
        }
    }

    impl HttpClient for TestClient {
        fn http_connector(
            &self,
            _settings: &HttpConnectorSettings,
            _components: &RuntimeComponents,
        ) -> SharedHttpConnector {
            self.inner.clone()
        }
    }

    fn create_test_fs_unexpired() -> Fs {
        Fs::from_map({
            let mut map = HashMap::new();
            map.insert(
                "/home/.aws/config".to_string(),
                br#"
[profile default]
login_session = arn:aws:iam::0123456789012:user/Admin
region = us-east-1
                "#
                .to_vec(),
            );
            map.insert(
                "/home/.aws/login/cache/36db1d138ff460920374e4c3d8e01f53f9f73537e89c88d639f68393df0e2726.json".to_string(),
                br#"{
                    "accessToken": {
                        "accessKeyId": "AKIAIOSFODNN7EXAMPLE",
                        "secretAccessKey": "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY",
                        "sessionToken": "session-token",
                        "accountId": "012345678901",
                        "expiresAt": "2199-12-25T21:30:00Z"
                    },
                    "tokenType": "aws_sigv4",
                    "refreshToken": "refresh-token-value",
                    "identityToken": "identity-token-value",
                    "clientId": "aws:signin:::cli/same-device",
                    "dpopKey": "-----BEGIN EC PRIVATE KEY-----\nMHcCAQEEIFDZHUzOG1Pzq+6F0mjMlOSp1syN9LRPBuHMoCFXTcXhoAoGCCqGSM49\nAwEHoUQDQgAE9qhj+KtcdHj1kVgwxWWWw++tqoh7H7UHs7oXh8jBbgF47rrYGC+t\ndjiIaHK3dBvvdE7MGj5HsepzLm3Kj91bqA==\n-----END EC PRIVATE KEY-----\n"
                }"#
                .to_vec(),
            );
            map
        })
    }

    fn create_test_fs_expired() -> Fs {
        Fs::from_map({
            let mut map = HashMap::new();
            map.insert(
                "/home/.aws/config".to_string(),
                br#"
[profile default]
login_session = arn:aws:iam::0123456789012:user/Admin
region = us-east-1
                "#
                .to_vec(),
            );
            map.insert(
                "/home/.aws/login/cache/36db1d138ff460920374e4c3d8e01f53f9f73537e89c88d639f68393df0e2726.json".to_string(),
                br#"{
                    "accessToken": {
                        "accessKeyId": "AKIAIOSFODNN7EXAMPLE",
                        "secretAccessKey": "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY",
                        "sessionToken": "session-token",
                        "accountId": "012345678901",
                        "expiresAt": "2020-01-01T00:00:00Z"
                    },
                    "tokenType": "aws_sigv4",
                    "refreshToken": "refresh-token-value",
                    "identityToken": "identity-token-value",
                    "clientId": "aws:signin:::cli/same-device",
                    "dpopKey": "-----BEGIN EC PRIVATE KEY-----\nMHcCAQEEIFDZHUzOG1Pzq+6F0mjMlOSp1syN9LRPBuHMoCFXTcXhoAoGCCqGSM49\nAwEHoUQDQgAE9qhj+KtcdHj1kVgwxWWWw++tqoh7H7UHs7oXh8jBbgF47rrYGC+t\ndjiIaHK3dBvvdE7MGj5HsepzLm3Kj91bqA==\n-----END EC PRIVATE KEY-----\n"
                }"#
                .to_vec(),
            );
            map
        })
    }

    #[cfg_attr(windows, ignore)]
    #[tokio::test]
    async fn unexpired_credentials_no_refresh() {
        let client = TestClient::new_success();

        let provider_config = ProviderConfig::empty()
            .with_fs(create_test_fs_unexpired())
            .with_env(Env::from_slice(&[("HOME", "/home")]))
            .with_http_client(client.clone())
            .with_region(Some(aws_types::region::Region::new("us-east-1")));

        let provider = crate::profile::credentials::Builder::default()
            .configure(&provider_config)
            .build();

        let creds = provider.provide_credentials().await.unwrap();
        assert_eq!("AKIAIOSFODNN7EXAMPLE", creds.access_key_id());
        assert_eq!(0, client.call_count());
    }

    #[cfg_attr(windows, ignore)]
    #[tokio::test]
    async fn expired_credentials_trigger_refresh() {
        let client = TestClient::new_success();

        let provider_config = ProviderConfig::empty()
            .with_fs(create_test_fs_expired())
            .with_env(Env::from_slice(&[("HOME", "/home")]))
            .with_http_client(client.clone())
            .with_region(Some(aws_types::region::Region::new("us-east-1")));

        let provider = crate::profile::credentials::Builder::default()
            .configure(&provider_config)
            .build();

        let creds = provider.provide_credentials().await.unwrap();
        assert_eq!("ASIARTESTID", creds.access_key_id());
        assert_eq!(1, client.call_count());
    }

    #[cfg_attr(windows, ignore)]
    #[tokio::test]
    async fn refresh_error_propagates() {
        let client = TestClient::new_error();

        let provider_config = ProviderConfig::empty()
            .with_fs(create_test_fs_expired())
            .with_env(Env::from_slice(&[("HOME", "/home")]))
            .with_http_client(client)
            .with_region(Some(aws_types::region::Region::new("us-east-1")));

        let provider = crate::profile::credentials::Builder::default()
            .configure(&provider_config)
            .build();

        let err = provider
            .provide_credentials()
            .await
            .expect_err("should fail on refresh error");

        match &err {
            CredentialsError::ProviderError(_) => {}
            _ => panic!("wrong error type"),
        }
    }
}
