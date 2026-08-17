//! ConnectTo abstraction for agents and proxies.
//!
//! This module provides the [`ConnectTo`] trait that defines the interface for things
//! that can be run as part of a conductor's chain - agents, proxies, or any ACP-speaking component.
//!
//! ## Usage
//!
//! Components connect to other components, creating a chain of message processors.
//! The type parameter `R` is the role that this component connects to (its counterpart).
//!
//! To implement a component, implement the `connect_to` method:
//!
//! ```rust
//! use agent_client_protocol::{Agent, Client, ConnectTo, Result};
//!
//! struct MyAgent;
//!
//! // An agent connects to clients
//! impl ConnectTo<Client> for MyAgent {
//!     async fn connect_to(self, client: impl ConnectTo<Agent>) -> Result<()> {
//!         Agent.builder()
//!             .name("my-agent")
//!             .connect_to(client)
//!             .await
//!     }
//! }
//! ```

use futures::future::BoxFuture;
use std::{fmt::Debug, future::Future, marker::PhantomData};

use crate::{Channel, Result, role::Role};

/// A component that can exchange JSON-RPC messages to an endpoint playing the role `R`
/// (e.g., an ACP [`Agent`](`crate::role::acp::Agent`) or an MCP [`Server`](`crate::role::mcp::Server`)).
///
/// This trait represents anything that can communicate via JSON-RPC messages over channels -
/// agents, proxies, in-process connections, or any ACP-speaking component.
///
/// The type parameter `R` is the role that this component connects to (its counterpart).
/// For example:
/// - An agent implements `ConnectTo<Client>` to connect to clients
/// - A proxy implements `ConnectTo<Conductor>` to connect to conductors
/// - Transports like `Channel` implement `ConnectTo<R>` for every `R` because they are role-agnostic
///
/// # Component Types
///
/// The trait is implemented by several built-in types representing different communication patterns:
///
/// - **[`ByteStreams`]**: A component communicating over byte streams (stdin/stdout, sockets, etc.)
/// - **[`Channel`]**: A component communicating via in-process message channels (for testing or direct connections)
/// - **[`AcpAgent`]**: An external agent running in a separate process with stdio communication
/// - **Custom components**: Proxies, transformers, or any ACP-aware service
///
/// # Two Ways to Connect
///
/// Components can be used in two ways:
///
/// 1. **`connect_to(client)`** - Connect directly to another component (most components implement this)
/// 2. **`into_channel_and_future()`** - Obtain a channel endpoint and a future that drives the connection
///
/// Most components only need to implement `connect_to(client)`. The
/// `into_channel_and_future()` method has a default implementation that creates an intermediate
/// channel and calls `connect_to`.
///
/// # Implementation Example
///
/// ```rust
/// use agent_client_protocol::{Agent, Client, ConnectTo, Result};
///
/// struct MyAgent;
///
/// impl ConnectTo<Client> for MyAgent {
///     async fn connect_to(self, client: impl ConnectTo<Agent>) -> Result<()> {
///         Agent.builder()
///             .name("my-agent")
///             .connect_to(client)
///             .await
///     }
/// }
/// ```
///
/// # Heterogeneous Collections
///
/// For storing different component types in the same collection, use [`DynConnectTo`]:
///
/// ```rust
/// use agent_client_protocol::{Channel, Client, DynConnectTo};
///
/// let (first, _first_peer) = Channel::duplex();
/// let (second, _second_peer) = Channel::duplex();
/// let components: Vec<DynConnectTo<Client>> = vec![
///     DynConnectTo::new(first),
///     DynConnectTo::new(second),
/// ];
/// assert_eq!(components.len(), 2);
/// ```
///
/// [`ByteStreams`]: crate::ByteStreams
/// [`AcpAgent`]: crate::AcpAgent
/// [`Builder`]: crate::Builder
pub trait ConnectTo<R: Role>: Send + 'static {
    /// Connect this component to another component.
    ///
    /// Most components implement this method to set up their connection and
    /// exchange messages with the provided component.
    ///
    /// # Arguments
    ///
    /// * `client` - The component to connect to (implements `ConnectTo<R::Counterpart>`)
    ///
    /// # Returns
    ///
    /// A future that resolves when the connection ends, either successfully
    /// or with an error. The future must be `Send`.
    ///
    /// A component that buffers outbound messages should not return `Ok(())`
    /// merely because its client completed: it should first finish messages the
    /// client already transferred to it. This lets wrappers preserve graceful
    /// drain guarantees through to the physical transport sink. Errors may
    /// still terminate the connection immediately.
    fn connect_to(
        self,
        client: impl ConnectTo<R::Counterpart>,
    ) -> impl Future<Output = Result<()>> + Send;

    /// Convert this component into a channel endpoint and connection future.
    ///
    /// The returned [`Channel`] is the canonical frame-aware boundary. It carries
    /// complete [`TransportFrame`](crate::TransportFrame) values so default
    /// adapters preserve batch grouping.
    ///
    /// This method returns:
    /// - A `Channel` that can be used to communicate with this component
    /// - A `BoxFuture` that drives the component's connection logic
    ///
    /// The default implementation creates an intermediate channel pair and calls `connect_to`
    /// on one endpoint while returning the other endpoint for the caller to use.
    ///
    /// Base cases like `Channel` and `ByteStreams` override this to avoid unnecessary copying.
    ///
    /// # Returns
    ///
    /// A tuple of `(Channel, BoxFuture)` where the channel is for the caller to use
    /// and the future must be polled to drive the connection.
    fn into_channel_and_future(self) -> (Channel, BoxFuture<'static, Result<()>>)
    where
        Self: Sized,
    {
        let (channel_a, channel_b) = Channel::duplex();
        let future = Box::pin(self.connect_to(channel_b));
        (channel_a, future)
    }
}

