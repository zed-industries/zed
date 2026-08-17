use zbus_names::{BusName, InterfaceName, MemberName};

use crate::{Connection, Error, Result, zvariant::ObjectPath};

/// A signal emitter.
///
/// For signal emission using the high-level API, you'll need instances of this type.
///
/// See [`crate::object_server::InterfaceRef::signal_emitter`] and [`crate::interface`]
/// documentation for details and examples of this type in use.
#[derive(Clone, Debug)]
pub struct SignalEmitter<'s> {
    conn: Connection,
    path: ObjectPath<'s>,
    destination: Option<BusName<'s>>,
}

impl<'s> SignalEmitter<'s> {
    /// Create a new signal context for the given connection and object path.
    pub fn new<P>(conn: &Connection, path: P) -> Result<Self>
    where
        P: TryInto<ObjectPath<'s>>,
        P::Error: Into<Error>,
    {
        path.try_into()
            .map(|p| Self {
                conn: conn.clone(),
                path: p,
                destination: None,
            })
            .map_err(Into::into)
    }

    /// Create a new signal context for the given connection and object path.
    pub fn from_parts(conn: Connection, path: ObjectPath<'s>) -> Self {
        Self {
            conn,
            path,
            destination: None,
        }
    }

    /// Emit a signal on the given interface with the given signal name and body.
    pub async fn emit<'i, 'm, I, M, B>(&self, interface: I, signal_name: M, body: &B) -> Result<()>
    where
        I: TryInto<InterfaceName<'i>>,
        I::Error: Into<Error>,
        M: TryInto<MemberName<'m>>,
        M::Error: Into<Error>,
        B: serde::ser::Serialize + zvariant::DynamicType,
    {
        self.conn
            .emit_signal(
                self.destination.as_ref(),
                &self.path,
                interface,
                signal_name,
                body,
            )
            .await
    }

    /// Set the destination for the signal emission.
    ///
    /// Signals are typically broadcasted and thus don't have a destination. However, there are
    /// cases where you need to unicast signals to specific peers. This method allows you to set the
    /// destination for the signals emitted with this context.
    pub fn set_destination(mut self, destination: BusName<'s>) -> Self {
        self.destination = Some(destination);

        self
    }

    /// Get a reference to the associated connection.
    pub fn connection(&self) -> &Connection {
        &self.conn
    }

    /// Get a reference to the associated object path.
    pub fn path(&self) -> &ObjectPath<'s> {
        &self.path
    }

    /// Get a reference to the associated destination (if any).
    pub fn destination(&self) -> Option<&BusName<'s>> {
        self.destination.as_ref()
    }

    /// Create an owned clone of `self`.
    pub fn to_owned(&self) -> SignalEmitter<'static> {
        SignalEmitter {
            conn: self.conn.clone(),
            path: self.path.to_owned(),
            destination: self.destination.as_ref().map(|d| d.to_owned()),
        }
    }

    /// Convert into an owned clone of `self`.
    pub fn into_owned(self) -> SignalEmitter<'static> {
        SignalEmitter {
            conn: self.conn,
            path: self.path.into_owned(),
            destination: self.destination.map(|d| d.into_owned()),
        }
    }
}
