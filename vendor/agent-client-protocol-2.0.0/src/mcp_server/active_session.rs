use std::sync::Arc;

use futures::channel::mpsc;
use futures::{SinkExt, StreamExt};
use rustc_hash::FxHashMap;
use serde_json::{Map, Value};

use crate::mcp_server::{McpConnectionContext, McpConnectionTo, McpServerConnect};
use crate::role;
use crate::role::HasPeer;
use crate::schema::v1::{
    ConnectMcpRequest, ConnectMcpResponse, DisconnectMcpRequest, DisconnectMcpResponse,
    McpConnectionId, McpServerAcpId, MessageMcpNotification, MessageMcpRequest, MessageMcpResponse,
};
use crate::util::MatchDispatchFrom;
use crate::{
    Agent, Channel, ConnectTo, ConnectionTo, Dispatch, HandleDispatchFrom, Handled,
    JsonRpcResponse, Responder, Role, UntypedMessage,
};

/// The message handler for an MCP server offered to a particular session.
/// This is added as a dynamic handler to the connection context and handles
/// native MCP-over-ACP messages for the declared server ID.
pub(super) struct McpActiveSession<Counterpart: Role> {
    /// The opaque ACP transport identifier for this MCP server.
    server_id: McpServerAcpId,

    /// The MCP server we are managing.
    mcp_connect: Arc<dyn McpServerConnect<Counterpart>>,

    /// Active connections to MCP server tasks.
    connections: FxHashMap<McpConnectionId, mpsc::Sender<Dispatch>>,
}

impl<Counterpart: Role> McpActiveSession<Counterpart>
where
    Counterpart: HasPeer<Agent>,
{
    pub fn new(
        server_id: McpServerAcpId,
        mcp_connect: Arc<dyn McpServerConnect<Counterpart>>,
    ) -> Self {
        Self {
            server_id,
            mcp_connect,
            connections: FxHashMap::default(),
        }
    }

    /// Handle a connection request for our MCP server by creating a new MCP connection.
    fn handle_connect_request(
        &mut self,
        request: ConnectMcpRequest,
        responder: Responder<ConnectMcpResponse>,
        acp_connection: &ConnectionTo<Counterpart>,
    ) -> Result<Handled<(ConnectMcpRequest, Responder<ConnectMcpResponse>)>, crate::Error> {
        if request.server_id != self.server_id {
            return Ok(Handled::No {
                message: (request, responder),
                retry: false,
            });
        }

        let connection_id =
            McpConnectionId::new(format!("mcp-over-acp-connection:{}", uuid::Uuid::new_v4()));
        let (mcp_server_tx, mut mcp_server_rx) = mpsc::channel(128);
        self.connections
            .insert(connection_id.clone(), mcp_server_tx);

        let (client_channel, server_channel) = Channel::duplex();

        let client_component = {
            let connection_id = connection_id.clone();
            let acp_connection = acp_connection.clone();

            role::mcp::Client
                .builder()
                .on_receive_dispatch(
                    async move |message: Dispatch, _mcp_connection| match message {
                        Dispatch::Request(request, responder) => {
                            let (method, params) = request.into_parts();
                            let params = match into_native_params(params) {
                                Ok(params) => params,
                                Err(error) => return responder.respond_with_error(error),
                            };
                            let request = MessageMcpRequest::new(connection_id.clone(), method)
                                .params(params);
                            let responder = responder.wrap_params(|method, result| {
                                result.and_then(|response: MessageMcpResponse| {
                                    response.into_json(method)
                                })
                            });
                            let message: Dispatch<MessageMcpRequest, MessageMcpNotification> =
                                Dispatch::Request(request, responder);
                            acp_connection.send_proxied_message_to(Agent, message)
                        }
                        Dispatch::Notification(notification) => {
                            let (method, params) = notification.into_parts();
                            let params = match into_native_params(params) {
                                Ok(params) => params,
                                Err(error) => {
                                    tracing::warn!(
                                        ?error,
                                        "ignoring MCP notification with positional parameters"
                                    );
                                    return Ok(());
                                }
                            };
                            let notification =
                                MessageMcpNotification::new(connection_id.clone(), method)
                                    .params(params);
                            let message: Dispatch<MessageMcpRequest, MessageMcpNotification> =
                                Dispatch::Notification(notification);
                            acp_connection.send_proxied_message_to(Agent, message)
                        }
                        Dispatch::Response(result, router) => router.route_with_result(result),
                    },
                    crate::on_receive_dispatch!(),
                )
                .with_spawned(move |mcp_connection| async move {
                    // These messages were sent by the ACP agent. Forward them to the MCP server.
                    while let Some(message) = mcp_server_rx.next().await {
                        mcp_connection.send_proxied_message_to(role::mcp::Server, message)?;
                    }
                    Ok(())
                })
        };

        let spawned_server = self.mcp_connect.connect(McpConnectionTo {
            context: McpConnectionContext::Acp {
                server_id: request.server_id.clone(),
                connection_id: connection_id.clone(),
            },
            connection: acp_connection.clone(),
        });

        let spawn_results = acp_connection
            .spawn(async move { client_component.connect_to(client_channel).await })
            .and_then(|()| {
                acp_connection.spawn(async move { spawned_server.connect_to(server_channel).await })
            });

        match spawn_results {
            Ok(()) => {
                responder.respond(ConnectMcpResponse::new(connection_id))?;
                Ok(Handled::Yes)
            }
            Err(error) => {
                self.connections.remove(&connection_id);
                responder.respond_with_error(error)?;
                Ok(Handled::Yes)
            }
        }
    }

    /// Forward a native MCP-over-ACP request to its MCP connection.
    async fn handle_mcp_over_acp_request(
        &mut self,
        request: MessageMcpRequest,
        responder: Responder<MessageMcpResponse>,
    ) -> Result<Handled<(MessageMcpRequest, Responder<MessageMcpResponse>)>, crate::Error> {
        let Some(mcp_server_tx) = self.connections.get_mut(&request.connection_id) else {
            return Ok(Handled::No {
                message: (request, responder),
                retry: false,
            });
        };

        let message = UntypedMessage {
            method: request.method.clone(),
            params: native_params_into_value(request.params.clone()),
        };
        let responder = responder.wrap_params(|method, result| {
            result.and_then(|response: Value| MessageMcpResponse::from_value(method, response))
        });
        mcp_server_tx
            .send(Dispatch::Request(message, responder))
            .await
            .map_err(crate::Error::into_internal_error)?;

        Ok(Handled::Yes)
    }

    /// Forward a native MCP-over-ACP notification to its MCP connection.
    async fn handle_mcp_over_acp_notification(
        &mut self,
        notification: MessageMcpNotification,
    ) -> Result<Handled<MessageMcpNotification>, crate::Error> {
        let Some(mcp_server_tx) = self.connections.get_mut(&notification.connection_id) else {
            return Ok(Handled::No {
                message: notification,
                retry: false,
            });
        };

        let message = UntypedMessage {
            method: notification.method.clone(),
            params: native_params_into_value(notification.params.clone()),
        };
        mcp_server_tx
            .send(Dispatch::Notification(message))
            .await
            .map_err(crate::Error::into_internal_error)?;

        Ok(Handled::Yes)
    }

    /// Disconnect an active native MCP-over-ACP connection.
    fn handle_mcp_disconnect_request(
        &mut self,
        request: DisconnectMcpRequest,
        responder: Responder<DisconnectMcpResponse>,
    ) -> Result<Handled<(DisconnectMcpRequest, Responder<DisconnectMcpResponse>)>, crate::Error>
    {
        if self.connections.remove(&request.connection_id).is_none() {
            return Ok(Handled::No {
                message: (request, responder),
                retry: false,
            });
        }

        responder.respond(DisconnectMcpResponse::new())?;
        Ok(Handled::Yes)
    }
}

