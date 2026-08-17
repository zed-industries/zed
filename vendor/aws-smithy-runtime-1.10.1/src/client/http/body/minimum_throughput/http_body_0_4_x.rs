/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use super::{BoxError, Error, MinimumThroughputDownloadBody};
use crate::client::http::body::minimum_throughput::throughput::DownloadReport;
use crate::client::http::body::minimum_throughput::ThroughputReadingBody;
use aws_smithy_async::rt::sleep::AsyncSleep;
use std::future::Future;
use std::pin::{pin, Pin};
use std::task::{Context, Poll};

impl<B> http_body_04x::Body for MinimumThroughputDownloadBody<B>
where
    B: http_body_04x::Body<Data = bytes::Bytes, Error = BoxError>,
{
    type Data = bytes::Bytes;
    type Error = BoxError;

    fn poll_data(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Self::Data, Self::Error>>> {
        #[allow(unused_imports)]
        use crate::client::http::body::minimum_throughput::throughput::ThroughputReport;
        // this code is called quite frequently in production—one every millisecond or so when downloading
        // a stream. However, SystemTime::now is on the order of nanoseconds
        let now = self.time_source.now();
        // Attempt to read the data from the inner body, then update the
        // throughput logs.
        let mut this = self.as_mut().project();
        let poll_res = match this.inner.poll_data(cx) {
            Poll::Ready(Some(Ok(bytes))) => {
                tracing::trace!("received data: {}", bytes.len());
                this.throughput_logs
                    .push_bytes_transferred(now, bytes.len() as u64);
                Poll::Ready(Some(Ok(bytes)))
            }
            Poll::Pending => {
                tracing::trace!("received poll pending");
                this.throughput_logs.push_pending(now);
                Poll::Pending
            }
            // If we've read all the data or an error occurred, then return that result.
            res => return res,
        };

        // Check the sleep future to see if it needs refreshing.
        let mut sleep_fut = this
            .sleep_fut
            .take()
            .unwrap_or_else(|| this.async_sleep.sleep(*this.resolution));
        if let Poll::Ready(()) = pin!(&mut sleep_fut).poll(cx) {
            tracing::trace!("sleep future triggered—triggering a wakeup");
            // Whenever the sleep future expires, we replace it.
            sleep_fut = this.async_sleep.sleep(*this.resolution);

            // We also schedule a wake up for current task to ensure that
            // it gets polled at least one more time.
            cx.waker().wake_by_ref();
        };
        this.sleep_fut.replace(sleep_fut);

        // Calculate the current throughput and emit an error if it's too low and
        // the grace period has elapsed.
        let report = this.throughput_logs.report(now);
        let (violated, current_throughput) =
            report.minimum_throughput_violated(this.options.minimum_throughput());
        if violated {
            if this.grace_period_fut.is_none() {
                tracing::debug!("entering minimum throughput grace period");
            }
            let mut grace_period_fut = this
                .grace_period_fut
                .take()
                .unwrap_or_else(|| this.async_sleep.sleep(this.options.grace_period()));
            if let Poll::Ready(()) = pin!(&mut grace_period_fut).poll(cx) {
                // The grace period has ended!
                return Poll::Ready(Some(Err(Box::new(Error::ThroughputBelowMinimum {
                    expected: self.options.minimum_throughput(),
                    actual: current_throughput,
                }))));
            };
            this.grace_period_fut.replace(grace_period_fut);
        } else {
            // Ensure we don't have an active grace period future if we're not
            // currently below the minimum throughput.
            if this.grace_period_fut.is_some() {
                tracing::debug!("throughput recovered; exiting grace period");
            }
            let _ = this.grace_period_fut.take();
        }

        poll_res
    }

    fn poll_trailers(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<Option<http_02x::HeaderMap>, Self::Error>> {
        let this = self.as_mut().project();
        this.inner.poll_trailers(cx)
    }

    fn is_end_stream(&self) -> bool {
        self.inner.is_end_stream()
    }

    fn size_hint(&self) -> http_body_04x::SizeHint {
        self.inner.size_hint()
    }
}

impl<B> http_body_04x::Body for ThroughputReadingBody<B>
where
    B: http_body_04x::Body<Data = bytes::Bytes, Error = BoxError>,
{
    type Data = bytes::Bytes;
    type Error = BoxError;

    fn poll_data(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Self::Data, Self::Error>>> {
        // this code is called quite frequently in production—one every millisecond or so when downloading
        // a stream. However, SystemTime::now is on the order of nanoseconds
        let now = self.time_source.now();
        // Attempt to read the data from the inner body, then update the
        // throughput logs.
        let this = self.as_mut().project();
        match this.inner.poll_data(cx) {
            Poll::Ready(Some(Ok(bytes))) => {
                tracing::trace!("received data: {}", bytes.len());
                this.throughput
                    .push_bytes_transferred(now, bytes.len() as u64);

                // hyper will optimistically stop polling when end of stream is reported
                // (e.g. when content-length amount of data has been consumed) which means
                // we may never get to `Poll:Ready(None)`. Check for same condition and
                // attempt to stop checking throughput violations _now_ as we may never
                // get polled again. The caveat here is that it depends on `Body` implementations
                // implementing `is_end_stream()` correctly. Users can also disable SSP as an
                // alternative for such fringe use cases.
                if self.is_end_stream() {
                    tracing::trace!("stream reported end of stream before Poll::Ready(None) reached; marking stream complete");
                    self.throughput.mark_complete();
                }
                Poll::Ready(Some(Ok(bytes)))
            }
            Poll::Pending => {
                tracing::trace!("received poll pending");
                this.throughput.push_pending(now);
                Poll::Pending
            }
            // If we've read all the data or an error occurred, then return that result.
            res => {
                if this.throughput.mark_complete() {
                    tracing::trace!("stream completed: {:?}", res);
                }
                res
            }
        }
    }

    fn poll_trailers(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<Option<http_02x::HeaderMap>, Self::Error>> {
        let this = self.as_mut().project();
        this.inner.poll_trailers(cx)
    }

    fn is_end_stream(&self) -> bool {
        self.inner.is_end_stream()
    }

    fn size_hint(&self) -> http_body_04x::SizeHint {
        self.inner.size_hint()
    }
}
