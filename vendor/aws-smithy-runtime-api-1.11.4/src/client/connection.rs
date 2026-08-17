/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Types related to connection monitoring and management.

use aws_smithy_types::config_bag::{Storable, StoreReplace};
use std::fmt;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

/// Metadata that tracks the state of an active connection.
#[derive(Clone)]
pub struct ConnectionMetadata {
    is_proxied: bool,
    remote_addr: Option<SocketAddr>,
    local_addr: Option<SocketAddr>,
    poison_fn: Arc<dyn Fn() + Send + Sync>,
}

impl ConnectionMetadata {
    /// Poison this connection, ensuring that it won't be reused.
    pub fn poison(&self) {
        tracing::info!(
            see_for_more_info = "https://smithy-lang.github.io/smithy-rs/design/client/detailed_error_explanations.html",
            "Connection encountered an issue and should not be re-used. Marking it for closure"
        );
        (self.poison_fn)()
    }

    /// Create a new [`ConnectionMetadata`].
    #[deprecated(
        since = "1.1.0",
        note = "`ConnectionMetadata::new` is deprecated in favour of `ConnectionMetadata::builder`."
    )]
    pub fn new(
        is_proxied: bool,
        remote_addr: Option<SocketAddr>,
        poison: impl Fn() + Send + Sync + 'static,
    ) -> Self {
        Self {
            is_proxied,
            remote_addr,
            // need to use builder to set this field
            local_addr: None,
            poison_fn: Arc::new(poison),
        }
    }

    /// Builder for this connection metadata
    pub fn builder() -> ConnectionMetadataBuilder {
        ConnectionMetadataBuilder::new()
    }

    /// Get the remote address for this connection, if one is set.
    pub fn remote_addr(&self) -> Option<SocketAddr> {
        self.remote_addr
    }

    /// Get the local address for this connection, if one is set.
    pub fn local_addr(&self) -> Option<SocketAddr> {
        self.local_addr
    }
}

impl fmt::Debug for ConnectionMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SmithyConnection")
            .field("is_proxied", &self.is_proxied)
            .field("remote_addr", &self.remote_addr)
            .field("local_addr", &self.local_addr)
            .finish()
    }
}

/// Builder type that is used to construct a [`ConnectionMetadata`] value.
#[derive(Default)]
pub struct ConnectionMetadataBuilder {
    is_proxied: Option<bool>,
    remote_addr: Option<SocketAddr>,
    local_addr: Option<SocketAddr>,
    poison_fn: Option<Arc<dyn Fn() + Send + Sync>>,
}

impl fmt::Debug for ConnectionMetadataBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ConnectionMetadataBuilder")
            .field("is_proxied", &self.is_proxied)
            .field("remote_addr", &self.remote_addr)
            .field("local_addr", &self.local_addr)
            .finish()
    }
}

impl ConnectionMetadataBuilder {
    /// Creates a new builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set whether or not the associated connection is to an HTTP proxy.
    pub fn proxied(mut self, proxied: bool) -> Self {
        self.set_proxied(Some(proxied));
        self
    }

    /// Set whether or not the associated connection is to an HTTP proxy.
    pub fn set_proxied(&mut self, proxied: Option<bool>) -> &mut Self {
        self.is_proxied = proxied;
        self
    }

    /// Set the remote address of the connection used.
    pub fn remote_addr(mut self, remote_addr: SocketAddr) -> Self {
        self.set_remote_addr(Some(remote_addr));
        self
    }

    /// Set the remote address of the connection used.
    pub fn set_remote_addr(&mut self, remote_addr: Option<SocketAddr>) -> &mut Self {
        self.remote_addr = remote_addr;
        self
    }

    /// Set the local address of the connection used.
    pub fn local_addr(mut self, local_addr: SocketAddr) -> Self {
        self.set_local_addr(Some(local_addr));
        self
    }

    /// Set the local address of the connection used.
    pub fn set_local_addr(&mut self, local_addr: Option<SocketAddr>) -> &mut Self {
        self.local_addr = local_addr;
        self
    }

    /// Set a closure which will poison the associated connection.
    ///
    /// A poisoned connection will not be reused for subsequent requests by the pool
    pub fn poison_fn(mut self, poison_fn: impl Fn() + Send + Sync + 'static) -> Self {
        self.set_poison_fn(Some(poison_fn));
        self
    }

    /// Set a closure which will poison the associated connection.
    ///
    /// A poisoned connection will not be reused for subsequent requests by the pool
    pub fn set_poison_fn(
        &mut self,
        poison_fn: Option<impl Fn() + Send + Sync + 'static>,
    ) -> &mut Self {
        self.poison_fn =
            poison_fn.map(|poison_fn| Arc::new(poison_fn) as Arc<dyn Fn() + Send + Sync>);
        self
    }

