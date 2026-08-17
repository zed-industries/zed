//! Role types for ACP connections.
//!
//! Roles represent the logical identity of an endpoint in an ACP connection.
//! Each role has a counterpart (who it connects to) and may have multiple peers
//! (who it can exchange messages with).

use std::{any::TypeId, fmt::Debug, future::Future, hash::Hash};

use serde::{Deserialize, Serialize};

use crate::schema::{METHOD_SUCCESSOR_MESSAGE, SuccessorMessage};
use crate::util::json_cast;
use crate::{Builder, ConnectionTo, Dispatch, Handled, JsonRpcMessage, UntypedMessage};

/// Roles for the ACP protocol.
pub mod acp;

/// Roles for the MCP protocol.
pub mod mcp;

/// The role that an endpoint plays in an ACP connection.
///
/// Roles are the fundamental building blocks of ACP's type system:
/// - [`acp::Client`] connects to [`acp::Agent`]
/// - [`acp::Agent`] connects to [`acp::Client`]
/// - [`acp::Proxy`] connects to [`acp::Conductor`]
/// - [`acp::Conductor`] connects to [`acp::Proxy`]
///
/// Each role determines:
/// - Who the counterpart is (via [`Role::Counterpart`])
/// - How unhandled messages are processed (via [`Role::default_handle_dispatch_from`])
pub trait Role: Debug + Clone + Send + Sync + 'static + Eq + Ord + Hash {
    /// The role that this endpoint connects to.
    ///
    /// For example:
    /// - `Client::Counterpart = Agent`
    /// - `Agent::Counterpart = Client`
    /// - `Proxy::Counterpart = Conductor`
    /// - `Conductor::Counterpart = Proxy`
    type Counterpart: Role<Counterpart = Self>;

    /// Creates a new builder playing this role.
    fn builder(self) -> Builder<Self>
    where
        Self: Sized,
    {
        Builder::new(self)
    }

    /// Returns a unique identifier for this role.
    fn role_id(&self) -> RoleId;

    /// Method invoked when there is no defined message handler.
    fn default_handle_dispatch_from(
        &self,
        message: Dispatch,
        connection: ConnectionTo<Self>,
    ) -> impl Future<Output = Result<Handled<Dispatch>, crate::Error>> + Send;

    /// Returns the counterpart role.
    fn counterpart(&self) -> Self::Counterpart;
}

/// Declares that a role can send messages to a specific peer.
///
/// Most roles only communicate with their counterpart, but some (like [`acp::Proxy`])
/// can communicate with multiple peers:
/// - `Proxy: HasPeer<Client>` - proxy can send/receive from clients
/// - `Proxy: HasPeer<Agent>` - proxy can send/receive from agents
/// - `Proxy: HasPeer<Conductor>` - proxy can send/receive from its conductor
///
/// The [`RemoteStyle`] determines how messages are transformed:
/// - [`RemoteStyle::Counterpart`] - pass through unchanged
/// - [`RemoteStyle::Predecessor`] - pass through, but reject wrapped messages
/// - [`RemoteStyle::Successor`] - wrap in a [`SuccessorMessage`] envelope
///
/// [`SuccessorMessage`]: crate::schema::SuccessorMessage
pub trait HasPeer<Peer: Role>: Role {
    /// Returns the remote style for sending to this peer.
    fn remote_style(&self, peer: Peer) -> RemoteStyle;
}

/// Describes how messages are transformed when sent to a remote peer.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum RemoteStyle {
    /// Pass each message through exactly as it is.
    Counterpart,

    /// Only messages not wrapped in successor.
    Predecessor,

    /// Wrap messages in a [`SuccessorMessage`] envelope.
    Successor,
}

impl RemoteStyle {
    pub(crate) fn transform_outgoing_message<M: JsonRpcMessage>(
        self,
        msg: M,
    ) -> Result<UntypedMessage, crate::Error> {
        match self {
            RemoteStyle::Counterpart | RemoteStyle::Predecessor => msg.to_untyped_message(),
            RemoteStyle::Successor => SuccessorMessage {
                message: msg,
                meta: None,
            }
            .to_untyped_message(),
        }
    }
}

