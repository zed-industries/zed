//! File backend implementation that can be backed by the [Secret portal](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Secret.html).
//!
//! ```no_run
//! use oo7::{Secret, file::UnlockedKeyring};
//!
//! # async fn run() -> oo7::Result<()> {
//! let keyring = UnlockedKeyring::load("default.keyring", Secret::text("some_text")).await?;
//! keyring
//!     .create_item("My Label", &[("account", "alice")], "My Password", true)
//!     .await?;
//!
//! let items = keyring.search_items(&[("account", "alice")]).await?;
//! assert_eq!(
//!     items[0].as_unlocked().secret(),
//!     oo7::Secret::blob("My Password")
//! );
//!
//! keyring.delete(&[("account", "alice")]).await?;
//! #   Ok(())
//! # }
//! ```

#[cfg(feature = "unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
pub mod api;
#[cfg(not(feature = "unstable"))]
pub(crate) mod api;

mod error;
mod locked_item;
mod locked_keyring;
mod unlocked_item;
mod unlocked_keyring;

pub use error::{Error, InvalidItemError, WeakKeyError};
pub use locked_item::LockedItem;
pub use locked_keyring::LockedKeyring;
pub use unlocked_item::UnlockedItem;
pub use unlocked_keyring::{ItemDefinition, UnlockedKeyring};

use crate::{AsAttributes, Key, Secret};

#[derive(Debug)]
pub enum Item {
    Locked(LockedItem),
    Unlocked(UnlockedItem),
}

impl Item {
    pub const fn is_locked(&self) -> bool {
        matches!(self, Self::Locked(_))
    }

    pub fn as_unlocked(&self) -> &UnlockedItem {
        match self {
            Self::Unlocked(item) => item,
            _ => panic!("The item is locked"),
        }
    }

    pub fn as_mut_unlocked(&mut self) -> &mut UnlockedItem {
        match self {
            Self::Unlocked(item) => item,
            _ => panic!("The item is locked"),
        }
    }

    pub fn as_locked(&self) -> &LockedItem {
        match self {
            Self::Locked(item) => item,
            _ => panic!("The item is unlocked"),
        }
    }

    /// Check if this item matches the given attributes
    pub fn matches_attributes(&self, attributes: &impl AsAttributes, key: &Key) -> bool {
        match self {
            Self::Unlocked(unlocked) => {
                let item_attrs = unlocked.attributes();
                attributes.as_attributes().iter().all(|(k, value)| {
                    item_attrs.get(k.as_str()).map(|v| v.as_ref()) == Some(value.as_str())
                })
            }
            Self::Locked(locked) => {
                let hashed_attrs = attributes.hash(key);

                hashed_attrs.iter().all(|(attr_key, mac_result)| {
                    mac_result
                        .as_ref()
                        .ok()
                        .map(|mac| locked.inner.has_attribute(attr_key.as_str(), mac))
                        .unwrap_or(false)
                })
            }
        }
    }
}

#[derive(Debug)]
pub enum Keyring {
    Locked(LockedKeyring),
    Unlocked(UnlockedKeyring),
}

impl Keyring {
    /// Validate that a secret can decrypt the items in this keyring.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self, secret)))]
    pub async fn validate_secret(&self, secret: &Secret) -> Result<bool, Error> {
        match self {
            Self::Locked(keyring) => keyring.validate_secret(secret).await,
            Self::Unlocked(keyring) => keyring.validate_secret(secret).await,
        }
    }

    /// Get the modification timestamp
    pub async fn modified_time(&self) -> std::time::Duration {
        match self {
            Self::Locked(keyring) => keyring.modified_time().await,
            Self::Unlocked(keyring) => keyring.modified_time().await,
        }
    }

    /// Get the creation timestamp from the filesystem if the keyring has an
    /// associated file.
    pub async fn created_time(&self) -> Option<std::time::Duration> {
        let path = self.path()?;

        #[cfg(feature = "tokio")]
        let metadata = tokio::fs::metadata(path).await.ok()?;
        #[cfg(feature = "async-std")]
        let metadata = async_fs::metadata(path).await.ok()?;

        metadata
            .created()
            .ok()
            .and_then(|time| time.duration_since(std::time::SystemTime::UNIX_EPOCH).ok())
    }

    pub async fn items(&self) -> Result<Vec<Result<Item, InvalidItemError>>, Error> {
        match self {
            Self::Locked(keyring) => keyring.items().await,
            Self::Unlocked(keyring) => keyring.items().await,
        }
    }

    /// Return the associated file if any.
    pub fn path(&self) -> Option<&std::path::Path> {
        match self {
            Self::Locked(keyring) => keyring.path(),
            Self::Unlocked(keyring) => keyring.path(),
        }
    }

    pub const fn is_locked(&self) -> bool {
        matches!(self, Self::Locked(_))
    }

    pub fn as_unlocked(&self) -> &UnlockedKeyring {
        match self {
            Self::Unlocked(unlocked_keyring) => unlocked_keyring,
            _ => panic!("The keyring is locked"),
        }
    }

    pub fn as_locked(&self) -> &LockedKeyring {
        match self {
            Self::Locked(locked_keyring) => locked_keyring,
            _ => panic!("The keyring is unlocked"),
        }
    }
}
