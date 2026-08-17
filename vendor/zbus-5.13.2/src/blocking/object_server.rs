//! The object server API.

use zvariant::ObjectPath;

use crate::{
    Error, Result,
    object_server::{Interface, InterfaceDeref, InterfaceDerefMut, SignalEmitter},
    utils::block_on,
};

/// Wrapper over an interface, along with its corresponding `SignalEmitter`
/// instance. A reference to the underlying interface may be obtained via
/// [`InterfaceRef::get`] and [`InterfaceRef::get_mut`].
pub struct InterfaceRef<I> {
    azync: crate::object_server::InterfaceRef<I>,
}

impl<I> InterfaceRef<I>
where
    I: 'static,
{
    /// Get a reference to the underlying interface.
    ///
    /// **WARNING:** If methods (e.g property setters) in `ObjectServer` require `&mut self`
    /// `ObjectServer` will not be able to access the interface in question until all references
    /// of this method are dropped; it is highly recommended that the scope of the interface
    /// returned is restricted.
    pub fn get(&self) -> InterfaceDeref<'_, I> {
        block_on(self.azync.get())
    }

    /// Get a reference to the underlying interface.
    ///
    /// **WARNING:** Since the `ObjectServer` will not be able to access the interface in question
    /// until the return value of this method is dropped, it is highly recommended that the scope
    /// of the interface returned is restricted.
    ///
    /// # Errors
    ///
    /// If the interface at this instance's path is not valid, an `Error::InterfaceNotFound` error
    /// is returned.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # use async_io::block_on;
    /// # use zbus::{blocking::Connection, interface};
    ///
    /// struct MyIface(u32);
    ///
    /// #[interface(name = "org.myiface.MyIface")]
    /// impl MyIface {
    ///    #[zbus(property)]
    ///    fn count(&self) -> u32 {
    ///        self.0
    ///    }
    /// }
    /// // Set up connection and object_server etc here and then in another part of the code:
    /// #
    /// # let connection = Connection::session()?;
    /// #
    /// # let path = "/org/zbus/path";
    /// # connection.object_server().at(path, MyIface(22))?;
    /// let object_server = connection.object_server();
    /// let iface_ref = object_server.interface::<_, MyIface>(path)?;
    /// let mut iface = iface_ref.get_mut();
    /// iface.0 = 42;
    /// block_on(iface.count_changed(iface_ref.signal_emitter()))?;
    /// #
    /// # Ok::<_, Box<dyn Error + Send + Sync>>(())
    /// ```
    pub fn get_mut(&self) -> InterfaceDerefMut<'_, I> {
        block_on(self.azync.get_mut())
    }

    pub fn signal_emitter(&self) -> &SignalEmitter<'static> {
        self.azync.signal_emitter()
    }
}

/// A blocking wrapper of [`crate::ObjectServer`].
///
/// # Example
///
/// This example exposes the `org.myiface.Example.Quit` method on the `/org/zbus/path`
/// path.
///
/// ```no_run
/// # use std::error::Error;
/// use zbus::{blocking::Connection, interface};
/// use event_listener::{Event, Listener};
///
/// struct Example {
///     // Interfaces are owned by the ObjectServer. They can have
///     // `&mut self` methods.
///     quit_event: Event,
/// }
///
/// impl Example {
///     fn new(quit_event: Event) -> Self {
///         Self { quit_event }
///     }
/// }
///
/// #[interface(name = "org.myiface.Example")]
/// impl Example {
///     // This will be the "Quit" D-Bus method.
///     fn quit(&mut self) {
///         self.quit_event.notify(1);
///     }
///
///     // See `interface` documentation to learn
///     // how to expose properties & signals as well.
/// }
///
/// let connection = Connection::session()?;
///
/// let quit_event = Event::new();
/// let quit_listener = quit_event.listen();
/// let interface = Example::new(quit_event);
/// connection
///     .object_server()
///     .at("/org/zbus/path", interface)?;
///
/// quit_listener.wait();
/// # Ok::<_, Box<dyn Error + Send + Sync>>(())
/// ```
#[derive(Debug, Clone)]
pub struct ObjectServer {
    azync: crate::ObjectServer,
}

impl ObjectServer {
    /// Create a new D-Bus `ObjectServer`.
    pub(crate) fn new(conn: &crate::Connection) -> Self {
        Self {
            azync: conn.object_server().clone(),
        }
    }

    /// Register a D-Bus [`crate::object_server::Interface`] at a given path (see the example
    /// above).
    ///
    /// Typically you'd want your interfaces to be registered immediately after the associated
    /// connection is established and therefore use
    /// [`zbus::blocking::connection::Builder::serve_at`] instead. However, there are
    /// situations where you'd need to register interfaces dynamically and that's where this
    /// method becomes useful.
    ///
    /// If the interface already exists at this path, returns false.
    pub fn at<'p, P, I>(&self, path: P, iface: I) -> Result<bool>
    where
        I: Interface,
        P: TryInto<ObjectPath<'p>>,
        P::Error: Into<Error>,
    {
        block_on(self.azync.at(path, iface))
    }

    /// Unregister a D-Bus [`crate::object_server::Interface`] at a given path.
    ///
    /// If there are no more interfaces left at that path, destroys the object as well.
    /// Returns whether the object was destroyed.
    pub fn remove<'p, I, P>(&self, path: P) -> Result<bool>
    where
        I: Interface,
        P: TryInto<ObjectPath<'p>>,
        P::Error: Into<Error>,
    {
        block_on(self.azync.remove::<I, P>(path))
    }

    /// Get the interface at the given path.
    ///
    /// # Errors
    ///
    /// If the interface is not registered at the given path, an `Error::InterfaceNotFound` error is
    /// returned.
    ///
    /// # Examples
    ///
    /// The typical use of this is to emit signals outside of a dispatched handler:
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # use zbus::block_on;
    /// # use zbus::{
    /// #    object_server::SignalEmitter,
    /// #    blocking::Connection,
    /// #    interface,
    /// # };
    /// #
    /// struct MyIface;
    /// #[interface(name = "org.myiface.MyIface")]
    /// impl MyIface {
    ///     #[zbus(signal)]
    ///     async fn emit_signal(emitter: &SignalEmitter<'_>) -> zbus::Result<()>;
    /// }
    ///
    /// # let connection = Connection::session()?;
    /// #
    /// # let path = "/org/zbus/path";
    /// # connection.object_server().at(path, MyIface)?;
    /// let iface_ref = connection
    ///     .object_server()
    ///     .interface::<_, MyIface>(path)?;
    /// block_on(MyIface::emit_signal(iface_ref.signal_emitter()))?;
    /// #
    /// #
    /// # Ok::<_, Box<dyn Error + Send + Sync>>(())
    /// ```
    pub fn interface<'p, P, I>(&self, path: P) -> Result<InterfaceRef<I>>
    where
        I: Interface,
        P: TryInto<ObjectPath<'p>>,
        P::Error: Into<Error>,
    {
        Ok(InterfaceRef {
            azync: block_on(self.azync.interface(path))?,
        })
    }

    /// Get a reference to the underlying async ObjectServer.
    pub fn inner(&self) -> &crate::ObjectServer {
        &self.azync
    }

    /// Get the underlying async ObjectServer, consuming `self`.
    pub fn into_inner(self) -> crate::ObjectServer {
        self.azync
    }
}

impl From<crate::ObjectServer> for ObjectServer {
    fn from(azync: crate::ObjectServer) -> Self {
        Self { azync }
    }
}
