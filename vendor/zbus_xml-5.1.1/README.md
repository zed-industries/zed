# zbus_xml

[![](https://docs.rs/zbus_xml/badge.svg)](https://docs.rs/zbus_xml/) [![](https://img.shields.io/crates/v/zbus_xml)](https://crates.io/crates/zbus_xml)

API to handle D-Bus introspection XML.

Thanks to the [`org.freedesktop.DBus.Introspectable`] interface, objects may be introspected at
runtime, returning an XML string that describes the object.

This crate provides facilities to parse the XML data into more convenient
Rust structures. The XML string may be parsed to a tree with [`Node::from_reader`].

**Status:** Stable.

[`Node::from_reader`]: https://docs.rs/zbus_xml/latest/zbus_xml/struct.Node.html#method.from_reader
[Introspection format]: https://dbus.freedesktop.org/doc/dbus-specification.html#introspection-format
[`org.freedesktop.DBus.Introspectable`]: https://dbus.freedesktop.org/doc/dbus-specification.html#standard-interfaces-introspectable
