//! Utilities to (de)serialize a value as a [`enum@zvariant::Value`].
//!
//! This is mainly useful for constructing a strongly-typed API for dealing with dictionaries
//! containing string keys and variant values (`a{sv}` in D-Bus language) See the relevant
//! [FAQ entry] in our book for more details and examples.
//!
//! [FAQ entry]: https://z-galaxy.github.io/zbus/faq.html#how-to-use-a-struct-as-a-dictionary

mod deserialize;
pub use deserialize::{Deserialize, deserialize};
mod serialize;
pub use serialize::{Serialize, serialize};

/// Utilities to (de)serialize an optional value as a [`enum@zvariant::Value`].
pub mod optional {
    use super::*;

    pub use deserialize::deserialize_optional as deserialize;
    pub use serialize::serialize_optional as serialize;
}
