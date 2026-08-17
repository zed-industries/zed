//! Utilities for pattern matching on untyped JSON-RPC messages.
//!
//! When handling [`UntypedMessage`]s, you can use [`MatchDispatch`] for simple parsing
//! or [`MatchDispatchFrom`] when you need peer-aware transforms (e.g., unwrapping
//! proxy envelopes).
//!
//! # When to use which
//!
//! - **[`MatchDispatchFrom`]**: Preferred over implementing [`HandleDispatchFrom`] directly.
//!   Use this in connection handlers when you need to match on message types with
//!   proper peer-aware transforms (e.g., unwrapping `SuccessorMessage` envelopes).
//!
//! - **[`MatchDispatch`]**: Use this when you already have an unwrapped message and
//!   just need to parse it, such as inside a [`MatchDispatchFrom`] callback or when
//!   processing messages that don't need peer transforms.
//!
//! [`HandleDispatchFrom`]: crate::HandleDispatchFrom

use crate::{
    ConnectionTo, Dispatch, HandleDispatchFrom, Handled, JsonRpcNotification, JsonRpcRequest,
    JsonRpcResponse, Responder, ResponseRouter, UntypedMessage,
    role::{HasPeer, Role, handle_incoming_dispatch},
};

fn preserve_retry(
    state: Result<Handled<Dispatch>, crate::Error>,
    prior_retry: bool,
) -> Result<Handled<Dispatch>, crate::Error> {
    state.map(|state| match state {
        Handled::Yes => Handled::Yes,
        Handled::No { message, retry } => Handled::No {
            message,
            retry: prior_retry | retry,
        },
    })
}

/// Role-agnostic helper for pattern-matching on untyped JSON-RPC messages.
///
/// Use this when you already have an unwrapped message and just need to parse it,
/// such as inside a [`MatchDispatchFrom`] callback or when processing messages
/// that don't need peer transforms.
///
/// For connection handlers where you need proper peer-aware transforms,
/// use [`MatchDispatchFrom`] instead.
///
/// # Example
///
/// ```
/// # use agent_client_protocol::Dispatch;
/// # use agent_client_protocol::schema::v1::{AgentCapabilities, InitializeRequest, InitializeResponse};
/// # use agent_client_protocol::util::MatchDispatch;
/// # async fn example(message: Dispatch) -> Result<(), agent_client_protocol::Error> {
/// MatchDispatch::new(message)
///     .if_request(|req: InitializeRequest, responder: agent_client_protocol::Responder<InitializeResponse>| async move {
///         let response = InitializeResponse::new(req.protocol_version)
///             .agent_capabilities(AgentCapabilities::new());
///         responder.respond(response)
///     })
///     .await
///     .otherwise(|message| async move {
///         match message {
///             Dispatch::Request(_, responder) => {
///                 responder.respond_with_error(agent_client_protocol::util::internal_error("unknown method"))
///             }
///             Dispatch::Notification(_) | Dispatch::Response(_, _) => Ok(()),
///         }
///     })
///     .await
/// # }
/// ```
#[must_use]
#[derive(Debug)]
pub struct MatchDispatch {
    state: Result<Handled<Dispatch>, crate::Error>,
}

impl MatchDispatch {
    /// Create a new pattern matcher for the given message.
    pub fn new(message: Dispatch) -> Self {
        Self {
            state: Ok(Handled::No {
                message,
                retry: false,
            }),
        }
    }

