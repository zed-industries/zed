/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::login::token::{LoginToken, LoginTokenError};
use crate::login::PROVIDER_NAME;
use aws_credential_types::Credentials;
use aws_runtime::fs_util::home_dir;
use aws_runtime::fs_util::Os;
use aws_smithy_json::deserialize::token::skip_value;
use aws_smithy_json::deserialize::{json_token_iter, Token};
use aws_smithy_json::serialize::JsonObjectWriter;
use aws_smithy_types::date_time::Format;
use aws_smithy_types::DateTime;
use aws_types::os_shim_internal::Env;
use aws_types::os_shim_internal::Fs;
use sha2::Digest;
use sha2::Sha256;
use std::path::Path;
use std::path::PathBuf;
use zeroize::Zeroizing;

const LOGIN_CACHE_DIRECTORY_ENV_VAR: &str = "AWS_LOGIN_CACHE_DIRECTORY";

/// Get the cache directory for Login (Sign-In) tokens
fn get_cache_dir(env: &Env) -> Result<PathBuf, LoginTokenError> {
    match env.get(LOGIN_CACHE_DIRECTORY_ENV_VAR).ok() {
        Some(cache_dir) => Ok(PathBuf::from(cache_dir)),
        None => {
            let home = home_dir(env, Os::real()).ok_or(LoginTokenError::NoHomeDirectory)?;
            Ok(PathBuf::from(home).join(".aws/login/cache"))
        }
    }
}

/// Determine the cached token path for a login session identifier.
///
/// The `cache_dir` is the directory used for caching AWS Sign-In tokens
fn cached_token_path(cache_dir: &Path, login_session: &str) -> PathBuf {
    let login_sesion_sha256 = hex::encode(Sha256::digest(login_session.trim().as_bytes()));
    let mut out = cache_dir.join(login_sesion_sha256);
    out.set_extension("json");
    out
}

/// Load the token for `identifier` from `~/.aws/login/cache/<hashofidentifier>.json`
///
/// The `identifier` is the `login_session` ARN to load the token for
pub(super) async fn load_cached_token(
    env: &Env,
    fs: &Fs,
    identifier: &str,
) -> Result<LoginToken, LoginTokenError> {
    let cache_dir = get_cache_dir(env)?;
    let path = cached_token_path(&cache_dir, identifier);
    let data =
        Zeroizing::new(
            fs.read_to_end(&path)
                .await
                .map_err(|source| LoginTokenError::IoError {
                    what: "read",
                    path,
                    source,
                })?,
        );
    parse_cached_token(&data)
}

/// Save the token for `identifier` to `~/.aws/login/cache/<hashofidentifier>.json`
///
/// The `identifier` is the `login_session` ARN to save the token for
pub(super) async fn save_cached_token(
    env: &Env,
    fs: &Fs,
    identifier: &str,
    token: &LoginToken,
) -> Result<(), LoginTokenError> {
    let cache_dir = get_cache_dir(env)?;
    let path = cached_token_path(&cache_dir, identifier);

    let expiration = DateTime::from(token.expires_at())
        .fmt(Format::DateTime)
        .map_err(|e| LoginTokenError::FailedToFormatDateTime { source: e.into() })?;

    let mut out = Zeroizing::new(String::new());
    let mut writer = JsonObjectWriter::new(&mut out);

    // Write accessToken object
    let mut access_token = writer.key("accessToken").start_object();
    access_token
        .key("accessKeyId")
        .string(token.access_token.access_key_id());
    access_token
        .key("secretAccessKey")
        .string(token.access_token.secret_access_key());
    access_token
        .key("sessionToken")
        .string(token.access_token.session_token().expect("session token"));
    access_token.key("accountId").string(
        token
            .access_token
            .account_id()
            .expect("account id")
            .as_str(),
    );
    access_token.key("expiresAt").string(&expiration);
    access_token.finish();

    if let Some(token_type) = &token.token_type {
        writer.key("tokenType").string(token_type.as_str());
    }

    writer
        .key("refreshToken")
        .string(token.refresh_token.as_str());
    if let Some(identity_token) = &token.identity_token {
        writer.key("idToken").string(identity_token);
    }
    writer.key("clientId").string(&token.client_id);
    writer.key("dpopKey").string(token.dpop_key.as_str());
    writer.finish();

    fs.write(&path, out.as_bytes())
        .await
        .map_err(|source| LoginTokenError::IoError {
            what: "write",
            path,
            source,
        })?;
    Ok(())
}

