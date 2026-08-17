use base64::engine::general_purpose::STANDARD as ENGINE;
use base64::Engine;
use bytes::Bytes;
use sha1::{Digest, Sha1};

use super::SecWebsocketKey;

/// The `Sec-Websocket-Accept` header.
///
/// This header is used in the Websocket handshake, sent back by the
/// server indicating a successful handshake. It is a signature
/// of the `Sec-Websocket-Key` header.
///
/// # Example
///
/// ```no_run
/// # extern crate headers;
/// use headers::{SecWebsocketAccept, SecWebsocketKey};
///
/// let sec_key: SecWebsocketKey = /* from request headers */
/// #    unimplemented!();
///
/// let sec_accept = SecWebsocketAccept::from(sec_key);
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SecWebsocketAccept(::HeaderValue);

derive_header! {
    SecWebsocketAccept(_),
    name: SEC_WEBSOCKET_ACCEPT
}

impl From<SecWebsocketKey> for SecWebsocketAccept {
    fn from(key: SecWebsocketKey) -> SecWebsocketAccept {
        sign(key.0.as_bytes())
    }
}

fn sign(key: &[u8]) -> SecWebsocketAccept {
    let mut sha1 = Sha1::default();
    sha1.update(key);
    sha1.update(&b"258EAFA5-E914-47DA-95CA-C5AB0DC85B11"[..]);
    let b64 = Bytes::from(ENGINE.encode(&sha1.finalize()));

    let val = ::HeaderValue::from_maybe_shared(b64).expect("base64 is a valid value");

    SecWebsocketAccept(val)
}

#[cfg(test)]
mod tests {
    use super::super::{test_decode, test_encode};
    use super::*;

    #[test]
    fn key_to_accept() {
        // From https://tools.ietf.org/html/rfc6455#section-1.2
        let key = test_decode::<SecWebsocketKey>(&["dGhlIHNhbXBsZSBub25jZQ=="]).expect("key");
        let accept = SecWebsocketAccept::from(key);
        let headers = test_encode(accept);

        assert_eq!(
            headers["sec-websocket-accept"],
            "s3pPLMBiTxaQ9kYGzzhZRbK+xOo="
        );
    }
}
