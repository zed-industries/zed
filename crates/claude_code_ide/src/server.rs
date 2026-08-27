//! The loopback WebSocket + MCP server Claude Code connects back into.
//!
//! Concurrency mirrors `context_server::listener::McpServer`: socket I/O on a
//! background task, dispatch on the foreground so tool handlers can `cx.update`
//! GPUI entities. Transport is a `smol` TCP listener upgraded by
//! `async_tungstenite` (the `typst_viewer` precedent) rather than the Unix socket
//! `context_server` uses.

use anyhow::{Context as _, Result};
use async_tungstenite::tungstenite::Message;
use async_tungstenite::tungstenite::handshake::server::{
    ErrorResponse, Request as HandshakeRequest, Response as HandshakeResponse,
};
use collections::HashMap;
use futures::channel::mpsc::{UnboundedReceiver, UnboundedSender, unbounded};
use futures::{FutureExt, StreamExt, select_biased};
use gpui::{AppContext as _, AsyncApp, Task};
use std::cell::RefCell;
use std::rc::Rc;

use crate::protocol::{self, CallToolParams, IncomingMessage, error_code, tool_error, tool_ok};
use crate::selection::{self, SharedSelection};
use serde_json::{Value, json};

/// The header Claude Code sends on the WebSocket upgrade carrying the token
/// from the lockfile.
const AUTH_HEADER: &str = "x-claude-code-ide-authorization";

/// Fallback MCP protocol version if the client's `initialize` omits one. In
/// practice we echo the client's version; this is only the default. Observed
/// value from claude-code 2.1.212 was "2025-11-25".
const MCP_PROTOCOL_VERSION: &str = "2025-11-25";

/// Live client connections, keyed by a monotonic id.
///
/// One workspace injects one `CLAUDE_CODE_SSE_PORT` into every terminal, so every
/// `claude` in it connects to this one server. Hence a set rather than a slot:
/// notifications go to all of them, and a connection closing removes only its own
/// entry so one client exiting never silences the rest.
type Connections = Rc<RefCell<ConnectionSet>>;

#[derive(Default)]
struct ConnectionSet {
    /// Monotonic id handed to the next accepted connection. Never reused, so a
    /// closing connection can remove exactly its own slot with no ABA race.
    next_id: u64,
    senders: HashMap<u64, UnboundedSender<String>>,
}

impl ConnectionSet {
    /// Register a new connection's sender, returning the id it must present when
    /// it later removes itself.
    fn insert(&mut self, sender: UnboundedSender<String>) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.senders.insert(id, sender);
        id
    }

    fn remove(&mut self, id: u64) {
        self.senders.remove(&id);
    }
}

/// A running bridge server. Dropping it stops the accept loop and closes the
/// listener; connected clients see the socket close.
pub struct BridgeServer {
    port: u16,
    /// Live senders to every connected client. Used to push `selection_changed`
    /// / `at_mentioned` notifications. Empty until the first client connects.
    connections: Connections,
    _accept_task: Task<()>,
}

impl BridgeServer {
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Broadcast a JSON-RPC notification to every connected client. Silently
    /// no-ops when no client is connected. Encoded once, sent to all.
    pub fn notify<T: serde::Serialize>(&self, method: &'static str, params: T) {
        let connections = self.connections.borrow();
        if connections.senders.is_empty() {
            log::debug!("claude_code_ide: no clients connected; dropping {method}");
            return;
        }
        let text = match serde_json::to_string(&protocol::notification(method, params)) {
            Ok(text) => text,
            Err(err) => {
                log::warn!("claude_code_ide: failed to encode {method}: {err}");
                return;
            }
        };
        for sender in connections.senders.values() {
            sender.unbounded_send(text.clone()).ok();
        }
    }
}

