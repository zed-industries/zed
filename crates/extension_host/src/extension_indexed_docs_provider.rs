use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use futures::FutureExt;
use indexed_docs::{IndexedDocsDatabase, IndexedDocsProvider, PackageName, ProviderId};
use wasmtime_wasi::WasiView;

use crate::wasm_host::{WasmExtension, WasmHost};

pub struct ExtensionIndexedDocsProvider {
    pub(crate) extension: WasmExtension,
    pub(crate) host: Arc<WasmHost>,
    pub(crate) id: ProviderId,
}

#[async_trait]
impl IndexedDocsProvider for ExtensionIndexedDocsProvider {
    fn id(&self) -> ProviderId {
        self.id.clone()
    }

    fn database_path(&self) -> PathBuf {
        let mut database_path = self.host.work_dir.clone();
        database_path.push(self.extension.manifest.id.as_ref());
        database_path.push("docs");
        database_path.push(format!("{}.0.mdb", self.id));

        database_path
    }

    async fn suggest_packages(&self) -> Result<Vec<PackageName>> {
        self.extension
            .call({
                let id = self.id.clone();
                |extension, store| {
                    async move {
                        let packages = extension
                            .call_suggest_docs_packages(store, id.as_ref())
                            .await?
                            .map_err(|err| anyhow!("{err:?}"))?;

                        Ok(packages
                            .into_iter()
                            .map(|package| PackageName::from(package.as_str()))
                            .collect())
                    }
                    .boxed()
                }
            })
            .await
    }

    async fn index(&self, package: PackageName, database: Arc<IndexedDocsDatabase>) -> Result<()> {
        self.extension
            .call({
                let id = self.id.clone();
                |extension, store| {
                    async move {
                        let database_resource = store.data_mut().table().push(database)?;
                        extension
                            .call_index_docs(
                                store,
                                id.as_ref(),
                                package.as_ref(),
                                database_resource,
                            )
                            .await?
                            .map_err(|err| anyhow!("{err:?}"))?;

                        anyhow::Ok(())
                    }
                    .boxed()
                }
            })
            .await
    }
}
