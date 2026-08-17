/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */
use aws_smithy_json::deserialize::token::skip_value;
use aws_smithy_json::deserialize::{json_token_iter, EscapeError, Token};
use aws_smithy_types::date_time::Format;
use aws_smithy_types::DateTime;
use std::borrow::Cow;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::time::SystemTime;

#[derive(Debug)]
pub(crate) enum InvalidJsonCredentials {
    /// The response did not contain valid JSON
    JsonError(Box<dyn Error + Send + Sync>),
    /// The response was missing a required field
    MissingField(&'static str),

    /// A field was invalid
    InvalidField {
        field: &'static str,
        err: Box<dyn Error + Send + Sync>,
    },

    /// Another unhandled error occurred
    Other(Cow<'static, str>),
}

impl From<EscapeError> for InvalidJsonCredentials {
    fn from(err: EscapeError) -> Self {
        InvalidJsonCredentials::JsonError(err.into())
    }
}

impl From<aws_smithy_json::deserialize::error::DeserializeError> for InvalidJsonCredentials {
    fn from(err: aws_smithy_json::deserialize::error::DeserializeError) -> Self {
        InvalidJsonCredentials::JsonError(err.into())
    }
}

impl Display for InvalidJsonCredentials {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InvalidJsonCredentials::JsonError(json) => {
                write!(f, "invalid JSON in response: {json}")
            }
            InvalidJsonCredentials::MissingField(field) => {
                write!(f, "Expected field `{field}` in response but it was missing",)
            }
            InvalidJsonCredentials::Other(msg) => write!(f, "{msg}"),
            InvalidJsonCredentials::InvalidField { field, err } => {
                write!(f, "Invalid field in response: `{field}`. {err}")
            }
        }
    }
}

impl Error for InvalidJsonCredentials {}

#[derive(PartialEq, Eq)]
pub(crate) struct RefreshableCredentials<'a> {
    pub(crate) access_key_id: Cow<'a, str>,
    pub(crate) secret_access_key: Cow<'a, str>,
    pub(crate) session_token: Cow<'a, str>,
    pub(crate) account_id: Option<Cow<'a, str>>,
    pub(crate) expiration: SystemTime,
}

impl fmt::Debug for RefreshableCredentials<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut debug = f.debug_struct("RefreshableCredentials");
        debug
            .field("access_key_id", &self.access_key_id)
            .field("secret_access_key", &"** redacted **")
            .field("session_token", &"** redacted **");
        if let Some(account_id) = &self.account_id {
            debug.field("account_id", account_id);
        }
        debug.field("expiration", &self.expiration).finish()
    }
}

#[non_exhaustive]
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum JsonCredentials<'a> {
    RefreshableCredentials(RefreshableCredentials<'a>),
    Error {
        code: Cow<'a, str>,
        message: Cow<'a, str>,
    }, // TODO(https://github.com/awslabs/aws-sdk-rust/issues/340): Add support for static credentials:
       //  {
       //    "AccessKeyId" : "MUA...",
       //    "SecretAccessKey" : "/7PC5om...."
       //  }

       // TODO(https://github.com/awslabs/aws-sdk-rust/issues/340): Add support for Assume role credentials:
       //   {
       //     // fields to construct STS client:
       //     "Region": "sts-region-name",
       //     "AccessKeyId" : "MUA...",
       //     "Expiration" : "2016-02-25T06:03:31Z", // optional
       //     "SecretAccessKey" : "/7PC5om....",
       //     "Token" : "AQoDY....=", // optional
       //     // fields controlling the STS role:
       //     "RoleArn": "...", // required
       //     "RoleSessionName": "...", // required
       //     // and also: DurationSeconds, ExternalId, SerialNumber, TokenCode, Policy
       //     ...
       //   }
}

