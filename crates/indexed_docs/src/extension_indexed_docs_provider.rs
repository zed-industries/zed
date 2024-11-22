use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use extension::{Extension, ExtensionChangeListeners, OnIndexedDocsProviderExtensionChange};
use gpui::AppContext;

use crate::{
    IndexedDocsDatabase, IndexedDocsProvider, IndexedDocsRegistry, PackageName, ProviderId,
};

pub fn init(cx: &AppContext) {
    let extension_change_listeners = ExtensionChangeListeners::global(cx);
    extension_change_listeners.register_indexed_docs_provider_listener(
        ExtensionIndexedDocsProviderListener {
            indexed_docs_registry: IndexedDocsRegistry::global(cx),
        },
    );
}

struct ExtensionIndexedDocsProviderListener {
    indexed_docs_registry: Arc<IndexedDocsRegistry>,
}

impl OnIndexedDocsProviderExtensionChange for ExtensionIndexedDocsProviderListener {
    fn register(&self, extension: Arc<dyn Extension>, provider_id: Arc<str>) {
        self.indexed_docs_registry
            .register_provider(Box::new(ExtensionIndexedDocsProvider::new(
                extension,
                ProviderId(provider_id),
            )));
    }
}

pub struct ExtensionIndexedDocsProvider {
    extension: Arc<dyn Extension>,
    id: ProviderId,
}

impl ExtensionIndexedDocsProvider {
    pub fn new(extension: Arc<dyn Extension>, id: ProviderId) -> Self {
        Self { extension, id }
    }
}

#[async_trait]
impl IndexedDocsProvider for ExtensionIndexedDocsProvider {
    fn id(&self) -> ProviderId {
        self.id.clone()
    }

    fn database_path(&self) -> PathBuf {
        let mut database_path = PathBuf::from(self.extension.work_dir().as_ref());
        database_path.push("docs");
        database_path.push(format!("{}.0.mdb", self.id));

        database_path
    }

    async fn suggest_packages(&self) -> Result<Vec<PackageName>> {
        let packages = self
            .extension
            .suggest_docs_packages(self.id.0.clone())
            .await?;

        Ok(packages
            .into_iter()
            .map(|package| PackageName::from(package.as_str()))
            .collect())
    }

    async fn index(&self, package: PackageName, database: Arc<IndexedDocsDatabase>) -> Result<()> {
        self.extension
            .index_docs(self.id.0.clone(), package.as_ref().into(), database)
            .await
    }
}
