//! Regression tests for incoming transport closure.

use std::{
    future, io,
    panic::{RefUnwindSafe, UnwindSafe},
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use agent_client_protocol::{
    ByteStreams, Channel, ConnectTo, ConnectionTo, Dispatch, Error, Handled, JsonRpcMessage,
    JsonRpcRequest, Lines, RawJsonRpcMessage, TransportFrame, UntypedMessage,
    is_incoming_transport_closed,
    role::{Role, UntypedRole},
    schema::v1::{RequestId, Response},
};
use agent_client_protocol_test::{MyRequest, MyResponse};
use futures::{FutureExt as _, SinkExt as _, StreamExt as _, future::join, stream};
use tokio::io::{AsyncBufReadExt as _, AsyncWriteExt as _};
use tokio_util::compat::{TokioAsyncReadCompatExt as _, TokioAsyncWriteCompatExt as _};

const TIMEOUT: Duration = Duration::from_secs(2);

#[test]
fn builder_auto_traits_follow_the_close_handler() {
    struct SendOnlyCallbackState {
        changed: std::cell::Cell<bool>,
        unwind_sensitive: std::marker::PhantomData<&'static mut ()>,
    }

    impl SendOnlyCallbackState {
        fn mark_changed(self) {
            self.changed.set(true);
        }
    }

    fn assert_released_auto_traits<T: Send + Sync + UnwindSafe + RefUnwindSafe>(_: &T) {}
    fn assert_send<T: Send>(_: &T) {}

    assert_released_auto_traits(&UntypedRole.builder());
    assert_released_auto_traits(&UntypedRole.builder().on_close(|_cx| async { Ok(()) }));

    // `Cell` keeps the callback from being `Sync` or `RefUnwindSafe`, while
    // the mutable-reference marker keeps it from being `UnwindSafe`. None of
    // those bounds are necessary because the callback is consumed once.
    let callback_state = SendOnlyCallbackState {
        changed: std::cell::Cell::new(false),
        unwind_sensitive: std::marker::PhantomData,
    };
    let send_only_builder = UntypedRole.builder().on_close(move |_cx| async move {
        callback_state.mark_changed();
        Ok(())
    });

    // The callback only needs to be sent to the connection driver. Its other
    // auto traits are reflected by the builder instead of being required or
    // hidden behind synchronization.
    assert_send(&send_only_builder);
}

struct DelegatingTransport<T>(T);
struct ImmediateClient;

struct PendingTransport {
    started: Option<futures::channel::oneshot::Sender<()>>,
    dropped: Arc<AtomicBool>,
}

impl Drop for PendingTransport {
    fn drop(&mut self) {
        self.dropped.store(true, Ordering::Release);
    }
}

impl<R: Role> ConnectTo<R> for PendingTransport {
    async fn connect_to(mut self, client: impl ConnectTo<R::Counterpart>) -> Result<(), Error> {
        if let Some(started) = self.started.take() {
            let _ = started.send(());
        }

        future::pending::<()>().await;
        drop((self, client));
        Ok(())
    }
}

struct QueuedClient {
    started: futures::channel::oneshot::Sender<()>,
    escaped:
        futures::channel::oneshot::Sender<futures::channel::mpsc::UnboundedSender<TransportFrame>>,
}

impl ConnectTo<UntypedRole> for QueuedClient {
    async fn connect_to(self, transport: impl ConnectTo<UntypedRole>) -> Result<(), Error> {
        let (channel, transport_future) = transport.into_channel_and_future();
        let message = RawJsonRpcMessage::notification(
            "queued".into(),
            serde_json::json!({ "payload": "x".repeat(1024) }),
        )?;
        channel
            .tx
            .unbounded_send(TransportFrame::Single(message))
            .map_err(Error::into_internal_error)?;
        drop(self.escaped.send(channel.tx.clone()));
        let _ = self.started.send(());
        drop(channel);
        transport_future.await
    }
}

#[derive(Clone)]
struct BlockingRequest {
    entered: Arc<Mutex<Option<futures::channel::oneshot::Sender<()>>>>,
    release: Arc<Mutex<std::sync::mpsc::Receiver<()>>>,
}

impl std::fmt::Debug for BlockingRequest {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("BlockingRequest").finish()
    }
}

impl JsonRpcMessage for BlockingRequest {
    fn matches_method(method: &str) -> bool {
        method == "blockingRequest"
    }

    fn method(&self) -> &'static str {
        "blockingRequest"
    }

    fn to_untyped_message(&self) -> Result<UntypedMessage, Error> {
        if let Some(entered) = self.entered.lock().unwrap().take() {
            let _ = entered.send(());
        }
        let _ = self.release.lock().unwrap().recv();
        UntypedMessage::new(self.method(), serde_json::json!({}))
    }

    fn parse_message(_method: &str, _params: &impl serde::Serialize) -> Result<Self, Error> {
        Err(Error::method_not_found().data("test-only outgoing request"))
    }
}

impl JsonRpcRequest for BlockingRequest {
    type Response = serde_json::Value;
}

impl<R, T> ConnectTo<R> for DelegatingTransport<T>
where
    R: Role,
    T: ConnectTo<R>,
{
    async fn connect_to(self, client: impl ConnectTo<R::Counterpart>) -> Result<(), Error> {
        self.0.connect_to(client).await
    }
}

impl<R: Role> ConnectTo<R> for ImmediateClient {
    async fn connect_to(self, _client: impl ConnectTo<R::Counterpart>) -> Result<(), Error> {
        Ok(())
    }
}

