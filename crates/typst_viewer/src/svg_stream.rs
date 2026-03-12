use anyhow::{Context as _, Result};
use async_tungstenite::tungstenite::client::IntoClientRequest;
use async_tungstenite::tungstenite::Message;
use futures::StreamExt;
use serde::{Deserialize, Serialize};

/// Messages sent from Zed to the preview server.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClientMessage {
    /// Request SVG for a viewport region.
    #[serde(rename = "viewport")]
    Viewport {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    },
}

/// A connected WebSocket to a preview server.
pub type PreviewSocket = async_tungstenite::WebSocketStream<smol::net::TcpStream>;

/// Connect to a preview server's WebSocket endpoint.
///
/// Returns the WebSocket stream. Callers can use `futures::StreamExt::next()`
/// to read messages and `futures::SinkExt::send()` to write, or call
/// `.split()` to get independent read/write halves.
pub async fn connect(url: &str) -> Result<PreviewSocket> {
    let parsed_url = url::Url::parse(url).context("parsing WebSocket URL")?;
    let host = parsed_url.host_str().context("WebSocket URL missing host")?;
    let port = parsed_url.port().unwrap_or(80);
    let addr = format!("{host}:{port}");

    log::info!("typst_viewer: connecting to preview server at {addr}");

    let tcp = smol::net::TcpStream::connect(&addr)
        .await
        .with_context(|| format!("TCP connect to {addr}"))?;

    let mut request = url.into_client_request().context("building WebSocket request")?;
    request.headers_mut().insert(
        "Origin",
        format!("http://{addr}")
            .parse()
            .context("building Origin header")?,
    );

    let (ws, _response) = async_tungstenite::client_async(request, tcp)
        .await
        .context("WebSocket handshake failed")?;

    log::info!("typst_viewer: WebSocket connected to {addr}");

    Ok(ws)
}

/// Hardcoded SVG for testing the steel thread rendering pipeline.
pub fn mock_svg() -> String {
    format!(
        concat!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 595 842" width="595" height="842">"#,
            r#"<rect width="595" height="842" fill="white"/>"#,
            r#"<text x="50" y="100" font-family="serif" font-size="24" fill="black">Typst Preview</text>"#,
            r##"<text x="50" y="140" font-family="serif" font-size="16" fill="{color}">"##,
            r#"Steel Thread — Connected via WebSocket</text>"#,
            r##"<rect x="40" y="170" width="515" height="1" fill="{rule_color}"/>"##,
            r##"<text x="50" y="200" font-family="serif" font-size="14" fill="{text_color}">"##,
            r#"This SVG was delivered over WebSocket from the preview server.</text>"#,
            r##"<text x="50" y="225" font-family="serif" font-size="14" fill="{text_color}">"##,
            r#"In production, tinymist compiles Typst then renders SVG server-side.</text>"#,
            r#"</svg>"#,
        ),
        color = "#666666",
        rule_color = "#cccccc",
        text_color = "#333333",
    )
}

/// Start a mock preview server that sends SVG to connected clients.
///
/// Returns the WebSocket URL and a task handle that keeps the server alive.
/// The server sends an initial SVG immediately on connection, and responds
/// to viewport requests with additional SVG.
pub async fn start_mock_server() -> Result<(String, smol::Task<()>)> {
    let listener = smol::net::TcpListener::bind("127.0.0.1:0")
        .await
        .context("binding mock server")?;
    let addr = listener.local_addr().context("getting mock server address")?;
    let url = format!("ws://{addr}");

    log::info!("typst_viewer: mock SVG server listening on {addr}");

    let task = smol::spawn(async move {
        loop {
            match listener.accept().await {
                Ok((stream, peer)) => {
                    log::debug!("typst_viewer: mock server accepted connection from {peer}");
                    smol::spawn(handle_mock_connection(stream)).detach();
                }
                Err(err) => {
                    log::error!("typst_viewer: mock server accept error: {err}");
                    break;
                }
            }
        }
    });

    Ok((url, task))
}