/// Deserialize an IMDS response from a string
///
/// There are two levels of error here: the top level distinguishes between a successfully parsed
/// response from the credential provider vs. something invalid / unexpected. The inner error
/// distinguishes between a successful response that contains credentials vs. an error with a code and
/// error message.
///
/// Keys are case insensitive.
pub(crate) fn parse_json_credentials(
    credentials_response: &str,
) -> Result<JsonCredentials<'_>, InvalidJsonCredentials> {
    let mut code = None;
    let mut access_key_id = None;
    let mut secret_access_key = None;
    let mut session_token = None;
    let mut account_id = None;
    let mut expiration = None;
    let mut message = None;
    json_parse_loop(credentials_response.as_bytes(), |key, value| {
        match (key, value) {
            /*
             "Code": "Success",
             "Type": "AWS-HMAC",
             "AccessKeyId" : "accessKey",
             "SecretAccessKey" : "secret",
             "Token | SessionToken" : "token",
             "AccountId" : "111122223333",
             "Expiration | ExpiresAt" : "....",
             "LastUpdated" : "2009-11-23T00:00:00Z"
            */
            (key, Token::ValueString { value, .. }) if key.eq_ignore_ascii_case("Code") => {
                code = Some(value.to_unescaped()?);
            }
            (key, Token::ValueString { value, .. }) if key.eq_ignore_ascii_case("AccessKeyId") => {
                access_key_id = Some(value.to_unescaped()?);
            }
            (key, Token::ValueString { value, .. })
                if key.eq_ignore_ascii_case("SecretAccessKey") =>
            {
                secret_access_key = Some(value.to_unescaped()?);
            }
            (key, Token::ValueString { value, .. })
                if key.eq_ignore_ascii_case("Token")
                    || key.eq_ignore_ascii_case("SessionToken") =>
            {
                session_token = Some(value.to_unescaped()?);
            }
            (key, Token::ValueString { value, .. }) if key.eq_ignore_ascii_case("AccountId") => {
                account_id = Some(value.to_unescaped()?);
            }
            (key, Token::ValueString { value, .. })
                if key.eq_ignore_ascii_case("Expiration")
                    || key.eq_ignore_ascii_case("ExpiresAt") =>
            {
                expiration = Some(value.to_unescaped()?);
            }

            // Error case handling: message will be set
            (key, Token::ValueString { value, .. }) if key.eq_ignore_ascii_case("Message") => {
                message = Some(value.to_unescaped()?);
            }
            _ => {}
        };
        Ok(())
    })?;
    match code {
        // IMDS does not appear to reply with a `Code` missing, but documentation indicates it
        // may be possible
        None | Some(Cow::Borrowed("Success")) => {
            let access_key_id =
                access_key_id.ok_or(InvalidJsonCredentials::MissingField("AccessKeyId"))?;
            let secret_access_key =
                secret_access_key.ok_or(InvalidJsonCredentials::MissingField("SecretAccessKey"))?;
            let session_token =
                session_token.ok_or(InvalidJsonCredentials::MissingField("Token"))?;
            let expiration =
                expiration.ok_or(InvalidJsonCredentials::MissingField("Expiration"))?;
            let expiration = SystemTime::try_from(
                DateTime::from_str(expiration.as_ref(), Format::DateTime).map_err(|err| {
                    InvalidJsonCredentials::InvalidField {
                        field: "Expiration",
                        err: err.into(),
                    }
                })?,
            )
            .map_err(|_| {
                InvalidJsonCredentials::Other(
                    "credential expiration time cannot be represented by a SystemTime".into(),
                )
            })?;
            Ok(JsonCredentials::RefreshableCredentials(
                RefreshableCredentials {
                    access_key_id,
                    secret_access_key,
                    session_token,
                    account_id,
                    expiration,
                },
            ))
        }
        Some(other) => Ok(JsonCredentials::Error {
            code: other,
            message: message.unwrap_or_else(|| "no message".into()),
        }),
    }
}

