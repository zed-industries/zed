// Types re-exported from crate root
use futures::StreamExt as _;
use futures::channel::mpsc;
use futures::stream;
use futures_concurrency::stream::StreamExt as _;
use rustc_hash::FxHashMap;
use std::collections::VecDeque;
use uuid::Uuid;

use crate::Dispatch;
use crate::UntypedMessage;
use crate::jsonrpc::ConnectionTo;
use crate::jsonrpc::HandleConnectionClose;
use crate::jsonrpc::HandleDispatchFrom;
use crate::jsonrpc::HandlerErrorTarget;
use crate::jsonrpc::OutgoingMessage;
use crate::jsonrpc::PendingReplies;
use crate::jsonrpc::PendingReply;
use crate::jsonrpc::RawJsonRpcMessage;
use crate::jsonrpc::RawJsonRpcParams;
use crate::jsonrpc::RequestReplyTarget;
use crate::jsonrpc::Responder;
use crate::jsonrpc::ResponseDestination;
use crate::jsonrpc::ResponseDispatch;
use crate::jsonrpc::ResponseRouter;
use crate::jsonrpc::TransportBatchEntry;
use crate::jsonrpc::TransportFrame;
use crate::jsonrpc::dynamic_handler::DynHandleDispatchFrom;
use crate::jsonrpc::dynamic_handler::DynamicHandlerMessage;
use crate::jsonrpc::outgoing_actor::send_raw_message;
use crate::jsonrpc::protocol_compat::ProtocolCompat;
use crate::jsonrpc::{is_response_only_shape, raw_is_response_only_shape};

use crate::role::Role;
use crate::schema::v1::{RequestId, Response};

use super::Handled;

/// Handlers owned by the incoming protocol actor.
pub(super) struct IncomingHandlers<Message, Close> {
    messages: Message,
    close: Close,
}

impl<Message, Close> IncomingHandlers<Message, Close> {
    pub(super) fn new(messages: Message, close: Close) -> Self {
        Self { messages, close }
    }
}

