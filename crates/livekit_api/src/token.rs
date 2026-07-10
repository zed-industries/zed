use anyhow::{Context as _, Result};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

const DEFAULT_TTL: Duration = Duration::from_secs(6 * 60 * 60); // 6 hours

pub trait UnixTimestampSource: Send + Sync {
    fn unix_timestamp(&self) -> Result<u64>;
}

pub struct SystemUnixTimestampSource;

impl UnixTimestampSource for SystemUnixTimestampSource {
    fn unix_timestamp(&self) -> Result<u64> {
        Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs())
    }
}

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
    create_with_timestamp_source(
        api_key,
        secret_key,
        identity,
        video_grant,
        &SystemUnixTimestampSource,
    )
}

pub fn create_with_timestamp_source(
    api_key: &str,
    secret_key: &str,
    identity: Option<&str>,
    video_grant: VideoGrant,
    timestamp_source: &dyn UnixTimestampSource,
) -> Result<String> {
    let issued_at = timestamp_source.unix_timestamp()?;
    create_with_issued_at(api_key, secret_key, identity, video_grant, issued_at)
}

fn create_with_issued_at(
    api_key: &str,
    secret_key: &str,
    identity: Option<&str>,
    video_grant: VideoGrant,
    issued_at: u64,
) -> Result<String> {
    let room_join = video_grant.room_join.unwrap_or(false);
    if room_join && identity.is_none() {
        anyhow::bail!("identity is required for room_join grant, but it is none");
    }

    let expires_at = issued_at
        .checked_add(DEFAULT_TTL.as_secs())
        .context("token expiration overflow")?;
    let not_before = if room_join {
        // LiveKit Cloud applies participant revocations by comparing the
        // revocation timestamp to room-join token `nbf`.
        issued_at
    } else {
        0
    };

    let claims = ClaimGrants {
        iss: Cow::Borrowed(api_key),
        sub: identity.map(Cow::Borrowed),
        iat: issued_at,
        exp: expires_at,
        nbf: not_before,
        jwtid: identity.map(Cow::Borrowed),
        video: video_grant,
    };
    Ok(jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret_key.as_ref()),
    )?)
}

pub fn validate<'a>(token: &'a str, secret_key: &str) -> Result<ClaimGrants<'a>> {
    let token = jsonwebtoken::decode(
        token,
        &DecodingKey::from_secret(secret_key.as_ref()),
        &Validation::default(),
    )?;

    Ok(token.claims)
}

#[cfg(any(test, feature = "test-support"))]
pub fn validate_with_timestamp_source<'a>(
    token: &'a str,
    secret_key: &str,
    timestamp_source: &dyn UnixTimestampSource,
) -> Result<ClaimGrants<'a>> {
    let mut validation = Validation::default();
    validation.validate_exp = false;
    validation.validate_nbf = false;
    let token: jsonwebtoken::TokenData<ClaimGrants<'_>> = jsonwebtoken::decode(
        token,
        &DecodingKey::from_secret(secret_key.as_ref()),
        &validation,
    )?;
    let claims = token.claims;
    let timestamp = timestamp_source.unix_timestamp()?;

    anyhow::ensure!(claims.nbf <= timestamp, "token is not yet valid");
    anyhow::ensure!(claims.exp > timestamp, "token has expired");

    Ok(claims)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Client as _, LiveKitClient};
    use std::sync::Arc;

    const ISSUED_AT: u64 = 1_234_567;

    struct FixedUnixTimestampSource(u64);

    impl UnixTimestampSource for FixedUnixTimestampSource {
        fn unix_timestamp(&self) -> Result<u64> {
            Ok(self.0)
        }
    }

    #[test]
    fn token_not_before_matches_issue_time() -> Result<()> {
        let token = create_with_timestamp_source(
            "api-key",
            "secret-key",
            Some("participant"),
            VideoGrant::to_join("room"),
            &FixedUnixTimestampSource(ISSUED_AT),
        )?;

        assert_claims_timestamp(&token, ISSUED_AT, ISSUED_AT)?;
        Ok(())
    }

    #[test]
    fn room_token_not_before_matches_issue_time() -> Result<()> {
        let client = LiveKitClient::new_with_timestamp_source(
            "http://livekit.test".into(),
            "api-key".into(),
            "secret-key".into(),
            Arc::new(FixedUnixTimestampSource(ISSUED_AT)),
        );
        let token = client.room_token("room", "participant")?;

        let claims = assert_claims_timestamp(&token, ISSUED_AT, ISSUED_AT)?;
        assert_eq!(claims.video.room_join, Some(true));
        assert_eq!(claims.video.can_publish, Some(true));
        assert_eq!(claims.video.can_subscribe, Some(true));

        Ok(())
    }

    #[test]
    fn guest_token_not_before_matches_issue_time() -> Result<()> {
        let client = LiveKitClient::new_with_timestamp_source(
            "http://livekit.test".into(),
            "api-key".into(),
            "secret-key".into(),
            Arc::new(FixedUnixTimestampSource(ISSUED_AT)),
        );
        let token = client.guest_token("room", "participant")?;

        let claims = assert_claims_timestamp(&token, ISSUED_AT, ISSUED_AT)?;
        assert_eq!(claims.video.room_join, Some(true));
        assert_eq!(claims.video.can_publish, Some(false));
        assert_eq!(claims.video.can_subscribe, Some(true));

        Ok(())
    }

    #[test]
    fn admin_token_not_before_remains_unset() -> Result<()> {
        let token = create_with_timestamp_source(
            "api-key",
            "secret-key",
            None,
            VideoGrant::to_admin("room"),
            &FixedUnixTimestampSource(ISSUED_AT),
        )?;

        let claims = assert_claims_timestamp(&token, ISSUED_AT, 0)?;
        assert_eq!(claims.video.room_admin, Some(true));

        Ok(())
    }

    fn assert_claims_timestamp(
        token: &str,
        issued_at: u64,
        expected_not_before: u64,
    ) -> Result<ClaimGrants<'_>> {
        let claims = validate_with_timestamp_source(
            token,
            "secret-key",
            &FixedUnixTimestampSource(issued_at),
        )?;

        assert_eq!(claims.iat, issued_at);
        assert_eq!(claims.nbf, expected_not_before);
        assert_eq!(claims.exp, issued_at + DEFAULT_TTL.as_secs());

        Ok(claims)
    }
}