fn assert_connection_closed(error: &Error, method: &str) {
    assert!(is_incoming_transport_closed(error));
    assert_eq!(error.message, "Incoming transport closed");
    let data = error.data.as_ref().expect("connection-close error data");
    assert_eq!(data["reason"], "incoming_transport_closed");
    assert_eq!(data["method"], method);
}

async fn receive_requests_then_close(mut peer: Channel, count: usize) {
    for _ in 0..count {
        assert!(matches!(
            peer.rx.next().await,
            Some(TransportFrame::Single(RawJsonRpcMessage::Request(_)))
        ));
    }
    drop(peer);
}

async fn respond_then_close(mut peer: Channel) {
    let Some(TransportFrame::Single(RawJsonRpcMessage::Request(request))) = peer.rx.next().await
    else {
        panic!("expected outgoing request");
    };
    peer.tx
        .send(TransportFrame::Single(RawJsonRpcMessage::response(
            request.id,
            Ok(serde_json::to_value(MyResponse {
                status: "received".into(),
            })
            .unwrap()),
        )))
        .await
        .expect("send response before EOF");
    drop(peer);
}

fn assert_response_channel_lost(error: &Error) {
    assert!(
        !is_incoming_transport_closed(error),
        "a response discarded locally must not be attributed to EOF"
    );
    assert!(
        error.data.as_ref().is_some_and(|data| data
            .to_string()
            .contains("response to `myRequest` never received")),
        "unexpected response-channel error: {error:?}"
    );
}

async fn assert_connect_to_flushes_final_response(
    transport: impl ConnectTo<UntypedRole>,
    mut peer_outgoing: tokio::io::DuplexStream,
    peer_incoming: tokio::io::DuplexStream,
) {
    let connection = UntypedRole
        .builder()
        .on_receive_request(
            async |_request: MyRequest, responder, _cx: ConnectionTo<UntypedRole>| {
                responder.respond(MyResponse {
                    status: "received".into(),
                })
            },
            agent_client_protocol::on_receive_request!(),
        )
        .connect_to(transport);

    let peer = async move {
        let request = RawJsonRpcMessage::request(
            "myRequest".into(),
            serde_json::json!({}),
            RequestId::Number(1),
        )
        .unwrap();
        let mut request_bytes = serde_json::to_vec(&request).unwrap();
        request_bytes.push(b'\n');
        peer_outgoing.write_all(&request_bytes).await.unwrap();

        // Half-close the peer's write direction while keeping its read
        // direction open for the final response.
        drop(peer_outgoing);

        let response_line = tokio::io::BufReader::new(peer_incoming)
            .lines()
            .next_line()
            .await
            .unwrap()
            .expect("peer should receive a complete response before EOF");
        let RawJsonRpcMessage::Response(Response::Result { id, result }) =
            serde_json::from_str(&response_line).expect("response must not be truncated")
        else {
            panic!("expected successful JSON-RPC response");
        };
        assert_eq!(id, RequestId::Number(1));
        assert_eq!(
            serde_json::from_value::<MyResponse>(result).unwrap().status,
            "received"
        );
    };

    tokio::time::timeout(TIMEOUT, async move {
        let (result, ()) = join(connection, peer).await;
        result
    })
    .await
    .expect("connection hung while draining its final response")
    .expect("clean EOF should succeed after flushing the response");
}

#[tokio::test]
async fn connect_to_returns_cleanly_on_incoming_eof_with_spawned_work() {
    let (transport, peer) = Channel::duplex();
    drop(peer);

    let result = tokio::time::timeout(
        TIMEOUT,
        UntypedRole
            .builder()
            .with_spawned(|_cx| future::pending::<Result<(), Error>>())
            .connect_to(transport),
    )
    .await
    .expect("connect_to hung after transport EOF");

    assert!(
        result.is_ok(),
        "clean EOF should stop connect_to: {result:?}"
    );
}

#[tokio::test]
async fn connect_to_flushes_response_queued_before_incoming_eof() {
    // A one-byte SDK output buffer forces the JSON response write to yield
    // repeatedly. Returning at EOF without a real transport drain truncates it.
    let (sdk_outgoing, peer_incoming) = tokio::io::duplex(1);
    let (peer_outgoing, sdk_incoming) = tokio::io::duplex(1024);
    let transport = ByteStreams::new(sdk_outgoing.compat_write(), sdk_incoming.compat());

    assert_connect_to_flushes_final_response(transport, peer_outgoing, peer_incoming).await;
}

#[tokio::test]
async fn channel_peer_receives_final_response_after_write_half_closes() {
    let (transport, peer) = Channel::duplex();
    let connection = UntypedRole
        .builder()
        .on_receive_request(
            async |_request: MyRequest, responder, _cx: ConnectionTo<UntypedRole>| {
                responder.respond(MyResponse {
                    status: "received".into(),
                })
            },
            agent_client_protocol::on_receive_request!(),
        )
        .connect_to(transport);

    let peer = async move {
        let Channel { mut rx, tx } = peer;
        tx.unbounded_send(TransportFrame::Single(
            RawJsonRpcMessage::request(
                "myRequest".into(),
                serde_json::json!({}),
                RequestId::Number(40),
            )
            .unwrap(),
        ))
        .expect("channel should accept the final request");
        tx.close_channel();
        drop(tx);

        let Some(TransportFrame::Single(RawJsonRpcMessage::Response(Response::Result {
            id,
            result,
        }))) = rx.next().await
        else {
            panic!("channel read half closed before the final response");
        };
        assert_eq!(id, RequestId::Number(40));
        assert_eq!(
            serde_json::from_value::<MyResponse>(result).unwrap().status,
            "received"
        );
    };

    tokio::time::timeout(TIMEOUT, async move {
        let (result, ()) = join(connection, peer).await;
        result
    })
    .await
    .expect("channel connection hung while draining its final response")
    .expect("channel connection failed after a write half-close");
}

