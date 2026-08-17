use agent_client_protocol::JsonRpcMessage;
use agent_client_protocol::schema::SuccessorMessage;
#[cfg(feature = "unstable_mcp_over_acp")]
use agent_client_protocol::schema::v1::MessageMcpRequest;
use agent_client_protocol::schema::v1::{
    ContentBlock, Meta, PromptRequest, SessionId, TextContent,
};
use serde_json::Value;

fn trace_context_meta() -> Meta {
    let mut meta = Meta::new();
    meta.insert(
        "traceparent".into(),
        Value::String("00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0b902b7-01".into()),
    );
    meta.insert(
        "tracestate".into(),
        Value::String("rojo=00f067aa0b902b7".into()),
    );
    meta.insert("baggage".into(), Value::String("tenant=acme".into()));
    meta
}

fn prompt_request(meta: Meta) -> PromptRequest {
    PromptRequest::new(
        SessionId::new("session-1"),
        vec![ContentBlock::Text(TextContent::new("hello"))],
    )
    .meta(meta)
}

fn trace_context_meta_value() -> Value {
    Value::Object(trace_context_meta())
}

#[test]
fn prompt_request_meta_round_trips_with_root_trace_context_keys()
-> Result<(), agent_client_protocol::Error> {
    let meta = trace_context_meta();
    let request = prompt_request(meta.clone());

    let untyped = request.to_untyped_message()?;

    assert_eq!(untyped.method(), "session/prompt");
    assert_eq!(untyped.params()["_meta"], Value::Object(meta.clone()));
    assert!(untyped.params().get("meta").is_none());

    let parsed = PromptRequest::parse_message(untyped.method(), untyped.params())?;
    assert_eq!(parsed.meta, Some(meta));

    Ok(())
}

#[test]
fn successor_message_meta_serializes_as_reserved_meta_field()
-> Result<(), agent_client_protocol::Error> {
    let envelope_meta = trace_context_meta_value();
    let inner_meta = trace_context_meta();
    let message = SuccessorMessage {
        message: prompt_request(inner_meta.clone()),
        meta: Some(envelope_meta.clone()),
    };

    let untyped = message.to_untyped_message()?;

    assert_eq!(untyped.method(), "_proxy/successor");
    assert_eq!(untyped.params()["_meta"], envelope_meta);
    assert_eq!(
        untyped.params()["params"]["_meta"],
        Value::Object(inner_meta.clone())
    );
    assert!(untyped.params().get("meta").is_none());

    let parsed =
        SuccessorMessage::<PromptRequest>::parse_message(untyped.method(), untyped.params())?;
    assert_eq!(parsed.meta, Some(envelope_meta));
    assert_eq!(parsed.message.meta, Some(inner_meta));

    Ok(())
}

#[test]
fn successor_message_accepts_legacy_meta_alias() -> Result<(), agent_client_protocol::Error> {
    let envelope_meta = trace_context_meta_value();
    let mut params = SuccessorMessage {
        message: prompt_request(Meta::new()),
        meta: Some(envelope_meta.clone()),
    }
    .to_untyped_message()?
    .params;

    let params_object = params.as_object_mut().expect("params should be an object");
    let meta = params_object
        .remove("_meta")
        .expect("successor metadata should be present");
    params_object.insert("meta".into(), meta);

    let parsed = SuccessorMessage::<PromptRequest>::parse_message("_proxy/successor", &params)?;
    assert_eq!(parsed.meta, Some(envelope_meta));

    Ok(())
}

#[test]
#[cfg(feature = "unstable_mcp_over_acp")]
fn native_mcp_over_acp_message_meta_serializes_as_reserved_meta_field()
-> Result<(), agent_client_protocol::Error> {
    let meta = trace_context_meta();
    let message = MessageMcpRequest::new("connection-1", "tools/list")
        .params(serde_json::Map::from_iter([(
            "cursor".into(),
            Value::String("abc".into()),
        )]))
        .meta(meta.clone());

    let untyped = message.to_untyped_message()?;

    assert_eq!(untyped.method(), "mcp/message");
    assert_eq!(untyped.params()["connectionId"], "connection-1");
    assert_eq!(untyped.params()["method"], "tools/list");
    assert_eq!(untyped.params()["params"]["cursor"], "abc");
    assert_eq!(untyped.params()["_meta"], Value::Object(meta.clone()));
    assert!(untyped.params().get("meta").is_none());

    let parsed = MessageMcpRequest::parse_message(untyped.method(), untyped.params())?;
    assert_eq!(parsed.meta, Some(meta));

    Ok(())
}
