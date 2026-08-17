use agent_client_protocol::Role;
use agent_client_protocol::role::UntypedRole;
use agent_client_protocol::util::MatchDispatch;
use agent_client_protocol::{
    ConnectTo, ConnectionTo, Dispatch, HandleDispatchFrom, Handled, JsonRpcMessage,
    JsonRpcNotification, JsonRpcRequest, JsonRpcResponse, Responder, util::MatchDispatchFrom,
};
use futures::channel::oneshot;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EchoRequestResponse {
    text: Vec<String>,
}

impl JsonRpcMessage for EchoRequestResponse {
    fn matches_method(method: &str) -> bool {
        method == "echo"
    }

    fn method(&self) -> &'static str {
        "echo"
    }

    fn to_untyped_message(
        &self,
    ) -> Result<agent_client_protocol::UntypedMessage, agent_client_protocol::Error> {
        Ok(agent_client_protocol::UntypedMessage {
            method: self.method().to_string(),
            params: agent_client_protocol::util::json_cast(self)?,
        })
    }

    fn parse_message(
        method: &str,
        params: &impl serde::Serialize,
    ) -> Result<Self, agent_client_protocol::Error> {
        if !<Self as JsonRpcMessage>::matches_method(method) {
            return Err(agent_client_protocol::Error::method_not_found());
        }
        agent_client_protocol::util::json_cast(params)
    }
}

impl JsonRpcResponse for EchoRequestResponse {
    fn into_json(self, _method: &str) -> Result<serde_json::Value, agent_client_protocol::Error> {
        agent_client_protocol::util::json_cast(self)
    }

    fn from_value(
        _method: &str,
        value: serde_json::Value,
    ) -> Result<Self, agent_client_protocol::Error> {
        agent_client_protocol::util::json_cast(value)
    }
}

impl JsonRpcRequest for EchoRequestResponse {
    type Response = EchoRequestResponse;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RetryNotification;

impl JsonRpcMessage for RetryNotification {
    fn matches_method(method: &str) -> bool {
        method == "retry"
    }

    fn method(&self) -> &'static str {
        "retry"
    }

    fn to_untyped_message(
        &self,
    ) -> Result<agent_client_protocol::UntypedMessage, agent_client_protocol::Error> {
        agent_client_protocol::UntypedMessage::new(self.method(), self)
    }

    fn parse_message(
        method: &str,
        params: &impl serde::Serialize,
    ) -> Result<Self, agent_client_protocol::Error> {
        if !Self::matches_method(method) {
            return Err(agent_client_protocol::Error::method_not_found());
        }
        agent_client_protocol::util::json_cast_params(params)
    }
}

impl JsonRpcNotification for RetryNotification {}

struct EchoHandler;

impl<Counterpart: Role> HandleDispatchFrom<Counterpart> for EchoHandler {
    async fn handle_dispatch_from(
        &mut self,
        message: Dispatch,
        _connection: ConnectionTo<Counterpart>,
    ) -> Result<Handled<Dispatch>, agent_client_protocol::Error> {
        MatchDispatch::new(message)
            .if_request(async move |request: EchoRequestResponse, responder| {
                responder.respond(request)
            })
            .await
            .done()
    }

    fn describe_chain(&self) -> impl std::fmt::Debug {
        "TestHandler"
    }
}

#[tokio::test]
async fn match_dispatch_from_preserves_retry_across_chained_matches()
-> Result<(), agent_client_protocol::Error> {
    struct RetryObserver {
        retry_tx: Option<oneshot::Sender<bool>>,
    }

    impl HandleDispatchFrom<UntypedRole> for RetryObserver {
        async fn handle_dispatch_from(
            &mut self,
            message: Dispatch,
            connection: ConnectionTo<UntypedRole>,
        ) -> Result<Handled<Dispatch>, agent_client_protocol::Error> {
            let state = MatchDispatchFrom::new(message, &connection)
                .if_notification(async |notification: RetryNotification| {
                    Ok(Handled::No {
                        message: notification,
                        retry: true,
                    })
                })
                .await
                .if_request(
                    async |_request: EchoRequestResponse,
                           _responder: Responder<EchoRequestResponse>| {
                        Ok(Handled::Yes)
                    },
                )
                .await
                .if_notification(async |notification: RetryNotification| {
                    Ok(Handled::No {
                        message: notification,
                        retry: false,
                    })
                })
                .await
                .if_dispatch_from(UntypedRole, async |message: Dispatch| {
                    Ok(Handled::No {
                        message,
                        retry: false,
                    })
                })
                .await
                .if_response_to::<EchoRequestResponse, _>(async |_result, _router| Ok(Handled::Yes))
                .await
                .if_response_to_from::<EchoRequestResponse, _, _>(
                    UntypedRole,
                    async |_result, _router| Ok(Handled::Yes),
                )
                .await
                .done()?;

            let retry = matches!(state, Handled::No { retry: true, .. });
            if let Some(retry_tx) = self.retry_tx.take()
                && retry_tx.send(retry).is_err()
            {
                return Err(agent_client_protocol::Error::internal_error()
                    .data("retry observer receiver dropped"));
            }

            Ok(Handled::Yes)
        }

        fn describe_chain(&self) -> impl std::fmt::Debug {
            "RetryObserver"
        }
    }

    struct TestComponent {
        retry_tx: oneshot::Sender<bool>,
    }

    impl ConnectTo<UntypedRole> for TestComponent {
        async fn connect_to(
            self,
            peer: impl ConnectTo<UntypedRole>,
        ) -> Result<(), agent_client_protocol::Error> {
            UntypedRole
                .builder()
                .with_handler(RetryObserver {
                    retry_tx: Some(self.retry_tx),
                })
                .connect_to(peer)
                .await
        }
    }

    let (retry_tx, retry_rx) = oneshot::channel();
    UntypedRole
        .builder()
        .connect_with(TestComponent { retry_tx }, async |connection| {
            connection.send_notification(RetryNotification)?;
            let retry = retry_rx
                .await
                .map_err(agent_client_protocol::Error::into_internal_error)?;
            assert!(retry, "a later matcher discarded the prior retry flag");
            Ok(())
        })
        .await
}

