use std::{collections::HashMap, fmt};

use ashpd::WindowIdentifier;
use futures_util::{Stream, StreamExt};
use zbus::zvariant::{ObjectPath, OwnedObjectPath, OwnedValue, Type, Value};

use super::{
    Collection, DBusSecret, DESTINATION, Item, PATH, Prompt, Properties, Session, Unlockable,
};
use crate::{
    AsAttributes, Key,
    dbus::{Algorithm, Error, ServiceError},
};

#[derive(Type)]
#[zvariant(signature = "o")]
#[doc(alias = "org.freedesktop.secrets")]
pub struct Service(zbus::Proxy<'static>);

impl zbus::proxy::Defaults for Service {
    const INTERFACE: &'static Option<zbus::names::InterfaceName<'static>> = &Some(
        zbus::names::InterfaceName::from_static_str_unchecked("org.freedesktop.Secret.Service"),
    );
    const DESTINATION: &'static Option<zbus::names::BusName<'static>> = &Some(DESTINATION);
    const PATH: &'static Option<ObjectPath<'static>> = &Some(PATH);
}

impl From<zbus::Proxy<'static>> for Service {
    fn from(value: zbus::Proxy<'static>) -> Self {
        Self(value)
    }
}

impl Service {
    pub async fn new(connection: &zbus::Connection) -> Result<Self, Error> {
        zbus::proxy::Builder::new(connection)
            .build()
            .await
            .map_err(From::from)
    }

