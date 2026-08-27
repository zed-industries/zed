//! Transport-level integration tests: drive the real WebSocket + MCP server as
//! a Claude Code client would, over a loopback socket.
//!
//! Dispatch runs on the foreground executor while socket I/O runs on the background
//! reactor, so these run the *client* on `cx.background_executor` (a real thread) and
//! drive the server with `run_until_parked`. `allow_parking` is required because the
//! client blocks on real socket reads.

use crate::selection::SharedSelection;
use crate::server;
use async_tungstenite::tungstenite::Message;
use async_tungstenite::tungstenite::client::IntoClientRequest;
use futures::StreamExt as _;
use gpui::TestAppContext;

pub(crate) type ClientWs = async_tungstenite::WebSocketStream<smol::net::TcpStream>;

/// Connect a WebSocket client to the bridge, optionally presenting an auth
/// token. Runs on the background reactor.
pub(crate) async fn connect(port: u16, token: Option<&str>) -> anyhow::Result<ClientWs> {
    let url = format!("ws://127.0.0.1:{port}");
    let addr = format!("127.0.0.1:{port}");
    let tcp = smol::net::TcpStream::connect(&addr).await?;
    let mut request = url.as_str().into_client_request()?;
    if let Some(token) = token {
        request
            .headers_mut()
            .insert("x-claude-code-ide-authorization", token.parse()?);
    }
    let (ws, _response) = async_tungstenite::client_async(request, tcp).await?;
    Ok(ws)
}

/// Read the next text frame and parse it as JSON.
pub(crate) async fn next_json(ws: &mut ClientWs) -> serde_json::Value {
    loop {
        match ws.next().await {
            Some(Ok(Message::Text(text))) => {
                return serde_json::from_str(&text).expect("valid JSON response");
            }
            Some(Ok(_)) => continue,
            other => panic!("expected a text frame, got {other:?}"),
        }
    }
}

#[gpui::test]
async fn rejects_missing_or_wrong_token(cx: &mut TestAppContext) {
    cx.executor().allow_parking();
    let token = "0123456789abcdef0123456789abcdef".to_string();
    let selection = SharedSelection::new();
    let mut async_cx = cx.to_async();
    let server = server::start(token.clone(), selection, &mut async_cx)
        .await
        .expect("server should start");
    let port = server.port();

    // Run all three handshake attempts on the background reactor; drive the
    // server's accept loop with run_until_parked.
    let token_clone = token.clone();
    let client = cx.background_executor.spawn(async move {
        let no_token = connect(port, None).await.is_ok();
        let wrong = connect(port, Some("ffffffffffffffffffffffffffffffff"))
            .await
            .is_ok();
        let correct = connect(port, Some(&token_clone)).await.is_ok();
        (no_token, wrong, correct)
    });
    cx.run_until_parked();
    let (no_token_ok, wrong_ok, correct_ok) = client.await;

    assert!(!no_token_ok, "connection without a token should be rejected");
    assert!(!wrong_ok, "connection with a wrong token should be rejected");
    assert!(correct_ok, "connection with the correct token should succeed");
}

#[gpui::test]
async fn initialize_and_list_tools_round_trip(cx: &mut TestAppContext) {
    cx.executor().allow_parking();
    let token = "abcdef0123456789abcdef0123456789".to_string();
    let selection = SharedSelection::new();
    let mut async_cx = cx.to_async();
    let server = server::start(token.clone(), selection, &mut async_cx)
        .await
        .expect("server should start");
    let port = server.port();

    // The whole client conversation runs on the background reactor and reports
    // the three responses back; run_until_parked drives the server between the
    // client's socket operations.
    let token_clone = token.clone();
    let client = cx.background_executor.spawn(async move {
        let mut ws = connect(port, Some(&token_clone))
            .await
            .expect("authorized connection");
        ws.send(Message::text(
            r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#,
        ))
        .await
        .expect("send initialize");
        let init = next_json(&mut ws).await;

        ws.send(Message::text(
            r#"{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}"#,
        ))
        .await
        .expect("send tools/list");
        let list = next_json(&mut ws).await;

        ws.send(Message::text(
            r#"{"jsonrpc":"2.0","id":3,"method":"no/such/method","params":{}}"#,
        ))
        .await
        .expect("send bogus");
        let err = next_json(&mut ws).await;

        (init, list, err)
    });
    cx.run_until_parked();
    let (init, list, err) = client.await;

    assert_eq!(init["id"], 1);
    assert_eq!(init["result"]["serverInfo"]["name"], "zed-claude-code-ide");
    assert_eq!(init["result"]["capabilities"]["tools"]["listChanged"], false);

    assert_eq!(list["id"], 2);
    let names: Vec<String> = list["result"]["tools"]
        .as_array()
        .expect("tools array")
        .iter()
        .map(|tool| tool["name"].as_str().unwrap_or_default().to_string())
        .collect();
    assert!(names.contains(&"getCurrentSelection".to_string()));
    assert!(names.contains(&"getWorkspaceFolders".to_string()));
    assert!(names.contains(&"executeCode".to_string()));

    assert_eq!(err["id"], 3);
    assert!(err["error"]["code"].is_number());
}

