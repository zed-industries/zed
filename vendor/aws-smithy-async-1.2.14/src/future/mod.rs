/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Useful runtime-agnostic future implementations.

use futures_util::Future;
use std::pin::Pin;

pub mod never;
pub mod now_or_later;
pub mod pagination_stream;
pub mod rendezvous;
pub mod timeout;

/// A boxed future that outputs a `Result<T, E>`.
pub type BoxFuture<'a, T, E> = Pin<Box<dyn Future<Output = Result<T, E>> + Send + 'a>>;