#[tokio::test]
async fn connect_to_flushes_a_buffered_byte_writer() {
    let (sdk_outgoing, peer_incoming) = tokio::io::duplex(1);
    let (peer_outgoing, sdk_incoming) = tokio::io::duplex(1024);
    let buffered_outgoing =
        futures::io::BufWriter::with_capacity(4096, sdk_outgoing.compat_write());
    let transport = ByteStreams::new(buffered_outgoing, sdk_incoming.compat());

    assert_connect_to_flushes_final_response(transport, peer_outgoing, peer_incoming).await;
}

#[tokio::test]
async fn transport_channel_keeps_read_half_open_after_write_half_closes() {
    let (sdk_outgoing, peer_incoming) = tokio::io::duplex(1024);
    let (mut peer_outgoing, sdk_incoming) = tokio::io::duplex(1024);
    let transport = ByteStreams::new(sdk_outgoing.compat_write(), sdk_incoming.compat());
    let (channel, transport_future) = ConnectTo::<UntypedRole>::into_channel_and_future(transport);
    let Channel { mut rx, tx } = channel;

    tx.unbounded_send(TransportFrame::Single(
        RawJsonRpcMessage::request(
            "myRequest".into(),
            serde_json::json!({}),
            RequestId::Number(41),
        )
        .unwrap(),
    ))
    .expect("transport channel should accept the request");
    tx.close_channel();
    drop(tx);

    let peer = async move {
        let mut lines = tokio::io::BufReader::new(peer_incoming).lines();
        let request = lines
            .next_line()
            .await
            .unwrap()
            .expect("peer should receive the request before write EOF");
        assert!(matches!(
            serde_json::from_str::<RawJsonRpcMessage>(&request).unwrap(),
            RawJsonRpcMessage::Request(_)
        ));
        assert!(
            lines.next_line().await.unwrap().is_none(),
            "closing Channel.tx should close only the physical write half"
        );

        let response = RawJsonRpcMessage::response(
            RequestId::Number(41),
            Ok(serde_json::json!({ "status": "received" })),
        );
        let mut response_bytes = serde_json::to_vec(&response).unwrap();
        response_bytes.push(b'\n');
        peer_outgoing.write_all(&response_bytes).await.unwrap();
        drop(peer_outgoing);
    };

    let receive_response = async move {
        let Some(TransportFrame::Single(RawJsonRpcMessage::Response(Response::Result {
            id,
            result,
        }))) = rx.next().await
        else {
            panic!("read half closed before delivering the peer's final response");
        };
        assert_eq!(id, RequestId::Number(41));
        assert_eq!(result["status"], "received");
    };

    tokio::time::timeout(TIMEOUT, async move {
        let (transport_result, (), ()) = tokio::join!(transport_future, receive_response, peer);
        transport_result
    })
    .await
    .expect("transport channel hung after the peer's final response")
    .expect("transport channel failed while draining the final response");
}

#[tokio::test]
async fn wrapped_byte_streams_flush_response_queued_before_incoming_eof() {
    let (sdk_outgoing, peer_incoming) = tokio::io::duplex(1);
    let (peer_outgoing, sdk_incoming) = tokio::io::duplex(1024);
    let transport = DelegatingTransport(ByteStreams::new(
        sdk_outgoing.compat_write(),
        sdk_incoming.compat(),
    ));

    assert_connect_to_flushes_final_response(transport, peer_outgoing, peer_incoming).await;
}

#[tokio::test]
async fn wrapped_byte_streams_client_success_does_not_wait_for_peer_eof() {
    let (sdk_outgoing, peer_incoming) = tokio::io::duplex(1);
    let (peer_outgoing, sdk_incoming) = tokio::io::duplex(1024);
    let transport = DelegatingTransport(ByteStreams::new(
        sdk_outgoing.compat_write(),
        sdk_incoming.compat(),
    ));

    tokio::time::timeout(
        TIMEOUT,
        ConnectTo::<UntypedRole>::connect_to(transport, ImmediateClient),
    )
    .await
    .expect("successful client shutdown waited for unrelated peer EOF")
    .expect("successful client shutdown should drain outgoing and return");

    // Keep both peer halves open until after the assertion so the result
    // proves the transport's incoming actor was canceled, not completed.
    drop(peer_outgoing);
    drop(peer_incoming);
}

