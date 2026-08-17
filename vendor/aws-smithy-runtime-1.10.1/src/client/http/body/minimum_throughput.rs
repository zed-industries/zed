/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! A body-wrapping type that ensures data is being streamed faster than some lower limit.
//!
//! If data is being streamed too slowly, this body type will emit an error next time it's polled.

/// An implementation of v0.4 `http_body::Body` for `MinimumThroughputBody` and related code.
pub mod http_body_0_4_x;

/// An implementation of v1.0 `http_body::Body` for `MinimumThroughputBody` and related code.
pub mod http_body_1_x;

/// Options for a [`MinimumThroughputBody`].
pub mod options;
pub use throughput::Throughput;
mod throughput;

use crate::client::http::body::minimum_throughput::throughput::ThroughputReport;
use aws_smithy_async::rt::sleep::Sleep;
use aws_smithy_async::rt::sleep::{AsyncSleep, SharedAsyncSleep};
use aws_smithy_async::time::{SharedTimeSource, TimeSource};
use aws_smithy_runtime_api::{
    box_error::BoxError,
    client::{
        http::HttpConnectorFuture, result::ConnectorError, runtime_components::RuntimeComponents,
        stalled_stream_protection::StalledStreamProtectionConfig,
    },
};
use aws_smithy_runtime_api::{client::orchestrator::HttpResponse, shared::IntoShared};
use aws_smithy_types::config_bag::{ConfigBag, Storable, StoreReplace};
use options::MinimumThroughputBodyOptions;
use std::{
    fmt,
    sync::{Arc, Mutex},
    task::Poll,
};
use std::{future::Future, pin::Pin};
use std::{
    task::Context,
    time::{Duration, SystemTime},
};
use throughput::ThroughputLogs;

/// Use [`MinimumThroughputDownloadBody`] instead.
#[deprecated(note = "Renamed to MinimumThroughputDownloadBody since it doesn't work for uploads")]
pub type MinimumThroughputBody<B> = MinimumThroughputDownloadBody<B>;

pin_project_lite::pin_project! {
    /// A body-wrapping type that ensures data is being streamed faster than some lower limit.
    ///
    /// If data is being streamed too slowly, this body type will emit an error next time it's polled.
    pub struct MinimumThroughputDownloadBody<B> {
        async_sleep: SharedAsyncSleep,
        time_source: SharedTimeSource,
        options: MinimumThroughputBodyOptions,
        throughput_logs: ThroughputLogs,
        resolution: Duration,
        #[pin]
        sleep_fut: Option<Sleep>,
        #[pin]
        grace_period_fut: Option<Sleep>,
        #[pin]
        inner: B,
    }
}

