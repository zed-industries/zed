# zbus-lockstep-macros

[![CI](https://github.com/luukvanderduim/zbus-lockstep/actions/workflows/rust.yml/badge.svg)](https://github.com/luukvanderduim/zbus-lockstep/actions/workflows/rust.yml)
![Maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)
[![crates-io](https://img.shields.io/crates/v/zbus-lockstep.svg)](https://crates.io/crates/zbus-lockstep)
[![api-docs](https://docs.rs/zbus-lockstep/badge.svg)](https://docs.rs/zbus-lockstep)

`zbus-lockstep-macros` extends `zbus-lockstep` to match the signature of signal types [`<T as zvariant::Type>::signature()`](https://docs.rs/zvariant/latest/zvariant/trait.Type.html#tymethod.signature) with a corresponding signature from a DBus XML file more conveniently and succinctly.

## Motivation

In the context of IPC over `DBus`, especially where there are multiple implementations of servers and/or clients communicating, it is necessary for each implementation to send what others expect and that expectations are in accordance with what is sent over the bus.

The `XML` protocol-descriptions may act as a shared frame of reference or "single source of all truth" for all implementers.
Having a single point of reference helps all implementers meet expectations on protocol conformance.

Keeping the types you send over `DBus` in lockstep with currently valid protocol-descriptions will reduce chances of miscommunication or failure to communicate.

## Use

Add `zbus-lockstep-macros` to `Cargo.toml`'s dependencies:

```toml
[dependencies]
zbus-lockstep-macros = "0.5.2"
```

If the `DBus` XML descriptions can be found in the crates root,
in either `xml/` or `XML/`, validating the type can be as easy as:

```rust
 use zbus_lockstep_macros::validate;
 use zvariant::Type;

 #[validate]
 #[derive(Type)]
 struct BirthdayEvent {
    name: String,
    new_age: u8,
}
```

Note that the macro assumes that the member name is contained in the struct name.
You can provide the member name if you have another naming-scheme in use.

Also, it may be necessary to disambiguate if multiple interfaces across the `DBus`
descriptions provide signals with the same name.

Any of the arguments are optional.

`#[validate(xml: <xml_path>, interface: <interface_name>, member: <member_name>)]`

See also the [crates docs](https://docs.rs/zbus-lockstep-macros/latest) for more detailed descriptions of the arguments.

## LICENSE

MIT
