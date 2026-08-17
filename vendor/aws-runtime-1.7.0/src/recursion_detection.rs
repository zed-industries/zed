/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_smithy_runtime_api::box_error::BoxError;
use aws_smithy_runtime_api::client::interceptors::context::BeforeTransmitInterceptorContextMut;
use aws_smithy_runtime_api::client::interceptors::Intercept;
use aws_smithy_runtime_api::client::runtime_components::RuntimeComponents;
use aws_smithy_types::config_bag::ConfigBag;
use aws_types::os_shim_internal::Env;
use http_1x::HeaderValue;
use percent_encoding::{percent_encode, CONTROLS};
use std::borrow::Cow;

const TRACE_ID_HEADER: &str = "x-amzn-trace-id";

mod env {
    pub(super) const LAMBDA_FUNCTION_NAME: &str = "AWS_LAMBDA_FUNCTION_NAME";
    pub(super) const TRACE_ID: &str = "_X_AMZN_TRACE_ID";
}

/// Recursion Detection Interceptor
///
/// This interceptor inspects the value of the `AWS_LAMBDA_FUNCTION_NAME` and `_X_AMZN_TRACE_ID` environment
/// variables to detect if the request is being invoked in a Lambda function. If it is, the `X-Amzn-Trace-Id` header
/// will be set. This enables downstream services to prevent accidentally infinitely recursive invocations spawned
/// from Lambda.
#[non_exhaustive]
#[derive(Debug, Default)]
pub struct RecursionDetectionInterceptor {
    env: Env,
}

impl RecursionDetectionInterceptor {
    /// Creates a new `RecursionDetectionInterceptor`
    pub fn new() -> Self {
        Self::default()
    }
}

impl Intercept for RecursionDetectionInterceptor {
    fn name(&self) -> &'static str {
        "RecursionDetectionInterceptor"
    }

    fn modify_before_signing(
        &self,
        context: &mut BeforeTransmitInterceptorContextMut<'_>,
        _runtime_components: &RuntimeComponents,
        _cfg: &mut ConfigBag,
    ) -> Result<(), BoxError> {
        let request = context.request_mut();
        if request.headers().contains_key(TRACE_ID_HEADER) {
            return Ok(());
        }

        if let (Ok(_function_name), Ok(trace_id)) = (
            self.env.get(env::LAMBDA_FUNCTION_NAME),
            self.env.get(env::TRACE_ID),
        ) {
            request
                .headers_mut()
                .insert(TRACE_ID_HEADER, encode_header(trace_id.as_bytes()));
        }
        Ok(())
    }
}

/// Encodes a byte slice as a header.
///
/// ASCII control characters are percent encoded which ensures that all byte sequences are valid headers
fn encode_header(value: &[u8]) -> HeaderValue {
    let value: Cow<'_, str> = percent_encode(value, CONTROLS).into();
    HeaderValue::from_bytes(value.as_bytes()).expect("header is encoded, header must be valid")
}

#[cfg(test)]
mod tests {
    use super::*;
    use aws_smithy_protocol_test::{assert_ok, validate_headers};
    use aws_smithy_runtime_api::client::interceptors::context::{Input, InterceptorContext};
    use aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder;
    use aws_smithy_types::body::SdkBody;
    use aws_types::os_shim_internal::Env;
    use http_1x::HeaderValue;
    use proptest::{prelude::*, proptest};
    use serde::Deserialize;
    use std::collections::HashMap;

    proptest! {
        #[test]
        fn header_encoding_never_panics(s in any::<Vec<u8>>()) {
            encode_header(&s);
        }
    }

    #[test]
    fn every_char() {
        let buff = (0..=255).collect::<Vec<u8>>();
        assert_eq!(
            encode_header(&buff),
            HeaderValue::from_static(
                r##"%00%01%02%03%04%05%06%07%08%09%0A%0B%0C%0D%0E%0F%10%11%12%13%14%15%16%17%18%19%1A%1B%1C%1D%1E%1F !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~%7F%80%81%82%83%84%85%86%87%88%89%8A%8B%8C%8D%8E%8F%90%91%92%93%94%95%96%97%98%99%9A%9B%9C%9D%9E%9F%A0%A1%A2%A3%A4%A5%A6%A7%A8%A9%AA%AB%AC%AD%AE%AF%B0%B1%B2%B3%B4%B5%B6%B7%B8%B9%BA%BB%BC%BD%BE%BF%C0%C1%C2%C3%C4%C5%C6%C7%C8%C9%CA%CB%CC%CD%CE%CF%D0%D1%D2%D3%D4%D5%D6%D7%D8%D9%DA%DB%DC%DD%DE%DF%E0%E1%E2%E3%E4%E5%E6%E7%E8%E9%EA%EB%EC%ED%EE%EF%F0%F1%F2%F3%F4%F5%F6%F7%F8%F9%FA%FB%FC%FD%FE%FF"##
            )
        );
    }

    #[test]
    fn run_tests() {
        let test_cases: Vec<TestCase> =
            serde_json::from_str(include_str!("../test-data/recursion-detection.json"))
                .expect("invalid test case");
        for test_case in test_cases {
            check(test_case)
        }
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct TestCase {
        env: HashMap<String, String>,
        request_headers_before: Vec<String>,
        request_headers_after: Vec<String>,
    }

    impl TestCase {
        fn env(&self) -> Env {
            Env::from(self.env.clone())
        }

        /// Headers on the input request
        fn request_headers_before(&self) -> impl Iterator<Item = (&str, &str)> {
            Self::split_headers(&self.request_headers_before)
        }

        /// Headers on the output request
        fn request_headers_after(&self) -> impl Iterator<Item = (&str, &str)> {
            Self::split_headers(&self.request_headers_after)
        }

        /// Split text headers on `: `
        fn split_headers(headers: &[String]) -> impl Iterator<Item = (&str, &str)> {
            headers
                .iter()
                .map(|header| header.split_once(": ").expect("header must contain :"))
        }
    }

    fn check(test_case: TestCase) {
        let rc = RuntimeComponentsBuilder::for_tests().build().unwrap();
        let env = test_case.env();
        let mut request = http_1x::Request::builder();
        for (name, value) in test_case.request_headers_before() {
            request = request.header(name, value);
        }
        let request = request
            .body(SdkBody::empty())
            .expect("must be valid")
            .try_into()
            .unwrap();
        let mut context = InterceptorContext::new(Input::doesnt_matter());
        context.enter_serialization_phase();
        context.set_request(request);
        let _ = context.take_input();
        context.enter_before_transmit_phase();
        let mut config = ConfigBag::base();

        let mut ctx = Into::into(&mut context);
        RecursionDetectionInterceptor { env }
            .modify_before_signing(&mut ctx, &rc, &mut config)
            .expect("interceptor must succeed");
        let mutated_request = context.request().expect("request is set");
        for (name, _) in mutated_request.headers() {
            assert_eq!(
                mutated_request.headers().get_all(name).count(),
                1,
                "No duplicated headers"
            )
        }
        assert_ok(validate_headers(
            mutated_request.headers(),
            test_case.request_headers_after(),
        ))
    }
}
