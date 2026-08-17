//! The object server API.

use std::{collections::HashMap, marker::PhantomData, sync::Arc};
use tracing::{Instrument, debug, instrument, trace, trace_span};

use zbus_names::InterfaceName;
use zvariant::{ObjectPath, Value};

use crate::{
    Connection, Error, Result,
    async_lock::RwLock,
    connection::WeakConnection,
    fdo,
    fdo::ObjectManager,
    message::{Header, Message},
};

mod interface;
pub(crate) use interface::ArcInterface;
pub use interface::{DispatchResult, Interface, InterfaceDeref, InterfaceDerefMut, InterfaceRef};

mod signal_emitter;
pub use signal_emitter::SignalEmitter;
#[deprecated(since = "5.0.0", note = "Please use `SignalEmitter` instead.")]
pub type SignalContext<'s> = SignalEmitter<'s>;

mod dispatch_notifier;
pub use dispatch_notifier::ResponseDispatchNotifier;

mod node;
pub(crate) use node::Node;

/// An object server, holding server-side D-Bus objects & interfaces.
///
/// Object servers hold interfaces on various object paths, and expose them over D-Bus.
///
/// All object paths will have the standard interfaces implemented on your behalf, such as
/// `org.freedesktop.DBus.Introspectable` or `org.freedesktop.DBus.Properties`.
///
/// # Example
///
/// This example exposes the `org.myiface.Example.Quit` method on the `/org/zbus/path`
/// path.
///
/// ```no_run
/// # use std::error::Error;
/// use zbus::{Connection, interface};
/// use event_listener::Event;
/// # use async_io::block_on;
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
///     async fn quit(&mut self) {
///         self.quit_event.notify(1);
///     }
///
///     // See `interface` documentation to learn
///     // how to expose properties & signals as well.
/// }
///
/// # block_on(async {
/// let connection = Connection::session().await?;
///
/// let quit_event = Event::new();
/// let quit_listener = quit_event.listen();
/// let interface = Example::new(quit_event);
/// connection
///     .object_server()
///     .at("/org/zbus/path", interface)
///     .await?;
///
/// quit_listener.await;
/// # Ok::<_, Box<dyn Error + Send + Sync>>(())
/// # })?;
/// # Ok::<_, Box<dyn Error + Send + Sync>>(())
/// ```
#[derive(Debug, Clone)]
pub struct ObjectServer {
    conn: WeakConnection,
    root: Arc<RwLock<Node>>,
}

impl ObjectServer {
    /// Create a new D-Bus `ObjectServer`.
    pub(crate) fn new(conn: &Connection) -> Self {
        Self {
            conn: conn.into(),
            root: Arc::new(RwLock::new(Node::new(
                "/".try_into().expect("zvariant bug"),
            ))),
        }
    }

    pub(crate) fn root(&self) -> &RwLock<Node> {
        &self.root
    }

    /// Register a D-Bus [`Interface`] at a given path (see the example above).
    ///
    /// Typically you'd want your interfaces to be registered immediately after the associated
    /// connection is established and therefore use [`zbus::connection::Builder::serve_at`] instead.
    /// However, there are situations where you'd need to register interfaces dynamically and that's
    /// where this method becomes useful.
    ///
    /// If the interface already exists at this path, returns false.
    pub async fn at<'p, P, I>(&self, path: P, iface: I) -> Result<bool>
    where
        I: Interface,
        P: TryInto<ObjectPath<'p>>,
        P::Error: Into<Error>,
    {
        self.add_arc_interface(path, I::name(), ArcInterface::new(iface))
            .await
    }

