use std::fmt;

use serde::Serialize;
use zbus::zvariant::{ObjectPath, Type};

use super::DESTINATION;
use crate::dbus::{Error, ServiceError};

#[derive(Type)]
#[zvariant(signature = "o")]
#[doc(alias = "org.freedesktop.Secret.Session")]
pub struct Session(zbus::Proxy<'static>);

impl zbus::proxy::Defaults for Session {
    const INTERFACE: &'static Option<zbus::names::InterfaceName<'static>> = &Some(
        zbus::names::InterfaceName::from_static_str_unchecked("org.freedesktop.Secret.Session"),
    );
    const DESTINATION: &'static Option<zbus::names::BusName<'static>> = &Some(DESTINATION);
    const PATH: &'static Option<ObjectPath<'static>> = &None;
}

impl From<zbus::Proxy<'static>> for Session {
    fn from(value: zbus::Proxy<'static>) -> Self {
        Self(value)
    }
}

impl Session {
    pub async fn new<P>(connection: &zbus::Connection, object_path: P) -> Result<Self, Error>
    where
        P: TryInto<ObjectPath<'static>>,
        P::Error: Into<zbus::Error>,
    {
        zbus::proxy::Builder::new(connection)
            .path(object_path)?
            .build()
            .await
            .map_err(From::from)
    }

    pub fn inner(&self) -> &zbus::Proxy<'static> {
        &self.0
    }

    pub async fn close(&self) -> Result<(), Error> {
        self.inner()
            .call_method("Close", &())
            .await
            .map_err::<ServiceError, _>(From::from)?;
        Ok(())
    }
}

impl Serialize for Session {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        ObjectPath::serialize(self.inner().path(), serializer)
    }
}

impl fmt::Debug for Session {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Session")
            .field(&self.inner().path().as_str())
            .finish()
    }
}
