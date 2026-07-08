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

    /// The value the home's handler returns. `()` for fire-and-forget casts.
    type Response: Serialize + DeserializeOwned + 'static;

    /// Stable method name used for dynamic dispatch on the home side.
    const METHOD: &'static str;
}

/// An entity type that can serve as the home of a shared entity of kind `S`.
pub trait SharedEntitySource<S: SharedSpec>: 'static {
    fn snapshot(&self, cx: &gpui::App) -> S::Snapshot;
}

/// A handler for one typed message, implemented by the home entity. The returned value is
/// serialized back to the caller when the message was a call.
pub trait HandleShared<M: SharedMessage>: 'static + Sized {
    fn handle(&mut self, message: M, cx: &mut gpui::Context<Self>) -> M::Response;
}

/// A type-erased method handler on an entity's home side: decode, dispatch, encode the
/// response. Receives the method name so wildcard handlers can interpret it. Used
/// identically by the native host and the wasm guest.
pub type MethodHandler =
    std::rc::Rc<dyn Fn(&str, &[u8], &mut gpui::App) -> anyhow::Result<Vec<u8>>>;

/// Registering a handler under this name makes it the fallback for any method that has no
/// explicit entry: fully dynamic dispatch, decided by the entity at runtime.
pub const WILDCARD_METHOD: &str = "*";

/// Typed registration of dynamically dispatched methods for a shared entity's home.
pub struct Methods<S: SharedSpec, T> {
    entity: gpui::WeakEntity<T>,
    map: std::collections::HashMap<String, MethodHandler>,
    _spec: std::marker::PhantomData<S>,
}

impl<S: SharedSpec, T: 'static> Methods<S, T> {
    #[doc(hidden)]
    pub fn new(entity: gpui::WeakEntity<T>) -> Self {
        Self {
            entity,
            map: std::collections::HashMap::new(),
            _spec: std::marker::PhantomData,
        }
    }

    /// Register the handler for message type `M`. The wire stays dynamic — this inserts a
    /// decode-dispatch-encode closure under `M::METHOD`.
    pub fn on<M>(&mut self) -> &mut Self
    where
        M: SharedMessage<Spec = S>,
        T: HandleShared<M>,
    {
        let entity = self.entity.clone();
        self.map.insert(
            M::METHOD.to_string(),
            std::rc::Rc::new(move |_method, payload, cx| {
                let message: M = decode(payload)?;
                let response = entity.update(cx, |entity, cx| entity.handle(message, cx))?;
                encode(&response)
            }),
        );
        self
    }

    /// The dynamic escape hatch: register a raw handler for an arbitrary method name, or
    /// for [`WILDCARD_METHOD`] to receive every method without an explicit entry.
    pub fn on_raw(
        &mut self,
        method: impl Into<String>,
        handler: impl Fn(&gpui::Entity<T>, &str, &[u8], &mut gpui::App) -> anyhow::Result<Vec<u8>>
        + 'static,
    ) -> &mut Self {
        let entity = self.entity.clone();
        self.map.insert(
            method.into(),
            std::rc::Rc::new(move |method, payload, cx| {
                let entity = entity
                    .upgrade()
                    .ok_or_else(|| anyhow::anyhow!("shared entity dropped"))?;
                handler(&entity, method, payload, cx)
            }),
        );
        self
    }

    #[doc(hidden)]
    pub fn into_map(self) -> std::collections::HashMap<String, MethodHandler> {
        self.map
    }
}

/// Resolves once the home has applied a send and the local replica reflects it (awaiting it
/// gives read-your-writes). Dropping it means "don't wait"; the message is unaffected.
pub struct SendReceipt(futures::channel::oneshot::Receiver<()>);

/// The sending half backing a [`SendReceipt`].
pub type AckSender = futures::channel::oneshot::Sender<()>;

impl SendReceipt {
    #[doc(hidden)]
    pub fn channel() -> (AckSender, Self) {
        let (sender, receiver) = futures::channel::oneshot::channel();
        (sender, Self(receiver))
    }

    #[doc(hidden)]
    pub fn dropped() -> Self {
        let (_sender, receiver) = futures::channel::oneshot::channel();
        Self(receiver)
    }
}

impl std::future::Future for SendReceipt {
    type Output = anyhow::Result<()>;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        std::pin::Pin::new(&mut self.0)
            .poll(cx)
            .map(|result| result.map_err(|_| anyhow::anyhow!("shared entity went away before ack")))
    }
}

/// The sending half backing a [`CallReceipt`]: the serialized response or an error string.
pub type ResponseSender = futures::channel::oneshot::Sender<Result<Vec<u8>, String>>;

/// Resolves with the home handler's return value. Responses are delivered after the
/// snapshot acking the call, so by the time a call resolves the local replica already
/// reflects it.
pub struct CallReceipt<R> {
    receiver: futures::channel::oneshot::Receiver<Result<Vec<u8>, String>>,
    _response: std::marker::PhantomData<fn() -> R>,
}

