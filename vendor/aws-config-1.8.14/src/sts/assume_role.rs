/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Assume credentials for a role through the AWS Security Token Service (STS).

use aws_credential_types::credential_feature::AwsCredentialFeature;
use aws_credential_types::provider::{
    self, error::CredentialsError, future, ProvideCredentials, SharedCredentialsProvider,
};
use aws_sdk_sts::operation::assume_role::builders::AssumeRoleFluentBuilder;
use aws_sdk_sts::operation::assume_role::AssumeRoleError;
use aws_sdk_sts::types::{PolicyDescriptorType, Tag};
use aws_sdk_sts::Client as StsClient;
use aws_smithy_runtime::client::identity::IdentityCache;
use aws_smithy_runtime_api::client::result::SdkError;
use aws_smithy_types::error::display::DisplayErrorContext;
use aws_types::region::Region;
use aws_types::SdkConfig;
use std::time::Duration;
use tracing::Instrument;

/// Credentials provider that uses credentials provided by another provider to assume a role
/// through the AWS Security Token Service (STS).
///
/// When asked to provide credentials, this provider will first invoke the inner credentials
/// provider to get AWS credentials for STS. Then, it will call STS to get assumed credentials for
/// the desired role.
///
/// # Examples
/// Create an AssumeRoleProvider explicitly set to us-east-2 that utilizes the default credentials chain.
/// ```no_run
/// use aws_config::sts::AssumeRoleProvider;
/// use aws_types::region::Region;
/// # async fn docs() {
/// let provider = AssumeRoleProvider::builder("arn:aws:iam::123456789012:role/demo")
///   .region(Region::from_static("us-east-2"))
///   .session_name("testAR")
///   .build().await;
/// }
/// ```
///
/// Create an AssumeRoleProvider from an explicitly configured base configuration.
/// ```no_run
/// use aws_config::sts::AssumeRoleProvider;
/// use aws_types::region::Region;
/// # async fn docs() {
/// let conf = aws_config::from_env().use_fips(true).load().await;
/// let provider = AssumeRoleProvider::builder("arn:aws:iam::123456789012:role/demo")
///   .configure(&conf)
///   .session_name("testAR")
///   .build().await;
/// }
/// ```
///
/// Create an AssumeroleProvider that sources credentials from a provider credential provider:
/// ```no_run
/// use aws_config::sts::AssumeRoleProvider;
/// use aws_types::region::Region;
/// use aws_config::environment::EnvironmentVariableCredentialsProvider;
/// # async fn docs() {
/// let provider = AssumeRoleProvider::builder("arn:aws:iam::123456789012:role/demo")
///   .session_name("test-assume-role-session")
///   // only consider environment variables, explicitly.
///   .build_from_provider(EnvironmentVariableCredentialsProvider::new()).await;
/// }
/// ```
///
#[derive(Debug)]
pub struct AssumeRoleProvider {
    inner: Inner,
}

#[derive(Debug)]
struct Inner {
    fluent_builder: AssumeRoleFluentBuilder,
}

impl AssumeRoleProvider {
    /// Build a new role-assuming provider for the given role.
    ///
    /// The `role` argument should take the form an Amazon Resource Name (ARN) like
    ///
    /// ```text
    /// arn:aws:iam::123456789012:role/example
    /// ```
    pub fn builder(role: impl Into<String>) -> AssumeRoleProviderBuilder {
        AssumeRoleProviderBuilder::new(role.into())
    }
}

/// A builder for [`AssumeRoleProvider`].
///
/// Construct one through [`AssumeRoleProvider::builder`].
#[derive(Debug)]
pub struct AssumeRoleProviderBuilder {
    role_arn: String,
    external_id: Option<String>,
    session_name: Option<String>,
    session_length: Option<Duration>,
    policy: Option<String>,
    policy_arns: Option<Vec<PolicyDescriptorType>>,
    region_override: Option<Region>,
    sdk_config: Option<SdkConfig>,
    tags: Option<Vec<Tag>>,
}