#[tokio::test]
async fn modify_message_en_route() -> Result<(), agent_client_protocol::Error> {
    // Demonstrate a case where we modify a message
    // using a `HandleDispatchFrom` invoked from `MatchDispatch`

    struct TestComponent;

    impl ConnectTo<UntypedRole> for TestComponent {
        async fn connect_to(
            self,
            client: impl ConnectTo<UntypedRole>,
        ) -> Result<(), agent_client_protocol::Error> {
            UntypedRole
                .builder()
                .with_handler(PushHandler {
                    message: "b".to_string(),
                })
                .with_handler(EchoHandler)
                .connect_to(client)
                .await
        }
    }

    struct PushHandler {
        message: String,
    }

    impl HandleDispatchFrom<UntypedRole> for PushHandler {
        async fn handle_dispatch_from(
            &mut self,
            message: Dispatch,
            cx: ConnectionTo<UntypedRole>,
        ) -> Result<Handled<Dispatch>, agent_client_protocol::Error> {
            MatchDispatchFrom::new(message, &cx)
                .if_request(async move |mut request: EchoRequestResponse, responder| {
                    request.text.push(self.message.clone());
                    Ok(Handled::No {
                        message: (request, responder),
                        retry: false,
                    })
                })
                .await
                .done()
        }

        fn describe_chain(&self) -> impl std::fmt::Debug {
            "TestHandler"
        }
    }

    UntypedRole
        .builder()
        .connect_with(TestComponent, async |cx| {
            let result = cx
                .send_request(EchoRequestResponse {
                    text: vec!["a".to_string()],
                })
                .block_task()
                .await?;

            expect_test::expect![[r#"
                EchoRequestResponse {
                    text: [
                        "a",
                        "b",
                    ],
                }
            "#]]
            .assert_debug_eq(&result);
            Ok(())
        })
        .await
}

#[tokio::test]
async fn modify_message_en_route_inline() -> Result<(), agent_client_protocol::Error> {
    // Demonstrate a case where we modify a message en route using an `on_receive_request` call

    struct TestComponent;

    impl ConnectTo<UntypedRole> for TestComponent {
        async fn connect_to(
            self,
            client: impl ConnectTo<UntypedRole>,
        ) -> Result<(), agent_client_protocol::Error> {
            UntypedRole
                .builder()
                .on_receive_request(
                    async move |mut request: EchoRequestResponse,
                                responder: Responder<EchoRequestResponse>,
                                _connection: ConnectionTo<UntypedRole>| {
                        request.text.push("b".to_string());
                        Ok(Handled::No {
                            message: (request, responder),
                            retry: false,
                        })
                    },
                    agent_client_protocol::on_receive_request!(),
                )
                .with_handler(EchoHandler)
                .connect_to(client)
                .await
        }
    }

    UntypedRole
        .builder()
        .connect_with(TestComponent, async |cx| {
            let result = cx
                .send_request(EchoRequestResponse {
                    text: vec!["a".to_string()],
                })
                .block_task()
                .await?;

            expect_test::expect![[r#"
                EchoRequestResponse {
                    text: [
                        "a",
                        "b",
                    ],
                }
            "#]]
            .assert_debug_eq(&result);
            Ok(())
        })
        .await
}

#[tokio::test]
async fn modify_message_and_stop() -> Result<(), agent_client_protocol::Error> {
    // Demonstrate a case where we have an async handler that just returns `()`
    // in front (and hence we never see the `'b`).

    struct TestComponent;

    impl ConnectTo<UntypedRole> for TestComponent {
        async fn connect_to(
            self,
            client: impl ConnectTo<UntypedRole>,
        ) -> Result<(), agent_client_protocol::Error> {
            UntypedRole
                .builder()
                .on_receive_request(
                    async move |request: EchoRequestResponse,
                                responder: Responder<EchoRequestResponse>,
                                _connection: ConnectionTo<UntypedRole>| {
                        responder.respond(request)
                    },
                    agent_client_protocol::on_receive_request!(),
                )
                .on_receive_request(
                    async move |mut request: EchoRequestResponse,
                                responder: Responder<EchoRequestResponse>,
                                _connection: ConnectionTo<UntypedRole>| {
                        request.text.push("b".to_string());
                        Ok(Handled::No {
                            message: (request, responder),
                            retry: false,
                        })
                    },
                    agent_client_protocol::on_receive_request!(),
                )
                .with_handler(EchoHandler)
                .connect_to(client)
                .await
        }
    }

    UntypedRole
        .builder()
        .connect_with(TestComponent, async |cx| {
            let result = cx
                .send_request(EchoRequestResponse {
                    text: vec!["a".to_string()],
                })
                .block_task()
                .await?;

            expect_test::expect![[r#"
                EchoRequestResponse {
                    text: [
                        "a",
                    ],
                }
            "#]]
            .assert_debug_eq(&result);
            Ok(())
        })
        .await
}
