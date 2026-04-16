use anyhow::Result;
use futures::AsyncReadExt;
use http_client::{AsyncBody, HttpClient, Method, Request as HttpRequest};
use serde::{Deserialize, Serialize};

use crate::{AnthropicError, ApiError, RateLimitInfo, Request, Response};

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchRequest {
    pub custom_id: String,
    pub params: Request,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBatchRequest {
    pub requests: Vec<BatchRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageBatchRequestCounts {
    pub processing: u64,
    pub succeeded: u64,
    pub errored: u64,
    pub canceled: u64,
    pub expired: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageBatch {
    pub id: String,
    #[serde(rename = "type")]
    pub batch_type: String,
    pub processing_status: String,
    pub request_counts: MessageBatchRequestCounts,
    pub ended_at: Option<String>,
    pub created_at: String,
    pub expires_at: String,
    pub archived_at: Option<String>,
    pub cancel_initiated_at: Option<String>,
    pub results_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum BatchResult {
    #[serde(rename = "succeeded")]
    Succeeded { message: Response },
    #[serde(rename = "errored")]
    Errored { error: BatchErrorResponse },
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "expired")]
    Expired,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchErrorResponse {
    #[serde(rename = "type")]
    pub response_type: String,
    pub error: ApiError,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchIndividualResponse {
    pub custom_id: String,
    pub result: BatchResult,
}

pub async fn create_batch(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
    request: CreateBatchRequest,
) -> Result<MessageBatch, AnthropicError> {
    let uri = format!("{api_url}/v1/messages/batches");

    let request_builder = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Anthropic-Version", "2023-06-01")
        .header("X-Api-Key", api_key.trim())
        .header("Content-Type", "application/json");

    let serialized_request =
        serde_json::to_string(&request).map_err(AnthropicError::SerializeRequest)?;
    let http_request = request_builder
        .body(AsyncBody::from(serialized_request))
        .map_err(AnthropicError::BuildRequestBody)?;

    let mut response = client
        .send(http_request)
        .await
        .map_err(AnthropicError::HttpSend)?;

    let rate_limits = RateLimitInfo::from_headers(response.headers());

    if response.status().is_success() {
        let mut body = String::new();
        response
            .body_mut()
            .read_to_string(&mut body)
            .await
            .map_err(AnthropicError::ReadResponse)?;

        serde_json::from_str(&body).map_err(AnthropicError::DeserializeResponse)
    } else {
        Err(crate::handle_error_response(response, rate_limits).await)
    }
}

pub async fn retrieve_batch(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
    message_batch_id: &str,
) -> Result<MessageBatch, AnthropicError> {
    let uri = format!("{api_url}/v1/messages/batches/{message_batch_id}");

    let request_builder = HttpRequest::builder()
        .method(Method::GET)
        .uri(uri)
        .header("Anthropic-Version", "2023-06-01")
        .header("X-Api-Key", api_key.trim());

    let http_request = request_builder
        .body(AsyncBody::default())
        .map_err(AnthropicError::BuildRequestBody)?;

    let mut response = client
        .send(http_request)
        .await
        .map_err(AnthropicError::HttpSend)?;

    let rate_limits = RateLimitInfo::from_headers(response.headers());

    if response.status().is_success() {
        let mut body = String::new();
        response
            .body_mut()
            .read_to_string(&mut body)
            .await
            .map_err(AnthropicError::ReadResponse)?;

        serde_json::from_str(&body).map_err(AnthropicError::DeserializeResponse)
    } else {
        Err(crate::handle_error_response(response, rate_limits).await)
    }
}

pub async fn retrieve_batch_results(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
    message_batch_id: &str,
) -> Result<Vec<BatchIndividualResponse>, AnthropicError> {
    let uri = format!("{api_url}/v1/messages/batches/{message_batch_id}/results");

    let request_builder = HttpRequest::builder()
        .method(Method::GET)
        .uri(uri)
        .header("Anthropic-Version", "2023-06-01")
        .header("X-Api-Key", api_key.trim());

    let http_request = request_builder
        .body(AsyncBody::default())
        .map_err(AnthropicError::BuildRequestBody)?;

    let mut response = client
        .send(http_request)
        .await
        .map_err(AnthropicError::HttpSend)?;

    let rate_limits = RateLimitInfo::from_headers(response.headers());

    if response.status().is_success() {
        let mut body = String::new();
        response
            .body_mut()
            .read_to_string(&mut body)
            .await
            .map_err(AnthropicError::ReadResponse)?;

        let mut results = Vec::new();
        for line in body.lines() {
            if line.trim().is_empty() {
                continue;
            }
            let result: BatchIndividualResponse =
                serde_json::from_str(line).map_err(AnthropicError::DeserializeResponse)?;
            results.push(result);
        }

        Ok(results)
    } else {
        Err(crate::handle_error_response(response, rate_limits).await)
    }
}