impl<Counterpart: Role> HandleDispatchFrom<Counterpart> for McpActiveSession<Counterpart>
where
    Counterpart: HasPeer<Agent>,
{
    fn describe_chain(&self) -> impl std::fmt::Debug {
        "McpServerSession"
    }

    async fn handle_dispatch_from(
        &mut self,
        message: Dispatch,
        connection: ConnectionTo<Counterpart>,
    ) -> Result<Handled<Dispatch>, crate::Error> {
        MatchDispatchFrom::new(message, &connection)
            .if_request_from(Agent, async |request, responder| {
                self.handle_connect_request(request, responder, &connection)
            })
            .await
            .if_request_from(Agent, async |request, responder| {
                self.handle_mcp_over_acp_request(request, responder).await
            })
            .await
            .if_notification_from(Agent, async |notification| {
                self.handle_mcp_over_acp_notification(notification).await
            })
            .await
            .if_request_from(Agent, async |request, responder| {
                self.handle_mcp_disconnect_request(request, responder)
            })
            .await
            .done()
    }
}

fn into_native_params(params: Value) -> Result<Option<Map<String, Value>>, crate::Error> {
    match params {
        Value::Null => Ok(None),
        Value::Object(params) => Ok(Some(params)),
        Value::Array(_) => Err(crate::Error::invalid_params()
            .data("MCP-over-ACP only supports named inner MCP parameters")),
        _ => {
            Err(crate::Error::invalid_params()
                .data("inner MCP parameters must be an object or null"))
        }
    }
}

fn native_params_into_value(params: Option<Map<String, Value>>) -> Value {
    params.map_or(Value::Null, Value::Object)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{into_native_params, native_params_into_value};

    #[test]
    fn native_mcp_params_round_trip_objects_and_null() {
        let object = json!({ "name": "echo", "arguments": {} });
        let params = into_native_params(object.clone()).expect("object params should be valid");
        assert_eq!(native_params_into_value(params), object);

        let params = into_native_params(serde_json::Value::Null)
            .expect("omitted params should be represented as null");
        assert_eq!(native_params_into_value(params), serde_json::Value::Null);
    }

    #[test]
    fn native_mcp_params_reject_positional_params() {
        let error = into_native_params(json!(["positional"]))
            .expect_err("native MCP-over-ACP cannot represent positional params");
        assert_eq!(error.code, crate::ErrorCode::InvalidParams);
    }
}
