/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_sdk_s3::config::interceptors::BeforeTransmitInterceptorContextMut;
use aws_sdk_s3::config::{Credentials, Region};
use aws_sdk_s3::Client;
use aws_smithy_http_client::test_util::capture_request;
use aws_smithy_runtime_api::box_error::BoxError;
use aws_smithy_runtime_api::client::interceptors::Intercept;
use aws_smithy_runtime_api::client::runtime_components::RuntimeComponents;
use aws_smithy_types::config_bag::{ConfigBag, Layer, Storable, StoreReplace};
use http_1x::header::USER_AGENT;
use http_1x::HeaderValue;

#[tokio::test]
async fn interceptor_priority() {
    #[derive(Debug, Eq, PartialEq)]
    struct TestValue(&'static str);
    impl Storable for TestValue {
        type Storer = StoreReplace<Self>;
    }

    #[derive(Debug)]
    struct TestInterceptor(&'static str);
    impl Intercept for TestInterceptor {
        fn name(&self) -> &'static str {
            "TestInterceptor"
        }

        fn modify_before_signing(
            &self,
            _context: &mut BeforeTransmitInterceptorContextMut<'_>,
            _components: &RuntimeComponents,
            cfg: &mut ConfigBag,
        ) -> Result<(), BoxError> {
            let mut layer = Layer::new("test");
            layer.store_put(TestValue(self.0));
            cfg.push_layer(layer);
            Ok(())
        }

        fn modify_before_transmit(
            &self,
            context: &mut BeforeTransmitInterceptorContextMut<'_>,
            _runtime_components: &RuntimeComponents,
            cfg: &mut ConfigBag,
        ) -> Result<(), BoxError> {
            let value = cfg.load::<TestValue>().unwrap();
            context
                .request_mut()
                .headers_mut()
                .insert("test-header", HeaderValue::from_static(value.0));
            Ok(())
        }
    }

    let (http_client, rx) = capture_request(None);

    // The first `TestInterceptor` will put `value1` into config
    let config = aws_sdk_s3::Config::builder()
        .credentials_provider(Credentials::for_tests())
        .region(Region::new("us-east-1"))
        .http_client(http_client)
        .interceptor(TestInterceptor("value1"))
        .build();
    let client = Client::from_conf(config);

    // The second `TestInterceptor` will replace `value1` with `value2` in config
    dbg!(
        client
            .list_objects_v2()
            .bucket("test-bucket")
            .prefix("prefix~")
            .customize()
            .interceptor(TestInterceptor("value2"))
            .send()
            .await
    )
    .expect_err("no fake response set");

    let request = rx.expect_request();
    assert_eq!("value2", request.headers().get("test-header").unwrap());
}

#[tokio::test]
async fn set_test_user_agent_through_request_mutation() {
    let (http_client, rx) = capture_request(None);

    let config = aws_sdk_s3::Config::builder()
        .credentials_provider(Credentials::for_tests())
        .region(Region::new("us-east-1"))
        .http_client(http_client.clone())
        .build();
    let client = Client::from_conf(config);

    dbg!(
        client
            .list_objects_v2()
            .bucket("test-bucket")
            .prefix("prefix~")
            .customize()
            .mutate_request(|request| {
                let headers = request.headers_mut();
                headers.insert(USER_AGENT, HeaderValue::try_from("test").unwrap());
                headers.insert("x-amz-user-agent", HeaderValue::try_from("test").unwrap());
            })
            .send()
            .await
    )
    .expect_err("no fake response set");

    let request = rx.expect_request();
    assert_eq!("test", request.headers().get(USER_AGENT).unwrap());
    assert_eq!("test", request.headers().get("x-amz-user-agent").unwrap());
}
