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
pub struct Update {
    #[allow(dead_code)]
    pub execution_id: ExecutionId,
    pub content: JupyterMessageContent,
}

#[derive(Debug)]
pub struct Request {
    pub execution_id: ExecutionId,
    pub request: runtimelib::JupyterMessageContent,
    pub iopub_sender: mpsc::UnboundedSender<Update>,
}

pub async fn connect_tokio_kernel_interface(
    connection_info: &runtimelib::ConnectionInfo,
    mut shell_request_rx: mpsc::UnboundedReceiver<Request>,
) -> Result<()> {
    let mut iopub = connection_info.create_client_iopub_connection("").await?;
    let mut shell = connection_info.create_client_shell_connection().await?;

    let executions: Arc<tokio::sync::Mutex<HashMap<ExecutionId, mpsc::UnboundedSender<Update>>>> =
        Default::default();

    let iopub_handle: tokio::task::JoinHandle<anyhow::Result<()>> = tokio::spawn({
        let executions = executions.clone();
        async move {
            loop {
                let message = iopub.read().await?;

                if let Some(parent_header) = message.parent_header {
                    let execution_id = ExecutionId::from(parent_header.msg_id);

                    if let Some(mut execution) = executions.lock().await.get(&execution_id) {
                        execution
                            .send(Update {
                                execution_id,
                                content: message.content,
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
            while let Some(request) = shell_request_rx.next().await {
                let mut message = JupyterMessage::new(request.request, None);
                message.header.msg_id.clone_from(&request.execution_id.0);

                let sender = request.iopub_sender.clone();

                executions
                    .lock()
                    .await
                    .insert(request.execution_id.clone(), sender.clone());

                shell.send(message).await?;

                let mut sender = sender.clone();

                let reply = shell.read().await?;

                sender
                    .send(Update {
                        execution_id: request.execution_id,
                        content: reply.content,
                    })
                    .await
                    .ok();
            }
            anyhow::Ok(())
        }
    });

    let join_fut = futures::future::try_join(iopub_handle, shell_handle);

    let results = join_fut.await?;

    // todo!("If any of these error, we should send back an error using the sender");
    if let Err(e) = results.0 {
        log::error!("iopub error: {e:?}");
    }
    if let Err(e) = results.1 {
        log::error!("shell error: {e:?}");
    }
    anyhow::Ok(())
}