async fn handle_mock_connection(stream: smol::net::TcpStream) {
    let ws = match async_tungstenite::accept_async(stream).await {
        Ok(ws) => ws,
        Err(err) => {
            log::error!("typst_viewer: mock server WebSocket accept error: {err}");
            return;
        }
    };

    let (mut write, mut read) = ws.split();

    if let Err(err) = write.send(Message::text(mock_svg())).await {
        log::error!("typst_viewer: mock server initial send error: {err}");
        return;
    }

    while let Some(Ok(msg)) = read.next().await {
        if let Message::Text(text) = msg {
            if serde_json::from_str::<ClientMessage>(&text).is_ok() {
                if let Err(err) = write.send(Message::text(mock_svg())).await {
                    log::error!("typst_viewer: mock server viewport response error: {err}");
                    break;
                }
            }
        }
    }
}

/// Generate an SVG tagged with a version number, for testing live updates.
/// Each version produces a distinct SVG so tests can verify which update arrived.
pub fn mock_svg_versioned(version: u32) -> String {
    format!(
        concat!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" "#,
            r#"viewBox="0 0 595 842" width="595" height="842">"#,
            r#"<rect width="595" height="842" fill="white"/>"#,
            r#"<text x="50" y="100" font-family="serif" font-size="24" "#,
            r#"fill="black">Version {version}</text>"#,
            r#"</svg>"#,
        ),
        version = version,
    )
}

/// Generate a multi-page mock document as a vec of page-framed SVG messages.
///
/// Each returned string is a complete WebSocket text message with the
/// `page:{index}:{total}\n` prefix that the client-side `parse_page_header`
/// expects. Page content includes distinguishing text so tests can verify
/// which page was received.
pub fn mock_svg_multipage(page_count: usize) -> Vec<String> {
    (0..page_count)
        .map(|i| {
            let svg = format!(
                concat!(
                    r#"<svg xmlns="http://www.w3.org/2000/svg" "#,
                    r#"viewBox="0 0 595 842" width="595" height="842">"#,
                    r#"<rect width="595" height="842" fill="white"/>"#,
                    r#"<text x="50" y="100" font-family="serif" font-size="24" "#,
                    r#"fill="black">Page {page} of {total}</text>"#,
                    r#"</svg>"#,
                ),
                page = i + 1,
                total = page_count,
            );
            format!("page:{i}:{page_count}\n{svg}")
        })
        .collect()
}

/// Start a mock server that simulates tinymist's live update behavior.
///
/// Unlike `start_mock_server`, this server pushes SVG updates to the connected
/// client whenever a message is sent on the returned channel — simulating the
/// tinymist flow where a file save triggers recompilation and an unprompted
/// SVG push over the existing WebSocket.
///
/// Accepts a single client connection (sufficient for testing).
///
/// Returns `(ws_url, server_task, update_sender)`.
pub async fn start_live_mock_server(
) -> Result<(String, smol::Task<()>, futures::channel::mpsc::UnboundedSender<String>)> {
    let listener = smol::net::TcpListener::bind("127.0.0.1:0")
        .await
        .context("binding live mock server")?;
    let addr = listener
        .local_addr()
        .context("getting live mock server address")?;
    let url = format!("ws://{addr}");

    let (update_tx, update_rx) = futures::channel::mpsc::unbounded::<String>();

    log::info!("typst_viewer: live mock server listening on {addr}");

    let task = smol::spawn(async move {
        if let Ok((stream, peer)) = listener.accept().await {
            log::debug!("typst_viewer: live mock server accepted connection from {peer}");
            handle_live_connection(stream, update_rx).await;
        }
    });

    Ok((url, task, update_tx))
}

