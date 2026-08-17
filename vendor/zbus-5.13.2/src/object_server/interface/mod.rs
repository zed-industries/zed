mod dispatch_result;
pub use dispatch_result::*;
mod interface_ref;
pub use interface_ref::*;
mod interface_deref;
pub use interface_deref::*;

use std::{
    any::{Any, TypeId},
    collections::HashMap,
    fmt::{self, Write},
    sync::Arc,
};

use async_trait::async_trait;
use zbus_names::{InterfaceName, MemberName};
use zvariant::{OwnedValue, Value};

use crate::{
    Connection, ObjectServer,
    async_lock::RwLock,
    fdo,
    message::{self, Header, Message},
    object_server::SignalEmitter,
};

/// This trait is used to dispatch messages to an interface instance.
///
/// This trait should be treated as an unstable API and compatibility may break in minor
/// version bumps. Because of this and other reasons, it is not recommended to manually implement
/// this trait. The [`crate::interface`] macro implements it for you.
///
/// If you have an advanced use case where `interface` is inadequate, consider using
/// [`crate::MessageStream`] or [`crate::blocking::MessageIterator`] instead.
#[async_trait]
pub trait Interface: Any + Send + Sync {
    /// Return the name of the interface. Ex: "org.foo.MyInterface"
    fn name() -> InterfaceName<'static>
    where
        Self: Sized;

    /// Whether each method call will be handled from a different spawned task.
    ///
    /// Note: When methods are called from separate tasks, they may not be run in the order in which
    /// they were called.
    fn spawn_tasks_for_methods(&self) -> bool {
        true
    }

    /// Get a property value. Returns `None` if the property doesn't exist.
    ///
    /// Note: The header parameter will be None when the getter is not being called as part
    /// of D-Bus communication (for example, when it is called as part of initial object setup,
    /// before it is registered on the bus, or when we manually send out property changed
    /// notifications).
    async fn get(
        &self,
        property_name: &str,
        server: &ObjectServer,
        connection: &Connection,
        header: Option<&message::Header<'_>>,
        emitter: &SignalEmitter<'_>,
    ) -> Option<fdo::Result<OwnedValue>>;

    /// Return all the properties.
    async fn get_all(
        &self,
        object_server: &ObjectServer,
        connection: &Connection,
        header: Option<&message::Header<'_>>,
        emitter: &SignalEmitter<'_>,
    ) -> fdo::Result<HashMap<String, OwnedValue>>;

    /// Set a property value.
    ///
    /// Return [`DispatchResult::NotFound`] if the property doesn't exist, or
    /// [`DispatchResult::RequiresMut`] if `set_mut` should be used instead. The default
    /// implementation just returns `RequiresMut`.
    fn set<'call>(
        &'call self,
        property_name: &'call str,
        value: &'call Value<'_>,
        object_server: &'call ObjectServer,
        connection: &'call Connection,
        header: Option<&'call message::Header<'_>>,
        emitter: &'call SignalEmitter<'_>,
    ) -> DispatchResult<'call> {
        let _ = (
            property_name,
            value,
            object_server,
            connection,
            header,
            emitter,
        );
        DispatchResult::RequiresMut
    }

    /// Set a property value.
    ///
    /// Returns `None` if the property doesn't exist.
    ///
    /// This will only be invoked if `set` returned `RequiresMut`.
    async fn set_mut(
        &mut self,
        property_name: &str,
        value: &Value<'_>,
        object_server: &ObjectServer,
        connection: &Connection,
        header: Option<&Header<'_>>,
        emitter: &SignalEmitter<'_>,
    ) -> Option<fdo::Result<()>>;

    /// Call a method.
    ///
    /// Return [`DispatchResult::NotFound`] if the method doesn't exist, or
    /// [`DispatchResult::RequiresMut`] if `call_mut` should be used instead.
    ///
    /// It is valid, though inefficient, for this to always return `RequiresMut`.
    fn call<'call>(
        &'call self,
        server: &'call ObjectServer,
        connection: &'call Connection,
        msg: &'call Message,
        name: MemberName<'call>,
    ) -> DispatchResult<'call>;

    /// Call a `&mut self` method.
    ///
    /// This will only be invoked if `call` returned `RequiresMut`.
    fn call_mut<'call>(
        &'call mut self,
        server: &'call ObjectServer,
        connection: &'call Connection,
        msg: &'call Message,
        name: MemberName<'call>,
    ) -> DispatchResult<'call>;

    /// Write introspection XML to the writer, with the given indentation level.
    fn introspect_to_writer(&self, writer: &mut dyn Write, level: usize);
}

/// A type for a reference-counted Interface trait-object, with associated run-time details and a
/// manual Debug impl.
#[derive(Clone)]
pub(crate) struct ArcInterface {
    pub instance: Arc<RwLock<dyn Interface>>,
    pub spawn_tasks_for_methods: bool,
}

impl ArcInterface {
    pub fn new<I>(iface: I) -> Self
    where
        I: Interface,
    {
        let spawn_tasks_for_methods = iface.spawn_tasks_for_methods();
        Self {
            instance: Arc::new(RwLock::new(iface)),
            spawn_tasks_for_methods,
        }
    }
}

impl fmt::Debug for ArcInterface {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Arc<RwLock<dyn Interface>>")
            .finish_non_exhaustive()
    }
}

// Note: while it is possible to implement this without `unsafe`, it currently requires a helper
// trait with a blanket impl that creates `dyn Any` refs.  It's simpler (and more performant) to
// just check the type ID and do the downcast ourself.
//
// See https://github.com/rust-lang/rust/issues/65991 for a rustc feature that will make it
// possible to get a `dyn Any` ref directly from a `dyn Interface` ref; once that is stable, we can
// remove this unsafe code.
impl dyn Interface {
    /// Return Any of self
    pub(crate) fn downcast_ref<T: Any>(&self) -> Option<&T> {
        if <dyn Interface as Any>::type_id(self) == TypeId::of::<T>() {
            // SAFETY: If type ID matches, it means object is of type T
            Some(unsafe { &*(self as *const dyn Interface as *const T) })
        } else {
            None
        }
    }

    /// Return Any of self
    pub(crate) fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
        if <dyn Interface as Any>::type_id(self) == TypeId::of::<T>() {
            // SAFETY: If type ID matches, it means object is of type T
            Some(unsafe { &mut *(self as *mut dyn Interface as *mut T) })
        } else {
            None
        }
    }
}
