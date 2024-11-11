use std::pin::Pin;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use context_servers::manager::{NativeContextServer, ServerConfig};
use context_servers::protocol::InitializedContextServerProtocol;
use context_servers::ContextServer;
use extension_host::wasm_host::{WasmExtension, WasmHost};
use futures::{Future, FutureExt};
use gpui::AsyncAppContext;

pub struct ExtensionContextServer {
    #[allow(unused)]
    pub(crate) extension: WasmExtension,
    #[allow(unused)]
    pub(crate) host: Arc<WasmHost>,
    id: Arc<str>,
    context_server: Arc<NativeContextServer>,
}

impl ExtensionContextServer {
    pub async fn new(extension: WasmExtension, host: Arc<WasmHost>, id: Arc<str>) -> Result<Self> {
        let command = extension
            .call({
                let id = id.clone();
                |extension, store| {
                    async move {
                        let command = extension
                            .call_context_server_command(store, id.clone())
                            .await?
                            .map_err(|e| anyhow!("{}", e))?;
                        anyhow::Ok(command)
                    }
                    .boxed()
                }
            })
            .await?;

        let config = Arc::new(ServerConfig {
            id: id.to_string(),
            executable: command.command,
            args: command.args,
            env: Some(command.env.into_iter().collect()),
        });

        anyhow::Ok(Self {
            extension,
            host,
            id,
            context_server: Arc::new(NativeContextServer::new(config)),
        })
    }
}

#[async_trait(?Send)]
impl ContextServer for ExtensionContextServer {
    fn id(&self) -> Arc<str> {
        self.id.clone()
    }

    fn config(&self) -> Arc<ServerConfig> {
        self.context_server.config()
    }

    fn client(&self) -> Option<Arc<InitializedContextServerProtocol>> {
        self.context_server.client()
    }

    fn start<'a>(
        self: Arc<Self>,
        cx: &'a AsyncAppContext,
    ) -> Pin<Box<dyn 'a + Future<Output = Result<()>>>> {
        self.context_server.clone().start(cx)
    }

    fn stop(&self) -> Result<()> {
        self.context_server.stop()
    }
}
