use agent_client_protocol::schema::v1::{
    ClientRequest, DeleteSessionRequest, DeleteSessionResponse,
};
use agent_client_protocol::{JsonRpcMessage, JsonRpcRequest, JsonRpcResponse};
use serde_json::json;

fn assert_request_response_pair<T: JsonRpcRequest<Response = DeleteSessionResponse>>() {}

#[test]
fn delete_session_request_has_jsonrpc_metadata() {
    let request = DeleteSessionRequest::new("sess_abc123");

    assert_eq!(request.method(), "session/delete");
    assert!(DeleteSessionRequest::matches_method("session/delete"));
    assert!(!DeleteSessionRequest::matches_method("session/close"));

    let untyped = request.to_untyped_message().unwrap();
    assert_eq!(untyped.method, "session/delete");
    assert_eq!(untyped.params, json!({ "sessionId": "sess_abc123" }));

    let parsed = DeleteSessionRequest::parse_message(
        "session/delete",
        &json!({ "sessionId": "sess_abc123" }),
    )
    .unwrap();
    assert_eq!(parsed.session_id.0.as_ref(), "sess_abc123");
}

#[test]
fn delete_session_participates_in_client_request_enum() {
    let request = ClientRequest::DeleteSessionRequest(DeleteSessionRequest::new("sess_abc123"));

    assert_eq!(request.method(), "session/delete");
    assert!(ClientRequest::matches_method("session/delete"));

    let parsed =
        ClientRequest::parse_message("session/delete", &json!({ "sessionId": "sess_abc123" }))
            .unwrap();
    assert!(matches!(parsed, ClientRequest::DeleteSessionRequest(_)));
}

#[test]
fn delete_session_response_round_trips_json() {
    let value = DeleteSessionResponse::new()
        .into_json("session/delete")
        .unwrap();
    assert_eq!(value, json!({}));

    let parsed = DeleteSessionResponse::from_value("session/delete", value).unwrap();
    assert_eq!(parsed, DeleteSessionResponse::new());

    assert_request_response_pair::<DeleteSessionRequest>();
}
