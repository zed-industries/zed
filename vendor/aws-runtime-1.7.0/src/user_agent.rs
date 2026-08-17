/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_smithy_types::config_bag::{Storable, StoreReplace};
use aws_types::app_name::AppName;
use aws_types::build_metadata::{OsFamily, BUILD_METADATA};
use aws_types::os_shim_internal::Env;
use std::borrow::Cow;
use std::error::Error;
use std::fmt;

mod interceptor;
mod metrics;
#[cfg(feature = "test-util")]
pub mod test_util;

const USER_AGENT_VERSION: &str = "2.1";

use crate::user_agent::metrics::BusinessMetrics;
pub use interceptor::UserAgentInterceptor;
pub use metrics::BusinessMetric;

/// AWS User Agent
///
/// Ths struct should be inserted into the [`ConfigBag`](aws_smithy_types::config_bag::ConfigBag)
/// during operation construction. The `UserAgentInterceptor` reads `AwsUserAgent`
/// from the config bag and sets the `User-Agent` and `x-amz-user-agent` headers.
#[derive(Clone, Debug)]
pub struct AwsUserAgent {
    sdk_metadata: SdkMetadata,
    ua_metadata: UaMetadata,
    api_metadata: ApiMetadata,
    os_metadata: OsMetadata,
    language_metadata: LanguageMetadata,
    exec_env_metadata: Option<ExecEnvMetadata>,
    business_metrics: BusinessMetrics,
    framework_metadata: Vec<FrameworkMetadata>,
    app_name: Option<AppName>,
    build_env_additional_metadata: Option<AdditionalMetadata>,
    additional_metadata: Vec<AdditionalMetadata>,
}

impl AwsUserAgent {
    /// Load a User Agent configuration from the environment
    ///
    /// This utilizes [`BUILD_METADATA`](const@aws_types::build_metadata::BUILD_METADATA) from `aws_types`
    /// to capture the Rust version & target platform. `ApiMetadata` provides
    /// the version & name of the specific service.
    pub fn new_from_environment(env: Env, api_metadata: ApiMetadata) -> Self {
        let build_metadata = &BUILD_METADATA;
        let sdk_metadata = SdkMetadata {
            name: "rust",
            version: build_metadata.core_pkg_version,
        };
        let ua_metadata = UaMetadata {
            version: USER_AGENT_VERSION,
        };
        let os_metadata = OsMetadata {
            os_family: &build_metadata.os_family,
            version: None,
        };
        let exec_env_metadata = env
            .get("AWS_EXECUTION_ENV")
            .ok()
            .map(|name| ExecEnvMetadata { name });

        // Retrieve additional metadata at compile-time from the AWS_SDK_RUST_BUILD_UA_METADATA env var
        let build_env_additional_metadata = option_env!("AWS_SDK_RUST_BUILD_UA_METADATA")
            .and_then(|value| AdditionalMetadata::new(value).ok());

        AwsUserAgent {
            sdk_metadata,
            ua_metadata,
            api_metadata,
            os_metadata,
            language_metadata: LanguageMetadata {
                lang: "rust",
                version: BUILD_METADATA.rust_version,
                extras: Default::default(),
            },
            exec_env_metadata,
            framework_metadata: Default::default(),
            business_metrics: Default::default(),
            app_name: Default::default(),
            build_env_additional_metadata,
            additional_metadata: Default::default(),
        }
    }

    /// For test purposes, construct an environment-independent User Agent
    ///
    /// Without this, running CI on a different platform would produce different user agent strings
    pub fn for_tests() -> Self {
        Self {
            sdk_metadata: SdkMetadata {
                name: "rust",
                version: "0.123.test",
            },
            ua_metadata: UaMetadata { version: "0.1" },
            api_metadata: ApiMetadata {
                service_id: "test-service".into(),
                version: "0.123",
            },
            os_metadata: OsMetadata {
                os_family: &OsFamily::Windows,
                version: Some("XPSP3".to_string()),
            },
            language_metadata: LanguageMetadata {
                lang: "rust",
                version: "1.50.0",
                extras: Default::default(),
            },
            exec_env_metadata: None,
            business_metrics: Default::default(),
            framework_metadata: Vec::new(),
            app_name: None,
            build_env_additional_metadata: None,
            additional_metadata: Vec::new(),
        }
    }

