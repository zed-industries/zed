use crate::{
    Key,
    file::{self, UnlockedItem, api},
};

/// A locked variant of [`UnlockedItem`]
#[derive(Clone, Debug)]
pub struct LockedItem {
    pub(crate) inner: api::EncryptedItem,
}

impl LockedItem {
    /// Unlocks the item.
    pub fn unlock(self, key: &Key) -> Result<UnlockedItem, file::Error> {
        self.inner.decrypt(key)
    }
}
