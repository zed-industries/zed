use std::sync::Arc;

use gpui::{AppContext, Global, ReadGlobal};
use parking_lot::RwLock;

use crate::Extension;

pub trait OnLanguageServerExtensionChange {}

pub trait OnIndexedDocsProviderExtensionChange {
    fn register(&self, extension: Arc<dyn Extension>, provider_id: Arc<str>);
}

#[derive(Clone)]
pub enum ExtensionChangeListener {
    LanguageServer(Arc<dyn OnLanguageServerExtensionChange>),
    IndexedDocsProvider(Arc<dyn OnIndexedDocsProviderExtensionChange>),
}

#[derive(Default)]
struct GlobalExtensionChangeListeners(Arc<ExtensionChangeListeners>);

impl Global for GlobalExtensionChangeListeners {}

#[derive(Default)]
pub struct ExtensionChangeListeners {
    listeners: RwLock<Vec<ExtensionChangeListener>>,
}

impl ExtensionChangeListeners {
    /// Returns the global [`ExtensionChangeListeners`].
    pub fn global(cx: &AppContext) -> Arc<Self> {
        GlobalExtensionChangeListeners::global(cx).0.clone()
    }

    /// Returns the global [`ExtensionChangeListeners`].
    ///
    /// Inserts a default [`ExtensionChangeListeners`] if one does not yet exist.
    pub fn default_global(cx: &mut AppContext) -> Arc<Self> {
        cx.default_global::<GlobalExtensionChangeListeners>()
            .0
            .clone()
    }

    pub fn new() -> Self {
        Self {
            listeners: RwLock::default(),
        }
    }

    pub fn listeners(&self) -> Vec<ExtensionChangeListener> {
        self.listeners.read().iter().cloned().collect()
    }

    pub fn indexed_docs_provider_listeners(
        &self,
    ) -> Vec<Arc<dyn OnIndexedDocsProviderExtensionChange>> {
        self.filter_listeners(|listener| {
            if let ExtensionChangeListener::IndexedDocsProvider(provider_listener) = listener {
                Some(provider_listener.clone())
            } else {
                None
            }
        })
    }

    fn filter_listeners<F, R>(&self, mut f: F) -> Vec<R>
    where
        F: FnMut(&ExtensionChangeListener) -> Option<R>,
    {
        self.listeners
            .read()
            .iter()
            .filter_map(|listener| f(listener))
            .collect()
    }

    pub fn register_indexed_docs_provider_listener(
        &self,
        listener: impl OnIndexedDocsProviderExtensionChange + 'static,
    ) {
        self.register(ExtensionChangeListener::IndexedDocsProvider(Arc::new(
            listener,
        )));
    }

    pub fn register(&self, listener: ExtensionChangeListener) {
        self.listeners.write().push(listener.into());
    }
}
