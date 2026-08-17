/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

#![cfg(all(
    feature = "client",
    any(feature = "test-util", feature = "legacy-test-util")
))]
// Extra imports are used by stalled_stream_download and stalled_stream_upload as conveniences
#![allow(unused_imports)]

pub use aws_smithy_async::{
    test_util::tick_advance_sleep::{
        tick_advance_time_and_sleep, TickAdvanceSleep, TickAdvanceTime,
    },
    time::TimeSource,
};
pub use aws_smithy_runtime::{
    assert_str_contains,
    client::{
        orchestrator::operation::Operation,
        stalled_stream_protection::StalledStreamProtectionInterceptor,
    },
    test_util::capture_test_logs::show_test_logs,
};
pub use aws_smithy_runtime_api::{
    box_error::BoxError,
    client::{
        http::{
            HttpClient, HttpConnector, HttpConnectorFuture, HttpConnectorSettings,
            SharedHttpConnector,
        },
        interceptors::context::{Error, Output},
        orchestrator::{HttpRequest, HttpResponse, OrchestratorError},
        result::SdkError,
        runtime_components::RuntimeComponents,
        ser_de::DeserializeResponse,
        stalled_stream_protection::StalledStreamProtectionConfig,
    },
    http::{Response, StatusCode},
    shared::IntoShared,
};
pub use aws_smithy_types::{
    body::SdkBody, error::display::DisplayErrorContext, timeout::TimeoutConfig,
};
pub use bytes::Bytes;
pub use pin_utils::pin_mut;
pub use std::{
    collections::VecDeque,
    convert::Infallible,
    future::poll_fn,
    mem,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll},
    time::Duration,
};
pub use tracing::{info, Instrument as _};

/// No really, it's 42 bytes long... super neat
pub const NEAT_DATA: Bytes = Bytes::from_static(b"some really neat data");

/// Ticks time forward by the given duration, and logs the current time for debugging.
#[macro_export]
macro_rules! tick {
    ($ticker:ident, $duration:expr) => {
        $ticker.tick($duration).await;
        let now = $ticker
            .now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap();
        tracing::info!("ticked {:?}, now at {:?}", $duration, now);
    };
}

#[derive(Debug)]
pub struct FakeServer(pub SharedHttpConnector);
impl HttpClient for FakeServer {
    fn http_connector(
        &self,
        _settings: &HttpConnectorSettings,
        _components: &RuntimeComponents,
    ) -> SharedHttpConnector {
        self.0.clone()
    }
}

struct ChannelBody {
    receiver: tokio::sync::mpsc::Receiver<Bytes>,
}

impl http_body_1x::Body for ChannelBody {
    type Data = Bytes;
    type Error = Infallible;

    fn poll_frame(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<http_body_1x::Frame<Self::Data>, Self::Error>>> {
        match self.receiver.poll_recv(cx) {
            Poll::Ready(value) => Poll::Ready(value.map(|b| Ok(http_body_1x::Frame::data(b)))),
            Poll::Pending => Poll::Pending,
        }
    }
}

pub fn channel_body() -> (SdkBody, tokio::sync::mpsc::Sender<Bytes>) {
    let (sender, receiver) = tokio::sync::mpsc::channel(1000);
    (SdkBody::from_body_1_x(ChannelBody { receiver }), sender)
}
