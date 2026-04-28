//! nREPL client: a single TCP connection that multiplexes requests by `id`
//! and exposes nREPL `session` lifecycle.
//!
//! nREPL has *two* identifiers that the design doc is explicit about
//! exposing separately:
//!
//! - **Request `id`** — opaque per-request identifier the client picks. The
//!   server echoes it back on every reply so we can demux concurrent
//!   requests sharing the connection. We allocate these as monotonically
//!   increasing decimal strings.
//! - **`session`** — server-side state token created by `{:op "clone"}`.
//!   One session corresponds to one logical REPL (its own `*1`/`*2`/`*3`,
//!   `*ns*`, dynamic vars). All evals that should share REPL state must
//!   carry the same `:session`.
//!
//! Replies are streams: a single `eval` produces any number of `out`/`err`/
//! `value` messages, terminated by one whose `:status` list contains
//! `"done"`. The read loop dispatches each reply to a per-request
//! `mpsc::UnboundedSender` keyed on the request `id`, and removes the entry
//! once `done` is observed (or the receiver is dropped).
//!
//! Interrupt is *per-eval*, not per-session: see [`NreplClient::interrupt`].

use std::{
    net::SocketAddr,
    pin::Pin,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
    task::{Context, Poll},
};

use anyhow::{Context as _, Result, anyhow, bail};
use collections::HashMap;
use futures::{
    AsyncReadExt as _, AsyncWriteExt as _, Stream, StreamExt as _,
    channel::mpsc::{self, UnboundedReceiver, UnboundedSender},
};
use gpui::{BackgroundExecutor, Task};
use parking_lot::Mutex;
use smol::net::TcpStream;

use crate::bencode::{self, DecodeOutcome, Value, dict};

/// Maximum size of a single bencode message we'll buffer before declaring
/// the connection poisoned. nREPL replies are small in practice
/// (`(println (range 100000))` chunks output across many `out` messages
/// rather than one giant string), so 16 MiB is generous.
const MAX_FRAME_BYTES: usize = 16 * 1024 * 1024;

/// A live nREPL connection.
///
/// Cheap to clone via the underlying [`Arc`]: clones share the same
/// connection, request multiplexer, and background tasks. Dropping the
/// last clone drops the I/O tasks, which closes the socket.
pub struct NreplClient {
    inner: Arc<Inner>,
    // Tasks are stored on the *original* client only. Cloned handles share
    // the inner state but don't extend task lifetime independently — see
    // `Clone` impl below.
    _io_tasks: Arc<IoTasks>,
}

struct IoTasks {
    _read: Task<()>,
    _write: Task<()>,
}

impl Clone for NreplClient {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            _io_tasks: self._io_tasks.clone(),
        }
    }
}

struct Inner {
    next_id: AtomicU64,
    pending: Mutex<Pending>,
    outgoing_tx: UnboundedSender<Vec<u8>>,
}

struct Pending {
    /// Request id (as wire string) -> reply forwarder.
    map: HashMap<String, UnboundedSender<Value>>,
    /// Set when the connection has been torn down. Further `send` calls
    /// fail with this error rather than queueing forever.
    closed: Option<Arc<anyhow::Error>>,
}

impl NreplClient {
    /// Connects to an nREPL server at `addr` and spawns the read/write
    /// background tasks on `executor`.
    ///
    /// This does **not** create a session; call [`Self::clone_session`]
    /// after a successful connect.
    pub async fn connect(addr: SocketAddr, executor: &BackgroundExecutor) -> Result<Self> {
        let stream = TcpStream::connect(addr)
            .await
            .with_context(|| format!("connecting to nREPL at {addr}"))?;
        // Disable Nagle: nREPL is request/response with small frames, and
        // 40ms of delay per eval is very visible in interactive use.
        stream.set_nodelay(true).ok();
        Ok(Self::from_stream(stream, executor))
    }

