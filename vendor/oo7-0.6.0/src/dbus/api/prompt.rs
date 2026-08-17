use std::fmt;

use ashpd::WindowIdentifier;
use futures_util::StreamExt;
use serde::Serialize;
use zbus::zvariant::{ObjectPath, OwnedValue, Type};

use super::DESTINATION;
use crate::dbus::{Error, ServiceError};

#[derive(Type)]
#[zvariant(signature = "o")]
#[doc(alias = "org.freedesktop.Secret.Prompt")]
pub struct Prompt(zbus::Proxy<'static>);

impl zbus::proxy::Defaults for Prompt {
    const INTERFACE: &'static Option<zbus::names::InterfaceName<'static>> = &Some(
        zbus::names::InterfaceName::from_static_str_unchecked("org.freedesktop.Secret.Prompt"),
    );
    const DESTINATION: &'static Option<zbus::names::BusName<'static>> = &Some(DESTINATION);
    const PATH: &'static Option<ObjectPath<'static>> = &None;
}

impl From<zbus::Proxy<'static>> for Prompt {
    fn from(value: zbus::Proxy<'static>) -> Self {
        Self(value)
    }
}

impl Prompt {
    pub async fn new<P>(
        connection: &zbus::Connection,
        object_path: P,
    ) -> Result<Option<Self>, Error>
    where
        P: TryInto<ObjectPath<'static>>,
        P::Error: Into<zbus::Error>,
    {
        let path = object_path.try_into().map_err(Into::into)?;
        if path != ObjectPath::default() {
            Ok(Some(
                zbus::proxy::Builder::new(connection)
                    .path(path)?
                    .build()
                    .await?,
            ))
        } else {
            Ok(None)
        }
    }

    pub fn inner(&self) -> &zbus::Proxy<'static> {
        &self.0
    }

    pub async fn prompt(&self, window_id: Option<WindowIdentifier>) -> Result<(), Error> {
        let id = match window_id {
            Some(id) => id.to_string(),
            None => Default::default(),
        };
        self.inner()
            .call_method("Prompt", &(id))
            .await
            .map_err::<ServiceError, _>(From::from)?;
        Ok(())
    }

    pub async fn dismiss(&self) -> Result<(), Error> {
        self.inner()
            .call_method("Dismiss", &())
            .await
            .map_err::<ServiceError, _>(From::from)?;
        Ok(())
    }

    pub async fn receive_completed(
        &self,
        window_id: Option<WindowIdentifier>,
    ) -> Result<OwnedValue, Error> {
        let mut stream = self.inner().receive_signal("Completed").await?;
        let (value, _) = futures_util::try_join!(
            async {
                let message = stream.next().await.unwrap();
                let (dismissed, result) = message.body().deserialize::<(bool, OwnedValue)>()?;
                if dismissed {
                    Err(Error::Dismissed)
                } else {
                    Ok(result)
                }
            },
            self.prompt(window_id)
        )?;
        Ok(value)
    }
}

impl Serialize for Prompt {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        ObjectPath::serialize(self.inner().path(), serializer)
    }
}

impl fmt::Debug for Prompt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Prompt")
            .field(&self.inner().path().as_str())
            .finish()
    }
}