    /// Try to handle the message as a request of type `Req`.
    ///
    /// If the message can be parsed as `Req`, the handler `op` is called with the parsed
    /// request and a typed request context. If parsing fails or the message was already
    /// handled by a previous call, this has no effect.
    pub async fn if_request<Req: JsonRpcRequest, H>(
        mut self,
        op: impl AsyncFnOnce(Req, Responder<Req::Response>) -> Result<H, crate::Error>,
    ) -> Self
    where
        H: crate::IntoHandled<(Req, Responder<Req::Response>)>,
    {
        if let Ok(Handled::No {
            message: dispatch,
            retry,
        }) = self.state
        {
            self.state = match dispatch {
                Dispatch::Request(untyped_request, untyped_responder) => {
                    if Req::matches_method(untyped_request.method()) {
                        match Req::parse_message(untyped_request.method(), untyped_request.params())
                        {
                            Ok(typed_request) => {
                                let typed_responder = untyped_responder.cast();
                                match op(typed_request, typed_responder).await {
                                    Ok(result) => match result.into_handled() {
                                        Handled::Yes => Ok(Handled::Yes),
                                        Handled::No {
                                            message: (request, responder),
                                            retry: request_retry,
                                        } => match request.to_untyped_message() {
                                            Ok(untyped) => Ok(Handled::No {
                                                message: Dispatch::Request(
                                                    untyped,
                                                    responder.erase_to_json(),
                                                ),
                                                retry: retry | request_retry,
                                            }),
                                            Err(err) => Err(err),
                                        },
                                    },
                                    Err(err) => Err(err),
                                }
                            }
                            Err(err) => Err(err),
                        }
                    } else {
                        Ok(Handled::No {
                            message: Dispatch::Request(untyped_request, untyped_responder),
                            retry,
                        })
                    }
                }
                Dispatch::Notification(_) | Dispatch::Response(_, _) => Ok(Handled::No {
                    message: dispatch,
                    retry,
                }),
            };
        }
        self
    }

    /// Try to handle the message as a notification of type `N`.
    ///
    /// If the message can be parsed as `N`, the handler `op` is called with the parsed
    /// notification. If parsing fails or the message was already handled, this has no effect.
    pub async fn if_notification<N: JsonRpcNotification, H>(
        mut self,
        op: impl AsyncFnOnce(N) -> Result<H, crate::Error>,
    ) -> Self
    where
        H: crate::IntoHandled<N>,
    {
        if let Ok(Handled::No {
            message: dispatch,
            retry,
        }) = self.state
        {
            self.state = match dispatch {
                Dispatch::Notification(untyped_notification) => {
                    if N::matches_method(untyped_notification.method()) {
                        match N::parse_message(
                            untyped_notification.method(),
                            untyped_notification.params(),
                        ) {
                            Ok(typed_notification) => match op(typed_notification).await {
                                Ok(result) => match result.into_handled() {
                                    Handled::Yes => Ok(Handled::Yes),
                                    Handled::No {
                                        message: notification,
                                        retry: notification_retry,
                                    } => match notification.to_untyped_message() {
                                        Ok(untyped) => Ok(Handled::No {
                                            message: Dispatch::Notification(untyped),
                                            retry: retry | notification_retry,
                                        }),
                                        Err(err) => Err(err),
                                    },
                                },
                                Err(err) => Err(err),
                            },
                            Err(err) => Err(err),
                        }
                    } else {
                        Ok(Handled::No {
                            message: Dispatch::Notification(untyped_notification),
                            retry,
                        })
                    }
                }
                Dispatch::Request(_, _) | Dispatch::Response(_, _) => Ok(Handled::No {
                    message: dispatch,
                    retry,
                }),
            };
        }
        self
    }

    /// Try to handle the message as a typed `Dispatch<R, N>`.
    ///
    /// This attempts to parse the message as either request type `R` or notification type `N`,
    /// providing a typed `Dispatch` to the handler if successful.
    pub async fn if_dispatch<R: JsonRpcRequest, N: JsonRpcNotification, H>(
        mut self,
        op: impl AsyncFnOnce(Dispatch<R, N>) -> Result<H, crate::Error>,
    ) -> Self
    where
        H: crate::IntoHandled<Dispatch<R, N>>,
    {
        if let Ok(Handled::No {
            message: dispatch,
            retry,
        }) = self.state
        {
            self.state = match dispatch.into_typed_dispatch::<R, N>() {
                Ok(Ok(typed_dispatch)) => match op(typed_dispatch).await {
                    Ok(result) => match result.into_handled() {
                        Handled::Yes => Ok(Handled::Yes),
                        Handled::No {
                            message: typed_dispatch,
                            retry: message_retry,
                        } => {
                            let untyped = match typed_dispatch {
                                Dispatch::Request(request, responder) => {
                                    match request.to_untyped_message() {
                                        Ok(untyped) => {
                                            Dispatch::Request(untyped, responder.erase_to_json())
                                        }
                                        Err(err) => return Self { state: Err(err) },
                                    }
                                }
                                Dispatch::Notification(notification) => {
                                    match notification.to_untyped_message() {
                                        Ok(untyped) => Dispatch::Notification(untyped),
                                        Err(err) => return Self { state: Err(err) },
                                    }
                                }
                                Dispatch::Response(result, router) => {
                                    let method = router.method();
                                    let untyped_result = match result {
                                        Ok(response) => match response.into_json(method) {
                                            Ok(json) => Ok(json),
                                            Err(err) => return Self { state: Err(err) },
                                        },
                                        Err(err) => Err(err),
                                    };
                                    Dispatch::Response(untyped_result, router.erase_to_json())
                                }
                            };
                            Ok(Handled::No {
                                message: untyped,
                                retry: retry | message_retry,
                            })
                        }
                    },
                    Err(err) => Err(err),
                },
                Ok(Err(dispatch)) => Ok(Handled::No {
                    message: dispatch,
                    retry,
                }),
                Err(err) => Err(err),
            };
        }
        self
    }