#[tokio::test]
async fn outgoing_drain_keeps_the_full_duplex_read_half_moving() {
    let (sdk_stream, peer_stream) = tokio::io::duplex(1);
    let (sdk_incoming, sdk_outgoing) = tokio::io::split(sdk_stream);
    let (peer_incoming, mut peer_outgoing) = tokio::io::split(peer_stream);
    let transport = ByteStreams::new(sdk_outgoing.compat_write(), sdk_incoming.compat());
    let (started_tx, started_rx) = futures::channel::oneshot::channel();
    let (escaped_tx, escaped_rx) = futures::channel::oneshot::channel();

    let connection = ConnectTo::<UntypedRole>::connect_to(
        transport,
        QueuedClient {
            started: started_tx,
            escaped: escaped_tx,
        },
    );
    let peer = async move {
        started_rx.await.expect("client should queue its message");
        let escaped = escaped_rx.await.expect("client should expose its sender");

        // Write two frames before reading. The first frame proves shutdown no
        // longer tries to forward into the client's closed channel; the second
        // proves the read half keeps draining after that point.
        for (method, payload_len) in [("first", 16), ("second", 1024)] {
            let message = RawJsonRpcMessage::notification(
                method.into(),
                serde_json::json!({ "payload": "x".repeat(payload_len) }),
            )
            .unwrap();
            let mut bytes = serde_json::to_vec(&message).unwrap();
            bytes.push(b'\n');
            peer_outgoing.write_all(&bytes).await.unwrap();
        }

        let line = tokio::io::BufReader::new(peer_incoming)
            .lines()
            .next_line()
            .await
            .unwrap()
            .expect("queued output should be drained");
        assert!(matches!(
            serde_json::from_str::<RawJsonRpcMessage>(&line).unwrap(),
            RawJsonRpcMessage::Notification(_)
        ));
        escaped
    };

    let escaped = tokio::time::timeout(TIMEOUT, async move {
        let (result, escaped) = join(connection, peer).await;
        result.map(|()| escaped)
    })
    .await
    .expect("full-duplex transport deadlocked while draining output")
    .expect("queued output should drain successfully");

    assert!(
        escaped
            .unbounded_send(TransportFrame::Single(
                RawJsonRpcMessage::notification("too-late".into(), serde_json::json!({}),).unwrap()
            ))
            .is_err(),
        "escaped sender accepted a message after client completion"
    );
}

#[tokio::test]
async fn outgoing_drain_propagates_incoming_read_error() {
    let (write_started_tx, write_started_rx) = futures::channel::oneshot::channel();
    let outgoing = futures::sink::unfold(
        Some(write_started_tx),
        |mut write_started, _line: String| async move {
            if let Some(write_started) = write_started.take() {
                let _ = write_started.send(());
            }
            future::pending::<Result<Option<futures::channel::oneshot::Sender<()>>, io::Error>>()
                .await
        },
    );
    let (incoming_tx, incoming) = futures::channel::mpsc::unbounded::<io::Result<String>>();
    let (client_started_tx, _client_started_rx) = futures::channel::oneshot::channel();
    let (escaped_tx, escaped_rx) = futures::channel::oneshot::channel();

    let connection = ConnectTo::<UntypedRole>::connect_to(
        Lines::new(outgoing, incoming),
        QueuedClient {
            started: client_started_tx,
            escaped: escaped_tx,
        },
    );
    let inject_error = async move {
        let escaped = escaped_rx
            .await
            .expect("client should expose its outgoing sender");
        write_started_rx
            .await
            .expect("queued output should reach the backpressured sink");
        assert!(
            escaped.is_closed(),
            "read error must be injected after the client enters drain mode"
        );
        incoming_tx
            .unbounded_send(Err(io::Error::other("read failed during outgoing drain")))
            .expect("incoming transport should still be polled during the drain");
    };

    let error = tokio::time::timeout(TIMEOUT, async move {
        let (result, ()) = join(connection, inject_error).await;
        result
    })
    .await
    .expect("incoming read error was swallowed behind the blocked outgoing drain")
    .expect_err("incoming read error should fail the connection");
    let detail = error
        .data
        .as_ref()
        .map(serde_json::Value::to_string)
        .unwrap_or_default();
    assert!(
        detail.contains("read failed during outgoing drain"),
        "unexpected transport error: {error:?}"
    );
}

#[tokio::test]
async fn clean_outgoing_drain_does_not_hide_ready_incoming_read_error() {
    let outgoing = futures::sink::unfold((), |(), _line: String| async { Ok::<_, io::Error>(()) });
    let incoming = stream::iter([Err(io::Error::other(
        "ready read failed during outgoing drain",
    ))]);

    let error = tokio::time::timeout(
        TIMEOUT,
        ConnectTo::<UntypedRole>::connect_to(Lines::new(outgoing, incoming), ImmediateClient),
    )
    .await
    .expect("connection should resolve when both drain sides are ready")
    .expect_err("ready incoming read error must win over a clean outgoing drain");
    let detail = error
        .data
        .as_ref()
        .map(serde_json::Value::to_string)
        .unwrap_or_default();
    assert!(
        detail.contains("ready read failed during outgoing drain"),
        "unexpected transport error: {error:?}"
    );
}

#[tokio::test]
async fn escaped_connection_handle_does_not_retain_the_transport() {
    let dropped = Arc::new(AtomicBool::new(false));
    let (started_tx, started_rx) = futures::channel::oneshot::channel();
    let (escaped_tx, escaped_rx) = futures::channel::oneshot::channel();

    let connection = UntypedRole.builder().connect_with(
        PendingTransport {
            started: Some(started_tx),
            dropped: dropped.clone(),
        },
        async move |cx| {
            started_rx
                .await
                .map_err(|_| Error::internal_error().data("transport never started"))?;
            let request = cx.send_request(MyRequest {});
            escaped_tx
                .send((cx, request))
                .map_err(|_| Error::internal_error().data("escaped handle receiver dropped"))?;
            Ok(())
        },
    );

    let (result, escaped) = tokio::time::timeout(TIMEOUT, join(connection, escaped_rx))
        .await
        .expect("connection did not stop after its foreground returned");
    result.expect("foreground shutdown should succeed");
    let (escaped, request) = escaped.expect("foreground should return connection state");

    assert!(
        dropped.load(Ordering::Acquire),
        "an escaped handle kept the transport future alive"
    );
    let error = tokio::time::timeout(TIMEOUT, request.block_task())
        .await
        .expect("escaped handle retained the pending-reply registry")
        .expect_err("request should fail when its connection driver stops");
    assert_response_channel_lost(&error);
    drop(escaped);
}

