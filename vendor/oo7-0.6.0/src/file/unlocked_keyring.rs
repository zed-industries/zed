use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};

#[cfg(feature = "async-std")]
use async_fs as fs;
#[cfg(feature = "async-std")]
use async_lock::{Mutex, RwLock};
#[cfg(feature = "async-std")]
use futures_lite::AsyncReadExt;
#[cfg(feature = "tokio")]
use tokio::{
    fs,
    io::AsyncReadExt,
    sync::{Mutex, RwLock},
};

use crate::{
    AsAttributes, Key, Secret,
    file::{Error, InvalidItemError, Item, LockedItem, LockedKeyring, UnlockedItem, api},
};

/// Definition for batch item creation: (label, attributes, secret, replace)
pub type ItemDefinition = (String, HashMap<String, String>, Secret, bool);

/// File backed keyring.
#[derive(Debug)]
pub struct UnlockedKeyring {
    pub(super) keyring: Arc<RwLock<api::Keyring>>,
    pub(super) path: Option<PathBuf>,
    /// Times are stored before reading the file to detect
    /// file changes before writing
    pub(super) mtime: Mutex<Option<std::time::SystemTime>>,
    pub(super) key: Mutex<Option<Arc<Key>>>,
    pub(super) secret: Mutex<Arc<Secret>>,
}

impl UnlockedKeyring {
    /// Load from a keyring file.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the file backend.
    /// * `secret` - The service key, usually retrieved from the Secrets portal.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(secret), fields(path = ?path.as_ref())))]
    pub async fn load(path: impl AsRef<Path>, secret: Secret) -> Result<Self, Error> {
        Self::load_inner(path, secret, true).await
    }

