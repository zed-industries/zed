/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

#![cfg(all(
    feature = "client",
    any(feature = "test-util", feature = "legacy-test-util")
))]

#[macro_use]
mod stalled_stream_common;

use aws_smithy_runtime_api::client::stalled_stream_protection::DEFAULT_GRACE_PERIOD;
use stalled_stream_common::*;

/// Scenario: Successful upload at a rate above the minimum throughput.
/// Expected: MUST NOT timeout.
#[tokio::test]
async fn upload_success() {
    let _logs = show_test_logs();

    let (server, time, sleep) = eager_server(true);
    let op = operation(server, time, sleep);

    let (body, body_sender) = channel_body();
    let result = tokio::spawn(async move { op.invoke(body).await });

    for _ in 0..100 {
        body_sender.send(NEAT_DATA).await.unwrap();
    }
    drop(body_sender);

    assert_eq!(200, result.await.unwrap().expect("success").as_u16());
}

/// Scenario: Upload takes some time to start, but then goes normally.
/// Expected: MUST NOT timeout.
#[tokio::test]
async fn upload_slow_start() {
    let _logs = show_test_logs();

    let (server, time, sleep) = eager_server(false);
    let op = operation(server, time.clone(), sleep);

    let (body, body_sender) = channel_body();
    let result = tokio::spawn(async move { op.invoke(body).await });

    let _streamer = tokio::spawn(async move {
        // Advance longer than the grace period. This shouldn't fail since
        // it is the customer's side that hasn't produced data yet, not a server issue.
        time.tick(Duration::from_secs(10)).await;

        for _ in 0..100 {
            body_sender.send(NEAT_DATA).await.unwrap();
            time.tick(Duration::from_secs(1)).await;
        }
        drop(body_sender);
        time.tick(Duration::from_secs(1)).await;
    });

    assert_eq!(200, result.await.unwrap().expect("success").as_u16());
}

