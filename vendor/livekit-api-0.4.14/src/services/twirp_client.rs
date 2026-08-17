// Copyright 2025 LiveKit, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fmt::Display;

use http::{
    header::{HeaderMap, HeaderValue, CONTENT_TYPE},
    StatusCode,
};
use serde::Deserialize;
use thiserror::Error;

use crate::http_client;

pub const DEFAULT_PREFIX: &str = "/twirp";

#[derive(Debug, Error)]
pub enum TwirpError {
    #[cfg(feature = "services-tokio")]
    #[error("failed to execute the request: {0}")]
    Request(#[from] reqwest::Error),
    #[cfg(feature = "services-async")]
    #[error("failed to execute the request: {0}")]
    Request(#[from] std::io::Error),
    #[error("twirp error: {0}")]
    Twirp(TwirpErrorCode),
    #[error("url error: {0}")]
    Url(#[from] url::ParseError),
    #[error("prost error: {0}")]
    Prost(#[from] prost::DecodeError),
}

#[derive(Debug, Deserialize)]
pub struct TwirpErrorCode {
    pub code: String,
    pub msg: String,
}

impl TwirpErrorCode {
    pub const CANCELED: &'static str = "canceled";
    pub const UNKNOWN: &'static str = "unknown";
    pub const INVALID_ARGUMENT: &'static str = "invalid_argument";
    pub const MALFORMED: &'static str = "malformed";
    pub const DEADLINE_EXCEEDED: &'static str = "deadline_exceeded";
    pub const NOT_FOUND: &'static str = "not_found";
    pub const BAD_ROUTE: &'static str = "bad_route";
    pub const ALREADY_EXISTS: &'static str = "already_exists";
    pub const PERMISSION_DENIED: &'static str = "permission_denied";
    pub const UNAUTHENTICATED: &'static str = "unauthenticated";
    pub const RESOURCE_EXHAUSTED: &'static str = "resource_exhausted";
    pub const FAILED_PRECONDITION: &'static str = "failed_precondition";
    pub const ABORTED: &'static str = "aborted";
    pub const OUT_OF_RANGE: &'static str = "out_of_range";
    pub const UNIMPLEMENTED: &'static str = "unimplemented";
    pub const INTERNAL: &'static str = "internal";
    pub const UNAVAILABLE: &'static str = "unavailable";
    pub const DATA_LOSS: &'static str = "dataloss";
}

impl Display for TwirpErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.msg)
    }
}

pub type TwirpResult<T> = Result<T, TwirpError>;

#[derive(Debug)]
pub struct TwirpClient {
    host: String,
    pkg: String,
    prefix: String,
    client: http_client::Client,
}

impl TwirpClient {
    pub fn new(host: &str, pkg: &str, prefix: Option<&str>) -> Self {
        Self {
            host: host.to_owned(),
            pkg: pkg.to_owned(),
            prefix: prefix.unwrap_or(DEFAULT_PREFIX).to_owned(),
            client: http_client::Client::new(),
        }
    }

    pub async fn request<D: prost::Message, R: prost::Message + Default>(
        &self,
        service: &str,
        method: &str,
        data: D,
        mut headers: HeaderMap,
    ) -> TwirpResult<R> {
        let mut url = url::Url::parse(&self.host)?;

        url.set_path(&format!("{}/{}.{}/{}", self.prefix, self.pkg, service, method));

        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/protobuf"));

        let resp = self.client.post(url).headers(headers).body(data.encode_to_vec()).send().await?;

        if resp.status() == StatusCode::OK {
            Ok(R::decode(resp.bytes().await?)?)
        } else {
            let err: TwirpErrorCode = resp.json().await?;
            Err(TwirpError::Twirp(err))
        }
    }
}
