/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::service_clock_skew::ServiceClockSkew;
use aws_smithy_async::time::TimeSource;
use aws_smithy_runtime_api::box_error::BoxError;
use aws_smithy_runtime_api::client::interceptors::context::BeforeTransmitInterceptorContextMut;
use aws_smithy_runtime_api::client::interceptors::Intercept;
use aws_smithy_runtime_api::client::retries::RequestAttempts;
use aws_smithy_runtime_api::client::runtime_components::RuntimeComponents;
use aws_smithy_types::config_bag::ConfigBag;
use aws_smithy_types::date_time::Format;
use aws_smithy_types::retry::RetryConfig;
use aws_smithy_types::timeout::TimeoutConfig;
use aws_smithy_types::DateTime;
use http_1x::{HeaderName, HeaderValue};
use std::borrow::Cow;
use std::time::Duration;

#[allow(clippy::declare_interior_mutable_const)] // we will never mutate this
const AMZ_SDK_REQUEST: HeaderName = HeaderName::from_static("amz-sdk-request");

/// Generates and attaches a request header that communicates request-related metadata.
/// Examples include:
///
/// - When the client will time out this request.
/// - How many times the request has been retried.
/// - The maximum number of retries that the client will attempt.
#[non_exhaustive]
#[derive(Debug, Default)]
pub struct RequestInfoInterceptor {}

impl RequestInfoInterceptor {
    /// Creates a new `RequestInfoInterceptor`
    pub fn new() -> Self {
        RequestInfoInterceptor {}
    }
}

impl RequestInfoInterceptor {
    fn build_attempts_pair(
        &self,
        cfg: &ConfigBag,
    ) -> Option<(Cow<'static, str>, Cow<'static, str>)> {
        let request_attempts = cfg
            .load::<RequestAttempts>()
            .map(|r_a| r_a.attempts())
            .unwrap_or(0);
        let request_attempts = request_attempts.to_string();
        Some((Cow::Borrowed("attempt"), Cow::Owned(request_attempts)))
    }

    fn build_max_attempts_pair(
        &self,
        cfg: &ConfigBag,
    ) -> Option<(Cow<'static, str>, Cow<'static, str>)> {
        if let Some(retry_config) = cfg.load::<RetryConfig>() {
            let max_attempts = retry_config.max_attempts().to_string();
            Some((Cow::Borrowed("max"), Cow::Owned(max_attempts)))
        } else {
            None
        }
    }

    fn build_ttl_pair(
        &self,
        cfg: &ConfigBag,
        timesource: impl TimeSource,
    ) -> Option<(Cow<'static, str>, Cow<'static, str>)> {
        let timeout_config = cfg.load::<TimeoutConfig>()?;
        let socket_read = timeout_config.read_timeout()?;
        let estimated_skew: Duration = cfg.load::<ServiceClockSkew>().cloned()?.into();
        let current_time = timesource.now();
        let ttl = current_time.checked_add(socket_read + estimated_skew)?;
        let mut timestamp = DateTime::from(ttl);
        // Set subsec_nanos to 0 so that the formatted `DateTime` won't have fractional seconds.
        timestamp.set_subsec_nanos(0);
        let mut formatted_timestamp = timestamp
            .fmt(Format::DateTime)
            .expect("the resulting DateTime will always be valid");

        // Remove dashes and colons
        formatted_timestamp = formatted_timestamp
            .chars()
            .filter(|&c| c != '-' && c != ':')
            .collect();

        Some((Cow::Borrowed("ttl"), Cow::Owned(formatted_timestamp)))
    }
}

impl Intercept for RequestInfoInterceptor {
    fn name(&self) -> &'static str {
        "RequestInfoInterceptor"
    }

    fn modify_before_transmit(
        &self,
        context: &mut BeforeTransmitInterceptorContextMut<'_>,
        runtime_components: &RuntimeComponents,
        cfg: &mut ConfigBag,
    ) -> Result<(), BoxError> {
        let mut pairs = RequestPairs::new();
        if let Some(pair) = self.build_ttl_pair(
            cfg,
            runtime_components
                .time_source()
                .ok_or("A timesource must be provided")?,
        ) {
            pairs = pairs.with_pair(pair);
        }
        if let Some(pair) = self.build_attempts_pair(cfg) {
            pairs = pairs.with_pair(pair);
        }
        if let Some(pair) = self.build_max_attempts_pair(cfg) {
            pairs = pairs.with_pair(pair);
        }

        let headers = context.request_mut().headers_mut();
        headers.insert(AMZ_SDK_REQUEST, pairs.try_into_header_value()?);

        Ok(())
    }
}

