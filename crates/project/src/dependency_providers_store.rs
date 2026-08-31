//! Global registry of [`DependencyLister`] providers, mirroring
//! [`ManifestProvidersStore`](crate::ManifestProvidersStore).
//!
//! Providers are registered once at startup (see `languages::init`) and queried
//! by the [`ExternalLibrariesStore`](crate::ExternalLibrariesStore) for each
//! worktree root to discover a project's external dependencies.

use std::{collections::HashMap, ops::Deref, sync::Arc};

use gpui::{App, Global};
use language::{DependencyLister, LanguageName};
use parking_lot::RwLock;

#[derive(Default)]
struct DependencyProvidersState {
    providers: HashMap<LanguageName, Arc<dyn DependencyLister>>,
}

#[derive(Clone, Default)]
pub struct DependencyProvidersStore(Arc<RwLock<DependencyProvidersState>>);

#[derive(Default)]
struct GlobalDependencyProvider(DependencyProvidersStore);

impl Deref for GlobalDependencyProvider {
    type Target = DependencyProvidersStore;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Global for GlobalDependencyProvider {}

impl DependencyProvidersStore {
    /// Returns the global [`DependencyProvidersStore`], inserting a default one
    /// if it does not yet exist.
    pub fn global(cx: &mut App) -> Self {
        cx.default_global::<GlobalDependencyProvider>().0.clone()
    }

    pub fn register(&self, provider: Arc<dyn DependencyLister>) {
        self.0
            .write()
            .providers
            .insert(provider.language_name(), provider);
    }

    pub fn providers(&self) -> Vec<Arc<dyn DependencyLister>> {
        self.0.read().providers.values().cloned().collect()
    }
}