impl AssumeRoleProviderBuilder {
    /// Start a new assume role builder for the given role.
    ///
    /// The `role` argument should take the form an Amazon Resource Name (ARN) like
    ///
    /// ```text
    /// arn:aws:iam::123456789012:role/example
    /// ```
    pub fn new(role: impl Into<String>) -> Self {
        Self {
            role_arn: role.into(),
            external_id: None,
            session_name: None,
            session_length: None,
            policy: None,
            policy_arns: None,
            sdk_config: None,
            region_override: None,
            tags: None,
        }
    }

    /// Set a unique identifier that might be required when you assume a role in another account.
    ///
    /// If the administrator of the account to which the role belongs provided you with an external
    /// ID, then provide that value in this parameter. The value can be any string, such as a
    /// passphrase or account number.
    pub fn external_id(mut self, id: impl Into<String>) -> Self {
        self.external_id = Some(id.into());
        self
    }

    /// Set an identifier for the assumed role session.
    ///
    /// Use the role session name to uniquely identify a session when the same role is assumed by
    /// different principals or for different reasons. In cross-account scenarios, the role session
    /// name is visible to, and can be logged by the account that owns the role. The role session
    /// name is also used in the ARN of the assumed role principal.
    pub fn session_name(mut self, name: impl Into<String>) -> Self {
        self.session_name = Some(name.into());
        self
    }

    /// Set an IAM policy in JSON format that you want to use as an inline session policy.
    ///
    /// This parameter is optional
    /// For more information, see
    /// [policy](aws_sdk_sts::operation::assume_role::builders::AssumeRoleInputBuilder::policy_arns)
    pub fn policy(mut self, policy: impl Into<String>) -> Self {
        self.policy = Some(policy.into());
        self
    }

    /// Set the Amazon Resource Names (ARNs) of the IAM managed policies that you want to use as managed session policies.
    ///
    /// This parameter is optional.
    /// For more information, see
    /// [policy_arns](aws_sdk_sts::operation::assume_role::builders::AssumeRoleInputBuilder::policy_arns)
    pub fn policy_arns(mut self, policy_arns: Vec<String>) -> Self {
        self.policy_arns = Some(
            policy_arns
                .into_iter()
                .map(|arn| PolicyDescriptorType::builder().arn(arn).build())
                .collect::<Vec<_>>(),
        );
        self
    }

    /// Set the expiration time of the role session.
    ///
    /// When unset, this value defaults to 1 hour.
    ///
    /// The value specified can range from 900 seconds (15 minutes) up to the maximum session duration
    /// set for the role. The maximum session duration setting can have a value from 1 hour to 12 hours.
    /// If you specify a value higher than this setting or the administrator setting (whichever is lower),
    /// **you will be unable to assume the role**. For example, if you specify a session duration of 12 hours,
    /// but your administrator set the maximum session duration to 6 hours, you cannot assume the role.
    ///
    /// For more information, see
    /// [duration_seconds](aws_sdk_sts::operation::assume_role::builders::AssumeRoleInputBuilder::duration_seconds)
    pub fn session_length(mut self, length: Duration) -> Self {
        self.session_length = Some(length);
        self
    }

    /// Set the region to assume the role in.
    ///
    /// This dictates which STS endpoint the AssumeRole action is invoked on. This will override
    /// a region set from `.configure(...)`
    pub fn region(mut self, region: Region) -> Self {
        self.region_override = Some(region);
        self
    }

