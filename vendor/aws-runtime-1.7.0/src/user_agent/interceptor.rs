/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use std::borrow::Cow;
use std::fmt;

use http_1x::header::{HeaderName, HeaderValue, InvalidHeaderValue, USER_AGENT};

use aws_credential_types::credential_feature::AwsCredentialFeature;
use aws_smithy_runtime::client::sdk_feature::SmithySdkFeature;
use aws_smithy_runtime_api::box_error::BoxError;
use aws_smithy_runtime_api::client::http::HttpClient;
use aws_smithy_runtime_api::client::interceptors::context::{
    BeforeTransmitInterceptorContextMut, BeforeTransmitInterceptorContextRef,
};
use aws_smithy_runtime_api::client::interceptors::Intercept;
use aws_smithy_runtime_api::client::runtime_components::RuntimeComponents;
use aws_smithy_types::config_bag::ConfigBag;
use aws_types::app_name::AppName;
use aws_types::os_shim_internal::Env;

use crate::sdk_feature::AwsSdkFeature;
use crate::user_agent::metrics::ProvideBusinessMetric;
use crate::user_agent::{AdditionalMetadata, ApiMetadata, AwsUserAgent, InvalidMetadataValue};

macro_rules! add_metrics_unique {
    ($features:expr, $ua:expr, $added:expr) => {
        for feature in $features {
            if let Some(m) = feature.provide_business_metric() {
                if !$added.contains(&m) {
                    $added.insert(m.clone());
                    $ua.add_business_metric(m);
                }
            }
        }
    };
}

macro_rules! add_metrics_unique_reverse {
    ($features:expr, $ua:expr, $added:expr) => {
        let mut unique_metrics = Vec::new();
        for feature in $features {
            if let Some(m) = feature.provide_business_metric() {
                if !$added.contains(&m) {
                    $added.insert(m.clone());
                    unique_metrics.push(m);
                }
            }
        }
        for m in unique_metrics.into_iter().rev() {
            $ua.add_business_metric(m);
        }
    };
}

#[allow(clippy::declare_interior_mutable_const)] // we will never mutate this
const X_AMZ_USER_AGENT: HeaderName = HeaderName::from_static("x-amz-user-agent");

#[derive(Debug)]
enum UserAgentInterceptorError {
    MissingApiMetadata,
    InvalidHeaderValue(InvalidHeaderValue),
    InvalidMetadataValue(InvalidMetadataValue),
}

impl std::error::Error for UserAgentInterceptorError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidHeaderValue(source) => Some(source),
            Self::InvalidMetadataValue(source) => Some(source),
            Self::MissingApiMetadata => None,
        }
    }
}

impl fmt::Display for UserAgentInterceptorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::InvalidHeaderValue(_) => "AwsUserAgent generated an invalid HTTP header value. This is a bug. Please file an issue.",
            Self::InvalidMetadataValue(_) => "AwsUserAgent generated an invalid metadata value. This is a bug. Please file an issue.",
            Self::MissingApiMetadata => "The UserAgentInterceptor requires ApiMetadata to be set before the request is made. This is a bug. Please file an issue.",
        })
    }
}

impl From<InvalidHeaderValue> for UserAgentInterceptorError {
    fn from(err: InvalidHeaderValue) -> Self {
        UserAgentInterceptorError::InvalidHeaderValue(err)
    }
}

impl From<InvalidMetadataValue> for UserAgentInterceptorError {
    fn from(err: InvalidMetadataValue) -> Self {
        UserAgentInterceptorError::InvalidMetadataValue(err)
    }
}

/// Generates and attaches the AWS SDK's user agent to a HTTP request
#[non_exhaustive]
#[derive(Debug, Default)]
pub struct UserAgentInterceptor;

impl UserAgentInterceptor {
    /// Creates a new `UserAgentInterceptor`
    pub fn new() -> Self {
        UserAgentInterceptor
    }
}

