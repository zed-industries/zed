/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::default_provider::use_dual_stack::use_dual_stack_provider;
use crate::default_provider::use_fips::use_fips_provider;
use crate::provider_config::ProviderConfig;
use aws_smithy_async::rt::sleep::{AsyncSleep, Sleep, TokioSleep};
use aws_smithy_http_client::test_util::dvr::{NetworkTraffic, RecordingClient, ReplayingClient};
use aws_smithy_runtime::test_util::capture_test_logs::capture_test_logs;
use aws_smithy_runtime_api::box_error::BoxError;
use aws_smithy_runtime_api::shared::IntoShared;
use aws_smithy_types::{date_time::Format, error::display::DisplayErrorContext, DateTime};
use aws_types::os_shim_internal::{Env, Fs};
use aws_types::sdk_config::SharedHttpClient;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Debug;
use std::future::Future;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::time::{Duration, UNIX_EPOCH};

mod sealed {
    /// Trait that provides secret values for a given test output type (credentials or tokens)
    pub(crate) trait Secrets {
        fn secrets(&self) -> Vec<String>;
    }
    impl Secrets for () {
        fn secrets(&self) -> Vec<String> {
            Vec::new()
        }
    }
}
use sealed::Secrets;

/// Test case credentials
///
/// Credentials for use in test cases. These implement Serialize/Deserialize and have a
/// non-hidden debug implementation.
#[derive(Deserialize, Debug, Eq, PartialEq)]
pub(crate) struct Credentials {
    pub(crate) access_key_id: String,
    pub(crate) secret_access_key: String,
    pub(crate) session_token: Option<String>,
    pub(crate) account_id: Option<String>,
    pub(crate) expiry: Option<u64>,
}

impl Secrets for Credentials {
    fn secrets(&self) -> Vec<String> {
        vec![self.secret_access_key.clone()]
    }
}

/// Convert real credentials to test credentials
///
/// Comparing equality on real credentials works, but it's a pain because the Debug implementation
/// hides the actual keys
impl From<&aws_credential_types::Credentials> for Credentials {
    fn from(credentials: &aws_credential_types::Credentials) -> Self {
        Self {
            access_key_id: credentials.access_key_id().into(),
            secret_access_key: credentials.secret_access_key().into(),
            session_token: credentials.session_token().map(ToString::to_string),
            account_id: credentials.account_id().map(|id| id.as_str().to_string()),
            expiry: credentials
                .expiry()
                .map(|t| t.duration_since(UNIX_EPOCH).unwrap().as_secs()),
        }
    }
}

impl From<aws_credential_types::Credentials> for Credentials {
    fn from(credentials: aws_credential_types::Credentials) -> Self {
        (&credentials).into()
    }
}

/// Test case token
///
/// Token for use in test cases. These implement Serialize/Deserialize and have a
/// non-hidden debug implementation.
#[derive(Deserialize, Debug, Eq, PartialEq)]
#[allow(dead_code)]
pub(crate) struct Token {
    pub(crate) token: String,
    pub(crate) expiration: Option<String>,
}

impl Secrets for Token {
    fn secrets(&self) -> Vec<String> {
        vec![self.token.clone()]
    }
}

impl From<&aws_credential_types::Token> for Token {
    fn from(value: &aws_credential_types::Token) -> Self {
        Self {
            token: value.token().into(),
            expiration: value
                .expiration()
                .map(|s| DateTime::from(s).fmt(Format::DateTime).unwrap()),
        }
    }
}

impl From<aws_credential_types::Token> for Token {
    fn from(value: aws_credential_types::Token) -> Self {
        Self::from(&value)
    }
}

/// Connector which expects no traffic
pub(crate) fn no_traffic_client() -> SharedHttpClient {
    ReplayingClient::new(Vec::new()).into_shared()
}

#[derive(Debug)]
pub(crate) struct InstantSleep;
impl AsyncSleep for InstantSleep {
    fn sleep(&self, _duration: Duration) -> Sleep {
        Sleep::new(std::future::ready(()))
    }
}

#[derive(Deserialize, Debug)]
pub(crate) enum GenericTestResult<T> {
    Ok(T),
    ErrorContains(String),
}

impl<O> GenericTestResult<O>
where
    O: PartialEq + Debug,
{
    #[track_caller]
    pub(crate) fn assert_matches<E>(&self, result: Result<&O, &E>)
    where
        E: Error,
    {
        match (result, &self) {
            (Ok(actual), GenericTestResult::Ok(expected)) => {
                assert_eq!(expected, actual, "incorrect result was returned")
            }
            (Err(err), GenericTestResult::ErrorContains(substr)) => {
                let message = format!("{}", DisplayErrorContext(err));
                assert!(
                    message.contains(substr),
                    "`{message}` did not contain `{substr}`"
                );
            }
            (Err(actual_error), GenericTestResult::Ok(expected_creds)) => {
                panic!(
                    "expected output ({:?}) but an error was returned: {}",
                    expected_creds,
                    DisplayErrorContext(actual_error)
                )
            }
            (Ok(output), GenericTestResult::ErrorContains(substr)) => panic!(
                "expected an error containing: `{}`, but a result was returned: {:?}",
                substr, output
            ),
        }
    }
}