    #[deprecated(
        since = "1.4.0",
        note = "This is a no-op; use `with_business_metric` instead."
    )]
    #[allow(unused_mut)]
    #[allow(deprecated)]
    #[doc(hidden)]
    /// Adds feature metadata to the user agent.
    pub fn with_feature_metadata(mut self, _metadata: FeatureMetadata) -> Self {
        self
    }

    #[deprecated(
        since = "1.4.0",
        note = "This is a no-op; use `add_business_metric` instead."
    )]
    #[allow(deprecated)]
    #[allow(unused_mut)]
    #[doc(hidden)]
    /// Adds feature metadata to the user agent.
    pub fn add_feature_metadata(&mut self, _metadata: FeatureMetadata) -> &mut Self {
        self
    }

    #[deprecated(
        since = "1.4.0",
        note = "This is a no-op; use `with_business_metric` instead."
    )]
    #[allow(deprecated)]
    #[allow(unused_mut)]
    #[doc(hidden)]
    /// Adds config metadata to the user agent.
    pub fn with_config_metadata(mut self, _metadata: ConfigMetadata) -> Self {
        self
    }

    #[deprecated(
        since = "1.4.0",
        note = "This is a no-op; use `add_business_metric` instead."
    )]
    #[allow(deprecated)]
    #[allow(unused_mut)]
    #[doc(hidden)]
    /// Adds config metadata to the user agent.
    pub fn add_config_metadata(&mut self, _metadata: ConfigMetadata) -> &mut Self {
        self
    }

    #[doc(hidden)]
    /// Adds business metric to the user agent.
    pub fn with_business_metric(mut self, metric: BusinessMetric) -> Self {
        self.business_metrics.push(metric);
        self
    }

    #[doc(hidden)]
    /// Adds business metric to the user agent.
    pub fn add_business_metric(&mut self, metric: BusinessMetric) -> &mut Self {
        self.business_metrics.push(metric);
        self
    }

    #[doc(hidden)]
    /// Adds framework metadata to the user agent.
    pub fn with_framework_metadata(mut self, metadata: FrameworkMetadata) -> Self {
        self.framework_metadata.push(metadata);
        self
    }

    #[doc(hidden)]
    /// Adds framework metadata to the user agent.
    pub fn add_framework_metadata(&mut self, metadata: FrameworkMetadata) -> &mut Self {
        self.framework_metadata.push(metadata);
        self
    }

    /// Adds additional metadata to the user agent.
    pub fn with_additional_metadata(mut self, metadata: AdditionalMetadata) -> Self {
        self.additional_metadata.push(metadata);
        self
    }

    /// Adds additional metadata to the user agent.
    pub fn add_additional_metadata(&mut self, metadata: AdditionalMetadata) -> &mut Self {
        self.additional_metadata.push(metadata);
        self
    }

    /// Sets the app name for the user agent.
    pub fn with_app_name(mut self, app_name: AppName) -> Self {
        self.app_name = Some(app_name);
        self
    }

    /// Sets the app name for the user agent.
    pub fn set_app_name(&mut self, app_name: AppName) -> &mut Self {
        self.app_name = Some(app_name);
        self
    }

    /// Generate a new-style user agent style header
    ///
    /// This header should be set at `x-amz-user-agent`
    pub fn aws_ua_header(&self) -> String {
        /*
        ABNF for the user agent (see the bottom of the file for complete ABNF):
        ua-string = sdk-metadata RWS
                    ua-metadata RWS
                    [api-metadata RWS]
                    os-metadata RWS
                    language-metadata RWS
                    [env-metadata RWS]
                        ; ordering is not strictly required in the following section
                    [business-metrics]
                    [appId]
                    *(framework-metadata RWS)
        */
        let mut ua_value = String::new();
        use std::fmt::Write;
        // unwrap calls should never fail because string formatting will always succeed.
        write!(ua_value, "{} ", &self.sdk_metadata).unwrap();
        write!(ua_value, "{} ", &self.ua_metadata).unwrap();
        write!(ua_value, "{} ", &self.api_metadata).unwrap();
        write!(ua_value, "{} ", &self.os_metadata).unwrap();
        write!(ua_value, "{} ", &self.language_metadata).unwrap();
        if let Some(ref env_meta) = self.exec_env_metadata {
            write!(ua_value, "{env_meta} ").unwrap();
        }
        if !self.business_metrics.is_empty() {
            write!(ua_value, "{} ", &self.business_metrics).unwrap()
        }
        for framework in &self.framework_metadata {
            write!(ua_value, "{framework} ").unwrap();
        }
        for additional_metadata in &self.additional_metadata {
            write!(ua_value, "{additional_metadata} ").unwrap();
        }
        if let Some(app_name) = &self.app_name {
            write!(ua_value, "app/{app_name}").unwrap();
        }
        if let Some(additional_metadata) = &self.build_env_additional_metadata {
            write!(ua_value, "{additional_metadata}").unwrap();
        }
        if ua_value.ends_with(' ') {
            ua_value.truncate(ua_value.len() - 1);
        }
        ua_value
    }

    /// Generate an old-style User-Agent header for backward compatibility
    ///
    /// This header is intended to be set at `User-Agent`
    pub fn ua_header(&self) -> String {
        let mut ua_value = String::new();
        use std::fmt::Write;
        write!(ua_value, "{} ", &self.sdk_metadata).unwrap();
        write!(ua_value, "{} ", &self.os_metadata).unwrap();
        write!(ua_value, "{}", &self.language_metadata).unwrap();
        ua_value
    }
}

