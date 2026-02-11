use anyhow::Result;
use futures::AsyncReadExt;
use http_client::{AsyncBody, HttpClient, Method, Request as HttpRequest};
use serde::{Deserialize, Serialize};

use crate::{Request, RequestError, Response};

/// A single request within a batch
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchRequestItem {
    pub custom_id: String,
    pub method: String,
    pub url: String,
    pub body: Request,
}

impl BatchRequestItem {
    pub fn new(custom_id: String, request: Request) -> Self {
        Self {
            custom_id,
            method: "POST".to_string(),
            url: "/v1/chat/completions".to_string(),
            body: request,
        }
    }

    pub fn to_jsonl_line(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

/// Request to create a batch
#[derive(Debug, Serialize)]
pub struct CreateBatchRequest {
    pub input_file_id: String,
    pub endpoint: String,
    pub completion_window: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

impl CreateBatchRequest {
    pub fn new(input_file_id: String) -> Self {
        Self {
            input_file_id,
            endpoint: "/v1/chat/completions".to_string(),
            completion_window: "24h".to_string(),
            metadata: None,
        }
    }
}

/// Response from batch creation or retrieval
#[derive(Debug, Serialize, Deserialize)]
pub struct Batch {
    pub id: String,
    pub object: String,
    pub endpoint: String,
    pub input_file_id: String,
    pub completion_window: String,
    pub status: String,
    pub output_file_id: Option<String>,
    pub error_file_id: Option<String>,
    pub created_at: u64,
    #[serde(default)]
    pub in_progress_at: Option<u64>,
    #[serde(default)]
    pub expires_at: Option<u64>,
    #[serde(default)]
    pub finalizing_at: Option<u64>,
    #[serde(default)]
    pub completed_at: Option<u64>,
    #[serde(default)]
    pub failed_at: Option<u64>,
    #[serde(default)]
    pub expired_at: Option<u64>,
    #[serde(default)]
    pub cancelling_at: Option<u64>,
    #[serde(default)]
    pub cancelled_at: Option<u64>,
    #[serde(default)]
    pub request_counts: Option<BatchRequestCounts>,
    #[serde(default)]
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BatchRequestCounts {
    pub total: u64,
    pub completed: u64,
    pub failed: u64,
}

/// Response from file upload
#[derive(Debug, Serialize, Deserialize)]
pub struct FileObject {
    pub id: String,
    pub object: String,
    pub bytes: u64,
    pub created_at: u64,
    pub filename: String,
    pub purpose: String,
}

/// Individual result from batch output
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchOutputItem {
    pub id: String,
    pub custom_id: String,
    pub response: Option<BatchResponseBody>,
    pub error: Option<BatchError>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchResponseBody {
    pub status_code: u16,
    pub body: Response,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchError {
    pub code: String,
    pub message: String,
}

/// Upload a JSONL file for batch processing
pub async fn upload_batch_file(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
    filename: &str,
    content: Vec<u8>,
) -> Result<FileObject, RequestError> {
    let uri = format!("{api_url}/files");

    let boundary = format!("----WebKitFormBoundary{:x}", rand::random::<u64>());

    let mut body = Vec::new();
    body.extend_from_slice(format!("--{boundary}\r\n").as_bytes());
    body.extend_from_slice(b"Content-Disposition: form-data; name=\"purpose\"\r\n\r\n");
    body.extend_from_slice(b"batch\r\n");
    body.extend_from_slice(format!("--{boundary}\r\n").as_bytes());
    body.extend_from_slice(
        format!("Content-Disposition: form-data; name=\"file\"; filename=\"{filename}\"\r\n")
            .as_bytes(),
    );
    body.extend_from_slice(b"Content-Type: application/jsonl\r\n\r\n");
    body.extend_from_slice(&content);
    body.extend_from_slice(format!("\r\n--{boundary}--\r\n").as_bytes());

    let request = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Authorization", format!("Bearer {}", api_key.trim()))
        .header(
            "Content-Type",
            format!("multipart/form-data; boundary={boundary}"),
        )
        .body(AsyncBody::from(body))
        .map_err(|e| RequestError::Other(e.into()))?;

    let mut response = client.send(request).await?;

    if response.status().is_success() {
        let mut body = String::new();
        response
            .body_mut()
            .read_to_string(&mut body)
            .await
            .map_err(|e| RequestError::Other(e.into()))?;

        serde_json::from_str(&body).map_err(|e| RequestError::Other(e.into()))
    } else {
        let mut body = String::new();
        response
            .body_mut()
            .read_to_string(&mut body)
            .await
            .map_err(|e| RequestError::Other(e.into()))?;

        Err(RequestError::HttpResponseError {
            provider: "openai".to_owned(),
            status_code: response.status(),
            body,
            headers: response.headers().clone(),
        })
    }
}

/// Create a batch from an uploaded file
pub async fn create_batch(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
    request: CreateBatchRequest,
) -> Result<Batch, RequestError> {
    let uri = format!("{api_url}/batches");

    let serialized = serde_json::to_string(&request).map_err(|e| RequestError::Other(e.into()))?;

    let request = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Authorization", format!("Bearer {}", api_key.trim()))
        .header("Content-Type", "application/json")
        .body(AsyncBody::from(serialized))
        .map_err(|e| RequestError::Other(e.into()))?;

    let mut response = client.send(request).await?;

    if response.status().is_success() {
        let mut body = String::new();
        response
            .body_mut()
            .read_to_string(&mut body)
            .await
            .map_err(|e| RequestError::Other(e.into()))?;

        serde_json::from_str(&body).map_err(|e| RequestError::Other(e.into()))
    } else {
        let mut body = String::new();
        response
            .body_mut()
            .read_to_string(&mut body)
            .await
            .map_err(|e| RequestError::Other(e.into()))?;

        Err(RequestError::HttpResponseError {
            provider: "openai".to_owned(),
            status_code: response.status(),
            body,
            headers: response.headers().clone(),
        })
    }
}

/// Retrieve batch status
pub async fn retrieve_batch(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
    batch_id: &str,
) -> Result<Batch, RequestError> {
    let uri = format!("{api_url}/batches/{batch_id}");

    let request = HttpRequest::builder()
        .method(Method::GET)
        .uri(uri)
        .header("Authorization", format!("Bearer {}", api_key.trim()))
        .body(AsyncBody::default())
        .map_err(|e| RequestError::Other(e.into()))?;

    let mut response = client.send(request).await?;

    if response.status().is_success() {
        let mut body = String::new();
        response
            .body_mut()
            .read_to_string(&mut body)
            .await
            .map_err(|e| RequestError::Other(e.into()))?;

        serde_json::from_str(&body).map_err(|e| RequestError::Other(e.into()))
    } else {
        let mut body = String::new();
        response
            .body_mut()
            .read_to_string(&mut body)
            .await
            .map_err(|e| RequestError::Other(e.into()))?;

        Err(RequestError::HttpResponseError {
            provider: "openai".to_owned(),
            status_code: response.status(),
            body,
            headers: response.headers().clone(),
        })
    }
}

/// Download file content (for batch results)
pub async fn download_file(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
    file_id: &str,
) -> Result<String, RequestError> {
    let uri = format!("{api_url}/files/{file_id}/content");

    let request = HttpRequest::builder()
        .method(Method::GET)
        .uri(uri)
        .header("Authorization", format!("Bearer {}", api_key.trim()))
        .body(AsyncBody::default())
        .map_err(|e| RequestError::Other(e.into()))?;

    let mut response = client.send(request).await?;

    if response.status().is_success() {
        let mut body = String::new();
        response
            .body_mut()
            .read_to_string(&mut body)
            .await
            .map_err(|e| RequestError::Other(e.into()))?;

        Ok(body)
    } else {
        let mut body = String::new();
        response
            .body_mut()
            .read_to_string(&mut body)
            .await
            .map_err(|e| RequestError::Other(e.into()))?;

        Err(RequestError::HttpResponseError {
            provider: "openai".to_owned(),
            status_code: response.status(),
            body,
            headers: response.headers().clone(),
        })
    }
}

/// Parse batch output JSONL into individual results
pub fn parse_batch_output(content: &str) -> Result<Vec<BatchOutputItem>, serde_json::Error> {
    content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| serde_json::from_str(line))
        .collect()
}
