//! The typed veneer over the dynamic shared-entity wire.
//!
//! On the wire, everything is `(entity_id, method: string, payload: bytes)` — fully dynamic,
//! so plugins can define their own entity types and methods without the protocol knowing
//! about them. These traits decorate that wire with compile-time names and payload types.
//! A *spec* stands in for the entity across the boundary: the home side implements handlers
//! for it, projections address messages through it, and neither side ever shares a Rust
//! layout — only names and serialized data.
//!
//! This crate is deliberately dependency-light (no gpui, no wasm bindings) so both the
//! native host and wasm guests can compile it.

// Lets macro-generated `#[serde(crate = "gpui_embedded_shared::serde")]` attributes resolve
// even when the macro expands inside this crate itself.
extern crate self as gpui_embedded_shared;

use serde::{Serialize, de::DeserializeOwned};

#[doc(hidden)]
pub use serde;

/// Identifies a kind of shared entity: a stable wire name plus the snapshot type its home
/// publishes to projections.
pub trait SharedSpec: 'static {
    /// Stable type name checked when a projection binds to an announcement.
    const TYPE_NAME: &'static str;

    /// The serializable state projections receive.
    type Snapshot: Serialize + DeserializeOwned + 'static;
}

/// A message that can be sent to the home side of a shared entity.
pub trait SharedMessage: Serialize + DeserializeOwned + 'static {
    /// The entity kind this message addresses.
    type Spec: SharedSpec;

    /// Stable method name used for dynamic dispatch on the home side.
    const METHOD: &'static str;
}

pub fn encode<T: Serialize>(value: &T) -> anyhow::Result<Vec<u8>> {
    Ok(serde_json::to_vec(value)?)
}

pub fn decode<T: DeserializeOwned>(bytes: &[u8]) -> anyhow::Result<T> {
    Ok(serde_json::from_slice(bytes)?)
}

/// The local replica of a shared entity's state, held in a GPUI entity on the projection
/// side. `state` is `None` until the first snapshot arrives from the home side.
pub struct SharedProjection<S> {
    pub state: Option<S>,
}

/// Generates the schema for one shared entity kind: the spec, its snapshot record, and its
/// message records, with all the trait wiring. Example:
///
/// ```ignore
/// shared_schema! {
///     entity CounterSpec as "my-plugin.counter" {
///         snapshot CounterSnapshot { clicks: u32 }
///         message "increment" Increment { by: u32 }
///     }
/// }
/// ```
#[macro_export]
macro_rules! shared_schema {
    (
        entity $spec:ident as $type_name:literal {
            snapshot $snapshot:ident { $($snapshot_field:ident : $snapshot_ty:ty),* $(,)? }
            $(message $method:literal $message:ident { $($message_field:ident : $message_ty:ty),* $(,)? })*
        }
    ) => {
        pub struct $spec;

        impl $crate::SharedSpec for $spec {
            const TYPE_NAME: &'static str = $type_name;
            type Snapshot = $snapshot;
        }

        #[derive(Clone, Debug, $crate::serde::Serialize, $crate::serde::Deserialize)]
        #[serde(crate = "gpui_embedded_shared::serde")]
        pub struct $snapshot {
            $(pub $snapshot_field: $snapshot_ty),*
        }

        $(
            #[derive(Clone, Debug, $crate::serde::Serialize, $crate::serde::Deserialize)]
            #[serde(crate = "gpui_embedded_shared::serde")]
            pub struct $message {
                $(pub $message_field: $message_ty),*
            }

            impl $crate::SharedMessage for $message {
                type Spec = $spec;
                const METHOD: &'static str = $method;
            }
        )*
    };
}

/// Schemas for the demo: a click counter homed on the host and projected into the plugin,
/// and the plugin's input-line text homed in the guest and projected into the host.
pub mod demo {
    crate::shared_schema! {
        entity CounterSpec as "gpui-embedded.demo.counter" {
            snapshot CounterSnapshot { clicks: u32 }
            message "increment" Increment { by: u32 }
        }
    }

    crate::shared_schema! {
        entity TextSpec as "gpui-embedded.demo.text" {
            snapshot TextSnapshot { text: String }
        }
    }
}
