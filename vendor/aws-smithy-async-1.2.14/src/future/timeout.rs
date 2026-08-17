/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

// This code was copied and then modified from Tokio.

/*
 * Copyright (c) 2021 Tokio Contributors
 *
 * Permission is hereby granted, free of charge, to any
 * person obtaining a copy of this software and associated
 * documentation files (the "Software"), to deal in the
 * Software without restriction, including without
 * limitation the rights to use, copy, modify, merge,
 * publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software
 * is furnished to do so, subject to the following
 * conditions:
 *
 * The above copyright notice and this permission notice
 * shall be included in all copies or substantial portions
 * of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
 * ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
 * TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
 * PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
 * SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
 * CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
 * OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
 * IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 * DEALINGS IN THE SOFTWARE.
 */

//! Provides the [`Timeout`] future for adding a timeout to another future.

use pin_project_lite::pin_project;
use std::error::Error;
use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

/// Error returned when [`Timeout`] times out
#[derive(Debug)]
pub struct TimedOutError;

impl Error for TimedOutError {}

impl fmt::Display for TimedOutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "timed out")
    }
}

pin_project! {
    /// Timeout Future
    #[non_exhaustive]
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    #[derive(Debug)]
    pub struct Timeout<T, S> {
        #[pin]
        value: T,
        #[pin]
        sleep: S,
    }
}

impl<T, S> Timeout<T, S> {
    /// Create a new future that will race `value` and `sleep`.
    ///
    /// If `sleep` resolves first, a timeout error is returned. Otherwise, the value is returned.
    pub fn new(value: T, sleep: S) -> Timeout<T, S> {
        Timeout { value, sleep }
    }
}

impl<T, S> Future for Timeout<T, S>
where
    T: Future,
    S: Future,
{
    type Output = Result<T::Output, TimedOutError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let me = self.project();

        // First, try polling the future
        if let Poll::Ready(v) = me.value.poll(cx) {
            return Poll::Ready(Ok(v));
        }

        // Now check the timer
        match me.sleep.poll(cx) {
            Poll::Ready(_) => Poll::Ready(Err(TimedOutError)),
            Poll::Pending => Poll::Pending,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{TimedOutError, Timeout};
    use crate::future::never::Never;

    #[tokio::test]
    async fn success() {
        assert!(matches!(
            Timeout::new(async { Ok::<isize, isize>(5) }, Never).await,
            Ok(Ok(5))
        ));
    }

    #[tokio::test]
    async fn failure() {
        assert!(matches!(
            Timeout::new(async { Err::<isize, isize>(0) }, Never).await,
            Ok(Err(0))
        ));
    }

    #[tokio::test]
    async fn timeout() {
        assert!(matches!(
            Timeout::new(Never, async {}).await,
            Err(TimedOutError)
        ));
    }

    // If the value is available at the same time as the timeout, then return the value
    #[tokio::test]
    async fn prefer_value_to_timeout() {
        assert!(matches!(Timeout::new(async { 5 }, async {}).await, Ok(5)));
    }
}