impl Storable for AwsUserAgent {
    type Storer = StoreReplace<Self>;
}

#[derive(Clone, Copy, Debug)]
struct SdkMetadata {
    name: &'static str,
    version: &'static str,
}

impl fmt::Display for SdkMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "aws-sdk-{}/{}", self.name, self.version)
    }
}

#[derive(Clone, Copy, Debug)]
struct UaMetadata {
    version: &'static str,
}

impl fmt::Display for UaMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ua/{}", self.version)
    }
}

/// Metadata about the client that's making the call.
#[derive(Clone, Debug)]
pub struct ApiMetadata {
    service_id: Cow<'static, str>,
    version: &'static str,
}

impl ApiMetadata {
    /// Creates new `ApiMetadata`.
    pub const fn new(service_id: &'static str, version: &'static str) -> Self {
        Self {
            service_id: Cow::Borrowed(service_id),
            version,
        }
    }
}

impl fmt::Display for ApiMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "api/{}/{}", self.service_id, self.version)
    }
}

impl Storable for ApiMetadata {
    type Storer = StoreReplace<Self>;
}

/// Error for when an user agent metadata doesn't meet character requirements.
///
/// Metadata may only have alphanumeric characters and any of these characters:
/// ```text
/// !#$%&'*+-.^_`|~
/// ```
/// Spaces are not allowed.
#[derive(Debug)]
#[non_exhaustive]
pub struct InvalidMetadataValue;

impl Error for InvalidMetadataValue {}

impl fmt::Display for InvalidMetadataValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "User agent metadata can only have alphanumeric characters, or any of \
             '!' |  '#' |  '$' |  '%' |  '&' |  '\\'' |  '*' |  '+' |  '-' | \
             '.' |  '^' |  '_' |  '`' |  '|' |  '~'"
        )
    }
}

fn validate_metadata(value: Cow<'static, str>) -> Result<Cow<'static, str>, InvalidMetadataValue> {
    fn valid_character(c: char) -> bool {
        match c {
            _ if c.is_ascii_alphanumeric() => true,
            '!' | '#' | '$' | '%' | '&' | '\'' | '*' | '+' | '-' | '.' | '^' | '_' | '`' | '|'
            | '~' => true,
            _ => false,
        }
    }
    if !value.chars().all(valid_character) {
        return Err(InvalidMetadataValue);
    }
    Ok(value)
}

#[doc(hidden)]
/// Additional metadata that can be bundled with framework or feature metadata.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct AdditionalMetadata {
    value: Cow<'static, str>,
}

impl AdditionalMetadata {
    /// Creates `AdditionalMetadata`.
    ///
    /// This will result in `InvalidMetadataValue` if the given value isn't alphanumeric or
    /// has characters other than the following:
    /// ```text
    /// !#$%&'*+-.^_`|~
    /// ```
    pub fn new(value: impl Into<Cow<'static, str>>) -> Result<Self, InvalidMetadataValue> {
        Ok(Self {
            value: validate_metadata(value.into())?,
        })
    }
}

impl fmt::Display for AdditionalMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // additional-metadata = "md/" ua-pair
        write!(f, "md/{}", self.value)
    }
}

#[derive(Clone, Debug, Default)]
struct AdditionalMetadataList(Vec<AdditionalMetadata>);

impl AdditionalMetadataList {
    fn push(&mut self, metadata: AdditionalMetadata) {
        self.0.push(metadata);
    }
}