    /// Try to handle the message as a response to a request of type `Req`.
    ///
    /// If the message is a `Response` variant and the method matches `Req`, the handler
    /// is called with the result (which may be `Ok` or `Err`) and a typed response context.
    /// Use this when you need to handle both success and error responses.
    ///
    /// For handling only successful responses, see [`if_ok_response_to`](Self::if_ok_response_to).
    pub async fn if_response_to<Req: JsonRpcRequest, H>(
        mut self,
        op: impl AsyncFnOnce(
            Result<Req::Response, crate::Error>,
            ResponseRouter<Req::Response>,
        ) -> Result<H, crate::Error>,
    ) -> Self
    where
        H: crate::IntoHandled<(
                Result<Req::Response, crate::Error>,
                ResponseRouter<Req::Response>,
            )>,
    {
        if let Ok(Handled::No {
            message: dispatch,
            retry,
        }) = self.state
        {
            self.state = match dispatch {
                Dispatch::Response(result, router) => {
                    // Check if the request type matches this method
                    if Req::matches_method(router.method()) {
                        // Method matches, parse the response
                        let typed_router: ResponseRouter<Req::Response> = router.cast();
                        let typed_result = match result {
                            Ok(value) => Req::Response::from_value(typed_router.method(), value),
                            Err(err) => Err(err),
                        };

                        match op(typed_result, typed_router).await {
                            Ok(handler_result) => match handler_result.into_handled() {
                                Handled::Yes => Ok(Handled::Yes),
                                Handled::No {
                                    message: (result, router),
                                    retry: response_retry,
                                } => {
                                    // Convert typed result back to untyped
                                    let untyped_result = match result {
                                        Ok(response) => response.into_json(router.method()),
                                        Err(err) => Err(err),
                                    };
                                    Ok(Handled::No {
                                        message: Dispatch::Response(
                                            untyped_result,
                                            router.erase_to_json(),
                                        ),
                                        retry: retry | response_retry,
                                    })
                                }
                            },
                            Err(err) => Err(err),
                        }
                    } else {
                        // Method doesn't match, return unhandled
                        Ok(Handled::No {
                            message: Dispatch::Response(result, router),
                            retry,
                        })
                    }
                }
                Dispatch::Request(_, _) | Dispatch::Notification(_) => Ok(Handled::No {
                    message: dispatch,
                    retry,
                }),
            };
        }
        self
    }

    /// Try to handle the message as a successful response to a request of type `Req`.
    ///
    /// If the message is a `Response` variant with an `Ok` result and the method matches `Req`,
    /// the handler is called with the parsed response and a typed response context.
    /// Error responses are passed through without calling the handler.
    ///
    /// This is a convenience wrapper around [`if_response_to`](Self::if_response_to) for the
    /// common case where you only care about successful responses.
    pub async fn if_ok_response_to<Req: JsonRpcRequest, H>(
        self,
        op: impl AsyncFnOnce(Req::Response, ResponseRouter<Req::Response>) -> Result<H, crate::Error>,
    ) -> Self
    where
        H: crate::IntoHandled<(Req::Response, ResponseRouter<Req::Response>)>,
    {
        self.if_response_to::<Req, _>(async move |result, router| match result {
            Ok(response) => {
                let handler_result = op(response, router).await?;
                match handler_result.into_handled() {
                    Handled::Yes => Ok(Handled::Yes),
                    Handled::No {
                        message: (resp, router),
                        retry,
                    } => Ok(Handled::No {
                        message: (Ok(resp), router),
                        retry,
                    }),
                }
            }
            Err(err) => Ok(Handled::No {
                message: (Err(err), router),
                retry: false,
            }),
        })
        .await
    }

