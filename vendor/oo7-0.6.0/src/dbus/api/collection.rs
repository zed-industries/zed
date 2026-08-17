use std::{fmt, time::Duration};

use ashpd::WindowIdentifier;
use futures_util::{Stream, StreamExt};
use serde::Serialize;
use zbus::zvariant::{ObjectPath, OwnedObjectPath, Type};

use super::{DBusSecret, DESTINATION, Item, Prompt, Properties, Unlockable};
use crate::{
    AsAttributes,
    dbus::{Error, ServiceError},
};

#[derive(Type)]
#[zvariant(signature = "o")]
#[doc(alias = "org.freedesktop.Secret.Collection")]
pub struct Collection(zbus::Proxy<'static>);

impl zbus::proxy::Defaults for Collection {
    const INTERFACE: &'static Option<zbus::names::InterfaceName<'static>> = &Some(
        zbus::names::InterfaceName::from_static_str_unchecked("org.freedesktop.Secret.Collection"),
    );
    const DESTINATION: &'static Option<zbus::names::BusName<'static>> = &Some(DESTINATION);
    const PATH: &'static Option<ObjectPath<'static>> = &None;
}

impl From<zbus::Proxy<'static>> for Collection {
    fn from(value: zbus::Proxy<'static>) -> Self {
        Self(value)
    }
}

impl Collection {
    pub async fn new<P>(connection: &zbus::Connection, object_path: P) -> Result<Self, Error>
    where
        P: TryInto<ObjectPath<'static>>,
        P::Error: Into<zbus::Error>,
    {
        zbus::proxy::Builder::new(connection)
            .path(object_path)?
            .uncached_properties(&["Label", "Modified", "Items", "Locked"])
            .build()
            .await
            .map_err(From::from)
    }

    pub fn inner(&self) -> &zbus::Proxy<'static> {
        &self.0
    }

    pub(crate) async fn from_paths<P>(
        connection: &zbus::Connection,
        paths: Vec<P>,
    ) -> Result<Vec<Self>, Error>
    where
        P: TryInto<ObjectPath<'static>>,
        P::Error: Into<zbus::Error>,
    {
        let mut collections = Vec::with_capacity(paths.capacity());
        for path in paths.into_iter() {
            collections.push(Self::new(connection, path).await?);
        }
        Ok(collections)
    }

    #[doc(alias = "ItemCreated")]
    pub async fn receive_item_created(&self) -> Result<impl Stream<Item = Item> + '_, Error> {
        let stream = self.inner().receive_signal("ItemCreated").await?;
        let conn = self.inner().connection();
        Ok(stream.filter_map(move |message| async move {
            let path = message.body().deserialize::<OwnedObjectPath>().ok()?;
            Item::new(conn, path).await.ok()
        }))
    }

    #[doc(alias = "ItemDeleted")]
    pub async fn receive_item_deleted(&self) -> Result<impl Stream<Item = OwnedObjectPath>, Error> {
        let stream = self.inner().receive_signal("ItemDeleted").await?;
        Ok(stream.filter_map(move |message| async move {
            message.body().deserialize::<OwnedObjectPath>().ok()
        }))
    }

    #[doc(alias = "ItemChanged")]
    pub async fn receive_item_changed(&self) -> Result<impl Stream<Item = Item> + '_, Error> {
        let stream = self.inner().receive_signal("ItemChanged").await?;
        let conn = self.inner().connection();
        Ok(stream.filter_map(move |message| async move {
            let path = message.body().deserialize::<OwnedObjectPath>().ok()?;
            Item::new(conn, path).await.ok()
        }))
    }

    pub async fn items(&self) -> Result<Vec<Item>, Error> {
        let item_paths = self
            .inner()
            .get_property::<Vec<ObjectPath>>("Items")
            .await?;
        Item::from_paths(self.inner().connection(), item_paths).await
    }

    pub async fn label(&self) -> Result<String, Error> {
        self.inner().get_property("Label").await.map_err(From::from)
    }

    pub async fn set_label(&self, label: &str) -> Result<(), Error> {
        self.inner().set_property("Label", label).await?;
        Ok(())
    }

    #[doc(alias = "Locked")]
    pub async fn is_locked(&self) -> Result<bool, Error> {
        self.inner()
            .get_property("Locked")
            .await
            .map_err(From::from)
    }

    pub async fn created(&self) -> Result<Duration, Error> {
        let time = self.inner().get_property::<u64>("Created").await?;
        Ok(Duration::from_secs(time))
    }

    pub async fn modified(&self) -> Result<Duration, Error> {
        let time = self.inner().get_property::<u64>("Modified").await?;
        Ok(Duration::from_secs(time))
    }

    pub async fn delete(&self, window_id: Option<WindowIdentifier>) -> Result<(), Error> {
        let prompt_path = self
            .inner()
            .call_method("Delete", &())
            .await
            .map_err::<ServiceError, _>(From::from)?
            .body()
            .deserialize::<OwnedObjectPath>()?;
        if let Some(prompt) = Prompt::new(self.inner().connection(), prompt_path).await? {
            let _ = prompt.receive_completed(window_id).await?;
        }
        Ok(())
    }

    #[doc(alias = "SearchItems")]
    pub async fn search_items(&self, attributes: &impl AsAttributes) -> Result<Vec<Item>, Error> {
        let msg = self
            .inner()
            .call_method("SearchItems", &(attributes.as_attributes()))
            .await
            .map_err::<ServiceError, _>(From::from)?;

        let item_paths = msg.body().deserialize::<Vec<OwnedObjectPath>>()?;
        Item::from_paths(self.inner().connection(), item_paths).await
    }

    #[doc(alias = "CreateItem")]
    pub async fn create_item(
        &self,
        label: &str,
        attributes: &impl AsAttributes,
        secret: &DBusSecret,
        replace: bool,
        window_id: Option<WindowIdentifier>,
    ) -> Result<Item, Error> {
        let properties = Properties::for_item(label, attributes);
        let (item_path, prompt_path) = self
            .inner()
            .call_method("CreateItem", &(properties, secret, replace))
            .await
            .map_err::<ServiceError, _>(From::from)?
            .body()
            .deserialize::<(OwnedObjectPath, OwnedObjectPath)>()?;
        let cnx = self.inner().connection();
        let item_path = if let Some(prompt) = Prompt::new(cnx, prompt_path).await? {
            let response = prompt.receive_completed(window_id).await?;
            OwnedObjectPath::try_from(response)?
        } else {
            item_path
        };
        Item::new(self.inner().connection(), item_path).await
    }
}

impl Serialize for Collection {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        ObjectPath::serialize(self.inner().path(), serializer)
    }
}

impl fmt::Debug for Collection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Collection")
            .field(&self.inner().path().as_str())
            .finish()
    }
}

impl Unlockable for Collection {}
