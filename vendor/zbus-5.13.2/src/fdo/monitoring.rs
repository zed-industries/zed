//! D-Bus standard interfaces.
//!
//! The D-Bus specification defines the message bus messages and some standard interfaces that may
//! be useful across various D-Bus applications. This module provides their proxy.

/// Proxy for the `org.freedesktop.DBus.Monitoring` interface.
#[crate::proxy(
    interface = "org.freedesktop.DBus.Monitoring",
    default_service = "org.freedesktop.DBus",
    default_path = "/org/freedesktop/DBus"
)]
pub trait Monitoring {
    /// Converts the connection into a monitor connection which can be used as a
    /// debugging/monitoring tool.
    ///
    /// After this call successfully returns, sending any messages on the bus will result
    /// in an error. This is why this method takes ownership of `self`, since there is not
    /// much use for the proxy anymore. It is highly recommended to convert the underlying
    /// [`Connection`] to a [`MessageStream`] and iterate over messages from the stream,
    /// after this call.
    ///
    /// See [the spec] for details on all the implications and caveats.
    ///
    /// # Arguments
    ///
    /// * `match_rules` - A list of match rules describing the messages you want to receive. An
    ///   empty list means you are want to receive all messages going through the bus.
    /// * `flags` - This argument is currently unused by the bus. Just pass a `0`.
    ///
    /// [the spec]: https://dbus.freedesktop.org/doc/dbus-specification.html#bus-messages-become-monitor
    /// [`Connection`]: https://docs.rs/zbus/latest/zbus/connection/struct.Connection.html
    /// [`MessageStream`]: https://docs.rs/zbus/latest/zbus/struct.MessageStream.html
    fn become_monitor(self, match_rules: &[crate::MatchRule<'_>], flags: u32) -> super::Result<()>;
}
