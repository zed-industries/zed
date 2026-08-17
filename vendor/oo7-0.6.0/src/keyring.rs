use std::{collections::HashMap, sync::Arc, time::Duration};

#[cfg(feature = "async-std")]
use async_lock::RwLock;
#[cfg(feature = "tokio")]
use tokio::sync::RwLock;

use crate::{AsAttributes, Result, Secret, dbus, file};

/// A [Secret Service](crate::dbus) or [file](crate::file) backed keyring
/// implementation.
///
/// It will automatically use the file backend if the application is sandboxed
/// and otherwise falls back to the DBus service using it [default
/// collection](crate::dbus::Service::default_collection).
///
/// The File backend requires a [`org.freedesktop.portal.Secret`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Secret.html) implementation
/// to retrieve the key that will be used to encrypt the backend file.
#[derive(Debug)]
pub enum Keyring {
    #[doc(hidden)]
    File(Arc<RwLock<Option<file::Keyring>>>),
    #[doc(hidden)]
    DBus(dbus::Collection),
}

impl Keyring {
    /// Create a new instance of the Keyring.
    pub async fn new() -> Result<Self> {
        if ashpd::is_sandboxed() {
            #[cfg(feature = "tracing")]
            tracing::debug!("Application is sandboxed, using the file backend");

            let secret = Secret::from(
                ashpd::desktop::secret::retrieve()
                    .await
                    .map_err(crate::file::Error::from)?,
            );
            match file::UnlockedKeyring::load(
                crate::file::api::Keyring::default_path()?,
                secret.clone(),
            )
            .await
            {
                Ok(file) => {
                    return Ok(Self::File(Arc::new(RwLock::new(Some(
                        file::Keyring::Unlocked(file),
                    )))));
                }
                // Do nothing in this case, we are supposed to fallback to the host keyring
                Err(super::file::Error::Portal(ashpd::Error::PortalNotFound(_))) => {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "org.freedesktop.portal.Secrets is not available, falling back to the Secret Service backend"
                    );
                }
                Err(e) => {
                    return Err(crate::Error::File(e));
                }
            };
        } else {
            #[cfg(feature = "tracing")]
            tracing::debug!(
                "Application is not sandboxed, falling back to the Secret Service backend"
            );
        }
        let service = dbus::Service::new().await?;
        let collection = service.default_collection().await?;
        Ok(Self::DBus(collection))
    }

    /// Unlock the used collection.
    pub async fn unlock(&self) -> Result<()> {
        match self {
            Self::DBus(backend) => backend.unlock(None).await?,
            Self::File(keyring) => {
                let mut kg = keyring.write().await;
                let kg_value = kg.take();
                if let Some(file::Keyring::Locked(locked)) = kg_value {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("Unlocking file backend keyring");

                    // Retrieve secret from portal
                    let secret = Secret::from(
                        ashpd::desktop::secret::retrieve()
                            .await
                            .map_err(crate::file::Error::from)?,
                    );

                    let unlocked = locked.unlock(secret).await.map_err(crate::Error::File)?;
                    *kg = Some(file::Keyring::Unlocked(unlocked));
                } else {
                    *kg = kg_value;
                }
            }
        };
        Ok(())
    }

    /// Lock the used collection.
    pub async fn lock(&self) -> Result<()> {
        match self {
            Self::DBus(backend) => backend.lock(None).await?,
            Self::File(keyring) => {
                let mut kg = keyring.write().await;
                let kg_value = kg.take();
                if let Some(file::Keyring::Unlocked(unlocked)) = kg_value {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("Locking file backend keyring");

                    let locked = unlocked.lock();
                    *kg = Some(file::Keyring::Locked(locked));
                } else {
                    *kg = kg_value;
                }
            }
        };
        Ok(())
    }

    /// Whether the keyring is locked or not.
    pub async fn is_locked(&self) -> Result<bool> {
        match self {
            Self::DBus(collection) => collection.is_locked().await.map_err(From::from),
            Self::File(keyring) => {
                let keyring_guard = keyring.read().await;
                Ok(keyring_guard
                    .as_ref()
                    .expect("Keyring must exist")
                    .is_locked())
            }
        }
    }

    /// Remove items that matches the attributes.
    pub async fn delete(&self, attributes: &impl AsAttributes) -> Result<()> {
        match self {
            Self::DBus(backend) => {
                let items = backend.search_items(attributes).await?;
                for item in items {
                    item.delete(None).await?;
                }
            }
            Self::File(keyring) => {
                let kg = keyring.read().await;
                match kg.as_ref() {
                    Some(file::Keyring::Unlocked(backend)) => {
                        backend
                            .delete(attributes)
                            .await
                            .map_err(crate::Error::File)?;
                    }
                    Some(file::Keyring::Locked(_)) => {
                        return Err(crate::file::Error::Locked.into());
                    }
                    _ => unreachable!("A keyring must exist"),
                }
            }
        };
        Ok(())
    }

    /// Retrieve all the items.
    pub async fn items(&self) -> Result<Vec<Item>> {
        let items = match self {
            Self::DBus(backend) => {
                let items = backend.items().await?;
                items.into_iter().map(Item::for_dbus).collect::<Vec<_>>()
            }
            Self::File(keyring) => {
                let kg = keyring.read().await;
                match kg.as_ref() {
                    Some(file::Keyring::Unlocked(backend)) => {
                        let items = backend.items().await.map_err(crate::Error::File)?;
                        items
                            .into_iter()
                            // Ignore invalid items
                            .flatten()
                            .map(|i| Item::for_file(i, Arc::clone(keyring)))
                            .collect::<Vec<_>>()
                    }
                    Some(file::Keyring::Locked(_)) => {
                        return Err(crate::file::Error::Locked.into());
                    }
                    _ => unreachable!("A keyring must exist"),
                }
            }
        };
        Ok(items)
    }

    /// Create a new item.
    pub async fn create_item(
        &self,
        label: &str,
        attributes: &impl AsAttributes,
        secret: impl Into<Secret>,
        replace: bool,
    ) -> Result<()> {
        match self {
            Self::DBus(backend) => {
                backend
                    .create_item(label, attributes, secret, replace, None)
                    .await?;
            }
            Self::File(keyring) => {
                let kg = keyring.read().await;
                match kg.as_ref() {
                    Some(file::Keyring::Unlocked(backend)) => {
                        backend
                            .create_item(label, attributes, secret, replace)
                            .await
                            .map_err(crate::Error::File)?;
                    }
                    Some(file::Keyring::Locked(_)) => {
                        return Err(crate::file::Error::Locked.into());
                    }
                    _ => unreachable!("A keyring must exist"),
                }
            }
        };
        Ok(())
    }

    /// Find items based on their attributes.
    pub async fn search_items(&self, attributes: &impl AsAttributes) -> Result<Vec<Item>> {
        let items = match self {
            Self::DBus(backend) => {
                let items = backend.search_items(attributes).await?;
                items.into_iter().map(Item::for_dbus).collect::<Vec<_>>()
            }
            Self::File(keyring) => {
                let kg = keyring.read().await;
                match kg.as_ref() {
                    Some(file::Keyring::Unlocked(backend)) => {
                        let items = backend
                            .search_items(attributes)
                            .await
                            .map_err(crate::Error::File)?;
                        items
                            .into_iter()
                            .map(|i| Item::for_file(i, Arc::clone(keyring)))
                            .collect::<Vec<_>>()
                    }
                    Some(file::Keyring::Locked(_)) => {
                        return Err(crate::file::Error::Locked.into());
                    }
                    _ => unreachable!("A keyring must exist"),
                }
            }
        };
        Ok(items)
    }
}

