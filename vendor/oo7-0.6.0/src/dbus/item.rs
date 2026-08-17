use std::{collections::HashMap, sync::Arc, time::Duration};

use ashpd::WindowIdentifier;
#[cfg(feature = "async-std")]
use async_lock::RwLock;
#[cfg(feature = "tokio")]
use tokio::sync::RwLock;
use zbus::zvariant::ObjectPath;

use super::{Algorithm, Error, api};
use crate::{AsAttributes, Key, Secret};

/// A secret with a label and attributes to identify it.
///
/// An item might be locked or unlocked, use [`Item::lock`] or [`Item::unlock`]
/// to lock or unlock it. Note that the Secret Service might not be able to
/// lock/unlock individual items and may lock/unlock the entire collection in
/// such case.
///
/// The item is attributes are used to identify and find the item later using
/// [`Collection::search_items`](crate::dbus::Collection::search_items).
/// They are not stored or transferred in a secure manner.
///
/// **Note**
///
/// If the item is deleted using [`Item::delete`] any future usage of it API
/// will fail with [`Error::Deleted`].
#[derive(Debug)]
pub struct Item {
    inner: Arc<api::Item>,
    session: Arc<api::Session>,
    service: Arc<api::Service>,
    algorithm: Algorithm,
    /// Defines whether the Item has been deleted or not
    available: RwLock<bool>,
    aes_key: Option<Arc<Key>>,
}

impl Item {
    pub(crate) fn new(
        service: Arc<api::Service>,
        session: Arc<api::Session>,
        algorithm: Algorithm,
        item: api::Item,
        aes_key: Option<Arc<Key>>,
    ) -> Self {
        Self {
            inner: Arc::new(item),
            service,
            session,
            algorithm,
            available: RwLock::new(true),
            aes_key,
        }
    }

    pub(crate) async fn is_available(&self) -> bool {
        *self.available.read().await
    }

    /// Get whether the item is locked.
    pub async fn is_locked(&self) -> Result<bool, Error> {
        if !self.is_available().await {
            Err(Error::Deleted)
        } else {
            self.inner.is_locked().await
        }
    }

    /// The item label.
    pub async fn label(&self) -> Result<String, Error> {
        if !self.is_available().await {
            Err(Error::Deleted)
        } else {
            self.inner.label().await
        }
    }

    /// Set the item label.
    pub async fn set_label(&self, label: &str) -> Result<(), Error> {
        if !self.is_available().await {
            Err(Error::Deleted)
        } else {
            self.inner.set_label(label).await
        }
    }

    /// The UNIX time when the item was created.
    pub async fn created(&self) -> Result<Duration, Error> {
        if !self.is_available().await {
            Err(Error::Deleted)
        } else {
            self.inner.created().await
        }
    }

    /// The UNIX time when the item was modified.
    pub async fn modified(&self) -> Result<Duration, Error> {
        if !self.is_available().await {
            Err(Error::Deleted)
        } else {
            self.inner.modified().await
        }
    }

    /// Retrieve the item attributes.
    pub async fn attributes(&self) -> Result<HashMap<String, String>, Error> {
        if !self.is_available().await {
            Err(Error::Deleted)
        } else {
            self.inner.attributes().await
        }
    }

    /// Retrieve the item attributes as a typed schema.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use oo7::{SecretSchema, dbus::Item};
    /// # #[derive(SecretSchema, Debug)]
    /// # #[schema(name = "org.example.Password")]
    /// # struct PasswordSchema {
    /// #     username: String,
    /// #     server: String,
    /// # }
    /// # async fn example(item: &Item) -> Result<(), Box<dyn std::error::Error>> {
    /// let schema = item.attributes_as::<PasswordSchema>().await?;
    /// println!("Username: {}", schema.username);
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "schema")]
    #[cfg_attr(docsrs, doc(cfg(feature = "schema")))]
    pub async fn attributes_as<T>(&self) -> Result<T, Error>
    where
        T: std::convert::TryFrom<HashMap<String, String>, Error = crate::SchemaError>,
    {
        let attrs = self.attributes().await?;
        T::try_from(attrs).map_err(Into::into)
    }

    /// Update the item attributes.
    pub async fn set_attributes(&self, attributes: &impl AsAttributes) -> Result<(), Error> {
        if !self.is_available().await {
            Err(Error::Deleted)
        } else {
            self.inner.set_attributes(attributes).await
        }
    }

    /// Delete the item.
    pub async fn delete(&self, window_id: Option<WindowIdentifier>) -> Result<(), Error> {
        if !self.is_available().await {
            Err(Error::Deleted)
        } else {
            self.inner.delete(window_id).await?;
            *self.available.write().await = false;
            Ok(())
        }
    }

    /// Retrieve the currently stored secret.
    pub async fn secret(&self) -> Result<Secret, Error> {
        if !self.is_available().await {
            Err(Error::Deleted)
        } else {
            self.inner
                .secret(&self.session)
                .await?
                .decrypt(self.aes_key.as_ref())
        }
    }

    /// Modify the stored secret on the item.
    ///
    /// # Arguments
    ///
    /// * `secret` - The secret to store.
    #[doc(alias = "SetSecret")]
    pub async fn set_secret(&self, secret: impl Into<Secret>) -> Result<(), Error> {
        if !self.is_available().await {
            Err(Error::Deleted)
        } else {
            let secret = match self.algorithm {
                Algorithm::Plain => api::DBusSecret::new(Arc::clone(&self.session), secret),
                Algorithm::Encrypted => {
                    let aes_key = self.aes_key.as_ref().unwrap();
                    api::DBusSecret::new_encrypted(Arc::clone(&self.session), secret, aes_key)?
                }
            };
            self.inner.set_secret(&secret).await?;
            Ok(())
        }
    }

    /// Unlock the item.
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

    /// Lock the item.
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

    /// Returns item path
    pub fn path(&self) -> &ObjectPath<'_> {
        self.inner.inner().path()
    }
}