impl fmt::Display for AdditionalMetadataList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for metadata in &self.0 {
            write!(f, " {metadata}")?;
        }
        Ok(())
    }
}

#[deprecated(since = "1.4.0", note = "Replaced by `BusinessMetric`.")]
#[doc(hidden)]
/// Metadata about a feature that is being used in the SDK.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct FeatureMetadata {
    name: Cow<'static, str>,
    version: Option<Cow<'static, str>>,
    additional: AdditionalMetadataList,
}

#[allow(deprecated)]
impl FeatureMetadata {
    /// Creates `FeatureMetadata`.
    ///
    /// This will result in `InvalidMetadataValue` if the given value isn't alphanumeric or
    /// has characters other than the following:
    /// ```text
    /// !#$%&'*+-.^_`|~
    /// ```
    pub fn new(
        name: impl Into<Cow<'static, str>>,
        version: Option<Cow<'static, str>>,
    ) -> Result<Self, InvalidMetadataValue> {
        Ok(Self {
            name: validate_metadata(name.into())?,
            version: version.map(validate_metadata).transpose()?,
            additional: Default::default(),
        })
    }

    /// Bundles additional arbitrary metadata with this feature metadata.
    pub fn with_additional(mut self, metadata: AdditionalMetadata) -> Self {
        self.additional.push(metadata);
        self
    }
}

#[allow(deprecated)]
impl fmt::Display for FeatureMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // feat-metadata = "ft/" name ["/" version] *(RWS additional-metadata)
        if let Some(version) = &self.version {
            write!(f, "ft/{}/{}{}", self.name, version, self.additional)
        } else {
            write!(f, "ft/{}{}", self.name, self.additional)
        }
    }
}

#[deprecated(since = "1.4.0", note = "Replaced by `BusinessMetric`.")]
#[doc(hidden)]
/// Metadata about a config value that is being used in the SDK.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct ConfigMetadata {
    config: Cow<'static, str>,
    value: Option<Cow<'static, str>>,
}

#[allow(deprecated)]
impl ConfigMetadata {
    /// Creates `ConfigMetadata`.
    ///
    /// This will result in `InvalidMetadataValue` if the given value isn't alphanumeric or
    /// has characters other than the following:
    /// ```text
    /// !#$%&'*+-.^_`|~
    /// ```
    pub fn new(
        config: impl Into<Cow<'static, str>>,
        value: Option<Cow<'static, str>>,
    ) -> Result<Self, InvalidMetadataValue> {
        Ok(Self {
            config: validate_metadata(config.into())?,
            value: value.map(validate_metadata).transpose()?,
        })
    }
}

#[allow(deprecated)]
impl fmt::Display for ConfigMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // config-metadata = "cfg/" config ["/" value]
        if let Some(value) = &self.value {
            write!(f, "cfg/{}/{}", self.config, value)
        } else {
            write!(f, "cfg/{}", self.config)
        }
    }
}

#[doc(hidden)]
/// Metadata about a software framework that is being used with the SDK.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct FrameworkMetadata {
    name: Cow<'static, str>,
    version: Option<Cow<'static, str>>,
    additional: AdditionalMetadataList,
}

impl FrameworkMetadata {
    /// Creates `FrameworkMetadata`.
    ///
    /// This will result in `InvalidMetadataValue` if the given value isn't alphanumeric or
    /// has characters other than the following:
    /// ```text
    /// !#$%&'*+-.^_`|~
    /// ```
    pub fn new(
        name: impl Into<Cow<'static, str>>,
        version: Option<Cow<'static, str>>,
    ) -> Result<Self, InvalidMetadataValue> {
        Ok(Self {
            name: validate_metadata(name.into())?,
            version: version.map(validate_metadata).transpose()?,
            additional: Default::default(),
        })
    }

    /// Bundles additional arbitrary metadata with this framework metadata.
    pub fn with_additional(mut self, metadata: AdditionalMetadata) -> Self {
        self.additional.push(metadata);
        self
    }
}

impl fmt::Display for FrameworkMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // framework-metadata = "lib/" name ["/" version] *(RWS additional-metadata)
        if let Some(version) = &self.version {
            write!(f, "lib/{}/{}{}", self.name, version, self.additional)
        } else {
            write!(f, "lib/{}{}", self.name, self.additional)
        }
    }
}

#[derive(Clone, Debug)]
struct OsMetadata {
    os_family: &'static OsFamily,
    version: Option<String>,
}