/// A generic secret with a label and attributes.
#[derive(Debug)]
pub enum Item {
    #[doc(hidden)]
    File(
        RwLock<Option<file::Item>>,
        Arc<RwLock<Option<file::Keyring>>>,
    ),
    #[doc(hidden)]
    DBus(dbus::Item),
}

impl Item {
    fn for_file(item: file::Item, backend: Arc<RwLock<Option<file::Keyring>>>) -> Self {
        Self::File(RwLock::new(Some(item)), backend)
    }

    fn for_dbus(item: dbus::Item) -> Self {
        Self::DBus(item)
    }

    /// The item label.
    pub async fn label(&self) -> Result<String> {
        let label = match self {
            Self::File(item, _) => {
                let item_guard = item.read().await;
                let file_item = item_guard.as_ref().expect("Item must exist");
                match file_item {
                    file::Item::Unlocked(unlocked) => unlocked.label().to_owned(),
                    file::Item::Locked(_) => return Err(crate::file::Error::Locked.into()),
                }
            }
            Self::DBus(item) => item.label().await?,
        };
        Ok(label)
    }

    /// Sets the item label.
    pub async fn set_label(&self, label: &str) -> Result<()> {
        match self {
            Self::File(item, keyring) => {
                let mut item_guard = item.write().await;
                let file_item = item_guard.as_mut().expect("Item must exist");

                match file_item {
                    file::Item::Unlocked(unlocked) => {
                        unlocked.set_label(label);

                        let kg = keyring.read().await;
                        match kg.as_ref() {
                            Some(file::Keyring::Unlocked(backend)) => {
                                backend
                                    .create_item(
                                        unlocked.label(),
                                        &unlocked.attributes(),
                                        unlocked.secret(),
                                        true,
                                    )
                                    .await
                                    .map_err(crate::Error::File)?;
                            }
                            Some(file::Keyring::Locked(_)) => {
                                return Err(crate::file::Error::Locked.into());
                            }
                            None => unreachable!("A keyring must exist"),
                        }
                    }
                    file::Item::Locked(_) => {
                        return Err(crate::file::Error::Locked.into());
                    }
                }
            }
            Self::DBus(item) => item.set_label(label).await?,
        };
        Ok(())
    }