impl<B> MinimumThroughputDownloadBody<B> {
    /// Create a new minimum throughput body.
    pub fn new(
        time_source: impl TimeSource + 'static,
        async_sleep: impl AsyncSleep + 'static,
        body: B,
        options: MinimumThroughputBodyOptions,
    ) -> Self {
        let time_source: SharedTimeSource = time_source.into_shared();
        let now = time_source.now();
        let throughput_logs = ThroughputLogs::new(options.check_window(), now);
        let resolution = throughput_logs.resolution();
        Self {
            throughput_logs,
            resolution,
            async_sleep: async_sleep.into_shared(),
            time_source,
            inner: body,
            sleep_fut: None,
            grace_period_fut: None,
            options,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Error {
    ThroughputBelowMinimum {
        expected: Throughput,
        actual: Throughput,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ThroughputBelowMinimum { expected, actual } => {
                write!(
                    f,
                    "minimum throughput was specified at {expected}, but throughput of {actual} was observed",
                )
            }
        }
    }
}

impl std::error::Error for Error {}

/// Used to store the upload throughput in the interceptor context.
#[derive(Clone, Debug)]
pub(crate) struct UploadThroughput {
    logs: Arc<Mutex<ThroughputLogs>>,
}

impl UploadThroughput {
    pub(crate) fn new(time_window: Duration, now: SystemTime) -> Self {
        Self {
            logs: Arc::new(Mutex::new(ThroughputLogs::new(time_window, now))),
        }
    }

    pub(crate) fn resolution(&self) -> Duration {
        self.logs.lock().unwrap().resolution()
    }

    pub(crate) fn push_pending(&self, now: SystemTime) {
        self.logs.lock().unwrap().push_pending(now);
    }
    pub(crate) fn push_bytes_transferred(&self, now: SystemTime, bytes: u64) {
        self.logs.lock().unwrap().push_bytes_transferred(now, bytes);
    }

    pub(crate) fn mark_complete(&self) -> bool {
        self.logs.lock().unwrap().mark_complete()
    }

    pub(crate) fn report(&self, now: SystemTime) -> ThroughputReport {
        self.logs.lock().unwrap().report(now)
    }
}

impl Storable for UploadThroughput {
    type Storer = StoreReplace<Self>;
}

pin_project_lite::pin_project! {
    pub(crate) struct ThroughputReadingBody<B> {
        time_source: SharedTimeSource,
        throughput: UploadThroughput,
        #[pin]
        inner: B,
    }
}

impl<B> ThroughputReadingBody<B> {
    pub(crate) fn new(
        time_source: SharedTimeSource,
        throughput: UploadThroughput,
        body: B,
    ) -> Self {
        Self {
            time_source,
            throughput,
            inner: body,
        }
    }
}

const ZERO_THROUGHPUT: Throughput = Throughput::new_bytes_per_second(0);

// Helper trait for interpretting the throughput report.
trait UploadReport {
    fn minimum_throughput_violated(self, minimum_throughput: Throughput) -> (bool, Throughput);
}
impl UploadReport for ThroughputReport {
    fn minimum_throughput_violated(self, minimum_throughput: Throughput) -> (bool, Throughput) {
        let throughput = match self {
            // stream has been exhausted, stop tracking violations
            ThroughputReport::Complete => return (false, ZERO_THROUGHPUT),
            // If the report is incomplete, then we don't have enough data yet to
            // decide if minimum throughput was violated.
            ThroughputReport::Incomplete => {
                tracing::trace!(
                    "not enough data to decide if minimum throughput has been violated"
                );
                return (false, ZERO_THROUGHPUT);
            }
            // If most of the datapoints are Poll::Pending, then the user has stalled.
            // In this case, we don't want to say minimum throughput was violated.
            ThroughputReport::Pending => {
                tracing::debug!(
                    "the user has stalled; this will not become a minimum throughput violation"
                );
                return (false, ZERO_THROUGHPUT);
            }
            // If there has been no polling, then the server has stalled. Alternatively,
            // if we're transferring data, but it's too slow, then we also want to say
            // that the minimum throughput has been violated.
            ThroughputReport::NoPolling => ZERO_THROUGHPUT,
            ThroughputReport::Transferred(tp) => tp,
        };
        if throughput < minimum_throughput {
            tracing::debug!(
                "current throughput: {throughput} is below minimum: {minimum_throughput}"
            );
            (true, throughput)
        } else {
            (false, throughput)
        }
    }
}

pin_project_lite::pin_project! {
    /// Future that pairs with [`UploadThroughput`] to add a minimum throughput
    /// requirement to a request upload stream.
    pub(crate) struct UploadThroughputCheckFuture {
        #[pin]
        response: HttpConnectorFuture,
        #[pin]
        check_interval: Option<Sleep>,
        #[pin]
        grace_period: Option<Sleep>,

        time_source: SharedTimeSource,
        sleep_impl: SharedAsyncSleep,
        upload_throughput: UploadThroughput,
        resolution: Duration,
        options: MinimumThroughputBodyOptions,

        failing_throughput: Option<Throughput>,
    }
}

impl UploadThroughputCheckFuture {
    fn new(
        response: HttpConnectorFuture,
        time_source: SharedTimeSource,
        sleep_impl: SharedAsyncSleep,
        upload_throughput: UploadThroughput,
        options: MinimumThroughputBodyOptions,
    ) -> Self {
        let resolution = upload_throughput.resolution();
        Self {
            response,
            check_interval: Some(sleep_impl.sleep(resolution)),
            grace_period: None,
            time_source,
            sleep_impl,
            upload_throughput,
            resolution,
            options,
            failing_throughput: None,
        }
    }
}

impl Future for UploadThroughputCheckFuture {
    type Output = Result<HttpResponse, ConnectorError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.project();

        if let Poll::Ready(output) = this.response.poll(cx) {
            return Poll::Ready(output);
        } else {
            let mut below_minimum_throughput = false;
            let check_interval_expired = this
                .check_interval
                .as_mut()
                .as_pin_mut()
                .expect("always set")
                .poll(cx)
                .is_ready();
            if check_interval_expired {
                // Set up the next check interval
                *this.check_interval = Some(this.sleep_impl.sleep(*this.resolution));

                // Wake so that the check interval future gets polled
                // next time this poll method is called. If it never gets polled,
                // then this task won't be woken to check again.
                cx.waker().wake_by_ref();
            }

            let should_check = check_interval_expired || this.grace_period.is_some();
            if should_check {
                let now = this.time_source.now();
                let report = this.upload_throughput.report(now);
                let (violated, current_throughput) =
                    report.minimum_throughput_violated(this.options.minimum_throughput());
                below_minimum_throughput = violated;
                if below_minimum_throughput && !this.failing_throughput.is_some() {
                    *this.failing_throughput = Some(current_throughput);
                } else if !below_minimum_throughput {
                    *this.failing_throughput = None;
                }
            }

            // If we kicked off a grace period and are now satisfied, clear out the grace period
            if !below_minimum_throughput && this.grace_period.is_some() {
                tracing::debug!("upload minimum throughput recovered during grace period");
                *this.grace_period = None;
            }
            if below_minimum_throughput {
                // Start a grace period if below minimum throughput
                if this.grace_period.is_none() {
                    tracing::debug!(
                        grace_period=?this.options.grace_period(),
                        "upload minimum throughput below configured minimum; starting grace period"
                    );
                    *this.grace_period = Some(this.sleep_impl.sleep(this.options.grace_period()));
                }
                // Check the grace period if one is already set and we're not satisfied
                if let Some(grace_period) = this.grace_period.as_pin_mut() {
                    if grace_period.poll(cx).is_ready() {
                        tracing::debug!("grace period ended; timing out request");
                        return Poll::Ready(Err(ConnectorError::timeout(
                            Error::ThroughputBelowMinimum {
                                expected: this.options.minimum_throughput(),
                                actual: this
                                    .failing_throughput
                                    .expect("always set if there's a grace period"),
                            }
                            .into(),
                        )));
                    }
                }
            }
        }
        Poll::Pending
    }
}

pin_project_lite::pin_project! {
    #[project = EnumProj]
    pub(crate) enum MaybeUploadThroughputCheckFuture {
        Direct { #[pin] future: HttpConnectorFuture },
        Checked { #[pin] future: UploadThroughputCheckFuture },
    }
}

impl MaybeUploadThroughputCheckFuture {
    pub(crate) fn new(
        cfg: &mut ConfigBag,
        components: &RuntimeComponents,
        connector_future: HttpConnectorFuture,
    ) -> Self {
        if let Some(sspcfg) = cfg.load::<StalledStreamProtectionConfig>().cloned() {
            if sspcfg.is_enabled() {
                let options = MinimumThroughputBodyOptions::from(sspcfg);
                return Self::new_inner(
                    connector_future,
                    components.time_source(),
                    components.sleep_impl(),
                    cfg.interceptor_state().load::<UploadThroughput>().cloned(),
                    Some(options),
                );
            }
        }
        tracing::debug!("no minimum upload throughput checks");
        Self::new_inner(connector_future, None, None, None, None)
    }

    fn new_inner(
        response: HttpConnectorFuture,
        time_source: Option<SharedTimeSource>,
        sleep_impl: Option<SharedAsyncSleep>,
        upload_throughput: Option<UploadThroughput>,
        options: Option<MinimumThroughputBodyOptions>,
    ) -> Self {
        match (time_source, sleep_impl, upload_throughput, options) {
            (Some(time_source), Some(sleep_impl), Some(upload_throughput), Some(options)) => {
                tracing::debug!(options=?options, "applying minimum upload throughput check future");
                Self::Checked {
                    future: UploadThroughputCheckFuture::new(
                        response,
                        time_source,
                        sleep_impl,
                        upload_throughput,
                        options,
                    ),
                }
            }
            _ => Self::Direct { future: response },
        }
    }
}

impl Future for MaybeUploadThroughputCheckFuture {
    type Output = Result<HttpResponse, ConnectorError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.project() {
            EnumProj::Direct { future } => future.poll(cx),
            EnumProj::Checked { future } => future.poll(cx),
        }
    }
}
