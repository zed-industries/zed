#![cfg(feature = "unstable_elicitation")]

use agent_client_protocol::schema::v1::{
    AgentNotification, AgentRequest, ClientCapabilities, ClientResponse,
    CompleteElicitationNotification, CreateElicitationRequest, CreateElicitationResponse,
    ElicitationAction, ElicitationCapabilities, ElicitationFormCapabilities, ElicitationFormMode,
    ElicitationSchema, ElicitationSessionScope, ElicitationUrlCapabilities, Error,
};
use agent_client_protocol::{JsonRpcMessage, JsonRpcNotification, JsonRpcRequest, JsonRpcResponse};
use serde::Serialize;
use serde_json::{Value, json};

fn json_value(value: impl Serialize) -> Result<Value, Error> {
    serde_json::to_value(value).map_err(Error::into_internal_error)
}

fn form_request() -> CreateElicitationRequest {
    CreateElicitationRequest::new(
        ElicitationFormMode::new(
            ElicitationSessionScope::new("sess_abc123"),
            ElicitationSchema::new().string("name", true),
        ),
        "Please enter your name",
    )
}

fn assert_request_response_pair<T: JsonRpcRequest<Response = CreateElicitationResponse>>() {}
fn assert_notification<T: JsonRpcNotification>() {}

#[test]
fn create_elicitation_request_has_jsonrpc_metadata() {
    let request = form_request();

    assert_eq!(request.method(), "elicitation/create");
    assert!(CreateElicitationRequest::matches_method(
        "elicitation/create"
    ));
    assert!(!CreateElicitationRequest::matches_method("session/prompt"));

    let untyped = request.to_untyped_message().unwrap();
    assert_eq!(untyped.method, "elicitation/create");
    assert_eq!(untyped.params["mode"], "form");
    assert_eq!(untyped.params["sessionId"], "sess_abc123");

    let parsed =
        CreateElicitationRequest::parse_message("elicitation/create", &untyped.params).unwrap();
    assert!(matches!(
        parsed.mode,
        agent_client_protocol::schema::v1::ElicitationMode::Form(_)
    ));

    assert_request_response_pair::<CreateElicitationRequest>();
}

#[test]
fn elicitation_participates_in_agent_request_enum() {
    let request = AgentRequest::CreateElicitationRequest(form_request());

    assert_eq!(request.method(), "elicitation/create");
    assert!(AgentRequest::matches_method("elicitation/create"));

    let parsed =
        AgentRequest::parse_message("elicitation/create", &json_value(form_request()).unwrap())
            .unwrap();
    assert!(matches!(parsed, AgentRequest::CreateElicitationRequest(_)));
}

#[test]
fn create_elicitation_response_round_trips_json() {
    let value = CreateElicitationResponse::new(ElicitationAction::Decline)
        .into_json("elicitation/create")
        .unwrap();
    assert_eq!(value, json!({ "action": "decline" }));

    let parsed = CreateElicitationResponse::from_value("elicitation/create", value).unwrap();
    assert!(matches!(parsed.action, ElicitationAction::Decline));

    let enum_response =
        ClientResponse::from_value("elicitation/create", json!({ "action": "cancel" })).unwrap();
    assert!(matches!(
        enum_response,
        ClientResponse::CreateElicitationResponse(_)
    ));
}

#[test]
fn complete_elicitation_notification_has_jsonrpc_metadata() {
    assert_notification::<CompleteElicitationNotification>();

    let notification = CompleteElicitationNotification::new("elicit_1");
    assert_eq!(notification.method(), "elicitation/complete");
    assert!(CompleteElicitationNotification::matches_method(
        "elicitation/complete"
    ));
    assert!(!CompleteElicitationNotification::matches_method(
        "session/update"
    ));

    let untyped = notification.to_untyped_message().unwrap();
    assert_eq!(untyped.method, "elicitation/complete");
    assert_eq!(untyped.params, json!({ "elicitationId": "elicit_1" }));

    let parsed = AgentNotification::parse_message("elicitation/complete", &untyped.params).unwrap();
    assert!(matches!(
        parsed,
        AgentNotification::CompleteElicitationNotification(_)
    ));
}

#[test]
fn client_capabilities_can_declare_elicitation_modes() {
    let capabilities = ClientCapabilities::new().elicitation(
        ElicitationCapabilities::new()
            .form(ElicitationFormCapabilities::new())
            .url(ElicitationUrlCapabilities::new()),
    );

    let value = json_value(capabilities).unwrap();
    assert_eq!(value["elicitation"], json!({ "form": {}, "url": {} }));

    let parsed: ClientCapabilities = serde_json::from_value(json!({ "elicitation": {} })).unwrap();
    assert!(parsed.elicitation.is_some());
}

