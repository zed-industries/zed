/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Provides the [`Never`] future that never completes.

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

/// Future that never completes.
#[non_exhaustive]
#[derive(Default, Debug)]
pub struct Never;

impl Never {
    /// Create a new `Never` future that never resolves
    pub fn new() -> Never {
        Default::default()
    }
}

impl Future for Never {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Pending
    }
}