impl<R> CallReceipt<R> {
    #[doc(hidden)]
    pub fn channel() -> (ResponseSender, Self) {
        let (sender, receiver) = futures::channel::oneshot::channel();
        (
            sender,
            Self {
                receiver,
                _response: std::marker::PhantomData,
            },
        )
    }

    #[doc(hidden)]
    pub fn dropped() -> Self {
        let (_sender, receiver) = futures::channel::oneshot::channel();
        Self {
            receiver,
            _response: std::marker::PhantomData,
        }
    }
}

impl<R: DeserializeOwned> std::future::Future for CallReceipt<R> {
    type Output = anyhow::Result<R>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let this = self.get_mut();
        std::pin::Pin::new(&mut this.receiver).poll(cx).map(|result| {
            let outcome =
                result.map_err(|_| anyhow::anyhow!("shared entity went away before response"))?;
            let bytes = outcome.map_err(|error| anyhow::anyhow!("shared call failed: {error}"))?;
            decode(&bytes)
        })
    }
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

/// Reserved control method: a projection announcing interest in an entity. The home starts
/// publishing snapshots to it (and the subscribe's ack doubles as the initial snapshot).
pub const SUBSCRIBE_METHOD: &str = "$subscribe";

/// Reserved control method: a projection relinquishing an entity. Anonymous homes drop
/// their strong handle, letting the entity die when nothing else owns it.
pub const RELEASE_METHOD: &str = "$release";

/// Reserved control method: derive a weaker capability to the same entity. The payload is
/// the list of method names to keep (intersected with the caller's own table, so
/// attenuation is monotonic); the response is the new ref. Callable on any ref you hold —
/// no cooperation from the entity's author required.
pub const ATTENUATE_METHOD: &str = "$attenuate";

/// A serializable capability reference to a shared entity of kind `S`.
///
/// On the wire this is nothing but the entity id — refs travel *inside* snapshot and
/// message payloads (including call responses), so object graphs never need the name
/// namespace: names are mounts, refs are pointers. Possession of a ref is the authority to
/// send to it; sharing the same entity twice with different method tables mints attenuated
/// capabilities.
pub struct SharedRef<S: SharedSpec> {
    entity_id: u64,
    _spec: std::marker::PhantomData<fn() -> S>,
}

impl<S: SharedSpec> SharedRef<S> {
    #[doc(hidden)]
    pub fn from_raw(entity_id: u64) -> Self {
        Self {
            entity_id,
            _spec: std::marker::PhantomData,
        }
    }

    pub fn entity_id(&self) -> u64 {
        self.entity_id
    }
}

impl<S: SharedSpec> Clone for SharedRef<S> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<S: SharedSpec> Copy for SharedRef<S> {}

impl<S: SharedSpec> std::fmt::Debug for SharedRef<S> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "SharedRef<{}>({})", S::TYPE_NAME, self.entity_id)
    }
}

impl<S: SharedSpec> PartialEq for SharedRef<S> {
    fn eq(&self, other: &Self) -> bool {
        self.entity_id == other.entity_id
    }
}

impl<S: SharedSpec> Eq for SharedRef<S> {}

impl<S: SharedSpec> Serialize for SharedRef<S> {
    fn serialize<Ser: serde::Serializer>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error> {
        serializer.serialize_u64(self.entity_id)
    }
}

impl<'de, S: SharedSpec> serde::Deserialize<'de> for SharedRef<S> {
    fn deserialize<De: serde::Deserializer<'de>>(deserializer: De) -> Result<Self, De::Error> {
        Ok(Self::from_raw(u64::deserialize(deserializer)?))
    }
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
            $(message $method:literal $message:ident { $($message_field:ident : $message_ty:ty),* $(,)? } $(-> $response:ty)?)*
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

            #[allow(unused_parens)]
            impl $crate::SharedMessage for $message {
                type Spec = $spec;
                // `(T)` is just T; an absent response type leaves the unit type.
                type Response = ($($response)?);
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
            message "increment" Increment { by: u32 } -> u32
        }
    }

    crate::shared_schema! {
        entity TextSpec as "gpui-embedded.demo.text" {
            snapshot TextSnapshot { text: String }
        }
    }
}

/// Schemas for the integration tests in `crates/gpui_embedded/tests`.
#[doc(hidden)]
pub mod test_schema {
    use super::SharedRef;

    crate::shared_schema! {
        entity TestCounterSpec as "test.counter" {
            snapshot TestCounterSnapshot { count: u32 }
            message "increment" TestIncrement { by: u32 } -> u32
        }
    }

    crate::shared_schema! {
        entity ItemSpec as "test.item" {
            snapshot ItemSnapshot { label: String, bumps: u32 }
            message "bump" Bump {} -> u32
        }
    }

    crate::shared_schema! {
        entity FactorySpec as "test.factory" {
            snapshot FactorySnapshot { created: u32 }
            message "create" CreateItem { label: String } -> SharedRef<ItemSpec>
            message "create-readonly" CreateReadonlyItem { label: String } -> SharedRef<ItemSpec>
        }
    }

    crate::shared_schema! {
        entity ChameleonSpec as "test.chameleon" {
            snapshot ChameleonSnapshot { mode: String, pokes: u32 }
        }
    }
}