#[derive(Deserialize)]
pub(crate) struct Metadata<T> {
    result: GenericTestResult<T>,
    docs: String,
    name: String,
}

pub(crate) trait RunTestProvider {
    type Output: for<'a> Deserialize<'a> + Secrets;
    type Error;

    fn run_provider(
        &self,
        provider_config: ProviderConfig,
    ) -> Pin<Box<dyn Future<Output = Result<Self::Output, Self::Error>> + Send + 'static>>;
}

type ResultFuture<O, E> = Pin<Box<dyn Future<Output = Result<O, E>> + Send + 'static>>;
pub(crate) struct StaticTestProvider<O, E> {
    run_provider_fn: Box<dyn Fn(ProviderConfig) -> ResultFuture<O, E> + 'static>,
}
impl<O, E> StaticTestProvider<O, E> {
    pub(crate) fn new<F>(run_provider_fn: F) -> Self
    where
        F: Fn(ProviderConfig) -> ResultFuture<O, E> + 'static,
    {
        Self {
            run_provider_fn: Box::new(run_provider_fn) as _,
        }
    }
}
impl<O, E> RunTestProvider for StaticTestProvider<O, E>
where
    O: for<'a> Deserialize<'a> + Secrets,
{
    type Output = O;
    type Error = E;

    fn run_provider(
        &self,
        provider_config: ProviderConfig,
    ) -> Pin<Box<dyn Future<Output = Result<Self::Output, Self::Error>> + Send + 'static>> {
        (self.run_provider_fn)(provider_config)
    }
}

pub(crate) fn test_credentials_provider<F, Fut, E>(
    run_provider_fn: F,
) -> impl RunTestProvider<Output = Credentials, Error = E>
where
    F: Fn(ProviderConfig) -> Fut + Send + Clone + 'static,
    Fut: Future<Output = Result<aws_credential_types::Credentials, E>> + Send,
{
    StaticTestProvider::<Credentials, E>::new(move |config| {
        let run_provider_fn = run_provider_fn.clone();
        Box::pin(async move { (run_provider_fn)(config).await.map(Into::into) })
    })
}

#[cfg(feature = "sso")]
pub(crate) fn test_token_provider<F, Fut, E>(
    run_provider_fn: F,
) -> impl RunTestProvider<Output = Token, Error = E>
where
    F: Fn(ProviderConfig) -> Fut + Send + Clone + 'static,
    Fut: Future<Output = Result<aws_credential_types::Token, E>> + Send,
{
    StaticTestProvider::<Token, E>::new(move |config| {
        let run_provider_fn = run_provider_fn.clone();
        Box::pin(async move { (run_provider_fn)(config).await.map(Into::into) })
    })
}

/// Provider test environment
///
/// A provider test environment is a directory containing:
/// - an `fs` directory. This is loaded into the test as if it was mounted at `/`
/// - an `env.json` file containing environment variables
/// - an  `http-traffic.json` file containing an http traffic log from [`dvr`](aws_smithy_runtime::client::http::test_utils::dvr)
/// - a `test-case.json` file defining the expected output of the test
pub(crate) struct TestEnvironment<O, E> {
    metadata: Metadata<O>,
    base_dir: PathBuf,
    http_client: ReplayingClient,
    provider_config: ProviderConfig,
    run_provider: Box<dyn RunTestProvider<Output = O, Error = E>>,
}

impl<O, E> TestEnvironment<O, E>
where
    O: for<'a> Deserialize<'a>,
{
    pub(crate) async fn from_dir(
        dir: impl AsRef<Path>,
        run_provider: impl RunTestProvider<Output = O, Error = E> + 'static,
    ) -> Result<Self, BoxError> {
        let dir = dir.as_ref();
        let env = std::fs::read_to_string(dir.join("env.json"))
            .map_err(|e| format!("failed to load env: {}", e))?;
        let env: HashMap<String, String> =
            serde_json::from_str(&env).map_err(|e| format!("failed to parse env: {}", e))?;
        let env = Env::from(env);
        let fs = Fs::from_test_dir(dir.join("fs"), "/");
        let network_traffic = std::fs::read_to_string(dir.join("http-traffic.json"))
            .map_err(|e| format!("failed to load http traffic: {}", e))?;
        let mut network_traffic: NetworkTraffic = serde_json::from_str(&network_traffic)?;
        network_traffic.correct_content_lengths();

        let metadata: Metadata<O> = serde_json::from_str(
            &std::fs::read_to_string(dir.join("test-case.json"))
                .map_err(|e| format!("failed to load test case: {}", e))?,
        )?;
        let http_client = ReplayingClient::new(network_traffic.events().clone());
        let provider_config = ProviderConfig::empty()
            .with_fs(fs.clone())
            .with_env(env.clone())
            .with_http_client(http_client.clone())
            .with_sleep_impl(TokioSleep::new())
            .load_default_region()
            .await;

        let use_dual_stack = use_dual_stack_provider(&provider_config).await;
        let use_fips = use_fips_provider(&provider_config).await;
        let provider_config = provider_config
            .with_use_fips(use_fips)
            .with_use_dual_stack(use_dual_stack);

        Ok(Self {
            base_dir: dir.into(),
            metadata,
            http_client,
            provider_config,
            run_provider: Box::new(run_provider),
        })
    }

    pub(crate) fn map_provider_config<F>(mut self, provider_config_builder: F) -> Self
    where
        F: Fn(ProviderConfig) -> ProviderConfig,
    {
        self.provider_config = provider_config_builder(self.provider_config.clone());
        self
    }

    pub(crate) fn provider_config(&self) -> &ProviderConfig {
        &self.provider_config
    }
}