    pub fn inner(&self) -> &zbus::Proxy<'static> {
        &self.0
    }

    #[doc(alias = "CollectionCreated")]
    pub async fn receive_collection_created(
        &self,
    ) -> Result<impl Stream<Item = Collection> + '_, Error> {
        let stream = self.inner().receive_signal("CollectionCreated").await?;
        let conn = self.inner().connection();
        Ok(stream.filter_map(move |message| async move {
            let path = message.body().deserialize::<OwnedObjectPath>().ok()?;
            Collection::new(conn, path).await.ok()
        }))
    }

    #[doc(alias = "CollectionDeleted")]
    pub async fn receive_collection_deleted(
        &self,
    ) -> Result<impl Stream<Item = OwnedObjectPath>, Error> {
        let stream = self.inner().receive_signal("CollectionDeleted").await?;
        Ok(stream.filter_map(move |message| async move {
            message.body().deserialize::<OwnedObjectPath>().ok()
        }))
    }

    #[doc(alias = "CollectionChanged")]
    pub async fn receive_collection_changed(
        &self,
    ) -> Result<impl Stream<Item = Collection> + '_, Error> {
        let stream = self.inner().receive_signal("CollectionChanged").await?;
        let conn = self.inner().connection();
        Ok(stream.filter_map(move |message| async move {
            let path = message.body().deserialize::<OwnedObjectPath>().ok()?;
            Collection::new(conn, path).await.ok()
        }))
    }

    pub async fn collections(&self) -> Result<Vec<Collection>, Error> {
        let collections_paths = self
            .inner()
            .get_property::<Vec<ObjectPath>>("Collections")
            .await?;
        Collection::from_paths(self.inner().connection(), collections_paths).await
    }

    #[doc(alias = "OpenSession")]
    pub async fn open_session(
        &self,
        client_public_key: Option<Key>,
    ) -> Result<(Option<Key>, Session), Error> {
        let (algorithm, key): (_, Value<'_>) = match client_public_key {
            None => (Algorithm::Plain, zvariant::Str::default().into()),
            Some(key) => (Algorithm::Encrypted, key.into()),
        };
        let (service_key, session_path) = self
            .inner()
            .call_method("OpenSession", &(&algorithm, key))
            .await
            .map_err::<ServiceError, _>(From::from)?
            .body()
            .deserialize::<(OwnedValue, OwnedObjectPath)>()?;
        let session = Session::new(self.inner().connection(), session_path).await?;

        let key = match algorithm {
            Algorithm::Plain => None,
            Algorithm::Encrypted => Some(Key::try_from(service_key)?),
        };

        Ok((key, session))
    }

    #[doc(alias = "CreateCollection")]
    pub async fn create_collection(
        &self,
        label: &str,
        alias: Option<&str>,
        window_id: Option<WindowIdentifier>,
    ) -> Result<Collection, Error> {
        let properties = Properties::for_collection(label);
        let (collection_path, prompt_path) = self
            .inner()
            .call_method("CreateCollection", &(properties, alias.unwrap_or_default()))
            .await
            .map_err::<ServiceError, _>(From::from)?
            .body()
            .deserialize::<(OwnedObjectPath, OwnedObjectPath)>()?;

        let collection_path =
            if let Some(prompt) = Prompt::new(self.inner().connection(), prompt_path).await? {
                let response = prompt.receive_completed(window_id).await?;
                OwnedObjectPath::try_from(response)?
            } else {
                collection_path
            };
        Collection::new(self.inner().connection(), collection_path).await
    }

    #[doc(alias = "SearchItems")]
    pub async fn search_items(
        &self,
        attributes: &impl AsAttributes,
    ) -> Result<(Vec<Item>, Vec<Item>), Error> {
        let (unlocked_item_paths, locked_item_paths) = self
            .inner()
            .call_method("SearchItems", &(attributes.as_attributes()))
            .await
            .map_err::<ServiceError, _>(From::from)?
            .body()
            .deserialize::<(Vec<OwnedObjectPath>, Vec<OwnedObjectPath>)>()?;
        let cnx = self.inner().connection();

        let unlocked_items = Item::from_paths(cnx, unlocked_item_paths).await?;
        let locked_items = Item::from_paths(cnx, locked_item_paths).await?;

        Ok((unlocked_items, locked_items))
    }

    pub async fn unlock(
        &self,
        items: &[impl Unlockable],
        window_id: Option<WindowIdentifier>,
    ) -> Result<Vec<OwnedObjectPath>, Error> {
        let (mut unlocked_item_paths, prompt_path) = self
            .inner()
            .call_method("Unlock", &(items))
            .await
            .map_err::<ServiceError, _>(From::from)?
            .body()
            .deserialize::<(Vec<OwnedObjectPath>, OwnedObjectPath)>()?;
        let cnx = self.inner().connection();

        if let Some(prompt) = Prompt::new(cnx, prompt_path).await? {
            let response = prompt.receive_completed(window_id).await?;
            let locked_paths = Vec::<OwnedObjectPath>::try_from(response)?;
            unlocked_item_paths.extend(locked_paths);
        };
        Ok(unlocked_item_paths)
    }

    pub async fn lock(
        &self,
        items: &[impl Unlockable],
        window_id: Option<WindowIdentifier>,
    ) -> Result<Vec<OwnedObjectPath>, Error> {
        let (mut locked_item_paths, prompt_path) = self
            .inner()
            .call_method("Lock", &(items))
            .await
            .map_err::<ServiceError, _>(From::from)?
            .body()
            .deserialize::<(Vec<OwnedObjectPath>, OwnedObjectPath)>()?;
        let cnx = self.inner().connection();

        if let Some(prompt) = Prompt::new(cnx, prompt_path).await? {
            let response = prompt.receive_completed(window_id).await?;
            let locked_paths = Vec::<OwnedObjectPath>::try_from(response)?;
            locked_item_paths.extend(locked_paths);
        };

        Ok(locked_item_paths)
    }

    #[doc(alias = "GetSecrets")]
    pub async fn secrets(
        &self,
        items: &[Item],
        session: &Session,
    ) -> Result<HashMap<Item, DBusSecret>, Error> {
        let secrets = self
            .inner()
            .call_method("GetSecrets", &(items, session))
            .await
            .map_err::<ServiceError, _>(From::from)?
            .body()
            .deserialize::<HashMap<OwnedObjectPath, super::secret::DBusSecretInner>>()?;

        let cnx = self.inner().connection();
        // Item's Hash implementation doesn't make use of any mutable internals
        #[allow(clippy::mutable_key_type)]
        let mut output = HashMap::with_capacity(secrets.capacity());
        for (path, secret_inner) in secrets {
            output.insert(
                Item::new(cnx, path).await?,
                DBusSecret::from_inner(cnx, secret_inner).await?,
            );
        }

        Ok(output)
    }

    #[doc(alias = "ReadAlias")]
    pub async fn read_alias(&self, name: &str) -> Result<Option<Collection>, Error> {
        let collection_path = self
            .inner()
            .call_method("ReadAlias", &(name))
            .await
            .map_err::<ServiceError, _>(From::from)?
            .body()
            .deserialize::<OwnedObjectPath>()?;

        if collection_path != OwnedObjectPath::default() {
            let collection = Collection::new(self.inner().connection(), collection_path).await?;
            Ok(Some(collection))
        } else {
            Ok(None)
        }
    }

    #[doc(alias = "SetAlias")]
    pub async fn set_alias(&self, name: &str, collection: &Collection) -> Result<(), Error> {
        self.inner()
            .call_method("SetAlias", &(name, collection))
            .await
            .map_err::<ServiceError, _>(From::from)?;
        Ok(())
    }
}

impl fmt::Debug for Service {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Service")
            .field(&self.inner().path().as_str())
            .finish()
    }
}
