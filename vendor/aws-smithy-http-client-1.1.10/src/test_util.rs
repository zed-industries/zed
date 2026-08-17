/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Various fake/mock clients for testing.
//!
//! Each test client is useful for different test use cases:
//! - [`capture_request()`]: If you don't care what the response is, but just want to
//!   check that the serialized request is what you expect, then use `capture_request`.
//!   Or, alternatively, if you don't care what the request is, but want to always
//!   respond with a given response, then capture request can also be useful since
//!   you can optionally give it a response to return.
#![cfg_attr(
    feature = "default-client",
    doc = "- [`dvr`]: If you want to record real-world traffic and then replay it later, then DVR's"
)]
//!   [`RecordingClient`](dvr::RecordingClient) and [`ReplayingClient`](dvr::ReplayingClient)
//!   can accomplish this, and the recorded traffic can be saved to JSON and checked in. Note: if
//!   the traffic recording has sensitive information in it, such as signatures or authorization,
//!   you will need to manually scrub this out if you intend to store the recording alongside
//!   your tests.
//! - [`StaticReplayClient`]: If you want to have a set list of requests and their responses in a test,
//!   then the static replay client will be useful. On construction, it takes a list of request/response
//!   pairs that represent each expected request and the response for that test. At the end of the test,
//!   you can ask the client to verify that the requests matched the expectations.
//! - [`infallible_client_fn`]: Allows you to create a client from an infallible function
//!   that takes a request and returns a response.
//! - [`NeverClient`]: Useful for testing timeouts, where you want the client to never respond.
//!
#![cfg_attr(
    any(feature = "hyper-014", feature = "default-client"),
    doc = "
There is also the [`NeverTcpConnector`], which makes it easy to test connect/read timeouts.

Finally, for socket-level mocking, see the [`wire`] module.
"
)]

mod capture_request;
pub use capture_request::{capture_request, CaptureRequestHandler, CaptureRequestReceiver};

#[cfg(feature = "legacy-test-util")]
pub use capture_request::legacy_capture_request;

pub mod dvr;

mod replay;
pub use replay::{ReplayEvent, StaticReplayClient};

mod infallible;
pub use infallible::infallible_client_fn;

// infallible based on http_02x stack had to be duplicated to avoid breaking API changes
#[allow(missing_docs)]
#[cfg(feature = "legacy-test-util")]
pub mod legacy_infallible;

mod never;
pub use never::NeverClient;

#[cfg(any(feature = "hyper-014", feature = "default-client"))]
pub use never::NeverTcpConnector;

mod body;
#[cfg(all(feature = "default-client", feature = "wire-mock"))]
pub mod wire;
