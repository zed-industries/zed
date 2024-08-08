use crate::{AppContext, SharedString, SharedUri};
use futures::Future;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub(crate) enum UriOrPath {
    Uri(SharedUri),
    Path(Arc<PathBuf>),
    Embedded(SharedString),
}

impl From<SharedUri> for UriOrPath {
    fn from(value: SharedUri) -> Self {
        Self::Uri(value)
    }
}

impl From<Arc<PathBuf>> for UriOrPath {
    fn from(value: Arc<PathBuf>) -> Self {
        Self::Path(value)
    }
}

/// A trait for asynchronous asset loading.
pub trait Asset {
    /// The source of the asset.
    type Source: Clone + Hash + Send;

    /// The loaded asset
    type Output: Clone + Send;

    /// Load the asset asynchronously
    fn load(
        source: Self::Source,
        cx: &mut AppContext,
    ) -> impl Future<Output = Self::Output> + Send + 'static;
}

/// Use a quick, non-cryptographically secure hash function to get an identifier from data
pub fn hash<T: Hash>(data: &T) -> u64 {
    let mut hasher = collections::FxHasher::default();
    data.hash(&mut hasher);
    hasher.finish()
}
