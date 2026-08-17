//! D-Bus standard interfaces.
//!
//! The D-Bus specification defines the message bus messages and some standard interfaces that may
//! be useful across various D-Bus applications. This module provides their proxy.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use zbus_names::{BusName, OwnedUniqueName};
use zvariant::{OwnedValue, Type, as_value::optional};

use super::Result;
use crate::proxy;

/// Proxy for the [`org.freedesktop.DBus.Debug.Stats`][link] interface.
///
/// [link]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-bus-debug-stats-interface
#[proxy(
    interface = "org.freedesktop.DBus.Debug.Stats",
    default_service = "org.freedesktop.DBus",
    default_path = "/org/freedesktop/DBus"
)]
pub trait Stats {
    /// Get statistics about the message bus itself.
    fn get_stats(&self) -> Result<Stats>;

    /// Get statistics about a connection, identified by its unique connection name or by any
    /// well-known bus name for which it is the primary owner. This method is not meaningful for
    /// the message bus `org.freedesktop.DBus` itself.
    fn get_connection_stats(&self, name: BusName<'_>) -> Result<ConnectionStats>;

    /// List all of the match rules that are active on this message bus. The keys in the result
    /// dictionary are unique connection names. The values are lists of match rules registered by
    /// that connection, in an unspecified order. If a connection has registered the same match rule
    /// more than once, it is unspecified whether duplicate entries appear in the list.
    fn get_all_match_rules(&self) -> Result<HashMap<OwnedUniqueName, Vec<crate::OwnedMatchRule>>>;
}

/// The stats returned by the [`StatsProxy::get_stats`] method.
#[derive(Debug, Default, Deserialize, PartialEq, Serialize, Type)]
#[zvariant(signature = "a{sv}", rename_all = "PascalCase")]
#[serde(default, rename_all = "PascalCase")]
pub struct Stats {
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub(crate) serial: Option<u32>,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub(crate) active_connections: Option<u32>,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub(crate) incomplete_connections: Option<u32>,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub(crate) match_rules: Option<u32>,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub(crate) peak_match_rules: Option<u32>,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub(crate) peak_match_rules_per_connection: Option<u32>,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub(crate) bus_names: Option<u32>,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub(crate) peak_bus_names: Option<u32>,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub(crate) peak_bus_names_per_connection: Option<u32>,
    #[serde(flatten)]
    pub(crate) rest: HashMap<String, OwnedValue>,
}

impl Stats {
    /// A serial number which is incremented with each call to the GetStats method.
    pub fn serial(&self) -> Option<u32> {
        self.serial
    }

    /// The number of active connections currently handled by this message bus. The exact meaning of
    /// an active connection is implementation-defined: in the reference dbus-daemon, a connection
    /// is considered to be active after it has successfully called the Hello method.
    pub fn active_connections(&self) -> Option<u32> {
        self.active_connections
    }

    /// The number of incomplete connections currently handled by this message bus. The exact
    /// meaning of an incomplete connection is implementation-defined: in the reference dbus-daemon,
    /// a connection is considered to be incomplete if it is still carrying out the SASL handshake
    /// or if it has not yet successfully called the `Hello` method.
    pub fn incomplete_connections(&self) -> Option<u32> {
        self.incomplete_connections
    }

    /// The total number of match rules that are currently in use.
    pub fn match_rules(&self) -> Option<u32> {
        self.match_rules
    }

    /// The largest total number of match rules that have been in use at any one time.
    pub fn peak_match_rules(&self) -> Option<u32> {
        self.peak_match_rules
    }

    /// The largest number of match rules that have been in use by a single connection at any one
    /// time.
    pub fn peak_match_rules_per_connection(&self) -> Option<u32> {
        self.peak_match_rules_per_connection
    }

    /// The total number of bus names that are currently in use.
    pub fn bus_names(&self) -> Option<u32> {
        self.bus_names
    }

    /// The largest total number of bus names that have been in use at any one time.
    pub fn peak_bus_names(&self) -> Option<u32> {
        self.peak_bus_names
    }

    /// The largest number of bus names that have been in use by a single connection at any one
    /// time.
    pub fn peak_bus_names_per_connection(&self) -> Option<u32> {
        self.peak_bus_names_per_connection
    }

