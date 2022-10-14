use anyhow::Result;
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use serde::Serialize;
use sha2::Sha256;
use std::{
    ops::Add,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

static DEFAULT_TTL: Duration = Duration::from_secs(6 * 60 * 60); // 6 hours

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
struct ClaimGrants<'a> {
    iss: &'a str,
    sub: &'a str,
    iat: u64,
    exp: u64,
    nbf: u64,
    jwtid: &'a str,
    video: VideoGrant<'a>,
}

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
struct VideoGrant<'a> {
    room_create: Option<bool>,
    room_join: Option<bool>,
    room_list: Option<bool>,
    room_record: Option<bool>,
    room_admin: Option<bool>,
    room: Option<&'a str>,
    can_publish: Option<bool>,
    can_subscribe: Option<bool>,
    can_publish_data: Option<bool>,
    hidden: Option<bool>,
    recorder: Option<bool>,
}

pub fn create(
    api_key: &str,
    secret_key: &str,
    room_name: &str,
    participant_name: &str,
) -> Result<String> {
    let secret_key: Hmac<Sha256> = Hmac::new_from_slice(secret_key.as_bytes())?;

    let now = SystemTime::now();

    let claims = ClaimGrants {
        iss: api_key,
        sub: participant_name,
        iat: now.duration_since(UNIX_EPOCH).unwrap().as_secs(),
        exp: now
            .add(DEFAULT_TTL)
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        nbf: 0,
        jwtid: participant_name,
        video: VideoGrant {
            room: Some(room_name),
            room_join: Some(true),
            can_publish: Some(true),
            can_subscribe: Some(true),
            ..Default::default()
        },
    };
    Ok(claims.sign_with_key(&secret_key)?)
}