    /// Wraps an already-connected `TcpStream`. Exposed primarily for tests
    /// that bind a loopback listener and want to drive both ends.
    pub fn from_stream(stream: TcpStream, executor: &BackgroundExecutor) -> Self {
        let (reader, writer) = stream.split();
        let (outgoing_tx, outgoing_rx) = mpsc::unbounded::<Vec<u8>>();

        let inner = Arc::new(Inner {
            next_id: AtomicU64::new(1),
            pending: Mutex::new(Pending {
                map: HashMap::default(),
                closed: None,
            }),
            outgoing_tx,
        });

        let read_task = executor.spawn({
            let inner = inner.clone();
            async move {
                let result = read_loop(reader, &inner).await;
                let err = match result {
                    Ok(()) => anyhow!("nREPL connection closed by server"),
                    Err(e) => e,
                };
                log::debug!("nrepl: read loop ended: {err:#}");
                inner.shutdown(err);
            }
        });

        let write_task = executor.spawn(async move {
            if let Err(e) = write_loop(writer, outgoing_rx).await {
                log::debug!("nrepl: write loop ended: {e:#}");
            }
        });

        Self {
            inner,
            _io_tasks: Arc::new(IoTasks {
                _read: read_task,
                _write: write_task,
            }),
        }
    }

    /// Sends a request and returns a stream of replies.
    ///
    /// `request` MUST be a [`Value::Dict`]. The client injects the `id`
    /// field automatically (overwriting any pre-existing `id`); callers
    /// are responsible for everything else, including `op`, `session`,
    /// and op-specific args.
    ///
    /// The returned stream yields each reply message as it arrives and
    /// closes after the message whose `:status` contains `"done"`. If the
    /// connection drops mid-request, the stream closes too — callers
    /// should treat end-of-stream without an explicit `done` as a
    /// connection-level error if it matters to them.
    pub fn send(&self, mut request: Value) -> Result<RequestStream> {
        let dict_map = match &mut request {
            Value::Dict(map) => map,
            _ => bail!("nREPL request must be a dict"),
        };

        let id_num = self.inner.next_id.fetch_add(1, Ordering::Relaxed);
        let id = id_num.to_string();
        dict_map.insert(b"id".to_vec(), Value::str(&id));

        let (tx, rx) = mpsc::unbounded::<Value>();
        {
            let mut pending = self.inner.pending.lock();
            if let Some(err) = pending.closed.as_ref() {
                bail!("nREPL client closed: {err:#}");
            }
            pending.map.insert(id.clone(), tx);
        }

        let frame = bencode::encode(&request);
        if self.inner.outgoing_tx.unbounded_send(frame).is_err() {
            // Write loop is gone; clean up the pending entry we just
            // installed so it doesn't linger.
            self.inner.pending.lock().map.remove(&id);
            bail!("nREPL client write loop closed");
        }

        Ok(RequestStream { id, rx })
    }

    /// Issues `{:op "clone"}` and returns the new session id.
    ///
    /// Per the nREPL spec, the reply contains a `new-session` field with
    /// the freshly-allocated session token. We surface that string
    /// directly; callers pass it as `:session` on subsequent requests.
    pub async fn clone_session(&self) -> Result<String> {
        let mut stream = self.send(dict([("op", Value::str("clone"))]))?;
        let mut new_session: Option<String> = None;
        let mut error: Option<String> = None;
        while let Some(msg) = stream.next().await {
            if let Some(s) = msg.get("new-session").and_then(Value::as_str) {
                new_session = Some(s.to_string());
            }
            // Servers may report failure via a non-"done" status entry
            // (e.g. ["error", "done"]). Capture it for a useful error
            // message rather than just bailing with "no new-session".
            if let Some(items) = msg.get("status").and_then(Value::as_list) {
                for item in items {
                    if let Some(s) = item.as_str() {
                        if s != "done" {
                            error.get_or_insert_with(|| s.to_string());
                        }
                    }
                }
            }
        }
        match (new_session, error) {
            (Some(s), _) => Ok(s),
            (None, Some(e)) => bail!("nREPL clone failed: {e}"),
            (None, None) => bail!("nREPL clone reply did not contain new-session"),
        }
    }

    /// Sends `{:op "interrupt" :session ... :interrupt-id ...}` for an
    /// in-flight request.
    ///
    /// `request_id` is the request `id` the client originally generated for
    /// the eval — i.e. [`RequestStream::id`] of the eval whose stream is
    /// still open. Per the nREPL spec this is the only reliable way to
    /// interrupt a specific eval; interrupting "the session" without an
    /// id can race with newly-submitted forms.
    pub fn interrupt(&self, session: &str, request_id: &str) -> Result<RequestStream> {
        self.send(dict([
            ("op", Value::str("interrupt")),
            ("session", Value::str(session)),
            ("interrupt-id", Value::str(request_id)),
        ]))
    }

