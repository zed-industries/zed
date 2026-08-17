# `atspi-common`

Common data structures for the `atspi` crate.
This crate is meant to only contain the absolute lowest common denominator for libraries to talk to each other about `atspi` information;
therefore, it attempts to be able to compile on basically any architecture (including WASM).

Please use the internal documentation to learn how to use these data structures.

## Feature Flags

- `default`: `zbus`, `wrappers`
- `zbus`: include support for serializing/deserializing from `zbus::Message`s over DBus.
- `wrappers`: container enum types that group events by AT-SPI interface, as well as `Event` wrapper that contains any possible event.
