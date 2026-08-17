use crate::jsonrpc::{HandleDispatchFrom, Handled, IntoHandled, JsonRpcResponse};

use crate::role::{HasPeer, Role, handle_incoming_dispatch};
use crate::{ConnectionTo, Dispatch, JsonRpcNotification, JsonRpcRequest, UntypedMessage};
// Types re-exported from crate root
use super::Responder;
use std::marker::PhantomData;
use std::ops::AsyncFnMut;

/// Null handler that accepts no messages.
#[derive(Debug)]
pub struct NullHandler;

impl Default for NullHandler {
    fn default() -> Self {
        Self
    }
}

impl<Counterpart: Role> HandleDispatchFrom<Counterpart> for NullHandler {
    fn describe_chain(&self) -> impl std::fmt::Debug {
        "(null)"
    }

    async fn handle_dispatch_from(
        &mut self,
        message: Dispatch,
        _cx: ConnectionTo<Counterpart>,
    ) -> Result<Handled<Dispatch>, crate::Error> {
        Ok(Handled::No {
            message,
            retry: false,
        })
    }
}

/// Handler for typed request messages
pub struct RequestHandler<
    Counterpart: Role,
    Peer: Role,
    Req: JsonRpcRequest = UntypedMessage,
    F = (),
    ToFut = (),
> {
    counterpart: Counterpart,
    peer: Peer,
    handler: F,
    to_future_hack: ToFut,
    phantom: PhantomData<fn(Req)>,
}

impl<Counterpart: Role, Peer: Role, Req: JsonRpcRequest, F, ToFut>
    RequestHandler<Counterpart, Peer, Req, F, ToFut>
{
    /// Creates a new request handler
    pub fn new(counterpart: Counterpart, peer: Peer, handler: F, to_future_hack: ToFut) -> Self {
        Self {
            counterpart,
            peer,
            handler,
            to_future_hack,
            phantom: PhantomData,
        }
    }
}

impl<Counterpart: Role, Peer: Role, Req, F, T, ToFut> HandleDispatchFrom<Counterpart>
    for RequestHandler<Counterpart, Peer, Req, F, ToFut>
where
    Counterpart: HasPeer<Peer>,
    Req: JsonRpcRequest,
    F: AsyncFnMut(
            Req,
            Responder<Req::Response>,
            ConnectionTo<Counterpart>,
        ) -> Result<T, crate::Error>
        + Send,
    T: crate::IntoHandled<(Req, Responder<Req::Response>)>,
    ToFut: Fn(
            &mut F,
            Req,
            Responder<Req::Response>,
            ConnectionTo<Counterpart>,
        ) -> crate::BoxFuture<'_, Result<T, crate::Error>>
        + Send
        + Sync,
{
    fn describe_chain(&self) -> impl std::fmt::Debug {
        std::any::type_name::<Req>()
    }

    async fn handle_dispatch_from(
        &mut self,
        dispatch: Dispatch,
        connection: ConnectionTo<Counterpart>,
    ) -> Result<Handled<Dispatch>, crate::Error> {
        handle_incoming_dispatch(
            self.counterpart.clone(),
            self.peer.clone(),
            dispatch,
            connection,
            async |dispatch, connection| {
                match dispatch {
                    Dispatch::Request(message, responder) => {
                        tracing::debug!(
                            request_type = std::any::type_name::<Req>(),
                            message = ?message,
                            "RequestHandler::handle_request"
                        );
                        if Req::matches_method(&message.method) {
                            match Req::parse_message(&message.method, &message.params) {
                                Ok(req) => {
                                    tracing::trace!(
                                        ?req,
                                        "RequestHandler::handle_request: parse completed"
                                    );
                                    let typed_responder = responder.cast();
                                    let result = (self.to_future_hack)(
                                        &mut self.handler,
                                        req,
                                        typed_responder,
                                        connection,
                                    )
                                    .await?;
                                    match result.into_handled() {
                                        Handled::Yes => Ok(Handled::Yes),
                                        Handled::No {
                                            message: (request, responder),
                                            retry,
                                        } => {
                                            // Handler returned the request back, convert to untyped
                                            let untyped = request.to_untyped_message()?;
                                            Ok(Handled::No {
                                                message: Dispatch::Request(
                                                    untyped,
                                                    responder.erase_to_json(),
                                                ),
                                                retry,
                                            })
                                        }
                                    }
                                }
                                Err(err) => {
                                    tracing::trace!(
                                        ?err,
                                        "RequestHandler::handle_request: parse errored"
                                    );
                                    Err(err)
                                }
                            }
                        } else {
                            tracing::trace!("RequestHandler::handle_request: method doesn't match");
                            Ok(Handled::No {
                                message: Dispatch::Request(message, responder),
                                retry: false,
                            })
                        }
                    }

                    Dispatch::Notification(..) | Dispatch::Response(..) => Ok(Handled::No {
                        message: dispatch,
                        retry: false,
                    }),
                }
            },
        )
        .await
    }
}