    /// Issues `{:op "close" :session ...}` and waits for the server's
    /// acknowledgement.
    pub async fn close_session(&self, session: &str) -> Result<()> {
        let mut stream = self.send(dict([
            ("op", Value::str("close")),
            ("session", Value::str(session)),
        ]))?;
        while stream.next().await.is_some() {}
        Ok(())
    }

    /// Returns true if the connection has been torn down (read loop ended).
    pub fn is_closed(&self) -> bool {
        self.inner.pending.lock().closed.is_some()
    }
}

impl Inner {
    fn dispatch(&self, value: Value) {
        let Some(id) = value.get("id").and_then(Value::as_str).map(str::to_owned) else {
            // Replies without an id can't be routed. nREPL servers don't
            // emit these in normal operation, but log so we notice if a
            // middleware ever does.
            log::debug!("nrepl: dropping reply with no id: {value:?}");
            return;
        };

        let is_done = value
            .get("status")
            .and_then(Value::as_list)
            .is_some_and(|items| items.iter().any(|item| item.as_str() == Some("done")));

        let mut pending = self.pending.lock();
        let Some(sender) = pending.map.get(&id) else {
            // Late reply for a request whose receiver was already dropped,
            // or duplicate `done`. Either way, harmless.
            log::debug!("nrepl: dropping reply for unknown id {id}");
            return;
        };

        if sender.unbounded_send(value).is_err() {
            // Receiver gone; drop the registration so we stop tracking it.
            pending.map.remove(&id);
            return;
        }

        if is_done {
            pending.map.remove(&id);
        }
    }

    fn shutdown(&self, err: anyhow::Error) {
        let err = Arc::new(err);
        let mut pending = self.pending.lock();
        // Dropping the senders closes every open request stream so callers
        // unblock. Future `send` calls observe `closed` and bail.
        pending.map.clear();
        if pending.closed.is_none() {
            pending.closed = Some(err);
        }
    }
}

/// Stream of replies for a single nREPL request.
///
/// Closes when the server sends a message with `:status` containing
/// `"done"`, or when the connection drops.
pub struct RequestStream {
    id: String,
    rx: UnboundedReceiver<Value>,
}

impl RequestStream {
    /// The client-generated request `id` echoed back on every reply.
    ///
    /// Pass this to [`NreplClient::interrupt`] to cancel the in-flight
    /// request.
    pub fn id(&self) -> &str {
        &self.id
    }
}

impl Stream for RequestStream {
    type Item = Value;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.rx).poll_next(cx)
    }
}

async fn read_loop(mut reader: futures::io::ReadHalf<TcpStream>, inner: &Inner) -> Result<()> {
    let mut buffer: Vec<u8> = Vec::with_capacity(8 * 1024);
    let mut chunk = [0u8; 8 * 1024];
    loop {
        let n = reader
            .read(&mut chunk)
            .await
            .context("reading from nREPL socket")?;
        if n == 0 {
            return Ok(());
        }
        buffer.extend_from_slice(&chunk[..n]);

        // Drain as many complete messages as we have. A single TCP read
        // can deliver several frames at once.
        loop {
            match bencode::decode_one(&buffer)? {
                DecodeOutcome::Incomplete => {
                    if buffer.len() > MAX_FRAME_BYTES {
                        bail!(
                            "nREPL frame exceeded {MAX_FRAME_BYTES} bytes without parsing; \
                             treating connection as poisoned"
                        );
                    }
                    break;
                }
                DecodeOutcome::Value { value, consumed } => {
                    buffer.drain(..consumed);
                    inner.dispatch(value);
                }
            }
        }
    }
}

