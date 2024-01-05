use anyhow::{anyhow, Result};
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::{
    borrow::Cow,
    ops::Add,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

static DEFAULT_TTL: Duration = Duration::from_secs(6 * 60 * 60); // 6 hours

#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimGrants<'a> {
    pub iss: Cow<'a, str>,
    pub sub: Option<Cow<'a, str>>,
    pub iat: u64,
    pub exp: u64,
    pub nbf: u64,
    pub jwtid: Option<Cow<'a, str>>,
    pub video: VideoGrant<'a>,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoGrant<'a> {
    pub room_create: Option<bool>,
    pub room_join: Option<bool>,
    pub room_list: Option<bool>,
    pub room_record: Option<bool>,
    pub room_admin: Option<bool>,
    pub room: Option<Cow<'a, str>>,
    pub can_publish: Option<bool>,
    pub can_subscribe: Option<bool>,
    pub can_publish_data: Option<bool>,
    pub hidden: Option<bool>,
    pub recorder: Option<bool>,
}

impl<'a> VideoGrant<'a> {
    pub fn to_admin(room: &'a str) -> Self {
        Self {
            room_admin: Some(true),
            room: Some(Cow::Borrowed(room)),
            ..Default::default()
        }
    }

    pub fn to_join(room: &'a str) -> Self {
        Self {
            room: Some(Cow::Borrowed(room)),
            room_join: Some(true),
            can_publish: Some(true),
            can_subscribe: Some(true),
            ..Default::default()
        }
    }

    pub fn for_guest(room: &'a str) -> Self {
        Self {
            room: Some(Cow::Borrowed(room)),
            room_join: Some(true),
            can_publish: Some(false),
            can_subscribe: Some(true),
            ..Default::default()
        }
    }
}

pub fn create(
    api_key: &str,
    secret_key: &str,
    identity: Option<&str>,
    video_grant: VideoGrant,
) -> Result<String> {
    if video_grant.room_join.is_some() && identity.is_none() {
        Err(anyhow!(
            "identity is required for room_join grant, but it is none"
        ))?;
    }

    let secret_key: Hmac<Sha256> = Hmac::new_from_slice(secret_key.as_bytes())?;

    let now = SystemTime::now();

    let claims = ClaimGrants {
        iss: Cow::Borrowed(api_key),
        sub: identity.map(Cow::Borrowed),
        iat: now.duration_since(UNIX_EPOCH).unwrap().as_secs(),
        exp: now
            .add(DEFAULT_TTL)
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        nbf: 0,
        jwtid: identity.map(Cow::Borrowed),
        video: video_grant,
    };
    Ok(claims.sign_with_key(&secret_key)?)
}

pub fn validate<'a>(token: &'a str, secret_key: &str) -> Result<ClaimGrants<'a>> {
    let secret_key: Hmac<Sha256> = Hmac::new_from_slice(secret_key.as_bytes())?;
    Ok(token.verify_with_key(&secret_key)?)
}
