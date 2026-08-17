# zbus-lockstep

[![CI](https://github.com/luukvanderduim/zbus-lockstep/actions/workflows/rust.yml/badge.svg)](https://github.com/luukvanderduim/zbus-lockstep/actions/workflows/rust.yml)
![Maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)
[![crates-io](https://img.shields.io/crates/v/zbus-lockstep.svg)](https://crates.io/crates/zbus-lockstep)
[![api-docs](https://docs.rs/zbus-lockstep/badge.svg)](https://docs.rs/zbus-lockstep)

`zbus-lockstep` helps keep type definitions in lockstep with DBus XML descriptions, using [`zbus-xml`](<https://github.com/dbus2/zbus>).

It offers means to match your type's signature - [`<T as zvariant::Type>::signature()`](https://docs.rs/zvariant/latest/zvariant/trait.Type.html#tymethod.signature) - with a corresponding signature retrieved from a DBus XML file.

This way `zbus-lockstep` prevents definitions from drifting apart.

## Motivation

In the context of IPC over `DBus` - especially when there are multiple implementations of servers and/or clients - it is necessary for each implementation to send what others expect and that expectations are in accordance with what is sent over the bus.

The `XML` protocol-descriptions may act as a shared frame of reference or "single source of all truth" for all implementers.
Having a single point of reference helps all implementers meet expectations on protocol conformance.

Keeping the types you send over `DBus` in lockstep with currently valid protocol-descriptions will reduce chances of miscommunication or failure to communicate.

## Usage

Add `zbus-lockstep` to `Cargo.toml`'s dev-dependencies:

```toml
[dev-dependencies]
zbus-lockstep = "0.5.2"
```

Consider the followwing XML description, an interface with a single signal.

```XML
<node>
  <interface name="org.example.Node">

    <signal name="RemoveNode">
      <arg name="nodeRemoved" type="(so)"/>
    </signal>

  </interface>
</node>
```

The type in our implementation might look like this:

```rust
#[derive(Type)]
struct Node {
    name: String,
    path: OwnedObjectPath,
}
```

The derive macro in this example implements the [`zvariant::Type`](https://docs.rs/zvariant/latest/zvariant/trait.Type.html).
This means we can now call `<Node as Type::signature()`, which will return a [`zvariant::Signature`](https://docs.rs/zvariant/latest/zvariant/struct.Signature.html) of the type.

The test below shows how `zbus-lockstep` may be used given what we know about the type.

```rust
    use zbus_lockstep;

    #[test]
    fn test_get_signal_body_type_remove_node() {
        let xml = PathBuf::from("../xml/test_definition_file.xml");
        let iface = "org.example.Node";
        let member = "RemoveNode";

        let signature = get_signal_body_type(xml, iface, member, None).unwrap();
        assert_eq!(signature, Signature::from_str_unchecked("(so)"));
    }
```

Alongside the functions, macros are provided which - if the path to the
definitions is known - can retrieve signatures more succinctly.

```rust
#[test]
fn macro_retrieve_signal_body_remove_node() {
std::env::set_var("LOCKSTEP_XML_PATH", "../xml");
use zbus_lockstep;

let sig = signal_body_type_signature!("RemoveNode");
assert_eq!(sig, zvariant::Signature::from_str_unchecked("(so)"));       
}

```

## Note

When using XML descriptions as point of reference, you should ensure that the descriptions in use are always the most recent available.

Automated synchronizing would be preferred.

## Acknowledgement

This crate started out as a fork of Tait Hoyem's [zbus-xml-match](https://github.com/TTWNO/zbus-xml-match).

## LICENSE

MIT