async fn write_loop(
    mut writer: futures::io::WriteHalf<TcpStream>,
    mut outgoing: UnboundedReceiver<Vec<u8>>,
) -> Result<()> {
    while let Some(frame) = outgoing.next().await {
        writer
            .write_all(&frame)
            .await
            .context("writing to nREPL socket")?;
        writer.flush().await.context("flushing nREPL socket")?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;
    use smol::net::TcpListener;
    use std::sync::atomic::AtomicUsize;

    /// A minimal stand-in for an nREPL server. Accepts a single connection,
    /// reads bencode requests, and dispatches them to the provided handler
    /// closure. The handler returns the replies to send back, in order.
    ///
    /// Returning `None` from the handler closes the connection without a
    /// reply, simulating a server-side disconnect.
    async fn run_fake_server<H>(listener: TcpListener, mut handler: H) -> Result<()>
    where
        H: FnMut(Value) -> Vec<Value> + Send + 'static,
    {
        let (stream, _) = listener.accept().await?;
        stream.set_nodelay(true).ok();
        let (mut reader, mut writer) = stream.split();
        let mut buffer: Vec<u8> = Vec::new();
        let mut chunk = [0u8; 4096];
        loop {
            let n = reader.read(&mut chunk).await?;
            if n == 0 {
                return Ok(());
            }
            buffer.extend_from_slice(&chunk[..n]);
            loop {
                match bencode::decode_one(&buffer)? {
                    DecodeOutcome::Incomplete => break,
                    DecodeOutcome::Value { value, consumed } => {
                        buffer.drain(..consumed);
                        for reply in handler(value) {
                            writer.write_all(&bencode::encode(&reply)).await?;
                            writer.flush().await?;
                        }
                    }
                }
            }
        }
    }

    /// Helper: bind a loopback listener and return its address.
    async fn bind_loopback() -> (TcpListener, SocketAddr) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        (listener, addr)
    }

    #[gpui::test]
    async fn clone_session_returns_new_session_id(cx: &mut TestAppContext) {
        // The fake server uses real loopback TCP, which parks the executor
        // while waiting on accept/read. The test scheduler forbids parking
        // by default; opt in here.
        cx.executor().allow_parking();
        let (listener, addr) = bind_loopback().await;
        let server = cx.executor().spawn(run_fake_server(listener, |req| {
            assert_eq!(req.get("op").and_then(Value::as_str), Some("clone"));
            let id = req.get("id").and_then(Value::as_str).unwrap().to_string();
            vec![dict([
                ("id", Value::str(id)),
                ("new-session", Value::str("session-abc")),
                ("status", Value::List(vec![Value::str("done")])),
            ])]
        }));

        let client = NreplClient::connect(addr, &cx.executor()).await.unwrap();
        let session = client.clone_session().await.unwrap();
        assert_eq!(session, "session-abc");

        drop(client);
        // Server task wraps up once the client closes the socket.
        server.await.ok();
    }

    #[gpui::test]
    async fn eval_streams_multiple_messages_until_done(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        let (listener, addr) = bind_loopback().await;
        let server = cx.executor().spawn(run_fake_server(listener, |req| {
            let id = req.get("id").and_then(Value::as_str).unwrap().to_string();
            match req.get("op").and_then(Value::as_str) {
                Some("clone") => vec![dict([
                    ("id", Value::str(id)),
                    ("new-session", Value::str("s1")),
                    ("status", Value::List(vec![Value::str("done")])),
                ])],
                Some("eval") => {
                    let session = req
                        .get("session")
                        .and_then(Value::as_str)
                        .unwrap_or("")
                        .to_string();
                    vec![
                        dict([
                            ("id", Value::str(&id)),
                            ("session", Value::str(&session)),
                            ("out", Value::str("hi\n")),
                        ]),
                        dict([
                            ("id", Value::str(&id)),
                            ("session", Value::str(&session)),
                            ("ns", Value::str("user")),
                            ("value", Value::str("3")),
                        ]),
                        dict([
                            ("id", Value::str(&id)),
                            ("session", Value::str(&session)),
                            ("status", Value::List(vec![Value::str("done")])),
                        ]),
                    ]
                }
                _ => vec![],
            }
        }));

        let client = NreplClient::connect(addr, &cx.executor()).await.unwrap();
        let session = client.clone_session().await.unwrap();
        let mut stream = client
            .send(dict([
                ("op", Value::str("eval")),
                ("session", Value::str(&session)),
                ("code", Value::str("(do (println \"hi\") (+ 1 2))")),
            ]))
            .unwrap();

        let m1 = stream.next().await.expect("first reply");
        assert_eq!(m1.get("out").and_then(Value::as_str), Some("hi\n"));
        let m2 = stream.next().await.expect("second reply");
        assert_eq!(m2.get("value").and_then(Value::as_str), Some("3"));
        assert_eq!(m2.get("ns").and_then(Value::as_str), Some("user"));
        let m3 = stream.next().await.expect("third reply");
        assert!(
            m3.get("status")
                .and_then(Value::as_list)
                .map(|items| items.iter().any(|i| i.as_str() == Some("done")))
                .unwrap_or(false)
        );
        // Stream closes after `done`.
        assert!(stream.next().await.is_none());

        drop(client);
        server.await.ok();
    }

    #[gpui::test]
    async fn concurrent_requests_are_demultiplexed_by_id(cx: &mut TestAppContext) {
        // Server holds two requests open simultaneously and answers them
        // out of order to make sure the client routes by id rather than by
        // arrival sequence.
        cx.executor().allow_parking();
        let pending: Arc<Mutex<Vec<(String, String)>>> = Arc::new(Mutex::new(Vec::new()));
        let (listener, addr) = bind_loopback().await;
        let pending_for_handler = pending.clone();
        let server = cx.executor().spawn(run_fake_server(listener, move |req| {
            let id = req.get("id").and_then(Value::as_str).unwrap().to_string();
            match req.get("op").and_then(Value::as_str) {
                Some("clone") => vec![dict([
                    ("id", Value::str(id)),
                    ("new-session", Value::str("s1")),
                    ("status", Value::List(vec![Value::str("done")])),
                ])],
                Some("eval") => {
                    let code = req
                        .get("code")
                        .and_then(Value::as_str)
                        .unwrap_or("")
                        .to_string();
                    pending_for_handler.lock().push((id, code));
                    if pending_for_handler.lock().len() < 2 {
                        // Hold the first request: emit nothing yet.
                        return vec![];
                    }
                    // Both requests are in. Reply in reverse order.
                    let mut entries = std::mem::take(&mut *pending_for_handler.lock());
                    entries.reverse();
                    let mut replies = Vec::new();
                    for (req_id, req_code) in entries {
                        replies.push(dict([
                            ("id", Value::str(&req_id)),
                            ("value", Value::str(format!("answer:{req_code}"))),
                        ]));
                        replies.push(dict([
                            ("id", Value::str(&req_id)),
                            ("status", Value::List(vec![Value::str("done")])),
                        ]));
                    }
                    replies
                }
                _ => vec![],
            }
        }));

        let client = NreplClient::connect(addr, &cx.executor()).await.unwrap();
        let session = client.clone_session().await.unwrap();

        let mut s1 = client
            .send(dict([
                ("op", Value::str("eval")),
                ("session", Value::str(&session)),
                ("code", Value::str("first")),
            ]))
            .unwrap();
        let mut s2 = client
            .send(dict([
                ("op", Value::str("eval")),
                ("session", Value::str(&session)),
                ("code", Value::str("second")),
            ]))
            .unwrap();
        // Distinct request ids.
        assert_ne!(s1.id(), s2.id());

        let v1 = s1.next().await.expect("first stream value");
        assert_eq!(
            v1.get("value").and_then(Value::as_str),
            Some("answer:first")
        );
        let v2 = s2.next().await.expect("second stream value");
        assert_eq!(
            v2.get("value").and_then(Value::as_str),
            Some("answer:second")
        );

        // Drain `done` markers and confirm both streams close.
        let _ = s1.next().await;
        assert!(s1.next().await.is_none());
        let _ = s2.next().await;
        assert!(s2.next().await.is_none());

        drop(client);
        server.await.ok();
        let _ = pending; // keep the Arc alive until here
    }

    #[gpui::test]
    async fn interrupt_targets_specific_request_id(cx: &mut TestAppContext) {
        // Records every request the server sees so the test can assert
        // on the shape of the interrupt op.
        cx.executor().allow_parking();
        let seen: Arc<Mutex<Vec<Value>>> = Arc::new(Mutex::new(Vec::new()));
        let (listener, addr) = bind_loopback().await;
        let seen_for_handler = seen.clone();
        let interrupt_count = Arc::new(AtomicUsize::new(0));
        let interrupt_count_for_handler = interrupt_count.clone();

        let server = cx.executor().spawn(run_fake_server(listener, move |req| {
            seen_for_handler.lock().push(req.clone());
            let id = req.get("id").and_then(Value::as_str).unwrap().to_string();
            match req.get("op").and_then(Value::as_str) {
                Some("clone") => vec![dict([
                    ("id", Value::str(id)),
                    ("new-session", Value::str("s1")),
                    ("status", Value::List(vec![Value::str("done")])),
                ])],
                Some("eval") => {
                    // Don't reply; the test will interrupt this request.
                    vec![]
                }
                Some("interrupt") => {
                    interrupt_count_for_handler.fetch_add(1, Ordering::Relaxed);
                    let target = req
                        .get("interrupt-id")
                        .and_then(Value::as_str)
                        .unwrap_or("")
                        .to_string();
                    vec![
                        // Acknowledge the interrupt itself.
                        dict([
                            ("id", Value::str(&id)),
                            ("status", Value::List(vec![Value::str("done")])),
                        ]),
                        // And terminate the held eval with an "interrupted"
                        // status so the eval stream closes too.
                        dict([
                            ("id", Value::str(&target)),
                            (
                                "status",
                                Value::List(vec![Value::str("interrupted"), Value::str("done")]),
                            ),
                        ]),
                    ]
                }
                _ => vec![],
            }
        }));

        let client = NreplClient::connect(addr, &cx.executor()).await.unwrap();
        let session = client.clone_session().await.unwrap();

        let mut eval = client
            .send(dict([
                ("op", Value::str("eval")),
                ("session", Value::str(&session)),
                ("code", Value::str("(loop [] (recur))")),
            ]))
            .unwrap();
        let eval_id = eval.id().to_string();

        let mut int = client.interrupt(&session, &eval_id).unwrap();
        // Drain interrupt's own reply stream.
        while int.next().await.is_some() {}

        // Eval stream receives the interrupt status and then closes.
        let last = eval.next().await.expect("interrupted reply");
        let statuses: Vec<&str> = last
            .get("status")
            .and_then(Value::as_list)
            .unwrap()
            .iter()
            .filter_map(Value::as_str)
            .collect();
        assert!(statuses.contains(&"interrupted"));
        assert!(statuses.contains(&"done"));
        assert!(eval.next().await.is_none());

        // The interrupt op carried the right `interrupt-id`.
        let interrupt_req = seen
            .lock()
            .iter()
            .find(|v| v.get("op").and_then(Value::as_str) == Some("interrupt"))
            .cloned()
            .expect("interrupt request observed");
        assert_eq!(
            interrupt_req.get("interrupt-id").and_then(Value::as_str),
            Some(eval_id.as_str())
        );
        assert_eq!(
            interrupt_req.get("session").and_then(Value::as_str),
            Some(session.as_str())
        );
        assert_eq!(interrupt_count.load(Ordering::Relaxed), 1);

        drop(client);
        server.await.ok();
    }

    #[gpui::test]
    async fn server_disconnect_closes_open_request_streams(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        let (listener, addr) = bind_loopback().await;
        let server = cx.executor().spawn(async move {
            // Accept the connection, read one request, then drop the socket
            // without replying. The client's open RequestStream should
            // close cleanly rather than hang forever.
            let (stream, _) = listener.accept().await.unwrap();
            let (mut reader, _writer) = stream.split();
            let mut buf = [0u8; 1024];
            let _ = reader.read(&mut buf).await;
            // drop happens here
        });

        let client = NreplClient::connect(addr, &cx.executor()).await.unwrap();
        let mut stream = client
            .send(dict([
                ("op", Value::str("eval")),
                ("code", Value::str("nope")),
            ]))
            .unwrap();
        // No reply ever comes; the read loop will observe EOF and shut down.
        assert!(stream.next().await.is_none());
        // Subsequent sends fail rather than hanging.
        let send_result = client.send(dict([("op", Value::str("clone"))]));
        assert!(send_result.is_err());
        assert!(client.is_closed());

        server.await;
    }
}
