/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_smithy_runtime_api::box_error::BoxError;
use aws_smithy_types::body::SdkBody;
use bytes::Bytes;
use http_body_1x::{Frame, SizeHint};
use pin_project_lite::pin_project;
use std::future::poll_fn;
use std::pin::{pin, Pin};
use std::task::{Context, Poll};
use tokio::sync::mpsc;

/// Create a `SdkBody` with an associated sender half.
///
/// Useful for sending data from another thread/task and test scenarios.
pub(crate) fn channel_body() -> (Sender, SdkBody) {
    let (tx, rx) = mpsc::channel(1);
    let sender = Sender { tx };
    let ch_body = ChannelBody { rx };
    (sender, SdkBody::from_body_1_x(ch_body))
}

/// Sender half of channel based `SdkBody` implementation useful for testing.
///
/// Roughly a replacement for hyper 0.14.x `Sender` body.
///
/// ## Body Closing
///
/// The request body will always be closed normally when the sender is dropped. If you
/// want to close the connection with an incomplete response, call [`Sender::abort()`] method to
/// abort the body in an abnormal fashion.
#[derive(Debug)]
pub(crate) struct Sender {
    tx: mpsc::Sender<Result<Frame<Bytes>, BoxError>>,
}

impl Sender {
    /// Send data on data channel when it's ready
    pub(crate) async fn send_data(&mut self, chunk: Bytes) -> Result<(), BoxError> {
        let frame = Frame::data(chunk);
        self.tx.send(Ok(frame)).await.map_err(|e| e.into())
    }

    // TODO(test-utils): we can add support for trailers if needed in the future

    /// Abort the body in an abnormal fashion
    pub(crate) fn abort(self) {
        let _ = self.tx.clone().try_send(Err("body write aborted".into()));
    }
}

pin_project! {
    struct ChannelBody {
        rx: mpsc::Receiver<Result<Frame<Bytes>, BoxError>>
    }
}

impl http_body_1x::Body for ChannelBody {
    type Data = Bytes;
    type Error = BoxError;

    fn poll_frame(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        let this = self.project();
        this.rx.poll_recv(cx)
    }

    fn is_end_stream(&self) -> bool {
        self.rx.is_closed()
    }

    fn size_hint(&self) -> SizeHint {
        SizeHint::default()
    }
}

pub(crate) async fn next_data_frame(body: &mut SdkBody) -> Option<Result<Bytes, BoxError>> {
    use http_body_1x::Body;
    let mut pinned = pin!(body);
    match poll_fn(|cx| pinned.as_mut().poll_frame(cx)).await? {
        Ok(frame) => {
            if frame.is_data() {
                Some(Ok(frame.into_data().unwrap()))
            } else {
                None
            }
        }
        Err(err) => Some(Err(err)),
    }
}