fn header_values(
    ua: &AwsUserAgent,
) -> Result<(HeaderValue, HeaderValue), UserAgentInterceptorError> {
    // Pay attention to the extremely subtle difference between ua_header and aws_ua_header below...
    Ok((
        HeaderValue::try_from(ua.ua_header())?,
        HeaderValue::try_from(ua.aws_ua_header())?,
    ))
}

impl Intercept for UserAgentInterceptor {
    fn name(&self) -> &'static str {
        "UserAgentInterceptor"
    }

    fn read_after_serialization(
        &self,
        _context: &BeforeTransmitInterceptorContextRef<'_>,
        _runtime_components: &RuntimeComponents,
        cfg: &mut ConfigBag,
    ) -> Result<(), BoxError> {
        // Allow for overriding the user agent by an earlier interceptor (so, for example,
        // tests can use `AwsUserAgent::for_tests()`) by attempting to grab one out of the
        // config bag before creating one.
        if cfg.load::<AwsUserAgent>().is_some() {
            return Ok(());
        }

        let api_metadata = cfg
            .load::<ApiMetadata>()
            .ok_or(UserAgentInterceptorError::MissingApiMetadata)?;
        let mut ua = AwsUserAgent::new_from_environment(Env::real(), api_metadata.clone());

        let maybe_app_name = cfg.load::<AppName>();
        if let Some(app_name) = maybe_app_name {
            ua.set_app_name(app_name.clone());
        }

        cfg.interceptor_state().store_put(ua);

        Ok(())
    }

    fn modify_before_signing(
        &self,
        context: &mut BeforeTransmitInterceptorContextMut<'_>,
        runtime_components: &RuntimeComponents,
        cfg: &mut ConfigBag,
    ) -> Result<(), BoxError> {
        let mut ua = cfg
            .load::<AwsUserAgent>()
            .expect("`AwsUserAgent should have been created in `read_before_execution`")
            .clone();

        let mut added_metrics = std::collections::HashSet::new();

        add_metrics_unique!(cfg.load::<SmithySdkFeature>(), &mut ua, &mut added_metrics);
        add_metrics_unique!(cfg.load::<AwsSdkFeature>(), &mut ua, &mut added_metrics);
        // The order we emit credential features matters.
        // Reverse to preserve emission order since StoreAppend pops backwards.
        add_metrics_unique_reverse!(
            cfg.load::<AwsCredentialFeature>(),
            &mut ua,
            &mut added_metrics
        );

        let maybe_connector_metadata = runtime_components
            .http_client()
            .and_then(|c| c.connector_metadata());
        if let Some(connector_metadata) = maybe_connector_metadata {
            let am = AdditionalMetadata::new(Cow::Owned(connector_metadata.to_string()))?;
            ua.add_additional_metadata(am);
        }

        let headers = context.request_mut().headers_mut();
        let (user_agent, x_amz_user_agent) = header_values(&ua)?;
        headers.append(USER_AGENT, user_agent);
        headers.append(X_AMZ_USER_AGENT, x_amz_user_agent);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aws_smithy_runtime_api::client::interceptors::context::{Input, InterceptorContext};
    use aws_smithy_runtime_api::client::interceptors::Intercept;
    use aws_smithy_runtime_api::client::orchestrator::HttpRequest;
    use aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder;
    use aws_smithy_types::config_bag::{ConfigBag, Layer};
    use aws_smithy_types::error::display::DisplayErrorContext;

    fn expect_header<'a>(context: &'a InterceptorContext, header_name: &str) -> &'a str {
        context
            .request()
            .expect("request is set")
            .headers()
            .get(header_name)
            .unwrap()
    }

    fn context() -> InterceptorContext {
        let mut context = InterceptorContext::new(Input::doesnt_matter());
        context.enter_serialization_phase();
        context.set_request(HttpRequest::empty());
        let _ = context.take_input();
        context.enter_before_transmit_phase();
        context
    }

    #[test]
    fn test_overridden_ua() {
        let rc = RuntimeComponentsBuilder::for_tests().build().unwrap();
        let mut context = context();

        let mut layer = Layer::new("test");
        layer.store_put(AwsUserAgent::for_tests());
        layer.store_put(ApiMetadata::new("unused", "unused"));
        let mut cfg = ConfigBag::of_layers(vec![layer]);

        let interceptor = UserAgentInterceptor::new();
        let mut ctx = Into::into(&mut context);
        interceptor
            .modify_before_signing(&mut ctx, &rc, &mut cfg)
            .unwrap();

        let header = expect_header(&context, "user-agent");
        assert_eq!(AwsUserAgent::for_tests().ua_header(), header);
        assert!(!header.contains("unused"));

        assert_eq!(
            AwsUserAgent::for_tests().aws_ua_header(),
            expect_header(&context, "x-amz-user-agent")
        );
    }

    #[test]
    fn test_default_ua() {
        let rc = RuntimeComponentsBuilder::for_tests().build().unwrap();
        let mut context = context();

        let api_metadata = ApiMetadata::new("some-service", "some-version");
        let mut layer = Layer::new("test");
        layer.store_put(api_metadata.clone());
        let mut config = ConfigBag::of_layers(vec![layer]);

        let interceptor = UserAgentInterceptor::new();
        let ctx = Into::into(&context);
        interceptor
            .read_after_serialization(&ctx, &rc, &mut config)
            .unwrap();
        let mut ctx = Into::into(&mut context);
        interceptor
            .modify_before_signing(&mut ctx, &rc, &mut config)
            .unwrap();

        let expected_ua = AwsUserAgent::new_from_environment(Env::real(), api_metadata);
        assert!(
            expected_ua.aws_ua_header().contains("some-service"),
            "precondition"
        );
        assert_eq!(
            expected_ua.ua_header(),
            expect_header(&context, "user-agent")
        );
        assert_eq!(
            expected_ua.aws_ua_header(),
            expect_header(&context, "x-amz-user-agent")
        );
    }

    #[test]
    fn test_modify_before_signing_no_duplicate_metrics() {
        let rc = RuntimeComponentsBuilder::for_tests().build().unwrap();
        let mut context = context();

        let api_metadata = ApiMetadata::new("test-service", "1.0");
        let mut layer = Layer::new("test");
        layer.store_put(api_metadata);
        // Duplicate features
        layer.store_append(SmithySdkFeature::Waiter);
        layer.store_append(SmithySdkFeature::Waiter);
        layer.store_append(AwsSdkFeature::S3Transfer);
        layer.store_append(AwsSdkFeature::S3Transfer);
        layer.store_append(AwsCredentialFeature::CredentialsCode);
        layer.store_append(AwsCredentialFeature::CredentialsCode);
        let mut config = ConfigBag::of_layers(vec![layer]);

        let interceptor = UserAgentInterceptor::new();
        let ctx = Into::into(&context);
        interceptor
            .read_after_serialization(&ctx, &rc, &mut config)
            .unwrap();
        let mut ctx = Into::into(&mut context);
        interceptor
            .modify_before_signing(&mut ctx, &rc, &mut config)
            .unwrap();

        let aws_ua_header = expect_header(&context, "x-amz-user-agent");
        let metrics_section = aws_ua_header.split(" m/").nth(1).unwrap();
        let waiter_count = metrics_section.matches("B").count();
        let s3_transfer_count = metrics_section.matches("G").count();
        let credentials_code_count = metrics_section.matches("e").count();
        assert_eq!(
            1, waiter_count,
            "Waiter metric should appear only once, but found {waiter_count} occurrences in: {aws_ua_header}",
        );
        assert_eq!(1, s3_transfer_count, "S3Transfer metric should appear only once, but found {s3_transfer_count} occurrences in metrics section: {aws_ua_header}");
        assert_eq!(1, credentials_code_count, "CredentialsCode metric should appear only once, but found {credentials_code_count} occurrences in metrics section: {aws_ua_header}");
    }

    #[test]
    fn test_metrics_order_preserved() {
        use aws_credential_types::credential_feature::AwsCredentialFeature;

        let rc = RuntimeComponentsBuilder::for_tests().build().unwrap();
        let mut context = context();

        let api_metadata = ApiMetadata::new("test-service", "1.0");
        let mut layer = Layer::new("test");
        layer.store_put(api_metadata);
        layer.store_append(AwsCredentialFeature::CredentialsCode);
        layer.store_append(AwsCredentialFeature::CredentialsEnvVars);
        layer.store_append(AwsCredentialFeature::CredentialsProfile);
        let mut config = ConfigBag::of_layers(vec![layer]);

        let interceptor = UserAgentInterceptor::new();
        let ctx = Into::into(&context);
        interceptor
            .read_after_serialization(&ctx, &rc, &mut config)
            .unwrap();
        let mut ctx = Into::into(&mut context);
        interceptor
            .modify_before_signing(&mut ctx, &rc, &mut config)
            .unwrap();

        let aws_ua_header = expect_header(&context, "x-amz-user-agent");
        let metrics_section = aws_ua_header.split(" m/").nth(1).unwrap();

        assert_eq!(
            metrics_section, "e,g,n",
            "AwsCredentialFeature metrics should preserve order"
        );
    }

    #[test]
    fn test_app_name() {
        let rc = RuntimeComponentsBuilder::for_tests().build().unwrap();
        let mut context = context();

        let api_metadata = ApiMetadata::new("some-service", "some-version");
        let mut layer = Layer::new("test");
        layer.store_put(api_metadata);
        layer.store_put(AppName::new("my_awesome_app").unwrap());
        let mut config = ConfigBag::of_layers(vec![layer]);

        let interceptor = UserAgentInterceptor::new();
        let ctx = Into::into(&context);
        interceptor
            .read_after_serialization(&ctx, &rc, &mut config)
            .unwrap();
        let mut ctx = Into::into(&mut context);
        interceptor
            .modify_before_signing(&mut ctx, &rc, &mut config)
            .unwrap();

        let app_value = "app/my_awesome_app";
        let header = expect_header(&context, "user-agent");
        assert!(
            !header.contains(app_value),
            "expected `{header}` to not contain `{app_value}`"
        );

        let header = expect_header(&context, "x-amz-user-agent");
        assert!(
            header.contains(app_value),
            "expected `{header}` to contain `{app_value}`"
        );
    }

    #[test]
    fn test_api_metadata_missing() {
        let rc = RuntimeComponentsBuilder::for_tests().build().unwrap();
        let context = context();
        let mut config = ConfigBag::base();

        let interceptor = UserAgentInterceptor::new();
        let ctx = Into::into(&context);

        let error = format!(
            "{}",
            DisplayErrorContext(
                &*interceptor
                    .read_after_serialization(&ctx, &rc, &mut config)
                    .expect_err("it should error")
            )
        );
        assert!(
            error.contains("This is a bug"),
            "`{error}` should contain message `This is a bug`"
        );
    }

    #[test]
    fn test_api_metadata_missing_with_ua_override() {
        let rc = RuntimeComponentsBuilder::for_tests().build().unwrap();
        let mut context = context();

        let mut layer = Layer::new("test");
        layer.store_put(AwsUserAgent::for_tests());
        let mut config = ConfigBag::of_layers(vec![layer]);

        let interceptor = UserAgentInterceptor::new();
        let mut ctx = Into::into(&mut context);

        interceptor
            .modify_before_signing(&mut ctx, &rc, &mut config)
            .expect("it should succeed");

        let header = expect_header(&context, "user-agent");
        assert_eq!(AwsUserAgent::for_tests().ua_header(), header);
        assert!(!header.contains("unused"));

        assert_eq!(
            AwsUserAgent::for_tests().aws_ua_header(),
            expect_header(&context, "x-amz-user-agent")
        );
    }
}