#[tokio::test]
async fn connect_with_can_await_incoming_close() {
    let (transport, peer) = Channel::duplex();
    drop(peer);

    tokio::time::timeout(
        TIMEOUT,
        UntypedRole.builder().connect_with(transport, async |cx| {
            cx.incoming_closed().await;
            assert!(cx.is_incoming_closed());
            Ok(())
        }),
    )
    .await
    .expect("incoming close signal was not delivered")
    .expect("observing a clean close should succeed");
}

#[tokio::test]
async fn on_close_can_borrow_from_the_connect_with_caller() {
    let (transport, peer) = Channel::duplex();
    drop(peer);

    let mut observed = false;
    let observed_by_callback = &mut observed;

    tokio::time::timeout(
        TIMEOUT,
        UntypedRole
            .builder()
            .on_close(move |_cx| async move {
                *observed_by_callback = true;
                Ok(())
            })
            .connect_with(transport, async |cx| {
                cx.incoming_closed().await;
                Ok(())
            }),
    )
    .await
    .expect("incoming close callback was not run")
    .expect("borrowed close callback should succeed");

    assert!(observed);
}

#[tokio::test]
async fn on_close_error_stops_a_pending_connect_with_foreground() {
    let (transport, peer) = Channel::duplex();
    drop(peer);

    let result = tokio::time::timeout(
        TIMEOUT,
        UntypedRole
            .builder()
            .on_close(async |_cx| Err(Error::internal_error().data("stop on transport close")))
            .connect_with(transport, async |_cx| {
                future::pending::<Result<(), Error>>().await
            }),
    )
    .await
    .expect("on_close did not stop connect_with");

    assert!(result.is_err(), "on_close error should stop the connection");
}

#[tokio::test]
async fn on_close_error_wins_over_foreground_close_waiter() {
    let (transport, peer) = Channel::duplex();
    drop(peer);

    let result = tokio::time::timeout(
        TIMEOUT,
        UntypedRole
            .builder()
            .on_close(async |_cx| Err(Error::internal_error().data("close callback failed")))
            .connect_with(transport, async |cx| {
                cx.incoming_closed().await;
                Ok(())
            }),
    )
    .await
    .expect("connection hung after close callback error");

    assert!(
        result.is_err(),
        "close callback error was swallowed by foreground close waiter"
    );
}

#[tokio::test]
async fn successful_on_close_does_not_cancel_unrelated_foreground_work() {
    let (transport, peer) = Channel::duplex();
    let (closed_tx, closed_rx) = futures::channel::oneshot::channel();
    let (release_tx, release_rx) = futures::channel::oneshot::channel();

    let connection = UntypedRole
        .builder()
        .on_close(async move |_cx| {
            closed_tx
                .send(())
                .map_err(|()| Error::internal_error().data("close observer dropped"))
        })
        .connect_with(transport, async move |_cx| {
            release_rx
                .await
                .map_err(|_| Error::internal_error().data("foreground release dropped"))
        });
    let connection = tokio::spawn(connection);

    drop(peer);
    tokio::time::timeout(TIMEOUT, closed_rx)
        .await
        .expect("on_close did not run")
        .expect("on_close sender dropped");
    tokio::task::yield_now().await;
    assert!(
        !connection.is_finished(),
        "clean EOF should not cancel unrelated connect_with work"
    );

    release_tx
        .send(())
        .expect("connection still receives release");
    connection
        .await
        .expect("connection task panicked")
        .expect("foreground release should end the connection cleanly");
}

#[tokio::test]
async fn all_on_close_callbacks_run_in_order_and_the_first_error_wins() {
    let (transport, peer) = Channel::duplex();
    let order = Arc::new(Mutex::new(Vec::new()));
    let first_order = order.clone();
    let second_order = order.clone();

    let connection = UntypedRole
        .builder()
        .on_close(async move |_cx| {
            first_order.lock().unwrap().push(1);
            Err(Error::internal_error().data("first close callback failed"))
        })
        .on_close(async move |_cx| {
            second_order.lock().unwrap().push(2);
            Err(Error::internal_error().data("second close callback failed"))
        })
        .connect_with(transport, async |_cx| {
            future::pending::<Result<(), Error>>().await
        });

    drop(peer);
    let error = tokio::time::timeout(TIMEOUT, connection)
        .await
        .expect("close callback error did not stop connection")
        .expect_err("close callback error should stop connection");
    assert_eq!(
        error.data,
        Some(serde_json::json!("first close callback failed"))
    );
    assert_eq!(*order.lock().unwrap(), vec![1, 2]);
}

#[tokio::test]
async fn merged_builders_preserve_close_callback_order() {
    let (transport, peer) = Channel::duplex();
    let order = Arc::new(Mutex::new(Vec::new()));
    let first_order = order.clone();
    let second_order = order.clone();

    let first = UntypedRole.builder().on_close(async move |_cx| {
        first_order.lock().unwrap().push(1);
        Err(Error::internal_error().data("first builder close failed"))
    });
    let second = UntypedRole.builder().on_close(async move |_cx| {
        second_order.lock().unwrap().push(2);
        Ok(())
    });
    let connection = first
        .with_connection_builder(second)
        .connect_with(transport, async |_cx| {
            future::pending::<Result<(), Error>>().await
        });

    drop(peer);
    let error = tokio::time::timeout(TIMEOUT, connection)
        .await
        .expect("merged close callback error did not stop connection")
        .expect_err("merged close callback error should stop connection");
    assert_eq!(
        error.data,
        Some(serde_json::json!("first builder close failed"))
    );
    assert_eq!(*order.lock().unwrap(), vec![1, 2]);
}