/// Incoming protocol actor: The central dispatch loop for a connection.
///
/// This actor handles JSON-RPC protocol semantics:
/// - Routes responses to pending request awaiters
/// - Routes requests/notifications to registered handlers
/// - Converts RawJsonRpcMessage requests/notifications to UntypedMessage for handlers
///
/// This is the protocol layer - it has no knowledge of how messages arrived.
pub(super) async fn incoming_protocol_actor<Counterpart: Role>(
    counterpart: Counterpart,
    connection: &ConnectionTo<Counterpart>,
    transport_rx: mpsc::UnboundedReceiver<TransportFrame>,
    dynamic_handler_rx: mpsc::UnboundedReceiver<DynamicHandlerMessage<Counterpart>>,
    pending_replies: PendingReplies,
    handlers: IncomingHandlers<
        impl HandleDispatchFrom<Counterpart>,
        impl HandleConnectionClose<Counterpart>,
    >,
    protocol_compat: ProtocolCompat,
) -> Result<(), crate::Error> {
    let IncomingHandlers {
        messages: mut handler,
        close: on_close,
    } = handlers;

    // `merge` does not expose when one of its source streams ends. Preserve
    // transport EOF as an explicit event so the other, connection-internal
    // streams cannot hide it.
    let transport_with_close = futures::StreamExt::chain(
        transport_rx.map(IncomingProtocolMsg::Transport),
        stream::iter([IncomingProtocolMsg::TransportClosed]),
    );
    let mut my_rx =
        transport_with_close.merge(dynamic_handler_rx.map(IncomingProtocolMsg::DynamicHandler));

    let mut dynamic_handlers: FxHashMap<Uuid, Box<dyn DynHandleDispatchFrom<Counterpart>>> =
        FxHashMap::default();
    let mut pending_messages: Vec<Dispatch> = vec![];

    let request_cancellations = super::RequestCancellationRegistry::new();
    let mut on_close = Some(on_close);

    let mut queued_transport_messages = VecDeque::new();
    loop {
        let message_result = if let Some(message) = queued_transport_messages.pop_front() {
            message
        } else {
            let Some(message) = my_rx.next().await else {
                break;
            };
            message
        };
        tracing::trace!(message = ?message_result, actor = "incoming_protocol_actor");
        match message_result {
            IncomingProtocolMsg::TransportClosed => {
                connection.begin_incoming_close();
                let pending_reply_count = pending_replies.close_incoming();
                tracing::debug!(pending_reply_count, "Incoming transport closed");

                let callback_result = on_close
                    .take()
                    .expect("incoming transport close handled more than once")
                    .handle_connection_close(connection.clone())
                    .await;

                connection.finish_incoming_close();
                callback_result?;
            }

            IncomingProtocolMsg::DynamicHandler(message) => {
                handle_dynamic_handler_message(
                    message,
                    connection,
                    &mut dynamic_handlers,
                    &mut pending_messages,
                )
                .await?;
            }

            IncomingProtocolMsg::Transport(frame) => {
                let (entries, batch_completion) = frame_entries(frame);
                for (message, destination) in entries {
                    match message {
                        Ok(RawJsonRpcMessage::Request(request)) => {
                            tracing::trace!(method = %request.method, id = ?request.id, "Handling request");
                            let request_method = request.method.to_string();
                            let request_id = request.id.clone();
                            let destination = destination
                                .expect("every incoming request has a response destination");
                            match dispatch_from_message(
                                connection,
                                request.method,
                                request.params,
                                Some(request.id),
                                &protocol_compat,
                                &request_cancellations,
                                Some(destination.clone()),
                            ) {
                                Ok(dispatches) => {
                                    for dispatch in dispatches {
                                        dispatch_dispatch(
                                            counterpart.clone(),
                                            connection,
                                            dispatch,
                                            &mut dynamic_handlers,
                                            &mut handler,
                                            &mut pending_messages,
                                            &request_cancellations,
                                        )
                                        .await?;
                                    }
                                }
                                Err(error) => {
                                    handle_handler_error(
                                        connection,
                                        Some(HandlerErrorTarget::Request(RequestReplyTarget {
                                            id: request_id,
                                            method: request_method.clone(),
                                            destination,
                                        })),
                                        request_method,
                                        error,
                                    )?;
                                }
                            }
                        }
                        Ok(RawJsonRpcMessage::Notification(notification)) => {
                            tracing::trace!(method = %notification.method, "Handling notification");
                            let request_method = notification.method.to_string();
                            match dispatch_from_message(
                                connection,
                                notification.method,
                                notification.params,
                                None,
                                &protocol_compat,
                                &request_cancellations,
                                None,
                            ) {
                                Ok(dispatches) => {
                                    for dispatch in dispatches {
                                        dispatch_dispatch(
                                            counterpart.clone(),
                                            connection,
                                            dispatch,
                                            &mut dynamic_handlers,
                                            &mut handler,
                                            &mut pending_messages,
                                            &request_cancellations,
                                        )
                                        .await?;
                                    }
                                }
                                Err(error) => {
                                    handle_handler_error(connection, None, request_method, error)?;
                                }
                            }
                        }
                        Ok(RawJsonRpcMessage::Response(response)) => {
                            let (id, result) = match response {
                                Response::Result { id, result } => (id, Ok(result)),
                                Response::Error { id, error } => (id, Err(error)),
                            };

                            tracing::trace!(?id, "Handling response");
                            if let Some(pending_reply) = pending_replies.remove(&id) {
                                let result = protocol_compat
                                    .incoming_response(&pending_reply.method, result);
                                let (dispatch, response_dispatch) =
                                    dispatch_from_response(id, pending_reply, result);
                                dispatch_dispatch(
                                    counterpart.clone(),
                                    connection,
                                    dispatch,
                                    &mut dynamic_handlers,
                                    &mut handler,
                                    &mut pending_messages,
                                    &request_cancellations,
                                )
                                .await?;
                                if let Some(ack_rx) = response_dispatch.complete() {
                                    let _ = ack_rx.await;

                                    // Ordered callbacks may synchronously queue dynamic handlers
                                    // (notably session routing) before acknowledging the response.
                                    // Put a marker behind those registrations, then install every
                                    // preceding handler before dispatching the next transport entry.
                                    connection
                                        .dynamic_handler_tx
                                        .unbounded_send(DynamicHandlerMessage::Barrier)
                                        .map_err(crate::util::internal_error)?;

                                    loop {
                                        let Some(message) = my_rx.next().await else {
                                            return Err(crate::util::internal_error(
                                                "dynamic-handler stream closed before its barrier",
                                            ));
                                        };
                                        match message {
                                            IncomingProtocolMsg::DynamicHandler(
                                                DynamicHandlerMessage::Barrier,
                                            ) => break,
                                            IncomingProtocolMsg::DynamicHandler(message) => {
                                                handle_dynamic_handler_message(
                                                    message,
                                                    connection,
                                                    &mut dynamic_handlers,
                                                    &mut pending_messages,
                                                )
                                                .await?;
                                            }
                                            message @ (IncomingProtocolMsg::Transport(_)
                                            | IncomingProtocolMsg::TransportClosed) => {
                                                queued_transport_messages.push_back(message);
                                            }
                                        }
                                    }
                                }
                            } else {
                                tracing::warn!(
                                    ?id,
                                    "incoming_actor: received response for unknown id, no subscriber found"
                                );
                            }
                        }
                        Err(error) => {
                            tracing::warn!(
                                ?error,
                                "Invalid transport input, sending error response"
                            );
                            let destination = destination.expect(
                                "invalid request input outside a response batch has a destination",
                            );
                            send_raw_message(
                                &connection.message_tx,
                                OutgoingMessage::UncorrelatedErrorResponse { error, destination },
                            )?;
                        }
                    }
                }
                if let Some(completion) = batch_completion {
                    send_raw_message(
                        &connection.message_tx,
                        OutgoingMessage::BatchDispatchComplete { completion },
                    )?;
                }
            }
        }
    }
    Ok(())
}