    /// Load from a keyring file.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the file backend.
    /// * `secret` - The service key, usually retrieved from the Secrets portal.
    ///
    /// # Safety
    ///
    /// The secret is not validated to be the correct one to decrypt the keyring
    /// items. Allowing the API user to write new items with a different
    /// secret on top of previously added items with a different secret.
    ///
    /// As it is not a supported behaviour, this API is mostly meant for
    /// recovering broken keyrings.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(secret), fields(path = ?path.as_ref())))]
    pub async unsafe fn load_unchecked(
        path: impl AsRef<Path>,
        secret: Secret,
    ) -> Result<Self, Error> {
        Self::load_inner(path, secret, false).await
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip(secret), fields(path = ?path.as_ref(), validate_items = validate_items)))]
    async fn load_inner(
        path: impl AsRef<Path>,
        secret: Secret,
        validate_items: bool,
    ) -> Result<Self, Error> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Trying to load keyring file at {:?}", path.as_ref());
        if validate_items {
            LockedKeyring::load(path).await?.unlock(secret).await
        } else {
            unsafe {
                LockedKeyring::load(path)
                    .await?
                    .unlock_unchecked(secret)
                    .await
            }
        }
    }

    /// Creates a temporary backend, that is never stored on disk.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(secret)))]
    pub async fn temporary(secret: Secret) -> Result<Self, Error> {
        let keyring = api::Keyring::new()?;
        Ok(Self {
            keyring: Arc::new(RwLock::new(keyring)),
            path: None,
            mtime: Default::default(),
            key: Default::default(),
            secret: Mutex::new(Arc::new(secret)),
        })
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip(file, secret), fields(path = ?path.as_ref())))]
    async fn migrate(
        file: &mut fs::File,
        path: impl AsRef<Path>,
        secret: Secret,
    ) -> Result<Self, Error> {
        let metadata = file.metadata().await?;
        let mut content = Vec::with_capacity(metadata.len() as usize);
        file.read_to_end(&mut content).await?;

        match api::Keyring::try_from(content.as_slice()) {
            Ok(keyring) => Ok(Self {
                keyring: Arc::new(RwLock::new(keyring)),
                path: Some(path.as_ref().to_path_buf()),
                mtime: Default::default(),
                key: Default::default(),
                secret: Mutex::new(Arc::new(secret)),
            }),
            Err(Error::VersionMismatch(Some(version)))
                if version[0] == api::LEGACY_MAJOR_VERSION =>
            {
                #[cfg(feature = "tracing")]
                tracing::debug!("Migrating from legacy keyring format");

                let legacy_keyring = api::LegacyKeyring::try_from(content.as_slice())?;
                let mut keyring = api::Keyring::new()?;
                let key = keyring.derive_key(&secret)?;

                let decrypted_items = legacy_keyring.decrypt_items(&secret)?;

                #[cfg(feature = "tracing")]
                let _migrate_span =
                    tracing::debug_span!("migrate_items", item_count = decrypted_items.len());

                for item in decrypted_items {
                    let encrypted_item = item.encrypt(&key)?;
                    keyring.items.push(encrypted_item);
                }

                Ok(Self {
                    keyring: Arc::new(RwLock::new(keyring)),
                    path: Some(path.as_ref().to_path_buf()),
                    mtime: Default::default(),
                    key: Default::default(),
                    secret: Mutex::new(Arc::new(secret)),
                })
            }
            Err(err) => Err(err),
        }
    }

    /// Open a keyring with given name from the default directory.
    ///
    /// This function will automatically migrate the keyring to the
    /// latest format.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the keyring.
    /// * `secret` - The service key, usually retrieved from the Secrets portal.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(secret)))]
    pub async fn open(name: &str, secret: Secret) -> Result<Self, Error> {
        let v1_path = api::Keyring::path(name, api::MAJOR_VERSION)?;
        if v1_path.exists() {
            #[cfg(feature = "tracing")]
            tracing::debug!("Loading v1 keyring file");
            return Self::load(v1_path, secret).await;
        }

        let v0_path = api::Keyring::path(name, api::LEGACY_MAJOR_VERSION)?;
        if v0_path.exists() {
            #[cfg(feature = "tracing")]
            tracing::debug!("Trying to load keyring file at {:?}", v0_path);
            match fs::File::open(&v0_path).await {
                Err(err) => Err(err.into()),
                Ok(mut file) => Self::migrate(&mut file, v1_path, secret).await,
            }
        } else {
            #[cfg(feature = "tracing")]
            tracing::debug!("Creating new keyring");
            Ok(Self {
                keyring: Arc::new(RwLock::new(api::Keyring::new()?)),
                path: Some(v1_path),
                mtime: Default::default(),
                key: Default::default(),
                secret: Mutex::new(Arc::new(secret)),
            })
        }
    }

    /// Lock the keyring.
    pub fn lock(self) -> LockedKeyring {
        LockedKeyring {
            keyring: self.keyring,
            path: self.path,
            mtime: self.mtime,
        }
    }

    /// Lock an item using the keyring's key.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self, item)))]
    pub async fn lock_item(&self, item: UnlockedItem) -> Result<LockedItem, Error> {
        let key = self.derive_key().await?;
        item.lock(&key)
    }

    /// Unlock an item using the keyring's key.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self, item)))]
    pub async fn unlock_item(&self, item: LockedItem) -> Result<UnlockedItem, Error> {
        let key = self.derive_key().await?;
        item.unlock(&key)
    }

    /// Get the encryption key for this keyring.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn key(&self) -> Result<Arc<Key>, crate::crypto::Error> {
        self.derive_key().await
    }

    /// Return the associated file if any.
    pub fn path(&self) -> Option<&std::path::Path> {
        self.path.as_deref()
    }

    /// Get the modification timestamp
    pub async fn modified_time(&self) -> std::time::Duration {
        self.keyring.read().await.modified_time()
    }

    /// Retrieve the number of items
    ///
    /// This function will not trigger a key derivation and can therefore be
    /// faster than [`items().len()`](Self::items).
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn n_items(&self) -> usize {
        self.keyring.read().await.items.len()
    }

    /// Retrieve the list of available [`UnlockedItem`]s.
    ///
    /// If items cannot be decrypted, [`InvalidItemError`]s are returned for
    /// them instead of [`UnlockedItem`]s.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn items(&self) -> Result<Vec<Result<Item, InvalidItemError>>, Error> {
        let key = self.derive_key().await?;
        let keyring = self.keyring.read().await;

        #[cfg(feature = "tracing")]
        let _span = tracing::debug_span!("decrypt", total_items = keyring.items.len());

        Ok(keyring
            .items
            .iter()
            .map(|e| {
                (*e).clone()
                    .decrypt(&key)
                    .map_err(|err| {
                        InvalidItemError::new(
                            err,
                            e.hashed_attributes.keys().map(|x| x.to_string()).collect(),
                        )
                    })
                    .map(Item::Unlocked)
            })
            .collect())
    }

    /// Search items matching the attributes.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self, attributes)))]
    pub async fn search_items(&self, attributes: &impl AsAttributes) -> Result<Vec<Item>, Error> {
        let key = self.derive_key().await?;
        let keyring = self.keyring.read().await;
        let results = keyring
            .search_items(attributes, &key)?
            .into_iter()
            .map(Item::Unlocked)
            .collect::<Vec<Item>>();

        #[cfg(feature = "tracing")]
        tracing::debug!("Found {} matching items", results.len());

        Ok(results)
    }

    /// Find the first item matching the attributes.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self, attributes)))]
    pub async fn lookup_item(&self, attributes: &impl AsAttributes) -> Result<Option<Item>, Error> {
        let key = self.derive_key().await?;
        let keyring = self.keyring.read().await;

        keyring
            .lookup_item(attributes, &key)
            .map(|maybe_item| maybe_item.map(Item::Unlocked))
    }

    /// Find the index in the list of items of the first item matching the
    /// attributes.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self, attributes)))]
    pub async fn lookup_item_index(
        &self,
        attributes: &impl AsAttributes,
    ) -> Result<Option<usize>, Error> {
        let key = self.derive_key().await?;
        let keyring = self.keyring.read().await;

        Ok(keyring.lookup_item_index(attributes, &key))
    }

    /// Delete an item.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self, attributes)))]
    pub async fn delete(&self, attributes: &impl AsAttributes) -> Result<(), Error> {
        #[cfg(feature = "tracing")]
        let items_before = { self.keyring.read().await.items.len() };

        {
            let key = self.derive_key().await?;
            let mut keyring = self.keyring.write().await;
            keyring.remove_items(attributes, &key)?;
        };

        self.write().await?;

        #[cfg(feature = "tracing")]
        {
            let items_after = self.keyring.read().await.items.len();
            let deleted_count = items_before.saturating_sub(items_after);
            tracing::info!("Deleted {} items", deleted_count);
        }

        Ok(())
    }

    /// Create a new item
    ///
    /// # Arguments
    ///
    /// * `label` - A user visible label of the item.
    /// * `attributes` - A map of key/value attributes, used to find the item
    ///   later.
    /// * `secret` - The secret to store.
    /// * `replace` - Whether to replace the value if the `attributes` matches
    ///   an existing `secret`.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self, secret, attributes), fields(replace = replace)))]
    pub async fn create_item(
        &self,
        label: &str,
        attributes: &impl AsAttributes,
        secret: impl Into<Secret>,
        replace: bool,
    ) -> Result<Item, Error> {
        let item = {
            let key = self.derive_key().await?;
            let mut keyring = self.keyring.write().await;
            if replace {
                keyring.remove_items(attributes, &key)?;
            }
            let item = UnlockedItem::new(label, attributes, secret);
            let encrypted_item = item.encrypt(&key)?;
            keyring.items.push(encrypted_item);
            item
        };
        match self.write().await {
            Err(e) => {
                #[cfg(feature = "tracing")]
                tracing::error!("Failed to write keyring after item creation");
                Err(e)
            }
            Ok(_) => {
                #[cfg(feature = "tracing")]
                tracing::info!("Successfully created item");
                Ok(Item::Unlocked(item))
            }
        }
    }

    /// Replaces item at the given index.
    ///
    /// The `index` refers to the index of the [`Vec`] returned by
    /// [`items()`](Self::items). If the index does not exist, the functions
    /// returns an error.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self, item), fields(index = index)))]
    pub async fn replace_item_index(&self, index: usize, item: &UnlockedItem) -> Result<(), Error> {
        {
            let key = self.derive_key().await?;
            let mut keyring = self.keyring.write().await;

            if let Some(item_store) = keyring.items.get_mut(index) {
                *item_store = item.encrypt(&key)?;
            } else {
                return Err(Error::InvalidItemIndex(index));
            }
        }
        self.write().await
    }

    /// Deletes item at the given index.
    ///
    /// The `index` refers to the index of the [`Vec`] returned by
    /// [`items()`](Self::items). If the index does not exist, the functions
    /// returns an error.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self), fields(index = index)))]
    pub async fn delete_item_index(&self, index: usize) -> Result<(), Error> {
        {
            let mut keyring = self.keyring.write().await;

            if index < keyring.items.len() {
                keyring.items.remove(index);
            } else {
                return Err(Error::InvalidItemIndex(index));
            }
        }
        self.write().await
    }

    /// Create multiple items in a single operation to avoid re-writing the file
    /// multiple times.
    ///
    /// This is more efficient than calling `create_item()` multiple times.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self, items), fields(item_count = items.len())))]
    pub async fn create_items(&self, items: Vec<ItemDefinition>) -> Result<(), Error> {
        let key = self.derive_key().await?;
        let mut mtime = self.mtime.lock().await;
        let mut keyring = self.keyring.write().await;

        #[cfg(feature = "tracing")]
        let _span = tracing::debug_span!("bulk_create", items_to_create = items.len());

        for (label, attributes, secret, replace) in items {
            if replace {
                keyring.remove_items(&attributes, &key)?;
            }
            let item = UnlockedItem::new(label, &attributes, secret);
            let encrypted_item = item.encrypt(&key)?;
            keyring.items.push(encrypted_item);
        }

        #[cfg(feature = "tracing")]
        tracing::debug!("Writing keyring back to the file");
        if let Some(ref path) = self.path {
            keyring.dump(path, *mtime).await?;
            // Update mtime after successful write
            if let Ok(modified) = fs::metadata(path).await?.modified() {
                *mtime = Some(modified);
            }
        }
        Ok(())
    }

    /// Write the changes to the keyring file.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn write(&self) -> Result<(), Error> {
        let mut mtime = self.mtime.lock().await;
        {
            let mut keyring = self.keyring.write().await;

            if let Some(ref path) = self.path {
                keyring.dump(path, *mtime).await?;
            }
        };
        let Some(ref path) = self.path else {
            return Ok(());
        };

        if let Ok(modified) = fs::metadata(path).await?.modified() {
            *mtime = Some(modified);
        }
        Ok(())
    }

    /// Return key, derive and store it first if not initialized
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    async fn derive_key(&self) -> Result<Arc<Key>, crate::crypto::Error> {
        let keyring = Arc::clone(&self.keyring);
        let secret_lock = self.secret.lock().await;
        let secret = Arc::clone(&secret_lock);
        drop(secret_lock);

        let mut key_lock = self.key.lock().await;
        if key_lock.is_none() {
            #[cfg(feature = "async-std")]
            let key = blocking::unblock(move || {
                async_io::block_on(async { keyring.read().await.derive_key(&secret) })
            })
            .await?;
            #[cfg(feature = "tokio")]
            let key = {
                tokio::task::spawn_blocking(move || keyring.blocking_read().derive_key(&secret))
                    .await
                    .unwrap()?
            };

            *key_lock = Some(Arc::new(key));
        }

        Ok(Arc::clone(key_lock.as_ref().unwrap()))
    }

    /// Change keyring secret
    ///
    /// # Arguments
    ///
    /// * `secret` - The new secret to store.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self, secret)))]
    pub async fn change_secret(&self, secret: Secret) -> Result<(), Error> {
        let keyring = self.keyring.read().await;
        let key = self.derive_key().await?;
        let mut items = Vec::with_capacity(keyring.items.len());

        #[cfg(feature = "tracing")]
        let _decrypt_span =
            tracing::debug_span!("decrypt_for_reencrypt", total_items = keyring.items.len());

        for item in &keyring.items {
            items.push(item.clone().decrypt(&key)?);
        }
        drop(keyring);

        #[cfg(feature = "tracing")]
        tracing::debug!("Updating secret and resetting key");

        let mut secret_lock = self.secret.lock().await;
        *secret_lock = Arc::new(secret);
        drop(secret_lock);

        let mut key_lock = self.key.lock().await;
        // Unset the old key
        *key_lock = None;
        drop(key_lock);

        // Reset Keyring content before setting the new key
        let mut keyring = self.keyring.write().await;
        keyring.reset()?;
        drop(keyring);

        // Set new key
        let key = self.derive_key().await?;

        #[cfg(feature = "tracing")]
        let _reencrypt_span = tracing::debug_span!("reencrypt", total_items = items.len());

        let mut keyring = self.keyring.write().await;
        for item in items {
            let encrypted_item = item.encrypt(&key)?;
            keyring.items.push(encrypted_item);
        }
        drop(keyring);

        self.write().await
    }

    /// Validate that a secret can decrypt the items in this keyring.
    ///
    /// For empty keyrings, this always returns `true` since there are no items
    /// to validate against.
    ///
    /// # Arguments
    ///
    /// * `secret` - The secret to validate.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self, secret)))]
    pub async fn validate_secret(&self, secret: &Secret) -> Result<bool, Error> {
        let keyring = self.keyring.read().await;
        Ok(keyring.validate_secret(secret)?)
    }

    /// Delete any item that cannot be decrypted with the key associated to the
    /// keyring.
    ///
    /// This can only happen if an item was created using
    /// [`Self::load_unchecked`] or prior to 0.4 where we didn't validate
    /// the secret when using [`Self::load`] or modified externally.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn delete_broken_items(&self) -> Result<usize, Error> {
        let key = self.derive_key().await?;
        let mut keyring = self.keyring.write().await;
        let mut broken_items = vec![];

        #[cfg(feature = "tracing")]
        let _span = tracing::debug_span!("identify_broken", total_items = keyring.items.len());

        for (index, encrypted_item) in keyring.items.iter().enumerate() {
            if !encrypted_item.is_valid(&key) {
                broken_items.push(index);
            }
        }
        let n_broken_items = broken_items.len();

        #[cfg(feature = "tracing")]
        tracing::info!("Found {} broken items to delete", n_broken_items);

        #[cfg(feature = "tracing")]
        let _remove_span = tracing::debug_span!("remove_broken", broken_count = n_broken_items);

        for index in broken_items.into_iter().rev() {
            keyring.items.remove(index);
        }
        drop(keyring);

        self.write().await?;
        Ok(n_broken_items)
    }
}