pub(crate) fn json_parse_loop<'a>(
    input: &'a [u8],
    mut f: impl FnMut(Cow<'a, str>, &Token<'a>) -> Result<(), InvalidJsonCredentials>,
) -> Result<(), InvalidJsonCredentials> {
    let mut tokens = json_token_iter(input).peekable();
    if !matches!(tokens.next().transpose()?, Some(Token::StartObject { .. })) {
        return Err(InvalidJsonCredentials::JsonError(
            "expected a JSON document starting with `{`".into(),
        ));
    }
    loop {
        match tokens.next().transpose()? {
            Some(Token::EndObject { .. }) => break,
            Some(Token::ObjectKey { key, .. }) => {
                if let Some(Ok(token)) = tokens.peek() {
                    let key = key.to_unescaped()?;
                    f(key, token)?
                }
                skip_value(&mut tokens)?;
            }
            other => {
                return Err(InvalidJsonCredentials::Other(
                    format!("expected object key, found: {other:?}").into(),
                ));
            }
        }
    }
    if tokens.next().is_some() {
        return Err(InvalidJsonCredentials::Other(
            "found more JSON tokens after completing parsing".into(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::json_credentials::{
        parse_json_credentials, InvalidJsonCredentials, JsonCredentials, RefreshableCredentials,
    };
    use std::time::{Duration, UNIX_EPOCH};

    #[test]
    fn json_credentials_success_response() {
        let response = r#"
        {
          "Code" : "Success",
          "LastUpdated" : "2021-09-17T20:57:08Z",
          "Type" : "AWS-HMAC",
          "AccessKeyId" : "ASIARTEST",
          "SecretAccessKey" : "xjtest",
          "Token" : "IQote///test",
          "AccountID" : "111122223333",
          "Expiration" : "2021-09-18T03:31:56Z"
        }"#;
        let parsed = parse_json_credentials(response).expect("valid JSON");
        assert_eq!(
            parsed,
            JsonCredentials::RefreshableCredentials(RefreshableCredentials {
                access_key_id: "ASIARTEST".into(),
                secret_access_key: "xjtest".into(),
                session_token: "IQote///test".into(),
                account_id: Some("111122223333".into()),
                expiration: UNIX_EPOCH + Duration::from_secs(1631935916),
            })
        )
    }

    #[test]
    fn json_credentials_invalid_json() {
        let error = parse_json_credentials("404: not found").expect_err("no json");
        match error {
            InvalidJsonCredentials::JsonError(_) => {} // ok.
            err => panic!("incorrect error: {:?}", err),
        }
    }

    #[test]
    fn json_credentials_not_json_object() {
        let error = parse_json_credentials("[1,2,3]").expect_err("no json");
        match error {
            InvalidJsonCredentials::JsonError(_) => {} // ok.
            _ => panic!("incorrect error"),
        }
    }

    #[test]
    fn json_credentials_missing_code() {
        let resp = r#"{
            "LastUpdated" : "2021-09-17T20:57:08Z",
            "Type" : "AWS-HMAC",
            "AccessKeyId" : "ASIARTEST",
            "SecretAccessKey" : "xjtest",
            "Token" : "IQote///test",
            "AccountID" : "111122223333",
            "Expiration" : "2021-09-18T03:31:56Z"
        }"#;
        let parsed = parse_json_credentials(resp).expect("code not required");
        assert_eq!(
            parsed,
            JsonCredentials::RefreshableCredentials(RefreshableCredentials {
                access_key_id: "ASIARTEST".into(),
                secret_access_key: "xjtest".into(),
                session_token: "IQote///test".into(),
                account_id: Some("111122223333".into()),
                expiration: UNIX_EPOCH + Duration::from_secs(1631935916),
            })
        )
    }

    #[test]
    fn json_credentials_required_session_token() {
        let resp = r#"{
            "LastUpdated" : "2021-09-17T20:57:08Z",
            "Type" : "AWS-HMAC",
            "AccessKeyId" : "ASIARTEST",
            "SecretAccessKey" : "xjtest",
            "AccountID" : "111122223333",
            "Expiration" : "2021-09-18T03:31:56Z"
        }"#;
        let parsed = parse_json_credentials(resp).expect_err("token missing");
        assert_eq!(
            format!("{}", parsed),
            "Expected field `Token` in response but it was missing"
        );
    }

    #[test]
    fn json_credentials_missing_akid() {
        let resp = r#"{
            "Code": "Success",
            "LastUpdated" : "2021-09-17T20:57:08Z",
            "Type" : "AWS-HMAC",
            "SecretAccessKey" : "xjtest",
            "Token" : "IQote///test",
            "AccountID" : "111122223333",
            "Expiration" : "2021-09-18T03:31:56Z"
        }"#;
        match parse_json_credentials(resp).expect_err("no code") {
            InvalidJsonCredentials::MissingField("AccessKeyId") => {} // ok
            resp => panic!("incorrect json_credentials response: {:?}", resp),
        }
    }

    #[test]
    fn json_credentials_error_response() {
        let response = r#"{
          "Code" : "AssumeRoleUnauthorizedAccess",
          "Message" : "EC2 cannot assume the role integration-test.",
          "LastUpdated" : "2021-09-17T20:46:56Z"
        }"#;
        let parsed = parse_json_credentials(response).expect("valid JSON");
        assert_eq!(
            parsed,
            JsonCredentials::Error {
                code: "AssumeRoleUnauthorizedAccess".into(),
                message: "EC2 cannot assume the role integration-test.".into(),
            }
        );
    }

    /// Validate the specific JSON response format sent by ECS
    #[test]
    fn json_credentials_ecs() {
        // identical, but extra `RoleArn` field is present
        let response = r#"{
            "RoleArn":"arn:aws:iam::123456789:role/ecs-task-role",
            "AccessKeyId":"ASIARTEST",
            "SecretAccessKey":"SECRETTEST",
            "Token":"tokenEaCXVzLXdlc3QtMiJGMEQCIHt47W18eF4dYfSlmKGiwuJnqmIS3LMXNYfODBCEhcnaAiAnuhGOpcdIDxin4QFzhtgaCR2MpcVqR8NFJdMgOt0/xyrnAwhhEAEaDDEzNDA5NTA2NTg1NiIM9M9GT+c5UfV/8r7PKsQDUa9xE9Eprz5N+jgxbFSD2aJR2iyXCcP9Q1cOh4fdZhyw2WNmq9XnIa2tkzrreiQ5R2t+kzergJHO1KRZPfesarfJ879aWJCSocsEKh7xXwwzTsVXrNo5eWkpwTh64q+Ksz15eoaBhtrvnGvPx6SmXv7SToi/DTHFafJlT/T9jITACZvZXSE9zfLka26Rna3rI4g0ugowha//j1f/c1XuKloqshpZvMKc561om9Y5fqBv1fRiS2KhetGTcmz3wUqNQAk8Dq9oINS7cCtdIO0atqCK69UaKeJ9uKY8mzY9dFWw2IrkpOoXmA9r955iU0NOz/95jVJiPZ/8aE8vb0t67gQfzBUCfky+mGSGWAfPRXQlFa5AEulCTHPd7IcTVCtasG033oKEKgB8QnTxvM2LaPlwaaHo7MHGYXeUKbn9NRKd8m1ShwmAlr4oKp1vQp6cPHDTsdTfPTzh/ZAjUPs+ljQbAwqXbPQdUUPpOk0vltY8k6Im9EA0pf80iUNoqrixpmPsR2hzI/ybUwdh+QhvCSBx+J8KHqF6X92u4qAVYIxLy/LGZKT9YC6Kr9Gywn+Ro+EK/xl3axHPzNpbjRDJnbW3HrMw5LmmiwY6pgGWgmD6IOq4QYUtu1uhaLQZyoI5o5PWn+d3kqqxifu8D0ykldB3lQGdlJ2rjKJjCdx8fce1SoXao9cc4hiwn39hUPuTqzVwv2zbzCKmNggIpXP6gqyRtUCakf6tI7ZwqTb2S8KF3t4ElIP8i4cPdNoI0JHSC+sT4LDPpUcX1CjGxfvo55mBHJedW3LXve8TRj4UckFXT1gLuTnzqPMrC5AHz4TAt+uv",
            "AccountID" : "111122223333",
            "Expiration" : "2009-02-13T23:31:30Z"
        }"#;
        let parsed = parse_json_credentials(response).expect("valid JSON");
        use std::borrow::Cow;
        assert!(
            matches!(
                &parsed,
                JsonCredentials::RefreshableCredentials(RefreshableCredentials{
                    access_key_id: Cow::Borrowed("ASIARTEST"),
                    secret_access_key: Cow::Borrowed("SECRETTEST"),
                    session_token,
                    account_id: Some(Cow::Borrowed("111122223333")),
                    expiration
                }) if session_token.starts_with("token") && *expiration == UNIX_EPOCH + Duration::from_secs(1234567890)
            ),
            "{:?}",
            parsed
        );
    }

    #[test]
    fn case_insensitive_code_parsing() {
        let response = r#"{
          "code" : "AssumeRoleUnauthorizedAccess",
          "message" : "EC2 cannot assume the role integration-test."
        }"#;
        let parsed = parse_json_credentials(response).expect("valid JSON");
        assert_eq!(
            parsed,
            JsonCredentials::Error {
                code: "AssumeRoleUnauthorizedAccess".into(),
                message: "EC2 cannot assume the role integration-test.".into(),
            }
        );
    }
}