/// Bind a loopback listener on an ephemeral port and start accepting Claude Code
/// connections. `auth_token` must match what we write to the lockfile.
/// `selection` is the shared handle the selection tools read.
pub fn start(
    auth_token: String,
    selection: SharedSelection,
    cx: &mut AsyncApp,
) -> Task<Result<BridgeServer>> {
    let bind = cx.background_spawn(async move {
        let listener = smol::net::TcpListener::bind("127.0.0.1:0")
            .await
            .context("binding claude_code_ide listener")?;
        let port = listener.local_addr()?.port();
        anyhow::Ok((listener, port))
    });

    cx.spawn(async move |cx| {
        let (listener, port) = bind.await?;
        let connections: Connections = Rc::new(RefCell::new(ConnectionSet::default()));

        let accept_task = cx.spawn({
            let connections = connections.clone();
            async move |cx| {
                loop {
                    match listener.accept().await {
                        Ok((stream, peer)) => {
                            log::info!("claude_code_ide: connection from {peer}");
                            serve_connection(
                                stream,
                                auth_token.clone(),
                                selection.clone(),
                                connections.clone(),
                                cx,
                            );
                        }
                        Err(err) => {
                            log::warn!("claude_code_ide: accept error: {err}");
                            break;
                        }
                    }
                }
            }
        });

        log::info!("claude_code_ide: listening on 127.0.0.1:{port}");
        Ok(BridgeServer {
            port,
            connections,
            _accept_task: accept_task,
        })
    })
}

/// Handle one client connection: perform the authenticated WebSocket handshake,
/// then split into an I/O task and a dispatch loop.
fn serve_connection(
    stream: smol::net::TcpStream,
    auth_token: String,
    selection: SharedSelection,
    connections: Connections,
    cx: &mut AsyncApp,
) {
    let (incoming_tx, mut incoming_rx) = unbounded::<IncomingMessage>();
    let (outgoing_tx, outgoing_rx) = unbounded::<String>();

    // Register this client's sender in the connection set and remember its id so
    // we remove exactly this connection (not whichever connected last) on close.
    let connection_id = connections.borrow_mut().insert(outgoing_tx.clone());

    // The socket I/O runs on a background thread, so it may only capture `Send`
    // values (the channel endpoints), never the `Rc`-based connection set.
    cx.background_spawn(async move {
        if let Err(err) = handle_io(stream, auth_token, incoming_tx, outgoing_rx).await {
            log::info!("claude_code_ide: connection closed: {err}");
        }
    })
    .detach();

    // The dispatch loop runs on the foreground, where touching `connections`
    // (an `Rc`) is fine. When `incoming_rx` closes (client gone / I/O task
    // finished), remove only THIS connection's sender so the remaining clients
    // keep receiving notifications.
    cx.spawn(async move |cx| {
        while let Some(message) = incoming_rx.next().await {
            dispatch(message, &selection, &outgoing_tx, cx).await;
        }
        connections.borrow_mut().remove(connection_id);
    })
    .detach();
}

/// Run the WebSocket handshake (validating the auth header), then pump messages
/// between the socket and the mpsc channels until either side closes.
///
/// The handshake callback's error type is fixed by `accept_hdr_async`: it must
/// return `Result<HandshakeResponse, ErrorResponse>`, and that error is 136 bytes.
/// Boxing it is not open to us, so the size lint is allowed here rather than
/// worked around.
#[allow(clippy::result_large_err)]
async fn handle_io(
    stream: smol::net::TcpStream,
    auth_token: String,
    incoming_tx: UnboundedSender<IncomingMessage>,
    mut outgoing_rx: UnboundedReceiver<String>,
) -> Result<()> {
    let mut authorized = false;
    let ws = async_tungstenite::accept_hdr_async(stream, |req: &HandshakeRequest, mut response: HandshakeResponse| {
        let presented = req
            .headers()
            .get(AUTH_HEADER)
            .and_then(|value| value.to_str().ok());
        match presented {
            Some(token) if token == auth_token => {
                authorized = true;
                // Claude Code requests the `mcp` subprotocol on the upgrade; echo
                // it back so the negotiation is well-formed (it connects without
                // this too, but a correct server confirms the subprotocol).
                if req
                    .headers()
                    .get("sec-websocket-protocol")
                    .and_then(|value| value.to_str().ok())
                    .is_some_and(|protocols| {
                        protocols.split(',').any(|p| p.trim() == "mcp")
                    })
                {
                    if let Ok(value) = "mcp".parse() {
                        response.headers_mut().insert("sec-websocket-protocol", value);
                    }
                }
                Ok(response)
            }
            _ => {
                let mut error = ErrorResponse::new(Some("invalid or missing authorization".into()));
                *error.status_mut() =
                    async_tungstenite::tungstenite::http::StatusCode::UNAUTHORIZED;
                Err(error)
            }
        }
    })
    .await
    .context("websocket handshake")?;

    if !authorized {
        anyhow::bail!("connection was not authorized");
    }
    log::info!("claude_code_ide: client authorized");

    let (mut write, mut read) = ws.split();
    loop {
        select_biased! {
            outgoing = outgoing_rx.next().fuse() => {
                match outgoing {
                    Some(text) => {
                        log::trace!("claude_code_ide send: {text}");
                        write.send(Message::text(text)).await.context("ws send")?;
                    }
                    None => break,
                }
            }
            incoming = read.next().fuse() => {
                match incoming {
                    Some(Ok(Message::Text(text))) => {
                        log::trace!("claude_code_ide recv: {text}");
                        match serde_json::from_str::<IncomingMessage>(&text) {
                            Ok(message) => { incoming_tx.unbounded_send(message).ok(); }
                            Err(err) => log::warn!("claude_code_ide: bad message: {err}; raw: {text}"),
                        }
                    }
                    Some(Ok(Message::Close(_))) | None => break,
                    Some(Ok(Message::Ping(payload))) => {
                        write.send(Message::Pong(payload)).await.ok();
                    }
                    Some(Ok(_)) => {} // ignore binary/pong/frame
                    Some(Err(err)) => {
                        log::info!("claude_code_ide: read error: {err}");
                        break;
                    }
                }
            }
        }
    }
    Ok(())
}

