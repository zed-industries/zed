use std::{
    convert::Infallible,
    time::Duration,
};

use anyhow::Context;
use async_trait::async_trait;
use convex_sync_types::{
    backoff::Backoff,
    headers::{
        DEPRECATION_MSG_HEADER_NAME,
        DEPRECATION_STATE_HEADER_NAME,
    },
    ClientMessage,
    SessionId,
    Timestamp,
};
use futures::{
    select_biased,
    stream::Fuse,
    FutureExt,
    SinkExt,
    StreamExt,
};
use tokio::{
    net::TcpStream,
    sync::{
        mpsc,
        oneshot,
    },
    task::JoinHandle,
    time::{
        Instant,
        Interval,
    },
};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{
        self,
        client::IntoClientRequest,
        http::HeaderMap,
        protocol::Message,
    },
    MaybeTlsStream,
    WebSocketStream,
};
use url::Url;
use uuid::Uuid;

use super::WebSocketState;
use crate::sync::{
    ProtocolResponse,
    ReconnectRequest,
    ServerMessage,
    SyncProtocol,
};

const INITIAL_BACKOFF: Duration = Duration::from_millis(100);
const MAX_BACKOFF: Duration = Duration::from_secs(15);
type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

#[derive(Debug)]
enum WebSocketRequest {
    SendMessage(ClientMessage, oneshot::Sender<()>),
    Reconnect(ReconnectRequest),
}

struct WebSocketInternal {
    ws_stream: WsStream,
    last_server_response: Instant,
}
struct WebSocketWorker {
    ws_url: Url,
    on_response: mpsc::Sender<ProtocolResponse>,
    on_state_change: Option<mpsc::Sender<WebSocketState>>,
    internal_receiver: Fuse<UnboundedReceiverStream<WebSocketRequest>>,
    ping_ticker: Interval,
    connection_count: u32,
    backoff: Backoff,
}

pub struct WebSocketManager {
    internal_sender: mpsc::UnboundedSender<WebSocketRequest>,
    worker_handle: JoinHandle<Infallible>,
}
impl Drop for WebSocketManager {
    fn drop(&mut self) {
        self.worker_handle.abort()
    }
}

#[async_trait]
impl SyncProtocol for WebSocketManager {
    async fn open(
        ws_url: Url,
        on_response: mpsc::Sender<ProtocolResponse>,
        on_state_change: Option<mpsc::Sender<WebSocketState>>,
        client_id: &str,
    ) -> anyhow::Result<Self> {
        let (internal_sender, internal_receiver) = mpsc::unbounded_channel();
        let worker_handle = tokio::spawn(WebSocketWorker::run(
            ws_url,
            on_response,
            on_state_change,
            internal_receiver,
            client_id.to_string(),
        ));

        Ok(WebSocketManager {
            internal_sender,
            worker_handle,
        })
    }

    async fn send(&mut self, message: ClientMessage) -> anyhow::Result<()> {
        let (tx, rx) = oneshot::channel();
        self.internal_sender
            .send(WebSocketRequest::SendMessage(message, tx))?;
        rx.await?;
        Ok(())
    }

    async fn reconnect(&mut self, request: ReconnectRequest) {
        let _ = self
            .internal_sender
            .send(WebSocketRequest::Reconnect(request));
    }
}

impl WebSocketWorker {
    /// How often heartbeat pings are sent.
    const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
    /// How long before lack of server response causes a timeout.
    const SERVER_INACTIVITY_THRESHOLD: Duration = Duration::from_secs(30);

    async fn run(
        ws_url: Url,
        on_response: mpsc::Sender<ProtocolResponse>,
        on_state_change: Option<mpsc::Sender<WebSocketState>>,
        internal_receiver: mpsc::UnboundedReceiver<WebSocketRequest>,
        client_id: String,
    ) -> Infallible {
        let ping_ticker = tokio::time::interval(Self::HEARTBEAT_INTERVAL);
        let backoff = Backoff::new(INITIAL_BACKOFF, MAX_BACKOFF);

        let mut worker = Self {
            ws_url,
            on_response,
            on_state_change,
            internal_receiver: UnboundedReceiverStream::new(internal_receiver).fuse(),
            ping_ticker,
            connection_count: 0,
            backoff,
        };

        let mut last_close_reason = "InitialConnect".to_string();
        let mut max_observed_timestamp = None;
        if let Some(state_change_sender) = &worker.on_state_change {
            let _ = state_change_sender.try_send(WebSocketState::Connecting);
        }
        loop {
            let exit_result = worker
                .work(last_close_reason, max_observed_timestamp, &client_id)
                .await;

            if let Some(state_change_sender) = &worker.on_state_change {
                let _ = state_change_sender.try_send(WebSocketState::Connecting);
            }

            let e = match exit_result {
                Ok(reconnect) => {
                    // WS worker exited cleanly because it got a request to reconnect
                    tracing::debug!("Reconnecting websocket due to {}", reconnect.reason);
                    last_close_reason = reconnect.reason;
                    max_observed_timestamp = reconnect.max_observed_timestamp;
                    continue;
                },
                Err(e) => e,
            };
            worker.connection_count += 1;
            last_close_reason = e.to_string();
            let delay = worker.backoff.fail(&mut rand::rng());
            tracing::error!(
                "Convex WebSocketWorker failed: {e:?}. Backing off for {delay:?} and retrying."
            );

            // Tell the worker that we've failed so it can coordinate the reconnect.
            // The worker will send a Reconnect message and the new query set all together.
            // Drain the input request queue until we get that reconnect message - which
            // will be followed by the refreshed query set.
            let _ = worker.on_response.send(ProtocolResponse::Failure).await;
            tracing::debug!("Waiting for base client to acknowledge reconnect");
            loop {
                let request = worker.internal_receiver.next().await;
                // TODO: There is a potential issue where we have multiple queued reconnect
                // requests in which case max_observed_timestamp might be lower than actually
                // observed. This is fine since it will never cause errors. Will can fix this
                // when we restructure the wider protocol to be a single routine.
                if let Some(WebSocketRequest::Reconnect(reconnect)) = request {
                    max_observed_timestamp = reconnect.max_observed_timestamp;
                    break;
                }
            }
            tracing::debug!(
                "Base client acknowledged reconnect. Sleeping {delay:?} and reconnecting"
            );
            tokio::time::sleep(delay).await;
            tracing::debug!("Reconnecting");
        }
    }