    /// Build a [`ConnectionMetadata`] value.
    ///
    /// # Panics
    ///
    /// If either the `is_proxied` or `poison_fn` has not been set, then this method will panic
    pub fn build(self) -> ConnectionMetadata {
        ConnectionMetadata {
            is_proxied: self
                .is_proxied
                .expect("is_proxied should be set for ConnectionMetadata"),
            remote_addr: self.remote_addr,
            local_addr: self.local_addr,
            poison_fn: self
                .poison_fn
                .expect("poison_fn should be set for ConnectionMetadata"),
        }
    }
}

type LoaderFn = dyn Fn() -> Option<ConnectionMetadata> + Send + Sync;

/// State for a middleware that will monitor and manage connections.
#[derive(Clone, Default)]
pub struct CaptureSmithyConnection {
    loader: Arc<Mutex<Option<Box<LoaderFn>>>>,
}

impl CaptureSmithyConnection {
    /// Create a new connection monitor.
    pub fn new() -> Self {
        Self {
            loader: Default::default(),
        }
    }

    /// Set the retriever that will capture the `hyper` connection.
    pub fn set_connection_retriever<F>(&self, f: F)
    where
        F: Fn() -> Option<ConnectionMetadata> + Send + Sync + 'static,
    {
        *self.loader.lock().unwrap() = Some(Box::new(f));
    }

    /// Get the associated connection metadata.
    pub fn get(&self) -> Option<ConnectionMetadata> {
        match self.loader.lock().unwrap().as_ref() {
            Some(loader) => loader(),
            None => {
                tracing::debug!("no loader was set on the CaptureSmithyConnection");
                None
            }
        }
    }
}

impl fmt::Debug for CaptureSmithyConnection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CaptureSmithyConnection")
    }
}

impl Storable for CaptureSmithyConnection {
    type Storer = StoreReplace<Self>;
}

#[cfg(test)]
mod tests {
    use std::{
        net::{IpAddr, Ipv6Addr},
        sync::Mutex,
    };

    use super::*;

    const TEST_SOCKET_ADDR: SocketAddr = SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), 100);

    #[test]
    #[should_panic]
    fn builder_panic_missing_proxied() {
        ConnectionMetadataBuilder::new()
            .poison_fn(|| {})
            .local_addr(TEST_SOCKET_ADDR)
            .remote_addr(TEST_SOCKET_ADDR)
            .build();
    }

    #[test]
    #[should_panic]
    fn builder_panic_missing_poison_fn() {
        ConnectionMetadataBuilder::new()
            .proxied(true)
            .local_addr(TEST_SOCKET_ADDR)
            .remote_addr(TEST_SOCKET_ADDR)
            .build();
    }

    #[test]
    fn builder_all_fields_successful() {
        let mutable_flag = Arc::new(Mutex::new(false));

        let connection_metadata = ConnectionMetadataBuilder::new()
            .proxied(true)
            .local_addr(TEST_SOCKET_ADDR)
            .remote_addr(TEST_SOCKET_ADDR)
            .poison_fn({
                let mutable_flag = Arc::clone(&mutable_flag);
                move || {
                    let mut guard = mutable_flag.lock().unwrap();
                    *guard = !*guard;
                }
            })
            .build();

        assert!(connection_metadata.is_proxied);
        assert_eq!(connection_metadata.remote_addr(), Some(TEST_SOCKET_ADDR));
        assert_eq!(connection_metadata.local_addr(), Some(TEST_SOCKET_ADDR));
        assert!(!(*mutable_flag.lock().unwrap()));
        connection_metadata.poison();
        assert!(*mutable_flag.lock().unwrap());
    }

    #[test]
    fn builder_optional_fields_translate() {
        let metadata1 = ConnectionMetadataBuilder::new()
            .proxied(true)
            .poison_fn(|| {})
            .build();

        assert_eq!(metadata1.local_addr(), None);
        assert_eq!(metadata1.remote_addr(), None);

        let metadata2 = ConnectionMetadataBuilder::new()
            .proxied(true)
            .poison_fn(|| {})
            .local_addr(TEST_SOCKET_ADDR)
            .build();

        assert_eq!(metadata2.local_addr(), Some(TEST_SOCKET_ADDR));
        assert_eq!(metadata2.remote_addr(), None);

        let metadata3 = ConnectionMetadataBuilder::new()
            .proxied(true)
            .poison_fn(|| {})
            .remote_addr(TEST_SOCKET_ADDR)
            .build();

        assert_eq!(metadata3.local_addr(), None);
        assert_eq!(metadata3.remote_addr(), Some(TEST_SOCKET_ADDR));
    }

    #[test]
    #[allow(clippy::redundant_clone)]
    fn retrieve_connection_metadata() {
        let retriever = CaptureSmithyConnection::new();
        let retriever_clone = retriever.clone();
        assert!(retriever.get().is_none());
        retriever.set_connection_retriever(|| {
            Some(
                ConnectionMetadata::builder()
                    .proxied(true)
                    .poison_fn(|| {})
                    .build(),
            )
        });

        assert!(retriever.get().is_some());
        assert!(retriever_clone.get().is_some());
    }
}