    /// The rest of the statistics, that are not defined by the D-Bus specificiation and hence
    /// specific to individual D-Bus broker implementations.
    pub fn rest(&self) -> &HashMap<String, OwnedValue> {
        &self.rest
    }
}

/// The stats returned by the [`StatsProxy::get_connection_stats`] method.
#[derive(Debug, Default, Deserialize, PartialEq, Serialize, Type)]
#[zvariant(signature = "a{sv}")]
#[serde(default, rename_all = "PascalCase")]
pub struct ConnectionStats {
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub(crate) serial: Option<u32>,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub(crate) unique_name: Option<OwnedUniqueName>,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub(crate) match_rules: Option<u32>,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub(crate) peak_match_rules: Option<u32>,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub(crate) bus_names: Option<u32>,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub(crate) peak_bus_names: Option<u32>,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub(crate) incoming_messages: Option<u32>,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub(crate) outgoing_messages: Option<u32>,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub(crate) incoming_bytes: Option<u32>,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub(crate) outgoing_bytes: Option<u32>,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub(crate) incoming_fds: Option<u32>,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub(crate) outgoing_fds: Option<u32>,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub(crate) peak_incoming_messages: Option<u32>,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub(crate) peak_outgoing_messages: Option<u32>,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub(crate) peak_incoming_bytes: Option<u32>,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub(crate) peak_outgoing_bytes: Option<u32>,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub(crate) peak_incoming_fds: Option<u32>,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub(crate) peak_outgoing_fds: Option<u32>,
    #[serde(flatten)]
    pub(crate) rest: HashMap<String, OwnedValue>,
}

impl ConnectionStats {
    /// A serial number which is incremented with each call to the GetConnectionStats method.
    pub fn serial(&self) -> Option<u32> {
        self.serial
    }

    /// The unique name of the connection.
    pub fn unique_name(&self) -> Option<&OwnedUniqueName> {
        self.unique_name.as_ref()
    }

    /// The total number of match rules that are currently in use by this connection.
    pub fn match_rules(&self) -> Option<u32> {
        self.match_rules
    }

    /// The largest total number of match rules that have been in use by this connection at any one
    /// time.
    pub fn peak_match_rules(&self) -> Option<u32> {
        self.peak_match_rules
    }

    /// The total number of bus names that are currently in use by this connection.
    pub fn bus_names(&self) -> Option<u32> {
        self.bus_names
    }

    /// The largest total number of bus names that have been in use by this connection at any one
    /// time.
    pub fn peak_bus_names(&self) -> Option<u32> {
        self.peak_bus_names
    }

    /// The total number of messages received by this connection.
    pub fn incoming_messages(&self) -> Option<u32> {
        self.incoming_messages
    }

    /// The total number of messages sent by this connection.
    pub fn outgoing_messages(&self) -> Option<u32> {
        self.outgoing_messages
    }

    /// The total number of bytes received by this connection.
    pub fn incoming_bytes(&self) -> Option<u32> {
        self.incoming_bytes
    }

    /// The total number of bytes sent by this connection.
    pub fn outgoing_bytes(&self) -> Option<u32> {
        self.outgoing_bytes
    }

    /// The total number of file descriptors received by this connection.
    pub fn incoming_fds(&self) -> Option<u32> {
        self.incoming_fds
    }

    /// The total number of file descriptors sent by this connection.
    pub fn outgoing_fds(&self) -> Option<u32> {
        self.outgoing_fds
    }

    /// The largest total number of messages that have been in use by this connection at any one
    /// time.
    pub fn peak_incoming_messages(&self) -> Option<u32> {
        self.peak_incoming_messages
    }

    /// The largest total number of messages that have been in use by this connection at any one
    /// time.
    pub fn peak_outgoing_messages(&self) -> Option<u32> {
        self.peak_outgoing_messages
    }

    /// The largest total number of bytes that have been in use by this connection at any one time.
    pub fn peak_incoming_bytes(&self) -> Option<u32> {
        self.peak_incoming_bytes
    }

    /// The largest total number of bytes that have been in use by this connection at any one time.
    pub fn peak_outgoing_bytes(&self) -> Option<u32> {
        self.peak_outgoing_bytes
    }

    /// The largest total number of file descriptors that have been in use by this connection at any
    /// one time.
    pub fn peak_incoming_fds(&self) -> Option<u32> {
        self.peak_incoming_fds
    }

    /// The largest total number of file descriptors that have been in use by this connection at any
    /// one time.
    pub fn peak_outgoing_fds(&self) -> Option<u32> {
        self.peak_outgoing_fds
    }

    /// The rest of the statistics, that are not defined by the D-Bus specificiation and hence
    /// specific to individual D-Bus broker implementations.
    pub fn rest(&self) -> &HashMap<String, OwnedValue> {
        &self.rest
    }
}