impl<O, E> TestEnvironment<O, E>
where
    O: for<'a> Deserialize<'a> + Secrets + PartialEq + Debug,
    E: Error,
{
    #[cfg(feature = "default-https-client")]
    #[allow(unused)]
    /// Record a test case from live (remote) HTTPS traffic
    ///
    /// The `default_connector()` from the crate will be used
    pub(crate) async fn execute_from_live_traffic(&self) {
        // swap out the connector generated from `http-traffic.json` for a real connector:

        use std::error::Error;
        let live_connector = aws_smithy_http_client::default_connector(
            &Default::default(),
            self.provider_config.sleep_impl(),
        )
        .expect("feature gate on this function makes this always return Some");

        let live_client = RecordingClient::new(live_connector);
        let config = self
            .provider_config
            .clone()
            .with_http_client(live_client.clone());
        let result = self.run_provider.run_provider(config).await;
        std::fs::write(
            self.base_dir.join("http-traffic-recorded.json"),
            serde_json::to_string(&live_client.network_traffic()).unwrap(),
        )
        .unwrap();
        self.check_results(result.as_ref());
    }

    #[allow(dead_code)]
    /// Execute the test suite & record a new traffic log
    ///
    /// A connector will be created with the factory, then request traffic will be recorded.
    /// Response are generated from the existing http-traffic.json.
    pub(crate) async fn execute_and_update(&self) {
        let recording_client = RecordingClient::new(self.http_client.clone());
        let config = self
            .provider_config
            .clone()
            .with_http_client(recording_client.clone());
        let result = self.run_provider.run_provider(config).await;
        std::fs::write(
            self.base_dir.join("http-traffic-recorded.json"),
            serde_json::to_string(&recording_client.network_traffic()).unwrap(),
        )
        .unwrap();
        self.check_results(result.as_ref());
    }

    /// Execute a test case. Failures lead to panics.
    pub(crate) async fn execute(&self) -> Result<O, E> {
        let (_guard, rx) = capture_test_logs();
        let result = self
            .run_provider
            .run_provider(self.provider_config.clone())
            .await;
        tokio::time::pause();
        self.log_info();
        self.check_results(result.as_ref());
        // todo: validate bodies
        match self
            .http_client
            .clone()
            .validate(
                &["CONTENT-TYPE", "x-aws-ec2-metadata-token"],
                |_expected, _actual| Ok(()),
            )
            .await
        {
            Ok(()) => {}
            Err(e) => panic!("{}", DisplayErrorContext(e.as_ref())),
        }
        if self
            .provider_config
            .env()
            .get("EXPOSE_SECRETS_FOR_TESTS")
            .is_err()
        {
            let contents = rx.contents();
            let leaking_lines = self.lines_with_secrets(&contents);
            assert!(
                leaking_lines.is_empty(),
                "secret was exposed\n{:?}\nSee the following log lines:\n  {}",
                self.metadata.result,
                leaking_lines.join("\n  ")
            );
        }
        result
    }

    fn log_info(&self) {
        eprintln!("test case: {}. {}", self.metadata.name, self.metadata.docs);
    }

    fn lines_with_secrets<'a>(&'a self, logs: &'a str) -> Vec<&'a str> {
        logs.lines()
            .filter(|l| self.contains_any_secrets(l))
            .collect()
    }

    fn contains_any_secrets(&self, log_line: &str) -> bool {
        assert!(log_line.lines().count() <= 1);
        match &self.metadata.result {
            // NOTE: we aren't currently erroring if the session token is leaked, that is in the canonical request among other things
            GenericTestResult::Ok(output) => output
                .secrets()
                .iter()
                .any(|secret| log_line.contains(secret)),
            GenericTestResult::ErrorContains(_) => false,
        }
    }

    #[track_caller]
    fn check_results(&self, result: Result<&O, &E>)
    where
        O: PartialEq + Debug,
        E: Error,
    {
        self.metadata.result.assert_matches(result);
    }
}