async fn handle_dynamic_handler_message<Counterpart: Role>(
    message: DynamicHandlerMessage<Counterpart>,
    connection: &ConnectionTo<Counterpart>,
    dynamic_handlers: &mut FxHashMap<Uuid, Box<dyn DynHandleDispatchFrom<Counterpart>>>,
    pending_messages: &mut Vec<Dispatch>,
) -> Result<(), crate::Error> {
    match message {
        DynamicHandlerMessage::AddDynamicHandler(uuid, mut handler) => {
            // Before adding the new handler, give it a chance to process
            // any pending messages.
            let mut new_pending_messages = vec![];
            for pending_message in std::mem::take(pending_messages) {
                tracing::trace!(method = pending_message.method(), handler = ?handler.dyn_describe_chain(), "Retrying message");
                let reply_target = pending_message.handler_error_target();
                let handler_attempt = reply_target
                    .as_ref()
                    .and_then(|target| target.begin_handler_attempt(&connection.message_tx));
                let method = pending_message.method().to_string();
                match handler
                    .dyn_handle_dispatch_from(pending_message, connection.clone())
                    .await
                {
                    Ok(Handled::Yes) => {
                        tracing::trace!("Message handled");
                    }
                    Ok(Handled::No {
                        message: m,
                        retry: _,
                    }) => {
                        tracing::trace!(method = m.method(), handler = ?handler.dyn_describe_chain(), "Message not handled");
                        new_pending_messages.push(m);
                    }
                    Err(err) => {
                        tracing::warn!(?err, handler = ?handler.dyn_describe_chain(), "Dynamic handler errored on pending message");
                        handle_handler_error(connection, reply_target, method, err)?;
                    }
                }
                drop(handler_attempt);
            }
            *pending_messages = new_pending_messages;

            // Add handler so it will be used for future incoming messages.
            dynamic_handlers.insert(uuid, handler);
        }
        DynamicHandlerMessage::RemoveDynamicHandler(uuid) => {
            dynamic_handlers.remove(&uuid);
        }
        DynamicHandlerMessage::Barrier => {}
    }
    Ok(())
}

#[derive(Debug)]
enum IncomingProtocolMsg<Counterpart: Role> {
    Transport(TransportFrame),
    TransportClosed,
    DynamicHandler(DynamicHandlerMessage<Counterpart>),
}

fn frame_entries(
    frame: TransportFrame,
) -> (
    Vec<(
        Result<RawJsonRpcMessage, crate::Error>,
        Option<ResponseDestination>,
    )>,
    Option<super::BatchDispatchCompletion>,
) {
    let entries: Vec<Result<RawJsonRpcMessage, crate::Error>> = match frame {
        TransportFrame::Single(message) => {
            let destination = matches!(&message, RawJsonRpcMessage::Request(_))
                .then(ResponseDestination::individual);
            return (vec![(Ok(message), destination)], None);
        }
        TransportFrame::Malformed { raw, error } => {
            if raw_is_response_only_shape(&raw) {
                return (Vec::new(), None);
            }
            return (
                vec![(Err(error), Some(ResponseDestination::individual()))],
                None,
            );
        }
        TransportFrame::Batch(batch) => batch
            .into_entries()
            .filter_map(|entry| match entry {
                TransportBatchEntry::Message(message) => Some(Ok(message)),
                TransportBatchEntry::Malformed { raw, error } => {
                    (!is_response_only_shape(&raw)).then_some(Err(error))
                }
            })
            .collect(),
    };

    let response_count = entries
        .iter()
        .filter(|entry| message_requires_response(entry))
        .count();
    if response_count == 0 {
        return (
            entries.into_iter().map(|entry| (entry, None)).collect(),
            None,
        );
    }

    let (mut destinations, completion) = ResponseDestination::batch(response_count);
    (
        entries
            .into_iter()
            .map(|entry| {
                let destination = message_requires_response(&entry)
                    .then(|| destinations.next().expect("one destination per batch call"));
                (entry, destination)
            })
            .collect(),
        Some(completion),
    )
}

