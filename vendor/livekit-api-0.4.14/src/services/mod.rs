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

use std::fmt::Debug;

use http::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use thiserror::Error;

use crate::access_token::{AccessToken, AccessTokenError, SIPGrants, VideoGrants};

pub use twirp_client::{TwirpError, TwirpErrorCode, TwirpResult};

pub mod agent_dispatch;
pub mod connector;
pub mod egress;
pub mod ingress;
pub mod room;
pub mod sip;

mod twirp_client;

pub const LIVEKIT_PACKAGE: &str = "livekit";

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("invalid environment: {0}")]
    Env(#[from] std::env::VarError),
    #[error("invalid access token: {0}")]
    AccessToken(#[from] AccessTokenError),
    #[error("twirp error: {0}")]
    Twirp(#[from] TwirpError),
}

pub type ServiceResult<T> = Result<T, ServiceError>;

struct ServiceBase {
    api_key: String,
    api_secret: String,
}

impl Debug for ServiceBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ServiceBase").field("api_key", &self.api_key).finish()
    }
}

impl ServiceBase {
    pub fn with_api_key(api_key: &str, api_secret: &str) -> Self {
        Self { api_key: api_key.to_owned(), api_secret: api_secret.to_owned() }
    }

    pub fn auth_header(
        &self,
        grants: VideoGrants,
        sip: Option<SIPGrants>,
    ) -> Result<HeaderMap, AccessTokenError> {
        let mut tok =
            AccessToken::with_api_key(&self.api_key, &self.api_secret).with_grants(grants);
        if sip.is_some() {
            tok = tok.with_sip_grants(sip.unwrap())
        }
        let token = tok.to_jwt()?;

        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap());
        Ok(headers)
    }
}