/// Scenario: The upload is going fine, but falls below the minimum throughput.
/// Expected: MUST timeout.
#[tokio::test]
async fn upload_too_slow() {
    let _logs = show_test_logs();

    // Server that starts off fast enough, but gets slower over time until it should timeout.
    let (server, time, sleep) = time_sequence_server([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    let op = operation(server, time, sleep);

    let (body, body_sender) = channel_body();
    let result = tokio::spawn(async move { op.invoke(body).await });

    let _streamer = tokio::spawn(async move {
        for send in 0..100 {
            info!("send {send}");
            body_sender.send(NEAT_DATA).await.unwrap();
        }
        drop(body_sender);
    });

    expect_timeout(result.await.expect("no panics"));
}

/// Scenario: The server stops asking for data, the client maxes out its send buffer,
///           and the request stream stops being polled.
/// Expected: MUST timeout after the grace period completes.
#[tokio::test]
async fn upload_stalls() {
    let _logs = show_test_logs();

    let (server, time, sleep) = stalling_server(None);
    let op = operation(server, time.clone(), sleep);

    let (body, body_sender) = channel_body();
    let result = tokio::spawn(async move { op.invoke(body).await });

    let _streamer = tokio::spawn(async move {
        for send in 1..=100 {
            info!("send {send}");
            body_sender.send(NEAT_DATA).await.unwrap();
            tick!(time, Duration::from_secs(1));
        }
        drop(body_sender);
        time.tick(Duration::from_secs(1)).await;
    });

    expect_timeout(result.await.expect("no panics"));
}

/// Scenario: Request does not have a body. Server response doesn't start coming through
///           until after the grace period.
/// Expected: MUST NOT timeout.
#[tokio::test]
async fn empty_request_body_delayed_response() {
    let _logs = show_test_logs();

    let (server, time, sleep) = stalling_server(Some(Duration::from_secs(6)));
    let op = operation(server, time.clone(), sleep);

    let result = tokio::spawn(async move { op.invoke(SdkBody::empty()).await });

    let _advance = tokio::spawn(async move {
        for _ in 0..6 {
            tick!(time, Duration::from_secs(1));
        }
    });

    assert_eq!(200, result.await.unwrap().expect("success").as_u16());
}

/// Scenario: All the request data is either uploaded to the server or buffered in the
///           HTTP client, but the response doesn't start coming through within the grace period.
/// Expected: MUST NOT timeout, upload throughput should only apply up until the request body has
/// been read completely and handed off to the HTTP client.
#[tokio::test]
async fn complete_upload_delayed_response() {
    let _logs = show_test_logs();

    let (server, time, sleep) = stalling_server(Some(Duration::from_secs(6)));
    let op = operation(server, time.clone(), sleep);

    let (body, body_sender) = channel_body();
    let result = tokio::spawn(async move { op.invoke(body).await });

    let _streamer = tokio::spawn(async move {
        info!("send data");
        body_sender.send(NEAT_DATA).await.unwrap();
        tick!(time, Duration::from_secs(1));
        info!("body send complete; dropping");
        drop(body_sender);
        tick!(time, DEFAULT_GRACE_PERIOD);
        info!("body stream task complete");
        // advance to unblock the stalled server
        tick!(time, Duration::from_secs(2));
    });

    assert_eq!(200, result.await.unwrap().expect("success").as_u16());
}

/// Scenario: Upload all request data and never poll again once content-length has
///           been reached. Hyper will stop polling once it detects end of stream so we can't rely
///           on reaching `Poll:Ready(None)` to detect end of stream.
///
///           ref: https://github.com/hyperium/hyper/issues/1545
///           ref: https://github.com/hyperium/hyper/issues/1521
///
/// Expected: MUST NOT timeout, upload throughput should only apply up until the request body has
/// been read completely. Once no more data is expected we should stop checking for throughput
/// violations.
#[tokio::test]
async fn complete_upload_stop_polling() {
    let _logs = show_test_logs();

    let (server, time, sleep) = limited_read_server(NEAT_DATA.len(), Some(Duration::from_secs(7)));
    let op = operation(server, time.clone(), sleep.clone());

    let body = SdkBody::from(NEAT_DATA);
    let result = tokio::spawn(async move { op.invoke(body).await });

    tokio::spawn(async move {
        // advance past the grace period
        tick!(time, DEFAULT_GRACE_PERIOD + Duration::from_secs(1));
        // unblock server
        tick!(time, Duration::from_secs(2));
    });

    assert_eq!(200, result.await.unwrap().expect("success").as_u16());
}

// Scenario: The server stops asking for data, the client maxes out its send buffer,
//           and the request stream stops being polled. However, before the grace period
//           is over, the server recovers and starts asking for data again.
// Expected: MUST NOT timeout.
#[tokio::test]
async fn upload_stall_recovery_in_grace_period() {
    let _logs = show_test_logs();

    // Server starts off fast enough, but then slows down almost up to
    // the grace period, and then recovers.
    let (server, time, sleep) = time_sequence_server([1, 4, 1]);
    let op = operation(server, time, sleep);

    let (body, body_sender) = channel_body();
    let result = tokio::spawn(async move { op.invoke(body).await });

    let _streamer = tokio::spawn(async move {
        for send in 0..100 {
            info!("send {send}");
            body_sender.send(NEAT_DATA).await.unwrap();
        }
        drop(body_sender);
    });

    assert_eq!(200, result.await.unwrap().expect("success").as_u16());
}

// Scenario: The customer isn't providing data on the stream fast enough to satisfy
//           the minimum throughput. This shouldn't be considered a stall since the
//           server is asking for more data and could handle it if it were available.
// Expected: MUST NOT timeout.
#[tokio::test]
async fn user_provides_data_too_slowly() {
    let _logs = show_test_logs();

    let (server, time, sleep) = eager_server(false);
    let op = operation(server, time.clone(), sleep.clone());

    let (body, body_sender) = channel_body();
    let result = tokio::spawn(async move { op.invoke(body).await });

    let _streamer = tokio::spawn(async move {
        body_sender.send(NEAT_DATA).await.unwrap();
        tick!(time, Duration::from_secs(1));
        body_sender.send(NEAT_DATA).await.unwrap();

        // Now advance 10 seconds before sending more data, simulating a
        // customer taking time to produce more data to stream.
        tick!(time, Duration::from_secs(10));
        body_sender.send(NEAT_DATA).await.unwrap();
        drop(body_sender);
        tick!(time, Duration::from_secs(1));
    });

    assert_eq!(200, result.await.unwrap().expect("success").as_u16());
}

use upload_test_tools::*;
mod upload_test_tools {
    use crate::stalled_stream_common::*;
    use aws_smithy_async::rt::sleep::AsyncSleep;
    use http_body_1x::Body;

    pub fn successful_response() -> HttpResponse {
        HttpResponse::try_from(
            http_02x::Response::builder()
                .status(200)
                .body(SdkBody::empty())
                .unwrap(),
        )
        .unwrap()
    }

    pub fn operation(
        http_connector: impl HttpConnector + 'static,
        time: TickAdvanceTime,
        sleep: TickAdvanceSleep,
    ) -> Operation<SdkBody, StatusCode, Infallible> {
        Operation::builder()
            .service_name("test")
            .operation_name("test")
            .http_client(FakeServer(http_connector.into_shared()))
            .endpoint_url("http://localhost:1234/doesntmatter")
            .no_auth()
            .no_retry()
            .timeout_config(TimeoutConfig::disabled())
            .serializer(|body: SdkBody| Ok(HttpRequest::new(body)))
            .deserializer::<_, Infallible>(|response| Ok(response.status()))
            .stalled_stream_protection(
                StalledStreamProtectionConfig::enabled()
                    .grace_period(Duration::from_secs(5))
                    .build(),
            )
            .interceptor(StalledStreamProtectionInterceptor::default())
            .sleep_impl(sleep)
            .time_source(time)
            .build()
    }

    /// Creates a fake HttpConnector implementation that calls the given async $body_fn
    /// to get the response body. This $body_fn is given a request body, time, and sleep.
    macro_rules! fake_server {
        ($name:ident, $body_fn:expr) => {
            fake_server!($name, $body_fn, (), ())
        };
        ($name:ident, $body_fn:expr, $params_ty:ty, $params:expr) => {{
            #[derive(Debug)]
            struct $name(TickAdvanceTime, TickAdvanceSleep, $params_ty);
            impl HttpConnector for $name {
                fn call(&self, mut request: HttpRequest) -> HttpConnectorFuture {
                    let time = self.0.clone();
                    let sleep = self.1.clone();
                    let params = self.2.clone();
                    let span = tracing::span!(tracing::Level::INFO, "FAKE SERVER");
                    HttpConnectorFuture::new(
                        async move {
                            let mut body = SdkBody::taken();
                            mem::swap(request.body_mut(), &mut body);
                            pin_mut!(body);

                            Ok($body_fn(body, time, sleep, params).await)
                        }
                        .instrument(span),
                    )
                }
            }
            let (time, sleep) = tick_advance_time_and_sleep();
            (
                $name(time.clone(), sleep.clone(), $params).into_shared(),
                time,
                sleep,
            )
        }};
    }

    /// Fake server/connector that immediately reads all incoming data with an
    /// optional 1 second gap in between polls.
    pub fn eager_server(
        advance_time: bool,
    ) -> (SharedHttpConnector, TickAdvanceTime, TickAdvanceSleep) {
        async fn fake_server(
            mut body: Pin<&mut SdkBody>,
            time: TickAdvanceTime,
            _: TickAdvanceSleep,
            advance_time: bool,
        ) -> HttpResponse {
            while poll_fn(|cx| body.as_mut().poll_frame(cx)).await.is_some() {
                if advance_time {
                    tick!(time, Duration::from_secs(1));
                }
            }
            successful_response()
        }
        fake_server!(FakeServerConnector, fake_server, bool, advance_time)
    }

    /// Fake server/connector that reads some data, and then stalls for the given time before
    /// returning a response. If `None` is given the server will stall indefinitely.
    pub fn stalling_server(
        respond_after: Option<Duration>,
    ) -> (SharedHttpConnector, TickAdvanceTime, TickAdvanceSleep) {
        async fn fake_server(
            mut body: Pin<&mut SdkBody>,
            _time: TickAdvanceTime,
            sleep: TickAdvanceSleep,
            respond_after: Option<Duration>,
        ) -> HttpResponse {
            let mut times = 5;
            while times > 0 && poll_fn(|cx| body.as_mut().poll_frame(cx)).await.is_some() {
                times -= 1;
            }

            match respond_after {
                Some(delay) => {
                    tracing::info!("stalling for {} seconds", delay.as_secs());
                    sleep.sleep(delay).await;
                    tracing::info!("returning delayed response");
                    successful_response()
                }
                None => {
                    // never awake after this
                    tracing::info!("stalling indefinitely");
                    std::future::pending::<()>().await;
                    unreachable!()
                }
            }
        }
        fake_server!(
            FakeServerConnector,
            fake_server,
            Option<Duration>,
            respond_after
        )
    }

    /// Fake server/connector that polls data after each period of time in the given
    /// sequence. Once the sequence completes, it will delay 1 second after each poll.
    pub fn time_sequence_server(
        time_sequence: impl IntoIterator<Item = u64>,
    ) -> (SharedHttpConnector, TickAdvanceTime, TickAdvanceSleep) {
        async fn fake_server(
            mut body: Pin<&mut SdkBody>,
            time: TickAdvanceTime,
            _sleep: TickAdvanceSleep,
            time_sequence: Vec<u64>,
        ) -> HttpResponse {
            let mut time_sequence: VecDeque<Duration> =
                time_sequence.into_iter().map(Duration::from_secs).collect();
            while poll_fn(|cx| body.as_mut().poll_frame(cx)).await.is_some() {
                let next_time = time_sequence.pop_front().unwrap_or(Duration::from_secs(1));
                tick!(time, next_time);
            }
            successful_response()
        }
        fake_server!(
            FakeServerConnector,
            fake_server,
            Vec<u64>,
            time_sequence.into_iter().collect()
        )
    }

    /// Fake server/connector that polls data only up to the content-length. Optionally delays
    /// sending the response by the given duration.
    pub fn limited_read_server(
        content_len: usize,
        respond_after: Option<Duration>,
    ) -> (SharedHttpConnector, TickAdvanceTime, TickAdvanceSleep) {
        async fn fake_server(
            mut body: Pin<&mut SdkBody>,
            _time: TickAdvanceTime,
            sleep: TickAdvanceSleep,
            params: (usize, Option<Duration>),
        ) -> HttpResponse {
            let mut remaining = params.0;
            loop {
                match poll_fn(|cx| body.as_mut().poll_frame(cx)).await {
                    Some(res) => {
                        let rc = res.unwrap().into_data().expect("data frame").len();
                        remaining -= rc;
                        tracing::info!("read {rc} bytes; remaining: {remaining}");
                        if remaining == 0 {
                            tracing::info!("read reported content-length data, stopping polling");
                            break;
                        };
                    }
                    None => {
                        tracing::info!(
                            "read until poll_frame() returned None, no data left, stopping polling"
                        );
                        break;
                    }
                }
            }

            let respond_after = params.1;
            if let Some(delay) = respond_after {
                tracing::info!("stalling for {} seconds", delay.as_secs());
                sleep.sleep(delay).await;
                tracing::info!("returning delayed response");
            }

            successful_response()
        }

        fake_server!(
            FakeServerConnector,
            fake_server,
            (usize, Option<Duration>),
            (content_len, respond_after)
        )
    }

    pub fn expect_timeout(result: Result<StatusCode, SdkError<Infallible, Response<SdkBody>>>) {
        let err = result.expect_err("should have timed out");
        assert_str_contains!(
            DisplayErrorContext(&err).to_string(),
            "minimum throughput was specified at 1 B/s, but throughput of 0 B/s was observed"
        );
    }
}