/// Handle a single live-update connection: forward every SVG from `updates`
/// to the WebSocket client as a text message (mirroring tinymist's behavior
/// when `server_svg` is enabled).
async fn handle_live_connection(
    stream: smol::net::TcpStream,
    mut updates: futures::channel::mpsc::UnboundedReceiver<String>,
) {
    let ws = match async_tungstenite::accept_async(stream).await {
        Ok(ws) => ws,
        Err(err) => {
            log::error!("typst_viewer: live mock WebSocket accept error: {err}");
            return;
        }
    };

    let (mut write, _read) = ws.split();

    while let Some(svg) = updates.next().await {
        if let Err(err) = write.send(Message::text(svg)).await {
            log::error!("typst_viewer: live mock send error: {err}");
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::typst_viewer_view::{inject_glyph_defs, parse_page_header};

    #[test]
    fn websocket_svg_roundtrip() {
        smol::block_on(async {
            let (url, _server_task) = start_mock_server()
                .await
                .expect("failed to start mock server");

            let ws = connect(&url)
                .await
                .expect("failed to connect to mock server");

            let (mut write, mut read) = ws.split();

            // Server sends initial SVG on connect
            let msg = read
                .next()
                .await
                .expect("expected a message from server")
                .expect("message should be Ok");

            match msg {
                Message::Text(text) => {
                    let svg: &str = &text;
                    assert!(
                        svg.contains("<svg"),
                        "expected SVG content, got: {}",
                        &svg[..svg.len().min(100)]
                    );
                    assert!(
                        svg.contains("Typst Preview"),
                        "SVG missing expected text content"
                    );
                }
                other => panic!("expected Text message, got: {other:?}"),
            }

            // Send viewport request, should get SVG response
            let viewport = ClientMessage::Viewport {
                x: 0.0,
                y: 0.0,
                width: 595.0,
                height: 842.0,
            };
            let json =
                serde_json::to_string(&viewport).expect("failed to serialize viewport message");
            write
                .send(Message::text(json))
                .await
                .expect("failed to send viewport");

            let msg = read
                .next()
                .await
                .expect("expected a response message")
                .expect("response should be Ok");

            match msg {
                Message::Text(text) => {
                    assert!(text.contains("<svg"), "expected SVG in viewport response");
                }
                other => panic!("expected Text message in viewport response, got: {other:?}"),
            }
        });
    }

    #[test]
    fn client_message_serialization_roundtrip() {
        let msg = ClientMessage::Viewport {
            x: 10.0,
            y: 20.0,
            width: 595.0,
            height: 842.0,
        };

        let json = serde_json::to_string(&msg).expect("serialize");
        let parsed: ClientMessage = serde_json::from_str(&json).expect("deserialize");

        match parsed {
            ClientMessage::Viewport {
                x,
                y,
                width,
                height,
            } => {
                assert!((x - 10.0).abs() < f32::EPSILON);
                assert!((y - 20.0).abs() < f32::EPSILON);
                assert!((width - 595.0).abs() < f32::EPSILON);
                assert!((height - 842.0).abs() < f32::EPSILON);
            }
        }
    }

    #[test]
    fn mock_svg_is_valid() {
        let svg = mock_svg();
        assert!(svg.starts_with("<svg"), "should start with <svg tag");
        assert!(svg.ends_with("</svg>"), "should end with </svg>");
        assert!(svg.contains("xmlns"), "should have xmlns attribute");
        assert!(svg.contains("viewBox"), "should have viewBox attribute");
    }

    #[test]
    fn versioned_svg_contains_version() {
        for v in [0, 1, 42] {
            let svg = mock_svg_versioned(v);
            assert!(svg.starts_with("<svg"), "should start with <svg tag");
            assert!(svg.ends_with("</svg>"), "should end with </svg>");
            assert!(
                svg.contains(&format!("Version {v}")),
                "version {v} SVG should contain 'Version {v}'"
            );
        }
    }

    /// Proves the live-update contract: the client receives each SVG pushed
    /// by the server, in order, without sending any requests. This mirrors
    /// the tinymist flow where file-save → recompile → unprompted SVG push.
    #[test]
    fn live_update_receives_document_changes() {
        smol::block_on(async {
            let (url, _server_task, update_tx) = start_live_mock_server()
                .await
                .expect("failed to start live mock server");

            let mut ws = connect(&url)
                .await
                .expect("failed to connect to live mock server");

            // Simulate initial render (like tinymist responding to "current")
            update_tx
                .unbounded_send(mock_svg_versioned(0))
                .expect("send initial SVG");

            let msg = ws
                .next()
                .await
                .expect("expected initial message")
                .expect("initial message should be Ok");
            match msg {
                Message::Text(text) => {
                    assert!(
                        text.contains("Version 0"),
                        "initial SVG should be version 0, got: {}",
                        &text.as_str()[..text.len().min(120)]
                    );
                }
                other => panic!("expected Text message, got: {other:?}"),
            }

            // Simulate file save → recompile → new SVG push
            update_tx
                .unbounded_send(mock_svg_versioned(1))
                .expect("send update 1");

            let msg = ws
                .next()
                .await
                .expect("expected update 1")
                .expect("update 1 should be Ok");
            match msg {
                Message::Text(text) => {
                    assert!(
                        text.contains("Version 1"),
                        "first update should be version 1"
                    );
                }
                other => panic!("expected Text message for update 1, got: {other:?}"),
            }

            // Simulate a second save
            update_tx
                .unbounded_send(mock_svg_versioned(2))
                .expect("send update 2");

            let msg = ws
                .next()
                .await
                .expect("expected update 2")
                .expect("update 2 should be Ok");
            match msg {
                Message::Text(text) => {
                    assert!(
                        text.contains("Version 2"),
                        "second update should be version 2"
                    );
                }
                other => panic!("expected Text message for update 2, got: {other:?}"),
            }
        });
    }

    /// Simulates rapid saves (e.g. the user types fast with on-type refresh).
    /// All updates should arrive in order, none dropped.
    #[test]
    fn live_update_handles_rapid_sequential_updates() {
        smol::block_on(async {
            let (url, _server_task, update_tx) = start_live_mock_server()
                .await
                .expect("failed to start live mock server");

            let mut ws = connect(&url)
                .await
                .expect("failed to connect to live mock server");

            let count = 10;
            for i in 0..count {
                update_tx
                    .unbounded_send(mock_svg_versioned(i))
                    .expect("send rapid update");
            }

            for i in 0..count {
                let msg = ws
                    .next()
                    .await
                    .unwrap_or_else(|| panic!("expected message for version {i}"))
                    .unwrap_or_else(|err| panic!("message {i} should be Ok: {err}"));

                match msg {
                    Message::Text(text) => {
                        assert!(
                            text.contains(&format!("Version {i}")),
                            "message {i} should contain 'Version {i}', got: {}",
                            &text.as_str()[..text.len().min(120)]
                        );
                    }
                    other => panic!("expected Text for version {i}, got: {other:?}"),
                }
            }
        });
    }

    /// Verifies the client sees the WebSocket close cleanly when the server's
    /// update channel is dropped (simulating tinymist shutdown).
    #[test]
    fn live_update_server_shutdown_closes_cleanly() {
        smol::block_on(async {
            let (url, _server_task, update_tx) = start_live_mock_server()
                .await
                .expect("failed to start live mock server");

            let mut ws = connect(&url)
                .await
                .expect("failed to connect to live mock server");

            // Send one SVG, then drop the sender to close the channel
            update_tx
                .unbounded_send(mock_svg_versioned(0))
                .expect("send SVG before shutdown");

            let msg = ws
                .next()
                .await
                .expect("expected message before shutdown")
                .expect("message should be Ok");
            assert!(matches!(msg, Message::Text(_)));

            // Drop sender — server handler's `while let Some(svg)` loop exits,
            // which closes the WebSocket.
            drop(update_tx);

            // The next read should indicate the connection is closed.
            // Depending on timing, this can be:
            // - None (stream ended cleanly)
            // - A Close frame (explicit WebSocket close)
            // - ResetWithoutClosingHandshake (server dropped without close handshake)
            let next = ws.next().await;
            match next {
                None => {}
                Some(Ok(Message::Close(_))) => {}
                Some(Err(async_tungstenite::tungstenite::Error::Protocol(
                    async_tungstenite::tungstenite::error::ProtocolError::ResetWithoutClosingHandshake,
                ))) => {}
                Some(other) => {
                    panic!("expected stream end, Close, or ResetWithoutClosingHandshake after shutdown, got: {other:?}")
                }
            }
        });
    }

    #[test]
    fn multipage_mock_produces_correct_count_and_headers() {
        for count in [1, 3, 5] {
            let pages = mock_svg_multipage(count);
            assert_eq!(pages.len(), count, "should produce {count} pages");

            for (i, msg) in pages.iter().enumerate() {
                let expected_prefix = format!("page:{i}:{count}\n");
                assert!(
                    msg.starts_with(&expected_prefix),
                    "page {i} should start with '{expected_prefix}', got: {}",
                    &msg[..msg.len().min(40)]
                );

                // The SVG body after the header should be valid.
                let svg = &msg[expected_prefix.len()..];
                assert!(svg.starts_with("<svg"), "page {i} body should start with <svg");
                assert!(svg.ends_with("</svg>"), "page {i} body should end with </svg>");
                assert!(
                    svg.contains(&format!("Page {} of {count}", i + 1)),
                    "page {i} should contain distinguishing text"
                );
            }
        }
    }

    #[test]
    fn parse_page_header_roundtrip() {
        let pages = mock_svg_multipage(3);
        for (i, msg) in pages.iter().enumerate() {
            let (header, svg) = parse_page_header(msg)
                .unwrap_or_else(|| panic!("should parse page header for page {i}"));
            assert_eq!(header.index, i, "page index mismatch");
            assert_eq!(header.total, 3, "page total mismatch");
            assert!(svg.starts_with("<svg"), "svg body should start with <svg");
        }
    }

    #[test]
    fn parse_page_header_rejects_non_page_messages() {
        assert!(parse_page_header("<svg>...</svg>").is_none());
        assert!(parse_page_header("viewport,0 0 0").is_none());
        assert!(parse_page_header("page:abc:def\n<svg/>").is_none());
        assert!(parse_page_header("page:0\n<svg/>").is_none()); // missing total
    }

    #[test]
    fn inject_glyph_defs_inserts_after_svg_tag() {
        let svg = br#"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100"><rect/></svg>"#;
        let defs = r#"<defs id="glyph"><symbol id="g1"><path d="M0 0"/></symbol></defs>"#;
        let result = inject_glyph_defs(svg, defs);
        let result_str = String::from_utf8(result).expect("valid utf8");

        // Defs should appear right after the opening <svg ...> tag.
        let svg_close = result_str.find('>').unwrap();
        let after_tag = &result_str[svg_close + 1..];
        assert!(
            after_tag.starts_with(r#"<defs id="glyph">"#),
            "defs should be inserted right after <svg> tag, got: {}",
            &after_tag[..after_tag.len().min(80)]
        );

        // Original content should still be present after the defs.
        assert!(result_str.contains("<rect/>"), "original content preserved");
        assert!(result_str.ends_with("</svg>"), "closing tag preserved");
    }

    #[test]
    fn inject_glyph_defs_noop_on_empty_defs() {
        let svg = b"<svg><text>hello</text></svg>";
        let result = inject_glyph_defs(svg, "");
        assert_eq!(result, svg, "empty defs should not change the SVG");
    }

    #[test]
    fn live_update_multipage_delivers_all_pages_in_order() {
        smol::block_on(async {
            let (url, _server_task, update_tx) = start_live_mock_server()
                .await
                .expect("failed to start live mock server");

            let mut ws = connect(&url)
                .await
                .expect("failed to connect to live mock server");

            let page_count = 3;
            let pages = mock_svg_multipage(page_count);
            for page_msg in &pages {
                update_tx
                    .unbounded_send(page_msg.clone())
                    .expect("send page");
            }

            for i in 0..page_count {
                let msg = ws
                    .next()
                    .await
                    .unwrap_or_else(|| panic!("expected message for page {i}"))
                    .unwrap_or_else(|err| panic!("page {i} should be Ok: {err}"));

                match msg {
                    Message::Text(text) => {
                        let (header, svg) = parse_page_header(&text)
                            .unwrap_or_else(|| panic!("page {i} should have header"));
                        assert_eq!(header.index, i, "page index");
                        assert_eq!(header.total, page_count, "page total");
                        assert!(
                            svg.contains(&format!("Page {} of {page_count}", i + 1)),
                            "page {i} content"
                        );
                    }
                    other => panic!("expected Text for page {i}, got: {other:?}"),
                }
            }
        });
    }
}