    /// Retrieve the item attributes.
    pub async fn attributes(&self) -> Result<HashMap<String, String>> {
        let attributes = match self {
            Self::File(item, _) => {
                let item_guard = item.read().await;
                let file_item = item_guard.as_ref().expect("Item must exist");
                match file_item {
                    file::Item::Unlocked(unlocked) => unlocked
                        .attributes()
                        .iter()
                        .map(|(k, v)| (k.to_owned(), v.to_string()))
                        .collect::<HashMap<_, _>>(),
                    file::Item::Locked(_) => return Err(crate::file::Error::Locked.into()),
                }
            }
            Self::DBus(item) => item.attributes().await?,
        };
        Ok(attributes)
    }

    /// Sets the item attributes.
    pub async fn set_attributes(&self, attributes: &impl AsAttributes) -> Result<()> {
        match self {
            Self::File(item, keyring) => {
                let kg = keyring.read().await;

                match kg.as_ref() {
                    Some(file::Keyring::Unlocked(backend)) => {
                        let mut item_guard = item.write().await;
                        let file_item = item_guard.as_mut().expect("Item must exist");

                        match file_item {
                            file::Item::Unlocked(unlocked) => {
                                let index = backend
                                    .lookup_item_index(&unlocked.attributes())
                                    .await
                                    .map_err(crate::Error::File)?;

                                unlocked.set_attributes(attributes);

                                if let Some(index) = index {
                                    backend
                                        .replace_item_index(index, unlocked)
                                        .await
                                        .map_err(crate::Error::File)?;
                                } else {
                                    backend
                                        .create_item(
                                            unlocked.label(),
                                            attributes,
                                            unlocked.secret(),
                                            true,
                                        )
                                        .await
                                        .map_err(crate::Error::File)?;
                                }
                            }
                            file::Item::Locked(_) => {
                                return Err(crate::file::Error::Locked.into());
                            }
                        }
                    }
                    Some(file::Keyring::Locked(_)) => {
                        return Err(crate::file::Error::Locked.into());
                    }
                    None => unreachable!("A keyring must exist"),
                }
            }
            Self::DBus(item) => item.set_attributes(attributes).await?,
        };
        Ok(())
    }

    /// Sets a new secret.
    pub async fn set_secret(&self, secret: impl Into<Secret>) -> Result<()> {
        match self {
            Self::File(item, keyring) => {
                let mut item_guard = item.write().await;
                let file_item = item_guard.as_mut().expect("Item must exist");

                match file_item {
                    file::Item::Unlocked(unlocked) => {
                        unlocked.set_secret(secret);

                        let kg = keyring.read().await;
                        match kg.as_ref() {
                            Some(file::Keyring::Unlocked(backend)) => {
                                backend
                                    .create_item(
                                        unlocked.label(),
                                        &unlocked.attributes(),
                                        unlocked.secret(),
                                        true,
                                    )
                                    .await
                                    .map_err(crate::Error::File)?;
                            }
                            Some(file::Keyring::Locked(_)) => {
                                return Err(crate::file::Error::Locked.into());
                            }
                            None => unreachable!("A keyring must exist"),
                        }
                    }
                    file::Item::Locked(_) => {
                        return Err(crate::file::Error::Locked.into());
                    }
                }
            }
            Self::DBus(item) => item.set_secret(secret).await?,
        };
        Ok(())
    }

    /// Retrieves the stored secret.
    pub async fn secret(&self) -> Result<Secret> {
        let secret = match self {
            Self::File(item, _) => {
                let item_guard = item.read().await;
                let file_item = item_guard.as_ref().expect("Item must exist");
                match file_item {
                    file::Item::Unlocked(unlocked) => unlocked.secret(),
                    file::Item::Locked(_) => return Err(crate::file::Error::Locked.into()),
                }
            }
            Self::DBus(item) => item.secret().await?,
        };
        Ok(secret)
    }