/// Parse SSO token JSON from input
fn parse_cached_token(cached_token_file_contents: &[u8]) -> Result<LoginToken, LoginTokenError> {
    use LoginTokenError as Error;

    /*
        {
          "accessToken": {
            "accessKeyId": "AKIAIOSFODNN7EXAMPLE",
            "secretAccessKey": "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY",
            "sessionToken": "AQoEXAMPLEH4aoAH0gNCAPyJxz4BlCFFxWNE1OPTgk5TthT+FvwqnKwRcOIfrRh3c/LTo6UDdyJwOOvEVPvLXCrrrUtdnniCEXAMPLE/IvU1dYUg2RVAJBanLiHb4IgRmpRV3zrkuWJOgQs8IZZaIv2BXIa2R4OlgkBN9bkUDNCJiBeb/AXlzBBko7b15fjrBs2+cTQtpZ3CYWFXG8C5zqx37wnOE49mRl/+OtkIKGO7fAE"
            "accountId": "012345678901",
            "expiresAt": "2025-09-14T04:05:45Z",
          },
          "tokenType": "aws_sigv4",
          "refreshToken": "<opaque string>",
          "idToken": "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiYWRtaW4iOnRydWV9.EkN-DOsnsuRjRO6BxXemmJDm3HbxrbRzXglbN2S4sOkopdU4IsDxTI8jO19W_A4K8ZPJijNLis4EZsHeY559a4DFOd50_OqgHs3UjpbCqhpuU5K_TGOj3pY-TJXSw",
          "clientId": "arn:aws:signin:::devtools/same-device",
          "dpopKey": "-----BEGIN EC PRIVATE KEY-----\nMHcCAQEEIFDZHUzOG1Pzq+6F0mjMlOSp1syN9LRPBuHMoCFXTcXhoAoGCCqGSM49\nAwEHoUQDQgAE9qhj+KtcdHj1kVgwxWWWw++tqoh7H7UHs7oXh8jBbgF47rrYGC+t\ndjiIaHK3dBvvdE7MGj5HsepzLm3Kj91bqA==\n-----END EC PRIVATE KEY-----\n"
        }
    */

    let mut access_key_id = None;
    let mut secret_access_key = None;
    let mut session_token = None;
    let mut account_id = None;
    let mut expires_at = None;
    let mut token_type = None;
    let mut refresh_token = None;
    let mut identity_token = None;
    let mut client_id = None;
    let mut dpop_key = None;

    let mut tokens = json_token_iter(cached_token_file_contents).peekable();
    if !matches!(tokens.next().transpose()?, Some(Token::StartObject { .. })) {
        return Err(Error::other(
            "expected a JSON document starting with `{`",
            None,
        ));
    }

    loop {
        match tokens.next().transpose()? {
            Some(Token::EndObject { .. }) => break,
            Some(Token::ObjectKey { key, .. }) => {
                let key = key.to_unescaped()?;

                if let Some(Ok(token)) = tokens.peek() {
                    if key.eq_ignore_ascii_case("accessToken") {
                        if let Token::StartObject { offset } = token {
                            let start = offset.0;
                            tokens.next(); // consume StartObject

                            loop {
                                match tokens.next().transpose()? {
                                    Some(Token::EndObject { offset }) => {
                                        let end = offset.0 + 1;
                                        let access_token_json = std::str::from_utf8(
                                            &cached_token_file_contents[start..end],
                                        )
                                        .map_err(|e| Error::JsonError(e.into()))?;

                                        let creds =
                                            crate::json_credentials::parse_json_credentials(
                                                access_token_json,
                                            )
                                            .map_err(|e| Error::JsonError(Box::new(e)))?;

                                        match creds {
                                            crate::json_credentials::JsonCredentials::RefreshableCredentials(c) => {
                                                access_key_id = Some(c.access_key_id.into_owned());
                                                secret_access_key = Some(c.secret_access_key.into_owned());
                                                session_token = Some(c.session_token.into_owned());
                                                account_id = c.account_id.map(|a| a.into_owned());
                                                expires_at = Some(c.expiration);
                                            }
                                            crate::json_credentials::JsonCredentials::Error { code, message } => {
                                                return Err(Error::JsonError(format!("error parsing `accessToken`: {code} - {message}").into()))
                                            }
                                        }
                                        break;
                                    }
                                    Some(Token::StartObject { .. }) => {
                                        return Err(Error::JsonError(
                                            "unexpected nested object in `accessToken`".into(),
                                        ));
                                    }
                                    None => {
                                        return Err(Error::JsonError(
                                            "unexpected end of JSON parsing `accessToken`".into(),
                                        ))
                                    }
                                    _ => {}
                                }
                            }
                            continue;
                        }
                    }

                    match (key.as_ref(), token) {
                        (k, Token::ValueString { value, .. })
                            if k.eq_ignore_ascii_case("tokenType") =>
                        {
                            token_type = Some(value.to_unescaped()?.into_owned());
                        }
                        (k, Token::ValueString { value, .. })
                            if k.eq_ignore_ascii_case("refreshToken") =>
                        {
                            refresh_token =
                                Some(Zeroizing::new(value.to_unescaped()?.into_owned()));
                        }
                        (k, Token::ValueString { value, .. })
                            if k.eq_ignore_ascii_case("idToken") =>
                        {
                            identity_token = Some(value.to_unescaped()?.into_owned());
                        }
                        (k, Token::ValueString { value, .. })
                            if k.eq_ignore_ascii_case("clientId") =>
                        {
                            client_id = Some(value.to_unescaped()?.into_owned());
                        }
                        (k, Token::ValueString { value, .. })
                            if k.eq_ignore_ascii_case("dpopKey") =>
                        {
                            dpop_key =
                                Some(Zeroizing::new(value.to_unescaped()?.replace("\\n", "\n")));
                        }
                        _ => {}
                    }
                }
                skip_value(&mut tokens)?;
            }
            other => {
                return Err(Error::other(
                    format!("expected object key, found: {other:?}"),
                    None,
                ));
            }
        }
    }

    let access_key_id = access_key_id.ok_or(Error::MissingField("accessKeyId"))?;
    let secret_access_key = secret_access_key.ok_or(Error::MissingField("secretAccessKey"))?;
    let session_token = session_token.ok_or(Error::MissingField("sessionToken"))?;
    let account_id = account_id.ok_or(Error::MissingField("accountId"))?;
    let client_id = client_id.ok_or(Error::MissingField("clientId"))?;
    let dpop_key = dpop_key.ok_or(Error::MissingField("dpopKey"))?;
    let refresh_token = refresh_token.ok_or(Error::MissingField("refreshToken"))?;
    let expires_at = expires_at.ok_or(Error::MissingField("expiresAt"))?;

    let credentials = Credentials::builder()
        .access_key_id(access_key_id)
        .secret_access_key(secret_access_key)
        .session_token(session_token)
        .account_id(account_id)
        .provider_name(PROVIDER_NAME)
        .expiry(expires_at)
        .build();

    Ok(LoginToken {
        access_token: credentials,
        token_type,
        identity_token,
        refresh_token,
        client_id,
        dpop_key,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    const TEST_CACHE_DIR: &str = "/home/someuser/.aws/login/cache";

    #[cfg_attr(windows, ignore)]
    #[test]
    fn determine_correct_cache_filenames() {
        let cache_dir = PathBuf::from(TEST_CACHE_DIR);
        assert_eq!(
            "/home/someuser/.aws/login/cache/36db1d138ff460920374e4c3d8e01f53f9f73537e89c88d639f68393df0e2726.json",
            cached_token_path(&cache_dir, "arn:aws:iam::0123456789012:user/Admin").as_os_str()
        );
        assert_eq!(
            "/home/someuser/.aws/login/cache/36db1d138ff460920374e4c3d8e01f53f9f73537e89c88d639f68393df0e2726.json",
            cached_token_path(&cache_dir, "  arn:aws:iam::0123456789012:user/Admin  ").as_os_str()
        );
        assert_eq!(
            "/home/someuser/.aws/login/cache/d19c78f768c6a12874de5f41d7f22cbb834ba205704102da0db20d8496efecb5.json",
            cached_token_path(&cache_dir, "arn:aws:iam::000000000000:user/PowerUser").as_os_str()
        );
    }

    #[test]
    fn parse_valid_token() {
        let file_contents = r#"{
            "accessToken": {
                "accessKeyId": "AKIAIOSFODNN7EXAMPLE",
                "secretAccessKey": "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY",
                "sessionToken": "session-token",
                "accountId": "012345678901",
                "expiresAt": "2021-12-25T21:30:00Z"
            },
            "tokenType": "aws_sigv4",
            "refreshToken": "refresh-token-value",
            "idToken": "identity-token-value",
            "clientId": "arn:aws:signin:::devtools/same-device",
            "dpopKey": "-----BEGIN EC PRIVATE KEY-----\ntest\n-----END EC PRIVATE KEY-----\n"
        }"#;
        let token = parse_cached_token(file_contents.as_bytes()).expect("success");
        assert_eq!("AKIAIOSFODNN7EXAMPLE", token.access_token.access_key_id());
        assert_eq!(
            "012345678901",
            token.access_token.account_id().unwrap().as_str()
        );
        assert_eq!(
            std::time::SystemTime::UNIX_EPOCH + Duration::from_secs(1640467800),
            token.access_token.expiry().unwrap()
        );
        assert_eq!("refresh-token-value", token.refresh_token.as_str());
        assert_eq!(
            Some("identity-token-value"),
            token.identity_token.as_deref()
        );

        assert_eq!(
            "arn:aws:signin:::devtools/same-device",
            token.client_id.as_str()
        );
        assert_eq!(
            "-----BEGIN EC PRIVATE KEY-----\ntest\n-----END EC PRIVATE KEY-----\n",
            token.dpop_key.as_str()
        );
    }

    #[test]
    fn parse_missing_fields() {
        // Missing accessToken
        let token = br#"{
            "tokenType": "aws_sigv4",
            "clientId": "client",
            "dpopKey": "key"
        }"#;
        let err = parse_cached_token(token).expect_err("missing accessToken");
        assert!(
            matches!(err, LoginTokenError::MissingField("accessKeyId")),
            "incorrect error: {:?}",
            err
        );

        // Missing clientId
        let token = br#"{
            "accessToken": {
                "accessKeyId": "AKID",
                "secretAccessKey": "SECRET",
                "sessionToken": "TOKEN",
                "accountId": "123456789012",
                "expiresAt": "2021-12-25T21:30:00Z"
            },
            "tokenType": "aws_sigv4",
            "dpopKey": "key"
        }"#;
        let err = parse_cached_token(token).expect_err("missing clientId");
        assert!(
            matches!(err, LoginTokenError::MissingField("clientId")),
            "incorrect error: {:?}",
            err
        );

        // Missing dpopKey
        let token = br#"{
            "accessToken": {
                "accessKeyId": "AKID",
                "secretAccessKey": "SECRET",
                "sessionToken": "TOKEN",
                "accountId": "123456789012",
                "expiresAt": "2021-12-25T21:30:00Z"
            },
            "tokenType": "aws_sigv4",
            "clientId": "client"
        }"#;
        let err = parse_cached_token(token).expect_err("missing dpopKey");
        assert!(
            matches!(err, LoginTokenError::MissingField("dpopKey")),
            "incorrect error: {:?}",
            err
        );
    }

    #[tokio::test]
    async fn load_token_from_cache() {
        use std::collections::HashMap;
        let token_json = r#"{
            "accessToken": {
                "accessKeyId": "AKIAIOSFODNN7EXAMPLE",
                "secretAccessKey": "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY",
                "sessionToken": "session-token",
                "accountId": "012345678901",
                "expiresAt": "2021-12-25T21:30:00Z"
            },
            "tokenType": "aws_sigv4",
            "refreshToken": "refresh-token-value",
            "idToken": "identity-token-value",
            "clientId": "arn:aws:signin:::devtools/same-device",
            "dpopKey": "-----BEGIN EC PRIVATE KEY-----\ntest\n-----END EC PRIVATE KEY-----\n"
        }"#;

        let env = Env::from_slice(&[("HOME", "/home/user")]);
        let fs = Fs::from_map(HashMap::from([(
            "/home/user/.aws/login/cache/36db1d138ff460920374e4c3d8e01f53f9f73537e89c88d639f68393df0e2726.json".to_string(),
            token_json.as_bytes().to_vec(),
        )]));

        let token = load_cached_token(&env, &fs, "arn:aws:iam::0123456789012:user/Admin")
            .await
            .expect("success");

        assert_eq!("AKIAIOSFODNN7EXAMPLE", token.access_token.access_key_id());
        assert_eq!(
            "012345678901",
            token.access_token.account_id().unwrap().as_str()
        );
        assert_eq!(
            "arn:aws:signin:::devtools/same-device",
            token.client_id.as_str()
        );
    }

    #[tokio::test]
    async fn error_on_missing_file() {
        let err = load_cached_token(
            &Env::from_slice(&[("HOME", "/home")]),
            &Fs::from_slice(&[]),
            "arn:aws:iam::123456789012:user/test",
        )
        .await
        .expect_err("should fail, file is missing");
        assert!(
            matches!(err, LoginTokenError::IoError { .. }),
            "should be io error, got {:?}",
            err
        );
    }
}