    /// Set the session tags
    ///
    /// A list of session tags that you want to pass. Each session tag consists of a key name and an associated value.
    /// For more information, see `[Tag]`.
    pub fn tags<K, V>(mut self, tags: impl IntoIterator<Item = (K, V)>) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.tags = Some(
            tags.into_iter()
                // Unwrap won't fail as both key and value are specified.
                // Currently Tag does not have an infallible build method.
                .map(|(k, v)| {
                    Tag::builder()
                        .key(k)
                        .value(v)
                        .build()
                        .expect("this is unreachable: both k and v are set")
                })
                .collect::<Vec<_>>(),
        );
        self
    }

    /// Sets the configuration used for this provider
    ///
    /// This enables overriding the connection used to communicate with STS in addition to other internal
    /// fields like the time source and sleep implementation used for caching.
    ///
    /// If this field is not provided, configuration from [`crate::load_from_env()`] is used.
    ///
    /// # Examples
    /// ```rust
    /// # async fn docs() {
    /// use aws_types::region::Region;
    /// use aws_config::sts::AssumeRoleProvider;
    /// let config = aws_config::from_env().region(Region::from_static("us-west-2")).load().await;
    /// let assume_role_provider = AssumeRoleProvider::builder("arn:aws:iam::123456789012:role/example")
    ///   .configure(&config)
    ///   .build();
    /// }
    pub fn configure(mut self, conf: &SdkConfig) -> Self {
        self.sdk_config = Some(conf.clone());
        self
    }

    /// Build a credentials provider for this role.
    ///
    /// Base credentials will be used from the [`SdkConfig`] set via [`Self::configure`] or loaded
    /// from [`aws_config::from_env`](crate::from_env) if `configure` was never called.
    pub async fn build(self) -> AssumeRoleProvider {
        let mut conf = match self.sdk_config {
            Some(conf) => conf,
            None => crate::load_defaults(crate::BehaviorVersion::latest()).await,
        };
        // ignore a identity cache set from SdkConfig
        conf = conf
            .into_builder()
            .identity_cache(IdentityCache::no_cache())
            .build();

        // set a region override if one exists
        if let Some(region) = self.region_override {
            conf = conf.into_builder().region(region).build()
        }

        let config = aws_sdk_sts::config::Builder::from(&conf);

        let time_source = conf.time_source().expect("A time source must be provided.");

        let session_name = self.session_name.unwrap_or_else(|| {
            super::util::default_session_name("assume-role-provider", time_source.now())
        });

        let sts_client = StsClient::from_conf(config.build());
        let fluent_builder = sts_client
            .assume_role()
            .set_role_arn(Some(self.role_arn))
            .set_external_id(self.external_id)
            .set_role_session_name(Some(session_name))
            .set_policy(self.policy)
            .set_policy_arns(self.policy_arns)
            .set_duration_seconds(self.session_length.map(|dur| dur.as_secs() as i32))
            .set_tags(self.tags);

        AssumeRoleProvider {
            inner: Inner { fluent_builder },
        }
    }

    /// Build a credentials provider for this role authorized by the given `provider`.
    pub async fn build_from_provider(
        mut self,
        provider: impl ProvideCredentials + 'static,
    ) -> AssumeRoleProvider {
        let conf = match self.sdk_config {
            Some(conf) => conf,
            None => crate::load_defaults(crate::BehaviorVersion::latest()).await,
        };
        let conf = conf
            .into_builder()
            .credentials_provider(SharedCredentialsProvider::new(provider))
            .build();
        self.sdk_config = Some(conf);
        self.build().await
    }
}

impl Inner {
    async fn credentials(&self) -> provider::Result {
        tracing::debug!("retrieving assumed credentials");

        let assumed = self.fluent_builder.clone().send().in_current_span().await;
        let assumed = match assumed {
            Ok(assumed) => {
                tracing::debug!(
                    access_key_id = ?assumed.credentials.as_ref().map(|c| &c.access_key_id),
                    "obtained assumed credentials"
                );
                super::util::into_credentials(
                    assumed.credentials,
                    assumed.assumed_role_user,
                    "AssumeRoleProvider",
                )
            }
            Err(SdkError::ServiceError(ref context))
                if matches!(
                    context.err(),
                    AssumeRoleError::RegionDisabledException(_)
                        | AssumeRoleError::MalformedPolicyDocumentException(_)
                ) =>
            {
                Err(CredentialsError::invalid_configuration(
                    assumed.err().unwrap(),
                ))
            }
            Err(SdkError::ServiceError(ref context)) => {
                tracing::warn!(error = %DisplayErrorContext(context.err()), "STS refused to grant assume role");
                Err(CredentialsError::provider_error(assumed.err().unwrap()))
            }
            Err(err) => Err(CredentialsError::provider_error(err)),
        };

        assumed.map(|mut creds| {
            creds
                .get_property_mut_or_default::<Vec<AwsCredentialFeature>>()
                .push(AwsCredentialFeature::CredentialsStsAssumeRole);
            creds
        })
    }
}

