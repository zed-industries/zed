//! A Unix socket letting programs in a workspace's terminals feed selections to
//! this workspace's bridge, which the editor path cannot see.
//!
//! The bridge advertises the socket to its terminals as `$ZED_SELECTION_SOCK`, so
//! connecting to the inherited path is itself the proof the caller belongs to this
//! workspace: no port matching, no lockfile hunt.
//!
//! One newline-terminated JSON object per connection, one reply line, then close:
//!
//! ```text
//! {"op":"push_selection","text":"...","filePath":"/abs/path",
//!  "fileUrl":"file:///abs/path","source":"cli:42",
//!  "selection":{"start":{"line":0,"character":0},"end":{"line":3,"character":0}}}
//! ```
//!
//! `text` and `filePath` are required, `fileUrl` defaults to `file://<filePath>`,
//! `selection` defaults to a zero span, and coordinates are zero-based. `source`
//! keys the region (see [`key_for`]). `clear_selection` retracts one source, or all
//! of them with `"all": true`.
//!
//! Clients live in separate repositories: `zed-claude-selection.nvim` and
//! `cc-ide-sel`. `bridge_tests` holds a client payload verbatim, so it doubles as
//! the contract.

use anyhow::{Context as _, Result};
use futures::{
    AsyncBufReadExt as _, AsyncReadExt as _, AsyncWriteExt as _, StreamExt as _, io::BufReader,
};
use gpui::{AsyncApp, EntityId, Task};
use net::async_net::{UnixListener, UnixStream};
use serde::Deserialize;
use std::path::{Path, PathBuf};

use crate::protocol::{Position, SelectionPayload, SelectionRange};

/// The variable naming this socket in the environment of every terminal the
/// workspace spawns. A client finds the socket by reading it, so the name is part
/// of the contract.
pub const SELECTION_SOCK_ENV: &str = "ZED_SELECTION_SOCK";

/// Refuse absurd requests rather than buffering them. A selection is text a
/// human chose, so a megabyte is already far past anything meaningful.
const MAX_REQUEST_BYTES: u64 = 1024 * 1024;

/// The directory holding every workspace's selection socket.
pub fn socket_dir() -> PathBuf {
    paths::temp_dir().join("zed-cc-ide")
}

/// The socket path for a bridge, keyed by the loopback port it already owns.
/// That port is unique per workspace and lives exactly as long as the bridge, so
/// the socket needs no separate identity and no workspace database id.
pub fn socket_path_for_port(port: u16) -> PathBuf {
    socket_dir().join(format!("{port}.sock"))
}

/// Create the parent directory private (0700) and clear any stale socket file,
/// which would otherwise make `bind` fail with `EADDRINUSE`.
fn prepare_socket_path(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating selection socket dir {}", parent.display()))?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            std::fs::set_permissions(parent, std::fs::Permissions::from_mode(0o700))
                .with_context(|| format!("restricting {}", parent.display()))?;
        }
    }
    if path.exists() {
        std::fs::remove_file(path)
            .with_context(|| format!("removing stale selection socket {}", path.display()))?;
        log::info!(
            "claude_code_ide: removed stale selection socket {}",
            path.display()
        );
    }
    Ok(())
}

/// Remove the socket file when the bridge goes away. A missing file is normal.
pub fn unlink_socket(path: &Path) {
    match std::fs::remove_file(path) {
        Ok(()) => log::info!(
            "claude_code_ide: unlinked selection socket {}",
            path.display()
        ),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {}
        Err(err) => log::warn!(
            "claude_code_ide: failed to unlink selection socket {}: {err}",
            path.display()
        ),
    }
}

/// One inbound request. Every field past `op` is optional on the wire, so a
/// malformed request is answered with an error rather than dropping the connection.
#[derive(Debug, Deserialize)]
struct Request {
    op: String,
    text: Option<String>,
    #[serde(rename = "filePath")]
    file_path: Option<String>,
    #[serde(rename = "fileUrl")]
    file_url: Option<String>,
    /// Stable per-caller key, and the field a client should set. Re-pushing under
    /// the same key replaces that caller's region instead of adding another, so a
    /// long-lived editor holds exactly one region however many files it visits.
    /// Absent, the file path keys it, and regions accumulate as the caller moves
    /// between files.
    source: Option<String>,
    selection: Option<Span>,
    /// On `clear_selection`, drop EVERY external region rather than just this
    /// caller's. The escape hatch for a region whose owner died leaving no pid to
    /// reap by, so nothing can address its key any more.
    #[serde(default)]
    all: bool,
}

