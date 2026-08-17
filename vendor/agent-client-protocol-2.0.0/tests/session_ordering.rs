use std::time::Duration;

use agent_client_protocol::{
    ActiveSession, Agent, Channel, Client, Conductor, ConnectionTo, RawJsonRpcMessage, Responder,
    SessionMessage, TransportBatch, TransportFrame,
    schema::v1::{
        ContentBlock, ContentChunk, NewSessionRequest, NewSessionResponse, PromptRequest,
        PromptResponse, SessionId, SessionNotification, SessionUpdate, StopReason, TextContent,
    },
};
use futures::{StreamExt as _, channel::oneshot};

const TIMEOUT: Duration = Duration::from_secs(10);

// Compile-time regressions for the callback future bounds on the two non-blocking
// session helpers. The callbacks themselves are `'static`, but their future
// types deliberately carry an arbitrary shorter lifetime.
#[allow(dead_code)]
mod callback_future_lifetimes {
    use std::{
        future::Future,
        marker::PhantomData,
        pin::Pin,
        task::{Context, Poll},
    };

    use super::*;

    struct LifetimeTaggedFuture<'a>(PhantomData<&'a ()>);

    impl Future for LifetimeTaggedFuture<'_> {
        type Output = Result<(), agent_client_protocol::Error>;

        fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
            Poll::Ready(Ok(()))
        }
    }

    fn session_callback<'a>()
    -> impl FnOnce(ActiveSession<'static, Agent>) -> LifetimeTaggedFuture<'a> + Send + 'static {
        |_session| LifetimeTaggedFuture(PhantomData)
    }

    fn proxy_session_callback<'a>()
    -> impl FnOnce(SessionId) -> LifetimeTaggedFuture<'a> + Send + 'static {
        |_session_id| LifetimeTaggedFuture(PhantomData)
    }

    fn on_session_start_accepts_non_static_callback_future<'a>(
        connection: &ConnectionTo<Agent>,
        _scope: &'a str,
    ) -> Result<(), agent_client_protocol::Error> {
        connection
            .build_session_cwd()?
            .on_session_start(session_callback::<'a>())
    }

    fn on_proxy_session_start_accepts_non_static_callback_future<'a>(
        connection: &ConnectionTo<Conductor>,
        request: NewSessionRequest,
        responder: Responder<NewSessionResponse>,
        _scope: &'a str,
    ) -> Result<(), agent_client_protocol::Error> {
        connection
            .build_session_from(request)
            .on_proxy_session_start(responder, proxy_session_callback::<'a>())
    }
}

#[tokio::test(flavor = "current_thread")]
async fn on_session_start_callback_can_consume_later_session_messages() {
    let session_id = SessionId::new("ordered-session");
    let new_session_id = session_id.clone();
    let prompt_session_id = session_id.clone();

    let agent = Agent
        .builder()
        .on_receive_request(
            async move |_request: NewSessionRequest,
                        responder: Responder<NewSessionResponse>,
                        _connection: ConnectionTo<Client>| {
                responder.respond(NewSessionResponse::new(new_session_id.clone()))
            },
            agent_client_protocol::on_receive_request!(),
        )
        .on_receive_request(
            async move |request: PromptRequest,
                        responder: Responder<PromptResponse>,
                        connection: ConnectionTo<Client>| {
                assert_eq!(request.session_id, prompt_session_id);
                connection.send_notification(SessionNotification::new(
                    request.session_id,
                    SessionUpdate::AgentMessageChunk(ContentChunk::new(ContentBlock::Text(
                        TextContent::new("ordered response"),
                    ))),
                ))?;
                responder.respond(PromptResponse::new(StopReason::EndTurn))
            },
            agent_client_protocol::on_receive_request!(),
        );

    let (result_tx, result_rx) = oneshot::channel();
    let client = Client
        .builder()
        .connect_with(agent, async move |connection| {
            connection
                .build_session_cwd()?
                .on_session_start(async move |mut session| {
                    session.send_prompt("test ordering")?;
                    let text = session.read_to_string().await?;
                    result_tx
                        .send(text)
                        .map_err(|_| agent_client_protocol::Error::internal_error())
                })?;

            let text = result_rx
                .await
                .map_err(|_| agent_client_protocol::Error::internal_error())?;
            assert_eq!(text, "ordered response");
            Ok(())
        });

    tokio::time::timeout(TIMEOUT, client)
        .await
        .expect("session callback deadlocked the incoming dispatch loop")
        .expect("session connection failed");
}

#[tokio::test(flavor = "current_thread")]
async fn on_session_start_installs_routing_before_later_batch_entry() {
    let session_id = SessionId::new("same-batch-session");
    let response_session_id = session_id.clone();
    let notification_session_id = session_id.clone();
    let (transport, mut peer) = Channel::duplex();
    let (result_tx, result_rx) = oneshot::channel();

    let client = Client
        .builder()
        .connect_with(transport, async move |connection| {
            connection
                .build_session_cwd()?
                .on_session_start(async move |mut session| {
                    let update = session.read_update().await?;
                    assert!(matches!(update, SessionMessage::SessionMessage(_)));
                    result_tx
                        .send(())
                        .map_err(|()| agent_client_protocol::Error::internal_error())
                })?;

            result_rx
                .await
                .map_err(|_| agent_client_protocol::Error::internal_error())?;
            Ok(())
        });

    let peer = async move {
        let Some(TransportFrame::Single(RawJsonRpcMessage::Request(request))) =
            peer.rx.next().await
        else {
            panic!("expected a session/new request");
        };
        assert_eq!(request.method.as_ref(), "session/new");

        let response = RawJsonRpcMessage::response(
            request.id,
            Ok(
                serde_json::to_value(NewSessionResponse::new(response_session_id))
                    .expect("session response should serialize"),
            ),
        );
        let notification = RawJsonRpcMessage::notification(
            "session/update".into(),
            serde_json::to_value(SessionNotification::new(
                notification_session_id,
                SessionUpdate::AgentMessageChunk(ContentChunk::new(ContentBlock::Text(
                    TextContent::new("same batch"),
                ))),
            ))
            .expect("session notification should serialize"),
        )
        .expect("session notification should form valid JSON-RPC parameters");
        let batch = TransportBatch::from_messages([response, notification])
            .expect("test batch should be non-empty");
        peer.tx
            .unbounded_send(TransportFrame::Batch(batch))
            .expect("client should accept the response batch");

        while peer.rx.next().await.is_some() {}
        Ok::<(), agent_client_protocol::Error>(())
    };

    tokio::time::timeout(TIMEOUT, async { futures::try_join!(client, peer) })
        .await
        .expect("same-batch session update was not routed")
        .expect("session connection failed");
}