#[tokio::test]
async fn pending_requests_fail_before_on_close_finishes() {
    let (transport, peer) = Channel::duplex();
    let (release_callback_tx, release_callback_rx) = futures::channel::oneshot::channel();
    let (request_failed_tx, request_failed_rx) = futures::channel::oneshot::channel();

    let connection = UntypedRole
        .builder()
        .on_close(async move |_cx| {
            release_callback_rx
                .await
                .map_err(|_| Error::internal_error().data("close callback release dropped"))
        })
        .connect_with(transport, async move |cx| {
            let error = cx
                .send_request(MyRequest {})
                .block_task()
                .await
                .expect_err("request should fail at EOF");
            assert_connection_closed(&error, "myRequest");
            request_failed_tx
                .send(())
                .map_err(|()| Error::internal_error().data("request failure observer dropped"))?;
            Ok(())
        });
    let release_callback = async move {
        request_failed_rx
            .await
            .expect("request should fail while close callback is still blocked");
        release_callback_tx
            .send(())
            .expect("close callback should still be running");
    };

    let scenario = async move {
        let (result, ((), ())) = join(
            connection,
            join(receive_requests_then_close(peer, 1), release_callback),
        )
        .await;
        result
    };
    tokio::time::timeout(TIMEOUT, scenario)
        .await
        .expect("pending request was withheld by close callback")
        .expect("connection foreground should handle the close error");
}

#[tokio::test]
async fn request_consumer_error_does_not_cancel_on_close_callbacks() {
    let (transport, peer) = Channel::duplex();
    let (request_failed_tx, request_failed_rx) = futures::channel::oneshot::channel();
    let callback_order = Arc::new(Mutex::new(Vec::new()));
    let first_callback_order = callback_order.clone();
    let second_callback_order = callback_order.clone();

    let connection = UntypedRole
        .builder()
        .on_close(async move |_cx| {
            // Wait until the EOF error has made the sibling task actor fail.
            // Close processing must keep this callback alive regardless.
            request_failed_rx
                .await
                .map_err(|_| Error::internal_error().data("request failure observer dropped"))?;
            first_callback_order.lock().unwrap().push(1);
            Err(Error::internal_error().data("close callback failed"))
        })
        .on_close(async move |_cx| {
            second_callback_order.lock().unwrap().push(2);
            Ok(())
        })
        .connect_with(transport, async move |cx| {
            cx.send_request(MyRequest {})
                .on_receiving_result(async move |result| {
                    let error = result.expect_err("request should fail at EOF");
                    assert_connection_closed(&error, "myRequest");
                    request_failed_tx
                        .send(())
                        .map_err(|()| Error::internal_error().data("close callback was dropped"))?;
                    Err(error)
                })?;

            cx.incoming_closed().await;
            Ok(())
        });

    let scenario = async move {
        let (result, ()) = join(connection, receive_requests_then_close(peer, 1)).await;
        result
    };
    let error = tokio::time::timeout(TIMEOUT, scenario)
        .await
        .expect("connection hung after request consumer failed")
        .expect_err("request consumer error should stop the connection");
    assert_eq!(
        error.data,
        Some(serde_json::json!("close callback failed")),
        "the configured close callback error should win over the sibling task error"
    );
    assert_eq!(*callback_order.lock().unwrap(), vec![1, 2]);
}

#[tokio::test]
async fn queued_request_is_failed_as_eof_before_outgoing_actor_runs() {
    let (transport, peer) = Channel::duplex();
    let (request_tx, request_rx) =
        futures::channel::oneshot::channel::<agent_client_protocol::SentRequest<MyResponse>>();

    let connection = UntypedRole
        .builder()
        .on_close(async |_cx| Err(Error::internal_error().data("stop after EOF")))
        .connect_with(transport, async move |cx| {
            request_tx
                .send(cx.send_request(MyRequest {}))
                .map_err(|_| Error::internal_error().data("request observer dropped"))?;
            future::pending::<Result<(), Error>>().await
        });
    let close_after_request_is_queued = async move {
        let request = request_rx.await.expect("main should create its request");
        // The outgoing actor has not been repolled since main_fn queued the
        // request. Make incoming EOF and its failing callback win that poll.
        drop(peer);
        request
    };

    let (connection_result, request) = join(connection, close_after_request_is_queued).await;
    connection_result.expect_err("close callback should stop the connection");
    let error = request
        .block_task()
        .await
        .expect_err("queued request should be failed by incoming EOF");
    assert_connection_closed(&error, "myRequest");
}

#[tokio::test]
async fn on_close_observes_queued_requests_already_failed() {
    let (transport, peer) = Channel::duplex();
    let (request_tx, request_rx) =
        futures::channel::oneshot::channel::<agent_client_protocol::SentRequest<MyResponse>>();
    let (request_queued_tx, request_queued_rx) = futures::channel::oneshot::channel();
    let (callback_done_tx, callback_done_rx) = futures::channel::oneshot::channel();

    let connection = UntypedRole
        .builder()
        .on_close(async move |_cx| {
            let request = request_rx
                .await
                .map_err(|_| Error::internal_error().data("queued request sender dropped"))?;
            let error = request
                .block_task()
                .now_or_never()
                .expect("queued request was still pending when close callback began")
                .expect_err("queued request should fail at EOF");
            assert_connection_closed(&error, "myRequest");
            callback_done_tx
                .send(())
                .map_err(|()| Error::internal_error().data("callback observer dropped"))
        })
        .connect_with(transport, async move |cx| {
            request_tx
                .send(cx.send_request(MyRequest {}))
                .map_err(|_| Error::internal_error().data("close callback receiver dropped"))?;
            request_queued_tx
                .send(())
                .map_err(|()| Error::internal_error().data("request queue observer dropped"))?;
            callback_done_rx
                .await
                .map_err(|_| Error::internal_error().data("close callback did not finish"))?;
            Ok(())
        });
    let close_after_request_is_queued = async move {
        request_queued_rx
            .await
            .expect("foreground should queue its request");
        drop(peer);
    };

    let scenario = async move {
        let (result, ()) = join(connection, close_after_request_is_queued).await;
        result
    };
    tokio::time::timeout(TIMEOUT, scenario)
        .await
        .expect("connection hung while closing a queued request")
        .expect("close callback and foreground should finish successfully");
}