/// Unique identifier for a role instance.
///
/// Used to identify the source/destination of messages when multiple
/// peers are possible on a single connection.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[non_exhaustive]
pub enum RoleId {
    /// Singleton role identified by type name and type ID.
    Singleton(&'static str, TypeId),
}

impl RoleId {
    /// Create the role ID for a singleton role type.
    pub fn from_singleton<R>(_role: &R) -> RoleId
    where
        R: Role + Default,
    {
        RoleId::Singleton(std::any::type_name::<R>(), TypeId::of::<R>())
    }
}

// ============================================================================
// Role implementations
// ============================================================================

pub(crate) async fn handle_incoming_dispatch<Counterpart, Peer>(
    counterpart: Counterpart,
    peer: Peer,
    dispatch: Dispatch,
    connection: ConnectionTo<Counterpart>,
    handle_dispatch: impl AsyncFnOnce(
        Dispatch,
        ConnectionTo<Counterpart>,
    ) -> Result<Handled<Dispatch>, crate::Error>,
) -> Result<Handled<Dispatch>, crate::Error>
where
    Counterpart: Role + HasPeer<Peer>,
    Peer: Role,
{
    tracing::trace!(
        method = %dispatch.method(),
        ?counterpart,
        ?peer,
        ?dispatch,
        "handle_incoming_dispatch: enter"
    );

    // Responses are different from other messages.
    //
    // For normal incoming messages, messages from non-default
    // peers are tagged with special method names and carry
    // special payload that have be "unwrapped".
    //
    // For responses, the payload is untouched. The response
    // carries an `id` and we use this `id` to look up information
    // on the request that was sent to determine which peer it was
    // directed at (and therefore which peer sent us the response).
    if let Dispatch::Response(_, router) = &dispatch {
        tracing::trace!(
            response_role_id = ?router.role_id(),
            peer_role_id = ?peer.role_id(),
            "handle_incoming_dispatch: response"
        );

        if router.role_id() == peer.role_id() {
            return handle_dispatch(dispatch, connection).await;
        }
        return Ok(Handled::No {
            message: dispatch,
            retry: false,
        });
    }

    // Handle other messages by looking at the 'remote style'
    let method = dispatch.method();
    match counterpart.remote_style(peer) {
        RemoteStyle::Counterpart => {
            // "Counterpart" is the default peer, no special checks required.
            tracing::trace!("handle_incoming_dispatch: Counterpart style, passing through");
            handle_dispatch(dispatch, connection).await
        }
        RemoteStyle::Predecessor => {
            // "Predecessor" is the default peer, no special checks required.
            tracing::trace!("handle_incoming_dispatch: Predecessor style, passing through");
            if method == METHOD_SUCCESSOR_MESSAGE {
                // Methods coming from the successor are not coming from
                // our counterpart.
                Ok(Handled::No {
                    message: dispatch,
                    retry: false,
                })
            } else {
                handle_dispatch(dispatch, connection).await
            }
        }
        RemoteStyle::Successor => {
            // Successor style means we have to look for a special method name.
            if method != METHOD_SUCCESSOR_MESSAGE {
                tracing::trace!(
                    method,
                    expected = METHOD_SUCCESSOR_MESSAGE,
                    "handle_incoming_dispatch: Successor style but method doesn't match, returning Handled::No"
                );
                return Ok(Handled::No {
                    message: dispatch,
                    retry: false,
                });
            }

            tracing::trace!(
                "handle_incoming_dispatch: Successor style, unwrapping SuccessorMessage"
            );

            // The outer message has method="_proxy/successor" and params containing the inner message.
            // We need to deserialize the params (not the whole message) to extract the inner UntypedMessage.
            let untyped_message = dispatch.message().ok_or_else(|| {
                crate::util::internal_error(
                    "Response variant cannot be unwrapped as SuccessorMessage",
                )
            })?;
            let SuccessorMessage { message, meta } = json_cast(untyped_message.params())?;
            let successor_dispatch = dispatch.try_map_message(|_| Ok(message))?;
            tracing::trace!(
                unwrapped_method = %successor_dispatch.method(),
                "handle_incoming_dispatch: unwrapped to inner message"
            );
            match handle_dispatch(successor_dispatch, connection).await? {
                Handled::Yes => {
                    tracing::trace!(
                        "handle_incoming_dispatch: inner handler returned Handled::Yes"
                    );
                    Ok(Handled::Yes)
                }

                Handled::No {
                    message: successor_dispatch,
                    retry,
                } => {
                    tracing::trace!(
                        "handle_incoming_dispatch: inner handler returned Handled::No, re-wrapping"
                    );
                    Ok(Handled::No {
                        message: successor_dispatch.try_map_message(|message| {
                            SuccessorMessage { message, meta }.to_untyped_message()
                        })?,
                        retry,
                    })
                }
            }
        }
    }
}

/// A dummy role you can use to exchange JSON-RPC messages without any knowledge of the underlying protocol.
/// Prefer a protocol-specific role when one is available.
#[derive(
    Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct UntypedRole;

impl UntypedRole {
    /// Creates a new builder for a connection from this role.
    pub fn builder(self) -> Builder<Self> {
        Builder::new(self)
    }
}

impl Role for UntypedRole {
    type Counterpart = UntypedRole;

    fn role_id(&self) -> RoleId {
        RoleId::from_singleton(self)
    }

    async fn default_handle_dispatch_from(
        &self,
        message: Dispatch,
        _connection: ConnectionTo<Self>,
    ) -> Result<Handled<Dispatch>, crate::Error> {
        Ok(Handled::No {
            message,
            retry: false,
        })
    }

    fn counterpart(&self) -> Self::Counterpart {
        *self
    }
}

impl HasPeer<UntypedRole> for UntypedRole {
    fn remote_style(&self, _peer: UntypedRole) -> RemoteStyle {
        RemoteStyle::Counterpart
    }
}
