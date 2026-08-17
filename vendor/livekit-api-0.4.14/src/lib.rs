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

#[cfg(feature = "access-token")]
pub mod access_token;

#[cfg(all(any(feature = "services-tokio", feature = "services-async"), feature = "access-token"))]
pub mod services;

#[cfg(any(
    feature = "signal-client-tokio",
    feature = "signal-client-async",
    feature = "signal-client-dispatcher"
))]
pub mod signal_client;

#[cfg(any(
    feature = "signal-client-tokio",
    feature = "signal-client-async",
    feature = "signal-client-dispatcher",
    feature = "services-tokio",
    feature = "services-async"
))]
mod http_client;

#[cfg(feature = "webhooks")]
pub mod webhooks;

#[allow(dead_code)]
pub(crate) fn get_env_keys() -> Result<(String, String), std::env::VarError> {
    let api_key = std::env::var("LIVEKIT_API_KEY")?;
    let api_secret = std::env::var("LIVEKIT_API_SECRET")?;
    Ok((api_key, api_secret))
}