fn message_requires_response(message: &Result<RawJsonRpcMessage, crate::Error>) -> bool {
    matches!(message, Ok(RawJsonRpcMessage::Request(_)) | Err(_))
}

/// Dispatches a JSON-RPC request to the handler.
/// Report an error back to the server if it does not get handled.
fn dispatch_from_message<Counterpart: Role>(
    connection: &ConnectionTo<Counterpart>,
    method: std::sync::Arc<str>,
    params: Option<RawJsonRpcParams>,
    id: Option<RequestId>,
    protocol_compat: &ProtocolCompat,
    request_cancellations: &super::RequestCancellationRegistry,
    response_destination: Option<ResponseDestination>,
) -> Result<Vec<Dispatch>, crate::Error> {
    let message = UntypedMessage::new(&method, crate::jsonrpc::params_from_transport(params))
        .expect("well-formed JSON");

    if let Some(id) = id {
        let message = protocol_compat.incoming_message(message)?;
        let response_destination =
            response_destination.expect("incoming requests always have a response destination");
        Ok(vec![Dispatch::Request(
            message,
            Responder::new(
                connection.message_tx.clone(),
                method.to_string(),
                id,
                request_cancellations,
                response_destination,
            ),
        )])
    } else {
        debug_assert!(response_destination.is_none());
        Ok(protocol_compat
            .incoming_notification(message)?
            .into_iter()
            .map(Dispatch::Notification)
            .collect())
    }
}

/// Dispatches a JSON-RPC response through the handler chain.
///
/// This allows handlers to intercept and process responses before they reach
/// the awaiting code. The default behavior is to forward the response to the
/// local awaiter via the oneshot channel.
fn dispatch_from_response(
    id: RequestId,
    pending_reply: PendingReply,
    result: Result<serde_json::Value, crate::Error>,
) -> (Dispatch, ResponseDispatch) {
    let PendingReply {
        method,
        role_id,
        sender,
        cancellation_disarm,
        ordering,
    } = pending_reply;
    let response_dispatch = ResponseDispatch::default();

    // Create a Dispatch::Response with a ResponseRouter that routes to the oneshot
    let router = ResponseRouter::new(
        method.clone(),
        id.clone(),
        role_id,
        sender,
        cancellation_disarm,
        ordering,
        response_dispatch.clone(),
    );
    (Dispatch::Response(result, router), response_dispatch)
}