#[derive(Debug, Deserialize, Default)]
struct Span {
    #[serde(default)]
    start: Point,
    #[serde(default)]
    end: Point,
}

#[derive(Debug, Deserialize, Default, Clone, Copy)]
struct Point {
    #[serde(default)]
    line: u32,
    #[serde(default)]
    character: u32,
}

/// What a validated request asks the bridge to do.
enum Action {
    /// Set this source's region, replacing whatever it held before.
    Push {
        source: String,
        payload: SelectionPayload,
    },
    /// Drop this source's region, because its selection has gone away.
    Clear { source: String },
    /// Drop every external region.
    ClearAll,
}

/// The key a caller's region is held under: its stated `source`, else the file path.
///
/// The peer's pid is deliberately not a key, though it is still read for reaping. It
/// resolves only while the peer is alive, and these clients close the socket as soon
/// as they have written, so keying by it gave `pid:` on one push and `file:` on the
/// next with nothing to explain why.
fn key_for(explicit: Option<String>, file_path: Option<&str>) -> Option<String> {
    if let Some(source) = explicit {
        return Some(source);
    }
    log::info!(
        "claude_code_ide: selection caller stated no source, keying by file path; \
         state a `source` to hold exactly one region"
    );
    file_path.map(|path| format!("file:{path}"))
}

/// Validate a request and decide what it asks for. Returns the reason on
/// rejection so the caller can be told what was wrong.
fn parse(request: Request) -> std::result::Result<Action, String> {
    match request.op.as_str() {
        "push_selection" => {
            let (Some(text), Some(file_path)) = (request.text, request.file_path) else {
                return Err("push_selection requires both text and filePath".to_string());
            };
            let source = key_for(request.source, Some(&file_path))
                .ok_or_else(|| "cannot identify the calling source".to_string())?;

            // An empty selection is a retraction: broadcasting an empty region
            // would show `claude` a selection with no content in it.
            if text.is_empty() {
                return Ok(Action::Clear { source });
            }

            let file_url = request
                .file_url
                .unwrap_or_else(|| format!("file://{file_path}"));
            let span = request.selection.unwrap_or_default();
            Ok(Action::Push {
                source,
                payload: SelectionPayload {
                    text,
                    file_path,
                    file_url,
                    selection: SelectionRange {
                        start: Position {
                            line: span.start.line,
                            character: span.start.character,
                        },
                        end: Position {
                            line: span.end.line,
                            character: span.end.character,
                        },
                    },
                },
            })
        }
        "clear_selection" if request.all => Ok(Action::ClearAll),
        // `filePath` is optional here: it only matters as the fallback key, which a
        // caller that states a `source` never needs. A clear naming neither says
        // nothing about what to drop, so it is refused rather than guessed at.
        "clear_selection" => {
            let source = key_for(request.source, request.file_path.as_deref()).ok_or_else(|| {
                "clear_selection needs a source or a filePath to say what to clear; \
                 pass `all: true` to clear every region"
                    .to_string()
            })?;
            Ok(Action::Clear { source })
        }
        other => Err(format!("unknown op: {other}")),
    }
}

/// Bind the socket and start accepting. Returns the bound path (to advertise and
/// later unlink) and the accept task, which stops when dropped.
///
/// Accept and dispatch both run on the foreground executor, so pushing into the
/// bridge needs no thread hand-off and nothing has to be `Send`.
pub fn start(
    workspace_id: EntityId,
    port: u16,
    cx: &mut AsyncApp,
) -> Result<(PathBuf, Task<()>)> {
    let path = socket_path_for_port(port);
    prepare_socket_path(&path)?;
    let listener = UnixListener::bind(&path)
        .with_context(|| format!("binding selection socket {}", path.display()))?;
    log::info!(
        "claude_code_ide: selection socket listening on {}",
        path.display()
    );

    let task = cx.spawn(async move |cx| {
        let mut incoming = listener.incoming();
        while let Some(stream) = incoming.next().await {
            match stream {
                Ok(stream) => serve(stream, workspace_id, cx).await,
                Err(err) => {
                    log::warn!("claude_code_ide: selection socket accept error: {err}");
                    break;
                }
            }
        }
        log::info!("claude_code_ide: selection socket accept loop ended");
    });

    Ok((path, task))
}

