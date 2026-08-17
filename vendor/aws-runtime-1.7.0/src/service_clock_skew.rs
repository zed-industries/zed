/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_smithy_runtime_api::box_error::BoxError;
use aws_smithy_runtime_api::client::interceptors::context::BeforeDeserializationInterceptorContextMut;
use aws_smithy_runtime_api::client::interceptors::Intercept;
use aws_smithy_runtime_api::client::runtime_components::RuntimeComponents;
use aws_smithy_types::config_bag::{ConfigBag, Storable, StoreReplace};
use aws_smithy_types::date_time::Format;
use aws_smithy_types::DateTime;
use std::time::Duration;

/// Amount of clock skew between the client and the service.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub(crate) struct ServiceClockSkew {
    inner: Duration,
}

impl ServiceClockSkew {
    fn new(inner: Duration) -> Self {
        Self { inner }
    }
}

impl Storable for ServiceClockSkew {
    type Storer = StoreReplace<Self>;
}

impl From<ServiceClockSkew> for Duration {
    fn from(skew: ServiceClockSkew) -> Duration {
        skew.inner
    }
}

/// Interceptor that determines the clock skew between the client and service.
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct ServiceClockSkewInterceptor;

impl ServiceClockSkewInterceptor {
    /// Creates a new `ServiceClockSkewInterceptor`.
    pub fn new() -> Self {
        Self::default()
    }
}

fn calculate_skew(time_sent: DateTime, time_received: DateTime) -> Duration {
    let skew = (time_sent.as_secs_f64() - time_received.as_secs_f64()).max(0.0);
    Duration::from_secs_f64(skew)
}

fn extract_time_sent_from_response(
    ctx: &mut BeforeDeserializationInterceptorContextMut<'_>,
) -> Result<DateTime, BoxError> {
    let date_header = ctx
        .response()
        .headers()
        .get("date")
        .ok_or("Response from server does not include a `date` header")?;
    DateTime::from_str(date_header, Format::HttpDate).map_err(Into::into)
}

impl Intercept for ServiceClockSkewInterceptor {
    fn name(&self) -> &'static str {
        "ServiceClockSkewInterceptor"
    }

    fn modify_before_deserialization(
        &self,
        ctx: &mut BeforeDeserializationInterceptorContextMut<'_>,
        runtime_components: &RuntimeComponents,
        cfg: &mut ConfigBag,
    ) -> Result<(), BoxError> {
        let time_received = DateTime::from(
            runtime_components
                .time_source()
                .ok_or("a time source is required (service clock skew)")?
                .now(),
        );
        let time_sent = match extract_time_sent_from_response(ctx) {
            Ok(time_sent) => time_sent,
            Err(e) => {
                // We don't want to fail a request for this because 1xx and 5xx responses and
                // responses from servers with no clock may omit this header. We still log it at the
                // trace level to aid in debugging.
                tracing::trace!("failed to calculate clock skew of service from response: {e}. Ignoring this error...",);
                return Ok(());
            }
        };
        let skew = ServiceClockSkew::new(calculate_skew(time_sent, time_received));
        cfg.interceptor_state().store_put(skew);
        Ok(())
    }
}