#[tokio::test]
async fn request_finishing_conversion_after_eof_keeps_the_eof_cause() {
    let (transport, mut peer) = Channel::duplex();
    let (connection_tx, connection_rx) =
        futures::channel::oneshot::channel::<ConnectionTo<UntypedRole>>();
    let connection_tx = Arc::new(Mutex::new(Some(connection_tx)));
    let connection_observer = connection_tx.clone();

    let connection = UntypedRole
        .builder()
        .on_receive_request(
            async move |_request: MyRequest, responder, cx: ConnectionTo<UntypedRole>| {
                connection_observer
                    .lock()
                    .unwrap()
                    .take()
                    .expect("connection observer should run once")
                    .send(cx)
                    .map_err(|_| {
                        Error::internal_error().data("connection handle receiver dropped")
                    })?;
                responder.respond(MyResponse {
                    status: "ready".into(),
                })
            },
            agent_client_protocol::on_receive_request!(),
        )
        .connect_to(transport);
    let connection = tokio::spawn(connection);

    peer.tx
        .send(TransportFrame::Single(
            RawJsonRpcMessage::request(
                "myRequest".into(),
                serde_json::json!({}),
                RequestId::Number(1),
            )
            .unwrap(),
        ))
        .await
        .expect("send request that exposes the connection handle");
    let cx = tokio::time::timeout(TIMEOUT, connection_rx)
        .await
        .expect("handler did not expose its connection handle")
        .expect("connection handle sender dropped");
    assert!(matches!(
        tokio::time::timeout(TIMEOUT, peer.rx.next())
            .await
            .expect("handler response was not sent"),
        Some(TransportFrame::Single(RawJsonRpcMessage::Response(_)))
    ));

    let (entered_tx, entered_rx) = futures::channel::oneshot::channel();
    let (release_tx, release_rx) = std::sync::mpsc::channel();
    let request = BlockingRequest {
        entered: Arc::new(Mutex::new(Some(entered_tx))),
        release: Arc::new(Mutex::new(release_rx)),
    };
    let request_thread = std::thread::spawn(move || cx.send_request(request));

    tokio::time::timeout(TIMEOUT, entered_rx)
        .await
        .expect("request conversion did not start")
        .expect("request conversion entry sender dropped");

    // EOF starts the reactive drain and closes the outgoing receiver while
    // request conversion is still blocked before enqueueing.
    drop(peer);
    tokio::time::timeout(TIMEOUT, connection)
        .await
        .expect("reactive connection did not finish its drain")
        .expect("connection task panicked")
        .expect("clean EOF should finish successfully");

    release_tx
        .send(())
        .expect("release blocked request conversion");
    let request = request_thread.join().expect("request thread panicked");
    let error = request
        .block_task()
        .await
        .expect_err("request should retain the EOF failure cause");
    assert_connection_closed(&error, "blockingRequest");
}

#[tokio::test]
async fn incoming_eof_fails_every_pending_request() {
    let (transport, peer) = Channel::duplex();

    let connection = UntypedRole.builder().connect_with(transport, async |cx| {
        let first = cx.send_request(MyRequest {}).block_task();
        let second = cx.send_request(MyRequest {}).block_task();
        let (first, second) = join(first, second).await;

        assert_connection_closed(&first.expect_err("first request should fail"), "myRequest");
        assert_connection_closed(
            &second.expect_err("second request should fail"),
            "myRequest",
        );
        Ok(())
    });

    let scenario = async move {
        let (result, ()) = join(connection, receive_requests_then_close(peer, 2)).await;
        result
    };
    tokio::time::timeout(TIMEOUT, scenario)
        .await
        .expect("pending requests hung after transport EOF")
        .expect("connection foreground should handle both close errors");
}

#[tokio::test]
async fn byte_stream_eof_fails_an_in_flight_request() {
    let (client_outgoing, peer_incoming) = tokio::io::duplex(1024);
    let (peer_outgoing, client_incoming) = tokio::io::duplex(1024);
    let transport = ByteStreams::new(client_outgoing.compat_write(), client_incoming.compat());

    let connection = UntypedRole.builder().connect_with(transport, async |cx| {
        let error = cx
            .send_request(MyRequest {})
            .block_task()
            .await
            .expect_err("request should fail when byte stream reaches EOF");
        assert_connection_closed(&error, "myRequest");
        Ok(())
    });
    let peer = async move {
        let mut lines = tokio::io::BufReader::new(peer_incoming).lines();
        assert!(
            lines.next_line().await.unwrap().is_some(),
            "peer should receive the request before closing"
        );
        drop(peer_outgoing);
    };

    tokio::time::timeout(TIMEOUT, async move {
        let (result, ()) = join(connection, peer).await;
        result
    })
    .await
    .expect("request hung after byte stream EOF")
    .expect("connection foreground should handle the close error");
}

