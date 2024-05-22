use anyhow::Result;
use collections::HashMap;
use futures::channel::mpsc;
use futures::{SinkExt as _, StreamExt as _};
use runtimelib::{JupyterMessage, JupyterMessageContent};
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ExecutionId(String);

impl ExecutionId {
    pub fn new() -> Self {
        ExecutionId(uuid::Uuid::new_v4().to_string())
    }
}

impl From<String> for ExecutionId {
    fn from(id: String) -> Self {
        ExecutionId(id)
    }
}

#[derive(Debug)]
pub struct ExecutionUpdate {
    #[allow(dead_code)]
    pub execution_id: ExecutionId,
    pub update: JupyterMessageContent,
}

#[derive(Debug)]
pub struct ExecutionRequest {
    pub execution_id: ExecutionId,
    pub request: runtimelib::ExecuteRequest,
    pub response_sender: mpsc::UnboundedSender<ExecutionUpdate>,
}

pub async fn connect_tokio_kernel_interface(
    connection_info: &runtimelib::ConnectionInfo,
    mut execution_request_rx: mpsc::UnboundedReceiver<ExecutionRequest>,
) -> Result<()> {
    let mut iopub = connection_info.create_client_iopub_connection("").await?;
    let mut shell = connection_info.create_client_shell_connection().await?;

    let executions: Arc<
        tokio::sync::Mutex<HashMap<ExecutionId, mpsc::UnboundedSender<ExecutionUpdate>>>,
    > = Default::default();

    let iopub_handle: tokio::task::JoinHandle<anyhow::Result<()>> = tokio::spawn({
        let executions = executions.clone();
        async move {
            loop {
                let message = iopub.read().await?;

                if let Some(parent_header) = message.parent_header {
                    let execution_id = ExecutionId::from(parent_header.msg_id);

                    if let Some(mut execution) = executions.lock().await.get(&execution_id) {
                        execution
                            .send(ExecutionUpdate {
                                execution_id,
                                update: message.content,
                            })
                            .await
                            .ok();
                    }
                }
            }
        }
    });

    let shell_handle: tokio::task::JoinHandle<anyhow::Result<()>> = tokio::spawn({
        let executions = executions.clone();
        async move {
            while let Some(execution) = execution_request_rx.next().await {
                let mut message: JupyterMessage = execution.request.into();
                message.header.msg_id.clone_from(&execution.execution_id.0);

                executions
                    .lock()
                    .await
                    .insert(execution.execution_id, execution.response_sender);

                shell
                    .send(message)
                    .await
                    .map_err(|e| anyhow::anyhow!("Failed to send execute request: {e:?}"))?;
            }
            anyhow::Ok(())
        }
    });

    let join_fut = futures::future::try_join(iopub_handle, shell_handle);

    let results = join_fut.await?;

    if let Err(e) = results.0 {
        log::error!("iopub error: {e:?}");
    }
    if let Err(e) = results.1 {
        log::error!("shell error: {e:?}");
    }
    anyhow::Ok(())
}
