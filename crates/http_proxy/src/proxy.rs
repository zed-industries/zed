//! The proxy itself: listener, connection handlers, upstream chaining.
//!
//! All synchronous, thread-per-connection. `ProxyHandle::spawn` binds a
//! `std::net::TcpListener` on `127.0.0.1:0` and returns once the listener
//! is bound and the listener thread has been spawned. Drop the handle to
//! shut everything down — the listener thread stops accepting new
//! connections; in-flight connection threads finish on their own when
//! either side closes.
//!
//! See the crate-level docs for trust assumptions and the "no proxy here"
//! principle.

mod connection;
mod upstream;

use crate::allowlist::Allowlist;
use anyhow::{Context, Result};
use futures::channel::mpsc;
use std::net::{Ipv4Addr, TcpListener, TcpStream};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread;

/// Cap on concurrently handled connections. Each connection costs the
/// editor process two threads and two pump buffers; the cap keeps a
/// runaway (or malicious) sandboxed command from exhausting the editor's
/// thread/fd budget. Well above what parallel package managers open.
const MAX_CONCURRENT_CONNECTIONS: usize = 256;

pub use upstream::UpstreamProxy;

/// Configuration for spawning a proxy.
#[derive(Debug, Clone)]
pub struct ProxyConfig {
    /// Hosts the proxy will allow to be reached.
    pub allowlist: Allowlist,
    /// Optional upstream HTTP proxy to chain through, with `NO_PROXY`-style
    /// bypasses for hosts that should connect direct.
    pub upstream: Option<UpstreamProxy>,
    /// Where the proxy reports per-connection events. Use
    /// [`mpsc::unbounded`] so connection threads (which are sync) never
    /// block on send. The receiver is async-friendly so `gpui` / `tokio`
    /// callers can poll it from their executor of choice.
    pub events: mpsc::UnboundedSender<ProxyEvent>,
}

/// A request method seen by the proxy.
///
/// Either a CONNECT (HTTPS tunnel) or an HTTP forward request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RequestMethod {
    Connect,
    Http(String),
}

impl RequestMethod {
    pub fn as_str(&self) -> &str {
        match self {
            RequestMethod::Connect => "CONNECT",
            RequestMethod::Http(method) => method.as_str(),
        }
    }
}

/// Outcome of a single connection's policy decision.
#[derive(Debug, Clone)]
pub enum RequestOutcome {
    Allowed,
    Denied { reason: DenyReason },
}

/// Why an attempted connection was denied.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DenyReason {
    /// Hostname (in punycode form on the wire) wasn't in the allowlist.
    HostNotInAllowlist { host: String },
    /// CONNECT or HTTP request targeted an IP literal. Denied unless the
    /// allowlist allows any host.
    IpLiteralRejected { target: String },
    /// The hostname resolved only to loopback / private / link-local
    /// addresses, which the sandbox policy never reaches via the allowlist
    /// (DNS-rebinding protection). Not applied when the allowlist allows
    /// any host.
    ResolvedToForbiddenIp { host: String },
}

impl DenyReason {
    pub(crate) fn proxy_status_error(&self) -> &'static str {
        match self {
            DenyReason::HostNotInAllowlist { .. } => "destination_ip_prohibited",
            DenyReason::IpLiteralRejected { .. } => "destination_ip_prohibited",
            DenyReason::ResolvedToForbiddenIp { .. } => "destination_ip_prohibited",
        }
    }

    pub(crate) fn human_explanation(&self) -> String {
        match self {
            DenyReason::HostNotInAllowlist { host } => {
                format!("host '{host}' is not in this conversation's network allowlist")
            }
            DenyReason::IpLiteralRejected { target } => format!(
                "target '{target}' is an IP literal; only hostnames are permitted by sandbox policy"
            ),
            DenyReason::ResolvedToForbiddenIp { host } => format!(
                "host '{host}' resolves only to loopback/private/link-local addresses, \
                 which sandbox policy blocks"
            ),
        }
    }
}

/// Events emitted by the proxy as it handles connections.
#[derive(Debug, Clone)]
pub enum ProxyEvent {
    /// Sent once after the listener is bound. Always the first event for
    /// a given proxy instance.
    Ready { port: u16 },

    /// Emitted at policy-decision time, before bytes flow to the upstream.
    RequestAttempt {
        host: String,
        port: u16,
        method: RequestMethod,
        outcome: RequestOutcome,
    },

    /// Emitted after an `Allowed` connection finishes. Carries throughput
    /// totals for diagnostics. Not emitted for denied connections.
    RequestCompleted {
        host: String,
        port: u16,
        method: RequestMethod,
        bytes_to_remote: u64,
        bytes_from_remote: u64,
        duration_ms: u64,
    },
}

/// Handle to a running proxy. Drop to stop the listener; in-flight
/// connection threads finish on their own as soon as either side closes.
pub struct ProxyHandle {
    port: u16,
    /// Listener thread sees this flip to `true` after `accept` returns and
    /// then exits.
    shutdown: Arc<AtomicBool>,
    /// Joined on drop to make shutdown deterministic in tests; ignored if
    /// the listener has already exited.
    listener_thread: Option<thread::JoinHandle<()>>,
}