impl ProvideCredentials for AssumeRoleProvider {
    fn provide_credentials<'a>(&'a self) -> future::ProvideCredentials<'a>
    where
        Self: 'a,
    {
        future::ProvideCredentials::new(
            self.inner
                .credentials()
                .instrument(tracing::debug_span!("assume_role")),
        )
    }
}

#[cfg(test)]
mod test {
    use crate::sts::AssumeRoleProvider;
    use aws_credential_types::credential_feature::AwsCredentialFeature;
    use aws_credential_types::credential_fn::provide_credentials_fn;
    use aws_credential_types::provider::{ProvideCredentials, SharedCredentialsProvider};
    use aws_credential_types::Credentials;
    use aws_smithy_async::rt::sleep::{SharedAsyncSleep, TokioSleep};
    use aws_smithy_async::test_util::instant_time_and_sleep;
    use aws_smithy_async::time::StaticTimeSource;
    use aws_smithy_http_client::test_util::{capture_request, ReplayEvent, StaticReplayClient};
    use aws_smithy_runtime::test_util::capture_test_logs::capture_test_logs;
    use aws_smithy_runtime_api::client::behavior_version::BehaviorVersion;
    use aws_smithy_types::body::SdkBody;
    use aws_types::os_shim_internal::Env;
    use aws_types::region::Region;
    use aws_types::SdkConfig;
    use http::header::AUTHORIZATION;
    use std::time::{Duration, UNIX_EPOCH};

    #[tokio::test]
    async fn configures_session_length() {
        let (http_client, request) = capture_request(None);
        let sdk_config = SdkConfig::builder()
            .sleep_impl(SharedAsyncSleep::new(TokioSleep::new()))
            .time_source(StaticTimeSource::new(
                UNIX_EPOCH + Duration::from_secs(1234567890 - 120),
            ))
            .http_client(http_client)
            .region(Region::from_static("this-will-be-overridden"))
            .behavior_version(crate::BehaviorVersion::latest())
            .build();
        let provider = AssumeRoleProvider::builder("myrole")
            .configure(&sdk_config)
            .region(Region::new("us-east-1"))
            .session_length(Duration::from_secs(1234567))
            .build_from_provider(provide_credentials_fn(|| async {
                Ok(Credentials::for_tests())
            }))
            .await;
        let _ = dbg!(provider.provide_credentials().await);
        let req = request.expect_request();
        let str_body = std::str::from_utf8(req.body().bytes().unwrap()).unwrap();
        assert!(str_body.contains("1234567"), "{}", str_body);
        assert_eq!(req.uri(), "https://sts.us-east-1.amazonaws.com/");
    }

    #[tokio::test]
    async fn loads_region_from_sdk_config() {
        let (http_client, request) = capture_request(None);
        let sdk_config = SdkConfig::builder()
            .behavior_version(crate::BehaviorVersion::latest())
            .sleep_impl(SharedAsyncSleep::new(TokioSleep::new()))
            .time_source(StaticTimeSource::new(
                UNIX_EPOCH + Duration::from_secs(1234567890 - 120),
            ))
            .http_client(http_client)
            .credentials_provider(SharedCredentialsProvider::new(provide_credentials_fn(
                || async {
                    panic!("don't call me — will be overridden");
                },
            )))
            .region(Region::from_static("us-west-2"))
            .build();
        let provider = AssumeRoleProvider::builder("myrole")
            .configure(&sdk_config)
            .session_length(Duration::from_secs(1234567))
            .build_from_provider(provide_credentials_fn(|| async {
                Ok(Credentials::for_tests())
            }))
            .await;
        let _ = dbg!(provider.provide_credentials().await);
        let req = request.expect_request();
        assert_eq!(req.uri(), "https://sts.us-west-2.amazonaws.com/");
    }

