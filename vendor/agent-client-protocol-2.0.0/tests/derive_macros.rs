//! Tests for the JsonRpcRequest, JsonRpcNotification, and JsonRpcResponse derive macros.

#![allow(non_upper_case_globals)]

use agent_client_protocol::{JsonRpcMessage, JsonRpcNotification, JsonRpcRequest, JsonRpcResponse};
use serde::{Deserialize, Serialize};

// ============================================================================
// Test types using derive macros
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, JsonRpcRequest)]
#[request(method = "_test/hello", response = HelloResponse)]
struct HelloRequest {
    name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonRpcResponse)]
struct HelloResponse {
    greeting: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonRpcNotification)]
#[notification(method = "_test/ping")]
struct PingNotification {
    timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonRpcResponse)]
struct GenericResponse<T>(T);

#[derive(Debug, Clone, Serialize, Deserialize, JsonRpcRequest)]
#[request(method = "_test/generic", response = GenericResponse<T>)]
struct GenericRequest<T>(T);

#[derive(Debug, Clone, Serialize, Deserialize, JsonRpcNotification)]
#[notification(method = "_test/generic-notification")]
struct GenericNotification<T>(T);

#[derive(Debug, Clone, Serialize, Deserialize, JsonRpcResponse)]
struct ConstGenericResponse<const value: usize>;

#[derive(Debug, Clone, Serialize, Deserialize, JsonRpcRequest)]
#[request(
    method = "_test/const-generic",
    response = ConstGenericResponse<method>
)]
struct ConstGenericRequest<const method: usize, const params: usize>;

#[derive(Debug, Clone, Serialize, Deserialize, JsonRpcNotification)]
#[notification(method = "_test/const-generic-notification")]
struct ConstGenericNotification<const __acp_method: usize, const __acp_params: usize>;

// ============================================================================
// Tests
// ============================================================================

#[test]
fn test_jr_request_method() {
    let req = HelloRequest {
        name: "world".into(),
    };
    assert_eq!(req.method(), "_test/hello");
}

#[test]
fn test_jr_request_to_untyped() {
    let req = HelloRequest {
        name: "world".into(),
    };
    let untyped = req.to_untyped_message().unwrap();
    assert_eq!(untyped.method(), "_test/hello");
}

#[test]
fn test_jr_request_parse_message() {
    let original = HelloRequest {
        name: "test".into(),
    };
    let untyped = original.to_untyped_message().unwrap();

    // matches_method should return true for correct method
    assert!(HelloRequest::matches_method(untyped.method()));
    assert!(!HelloRequest::matches_method("wrong/method"));

    // Parse should succeed for matching method
    let parsed = HelloRequest::parse_message(untyped.method(), untyped.params());
    assert!(parsed.is_ok());
    let parsed = parsed.unwrap();
    assert_eq!(parsed.name, "test");

    // Parse should return Err for non-matching method
    let wrong_method = HelloRequest::parse_message("wrong/method", untyped.params());
    assert!(wrong_method.is_err());
}

#[test]
fn test_jr_request_response_type() {
    // This is a compile-time check that the Response type is correctly set
    fn assert_response_type<R: JsonRpcRequest<Response = HelloResponse>>() {}
    assert_response_type::<HelloRequest>();
}

#[test]
fn test_generic_derives_add_conditional_bounds() {
    fn assert_request<R: JsonRpcRequest<Response = GenericResponse<u64>>>() {}
    fn assert_any_request<R: JsonRpcRequest>() {}
    fn assert_notification<N: JsonRpcNotification>() {}

    assert_request::<GenericRequest<u64>>();
    assert_notification::<GenericNotification<u64>>();

    assert_any_request::<ConstGenericRequest<1, 2>>();
    assert_notification::<ConstGenericNotification<1, 2>>();
}

#[test]
fn test_jr_notification_method() {
    let notif = PingNotification { timestamp: 12345 };
    assert_eq!(notif.method(), "_test/ping");
}

#[test]
fn test_jr_notification_to_untyped() {
    let notif = PingNotification { timestamp: 12345 };
    let untyped = notif.to_untyped_message().unwrap();
    assert_eq!(untyped.method(), "_test/ping");
}

#[test]
fn test_jr_notification_parse_message() {
    let original = PingNotification { timestamp: 99999 };
    let untyped = original.to_untyped_message().unwrap();

    // matches_method should work correctly
    assert!(PingNotification::matches_method(untyped.method()));
    assert!(!PingNotification::matches_method("wrong/method"));

    let parsed = PingNotification::parse_message(untyped.method(), untyped.params());
    assert!(parsed.is_ok());
    let parsed = parsed.unwrap();
    assert_eq!(parsed.timestamp, 99999);
}

#[test]
fn test_jr_response_payload_into_json() {
    let response = HelloResponse {
        greeting: "Hello, world!".into(),
    };
    let json = response.into_json("_test/hello").unwrap();
    assert_eq!(json["greeting"], "Hello, world!");
}

#[test]
fn test_jr_response_payload_from_value() {
    let json = serde_json::json!({
        "greeting": "Hi there!"
    });
    let response = HelloResponse::from_value("_test/hello", json).unwrap();
    assert_eq!(response.greeting, "Hi there!");
}

// ============================================================================
// Test that JsonRpcNotification is a marker trait
// ============================================================================

#[test]
fn test_jr_notification_is_marker() {
    fn assert_notification<N: agent_client_protocol::JsonRpcNotification>() {}
    assert_notification::<PingNotification>();
}