/// Handler for typed notification messages
pub struct NotificationHandler<
    Counterpart: Role,
    Peer: Role,
    Notif: JsonRpcNotification = UntypedMessage,
    F = (),
    ToFut = (),
> {
    counterpart: Counterpart,
    peer: Peer,
    handler: F,
    to_future_hack: ToFut,
    phantom: PhantomData<fn(Notif)>,
}

impl<Counterpart: Role, Peer: Role, Notif: JsonRpcNotification, F, ToFut>
    NotificationHandler<Counterpart, Peer, Notif, F, ToFut>
{
    /// Creates a new notification handler
    pub fn new(counterpart: Counterpart, peer: Peer, handler: F, to_future_hack: ToFut) -> Self {
        Self {
            counterpart,
            peer,
            handler,
            to_future_hack,
            phantom: PhantomData,
        }
    }
}

impl<Counterpart: Role, Peer: Role, Notif, F, T, ToFut> HandleDispatchFrom<Counterpart>
    for NotificationHandler<Counterpart, Peer, Notif, F, ToFut>
where
    Counterpart: HasPeer<Peer>,
    Notif: JsonRpcNotification,
    F: AsyncFnMut(Notif, ConnectionTo<Counterpart>) -> Result<T, crate::Error> + Send,
    T: crate::IntoHandled<(Notif, ConnectionTo<Counterpart>)>,
    ToFut: Fn(
            &mut F,
            Notif,
            ConnectionTo<Counterpart>,
        ) -> crate::BoxFuture<'_, Result<T, crate::Error>>
        + Send
        + Sync,
{
    fn describe_chain(&self) -> impl std::fmt::Debug {
        std::any::type_name::<Notif>()
    }

    async fn handle_dispatch_from(
        &mut self,
        dispatch: Dispatch,
        connection: ConnectionTo<Counterpart>,
    ) -> Result<Handled<Dispatch>, crate::Error> {
        handle_incoming_dispatch(
            self.counterpart.clone(),
            self.peer.clone(),
            dispatch,
            connection,
            async |dispatch, connection| {
                match dispatch {
                    Dispatch::Notification(message) => {
                        tracing::debug!(
                            request_type = std::any::type_name::<Notif>(),
                            message = ?message,
                            "NotificationHandler::handle_dispatch"
                        );
                        if Notif::matches_method(&message.method) {
                            match Notif::parse_message(&message.method, &message.params) {
                                Ok(notif) => {
                                    tracing::trace!(
                                        ?notif,
                                        "NotificationHandler::handle_notification: parse completed"
                                    );
                                    let result =
                                        (self.to_future_hack)(&mut self.handler, notif, connection)
                                            .await?;
                                    match result.into_handled() {
                                        Handled::Yes => Ok(Handled::Yes),
                                        Handled::No {
                                            message: (notification, _cx),
                                            retry,
                                        } => {
                                            // Handler returned the notification back, convert to untyped
                                            let untyped = notification.to_untyped_message()?;
                                            Ok(Handled::No {
                                                message: Dispatch::Notification(untyped),
                                                retry,
                                            })
                                        }
                                    }
                                }
                                Err(err) => {
                                    tracing::trace!(
                                        ?err,
                                        "NotificationHandler::handle_notification: parse errored"
                                    );
                                    Err(err)
                                }
                            }
                        } else {
                            tracing::trace!(
                                "NotificationHandler::handle_notification: method doesn't match"
                            );
                            Ok(Handled::No {
                                message: Dispatch::Notification(message),
                                retry: false,
                            })
                        }
                    }

                    Dispatch::Request(..) | Dispatch::Response(..) => Ok(Handled::No {
                        message: dispatch,
                        retry: false,
                    }),
                }
            },
        )
        .await
    }
}

/// Handler for typed requests, notifications, and matching responses.
pub struct MessageHandler<
    Counterpart: Role,
    Peer: Role,
    Req: JsonRpcRequest = UntypedMessage,
    Notif: JsonRpcNotification = UntypedMessage,
    F = (),
    ToFut = (),
> {
    counterpart: Counterpart,
    peer: Peer,
    handler: F,
    to_future_hack: ToFut,
    phantom: PhantomData<fn(Dispatch<Req, Notif>)>,
}

impl<Counterpart: Role, Peer: Role, Req: JsonRpcRequest, Notif: JsonRpcNotification, F, ToFut>
    MessageHandler<Counterpart, Peer, Req, Notif, F, ToFut>
{
    /// Creates a new message handler
    pub fn new(counterpart: Counterpart, peer: Peer, handler: F, to_future_hack: ToFut) -> Self {
        Self {
            counterpart,
            peer,
            handler,
            to_future_hack,
            phantom: PhantomData,
        }
    }
}

impl<Counterpart: Role, Peer: Role, Req: JsonRpcRequest, Notif: JsonRpcNotification, F, T, ToFut>
    HandleDispatchFrom<Counterpart> for MessageHandler<Counterpart, Peer, Req, Notif, F, ToFut>