/// Handle one connection: read a single line, push it, answer one line.
async fn serve(mut stream: UnixStream, workspace_id: EntityId, cx: &mut AsyncApp) {
    // Identify the caller from the kernel before reading a byte it sent, so the
    // region key cannot be forged by the request body.
    let peer_pid = crate::peer_cred::peer_pid(&stream);

    let mut line = String::new();
    // Cap the stream before buffering it: `Take` is `AsyncRead` but not
    // `AsyncBufRead`, so capping the reader instead would lose `read_line`. The
    // borrow ends with this block, freeing the stream for the reply below.
    let read = {
        let mut reader = BufReader::new((&mut stream).take(MAX_REQUEST_BYTES));
        reader.read_line(&mut line).await
    };
    if let Err(err) = read {
        log::warn!("claude_code_ide: selection socket read failed: {err}");
        return;
    }
    if line.trim().is_empty() {
        return;
    }

    let reply = match serde_json::from_str::<Request>(line.trim()) {
        Ok(request) => match parse(request) {
            Ok(Action::Push { source, payload }) => {
                let file_path = payload.file_path.clone();
                let logged_source = source.clone();
                cx.update(|cx| {
                    crate::bridge::push_external_selection(
                        workspace_id,
                        source,
                        payload,
                        peer_pid,
                        cx,
                    );
                });
                log::info!(
                    "claude_code_ide: relayed selection from {logged_source} ({file_path}) \
                     to the bridge"
                );
                serde_json::json!({ "ok": true })
            }
            Ok(Action::Clear { source }) => {
                let logged_source = source.clone();
                cx.update(|cx| {
                    crate::bridge::clear_external_selection(workspace_id, source, cx);
                });
                log::info!("claude_code_ide: cleared the selection held for {logged_source}");
                serde_json::json!({ "ok": true })
            }
            Ok(Action::ClearAll) => {
                cx.update(|cx| {
                    crate::bridge::clear_all_external_selections(workspace_id, cx);
                });
                log::info!("claude_code_ide: cleared every external selection region");
                serde_json::json!({ "ok": true })
            }
            Err(reason) => {
                log::info!("claude_code_ide: rejected selection request: {reason}");
                serde_json::json!({ "ok": false, "error": reason })
            }
        },
        Err(err) => {
            log::warn!("claude_code_ide: malformed selection request: {err}");
            serde_json::json!({ "ok": false, "error": format!("malformed request: {err}") })
        }
    };

    // The caller is fire-and-forget (the Neovim shim closes without reading), so
    // a failed write is expected and only worth a debug line.
    let encoded = format!("{reply}\n");
    if let Err(err) = stream.write_all(encoded.as_bytes()).await {
        log::debug!("claude_code_ide: selection socket reply not delivered: {err}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Parse a raw line the way `serve` does.
    fn parse_raw(raw: &str) -> std::result::Result<Action, String> {
        let request = serde_json::from_str::<Request>(raw).expect("request parses as JSON");
        parse(request)
    }

    fn expect_push(action: Action) -> (String, SelectionPayload) {
        match action {
            Action::Push { source, payload } => (source, payload),
            Action::Clear { source } => panic!("expected a push, got a clear for {source}"),
            Action::ClearAll => panic!("expected a push, got a clear-all"),
        }
    }

    fn expect_clear(action: Action) -> String {
        match action {
            Action::Clear { source } => source,
            Action::Push { source, .. } => panic!("expected a clear, got a push for {source}"),
            Action::ClearAll => panic!("expected a scoped clear, got a clear-all"),
        }
    }

    /// `all: true` sweeps every region and needs no identity, since it is the
    /// escape hatch for a key nothing can address any more.
    #[test]
    fn clear_all_needs_no_source() {
        let action = parse_raw(r#"{"op":"clear_selection","all":true}"#)
            .expect("clear-all is valid with nothing naming a source");
        assert!(
            matches!(action, Action::ClearAll),
            "expected ClearAll, got something else"
        );
    }

    /// A client that states its own `source` is keyed by it, whatever file the
    /// selection came from, which is how one editor holds exactly one region
    /// however many files it visits.
    #[test]
    fn parses_a_client_payload_keyed_by_its_stated_source() {
        let raw = r#"{"op":"push_selection","text":"line two\nline three",
                      "filePath":"/repo/a.rs","fileUrl":"file:///repo/a.rs",
                      "source":"nvim:4242",
                      "selection":{"start":{"line":1,"character":0},
                                   "end":{"line":2,"character":0}}}"#;
        let (source, payload) = expect_push(parse_raw(raw).expect("payload is valid"));
        log::info!("observed source: {source}");
        assert_eq!(source, "nvim:4242");
        assert_eq!(payload.text, "line two\nline three");
        assert_eq!(payload.file_url, "file:///repo/a.rs");
        assert_eq!(payload.selection.start.line, 1);
        assert_eq!(payload.selection.end.line, 2);
    }

    /// `fileUrl` and `selection` are optional: the URL is derived from the path
    /// and the span defaults to zero.
    #[test]
    fn derives_file_url_and_defaults_span() {
        let raw = r#"{"op":"push_selection","text":"hi","filePath":"/a/b.txt"}"#;
        let (_source, payload) = expect_push(parse_raw(raw).expect("minimal payload is valid"));
        log::info!("observed derived url: {}", payload.file_url);
        assert_eq!(payload.file_url, "file:///a/b.txt");
        assert_eq!(payload.selection.start.line, 0);
        assert_eq!(payload.selection.end.character, 0);
    }

    /// The key is the stated `source`, else the file path. There is no third rule, so
    /// the same request always yields the same key. The consequences that matter: one
    /// source holds ONE region across several files (so a push replaces rather than
    /// accumulates), and two sources stay independent.
    #[test]
    fn key_is_the_stated_source_else_the_file_path() {
        let key = |raw: &str| expect_push(parse_raw(raw).expect("valid")).0;

        assert_eq!(
            key(r#"{"op":"push_selection","text":"x","filePath":"/a.rs","source":"cli"}"#),
            "cli",
            "a stated source is used verbatim"
        );
        assert_eq!(
            key(r#"{"op":"push_selection","text":"x","filePath":"/a.rs"}"#),
            "file:/a.rs",
            "without a source the path keys it"
        );
        assert_eq!(
            key(r#"{"op":"push_selection","text":"a","filePath":"/one.rs","source":"nvim:7"}"#),
            key(r#"{"op":"push_selection","text":"b","filePath":"/two.rs","source":"nvim:7"}"#),
            "one source reuses its key across files, so its region is replaced"
        );
        assert_ne!(
            key(r#"{"op":"push_selection","text":"x","filePath":"/a.rs","source":"nvim:11"}"#),
            key(r#"{"op":"push_selection","text":"x","filePath":"/a.rs","source":"nvim:12"}"#),
            "distinct sources must not collide"
        );
    }

    /// A retraction arrives two ways: the explicit op, or a push whose text is
    /// empty (which is what an editor with nothing selected has to say). Both key
    /// the same way as a push, so a client retracts exactly its own region.
    #[test]
    fn clears_via_the_op_and_via_empty_text() {
        let explicit = r#"{"op":"clear_selection","source":"nvim:5"}"#;
        assert_eq!(
            expect_clear(parse_raw(explicit).expect("valid")),
            "nvim:5",
            "clear_selection needs no filePath when a source is stated"
        );

        let empty_push =
            r#"{"op":"push_selection","text":"","filePath":"/a.rs","source":"nvim:5"}"#;
        assert_eq!(
            expect_clear(parse_raw(empty_push).expect("valid")),
            "nvim:5",
            "an empty selection is a retraction, not a region"
        );
    }

    /// Missing `text` or `filePath` is rejected with a reason, and an unknown op
    /// is named in the error rather than silently ignored.
    #[test]
    fn rejects_incomplete_and_unknown_requests() {
        for raw in [
            r#"{"op":"push_selection","filePath":"/a.rs"}"#,
            r#"{"op":"push_selection","text":"hi"}"#,
            r#"{"op":"push_selection"}"#,
        ] {
            let observed = parse_raw(raw).err();
            log::info!("observed rejection for {raw}: {observed:?}");
            assert!(observed.is_some(), "{raw} must be rejected");
        }

        let observed = parse_raw(r#"{"op":"send_input","text":"x"}"#)
            .err()
            .expect("unknown op is rejected");
        assert!(
            observed.contains("send_input"),
            "error should name the op, observed {observed}"
        );

        // A clear naming neither a source nor a file says nothing about what to
        // drop, and the error points at the sweep instead of guessing.
        let observed = parse_raw(r#"{"op":"clear_selection"}"#)
            .err()
            .expect("an unaddressed clear is rejected");
        log::info!("observed unaddressed-clear rejection: {observed}");
        assert!(observed.contains("all"), "observed {observed}");
    }

    /// The socket path is keyed by port, so two workspaces never collide and the
    /// path is stable for a given bridge.
    #[test]
    fn socket_path_is_per_port() {
        let a = socket_path_for_port(41234);
        let b = socket_path_for_port(41235);
        log::info!("observed paths a={a:?} b={b:?}");
        assert_ne!(a, b);
        assert_eq!(a, socket_path_for_port(41234));
        assert!(a.to_string_lossy().ends_with("41234.sock"));
        assert!(a.starts_with(socket_dir()));
    }
}