    /// Whether the item is locked or not
    pub async fn is_locked(&self) -> Result<bool> {
        match self {
            Self::DBus(item) => item.is_locked().await.map_err(From::from),
            Self::File(item, _) => {
                let item_guard = item.read().await;
                let file_item = item_guard.as_ref().expect("Item must exist");
                Ok(file_item.is_locked())
            }
        }
    }

    /// Lock the item
    pub async fn lock(&self) -> Result<()> {
        match self {
            Self::DBus(item) => item.lock(None).await?,
            Self::File(item, keyring) => {
                let mut item_guard = item.write().await;
                let item_value = item_guard.take();
                if let Some(file::Item::Unlocked(unlocked)) = item_value {
                    let kg = keyring.read().await;
                    match kg.as_ref() {
                        Some(file::Keyring::Unlocked(backend)) => {
                            let locked = backend
                                .lock_item(unlocked)
                                .await
                                .map_err(crate::Error::File)?;
                            *item_guard = Some(file::Item::Locked(locked));
                        }
                        Some(file::Keyring::Locked(_)) => {
                            *item_guard = Some(file::Item::Unlocked(unlocked));
                            return Err(crate::file::Error::Locked.into());
                        }
                        None => unreachable!("A keyring must exist"),
                    }
                } else {
                    *item_guard = item_value;
                }
            }
        }
        Ok(())
    }

    /// Unlock the item
    pub async fn unlock(&self) -> Result<()> {
        match self {
            Self::DBus(item) => item.unlock(None).await?,
            Self::File(item, keyring) => {
                let mut item_guard = item.write().await;
                let item_value = item_guard.take();
                if let Some(file::Item::Locked(locked)) = item_value {
                    let kg = keyring.read().await;
                    match kg.as_ref() {
                        Some(file::Keyring::Unlocked(backend)) => {
                            let unlocked = backend
                                .unlock_item(locked)
                                .await
                                .map_err(crate::Error::File)?;
                            *item_guard = Some(file::Item::Unlocked(unlocked));
                        }
                        Some(file::Keyring::Locked(_)) => {
                            *item_guard = Some(file::Item::Locked(locked));
                            return Err(crate::file::Error::Locked.into());
                        }
                        None => unreachable!("A keyring must exist"),
                    }
                } else {
                    *item_guard = item_value;
                }
            }
        }
        Ok(())
    }

    /// Delete the item.
    pub async fn delete(&self) -> Result<()> {
        match self {
            Self::File(item, keyring) => {
                let item_guard = item.read().await;
                let file_item = item_guard.as_ref().expect("Item must exist");

                match file_item {
                    file::Item::Unlocked(unlocked) => {
                        let kg = keyring.read().await;
                        match kg.as_ref() {
                            Some(file::Keyring::Unlocked(backend)) => {
                                backend
                                    .delete(&unlocked.attributes())
                                    .await
                                    .map_err(crate::Error::File)?;
                            }
                            Some(file::Keyring::Locked(_)) => {
                                return Err(crate::file::Error::Locked.into());
                            }
                            None => unreachable!("A keyring must exist"),
                        }
                    }
                    file::Item::Locked(_) => {
                        return Err(crate::file::Error::Locked.into());
                    }
                }
            }
            Self::DBus(item) => {
                item.delete(None).await?;
            }
        };
        Ok(())
    }

    /// The UNIX time when the item was created.
    pub async fn created(&self) -> Result<Duration> {
        match self {
            Self::DBus(item) => Ok(item.created().await?),
            Self::File(item, _) => {
                let item_guard = item.read().await;
                let file_item = item_guard.as_ref().expect("Item must exist");
                match file_item {
                    file::Item::Unlocked(unlocked) => Ok(unlocked.created()),
                    file::Item::Locked(_) => Err(crate::file::Error::Locked.into()),
                }
            }
        }
    }

    /// The UNIX time when the item was modified.
    pub async fn modified(&self) -> Result<Duration> {
        match self {
            Self::DBus(item) => Ok(item.modified().await?),
            Self::File(item, _) => {
                let item_guard = item.read().await;
                let file_item = item_guard.as_ref().expect("Item must exist");
                match file_item {
                    file::Item::Unlocked(unlocked) => Ok(unlocked.modified()),
                    file::Item::Locked(_) => Err(crate::file::Error::Locked.into()),
                }
            }
        }
    }
}