/// Type-erased connect trait for object-safe dynamic dispatch.
///
/// This trait is internal and used by [`DynConnectTo`]. Users should implement
/// [`ConnectTo`] instead, which is automatically converted to `ErasedConnectTo`
/// via a blanket implementation.
trait ErasedConnectTo<R: Role>: Send {
    fn type_name(&self) -> &'static str;

    fn connect_to_erased(
        self: Box<Self>,
        client: Box<dyn ErasedConnectTo<R::Counterpart>>,
    ) -> BoxFuture<'static, Result<()>>;

    fn into_channel_and_future_erased(self: Box<Self>)
    -> (Channel, BoxFuture<'static, Result<()>>);
}

/// Blanket implementation: any `ConnectTo<R>` can be type-erased.
impl<C: ConnectTo<R>, R: Role> ErasedConnectTo<R> for C {
    fn type_name(&self) -> &'static str {
        std::any::type_name::<C>()
    }

    fn connect_to_erased(
        self: Box<Self>,
        client: Box<dyn ErasedConnectTo<R::Counterpart>>,
    ) -> BoxFuture<'static, Result<()>> {
        Box::pin(async move {
            (*self)
                .connect_to(DynConnectTo {
                    inner: client,
                    _marker: PhantomData,
                })
                .await
        })
    }

    fn into_channel_and_future_erased(
        self: Box<Self>,
    ) -> (Channel, BoxFuture<'static, Result<()>>) {
        (*self).into_channel_and_future()
    }
}

/// A dynamically-typed component for heterogeneous collections.
///
/// This type wraps any [`ConnectTo`] implementation and provides dynamic dispatch,
/// allowing you to store different component types in the same collection.
///
/// The type parameter `R` is the role that all components in the
/// collection connect to (their counterpart).
///
/// # Examples
///
/// ```rust
/// use agent_client_protocol::{Channel, Client, DynConnectTo};
///
/// let (first, _first_peer) = Channel::duplex();
/// let (second, _second_peer) = Channel::duplex();
/// let components: Vec<DynConnectTo<Client>> = vec![
///     DynConnectTo::new(first),
///     DynConnectTo::new(second),
/// ];
/// assert_eq!(components.len(), 2);
/// ```
pub struct DynConnectTo<R: Role> {
    inner: Box<dyn ErasedConnectTo<R>>,
    _marker: PhantomData<R>,
}

impl<R: Role> DynConnectTo<R> {
    /// Create a new `DynConnectTo` from any type implementing [`ConnectTo`].
    pub fn new<C: ConnectTo<R>>(component: C) -> Self {
        Self {
            inner: Box::new(component),
            _marker: PhantomData,
        }
    }

    /// Returns the type name of the wrapped component.
    #[must_use]
    pub fn type_name(&self) -> &'static str {
        self.inner.type_name()
    }
}

impl<R: Role> ConnectTo<R> for DynConnectTo<R> {
    async fn connect_to(self, client: impl ConnectTo<R::Counterpart>) -> Result<()> {
        self.inner
            .connect_to_erased(Box::new(client) as Box<dyn ErasedConnectTo<R::Counterpart>>)
            .await
    }

    fn into_channel_and_future(self) -> (Channel, BoxFuture<'static, Result<()>>) {
        self.inner.into_channel_and_future_erased()
    }
}

impl<R: Role> Debug for DynConnectTo<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DynConnectTo")
            .field("type_name", &self.type_name())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::role::UntypedRole;

    #[test]
    fn dyn_connect_to_reports_static_type_name_and_correct_debug_label() {
        let (channel, _other) = Channel::duplex();
        let component = DynConnectTo::<UntypedRole>::new(channel);

        let type_name: &'static str = component.type_name();
        assert_eq!(type_name, std::any::type_name::<Channel>());
        assert_eq!(
            format!("{component:?}"),
            format!("DynConnectTo {{ type_name: {type_name:?} }}")
        );
    }
}