    /// Complete matching, returning `Handled::No` if no match was found.
    pub fn done(self) -> Result<Handled<Dispatch>, crate::Error> {
        self.state
    }

    /// Handle messages that didn't match any previous handler.
    pub async fn otherwise(
        self,
        op: impl AsyncFnOnce(Dispatch) -> Result<(), crate::Error>,
    ) -> Result<(), crate::Error> {
        match self.state {
            Ok(Handled::Yes) => Ok(()),
            Ok(Handled::No { message, retry: _ }) => op(message).await,
            Err(err) => Err(err),
        }
    }

    /// Handle messages that didn't match any previous handler.
    pub fn otherwise_ignore(self) -> Result<(), crate::Error> {
        match self.state {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
}

/// Role-aware helper for pattern-matching on untyped JSON-RPC requests.
///
/// **Prefer this over implementing [`HandleDispatchFrom`] directly.** This provides
/// a more ergonomic API for matching on message types in connection handlers.
///
/// Use this when you need peer-aware transforms (e.g., unwrapping proxy envelopes)
/// before parsing messages. For simple parsing without peer awareness (e.g., inside
/// a callback), use [`MatchDispatch`] instead.
///
/// This wraps [`MatchDispatch`] and applies peer-specific message transformations
/// via `remote_style().handle_incoming_dispatch()` before delegating to `MatchDispatch`
/// for the actual parsing.
///
/// [`HandleDispatchFrom`]: crate::HandleDispatchFrom
///
/// # Example
///
/// ```
/// # use agent_client_protocol::Dispatch;
/// # use agent_client_protocol::schema::v1::{AgentCapabilities, InitializeRequest, InitializeResponse, PromptRequest, PromptResponse, StopReason};
/// # use agent_client_protocol::util::MatchDispatchFrom;
/// # async fn example(message: Dispatch, cx: &agent_client_protocol::ConnectionTo<agent_client_protocol::Client>) -> Result<(), agent_client_protocol::Error> {
/// MatchDispatchFrom::new(message, cx)
///     .if_request(|req: InitializeRequest, responder: agent_client_protocol::Responder<InitializeResponse>| async move {
///         // Handle initialization
///         let response = InitializeResponse::new(req.protocol_version)
///             .agent_capabilities(AgentCapabilities::new());
///         responder.respond(response)
///     })
///     .await
///     .if_request(|_req: PromptRequest, responder: agent_client_protocol::Responder<PromptResponse>| async move {
///         // Handle prompts
///         responder.respond(PromptResponse::new(StopReason::EndTurn))
///     })
///     .await
///     .otherwise(|message| async move {
///         // Fallback for unrecognized messages
///         match message {
///             Dispatch::Request(_, responder) => responder.respond_with_error(agent_client_protocol::util::internal_error("unknown method")),
///             Dispatch::Notification(_) | Dispatch::Response(_, _) => Ok(()),
///         }
///     })
///     .await
/// # }
/// ```
#[must_use]
#[derive(Debug)]
pub struct MatchDispatchFrom<Counterpart: Role> {
    state: Result<Handled<Dispatch>, crate::Error>,
    connection: ConnectionTo<Counterpart>,
}

impl<Counterpart: Role> MatchDispatchFrom<Counterpart> {
    /// Create a new pattern matcher for the given untyped request message.
    pub fn new(message: Dispatch, cx: &ConnectionTo<Counterpart>) -> Self {
        Self {
            state: Ok(Handled::No {
                message,
                retry: false,
            }),
            connection: cx.clone(),
        }
    }

    /// Try to handle the message as a request of type `Req`.
    ///
    /// If the message can be parsed as `Req`, the handler `op` is called with the parsed
    /// request and a typed request context. If parsing fails or the message was already
    /// handled by a previous `handle_if`, this call has no effect.
    ///
    /// The handler can return either `()` (which becomes `Handled::Yes`) or an explicit
    /// `Handled` value to control whether the message should be passed to the next handler.
    ///
    /// Returns `self` to allow chaining multiple `handle_if` calls.
    pub async fn if_request<Req: JsonRpcRequest, H>(
        self,
        op: impl AsyncFnOnce(Req, Responder<Req::Response>) -> Result<H, crate::Error>,
    ) -> Self
    where
        Counterpart: HasPeer<Counterpart>,
        H: crate::IntoHandled<(Req, Responder<Req::Response>)>,
    {
        let counterpart = self.connection.counterpart();
        self.if_request_from(counterpart, op).await
    }

    /// Try to handle the message as a request of type `Req` from a specific peer.
    ///
    /// This is similar to [`if_request`](Self::if_request), but first applies peer-specific
    /// message transformation (e.g., unwrapping `SuccessorMessage` envelopes when receiving
    /// from an agent via a proxy).
    ///
    /// # Parameters
    ///
    /// * `peer` - The peer the message is expected to come from
    /// * `op` - The handler to call if the message matches
    pub async fn if_request_from<Peer: Role, Req: JsonRpcRequest, H>(
        mut self,
        peer: Peer,
        op: impl AsyncFnOnce(Req, Responder<Req::Response>) -> Result<H, crate::Error>,
    ) -> Self
    where
        Counterpart: HasPeer<Peer>,
        H: crate::IntoHandled<(Req, Responder<Req::Response>)>,
    {
        if let Ok(Handled::No { message, retry }) = self.state {
            let state = handle_incoming_dispatch(
                self.connection.counterpart(),
                peer,
                message,
                self.connection.clone(),
                async |dispatch, _connection| {
                    // Delegate to MatchDispatch for parsing
                    MatchDispatch::new(dispatch).if_request(op).await.done()
                },
            )
            .await;
            self.state = preserve_retry(state, retry);
        }
        self
    }

    /// Try to handle the message as a notification of type `N`.
    ///
    /// If the message can be parsed as `N`, the handler `op` is called with the parsed
    /// notification and connection context. If parsing fails or the message was already
    /// handled by a previous `handle_if`, this call has no effect.
    ///
    /// The handler can return either `()` (which becomes `Handled::Yes`) or an explicit
    /// `Handled` value to control whether the message should be passed to the next handler.
    ///
    /// Returns `self` to allow chaining multiple `handle_if` calls.
    pub async fn if_notification<N: JsonRpcNotification, H>(
        self,
        op: impl AsyncFnOnce(N) -> Result<H, crate::Error>,
    ) -> Self
    where
        Counterpart: HasPeer<Counterpart>,
        H: crate::IntoHandled<N>,
    {
        let counterpart = self.connection.counterpart();
        self.if_notification_from(counterpart, op).await
    }

    /// Try to handle the message as a notification of type `N` from a specific peer.
    ///
    /// This is similar to [`if_notification`](Self::if_notification), but first applies peer-specific
    /// message transformation (e.g., unwrapping `SuccessorMessage` envelopes when receiving
    /// from an agent via a proxy).
    ///
    /// # Parameters
    ///
    /// * `peer` - The peer the message is expected to come from
    /// * `op` - The handler to call if the message matches
    pub async fn if_notification_from<Peer: Role, N: JsonRpcNotification, H>(
        mut self,
        peer: Peer,
        op: impl AsyncFnOnce(N) -> Result<H, crate::Error>,
    ) -> Self
    where
        Counterpart: HasPeer<Peer>,
        H: crate::IntoHandled<N>,
    {
        if let Ok(Handled::No { message, retry }) = self.state {
            let state = handle_incoming_dispatch(
                self.connection.counterpart(),
                peer,
                message,
                self.connection.clone(),
                async |dispatch, _connection| {
                    // Delegate to MatchDispatch for parsing
                    MatchDispatch::new(dispatch)
                        .if_notification(op)
                        .await
                        .done()
                },
            )
            .await;
            self.state = preserve_retry(state, retry);
        }
        self
    }

    /// Try to handle the message as a typed `Dispatch<Req, N>` from a specific peer.
    ///
    /// This is similar to [`MatchDispatch::if_dispatch`], but first applies peer-specific
    /// message transformation (e.g., unwrapping `SuccessorMessage` envelopes).
    ///
    /// # Parameters
    ///
    /// * `peer` - The peer the message is expected to come from
    /// * `op` - The handler to call if the message matches
    pub async fn if_dispatch_from<Peer: Role, Req: JsonRpcRequest, N: JsonRpcNotification, H>(
        mut self,
        peer: Peer,
        op: impl AsyncFnOnce(Dispatch<Req, N>) -> Result<H, crate::Error>,
    ) -> Self
    where
        Counterpart: HasPeer<Peer>,
        H: crate::IntoHandled<Dispatch<Req, N>>,
    {
        if let Ok(Handled::No { message, retry }) = self.state {
            let state = handle_incoming_dispatch(
                self.connection.counterpart(),
                peer,
                message,
                self.connection.clone(),
                async |dispatch, _connection| {
                    // Delegate to MatchDispatch for parsing
                    MatchDispatch::new(dispatch).if_dispatch(op).await.done()
                },
            )
            .await;
            self.state = preserve_retry(state, retry);
        }
        self
    }

    /// Try to handle the message as a response to a request of type `Req`.
    ///
    /// If the message is a `Response` variant and the method matches `Req`, the handler
    /// is called with the result (which may be `Ok` or `Err`) and a typed response context.
    ///
    /// Unlike requests and notifications, responses don't need peer-specific transforms
    /// (they don't have the `SuccessorMessage` envelope structure), so this method
    /// delegates directly to [`MatchDispatch::if_response_to`].
    pub async fn if_response_to<Req: JsonRpcRequest, H>(
        mut self,
        op: impl AsyncFnOnce(
            Result<Req::Response, crate::Error>,
            ResponseRouter<Req::Response>,
        ) -> Result<H, crate::Error>,
    ) -> Self
    where
        H: crate::IntoHandled<(
                Result<Req::Response, crate::Error>,
                ResponseRouter<Req::Response>,
            )>,
    {
        if let Ok(Handled::No { message, retry }) = self.state {
            let state = MatchDispatch::new(message)
                .if_response_to::<Req, H>(op)
                .await
                .done();
            self.state = preserve_retry(state, retry);
        }
        self
    }

    /// Try to handle the message as a successful response to a request of type `Req`.
    ///
    /// If the message is a `Response` variant with an `Ok` result and the method matches `Req`,
    /// the handler is called with the parsed response and a typed response context.
    /// Error responses are passed through without calling the handler.
    ///
    /// This is a convenience wrapper around [`if_response_to`](Self::if_response_to).
    pub async fn if_ok_response_to<Req: JsonRpcRequest, H>(
        self,
        op: impl AsyncFnOnce(Req::Response, ResponseRouter<Req::Response>) -> Result<H, crate::Error>,
    ) -> Self
    where
        Counterpart: HasPeer<Counterpart>,
        H: crate::IntoHandled<(Req::Response, ResponseRouter<Req::Response>)>,
    {
        let counterpart = self.connection.counterpart();
        self.if_ok_response_to_from::<Req, Counterpart, H>(counterpart, op)
            .await
    }

    /// Try to handle the message as a response to a request of type `Req` from a specific peer.
    ///
    /// If the message is a `Response` variant, the method matches `Req`, and the `role_id`
    /// matches the expected peer, the handler is called with the result and a typed response context.
    ///
    /// This is used to filter responses by the peer they came from, which is important
    /// in proxy scenarios where responses might arrive from multiple peers.
    pub async fn if_response_to_from<Req: JsonRpcRequest, Peer: Role, H>(
        mut self,
        peer: Peer,
        op: impl AsyncFnOnce(
            Result<Req::Response, crate::Error>,
            ResponseRouter<Req::Response>,
        ) -> Result<H, crate::Error>,
    ) -> Self
    where
        Counterpart: HasPeer<Peer>,
        H: crate::IntoHandled<(
                Result<Req::Response, crate::Error>,
                ResponseRouter<Req::Response>,
            )>,
    {
        if let Ok(Handled::No { message, retry }) = self.state {
            let state = handle_incoming_dispatch(
                self.connection.counterpart(),
                peer,
                message,
                self.connection.clone(),
                async |dispatch, _connection| {
                    // Delegate to MatchDispatch for parsing
                    MatchDispatch::new(dispatch)
                        .if_response_to::<Req, H>(op)
                        .await
                        .done()
                },
            )
            .await;
            self.state = preserve_retry(state, retry);
        }
        self
    }

    /// Try to handle the message as a successful response to a request of type `Req` from a specific peer.
    ///
    /// This is a convenience wrapper around [`if_response_to_from`](Self::if_response_to_from)
    /// for the common case where you only care about successful responses.
    pub async fn if_ok_response_to_from<Req: JsonRpcRequest, Peer: Role, H>(
        self,
        peer: Peer,
        op: impl AsyncFnOnce(Req::Response, ResponseRouter<Req::Response>) -> Result<H, crate::Error>,
    ) -> Self
    where
        Counterpart: HasPeer<Peer>,
        H: crate::IntoHandled<(Req::Response, ResponseRouter<Req::Response>)>,
    {
        self.if_response_to_from::<Req, _, _>(peer, async move |result, router| match result {
            Ok(response) => {
                let handler_result = op(response, router).await?;
                match handler_result.into_handled() {
                    Handled::Yes => Ok(Handled::Yes),
                    Handled::No {
                        message: (resp, router),
                        retry,
                    } => Ok(Handled::No {
                        message: (Ok(resp), router),
                        retry,
                    }),
                }
            }
            Err(err) => Ok(Handled::No {
                message: (Err(err), router),
                retry: false,
            }),
        })
        .await
    }

    /// Complete matching, returning `Handled::No` if no match was found.
    pub fn done(self) -> Result<Handled<Dispatch>, crate::Error> {
        match self.state {
            Ok(Handled::Yes) => Ok(Handled::Yes),
            Ok(Handled::No { message, retry }) => Ok(Handled::No { message, retry }),
            Err(err) => Err(err),
        }
    }

    /// Handle messages that didn't match any previous `handle_if` call.
    ///
    /// This is the fallback handler that receives the original untyped message if none
    /// of the typed handlers matched. You must call this method to complete the pattern
    /// matching chain and get the final result.
    pub async fn otherwise(
        self,
        op: impl AsyncFnOnce(Dispatch) -> Result<(), crate::Error>,
    ) -> Result<(), crate::Error> {
        match self.state {
            Ok(Handled::Yes) => Ok(()),
            Ok(Handled::No { message, retry: _ }) => op(message).await,
            Err(err) => Err(err),
        }
    }

    /// Handle messages that didn't match any previous `handle_if` call.
    ///
    /// This is the fallback handler that receives the original untyped message if none
    /// of the typed handlers matched. You must call this method to complete the pattern
    /// matching chain and get the final result.
    pub async fn otherwise_delegate(
        self,
        mut handler: impl HandleDispatchFrom<Counterpart>,
    ) -> Result<Handled<Dispatch>, crate::Error> {
        match self.state? {
            Handled::Yes => Ok(Handled::Yes),
            Handled::No {
                message,
                retry: outer_retry,
            } => match handler
                .handle_dispatch_from(message, self.connection.clone())
                .await?
            {
                Handled::Yes => Ok(Handled::Yes),
                Handled::No {
                    message,
                    retry: inner_retry,
                } => Ok(Handled::No {
                    message,
                    retry: inner_retry | outer_retry,
                }),
            },
        }
    }
}

/// Builder for pattern-matching on untyped JSON-RPC notifications.
///
/// Similar to [`MatchDispatch`] but specifically for notifications (fire-and-forget messages with no response).
///
/// # Pattern
///
/// The typical pattern is:
/// 1. Create a `TypeNotification` from an untyped message
/// 2. Chain `.handle_if()` calls for each type you want to try
/// 3. End with `.otherwise()` for messages that don't match any type
///
/// # Example
///
/// ```
/// # use agent_client_protocol::UntypedMessage;
/// # use agent_client_protocol::schema::v1::SessionNotification;
/// # use agent_client_protocol::util::TypeNotification;
/// # async fn example(message: UntypedMessage) -> Result<(), agent_client_protocol::Error> {
/// TypeNotification::new(message)
///     .handle_if(|notif: SessionNotification| async move {
///         // Handle session notifications
///         println!("Session update: {:?}", notif);
///         Ok(())
///     })
///     .await
///     .otherwise(|untyped: UntypedMessage| async move {
///         // Fallback for unrecognized notifications
///         println!("Unknown notification: {}", untyped.method);
///         Ok(())
///     })
///     .await
/// # }
/// ```
///
/// Since notifications don't expect responses, handlers only receive the parsed
/// notification (not a request context).
#[must_use]
#[derive(Debug)]
pub struct TypeNotification {
    state: Option<TypeNotificationState>,
}

#[derive(Debug)]
enum TypeNotificationState {
    Unhandled(String, serde_json::Value),
    Handled(Result<(), crate::Error>),
}

impl TypeNotification {
    /// Create a new pattern matcher for the given untyped notification message.
    pub fn new(request: UntypedMessage) -> Self {
        let UntypedMessage { method, params } = request;
        Self {
            state: Some(TypeNotificationState::Unhandled(method, params)),
        }
    }

    /// Try to handle the message as type `N`.
    ///
    /// If the message can be parsed as `N`, the handler `op` is called with the parsed
    /// notification. If parsing fails, the malformed matching notification is consumed
    /// and logged without invoking the handler or sending a response. If the message was
    /// already handled by a previous `handle_if`, this call has no effect.
    ///
    /// Returns `self` to allow chaining multiple `handle_if` calls.
    pub async fn handle_if<N: JsonRpcNotification>(
        mut self,
        op: impl AsyncFnOnce(N) -> Result<(), crate::Error>,
    ) -> Self {
        self.state = Some(match self.state.take().expect("valid state") {
            TypeNotificationState::Unhandled(method, params) => {
                if N::matches_method(&method) {
                    match N::parse_message(&method, &params) {
                        Ok(request) => TypeNotificationState::Handled(op(request).await),
                        Err(error) => {
                            tracing::warn!(
                                ?error,
                                method,
                                "Invalid notification params; ignoring notification without replying"
                            );
                            TypeNotificationState::Handled(Ok(()))
                        }
                    }
                } else {
                    TypeNotificationState::Unhandled(method, params)
                }
            }

            TypeNotificationState::Handled(err) => TypeNotificationState::Handled(err),
        });
        self
    }

    /// Handle messages that didn't match any previous `handle_if` call.
    ///
    /// This is the fallback handler that receives the original untyped message if none
    /// of the typed handlers matched. You must call this method to complete the pattern
    /// matching chain and get the final result.
    pub async fn otherwise(
        mut self,
        op: impl AsyncFnOnce(UntypedMessage) -> Result<(), crate::Error>,
    ) -> Result<(), crate::Error> {
        match self.state.take().expect("valid state") {
            TypeNotificationState::Unhandled(method, params) => {
                match UntypedMessage::new(&method, params) {
                    Ok(m) => op(m).await,
                    Err(error) => {
                        tracing::warn!(
                            ?error,
                            method,
                            "Invalid untyped notification; ignoring notification without replying"
                        );
                        Ok(())
                    }
                }
            }
            TypeNotificationState::Handled(r) => r,
        }
    }
}

#[cfg(test)]
mod type_notification_tests {
    use std::{cell::Cell, rc::Rc};

    use serde::{Deserialize, Serialize};

    use super::{TypeNotification, UntypedMessage};

    #[derive(Clone, Debug, Serialize, Deserialize)]
    struct TestNotification {
        required: String,
    }

    impl crate::JsonRpcMessage for TestNotification {
        fn matches_method(method: &str) -> bool {
            method == "test/notification"
        }

        fn method(&self) -> &'static str {
            "test/notification"
        }

        fn to_untyped_message(&self) -> Result<UntypedMessage, crate::Error> {
            UntypedMessage::new(self.method(), self)
        }

        fn parse_message(method: &str, params: &impl Serialize) -> Result<Self, crate::Error> {
            if !Self::matches_method(method) {
                return Err(crate::Error::method_not_found());
            }
            crate::util::json_cast_params(params)
        }
    }

    impl crate::JsonRpcNotification for TestNotification {}

    #[test]
    fn invalid_matching_notification_is_consumed_without_fallback() {
        futures::executor::block_on(async {
            let handler_called = Rc::new(Cell::new(false));
            let fallback_called = Rc::new(Cell::new(false));
            let handler_called_in_callback = Rc::clone(&handler_called);
            let fallback_called_in_callback = Rc::clone(&fallback_called);
            let message =
                UntypedMessage::new("test/notification", serde_json::json!({ "wrong": "field" }))
                    .expect("test notification should serialize");

            TypeNotification::new(message)
                .handle_if(move |_: TestNotification| async move {
                    handler_called_in_callback.set(true);
                    Ok(())
                })
                .await
                .otherwise(move |_| async move {
                    fallback_called_in_callback.set(true);
                    Ok(())
                })
                .await
                .expect("invalid notification should be ignored");

            assert!(!handler_called.get());
            assert!(!fallback_called.get());
        });
    }
}