/// Route one inbound JSON-RPC message to its handler and send the response.
async fn dispatch(
    message: IncomingMessage,
    selection: &SharedSelection,
    outgoing_tx: &UnboundedSender<String>,
    cx: &mut AsyncApp,
) {
    // Notifications (no id) get no response. We currently expect none inbound,
    // but the client may send `notifications/initialized`.
    if message.is_notification() {
        log::debug!("claude_code_ide: notification {} (ignored)", message.method);
        return;
    }
    let id = message.id.clone().unwrap_or(serde_json::Value::Null);

    match message.method.as_str() {
        "initialize" => {
            // Echo the client's protocolVersion (MCP negotiation): the real
            // Claude Code CLI sends a newer version than any we'd hardcode, and
            // reflecting it avoids a version-mismatch rejection. Fall back to our
            // default if the client omitted it.
            let protocol_version = message
                .params
                .as_ref()
                .and_then(|raw| serde_json::from_str::<serde_json::Value>(raw.get()).ok())
                .and_then(|params| {
                    params
                        .get("protocolVersion")
                        .and_then(|v| v.as_str())
                        .map(str::to_owned)
                })
                .unwrap_or_else(|| MCP_PROTOCOL_VERSION.to_string());
            send_ok(
                outgoing_tx,
                id,
                json!({
                    "protocolVersion": protocol_version,
                    // The tool set is static per session, so we never push
                    // tools/list_changed.
                    "capabilities": { "tools": { "listChanged": false } },
                    "serverInfo": {
                        "name": "zed-claude-code-ide",
                        "version": env!("CARGO_PKG_VERSION"),
                    },
                }),
            );
        }
        "tools/list" => send_ok(outgoing_tx, id, json!({ "tools": selection::tools() })),
        "tools/call" => {
            let params: CallToolParams = match message
                .params
                .as_ref()
                .map(|raw| serde_json::from_str(raw.get()))
                .transpose()
            {
                Ok(Some(params)) => params,
                Ok(None) => {
                    send_err(outgoing_tx, id, error_code::INVALID_PARAMS, "missing params");
                    return;
                }
                Err(err) => {
                    send_err(
                        outgoing_tx,
                        id,
                        error_code::INVALID_PARAMS,
                        format!("bad params: {err}"),
                    );
                    return;
                }
            };
            let result = call_tool(&params, selection, cx).await;
            send_ok(outgoing_tx, id, result);
        }
        other => {
            send_err(
                outgoing_tx,
                id,
                error_code::METHOD_NOT_FOUND,
                format!("unhandled method {other}"),
            );
        }
    }
}

