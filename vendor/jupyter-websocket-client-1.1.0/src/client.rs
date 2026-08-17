use anyhow::{Context, Result};
use async_tungstenite::{
    tokio::connect_async,
    tungstenite::{
        client::IntoClientRequest,
        http::{HeaderValue, Request, Response},
    },
};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::binary_protocol::KERNEL_WEBSOCKET_PROTOCOL;
use crate::websocket::{JupyterWebSocket, ProtocolMode};

pub struct RemoteServer {
    pub base_url: String,
    pub token: String,
}

// Only `id` is used right now, but other fields will be useful when pulling up a listing later
#[derive(Debug, Serialize, Deserialize)]
pub struct Kernel {
    pub id: String,
    pub name: String,
    pub last_activity: String,
    pub execution_state: String,
    pub connections: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub path: String,
    pub name: String,
    #[serde(rename = "type")]
    pub session_type: String,
    pub kernel: Kernel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewSession {
    pub path: String,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KernelSpec {
    pub name: String,
    pub spec: jupyter_protocol::JupyterKernelspec,
    pub resources: std::collections::HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KernelLaunchRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HelpLink {
    pub text: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KernelSpecsResponse {
    pub default: String,
    pub kernelspecs: std::collections::HashMap<String, KernelSpec>,
}

fn api_url(base_url: &str, path: &str) -> String {
    format!(
        "{}/api/{}",
        base_url.trim_end_matches('/'),
        path.trim_start_matches('/')
    )
}

impl RemoteServer {
    pub fn from_url(url: &str) -> Result<Self> {
        let parsed_url = Url::parse(url).context("Failed to parse Jupyter URL")?;
        let base_url = format!(
            "{}://{}{}{}",
            parsed_url.scheme(),
            parsed_url.host_str().unwrap_or("localhost"),
            parsed_url
                .port()
                .map(|p| format!(":{}", p))
                .unwrap_or_default(),
            parsed_url.path().trim_end_matches("/tree")
        );

        let token = parsed_url
            .query_pairs()
            .find(|(key, _)| key == "token")
            .map(|(_, value)| value.into_owned())
            .ok_or_else(|| anyhow::anyhow!("Token not found in URL"))?;

        Ok(Self { base_url, token })
    }

    pub fn api_url(&self, path: &str) -> String {
        api_url(&self.base_url, path)
    }

    /// Connect to a kernel by ID
    ///
    /// ```rust
    /// use jupyter_websocket_client::RemoteServer;
    ///
    /// use jupyter_protocol::{KernelInfoRequest, JupyterMessageContent};
    ///
    /// // Import the sink and stream extensions to allow splitting the socket into a writer and reader pair
    /// use futures::{SinkExt as _, StreamExt as _};
    ///
    /// pub async fn connect_kernel() -> anyhow::Result<()> {
    ///     let server = RemoteServer::from_url(
    ///         "http://127.0.0.1:8888/lab?token=f487535a46268da4a0752c0e162c873b721e33a9e6ec8390"
    ///     )?;
    ///
    ///     // You'll need to launch a kernel and get a kernel ID using your own HTTP
    ///     // request library
    ///     let kernel_id = "1057-1057-1057-1057";
    ///
    ///     let (kernel_socket, response) = server.connect_to_kernel(kernel_id).await?;
    ///
    ///     let (mut w, mut r) = kernel_socket.split();
    ///
    ///     w.send(KernelInfoRequest {}.into()).await?;
    ///
    ///     while let Some(response) = r.next().await.transpose()? {
    ///         match response.content {
    ///             JupyterMessageContent::KernelInfoReply(kernel_info_reply) => {
    ///                 println!("Received kernel_info_reply");
    ///                 println!("{:?}", kernel_info_reply);
    ///                 break;
    ///             }
    ///             other => {
    ///                 println!("Received");
    ///                 println!("{:?}", other);
    ///             }
    ///         }
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn connect_to_kernel(
        &self,
        kernel_id: &str,
    ) -> Result<(JupyterWebSocket, Response<Option<Vec<u8>>>)> {
        self.connect_to_kernel_with_session(kernel_id, None).await
    }

    /// Connect to a kernel by ID with an optional session ID.
    ///
    /// The session_id parameter associates this WebSocket connection with a
    /// Jupyter session. This is important for servers that track connections
    /// by session (like jupyter-server-documents) to avoid premature kernel
    /// shutdown when connections drop.
    pub async fn connect_to_kernel_with_session(
        &self,
        kernel_id: &str,
        session_id: Option<&str>,
    ) -> Result<(JupyterWebSocket, Response<Option<Vec<u8>>>)> {
        let mut ws_url = format!(
            "{}?token={}",
            api_url(&self.base_url, &format!("kernels/{}/channels", kernel_id))
                .replace("http", "ws"),
            self.token
        );

        if let Some(sid) = session_id {
            ws_url.push_str(&format!("&session_id={}", sid));
        }

        let mut req: Request<()> = ws_url.into_client_request()?;
        let headers = req.headers_mut();
        headers.insert(
            "User-Agent",
            HeaderValue::from_str("runtimed/jupyter-websocket-client")?,
        );
        // Request the v1 binary protocol
        headers.insert(
            "Sec-WebSocket-Protocol",
            HeaderValue::from_static(KERNEL_WEBSOCKET_PROTOCOL),
        );

        let response = connect_async(req).await;

        let (ws_stream, response) = response?;

        // Check if server accepted the v1 binary protocol
        let protocol_mode = response
            .headers()
            .get("Sec-WebSocket-Protocol")
            .and_then(|v| v.to_str().ok())
            .map(|v| {
                if v == KERNEL_WEBSOCKET_PROTOCOL {
                    ProtocolMode::BinaryV1
                } else {
                    ProtocolMode::Json
                }
            })
            .unwrap_or(ProtocolMode::Json);

        Ok((
            JupyterWebSocket {
                inner: ws_stream,
                protocol_mode,
            },
            response,
        ))
    }
}