#[cfg(feature = "unstable_protocol_v2")]
#[test]
fn protocol_v2_elicitation_variants_are_jsonrpc_mapped() -> Result<(), Error> {
    use agent_client_protocol::schema::v2;

    let request = v2::CreateElicitationRequest::new(
        v2::ElicitationFormMode::new(
            v2::ElicitationSessionScope::new("sess_abc123"),
            v2::ElicitationSchema::new().string("name", true),
        ),
        "Please enter your name",
    );

    let parsed_request =
        v2::AgentRequest::parse_message("elicitation/create", &json_value(request.clone())?)?;
    assert!(matches!(
        parsed_request,
        v2::AgentRequest::CreateElicitationRequest(_)
    ));

    let parsed_response =
        v2::ClientResponse::from_value("elicitation/create", json!({ "action": "decline" }))?;
    assert!(matches!(
        parsed_response,
        v2::ClientResponse::CreateElicitationResponse(_)
    ));

    let notification = v2::CompleteElicitationNotification::new("elicit_1");
    let parsed_notification =
        v2::AgentNotification::parse_message("elicitation/complete", &json_value(notification)?)?;
    assert!(matches!(
        parsed_notification,
        v2::AgentNotification::CompleteElicitationNotification(_)
    ));

    Ok(())
}

#[cfg(feature = "unstable_protocol_v2")]
#[tokio::test(flavor = "current_thread")]
async fn v2_agent_can_elicit_from_v2_client_before_prompt_completion() -> Result<(), Error> {
    use agent_client_protocol::schema::{ProtocolVersion, v2};
    use agent_client_protocol::{Agent, Client};
    use std::collections::BTreeMap;

    fn v2_initialize_response_with_session(
        protocol_version: ProtocolVersion,
    ) -> v2::InitializeResponse {
        v2::InitializeResponse::new(
            protocol_version,
            v2::Implementation::new("schema-elicitation-test", env!("CARGO_PKG_VERSION")),
        )
        .capabilities(v2::AgentCapabilities::new().session(v2::SessionCapabilities::new()))
    }

    let agent = Agent
        .v2()
        .on_receive_request(
            async |initialize: v2::InitializeRequest, responder, _cx| {
                assert_eq!(initialize.protocol_version, ProtocolVersion::V2);
                responder.respond(v2_initialize_response_with_session(ProtocolVersion::V2))
            },
            agent_client_protocol::on_receive_request!(),
        )
        .on_receive_request(
            async |_prompt: v2::PromptRequest, responder, cx| {
                let request = v2::CreateElicitationRequest::new(
                    v2::ElicitationFormMode::new(
                        v2::ElicitationSessionScope::new("sess_abc123"),
                        v2::ElicitationSchema::new().string("name", true),
                    ),
                    "Please enter your name",
                );

                cx.send_request(request)
                    .on_receiving_result(async move |result| {
                        let response = result?;
                        let v2::ElicitationAction::Accept(action) = response.action else {
                            return Err(Error::invalid_params().data("expected accept action"));
                        };
                        let content = action.content.ok_or_else(|| {
                            Error::invalid_params().data("expected response content")
                        })?;
                        assert_eq!(
                            content.get("name"),
                            Some(&v2::ElicitationContentValue::String("Ada".into()))
                        );
                        responder.respond_with_error(Error::request_cancelled())
                    })?;

                Ok(())
            },
            agent_client_protocol::on_receive_request!(),
        );

    Client
        .v2()
        .on_receive_request(
            async |request: v2::CreateElicitationRequest, responder, _cx| {
                assert_eq!(request.method(), "elicitation/create");
                assert!(matches!(
                    request.mode,
                    v2::ElicitationMode::Form(v2::ElicitationFormMode { .. })
                ));

                let content = BTreeMap::from([(
                    "name".to_string(),
                    v2::ElicitationContentValue::from("Ada"),
                )]);
                responder.respond(v2::CreateElicitationResponse::new(
                    v2::ElicitationAction::Accept(
                        v2::ElicitationAcceptAction::new().content(content),
                    ),
                ))
            },
            agent_client_protocol::on_receive_request!(),
        )
        .connect_with(agent, async |cx| {
            let initialize = cx
                .send_request(v2::InitializeRequest::new(
                    ProtocolVersion::V2,
                    v2::Implementation::new("schema-elicitation-test", env!("CARGO_PKG_VERSION")),
                ))
                .block_task()
                .await?;
            assert_eq!(initialize.protocol_version, ProtocolVersion::V2);

            let error = cx
                .send_request(v2::PromptRequest::new(
                    "sess_abc123",
                    vec!["continue".into()],
                ))
                .block_task()
                .await
                .expect_err("test agent cancels the prompt after the elicitation round trip");
            assert_eq!(i32::from(error.code), -32800);
            Ok(())
        })
        .await
}
