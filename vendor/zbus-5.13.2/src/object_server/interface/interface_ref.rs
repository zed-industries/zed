use std::{marker::PhantomData, sync::Arc};

use super::{Interface, InterfaceDeref, InterfaceDerefMut, SignalEmitter};
use crate::async_lock::RwLock;

/// Wrapper over an interface, along with its corresponding `SignalEmitter`
/// instance. A reference to the underlying interface may be obtained via
/// [`InterfaceRef::get`] and [`InterfaceRef::get_mut`].
pub struct InterfaceRef<I> {
    pub(crate) emitter: SignalEmitter<'static>,
    pub(crate) lock: Arc<RwLock<dyn Interface>>,
    pub(crate) phantom: PhantomData<I>,
}

impl<I> InterfaceRef<I>
where
    I: 'static,
{
    /// Get a reference to the underlying interface.
    ///
    /// **WARNING:** If methods (e.g property setters) in `ObjectServer` require `&mut self`
    /// `ObjectServer` will not be able to access the interface in question until all references
    /// of this method are dropped; it is highly recommended that the scope of the interface
    /// returned is restricted.
    pub async fn get(&self) -> InterfaceDeref<'_, I> {
        let iface = self.lock.read().await;

        iface
            .downcast_ref::<I>()
            .expect("Unexpected interface type");

        InterfaceDeref {
            iface,
            phantom: PhantomData,
        }
    }

    /// Get a reference to the underlying interface.
    ///
    /// **WARNINGS:** Since the `ObjectServer` will not be able to access the interface in question
    /// until the return value of this method is dropped, it is highly recommended that the scope
    /// of the interface returned is restricted.
    ///
    /// # Errors
    ///
    /// If the interface at this instance's path is not valid, an `Error::InterfaceNotFound` error
    /// is returned.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # use async_io::block_on;
    /// # use zbus::{Connection, interface};
    ///
    /// struct MyIface(u32);
    ///
    /// #[interface(name = "org.myiface.MyIface")]
    /// impl MyIface {
    ///    #[zbus(property)]
    ///    async fn count(&self) -> u32 {
    ///        self.0
    ///    }
    /// }
    ///
    /// # block_on(async {
    /// // Set up connection and object_server etc here and then in another part of the code:
    /// # let connection = Connection::session().await?;
    /// #
    /// # let path = "/org/zbus/path";
    /// # connection.object_server().at(path, MyIface(22)).await?;
    /// let object_server = connection.object_server();
    /// let iface_ref = object_server.interface::<_, MyIface>(path).await?;
    /// let mut iface = iface_ref.get_mut().await;
    /// iface.0 = 42;
    /// iface.count_changed(iface_ref.signal_emitter()).await?;
    /// # Ok::<_, Box<dyn Error + Send + Sync>>(())
    /// # })?;
    /// #
    /// # Ok::<_, Box<dyn Error + Send + Sync>>(())
    /// ```
    pub async fn get_mut(&self) -> InterfaceDerefMut<'_, I> {
        let mut iface = self.lock.write().await;

        iface
            .downcast_ref::<I>()
            .expect("Unexpected interface type");
        iface
            .downcast_mut::<I>()
            .expect("Unexpected interface type");

        InterfaceDerefMut {
            iface,
            phantom: PhantomData,
        }
    }

    pub fn signal_emitter(&self) -> &SignalEmitter<'static> {
        &self.emitter
    }

    #[deprecated(since = "0.5.0", note = "Please use `signal_emitter` instead.")]
    pub fn signal_context(&self) -> &SignalEmitter<'static> {
        &self.emitter
    }
}

impl<I> Clone for InterfaceRef<I> {
    fn clone(&self) -> Self {
        Self {
            emitter: self.emitter.clone(),
            lock: self.lock.clone(),
            phantom: PhantomData,
        }
    }
}