    pub(crate) async fn add_arc_interface<'p, P>(
        &self,
        path: P,
        name: InterfaceName<'static>,
        arc_iface: ArcInterface,
    ) -> Result<bool>
    where
        P: TryInto<ObjectPath<'p>>,
        P::Error: Into<Error>,
    {
        let path = path.try_into().map_err(Into::into)?;
        let mut root = self.root().write().await;
        let (node, manager_path) = root.get_child_mut(&path, true);
        let node = node.unwrap();
        let added = node.add_arc_interface(name.clone(), arc_iface);
        if added {
            if name == ObjectManager::name() {
                // Just added an object manager. Need to signal all managed objects under it.
                let emitter = SignalEmitter::new(&self.connection(), path)?;
                let objects = node.get_managed_objects(self, &self.connection()).await?;
                for (path, owned_interfaces) in objects {
                    let interfaces = owned_interfaces
                        .iter()
                        .map(|(i, props)| {
                            let props = props
                                .iter()
                                .map(|(k, v)| Ok((k.as_str(), Value::try_from(v)?)))
                                .collect::<Result<_>>();
                            Ok((i.into(), props?))
                        })
                        .collect::<Result<_>>()?;
                    ObjectManager::interfaces_added(&emitter, path.into(), interfaces).await?;
                }
            } else if let Some(manager_path) = manager_path {
                let emitter = SignalEmitter::new(&self.connection(), manager_path.clone())?;
                let mut interfaces = HashMap::new();
                let owned_props = node
                    .get_properties(self, &self.connection(), name.clone())
                    .await?;
                let props = owned_props
                    .iter()
                    .map(|(k, v)| Ok((k.as_str(), Value::try_from(v)?)))
                    .collect::<Result<_>>()?;
                interfaces.insert(name, props);

                ObjectManager::interfaces_added(&emitter, path, interfaces).await?;
            }
        }

        Ok(added)
    }

    /// Unregister a D-Bus [`Interface`] at a given path.
    ///
    /// If there are no more interfaces left at that path, destroys the object as well.
    /// Returns whether the object was destroyed.
    pub async fn remove<'p, I, P>(&self, path: P) -> Result<bool>
    where
        I: Interface,
        P: TryInto<ObjectPath<'p>>,
        P::Error: Into<Error>,
    {
        let path = path.try_into().map_err(Into::into)?;
        let mut root = self.root.write().await;
        let (node, manager_path) = root.get_child_mut(&path, false);
        let node = node.ok_or(Error::InterfaceNotFound)?;
        if !node.remove_interface(I::name()) {
            return Err(Error::InterfaceNotFound);
        }
        if let Some(manager_path) = manager_path {
            let ctxt = SignalEmitter::new(&self.connection(), manager_path.clone())?;
            ObjectManager::interfaces_removed(&ctxt, path.clone(), (&[I::name()]).into()).await?;
        }
        if node.is_empty() {
            let mut path_parts = path.rsplit('/').filter(|i| !i.is_empty());
            let last_part = path_parts.next().unwrap();
            let ppath = ObjectPath::from_string_unchecked(
                path_parts.fold(String::new(), |a, p| format!("/{p}{a}")),
            );
            root.get_child_mut(&ppath, false)
                .0
                .unwrap()
                .remove_node(last_part);
            return Ok(true);
        }
        Ok(false)
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
    /// The typical use of this is property changes outside of a dispatched handler:
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # use zbus::{Connection, interface};
    /// # use async_io::block_on;
    /// #
    /// struct MyIface(u32);
    ///
    /// #[interface(name = "org.myiface.MyIface")]
    /// impl MyIface {
    ///      #[zbus(property)]
    ///      async fn count(&self) -> u32 {
    ///          self.0
    ///      }
    /// }
    ///
    /// # block_on(async {
    /// # let connection = Connection::session().await?;
    /// #
    /// # let path = "/org/zbus/path";
    /// # connection.object_server().at(path, MyIface(0)).await?;
    /// let iface_ref = connection
    ///     .object_server()
    ///     .interface::<_, MyIface>(path).await?;
    /// let mut iface = iface_ref.get_mut().await;
    /// iface.0 = 42;
    /// iface.count_changed(iface_ref.signal_emitter()).await?;
    /// # Ok::<_, Box<dyn Error + Send + Sync>>(())
    /// # })?;
    /// #
    /// # Ok::<_, Box<dyn Error + Send + Sync>>(())
    /// ```
    pub async fn interface<'p, P, I>(&self, path: P) -> Result<InterfaceRef<I>>
    where
        I: Interface,
        P: TryInto<ObjectPath<'p>>,
        P::Error: Into<Error>,
    {
        let path = path.try_into().map_err(Into::into)?;
        let root = self.root().read().await;
        let node = root.get_child(&path).ok_or(Error::InterfaceNotFound)?;

        let lock = node
            .interface_lock(I::name())
            .ok_or(Error::InterfaceNotFound)?
            .instance
            .clone();

        // Ensure what we return can later be dowcasted safely.
        lock.read()
            .await
            .downcast_ref::<I>()
            .ok_or(Error::InterfaceNotFound)?;

        let conn = self.connection();
        // SAFETY: We know that there is a valid path on the node as we already converted w/o error.
        let emitter = SignalEmitter::new(&conn, path).unwrap().into_owned();

        Ok(InterfaceRef {
            emitter,
            lock,
            phantom: PhantomData,
        })
    }

    async fn dispatch_call_to_iface(
        &self,
        iface: Arc<RwLock<dyn Interface>>,
        connection: &Connection,
        msg: &Message,
        hdr: &Header<'_>,
    ) -> fdo::Result<()> {
        let member = hdr
            .member()
            .ok_or_else(|| fdo::Error::Failed("Missing member".into()))?;
        let iface_name = hdr
            .interface()
            .ok_or_else(|| fdo::Error::Failed("Missing interface".into()))?;

        trace!("acquiring read lock on interface `{}`", iface_name);
        let read_lock = iface.read().await;
        trace!("acquired read lock on interface `{}`", iface_name);
        match read_lock.call(self, connection, msg, member.as_ref()) {
            DispatchResult::NotFound => {
                return Err(fdo::Error::UnknownMethod(format!(
                    "Unknown method '{member}'"
                )));
            }
            DispatchResult::Async(f) => {
                return f.await.map_err(|e| match e {
                    Error::FDO(e) => *e,
                    e => fdo::Error::Failed(format!("{e}")),
                });
            }
            DispatchResult::RequiresMut => {}
        }
        drop(read_lock);
        trace!("acquiring write lock on interface `{}`", iface_name);
        let mut write_lock = iface.write().await;
        trace!("acquired write lock on interface `{}`", iface_name);
        match write_lock.call_mut(self, connection, msg, member.as_ref()) {
            DispatchResult::NotFound => {}
            DispatchResult::RequiresMut => {}
            DispatchResult::Async(f) => {
                return f.await.map_err(|e| match e {
                    Error::FDO(e) => *e,
                    e => fdo::Error::Failed(format!("{e}")),
                });
            }
        }
        drop(write_lock);
        Err(fdo::Error::UnknownMethod(format!(
            "Unknown method '{member}'"
        )))
    }

    async fn dispatch_method_call_try(
        &self,
        connection: &Connection,
        msg: &Message,
        hdr: &Header<'_>,
    ) -> fdo::Result<()> {
        let path = hdr
            .path()
            .ok_or_else(|| fdo::Error::Failed("Missing object path".into()))?;
        let iface_name = hdr
            .interface()
            // TODO: In the absence of an INTERFACE field, if two or more interfaces on the same
            // object have a method with the same name, it is undefined which of those
            // methods will be invoked. Implementations may choose to either return an
            // error, or deliver the message as though it had an arbitrary one of those
            // interfaces.
            .ok_or_else(|| fdo::Error::Failed("Missing interface".into()))?;
        // Check that the message has a member before spawning.
        // Note that an unknown member will still spawn a task. We should instead gather
        // all the details for the call before spawning.
        // See also https://github.com/z-galaxy/zbus/issues/674 for future of Interface.
        let _ = hdr
            .member()
            .ok_or_else(|| fdo::Error::Failed("Missing member".into()))?;

        // Ensure the root lock isn't held while dispatching the message. That
        // way, the object server can be mutated during that time.
        let (iface, with_spawn) = {
            let root = self.root.read().await;

            // D-Bus spec: org.freedesktop.DBus.Peer interface works on ANY path, even unregistered
            // ones. See: https://dbus.freedesktop.org/doc/dbus-specification.html#standard-interfaces-peer
            // Switch the path to "/" for Peer interface calls.
            let path = if *iface_name == fdo::Peer::name() {
                ObjectPath::from_static_str_unchecked("/")
            } else {
                path.clone()
            };

            let node = root
                .get_child(&path)
                .ok_or_else(|| fdo::Error::UnknownObject(format!("Unknown object '{path}'")))?;

            let iface = node.interface_lock(iface_name.as_ref()).ok_or_else(|| {
                fdo::Error::UnknownInterface(format!("Unknown interface '{iface_name}'"))
            })?;
            (iface.instance, iface.spawn_tasks_for_methods)
        };

        if with_spawn {
            let executor = connection.executor().clone();
            let task_name = format!("`{msg}` method dispatcher");
            let connection = connection.clone();
            let msg = msg.clone();
            executor
                .spawn(
                    async move {
                        let server = connection.object_server();
                        let hdr = msg.header();
                        if let Err(e) = server
                            .dispatch_call_to_iface(iface, &connection, &msg, &hdr)
                            .await
                        {
                            // When not spawning a task, this error is handled by the caller.
                            debug!("Returning error: {}", e);
                            if let Err(e) = connection.reply_dbus_error(&hdr, e).await {
                                debug!(
                                    "Error dispatching message. Message: {:?}, error: {:?}",
                                    msg, e
                                );
                            }
                        }
                    }
                    .instrument(trace_span!("{}", task_name)),
                    &task_name,
                )
                .detach();
            Ok(())
        } else {
            self.dispatch_call_to_iface(iface, connection, msg, hdr)
                .await
        }
    }

    /// Dispatch an incoming message to a registered interface.
    ///
    /// The object server will handle the message by:
    ///
    /// - looking up the called object path & interface,
    ///
    /// - calling the associated method if one exists,
    ///
    /// - returning a message (responding to the caller with either a return or error message) to
    ///   the caller through the associated server connection.
    ///
    /// Returns an error if the message is malformed.
    #[instrument(skip(self))]
    pub(crate) async fn dispatch_call(&self, msg: &Message, hdr: &Header<'_>) -> Result<()> {
        let conn = self.connection();

        if let Err(e) = self.dispatch_method_call_try(&conn, msg, hdr).await {
            debug!("Returning error: {}", e);
            conn.reply_dbus_error(hdr, e).await?;
        }
        trace!("Handled: {}", msg);

        Ok(())
    }

    pub(crate) fn connection(&self) -> Connection {
        self.conn
            .upgrade()
            .expect("ObjectServer can't exist w/o an associated Connection")
    }
}

#[cfg(feature = "blocking-api")]
impl From<crate::blocking::ObjectServer> for ObjectServer {
    fn from(server: crate::blocking::ObjectServer) -> Self {
        server.into_inner()
    }
}
