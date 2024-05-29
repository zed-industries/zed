use anyhow::Result;
use collections::HashMap;
use futures::channel::mpsc;
use futures::{SinkExt as _, StreamExt as _};
use runtimelib::{JupyterMessage, JupyterMessageContent};
use std::sync::Arc;

#[derive(Debug)]
pub struct Request {
    pub request: runtimelib::JupyterMessageContent,
    pub responses_rx: mpsc::UnboundedSender<JupyterMessageContent>,
}

pub async fn connect_tokio_kernel_interface(
    connection_info: &runtimelib::ConnectionInfo,
    mut request_rx: mpsc::UnboundedReceiver<Request>,
) -> Result<()> {
    // This is a one way channel that feeds us message from the kernel
    // Event Stream --> always emitting
    let mut iopub = connection_info.create_client_iopub_connection("").await?;
    // Request/Reply
    let mut shell = connection_info.create_client_shell_connection().await?;
    // Request/Reply
    // let mut control = connection_info.create_client_control_connection().await?;

    let child_messages: Arc<
        tokio::sync::Mutex<HashMap<String, mpsc::UnboundedSender<JupyterMessageContent>>>,
    > = Default::default();

    let iopub_handle: tokio::task::JoinHandle<anyhow::Result<()>> = tokio::spawn({
        let child_messages = child_messages.clone();
        async move {
            loop {
                let message = iopub.read().await?;

                if let Some(parent_header) = message.parent_header {
                    if let Some(mut sender) = child_messages.lock().await.get(&parent_header.msg_id)
                    {
                        sender.send(message.content).await.ok();
                    }
                }
            }
        }
    });

    let shell_handle: tokio::task::JoinHandle<anyhow::Result<()>> = tokio::spawn({
        let child_messages = child_messages.clone();
        async move {
            while let Some(request) = request_rx.next().await {
                let message = JupyterMessage::new(request.request, None);

                let sender = request.responses_rx.clone();

                child_messages
                    .lock()
                    .await
                    .insert(message.header.msg_id.clone(), sender.clone());

                shell.send(message).await?;

                let mut sender = sender.clone();
                let reply = shell.read().await?;
                sender.send(reply.content).await.ok();
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