    async fn work(
        &mut self,
        last_close_reason: String,
        max_seen_transition: Option<Timestamp>,
        client_id: &str,
    ) -> anyhow::Result<ReconnectRequest> {
        let verb = if self.connection_count == 0 {
            "connect"
        } else {
            "reconnect"
        };
        tracing::debug!("trying to {verb} to {}", self.ws_url);
        let mut internal = WebSocketInternal::new(
            self.ws_url.clone(),
            self.connection_count,
            last_close_reason,
            max_seen_transition,
            client_id,
        )
        .await?;
        tracing::debug!("completed websocket {verb} to {}", self.ws_url);
        if let Some(state_change_sender) = &self.on_state_change {
            let _ = state_change_sender.try_send(WebSocketState::Connected);
        }

        loop {
            select_biased! {
                _ = self.ping_ticker.tick().fuse() => {
                    let now = Instant::now();
                    if now - internal.last_server_response > Self::SERVER_INACTIVITY_THRESHOLD {
                        anyhow::bail!("InactiveServer");
                    }
                },
                server_msg = internal.ws_stream.select_next_some() => {
                    internal.last_server_response = Instant::now();

                    match server_msg.context("WebsocketConnectionError")? {
                        Message::Close(close_frame) => {
                            let close_frame = close_frame.context("CloseMessageWithoutFrame")?;
                            tracing::debug!("Close frame {close_frame}");
                            anyhow::bail!("{}", close_frame.reason);
                        },
                        Message::Text(t) => {
                            let json: serde_json::Value = serde_json::from_str(&t).context("JsonDeserializeError")?;
                            let server_message = json.try_into()?;
                            match server_message {
                                ServerMessage::Ping => tracing::trace!("received message {server_message:?}"),
                                _ => tracing::trace!("received message {server_message:?}"),
                            };

                            let resp = ProtocolResponse::ServerMessage(server_message);
                            let _ = self.on_response.send(resp).await;

                            // TODO: Similar to JS, we should ideally only reset backoff if we get
                            // the client gets into a correct state, where we have Connected and
                            // received a response to our pending Queries and Mutations.
                            self.backoff.reset();
                        },
                        Message::Ping(_) => {
                            tracing::trace!("received Ping");
                        }
                        server_msg => {
                            tracing::debug!("received unknown message {server_msg:?}");
                        },
                    }
                },
                request = self.internal_receiver.select_next_some() => {
                    match request {
                        WebSocketRequest::SendMessage(message, sender) => {
                            tracing::debug!("Sending {message:?}");
                            let msg = Message::Text(serde_json::Value::try_from(message).context("JsonSerializeError")?.to_string().into());
                            internal.send_worker(msg.clone()).await?;
                            let _ = sender.send(());
                        },
                        WebSocketRequest::Reconnect(reason) => return Ok(reason),
                    };
                }
            };
        }
    }
}

fn deprecation_message(headers: &HeaderMap) -> Option<String> {
    let dep_state = headers.get(DEPRECATION_STATE_HEADER_NAME)?.to_str().ok()?;
    let msg = headers.get(DEPRECATION_MSG_HEADER_NAME)?.to_str().ok()?;
    Some(format!("{dep_state}: {msg}"))
}

impl WebSocketInternal {
    async fn new(
        ws_url: Url,
        connection_count: u32,
        last_close_reason: String,
        max_observed_timestamp: Option<Timestamp>,
        client_id: &str,
    ) -> anyhow::Result<WebSocketInternal> {
        let mut request = (&ws_url).into_client_request().context("Bad WS Url")?;
        request.headers_mut().insert(
            "Convex-Client",
            client_id.try_into().context("Bad client id")?,
        );
        let (ws_stream, response) = connect_async(request).await.map_err(|e| {
            if let tungstenite::Error::Http(ref response) = e {
                let body = response
                    .body()
                    .as_deref()
                    .map(String::from_utf8_lossy)
                    .unwrap_or_default();
                return anyhow::anyhow!("Connection to {ws_url} failed: {e}: {body}");
            }
            anyhow::anyhow!("Connection to {ws_url} failed: {e}")
        })?;

        if let Some(msg) = deprecation_message(response.headers()) {
            tracing::warn!("{msg}");
        }

        let last_server_response = Instant::now();
        let mut internal = WebSocketInternal {
            ws_stream,
            last_server_response,
        };

        // Send an initial connect message on the new websocket
        let session_id = Uuid::new_v4();
        let message = ClientMessage::Connect {
            session_id: SessionId::new(session_id),
            connection_count,
            last_close_reason,
            max_observed_timestamp,
        };
        let msg = Message::Text(
            serde_json::Value::try_from(message)
                .context("JSONSerializationErrorOnConnect")?
                .to_string()
                .into(),
        );
        internal.send_worker(msg).await?;

        Ok(internal)
    }

    async fn send_worker(&mut self, message: Message) -> anyhow::Result<()> {
        self.ws_stream
            .send(message)
            .await
            .context("WebsocketClosedOnSend")
    }
}
