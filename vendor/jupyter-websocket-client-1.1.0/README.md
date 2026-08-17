# `jupyter-websocket-client` crate

A WebSocket client for connecting to Jupyter kernels via a Jupyter Server.

For message types like `KernelInfoRequest` and `JupyterMessageContent`, use the [`jupyter-protocol`](https://crates.io/crates/jupyter-protocol) crate.

Note: This crate does _not_ support tokio at this time.

## Usage

```rust
use jupyter_websocket_client::RemoteServer;

use jupyter_protocol::{KernelInfoRequest, JupyterMessageContent};

// Import the sink and stream extensions to allow splitting the socket into a writer and reader pair
use futures::{SinkExt as _, StreamExt as _};

pub async fn connect_kernel() -> anyhow::Result<()> {
    let server = RemoteServer::from_url(
        "http://127.0.0.1:8888/lab?token=f487535a46268da4a0752c0e162c873b721e33a9e6ec8390"
    )?;

    // You'll need to launch a kernel and get a kernel ID using your own HTTP
    // request library
    let kernel_id = "1057-1057-1057-1057";

    let (kernel_socket, _response) = server.connect_to_kernel(kernel_id).await?;

    let (mut w, mut r) = kernel_socket.split();

    w.send(KernelInfoRequest {}.into()).await?;

    while let Some(response) = r.next().await.transpose()? {
        match response.content {
            JupyterMessageContent::KernelInfoReply(kernel_info_reply) => {
                println!("Received kernel_info_reply");
                println!("{:?}", kernel_info_reply);
                break;
            }
            other => {
                println!("Received");
                println!("{:?}", other);
            }
        }
    }

    Ok(())
}
```
