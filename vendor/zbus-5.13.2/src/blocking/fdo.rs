//! D-Bus standard interfaces.
//!
//! Provides blocking versions of the proxy types in [`zbus::fdo`] module.

pub use crate::fdo::{
    dbus::{
        DBusProxyBlocking as DBusProxy, NameAcquiredIterator, NameLostIterator,
        NameOwnerChangedIterator,
    },
    introspectable::IntrospectableProxyBlocking as IntrospectableProxy,
    monitoring::MonitoringProxyBlocking as MonitoringProxy,
    object_manager::{
        InterfacesAddedIterator, InterfacesRemovedIterator,
        ObjectManagerProxyBlocking as ObjectManagerProxy,
    },
    peer::PeerProxyBlocking as PeerProxy,
    properties::{PropertiesChangedIterator, PropertiesProxyBlocking as PropertiesProxy},
    stats::StatsProxyBlocking as StatsProxy,
};
