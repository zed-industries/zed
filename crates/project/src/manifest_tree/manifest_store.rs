use collections::HashMap;
use gpui::{App, Global, SharedString};
use parking_lot::RwLock;
use std::{ops::Deref, sync::Arc};

use language::{ManifestName, ManifestProvider};

#[derive(Default)]
struct ManifestProvidersState {
    providers: HashMap<ManifestName, Arc<dyn ManifestProvider>>,
}

#[derive(Clone, Default)]
pub struct ManifestProviders(Arc<RwLock<ManifestProvidersState>>);

#[derive(Default)]
struct GlobalManifestProvider(ManifestProviders);

impl Deref for GlobalManifestProvider {
    type Target = ManifestProviders;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Global for GlobalManifestProvider {}

impl ManifestProviders {
    /// Returns the global [`ManifestStore`].
    ///
    /// Inserts a default [`ManifestStore`] if one does not yet exist.
    pub fn global(cx: &mut App) -> Self {
        cx.default_global::<GlobalManifestProvider>().0.clone()
    }

    pub fn register(&self, provider: Arc<dyn ManifestProvider>) {
        self.0.write().providers.insert(provider.name(), provider);
    }

    pub fn unregister(&self, name: &SharedString) {
        self.0.write().providers.remove(name);
    }

    pub(super) fn get(&self, name: &SharedString) -> Option<Arc<dyn ManifestProvider>> {
        self.0.read().providers.get(name).cloned()
    }
}