    /// Test that `build()` where no provider is passed still works
    #[tokio::test]
    async fn build_method_from_sdk_config() {
        let _guard = capture_test_logs();
        let (http_client, request) = capture_request(Some(
            http::Response::builder()
                .status(404)
                .body(SdkBody::from(""))
                .unwrap(),
        ));
        let conf = crate::defaults(BehaviorVersion::latest())
            .env(Env::from_slice(&[
                ("AWS_ACCESS_KEY_ID", "123-key"),
                ("AWS_SECRET_ACCESS_KEY", "456"),
                ("AWS_REGION", "us-west-17"),
            ]))
            .use_dual_stack(true)
            .use_fips(true)
            .time_source(StaticTimeSource::from_secs(1234567890))
            .http_client(http_client)
            .load()
            .await;
        let provider = AssumeRoleProvider::builder("role")
            .configure(&conf)
            .build()
            .await;
        let _ = dbg!(provider.provide_credentials().await);
        let req = request.expect_request();
        let auth_header = req.headers().get(AUTHORIZATION).unwrap().to_string();
        let expect = "Credential=123-key/20090213/us-west-17/sts/aws4_request";
        assert!(
            auth_header.contains(expect),
            "Expected header to contain {expect} but it was {auth_header}"
        );
        // ensure that FIPS & DualStack are also respected
        assert_eq!("https://sts-fips.us-west-17.api.aws/", req.uri())
    }

    fn create_test_http_client() -> StaticReplayClient {
        StaticReplayClient::new(vec![
            ReplayEvent::new(http::Request::new(SdkBody::from("request body")),
            http::Response::builder().status(200).body(SdkBody::from(
                "<AssumeRoleResponse xmlns=\"https://sts.amazonaws.com/doc/2011-06-15/\">\n  <AssumeRoleResult>\n    <AssumedRoleUser>\n      <AssumedRoleId>AROAR42TAWARILN3MNKUT:assume-role-from-profile-1632246085998</AssumedRoleId>\n      <Arn>arn:aws:sts::130633740322:assumed-role/assume-provider-test/assume-role-from-profile-1632246085998</Arn>\n    </AssumedRoleUser>\n    <Credentials>\n      <AccessKeyId>ASIARCORRECT</AccessKeyId>\n      <SecretAccessKey>secretkeycorrect</SecretAccessKey>\n      <SessionToken>tokencorrect</SessionToken>\n      <Expiration>2009-02-13T23:31:30Z</Expiration>\n    </Credentials>\n  </AssumeRoleResult>\n  <ResponseMetadata>\n    <RequestId>d9d47248-fd55-4686-ad7c-0fb7cd1cddd7</RequestId>\n  </ResponseMetadata>\n</AssumeRoleResponse>\n"
            )).unwrap()),
            ReplayEvent::new(http::Request::new(SdkBody::from("request body")),
            http::Response::builder().status(200).body(SdkBody::from(
                "<AssumeRoleResponse xmlns=\"https://sts.amazonaws.com/doc/2011-06-15/\">\n  <AssumeRoleResult>\n    <AssumedRoleUser>\n      <AssumedRoleId>AROAR42TAWARILN3MNKUT:assume-role-from-profile-1632246085998</AssumedRoleId>\n      <Arn>arn:aws:sts::130633740322:assumed-role/assume-provider-test/assume-role-from-profile-1632246085998</Arn>\n    </AssumedRoleUser>\n    <Credentials>\n      <AccessKeyId>ASIARCORRECT</AccessKeyId>\n      <SecretAccessKey>TESTSECRET</SecretAccessKey>\n      <SessionToken>tokencorrect</SessionToken>\n      <Expiration>2009-02-13T23:33:30Z</Expiration>\n    </Credentials>\n  </AssumeRoleResult>\n  <ResponseMetadata>\n    <RequestId>c2e971c2-702d-4124-9b1f-1670febbea18</RequestId>\n  </ResponseMetadata>\n</AssumeRoleResponse>\n"
            )).unwrap()),
        ])
    }

