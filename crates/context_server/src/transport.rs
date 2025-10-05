pub mod http;
pub mod sse;
mod stdio_transport;

use anyhow::{Result, anyhow};
use async_trait::async_trait;
use collections::HashMap;
use futures::Stream;
use gpui::App;
use http_client::HttpClient;
use std::{pin::Pin, sync::Arc};
use url::Url;

pub use self::http::*;
pub use self::sse::*;
pub use stdio_transport::*;

/// Authentication configuration for HTTP transports
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthType {
    /// Bearer token authentication (Authorization: Bearer <token>)
    Bearer(String),
    /// API Key with custom header name
    ApiKey { header_name: String, value: String },
    /// Custom headers
    Custom(HashMap<String, String>),
}

impl AuthType {
    /// Convert authentication type to HTTP headers
    pub fn to_headers(&self) -> HashMap<String, String> {
        match self {
            AuthType::Bearer(token) => {
                let mut headers = HashMap::default();
                headers.insert("Authorization".to_string(), format!("Bearer {}", token));
                headers
            }
            AuthType::ApiKey { header_name, value } => {
                let mut headers = HashMap::default();
                headers.insert(header_name.clone(), value.clone());
                headers
            }
            AuthType::Custom(custom_headers) => custom_headers.clone(),
        }
    }
}

/// Configuration for transport creation
#[derive(Debug, Clone, Default)]
pub struct TransportConfig {
    pub auth: Option<AuthType>,
}

#[async_trait]
pub trait Transport: Send + Sync {
    async fn send(&self, message: String) -> Result<()>;
    fn receive(&self) -> Pin<Box<dyn Stream<Item = String> + Send>>;
    fn receive_err(&self) -> Pin<Box<dyn Stream<Item = String> + Send>>;
}

pub fn build_transport(
    http_client: Arc<dyn HttpClient>,
    endpoint: &Url,
    config: Option<TransportConfig>,
    cx: &App,
) -> Result<Arc<dyn Transport>> {
    log::info!("Creating transport for endpoint: {}", endpoint);
    
    let auth_headers = config
        .and_then(|c| c.auth)
        .map(|auth| auth.to_headers())
        .unwrap_or_default();
    
    match endpoint.scheme() {
        "http" | "https" => {
            log::info!("Using HTTP transport for {}", endpoint);
            let transport = HttpTransport::new(
                http_client,
                endpoint.to_string(),
                cx,
            );
            let transport = if !auth_headers.is_empty() {
                transport.with_auth_headers(auth_headers)
            } else {
                transport
            };
            Ok(Arc::new(transport))
        }
        "sse" => {
            log::info!("Using SSE transport for {}", endpoint);
            Ok(Arc::new(SseTransport::new(
                http_client,
                endpoint.to_string(),
            )))
        }
        _ => {
            log::error!("Unsupported URL scheme: {}", endpoint.scheme());
            Err(anyhow!("unsupported scheme {}", endpoint.scheme()))
        }
    }
}