#[tokio::test]
async fn incoming_eof_delivers_error_to_callback_style_request_consumer() {
    let (transport, peer) = Channel::duplex();

    let connection = UntypedRole.builder().connect_with(transport, async |cx| {
        let (callback_tx, callback_rx) = futures::channel::oneshot::channel();
        cx.send_request(MyRequest {})
            .on_receiving_result(async move |result| {
                callback_tx
                    .send(result)
                    .map_err(|_| Error::internal_error().data("request callback receiver dropped"))
            })?;

        let error = callback_rx
            .await
            .map_err(|_| Error::internal_error().data("request callback did not run"))?
            .expect_err("callback should receive the incoming-close error");
        assert_connection_closed(&error, "myRequest");
        Ok(())
    });

    let scenario = async move {
        let (result, ()) = join(connection, receive_requests_then_close(peer, 1)).await;
        result
    };
    tokio::time::timeout(TIMEOUT, scenario)
        .await
        .expect("callback-style request consumer hung")
        .expect("connection foreground should handle callback error");
}

#[tokio::test]
async fn response_buffered_before_eof_is_delivered() {
    let (transport, mut peer) = Channel::duplex();

    let connection = UntypedRole.builder().connect_with(transport, async |cx| {
        let response = cx.send_request(MyRequest {}).block_task().await?;
        assert_eq!(response.status, "received");
        Ok(())
    });
    let respond_then_close = async move {
        let Some(TransportFrame::Single(RawJsonRpcMessage::Request(request))) =
            peer.rx.next().await
        else {
            panic!("expected outgoing request");
        };
        peer.tx
            .send(TransportFrame::Single(RawJsonRpcMessage::response(
                request.id,
                Ok(serde_json::to_value(MyResponse {
                    status: "received".into(),
                })
                .unwrap()),
            )))
            .await
            .expect("send response before closing transport");
        drop(peer);
    };

    tokio::time::timeout(TIMEOUT, async move {
        let (result, ()) = join(connection, respond_then_close).await;
        result
    })
    .await
    .expect("buffered response was lost at EOF")
    .expect("buffered response should win over later EOF");
}

#[tokio::test]
async fn response_router_drop_before_eof_is_not_reported_as_eof() {
    let (transport, peer) = Channel::duplex();

    let connection = UntypedRole
        .builder()
        .on_receive_dispatch(
            async |dispatch: Dispatch, _cx: ConnectionTo<UntypedRole>| match dispatch {
                // Claim and drop the router without delivering the response to
                // its SentRequest consumer.
                Dispatch::Response(..) => Ok(Handled::Yes),
                message => Ok(Handled::No {
                    message,
                    retry: false,
                }),
            },
            agent_client_protocol::on_receive_dispatch!(),
        )
        .connect_with(transport, async |cx| {
            let request = cx.send_request(MyRequest {});
            cx.incoming_closed().await;

            let error = request
                .block_task()
                .await
                .expect_err("dropped response router should cancel its local channel");
            assert_response_channel_lost(&error);
            Ok(())
        });

    let scenario = async move {
        let (result, ()) = join(connection, respond_then_close(peer)).await;
        result
    };
    tokio::time::timeout(TIMEOUT, scenario)
        .await
        .expect("connection hung after the response router was dropped")
        .expect("foreground should handle the local response-channel loss");
}

#[tokio::test]
async fn response_router_drop_after_eof_does_not_invoke_result_callback() {
    let (transport, peer) = Channel::duplex();
    let callback_ran = Arc::new(AtomicBool::new(false));
    let callback_observer = callback_ran.clone();

    let connection = UntypedRole
        .builder()
        .on_receive_dispatch(
            async |dispatch: Dispatch, _cx: ConnectionTo<UntypedRole>| match dispatch {
                Dispatch::Response(..) => Ok(Handled::Yes),
                message => Ok(Handled::No {
                    message,
                    retry: false,
                }),
            },
            agent_client_protocol::on_receive_dispatch!(),
        )
        .connect_with(transport, async move |cx| {
            let request = cx.send_request(MyRequest {});
            cx.incoming_closed().await;

            request.on_receiving_result(async move |_result| {
                callback_observer.store(true, Ordering::Release);
                Ok(())
            })?;
            future::pending::<Result<(), Error>>().await
        });

    let scenario = async move {
        let (result, ()) = join(connection, respond_then_close(peer)).await;
        result
    };
    let error = tokio::time::timeout(TIMEOUT, scenario)
        .await
        .expect("response consumer should fail instead of hanging")
        .expect_err("local response-channel loss should stop its consuming task");
    assert_response_channel_lost(&error);
    assert!(
        !callback_ran.load(Ordering::Acquire),
        "on_receiving_result must not treat local channel loss as a peer EOF error"
    );
}

#[tokio::test]
async fn request_created_after_incoming_eof_fails_immediately() {
    let (transport, peer) = Channel::duplex();
    drop(peer);

    tokio::time::timeout(
        TIMEOUT,
        UntypedRole.builder().connect_with(transport, async |cx| {
            cx.incoming_closed().await;
            let error = cx
                .send_request(MyRequest {})
                .block_task()
                .await
                .expect_err("post-close request should fail");
            assert_connection_closed(&error, "myRequest");
            Ok(())
        }),
    )
    .await
    .expect("post-close request hung")
    .expect("connection foreground should handle the close error");
}

#[tokio::test]
async fn incoming_read_error_is_not_reported_as_clean_eof() {
    let outgoing = futures::sink::unfold((), |(), _line: String| async { Ok::<_, io::Error>(()) });
    let incoming = stream::iter([Err(io::Error::other("read failed"))]);

    let result = tokio::time::timeout(
        TIMEOUT,
        UntypedRole
            .builder()
            .connect_to(Lines::new(outgoing, incoming)),
    )
    .await
    .expect("connection hung after transport read error");

    assert!(result.is_err(), "transport read error was swallowed as EOF");
}
