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

use std::{
    collections::HashMap,
    env,
    fmt::Debug,
    ops::Add,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use jsonwebtoken::{self, DecodingKey, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::get_env_keys;

pub const DEFAULT_TTL: Duration = Duration::from_secs(3600 * 6); // 6 hours

#[derive(Debug, Error)]
pub enum AccessTokenError {
    #[error("Invalid API Key or Secret Key")]
    InvalidKeys,
    #[error("Invalid environment")]
    InvalidEnv(#[from] env::VarError),
    #[error("invalid claims: {0}")]
    InvalidClaims(&'static str),
    #[error("failed to encode jwt")]
    Encoding(#[from] jsonwebtoken::errors::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VideoGrants {
    // actions on rooms
    #[serde(default)]
    pub room_create: bool,
    #[serde(default)]
    pub room_list: bool,
    #[serde(default)]
    pub room_record: bool,

    // actions on a particular room
    #[serde(default)]
    pub room_admin: bool,
    #[serde(default)]
    pub room_join: bool,
    #[serde(default)]
    pub room: String,
    #[serde(default)]
    pub destination_room: String,

    // permissions within a room
    #[serde(default = "default_true")]
    pub can_publish: bool,
    #[serde(default = "default_true")]
    pub can_subscribe: bool,
    #[serde(default = "default_true")]
    pub can_publish_data: bool,

    // TrackSource types that a participant may publish.
    // When set, it supercedes CanPublish. Only sources explicitly set here can be published
    #[serde(default)]
    pub can_publish_sources: Vec<String>, // keys keep track of each source

    // by default, a participant is not allowed to update its own metadata
    #[serde(default)]
    pub can_update_own_metadata: bool,

    // actions on ingresses
    #[serde(default)]
    pub ingress_admin: bool, // applies to all ingress

    // participant is not visible to other participants (useful when making bots)
    #[serde(default)]
    pub hidden: bool,

    // indicates to the room that current participant is a recorder
    #[serde(default)]
    pub recorder: bool,
}

/// Used for fields that default to true instead of using the `Default` trait.
fn default_true() -> bool {
    true
}

impl Default for VideoGrants {
    fn default() -> Self {
        Self {
            room_create: false,
            room_list: false,
            room_record: false,
            room_admin: false,
            room_join: false,
            room: "".to_string(),
            destination_room: "".to_string(),
            can_publish: true,
            can_subscribe: true,
            can_publish_data: true,
            can_publish_sources: Vec::default(),
            can_update_own_metadata: false,
            ingress_admin: false,
            hidden: false,
            recorder: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SIPGrants {
    // manage sip resources
    pub admin: bool,
    // make outbound calls
    pub call: bool,
}

impl Default for SIPGrants {
    fn default() -> Self {
        Self { admin: false, call: false }
    }
}

#[derive(Debug, Clone, Serialize, Default, Deserialize, PartialEq)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct Claims {
    pub exp: usize,  // Expiration
    pub iss: String, // ApiKey
    pub nbf: usize,
    pub sub: String, // Identity

    pub name: String,
    pub video: VideoGrants,
    pub sip: SIPGrants,
    pub sha256: String, // Used to verify the integrity of the message body
    pub metadata: String,
    pub attributes: HashMap<String, String>,
    pub room_config: Option<livekit_protocol::RoomConfiguration>,
}

impl Claims {
    pub fn from_unverified(token: &str) -> Result<Self, AccessTokenError> {
        let token = jsonwebtoken::dangerous::insecure_decode::<Claims>(token)?;
        Ok(token.claims)
    }
}

#[derive(Clone)]
pub struct AccessToken {
    api_key: String,
    api_secret: String,
    claims: Claims,
}

impl Debug for AccessToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Don't show api_secret here
        f.debug_struct("AccessToken")
            .field("api_key", &self.api_key)
            .field("claims", &self.claims)
            .finish()
    }
}

impl AccessToken {
    pub fn with_api_key(api_key: &str, api_secret: &str) -> Self {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        Self {
            api_key: api_key.to_owned(),
            api_secret: api_secret.to_owned(),
            claims: Claims {
                exp: now.add(DEFAULT_TTL).as_secs() as usize,
                iss: api_key.to_owned(),
                nbf: now.as_secs() as usize,
                sub: Default::default(),
                name: Default::default(),
                video: VideoGrants::default(),
                sip: SIPGrants::default(),
                sha256: Default::default(),
                metadata: Default::default(),
                attributes: HashMap::new(),
                room_config: Default::default(),
            },
        }
    }

    #[cfg(test)]
    pub fn from_parts(api_key: &str, api_secret: &str, claims: Claims) -> Self {
        Self { api_key: api_key.to_owned(), api_secret: api_secret.to_owned(), claims }
    }

    pub fn new() -> Result<Self, AccessTokenError> {
        // Try to get the API Key and the Secret Key from the environment
        let (api_key, api_secret) = get_env_keys()?;
        Ok(Self::with_api_key(&api_key, &api_secret))
    }

    pub fn with_ttl(mut self, ttl: Duration) -> Self {
        let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap() + ttl;
        self.claims.exp = time.as_secs() as usize;
        self
    }

    pub fn with_grants(mut self, grants: VideoGrants) -> Self {
        self.claims.video = grants;
        self
    }

    pub fn with_sip_grants(mut self, grants: SIPGrants) -> Self {
        self.claims.sip = grants;
        self
    }

    pub fn with_identity(mut self, identity: &str) -> Self {
        self.claims.sub = identity.to_owned();
        self
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.claims.name = name.to_owned();
        self
    }

    pub fn with_metadata(mut self, metadata: &str) -> Self {
        self.claims.metadata = metadata.to_owned();
        self
    }

    pub fn with_attributes<I, K, V>(mut self, attributes: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        self.claims.attributes =
            attributes.into_iter().map(|(k, v)| (k.into(), v.into())).collect::<HashMap<_, _>>();
        self
    }

    pub fn with_sha256(mut self, sha256: &str) -> Self {
        self.claims.sha256 = sha256.to_owned();
        self
    }

    pub fn with_room_config(mut self, config: livekit_protocol::RoomConfiguration) -> Self {
        self.claims.room_config = Some(config);
        self
    }

    pub fn to_jwt(self) -> Result<String, AccessTokenError> {
        if self.api_key.is_empty() || self.api_secret.is_empty() {
            return Err(AccessTokenError::InvalidKeys);
        }

        if self.claims.video.room_join
            && (self.claims.sub.is_empty() || self.claims.video.room.is_empty())
        {
            return Err(AccessTokenError::InvalidClaims(
                "token grants room_join but doesn't have an identity or room",
            ));
        }

        Ok(jsonwebtoken::encode(
            &Header::new(jsonwebtoken::Algorithm::HS256),
            &self.claims,
            &EncodingKey::from_secret(self.api_secret.as_ref()),
        )?)
    }
}

#[derive(Clone)]
pub struct TokenVerifier {
    api_key: String,
    api_secret: String,
}

impl Debug for TokenVerifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TokenVerifier").field("api_key", &self.api_key).finish()
    }
}

impl TokenVerifier {
    pub fn with_api_key(api_key: &str, api_secret: &str) -> Self {
        Self { api_key: api_key.to_owned(), api_secret: api_secret.to_owned() }
    }

    pub fn new() -> Result<Self, AccessTokenError> {
        let (api_key, api_secret) = get_env_keys()?;
        Ok(Self::with_api_key(&api_key, &api_secret))
    }

    pub fn verify(&self, token: &str) -> Result<Claims, AccessTokenError> {
        let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
        validation.validate_exp = true;
        validation.validate_nbf = true;
        validation.set_issuer(&[&self.api_key]);

        let token = jsonwebtoken::decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.api_secret.as_ref()),
            &validation,
        )?;

        Ok(token.claims)
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::{AccessToken, Claims, TokenVerifier, VideoGrants};

    const TEST_API_KEY: &str = "myapikey";
    const TEST_API_SECRET: &str = "thiskeyistotallyunsafe";
    const TEST_TOKEN: &str = include_str!("test_token.txt");

    #[test]
    fn test_access_token() {
        let room_config = livekit_protocol::RoomConfiguration {
            name: "name".to_string(),
            agents: vec![livekit_protocol::RoomAgentDispatch {
                agent_name: "test-agent".to_string(),
                metadata: "test-metadata".to_string(),
            }],
            ..Default::default()
        };

        let token = AccessToken::with_api_key(TEST_API_KEY, TEST_API_SECRET)
            .with_ttl(Duration::from_secs(60))
            .with_identity("test")
            .with_name("test")
            .with_grants(VideoGrants::default())
            .with_room_config(room_config.clone())
            .to_jwt()
            .unwrap();

        let verifier = TokenVerifier::with_api_key(TEST_API_KEY, TEST_API_SECRET);
        let claims = verifier.verify(&token).unwrap();

        assert_eq!(claims.sub, "test");
        assert_eq!(claims.name, "test");
        assert_eq!(claims.iss, TEST_API_KEY);
        assert_eq!(claims.room_config, Some(room_config));

        let incorrect_issuer = TokenVerifier::with_api_key("incorrect", TEST_API_SECRET);
        assert!(incorrect_issuer.verify(&token).is_err());

        let incorrect_token = TokenVerifier::with_api_key(TEST_API_KEY, "incorrect");
        assert!(incorrect_token.verify(&token).is_err());
    }

    #[test]
    fn test_verify_token_with_room_config() {
        let verifier = TokenVerifier::with_api_key(TEST_API_KEY, TEST_API_SECRET);
        // This token was generated using the Python SDK.
        let claims = verifier.verify(TEST_TOKEN).expect("Failed to verify token.");

        assert_eq!(
            super::Claims {
                sub: "identity".to_string(),
                name: "name".to_string(),
                room_config: Some(livekit_protocol::RoomConfiguration {
                    agents: vec![livekit_protocol::RoomAgentDispatch {
                        agent_name: "test-agent".to_string(),
                        metadata: "test-metadata".to_string(),
                    }],
                    ..Default::default()
                }),
                ..claims.clone()
            },
            claims
        );
    }

    #[test]
    fn test_unverified_token() {
        let claims = Claims::from_unverified(TEST_TOKEN).expect("Failed to parse token");

        assert_eq!(claims.sub, "identity");
        assert_eq!(claims.name, "name");
        assert_eq!(claims.iss, TEST_API_KEY);
        assert_eq!(
            claims.room_config,
            Some(livekit_protocol::RoomConfiguration {
                agents: vec![livekit_protocol::RoomAgentDispatch {
                    agent_name: "test-agent".to_string(),
                    metadata: "test-metadata".to_string(),
                }],
                ..Default::default()
            })
        );

        let token = AccessToken::with_api_key(TEST_API_KEY, TEST_API_SECRET)
            .with_ttl(Duration::from_secs(60))
            .with_identity("test")
            .with_name("test")
            .with_grants(VideoGrants {
                room_join: true,
                room: "test-room".to_string(),
                ..Default::default()
            })
            .to_jwt()
            .unwrap();

        let claims = Claims::from_unverified(&token).expect("Failed to parse fresh token");
        assert_eq!(claims.sub, "test");
        assert_eq!(claims.name, "test");
        assert_eq!(claims.video.room, "test-room");
        assert!(claims.video.room_join);

        let parts: Vec<&str> = token.split('.').collect();
        let malformed_token = format!("{}.{}.wrongsignature", parts[0], parts[1]);

        let claims = Claims::from_unverified(&malformed_token)
            .expect("Failed to parse token with wrong signature");
        assert_eq!(claims.sub, "test");
        assert_eq!(claims.name, "test");
    }
}
