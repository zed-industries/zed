use crate::{SharedString, SharedUri, WindowContext};
use collections::FxHashMap;
use futures::Future;
use parking_lot::Mutex;
use std::any::TypeId;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use std::{any::Any, path::PathBuf};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub(crate) enum UriOrPath {
    Uri(SharedUri),
    Path(Arc<PathBuf>),
    Asset(SharedString),
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
        cx: &mut WindowContext,
    ) -> impl Future<Output = Self::Output> + Send + 'static;
}

/// Use a quick, non-cryptographically secure hash function to get an identifier from data
pub fn hash<T: Hash>(data: &T) -> u64 {
    let mut hasher = collections::FxHasher::default();
    data.hash(&mut hasher);
    hasher.finish()
}

/// A cache for assets.
#[derive(Clone)]
pub struct AssetCache {
    assets: Arc<Mutex<FxHashMap<(TypeId, u64), Box<dyn Any + Send>>>>,
}

impl AssetCache {
    pub(crate) fn new() -> Self {
        Self {
            assets: Default::default(),
        }
    }

    /// Get the asset from the cache, if it exists.
    pub fn get<A: Asset + 'static>(&self, source: &A::Source) -> Option<A::Output> {
        self.assets
            .lock()
            .get(&(TypeId::of::<A>(), hash(&source)))
            .and_then(|task| task.downcast_ref::<A::Output>())
            .cloned()
    }

    /// Insert the asset into the cache.
    pub fn insert<A: Asset + 'static>(&mut self, source: A::Source, output: A::Output) {
        self.assets
            .lock()
            .insert((TypeId::of::<A>(), hash(&source)), Box::new(output));
    }

    /// Remove an entry from the asset cache
    pub fn remove<A: Asset + 'static>(&mut self, source: &A::Source) -> Option<A::Output> {
        self.assets
            .lock()
            .remove(&(TypeId::of::<A>(), hash(&source)))
            .and_then(|any| any.downcast::<A::Output>().ok())
            .map(|boxed| *boxed)
    }
}
