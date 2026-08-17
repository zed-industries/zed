//! D-Bus standard interfaces.
//!
//! The D-Bus specification defines the message bus messages and some standard interfaces that may
//! be useful across various D-Bus applications. This module provides their proxy.

use std::{borrow::Cow, collections::HashMap};
use zbus_names::{InterfaceName, OwnedInterfaceName};
use zvariant::{ObjectPath, OwnedObjectPath, OwnedValue, Value};

use super::{Error, Result};
use crate::{Connection, ObjectServer, interface, message::Header, object_server::SignalEmitter};

/// The type returned by the [`ObjectManagerProxy::get_managed_objects`] method.
pub type ManagedObjects =
    HashMap<OwnedObjectPath, HashMap<OwnedInterfaceName, HashMap<String, OwnedValue>>>;

/// Service-side [Object Manager][om] interface implementation.
///
/// The recommended path to add this interface at is the path form of the well-known name of a D-Bus
/// service, or below. For example, if a D-Bus service is available at the well-known name
/// `net.example.ExampleService1`, this interface should typically be registered at
/// `/net/example/ExampleService1`, or below (to allow for multiple object managers in a service).
///
/// It is supported, but not recommended, to add this interface at the root path, `/`.
///
/// When added to an `ObjectServer`, the `InterfacesAdded` signal is emitted for all the objects
/// under the `path` it's added at. You can use this fact to minimize the signal emissions by
/// populating the entire (sub)tree under `path` before registering an object manager.
///
/// [om]: https://dbus.freedesktop.org/doc/dbus-specification.html#standard-interfaces-objectmanager
#[derive(Debug, Clone)]
pub struct ObjectManager;

#[interface(
    name = "org.freedesktop.DBus.ObjectManager",
    introspection_docs = false,
    proxy(visibility = "pub")
)]
impl ObjectManager {
    /// The return value of this method is a dict whose keys are object paths. All returned object
    /// paths are children of the object path implementing this interface, i.e. their object paths
    /// start with the ObjectManager's object path plus '/'.
    ///
    /// Each value is a dict whose keys are interfaces names. Each value in this inner dict is the
    /// same dict that would be returned by the org.freedesktop.DBus.Properties.GetAll() method for
    /// that combination of object path and interface. If an interface has no properties, the empty
    /// dict is returned.
    async fn get_managed_objects(
        &self,
        #[zbus(object_server)] server: &ObjectServer,
        #[zbus(connection)] connection: &Connection,
        #[zbus(header)] header: Header<'_>,
    ) -> Result<ManagedObjects> {
        let path = header.path().ok_or(crate::Error::MissingField)?;
        let root = server.root().read().await;
        let node = root
            .get_child(path)
            .ok_or_else(|| Error::UnknownObject(format!("Unknown object '{path}'")))?;

        node.get_managed_objects(server, connection).await
    }

    /// This signal is emitted when either a new object is added or when an existing object gains
    /// one or more interfaces. The `interfaces_and_properties` argument contains a map with the
    /// interfaces and properties (if any) that have been added to the given object path.
    #[zbus(signal)]
    pub async fn interfaces_added(
        emitter: &SignalEmitter<'_>,
        object_path: ObjectPath<'_>,
        interfaces_and_properties: HashMap<InterfaceName<'_>, HashMap<&str, Value<'_>>>,
    ) -> zbus::Result<()>;

    /// This signal is emitted whenever an object is removed or it loses one or more interfaces.
    /// The `interfaces` parameters contains a list of the interfaces that were removed.
    #[zbus(signal)]
    pub async fn interfaces_removed(
        emitter: &SignalEmitter<'_>,
        object_path: ObjectPath<'_>,
        interfaces: Cow<'_, [InterfaceName<'_>]>,
    ) -> zbus::Result<()>;
}