impl fmt::Display for OsMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let os_family = match self.os_family {
            OsFamily::Windows => "windows",
            OsFamily::Linux => "linux",
            OsFamily::Macos => "macos",
            OsFamily::Android => "android",
            OsFamily::Ios => "ios",
            OsFamily::Other => "other",
        };
        write!(f, "os/{os_family}")?;
        if let Some(ref version) = self.version {
            write!(f, "/{version}")?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct LanguageMetadata {
    lang: &'static str,
    version: &'static str,
    extras: AdditionalMetadataList,
}
impl fmt::Display for LanguageMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // language-metadata = "lang/" language "/" version *(RWS additional-metadata)
        write!(f, "lang/{}/{}{}", self.lang, self.version, self.extras)
    }
}

#[derive(Clone, Debug)]
struct ExecEnvMetadata {
    name: String,
}
impl fmt::Display for ExecEnvMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "exec-env/{}", &self.name)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use aws_types::app_name::AppName;
    use aws_types::build_metadata::OsFamily;
    use aws_types::os_shim_internal::Env;
    use std::borrow::Cow;

    fn make_deterministic(ua: &mut AwsUserAgent) {
        // hard code some variable things for a deterministic test
        ua.sdk_metadata.version = "0.1";
        ua.ua_metadata.version = "0.1";
        ua.language_metadata.version = "1.50.0";
        ua.os_metadata.os_family = &OsFamily::Macos;
        ua.os_metadata.version = Some("1.15".to_string());
    }

    #[test]
    fn generate_a_valid_ua() {
        let api_metadata = ApiMetadata {
            service_id: "dynamodb".into(),
            version: "123",
        };
        let mut ua = AwsUserAgent::new_from_environment(Env::from_slice(&[]), api_metadata);
        make_deterministic(&mut ua);
        assert_eq!(
            ua.aws_ua_header(),
            "aws-sdk-rust/0.1 ua/0.1 api/dynamodb/123 os/macos/1.15 lang/rust/1.50.0"
        );
        assert_eq!(
            ua.ua_header(),
            "aws-sdk-rust/0.1 os/macos/1.15 lang/rust/1.50.0"
        );
    }

    #[test]
    fn generate_a_valid_ua_with_execution_env() {
        let api_metadata = ApiMetadata {
            service_id: "dynamodb".into(),
            version: "123",
        };
        let mut ua = AwsUserAgent::new_from_environment(
            Env::from_slice(&[("AWS_EXECUTION_ENV", "lambda")]),
            api_metadata,
        );
        make_deterministic(&mut ua);
        assert_eq!(
            ua.aws_ua_header(),
            "aws-sdk-rust/0.1 ua/0.1 api/dynamodb/123 os/macos/1.15 lang/rust/1.50.0 exec-env/lambda"
        );
        assert_eq!(
            ua.ua_header(),
            "aws-sdk-rust/0.1 os/macos/1.15 lang/rust/1.50.0"
        );
    }

    #[test]
    fn generate_a_valid_ua_with_frameworks() {
        let api_metadata = ApiMetadata {
            service_id: "dynamodb".into(),
            version: "123",
        };
        let mut ua = AwsUserAgent::new_from_environment(Env::from_slice(&[]), api_metadata)
            .with_framework_metadata(
                FrameworkMetadata::new("some-framework", Some(Cow::Borrowed("1.3")))
                    .unwrap()
                    .with_additional(AdditionalMetadata::new("something").unwrap()),
            )
            .with_framework_metadata(FrameworkMetadata::new("other", None).unwrap());
        make_deterministic(&mut ua);
        assert_eq!(
            ua.aws_ua_header(),
            "aws-sdk-rust/0.1 ua/0.1 api/dynamodb/123 os/macos/1.15 lang/rust/1.50.0 lib/some-framework/1.3 md/something lib/other"
        );
        assert_eq!(
            ua.ua_header(),
            "aws-sdk-rust/0.1 os/macos/1.15 lang/rust/1.50.0"
        );
    }

    #[test]
    fn generate_a_valid_ua_with_app_name() {
        let api_metadata = ApiMetadata {
            service_id: "dynamodb".into(),
            version: "123",
        };
        let mut ua = AwsUserAgent::new_from_environment(Env::from_slice(&[]), api_metadata)
            .with_app_name(AppName::new("my_app").unwrap());
        make_deterministic(&mut ua);
        assert_eq!(
            ua.aws_ua_header(),
            "aws-sdk-rust/0.1 ua/0.1 api/dynamodb/123 os/macos/1.15 lang/rust/1.50.0 app/my_app"
        );
        assert_eq!(
            ua.ua_header(),
            "aws-sdk-rust/0.1 os/macos/1.15 lang/rust/1.50.0"
        );
    }

    #[test]
    fn generate_a_valid_ua_with_build_env_additional_metadata() {
        let mut ua = AwsUserAgent::for_tests();
        ua.build_env_additional_metadata = Some(AdditionalMetadata::new("asdf").unwrap());
        assert_eq!(
            ua.aws_ua_header(),
            "aws-sdk-rust/0.123.test ua/0.1 api/test-service/0.123 os/windows/XPSP3 lang/rust/1.50.0 md/asdf"
        );
        assert_eq!(
            ua.ua_header(),
            "aws-sdk-rust/0.123.test os/windows/XPSP3 lang/rust/1.50.0"
        );
    }

    #[test]
    fn generate_a_valid_ua_with_business_metrics() {
        // single metric ID
        {
            let ua = AwsUserAgent::for_tests().with_business_metric(BusinessMetric::ResourceModel);
            assert_eq!(
                ua.aws_ua_header(),
                "aws-sdk-rust/0.123.test ua/0.1 api/test-service/0.123 os/windows/XPSP3 lang/rust/1.50.0 m/A"
            );
            assert_eq!(
                ua.ua_header(),
                "aws-sdk-rust/0.123.test os/windows/XPSP3 lang/rust/1.50.0"
            );
        }
        // multiple metric IDs
        {
            let ua = AwsUserAgent::for_tests()
                .with_business_metric(BusinessMetric::RetryModeAdaptive)
                .with_business_metric(BusinessMetric::S3Transfer)
                .with_business_metric(BusinessMetric::S3ExpressBucket);
            assert_eq!(
                ua.aws_ua_header(),
                "aws-sdk-rust/0.123.test ua/0.1 api/test-service/0.123 os/windows/XPSP3 lang/rust/1.50.0 m/F,G,J"
            );
            assert_eq!(
                ua.ua_header(),
                "aws-sdk-rust/0.123.test os/windows/XPSP3 lang/rust/1.50.0"
            );
        }
    }
}