where
    Counterpart: HasPeer<Peer>,
    F: AsyncFnMut(Dispatch<Req, Notif>, ConnectionTo<Counterpart>) -> Result<T, crate::Error>
        + Send,
    T: IntoHandled<Dispatch<Req, Notif>>,
    ToFut: Fn(
            &mut F,
            Dispatch<Req, Notif>,
            ConnectionTo<Counterpart>,
        ) -> crate::BoxFuture<'_, Result<T, crate::Error>>
        + Send
        + Sync,
{
    fn describe_chain(&self) -> impl std::fmt::Debug {
        format!(
            "({}, {})",
            std::any::type_name::<Req>(),
            std::any::type_name::<Notif>()
        )
    }

    async fn handle_dispatch_from(
        &mut self,
        dispatch: Dispatch,
        connection: ConnectionTo<Counterpart>,
    ) -> Result<Handled<Dispatch>, crate::Error> {
        handle_incoming_dispatch(
            self.counterpart.clone(),
            self.peer.clone(),
            dispatch,
            connection,
            async |dispatch, connection| match dispatch.into_typed_dispatch::<Req, Notif>()? {
                Ok(typed_dispatch) => {
                    let result =
                        (self.to_future_hack)(&mut self.handler, typed_dispatch, connection)
                            .await?;
                    match result.into_handled() {
                        Handled::Yes => Ok(Handled::Yes),
                        Handled::No {
                            message: Dispatch::Request(request, responder),
                            retry,
                        } => {
                            let untyped = request.to_untyped_message()?;
                            Ok(Handled::No {
                                message: Dispatch::Request(untyped, responder.erase_to_json()),
                                retry,
                            })
                        }
                        Handled::No {
                            message: Dispatch::Notification(notification),
                            retry,
                        } => {
                            let untyped = notification.to_untyped_message()?;
                            Ok(Handled::No {
                                message: Dispatch::Notification(untyped),
                                retry,
                            })
                        }
                        Handled::No {
                            message: Dispatch::Response(result, responder),
                            retry,
                        } => {
                            let method = responder.method();
                            let untyped_result = match result {
                                Ok(response) => response.into_json(method).map(Ok),
                                Err(err) => Ok(Err(err)),
                            }?;
                            Ok(Handled::No {
                                message: Dispatch::Response(
                                    untyped_result,
                                    responder.erase_to_json(),
                                ),
                                retry,
                            })
                        }
                    }
                }

                Err(dispatch) => Ok(Handled::No {
                    message: dispatch,
                    retry: false,
                }),
            },
        )
        .await
    }
}

/// Wraps a handler with an optional name for tracing/debugging.
pub struct NamedHandler<H> {
    name: Option<String>,
    handler: H,
}

impl<H> NamedHandler<H> {
    /// Creates a new named handler
    pub fn new(name: Option<String>, handler: H) -> Self {
        Self { name, handler }
    }
}

impl<Counterpart: Role, H: HandleDispatchFrom<Counterpart>> HandleDispatchFrom<Counterpart>
    for NamedHandler<H>
{
    fn describe_chain(&self) -> impl std::fmt::Debug {
        format!(
            "NamedHandler({:?}, {:?})",
            self.name,
            self.handler.describe_chain()
        )
    }

    async fn handle_dispatch_from(
        &mut self,
        message: Dispatch,
        connection: ConnectionTo<Counterpart>,
    ) -> Result<Handled<Dispatch>, crate::Error> {
        if let Some(name) = &self.name {
            crate::util::instrumented_with_connection_name(
                name.clone(),
                self.handler.handle_dispatch_from(message, connection),
            )
            .await
        } else {
            self.handler.handle_dispatch_from(message, connection).await
        }
    }
}

/// Chains two handlers together, trying the first handler and falling back to the second
pub struct ChainedHandler<H1, H2> {
    handler1: H1,
    handler2: H2,
}

impl<H1, H2> ChainedHandler<H1, H2> {
    /// Creates a new chain handler
    pub fn new(handler1: H1, handler2: H2) -> Self {
        Self { handler1, handler2 }
    }
}

impl<Counterpart: Role, H1, H2> HandleDispatchFrom<Counterpart> for ChainedHandler<H1, H2>
where
    H1: HandleDispatchFrom<Counterpart>,
    H2: HandleDispatchFrom<Counterpart>,
{
    fn describe_chain(&self) -> impl std::fmt::Debug {
        format!(
            "{:?}, {:?}",
            self.handler1.describe_chain(),
            self.handler2.describe_chain()
        )
    }

    async fn handle_dispatch_from(
        &mut self,
        message: Dispatch,
        connection: ConnectionTo<Counterpart>,
    ) -> Result<Handled<Dispatch>, crate::Error> {
        match self
            .handler1
            .handle_dispatch_from(message, connection.clone())
            .await?
        {
            Handled::Yes => Ok(Handled::Yes),
            Handled::No {
                message,
                retry: retry1,
            } => match self
                .handler2
                .handle_dispatch_from(message, connection)
                .await?
            {
                Handled::Yes => Ok(Handled::Yes),
                Handled::No {
                    message,
                    retry: retry2,
                } => Ok(Handled::No {
                    message,
                    retry: retry1 | retry2,
                }),
            },
        }
    }
}
