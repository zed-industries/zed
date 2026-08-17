use std::{sync::Arc, time::Duration};

use ashpd::WindowIdentifier;
#[cfg(feature = "async-std")]
use async_lock::RwLock;
use futures_util::{Stream, StreamExt};
#[cfg(feature = "tokio")]
use tokio::sync::RwLock;
use zbus::zvariant::{ObjectPath, OwnedObjectPath};

use super::{Algorithm, Error, Item, api};
use crate::{AsAttributes, Key, Secret};

/// A collection allows to store and retrieve items.
///
/// The collection can be either in a locked or unlocked state, use
/// [`Collection::lock`] or [`Collection::unlock`] to lock or unlock it.
///
/// Using [`Collection::search_items`] or [`Collection::items`] will return no
/// items if the collection is locked.
///
/// **Note**
///
/// If the collection is deleted using [`Collection::delete`] any future usage
/// of it API will fail with [`Error::Deleted`].
#[derive(Debug)]
pub struct Collection {
    inner: Arc<api::Collection>,
    service: Arc<api::Service>,
    session: Arc<api::Session>,
    algorithm: Algorithm,
    /// Defines whether the Collection has been deleted or not
    available: RwLock<bool>,
    aes_key: Option<Arc<Key>>,
}

impl Collection {
    pub(crate) fn new(
        service: Arc<api::Service>,
        session: Arc<api::Session>,
        algorithm: Algorithm,
        collection: api::Collection,
        aes_key: Option<Arc<Key>>,
    ) -> Self {
        Self {
            inner: Arc::new(collection),
            session,
            service,
            algorithm,
            available: RwLock::new(true),
            aes_key,
        }
    }

    pub(crate) async fn is_available(&self) -> bool {
        *self.available.read().await
    }

    /// Retrieve the list of available [`Item`] in the collection.
    pub async fn items(&self) -> Result<Vec<Item>, Error> {
        if !self.is_available().await {
            Err(Error::Deleted)
        } else {
            Ok(self
                .inner
                .items()
                .await?
                .into_iter()
                .map(|item| self.new_item(item))
                .collect::<Vec<_>>())
        }
    }

    /// The collection label.
    pub async fn label(&self) -> Result<String, Error> {
        if !self.is_available().await {
            Err(Error::Deleted)
        } else {
            self.inner.label().await
        }
    }

    /// Set the collection label.
    pub async fn set_label(&self, label: &str) -> Result<(), Error> {
        if !self.is_available().await {
            Err(Error::Deleted)
        } else {
            self.inner.set_label(label).await
        }
    }

    /// Get whether the collection is locked.
    #[doc(alias = "Locked")]
    pub async fn is_locked(&self) -> Result<bool, Error> {
        if !self.is_available().await {
            Err(Error::Deleted)
        } else {
            self.inner.is_locked().await
        }
    }

    /// The UNIX time when the collection was created.
    pub async fn created(&self) -> Result<Duration, Error> {
        if !self.is_available().await {
            Err(Error::Deleted)
        } else {
            self.inner.created().await
        }
    }

    /// The UNIX time when the collection was modified.
    pub async fn modified(&self) -> Result<Duration, Error> {
        if !self.is_available().await {
            Err(Error::Deleted)
        } else {
            self.inner.modified().await
        }
    }

    /// Search for items based on their attributes.
    pub async fn search_items(&self, attributes: &impl AsAttributes) -> Result<Vec<Item>, Error> {
        if !self.is_available().await {
            Err(Error::Deleted)
        } else {
            let items = self.inner.search_items(attributes).await?;
            Ok(items
                .into_iter()
                .map(|item| self.new_item(item))
                .collect::<Vec<_>>())
        }
    }

    /// Create a new item on the collection
    ///
    /// # Arguments
    ///
    /// * `label` - A user visible label of the item.
    /// * `attributes` - A map of key/value attributes, used to find the item
    ///   later.
    /// * `secret` - The secret to store.
    /// * `replace` - Whether to replace the value if the `attributes` matches
    ///   an existing `secret`.
    pub async fn create_item(
        &self,
        label: &str,
        attributes: &impl AsAttributes,
        secret: impl Into<Secret>,
        replace: bool,
        window_id: Option<WindowIdentifier>,
    ) -> Result<Item, Error> {
        if !self.is_available().await {
            Err(Error::Deleted)
        } else {
            let secret = match self.algorithm {
                Algorithm::Plain => api::DBusSecret::new(Arc::clone(&self.session), secret),
                Algorithm::Encrypted => api::DBusSecret::new_encrypted(
                    Arc::clone(&self.session),
                    secret,
                    self.aes_key.as_ref().unwrap(),
                )?,
            };
            let item = self
                .inner
                .create_item(label, attributes, &secret, replace, window_id)
                .await?;

            Ok(self.new_item(item))
        }
    }

    /// Unlock the collection.
    pub async fn unlock(&self, window_id: Option<WindowIdentifier>) -> Result<(), Error> {
        if !self.is_available().await {
            Err(Error::Deleted)
        } else {
            self.service
                .unlock(&[self.inner.inner().path()], window_id)
                .await?;
            Ok(())
        }
    }

    /// Lock the collection.
    pub async fn lock(&self, window_id: Option<WindowIdentifier>) -> Result<(), Error> {
        if !self.is_available().await {
            Err(Error::Deleted)
        } else {
            self.service
                .lock(&[self.inner.inner().path()], window_id)
                .await?;
            Ok(())
        }
    }

    /// Delete the collection.
    pub async fn delete(&self, window_id: Option<WindowIdentifier>) -> Result<(), Error> {
        if !self.is_available().await {
            Err(Error::Deleted)
        } else {
            self.inner.delete(window_id).await?;
            *self.available.write().await = false;
            Ok(())
        }
    }

    /// Returns collection path
    pub fn path(&self) -> &ObjectPath<'_> {
        self.inner.inner().path()
    }

    /// Stream yielding when new items get created
    pub async fn receive_item_created(&self) -> Result<impl Stream<Item = Item> + '_, Error> {
        Ok(self
            .inner
            .receive_item_created()
            .await?
            .map(|item| self.new_item(item)))
    }

    /// Stream yielding when existing items get changed
    pub async fn receive_item_changed(&self) -> Result<impl Stream<Item = Item> + '_, Error> {
        Ok(self
            .inner
            .receive_item_changed()
            .await?
            .map(|item| self.new_item(item)))
    }

    /// Stream yielding when existing items get deleted
    pub async fn receive_item_deleted(&self) -> Result<impl Stream<Item = OwnedObjectPath>, Error> {
        self.inner.receive_item_deleted().await
    }

    // Get public `Item`` from `api::Item`
    fn new_item(&self, item: api::Item) -> Item {
        Item::new(
            Arc::clone(&self.service),
            Arc::clone(&self.session),
            self.algorithm,
            item,
            self.aes_key.clone(), // Cheap clone, it is an Arc,
        )
    }
}