    #[tokio::test]
    async fn provider_does_not_cache_credentials_by_default() {
        let http_client = create_test_http_client();

        let (testing_time_source, sleep) = instant_time_and_sleep(
            UNIX_EPOCH + Duration::from_secs(1234567890 - 120), // 1234567890 since UNIX_EPOCH is 2009-02-13T23:31:30Z
        );

        let sdk_config = SdkConfig::builder()
            .sleep_impl(SharedAsyncSleep::new(sleep))
            .time_source(testing_time_source.clone())
            .http_client(http_client)
            .behavior_version(crate::BehaviorVersion::latest())
            .build();
        let credentials_list = std::sync::Arc::new(std::sync::Mutex::new(vec![
            Credentials::new(
                "test",
                "test",
                None,
                Some(UNIX_EPOCH + Duration::from_secs(1234567890 + 1)),
                "test",
            ),
            Credentials::new(
                "test",
                "test",
                None,
                Some(UNIX_EPOCH + Duration::from_secs(1234567890 + 120)),
                "test",
            ),
        ]));
        let credentials_list_cloned = credentials_list.clone();
        let provider = AssumeRoleProvider::builder("myrole")
            .configure(&sdk_config)
            .region(Region::new("us-east-1"))
            .build_from_provider(provide_credentials_fn(move || {
                let list = credentials_list.clone();
                async move {
                    let next = list.lock().unwrap().remove(0);
                    Ok(next)
                }
            }))
            .await;

        let creds_first = provider
            .provide_credentials()
            .await
            .expect("should return valid credentials");

        // After time has been advanced by 120 seconds, the first credentials _could_ still be valid
        // if `LazyCredentialsCache` were used, but the provider uses `NoCredentialsCache` by default
        // so the first credentials will not be used.
        testing_time_source.advance(Duration::from_secs(120));

        let creds_second = provider
            .provide_credentials()
            .await
            .expect("should return the second credentials");
        assert_ne!(creds_first, creds_second);
        assert!(credentials_list_cloned.lock().unwrap().is_empty());
    }

    #[tokio::test]
    async fn credentials_feature() {
        let http_client = create_test_http_client();

        let (testing_time_source, sleep) = instant_time_and_sleep(
            UNIX_EPOCH + Duration::from_secs(1234567890), // 1234567890 since UNIX_EPOCH is 2009-02-13T23:31:30Z
        );

        let sdk_config = SdkConfig::builder()
            .sleep_impl(SharedAsyncSleep::new(sleep))
            .time_source(testing_time_source.clone())
            .http_client(http_client)
            .behavior_version(crate::BehaviorVersion::latest())
            .build();
        let credentials = Credentials::new(
            "test",
            "test",
            None,
            Some(UNIX_EPOCH + Duration::from_secs(1234567890 + 1)),
            "test",
        );
        let provider = AssumeRoleProvider::builder("myrole")
            .configure(&sdk_config)
            .region(Region::new("us-east-1"))
            .build_from_provider(credentials)
            .await;

        let creds = provider
            .provide_credentials()
            .await
            .expect("should return valid credentials");

        assert_eq!(
            &vec![AwsCredentialFeature::CredentialsStsAssumeRole],
            creds.get_property::<Vec<AwsCredentialFeature>>().unwrap()
        )
    }
}
