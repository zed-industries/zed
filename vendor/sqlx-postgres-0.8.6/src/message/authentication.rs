use std::str::from_utf8;

use memchr::memchr;
use sqlx_core::bytes::{Buf, Bytes};

use crate::error::Error;
use crate::io::ProtocolDecode;

use crate::message::{BackendMessage, BackendMessageFormat};
use base64::prelude::{Engine as _, BASE64_STANDARD};
// On startup, the server sends an appropriate authentication request message,
// to which the frontend must reply with an appropriate authentication
// response message (such as a password).

// For all authentication methods except GSSAPI, SSPI and SASL, there is at
// most one request and one response. In some methods, no response at all is
// needed from the frontend, and so no authentication request occurs.

// For GSSAPI, SSPI and SASL, multiple exchanges of packets may
// be needed to complete the authentication.

// <https://www.postgresql.org/docs/devel/protocol-flow.html#id-1.10.5.7.3>
// <https://www.postgresql.org/docs/devel/protocol-message-formats.html>

#[derive(Debug)]
pub enum Authentication {
    /// The authentication exchange is successfully completed.
    Ok,

    /// The frontend must now send a [PasswordMessage] containing the
    /// password in clear-text form.
    CleartextPassword,

    /// The frontend must now send a [PasswordMessage] containing the
    /// password (with user name) encrypted via MD5, then encrypted
    /// again using the 4-byte random salt.
    Md5Password(AuthenticationMd5Password),

    /// The frontend must now initiate a SASL negotiation,
    /// using one of the SASL mechanisms listed in the message.
    ///
    /// The frontend will send a [SaslInitialResponse] with the name
    /// of the selected mechanism, and the first part of the SASL
    /// data stream in response to this.
    ///
    /// If further messages are needed, the server will
    /// respond with [Authentication::SaslContinue].
    Sasl(AuthenticationSasl),

    /// This message contains challenge data from the previous step of SASL negotiation.
    ///
    /// The frontend must respond with a [SaslResponse] message.
    SaslContinue(AuthenticationSaslContinue),

    /// SASL authentication has completed with additional mechanism-specific
    /// data for the client.
    ///
    /// The server will next send [Authentication::Ok] to
    /// indicate successful authentication.
    SaslFinal(AuthenticationSaslFinal),
}

impl BackendMessage for Authentication {
    const FORMAT: BackendMessageFormat = BackendMessageFormat::Authentication;

    fn decode_body(mut buf: Bytes) -> Result<Self, Error> {
        Ok(match buf.get_u32() {
            0 => Authentication::Ok,

            3 => Authentication::CleartextPassword,

            5 => {
                let mut salt = [0; 4];
                buf.copy_to_slice(&mut salt);

                Authentication::Md5Password(AuthenticationMd5Password { salt })
            }

            10 => Authentication::Sasl(AuthenticationSasl(buf)),
            11 => Authentication::SaslContinue(AuthenticationSaslContinue::decode(buf)?),
            12 => Authentication::SaslFinal(AuthenticationSaslFinal::decode(buf)?),

            ty => {
                return Err(err_protocol!("unknown authentication method: {}", ty));
            }
        })
    }
}

/// Body of [Authentication::Md5Password].
#[derive(Debug)]
pub struct AuthenticationMd5Password {
    pub salt: [u8; 4],
}

/// Body of [Authentication::Sasl].
#[derive(Debug)]
pub struct AuthenticationSasl(Bytes);

impl AuthenticationSasl {
    #[inline]
    pub fn mechanisms(&self) -> SaslMechanisms<'_> {
        SaslMechanisms(&self.0)
    }
}

/// An iterator over the SASL authentication mechanisms provided by the server.
pub struct SaslMechanisms<'a>(&'a [u8]);

impl<'a> Iterator for SaslMechanisms<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.0.is_empty() && self.0[0] == b'\0' {
            return None;
        }

        let mechanism = memchr(b'\0', self.0).and_then(|nul| from_utf8(&self.0[..nul]).ok())?;

        self.0 = &self.0[(mechanism.len() + 1)..];

        Some(mechanism)
    }
}

#[derive(Debug)]
pub struct AuthenticationSaslContinue {
    pub salt: Vec<u8>,
    pub iterations: u32,
    pub nonce: String,
    pub message: String,
}

impl ProtocolDecode<'_> for AuthenticationSaslContinue {
    fn decode_with(buf: Bytes, _: ()) -> Result<Self, Error> {
        let mut iterations: u32 = 4096;
        let mut salt = Vec::new();
        let mut nonce = Bytes::new();

        // [Example]
        // r=/z+giZiTxAH7r8sNAeHr7cvpqV3uo7G/bJBIJO3pjVM7t3ng,s=4UV68bIkC8f9/X8xH7aPhg==,i=4096

        for item in buf.split(|b| *b == b',') {
            let key = item[0];
            let value = &item[2..];

            match key {
                b'r' => {
                    nonce = buf.slice_ref(value);
                }

                b'i' => {
                    iterations = atoi::atoi(value).unwrap_or(4096);
                }

                b's' => {
                    salt = BASE64_STANDARD.decode(value).map_err(Error::protocol)?;
                }

                _ => {}
            }
        }

        Ok(Self {
            iterations,
            salt,
            nonce: from_utf8(&nonce).map_err(Error::protocol)?.to_owned(),
            message: from_utf8(&buf).map_err(Error::protocol)?.to_owned(),
        })
    }
}

#[derive(Debug)]
pub struct AuthenticationSaslFinal {
    pub verifier: Vec<u8>,
}

impl ProtocolDecode<'_> for AuthenticationSaslFinal {
    fn decode_with(buf: Bytes, _: ()) -> Result<Self, Error> {
        let mut verifier = Vec::new();

        for item in buf.split(|b| *b == b',') {
            let key = item[0];
            let value = &item[2..];

            if let b'v' = key {
                verifier = BASE64_STANDARD.decode(value).map_err(Error::protocol)?;
            }
        }

        Ok(Self { verifier })
    }
}
