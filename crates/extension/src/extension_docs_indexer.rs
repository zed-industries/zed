use std::sync::Arc;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use futures::FutureExt;
use indexed_docs::{IndexDocs, IndexedDocsDatabase, IndexedDocsProvider, PackageName};
use wasmtime_wasi::WasiView;

use crate::wasm_host::{WasmExtension, WasmHost};

pub struct ExtensionDocsIndexer {
    pub(crate) extension: WasmExtension,
    pub(crate) host: Arc<WasmHost>,
    pub(crate) name: Arc<str>,
}

#[async_trait]
impl IndexDocs for ExtensionDocsIndexer {
    async fn index(
        &self,
        package: PackageName,
        database: Arc<IndexedDocsDatabase>,
        provider: Box<dyn IndexedDocsProvider + Send + Sync + 'static>,
    ) -> Result<()> {
        self.extension
            .call({
                let name = self.name.clone();
                |extension, store| {
                    async move {
                        let database_resource = store.data_mut().table().push(database)?;
                        extension
                            .call_index_docs(
                                store,
                                name.as_ref(),
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