#[gpui::test]
async fn notify_broadcasts_to_all_connected_clients(cx: &mut TestAppContext) {
    // Two `claude` sessions in one workspace both connect to the single server.
    // A selection_changed broadcast must reach BOTH: the multi-connection fix.
    cx.executor().allow_parking();
    let token = "abcabcabcabcabcabcabcabcabcabcab".to_string();
    let selection = SharedSelection::new();
    let mut async_cx = cx.to_async();
    let server = server::start(token.clone(), selection, &mut async_cx)
        .await
        .expect("server should start");
    let port = server.port();

    // Connect two clients on the background reactor and complete their MCP
    // handshakes so their dispatch loops register the outgoing senders.
    let token_a = token.clone();
    let connect_both = cx.background_executor.spawn(async move {
        let mut a = connect(port, Some(&token_a)).await.expect("client A connects");
        let mut b = connect(port, Some(&token_a)).await.expect("client B connects");
        for ws in [&mut a, &mut b] {
            ws.send(Message::text(
                r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#,
            ))
            .await
            .expect("send initialize");
            let _ = next_json(ws).await;
        }
        (a, b)
    });
    cx.run_until_parked();
    let (mut a, mut b) = connect_both.await;

    // Fire the broadcast from the foreground, then let the I/O tasks flush it.
    server.notify(
        "selection_changed",
        serde_json::json!({ "text": "hello", "filePath": "/x" }),
    );
    cx.run_until_parked();

    // Both clients must receive the same notification frame.
    let received = cx.background_executor.spawn(async move {
        let first = next_json(&mut a).await;
        let second = next_json(&mut b).await;
        (first, second)
    });
    cx.run_until_parked();
    let (first, second) = received.await;

    for (label, frame) in [("A", &first), ("B", &second)] {
        assert_eq!(
            frame["method"], "selection_changed",
            "client {label} should receive selection_changed, got {frame}"
        );
        assert_eq!(
            frame["params"]["text"], "hello",
            "client {label} should receive the broadcast payload, got {frame}"
        );
        assert!(
            frame["id"].is_null(),
            "a notification carries no id, got {frame}"
        );
    }
}

#[gpui::test]
async fn call_tool_without_selection_is_empty(cx: &mut TestAppContext) {
    cx.executor().allow_parking();
    let token = "1111111111111111aaaaaaaaaaaaaaaa".to_string();
    let selection = SharedSelection::new();
    let mut async_cx = cx.to_async();
    let server = server::start(token.clone(), selection, &mut async_cx)
        .await
        .expect("server should start");
    let port = server.port();

    let token_clone = token.clone();
    let client = cx.background_executor.spawn(async move {
        let mut ws = connect(port, Some(&token_clone))
            .await
            .expect("authorized connection");
        // No workspace bound and no cached selection: getCurrentSelection
        // returns an empty object rather than erroring.
        ws.send(Message::text(
            r#"{"jsonrpc":"2.0","id":7,"method":"tools/call","params":{"name":"getCurrentSelection","arguments":{}}}"#,
        ))
        .await
        .expect("send tools/call");
        next_json(&mut ws).await
    });
    cx.run_until_parked();
    let response = client.await;

    assert_eq!(response["id"], 7);
    let text = response["result"]["content"][0]["text"]
        .as_str()
        .expect("text content");
    assert_eq!(text, "{}");
}
