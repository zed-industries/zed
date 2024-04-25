// use anyhow::{anyhow, Context, Result};
// use futures::{io::BufReader, stream::BoxStream, AsyncBufReadExt, AsyncReadExt, StreamExt};
use serde::{Deserialize, Serialize};
// use serde_json::{Map, Value};
// use std::{convert::TryFrom, future::Future};
// use util::http::{AsyncBody, HttpClient, Method, Request as HttpRequest};

#[derive(Serialize)]
pub struct GetApiKeyRequest {
    pub user_id: String,
}

#[derive(Serialize)]
pub struct CreateApiKeyRequest {
    pub user_id: String,
    pub email: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateApiKeyResponse {
    pub api_key: String,
}

#[derive(Deserialize)]
pub struct SupermavenApiError {
    pub message: String,
}