impl ProxyHandle {
    /// Spawns the proxy: binds a listener on `127.0.0.1:0`, spawns the
    /// listener thread, sends a `Ready` event, and returns. The returned
    /// port is what callers should use for `HTTPS_PROXY`/`HTTP_PROXY` env
    /// vars and for the seatbelt rule narrowing `localhost:<port>`.
    pub fn spawn(config: ProxyConfig) -> Result<ProxyHandle> {
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0))
            .context("failed to bind proxy listener on 127.0.0.1:0")?;
        let port = listener
            .local_addr()
            .context("failed to read proxy local addr")?
            .port();

        // Inform the parent the proxy is ready before starting the accept
        // loop. Send is fire-and-forget on an unbounded channel — never
        // blocks, never errors meaningfully.
        let _ = config.events.unbounded_send(ProxyEvent::Ready { port });

        let shutdown = Arc::new(AtomicBool::new(false));
        let runtime_state = Arc::new(RuntimeState {
            allowlist: config.allowlist,
            upstream: config.upstream,
            events: config.events,
            active_connections: AtomicUsize::new(0),
        });

        let listener_thread = thread::Builder::new()
            .name("http-proxy-listener".to_string())
            // Listener thread does almost nothing on its stack — accept,
            // spawn, loop. 128 KiB is plenty.
            .stack_size(128 * 1024)
            .spawn({
                let shutdown = shutdown.clone();
                move || run_listener(listener, runtime_state, shutdown)
            })
            .context("failed to spawn proxy listener thread")?;

        Ok(ProxyHandle {
            port,
            shutdown,
            listener_thread: Some(listener_thread),
        })
    }

    /// The bound port. Stable for the lifetime of this handle.
    pub fn port(&self) -> u16 {
        self.port
    }
}

impl Drop for ProxyHandle {
    fn drop(&mut self) {
        self.shutdown.store(true, Ordering::SeqCst);
        // The listener is blocked in `accept()`. Waking it up cleanly via
        // a flag alone isn't possible with `std::net::TcpListener` — there's
        // no way to interrupt the syscall. Connect to ourselves: the
        // listener wakes up, accepts the connection, sees the shutdown
        // flag, breaks the loop. The accepted connection's worker thread
        // will read the empty stream and exit too.
        let _ = TcpStream::connect((Ipv4Addr::LOCALHOST, self.port));

        if let Some(thread) = self.listener_thread.take() {
            // Give the listener a chance to clean up. A join error means the
            // listener thread panicked; there's nothing to recover, but it
            // shouldn't pass unnoticed.
            if thread.join().is_err() {
                log::warn!("[http_proxy] listener thread panicked");
            }
        }
    }
}

/// State shared across all connection threads for a single proxy instance.
pub(crate) struct RuntimeState {
    pub(crate) allowlist: Allowlist,
    pub(crate) upstream: Option<UpstreamProxy>,
    pub(crate) events: mpsc::UnboundedSender<ProxyEvent>,
    active_connections: AtomicUsize,
}

/// Decrements the active-connection count when a connection thread finishes
/// (normally or by panic).
struct ConnectionSlot(Arc<RuntimeState>);

impl Drop for ConnectionSlot {
    fn drop(&mut self) {
        self.0.active_connections.fetch_sub(1, Ordering::SeqCst);
    }
}

fn run_listener(listener: TcpListener, state: Arc<RuntimeState>, shutdown: Arc<AtomicBool>) {
    for stream in listener.incoming() {
        if shutdown.load(Ordering::SeqCst) {
            log::debug!("[http_proxy] listener stopping (shutdown signaled)");
            break;
        }
        match stream {
            Ok(stream) => {
                let previous = state.active_connections.fetch_add(1, Ordering::SeqCst);
                if previous >= MAX_CONCURRENT_CONNECTIONS {
                    state.active_connections.fetch_sub(1, Ordering::SeqCst);
                    log::warn!(
                        "[http_proxy] dropping connection: {MAX_CONCURRENT_CONNECTIONS} \
                         connections already active"
                    );
                    drop(stream);
                    continue;
                }
                let slot = ConnectionSlot(state.clone());
                let state = state.clone();
                let result = thread::Builder::new()
                    .name("http-proxy-conn".to_string())
                    // Connection workers do bidir copy with a 64 KiB buffer
                    // and a few syscall stack frames. 128 KiB is plenty.
                    .stack_size(128 * 1024)
                    .spawn(move || {
                        let _slot = slot;
                        if let Err(e) = connection::handle(stream, state) {
                            log::debug!("[http_proxy] connection handler error: {e}");
                        }
                    });
                if let Err(e) = result {
                    log::warn!("[http_proxy] failed to spawn connection thread: {e}");
                }
            }
            Err(e) => {
                // EMFILE / per-process fd exhaustion is the realistic
                // failure here. Log and keep going — accept errors are
                // usually transient.
                log::warn!("[http_proxy] accept failed: {e}");
            }
        }
    }
}