/// A builder for creating a `RequestPairs` header value. `RequestPairs` is used to generate a
/// retry information header that is sent with every request. The information conveyed by this
/// header allows services to anticipate whether a client will time out or retry a request.
#[derive(Default, Debug)]
struct RequestPairs {
    inner: Vec<(Cow<'static, str>, Cow<'static, str>)>,
}

impl RequestPairs {
    /// Creates a new `RequestPairs` builder.
    fn new() -> Self {
        Default::default()
    }

    /// Adds a pair to the `RequestPairs` builder.
    /// Only strings that can be converted to header values are considered valid.
    fn with_pair(
        mut self,
        pair: (impl Into<Cow<'static, str>>, impl Into<Cow<'static, str>>),
    ) -> Self {
        let pair = (pair.0.into(), pair.1.into());
        self.inner.push(pair);
        self
    }

    /// Converts the `RequestPairs` builder into a `HeaderValue`.
    fn try_into_header_value(self) -> Result<HeaderValue, BoxError> {
        self.try_into()
    }
}

impl TryFrom<RequestPairs> for HeaderValue {
    type Error = BoxError;

    fn try_from(value: RequestPairs) -> Result<Self, BoxError> {
        let mut pairs = String::new();
        for (key, value) in value.inner {
            if !pairs.is_empty() {
                pairs.push_str("; ");
            }

            pairs.push_str(&key);
            pairs.push('=');
            pairs.push_str(&value);
            continue;
        }
        HeaderValue::from_str(&pairs).map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::RequestInfoInterceptor;
    use crate::request_info::RequestPairs;
    use aws_smithy_runtime_api::client::interceptors::context::Input;
    use aws_smithy_runtime_api::client::interceptors::context::InterceptorContext;
    use aws_smithy_runtime_api::client::interceptors::Intercept;
    use aws_smithy_runtime_api::client::orchestrator::HttpRequest;
    use aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder;
    use aws_smithy_types::config_bag::{ConfigBag, Layer};
    use aws_smithy_types::retry::RetryConfig;
    use aws_smithy_types::timeout::TimeoutConfig;

    use http_1x::HeaderValue;
    use std::time::Duration;

    fn expect_header<'a>(context: &'a InterceptorContext, header_name: &str) -> &'a str {
        context
            .request()
            .expect("request is set")
            .headers()
            .get(header_name)
            .unwrap()
    }

    #[test]
    fn test_request_pairs_for_initial_attempt() {
        let rc = RuntimeComponentsBuilder::for_tests().build().unwrap();
        let mut context = InterceptorContext::new(Input::doesnt_matter());
        context.enter_serialization_phase();
        context.set_request(HttpRequest::empty());

        let mut layer = Layer::new("test");
        layer.store_put(RetryConfig::standard());
        layer.store_put(
            TimeoutConfig::builder()
                .read_timeout(Duration::from_secs(30))
                .build(),
        );
        let mut config = ConfigBag::of_layers(vec![layer]);

        let _ = context.take_input();
        context.enter_before_transmit_phase();
        let interceptor = RequestInfoInterceptor::new();
        let mut ctx = (&mut context).into();
        interceptor
            .modify_before_transmit(&mut ctx, &rc, &mut config)
            .unwrap();

        assert_eq!(
            expect_header(&context, "amz-sdk-request"),
            "attempt=0; max=3"
        );
    }

    #[test]
    fn test_header_value_from_request_pairs_supports_all_valid_characters() {
        // The list of valid characters is defined by an internal-only spec.
        let rp = RequestPairs::new()
            .with_pair(("allowed-symbols", "!#$&'*+-.^_`|~"))
            .with_pair(("allowed-digits", "01234567890"))
            .with_pair((
                "allowed-characters",
                "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ",
            ))
            .with_pair(("allowed-whitespace", " \t"));
        let _header_value: HeaderValue = rp
            .try_into()
            .expect("request pairs can be converted into valid header value.");
    }
}
