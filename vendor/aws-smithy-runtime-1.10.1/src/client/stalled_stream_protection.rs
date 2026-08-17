/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::client::http::body::minimum_throughput::{
    options::MinimumThroughputBodyOptions, MinimumThroughputDownloadBody, ThroughputReadingBody,
    UploadThroughput,
};
use aws_smithy_async::rt::sleep::SharedAsyncSleep;
use aws_smithy_async::time::SharedTimeSource;
use aws_smithy_runtime_api::box_error::BoxError;
use aws_smithy_runtime_api::client::interceptors::context::{
    BeforeDeserializationInterceptorContextMut, BeforeTransmitInterceptorContextMut,
};
use aws_smithy_runtime_api::client::interceptors::Intercept;
use aws_smithy_runtime_api::client::runtime_components::RuntimeComponents;
use aws_smithy_runtime_api::client::stalled_stream_protection::StalledStreamProtectionConfig;
use aws_smithy_types::body::SdkBody;
use aws_smithy_types::config_bag::ConfigBag;
use std::mem;

/// Adds stalled stream protection when sending requests and/or receiving responses.
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct StalledStreamProtectionInterceptor;

/// Stalled stream protection can be enable for request bodies, response bodies,
/// or both.
#[deprecated(
    since = "1.2.0",
    note = "This kind enum is no longer used. Configuration is stored in StalledStreamProtectionConfig in the config bag."
)]
pub enum StalledStreamProtectionInterceptorKind {
    /// Enable stalled stream protection for request bodies.
    RequestBody,
    /// Enable stalled stream protection for response bodies.
    ResponseBody,
    /// Enable stalled stream protection for both request and response bodies.
    RequestAndResponseBody,
}

impl StalledStreamProtectionInterceptor {
    /// Create a new stalled stream protection interceptor.
    #[deprecated(
        since = "1.2.0",
        note = "The kind enum is no longer used. Configuration is stored in StalledStreamProtectionConfig in the config bag. Construct the interceptor using Default."
    )]
    #[allow(deprecated)]
    pub fn new(_kind: StalledStreamProtectionInterceptorKind) -> Self {
        Default::default()
    }
}

impl Intercept for StalledStreamProtectionInterceptor {
    fn name(&self) -> &'static str {
        "StalledStreamProtectionInterceptor"
    }

    fn modify_before_transmit(
        &self,
        context: &mut BeforeTransmitInterceptorContextMut<'_>,
        runtime_components: &RuntimeComponents,
        cfg: &mut ConfigBag,
    ) -> Result<(), BoxError> {
        if let Some(sspcfg) = cfg.load::<StalledStreamProtectionConfig>().cloned() {
            if sspcfg.upload_enabled() {
                if let Some(0) = context.request().body().content_length() {
                    tracing::trace!(
                        "skipping stalled stream protection for zero length request body"
                    );
                    return Ok(());
                }
                let (_async_sleep, time_source) = get_runtime_component_deps(runtime_components)?;
                let now = time_source.now();

                let options: MinimumThroughputBodyOptions = sspcfg.into();
                let throughput = UploadThroughput::new(options.check_window(), now);
                cfg.interceptor_state().store_put(throughput.clone());

                tracing::trace!("adding stalled stream protection to request body");
                let it = mem::replace(context.request_mut().body_mut(), SdkBody::taken());
                let it = it.map_preserve_contents(move |body| {
                    let time_source = time_source.clone();
                    SdkBody::from_body_1_x(ThroughputReadingBody::new(
                        time_source,
                        throughput.clone(),
                        body,
                    ))
                });
                let _ = mem::replace(context.request_mut().body_mut(), it);
            }
        }

        Ok(())
    }

    fn modify_before_deserialization(
        &self,
        context: &mut BeforeDeserializationInterceptorContextMut<'_>,
        runtime_components: &RuntimeComponents,
        cfg: &mut ConfigBag,
    ) -> Result<(), BoxError> {
        if let Some(sspcfg) = cfg.load::<StalledStreamProtectionConfig>() {
            if sspcfg.download_enabled() {
                let (async_sleep, time_source) = get_runtime_component_deps(runtime_components)?;
                tracing::trace!("adding stalled stream protection to response body");
                let sspcfg = sspcfg.clone();
                let it = mem::replace(context.response_mut().body_mut(), SdkBody::taken());
                let it = it.map_preserve_contents(move |body| {
                    let sspcfg = sspcfg.clone();
                    let async_sleep = async_sleep.clone();
                    let time_source = time_source.clone();
                    let mtb = MinimumThroughputDownloadBody::new(
                        time_source,
                        async_sleep,
                        body,
                        sspcfg.into(),
                    );
                    SdkBody::from_body_1_x(mtb)
                });
                let _ = mem::replace(context.response_mut().body_mut(), it);
            }
        }
        Ok(())
    }
}

fn get_runtime_component_deps(
    runtime_components: &RuntimeComponents,
) -> Result<(SharedAsyncSleep, SharedTimeSource), BoxError> {
    let async_sleep = runtime_components.sleep_impl().ok_or(
        "An async sleep implementation is required when stalled stream protection is enabled",
    )?;
    let time_source = runtime_components
        .time_source()
        .ok_or("A time source is required when stalled stream protection is enabled")?;
    Ok((async_sleep, time_source))
}