#[tracing::instrument(
    skip(connection, dispatch, dynamic_handlers, handler, pending_messages),
    fields(method = dispatch.method()),
    level = "trace",
)]
async fn dispatch_dispatch<Counterpart: Role>(
    counterpart: Counterpart,
    connection: &ConnectionTo<Counterpart>,
    mut dispatch: Dispatch,
    dynamic_handlers: &mut FxHashMap<Uuid, Box<dyn DynHandleDispatchFrom<Counterpart>>>,
    handler: &mut impl HandleDispatchFrom<Counterpart>,
    pending_messages: &mut Vec<Dispatch>,
    request_cancellations: &super::RequestCancellationRegistry,
) -> Result<(), crate::Error> {
    tracing::trace!(?dispatch, "dispatch_dispatch");

    let mut retry_any = false;

    let id = dispatch.id().cloned();
    let method = dispatch.method().to_string();
    let error_target = dispatch.handler_error_target();
    let _handler_attempt = error_target
        .as_ref()
        .and_then(|target| target.begin_handler_attempt(&connection.message_tx));

    match request_cancellations.cancel_if_requested(&dispatch) {
        Ok(true) => {
            tracing::debug!(?method, "Marked request as cancelled");
        }
        Ok(false) => {}
        Err(err) => {
            tracing::warn!(
                ?method,
                ?id,
                ?err,
                "Request cancellation notification errored"
            );
            return handle_handler_error(connection, error_target, method, err);
        }
    }

    // First, apply the handlers given by the user.
    tracing::trace!(handler = ?handler.describe_chain(), "Attempting handler chain");
    match handler
        .handle_dispatch_from(dispatch, connection.clone())
        .await
    {
        Ok(Handled::Yes) => {
            tracing::trace!(?method, ?id, handler = ?handler.describe_chain(), "Handler accepted message");
            return Ok(());
        }

        Ok(Handled::No { message: m, retry }) => {
            tracing::trace!(?method, ?id, handler = ?handler.describe_chain(), "Handler declined message");
            dispatch = m;
            retry_any |= retry;
        }

        Err(err) => {
            tracing::warn!(?method, ?id, ?err, handler = ?handler.describe_chain(), "Handler errored");
            return handle_handler_error(connection, error_target, method, err);
        }
    }

    // Next, apply any dynamic handlers.
    for dynamic_handler in dynamic_handlers.values_mut() {
        tracing::trace!(handler = ?dynamic_handler.dyn_describe_chain(), "Attempting dynamic handler");
        match dynamic_handler
            .dyn_handle_dispatch_from(dispatch, connection.clone())
            .await
        {
            Ok(Handled::Yes) => {
                tracing::trace!(?method, ?id, handler = ?dynamic_handler.dyn_describe_chain(), "Dynamic handler accepted message");
                return Ok(());
            }

            Ok(Handled::No { message: m, retry }) => {
                tracing::trace!(?method, ?id, handler = ?dynamic_handler.dyn_describe_chain(),  "Dynamic handler declined message");
                retry_any |= retry;
                dispatch = m;
            }

            Err(err) => {
                tracing::warn!(?method, ?id, ?err, handler = ?dynamic_handler.dyn_describe_chain(), "Dynamic handler errored");
                return handle_handler_error(connection, error_target, method, err);
            }
        }
    }

    // Finally, apply the default handler for the role.
    tracing::trace!(role = ?counterpart, "Attempting default handler");
    match counterpart
        .default_handle_dispatch_from(dispatch, connection.clone())
        .await
    {
        Ok(Handled::Yes) => {
            tracing::trace!(?method, handler = "default", "Role accepted message");
            return Ok(());
        }
        Ok(Handled::No { message: m, retry }) => {
            tracing::trace!(?method, handler = "default", "Role declined message");
            dispatch = m;
            retry_any |= retry;
        }
        Err(err) => {
            tracing::warn!(
                ?method,
                ?id,
                ?err,
                handler = "default",
                "Default handler errored"
            );
            return handle_handler_error(connection, error_target, method, err);
        }
    }

    // If the message was never handled, check whether the retry flag was set.
    // If so, enqueue it for later processing. Else, reject it.
    //
    // An explicit retry request takes precedence over the unhandled-message
    // fallback below, so that handlers may defer notifications to a dynamic
    // handler that has not been registered yet.
    if retry_any {
        tracing::debug!(
            ?method,
            "Retrying message as new dynamic handlers are added"
        );
        pending_messages.push(dispatch);
        Ok(())
    } else {
        match dispatch {
            Dispatch::Notification(_) => {
                tracing::debug!(?method, "Ignoring unhandled notification");
                Ok(())
            }
            Dispatch::Request(_, responder) => {
                tracing::info!(?method, "Rejecting request with error, no handler");
                responder.respond_with_error(crate::Error::method_not_found().data(method))
            }
            Dispatch::Response(result, router) => {
                tracing::trace!(?method, "Forwarding response");
                router.route_with_result(result)
            }
        }
    }
}

/// Handle a message-processing error without tearing down the connection.
///
/// For requests, sends a JSON-RPC error response to the request's exact output
/// destination. For responses, routes the error to the local request awaiter.
/// Notification errors are logged without replying.
fn handle_handler_error<Counterpart: Role>(
    connection: &ConnectionTo<Counterpart>,
    target: Option<HandlerErrorTarget>,
    method: String,
    error: crate::Error,
) -> Result<(), crate::Error> {
    match target {
        Some(HandlerErrorTarget::Request(reply_target)) => send_raw_message(
            &connection.message_tx,
            OutgoingMessage::Response {
                id: reply_target.id,
                method: reply_target.method,
                response: Err(error),
                destination: reply_target.destination,
            },
        ),
        Some(HandlerErrorTarget::Response(reply_target)) => {
            reply_target.route(Err(error));
            Ok(())
        }
        None => {
            tracing::warn!(
                %method,
                ?error,
                "Ignoring message-processing error because there is no request to answer"
            );
            Ok(())
        }
    }
}