/*
Appendix: User Agent ABNF
sdk-ua-header                 = "x-amz-user-agent:" OWS ua-string OWS
ua-pair                       = ua-name ["/" ua-value]
ua-name                       = token
ua-value                      = token
version                       = token
name                          = token
service-id                    = token
sdk-name                      = java / ruby / php / dotnet / python / cli / kotlin / rust / js / cpp / go / go-v2
os-family                     = windows / linux / macos / android / ios / other
config                        = retry-mode
additional-metadata           = "md/" ua-pair
sdk-metadata                  = "aws-sdk-" sdk-name "/" version
api-metadata                  = "api/" service-id "/" version
os-metadata                   = "os/" os-family ["/" version]
language-metadata             = "lang/" language "/" version *(RWS additional-metadata)
env-metadata                  = "exec-env/" name
framework-metadata            = "lib/" name ["/" version] *(RWS additional-metadata)
app-id                        = "app/" name
build-env-additional-metadata = "md/" value
ua-metadata                   = "ua/2.1"
business-metrics              = "m/" metric_id *(comma metric_id)
metric_id                     = 1*m_char
m_char                        = DIGIT / ALPHA / "+" / "-"
comma                         = ","
ua-string                     = sdk-metadata RWS
                                ua-metadata RWS
                                [api-metadata RWS]
                                os-metadata RWS
                                language-metadata RWS
                                [env-metadata RWS]
                                       ; ordering is not strictly required in the following section
                                [business-metrics]
                                [app-id]
                                [build-env-additional-metadata]
                                *(framework-metadata RWS)

# New metadata field might be added in the future and they must follow this format
prefix               = token
metadata             = prefix "/" ua-pair

# token, RWS and OWS are defined in [RFC 7230](https://tools.ietf.org/html/rfc7230)
OWS            = *( SP / HTAB )
               ; optional whitespace
RWS            = 1*( SP / HTAB )
               ; required whitespace
token          = 1*tchar
tchar          = "!" / "#" / "$" / "%" / "&" / "'" / "*" / "+" / "-" / "." /
                 "^" / "_" / "`" / "|" / "~" / DIGIT / ALPHA
*/