/// Execute a named MCP tool. Unknown tools return a tool-level error rather than
/// a transport error, matching how Claude Code's other IDE servers behave.
async fn call_tool(
    params: &CallToolParams,
    selection: &SharedSelection,
    cx: &mut AsyncApp,
) -> serde_json::Value {
    match params.name.as_str() {
        "getCurrentSelection" | "getLatestSelection" => {
            selection::current_selection_result(selection, cx).await
        }
        "getWorkspaceFolders" => selection::workspace_folders_result(selection, cx).await,
        // Claude Code calls closeAllDiffTabs unprompted on connect, and drives
        // openDiff from its edit flow, so both must answer or every connection and
        // every edit reports an error. Acknowledging is not implementing: an
        // interactive keep/reject surface, and Jupyter execution, are separate work.
        "closeAllDiffTabs" => tool_ok("0"),
        "openDiff" => tool_ok("DIFF_REJECTED"),
        "executeCode" => tool_error("Jupyter code execution is not available in Zed"),
        other => tool_error(format!("unknown tool: {other}")),
    }
}

fn send_ok<T: serde::Serialize>(tx: &UnboundedSender<String>, id: Value, result: T) {
    send(tx, protocol::ok_response(id, result));
}

fn send_err(tx: &UnboundedSender<String>, id: Value, code: i32, message: impl Into<String>) {
    send(tx, protocol::err_response(id, code, message));
}

fn send(tx: &UnboundedSender<String>, message: Value) {
    match serde_json::to_string(&message) {
        Ok(text) => {
            tx.unbounded_send(text).ok();
        }
        Err(err) => log::warn!("claude_code_ide: failed to encode response: {err}"),
    }
}

#[cfg(test)]
mod connection_set_tests {
    use super::*;

    /// A broadcast reaches every registered connection, and closing one
    /// connection (removing its slot) leaves the others receiving. This is the
    /// heart of the multi-`claude` fix: N sessions share one server, so a
    /// notification must fan out to all of them and one session exiting must not
    /// silence the rest.
    #[test]
    fn broadcast_reaches_all_and_remove_is_scoped_to_one() {
        let mut set = ConnectionSet::default();

        let (tx_a, mut rx_a) = unbounded::<String>();
        let (tx_b, mut rx_b) = unbounded::<String>();
        let id_a = set.insert(tx_a);
        let id_b = set.insert(tx_b);
        assert_ne!(id_a, id_b, "each connection gets a distinct id");

        // A "broadcast" writes the same text to every sender in the set.
        // `try_recv()` yields `Ok(msg)` for a delivered message.
        for sender in set.senders.values() {
            sender.unbounded_send("first".to_string()).ok();
        }
        assert_eq!(rx_a.try_recv().ok(), Some("first".to_string()));
        assert_eq!(rx_b.try_recv().ok(), Some("first".to_string()));

        // Remove only connection A; B's slot must survive.
        set.remove(id_a);
        assert!(!set.senders.contains_key(&id_a), "A's slot is gone");
        assert!(set.senders.contains_key(&id_b), "B's slot survives A closing");

        for sender in set.senders.values() {
            sender.unbounded_send("second".to_string()).ok();
        }
        // B still receives; A's sender was dropped by remove(), so A's channel is
        // closed: `try_recv()` returns `Err(TryRecvError::Closed)`, never
        // "second".
        assert_eq!(rx_b.try_recv().ok(), Some("second".to_string()));
        assert!(
            rx_a.try_recv().is_err(),
            "a removed connection must receive no further broadcasts"
        );
    }

    /// Ids are monotonic and never reused, so a late close removing an old id
    /// cannot evict a newer connection that happened to reuse a slot (there is no
    /// slot reuse: the ABA hazard the monotonic counter exists to prevent).
    #[test]
    fn ids_are_monotonic_and_not_reused() {
        let mut set = ConnectionSet::default();
        let (tx0, _rx0) = unbounded::<String>();
        let (tx1, _rx1) = unbounded::<String>();
        let id0 = set.insert(tx0);
        set.remove(id0);
        let id1 = set.insert(tx1);
        assert!(id1 > id0, "a fresh connection never reuses a closed id");
    }
}
