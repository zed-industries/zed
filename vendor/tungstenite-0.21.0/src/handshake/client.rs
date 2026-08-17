//! Client handshake machine.

use std::{
    io::{Read, Write},
    marker::PhantomData,
};

use http::{
    header::HeaderName, HeaderMap, Request as HttpRequest, Response as HttpResponse, StatusCode,
};
use httparse::Status;
use log::*;

use super::{
    derive_accept_key,
    headers::{FromHttparse, MAX_HEADERS},
    machine::{HandshakeMachine, StageResult, TryParse},
    HandshakeRole, MidHandshake, ProcessingResult,
};
use crate::{
    error::{Error, ProtocolError, Result, UrlError},
    protocol::{Role, WebSocket, WebSocketConfig},
};

/// Client request type.
pub type Request = HttpRequest<()>;

/// Client response type.
pub type Response = HttpResponse<Option<Vec<u8>>>;

/// Client handshake role.
#[derive(Debug)]
pub struct ClientHandshake<S> {
    verify_data: VerifyData,
    config: Option<WebSocketConfig>,
    _marker: PhantomData<S>,
}

impl<S: Read + Write> ClientHandshake<S> {
    /// Initiate a client handshake.
    pub fn start(
        stream: S,
        request: Request,
        config: Option<WebSocketConfig>,
    ) -> Result<MidHandshake<Self>> {
        if request.method() != http::Method::GET {
            return Err(Error::Protocol(ProtocolError::WrongHttpMethod));
        }

        if request.version() < http::Version::HTTP_11 {
            return Err(Error::Protocol(ProtocolError::WrongHttpVersion));
        }

        // Check the URI scheme: only ws or wss are supported
        let _ = crate::client::uri_mode(request.uri())?;

        // Convert and verify the `http::Request` and turn it into the request as per RFC.
        // Also extract the key from it (it must be present in a correct request).
        let (request, key) = generate_request(request)?;

        let machine = HandshakeMachine::start_write(stream, request);

        let client = {
            let accept_key = derive_accept_key(key.as_ref());
            ClientHandshake { verify_data: VerifyData { accept_key }, config, _marker: PhantomData }
        };

        trace!("Client handshake initiated.");
        Ok(MidHandshake { role: client, machine })
    }
}

impl<S: Read + Write> HandshakeRole for ClientHandshake<S> {
    type IncomingData = Response;
    type InternalStream = S;
    type FinalResult = (WebSocket<S>, Response);
    fn stage_finished(
        &mut self,
        finish: StageResult<Self::IncomingData, Self::InternalStream>,
    ) -> Result<ProcessingResult<Self::InternalStream, Self::FinalResult>> {
        Ok(match finish {
            StageResult::DoneWriting(stream) => {
                ProcessingResult::Continue(HandshakeMachine::start_read(stream))
            }
            StageResult::DoneReading { stream, result, tail } => {
                let result = match self.verify_data.verify_response(result) {
                    Ok(r) => r,
                    Err(Error::Http(mut e)) => {
                        *e.body_mut() = Some(tail);
                        return Err(Error::Http(e));
                    }
                    Err(e) => return Err(e),
                };

                debug!("Client handshake done.");
                let websocket =
                    WebSocket::from_partially_read(stream, tail, Role::Client, self.config);
                ProcessingResult::Done((websocket, result))
            }
        })
    }
}

/// Verifies and generates a client WebSocket request from the original request and extracts a WebSocket key from it.
pub fn generate_request(mut request: Request) -> Result<(Vec<u8>, String)> {
    let mut req = Vec::new();
    write!(
        req,
        "GET {path} {version:?}\r\n",
        path = request.uri().path_and_query().ok_or(Error::Url(UrlError::NoPathOrQuery))?.as_str(),
        version = request.version()
    )
    .unwrap();

    // Headers that must be present in a correct request.
    const KEY_HEADERNAME: &str = "Sec-WebSocket-Key";
    const WEBSOCKET_HEADERS: [&str; 5] =
        ["Host", "Connection", "Upgrade", "Sec-WebSocket-Version", KEY_HEADERNAME];

    // We must extract a WebSocket key from a properly formed request or fail if it's not present.
    let key = request
        .headers()
        .get(KEY_HEADERNAME)
        .ok_or_else(|| {
            Error::Protocol(ProtocolError::InvalidHeader(
                HeaderName::from_bytes(KEY_HEADERNAME.as_bytes()).unwrap(),
            ))
        })?
        .to_str()?
        .to_owned();

    // We must check that all necessary headers for a valid request are present. Note that we have to
    // deal with the fact that some apps seem to have a case-sensitive check for headers which is not
    // correct and should not considered the correct behavior, but it seems like some apps ignore it.
    // `http` by default writes all headers in lower-case which is fine (and does not violate the RFC)
    // but some servers seem to be poorely written and ignore RFC.
    //
    // See similar problem in `hyper`: https://github.com/hyperium/hyper/issues/1492
    let headers = request.headers_mut();
    for &header in &WEBSOCKET_HEADERS {
        let value = headers.remove(header).ok_or_else(|| {
            Error::Protocol(ProtocolError::InvalidHeader(
                HeaderName::from_bytes(header.as_bytes()).unwrap(),
            ))
        })?;
        write!(req, "{header}: {value}\r\n", header = header, value = value.to_str()?).unwrap();
    }

    // Now we must ensure that the headers that we've written once are not anymore present in the map.
    // If they do, then the request is invalid (some headers are duplicated there for some reason).
    let insensitive: Vec<String> =
        WEBSOCKET_HEADERS.iter().map(|h| h.to_ascii_lowercase()).collect();
    for (k, v) in headers {
        let mut name = k.as_str();

        // We have already written the necessary headers once (above) and removed them from the map.
        // If we encounter them again, then the request is considered invalid and error is returned.
        // Note that we can't use `.contains()`, since `&str` does not coerce to `&String` in Rust.
        if insensitive.iter().any(|x| x == name) {
            return Err(Error::Protocol(ProtocolError::InvalidHeader(k.clone())));
        }

        // Relates to the issue of some servers treating headers in a case-sensitive way, please see:
        // https://github.com/snapview/tungstenite-rs/pull/119 (original fix of the problem)
        if name == "sec-websocket-protocol" {
            name = "Sec-WebSocket-Protocol";
        }

        if name == "origin" {
            name = "Origin";
        }

        writeln!(req, "{}: {}\r", name, v.to_str()?).unwrap();
    }

    writeln!(req, "\r").unwrap();
    trace!("Request: {:?}", String::from_utf8_lossy(&req));
    Ok((req, key))
}

/// Information for handshake verification.
#[derive(Debug)]
struct VerifyData {
    /// Accepted server key.
    accept_key: String,
}

impl VerifyData {
    pub fn verify_response(&self, response: Response) -> Result<Response> {
        // 1. If the status code received from the server is not 101, the
        // client handles the response per HTTP [RFC2616] procedures. (RFC 6455)
        if response.status() != StatusCode::SWITCHING_PROTOCOLS {
            return Err(Error::Http(response));
        }

        let headers = response.headers();

        // 2. If the response lacks an |Upgrade| header field or the |Upgrade|
        // header field contains a value that is not an ASCII case-
        // insensitive match for the value "websocket", the client MUST
        // _Fail the WebSocket Connection_. (RFC 6455)
        if !headers
            .get("Upgrade")
            .and_then(|h| h.to_str().ok())
            .map(|h| h.eq_ignore_ascii_case("websocket"))
            .unwrap_or(false)
        {
            return Err(Error::Protocol(ProtocolError::MissingUpgradeWebSocketHeader));
        }
        // 3.  If the response lacks a |Connection| header field or the
        // |Connection| header field doesn't contain a token that is an
        // ASCII case-insensitive match for the value "Upgrade", the client
        // MUST _Fail the WebSocket Connection_. (RFC 6455)
        if !headers
            .get("Connection")
            .and_then(|h| h.to_str().ok())
            .map(|h| h.eq_ignore_ascii_case("Upgrade"))
            .unwrap_or(false)
        {
            return Err(Error::Protocol(ProtocolError::MissingConnectionUpgradeHeader));
        }
        // 4.  If the response lacks a |Sec-WebSocket-Accept| header field or
        // the |Sec-WebSocket-Accept| contains a value other than the
        // base64-encoded SHA-1 of ... the client MUST _Fail the WebSocket
        // Connection_. (RFC 6455)
        if !headers.get("Sec-WebSocket-Accept").map(|h| h == &self.accept_key).unwrap_or(false) {
            return Err(Error::Protocol(ProtocolError::SecWebSocketAcceptKeyMismatch));
        }
        // 5.  If the response includes a |Sec-WebSocket-Extensions| header
        // field and this header field indicates the use of an extension
        // that was not present in the client's handshake (the server has
        // indicated an extension not requested by the client), the client
        // MUST _Fail the WebSocket Connection_. (RFC 6455)
        // TODO

        // 6.  If the response includes a |Sec-WebSocket-Protocol| header field
        // and this header field indicates the use of a subprotocol that was
        // not present in the client's handshake (the server has indicated a
        // subprotocol not requested by the client), the client MUST _Fail
        // the WebSocket Connection_. (RFC 6455)
        // TODO

        Ok(response)
    }
}

impl TryParse for Response {
    fn try_parse(buf: &[u8]) -> Result<Option<(usize, Self)>> {
        let mut hbuffer = [httparse::EMPTY_HEADER; MAX_HEADERS];
        let mut req = httparse::Response::new(&mut hbuffer);
        Ok(match req.parse(buf)? {
            Status::Partial => None,
            Status::Complete(size) => Some((size, Response::from_httparse(req)?)),
        })
    }
}

impl<'h, 'b: 'h> FromHttparse<httparse::Response<'h, 'b>> for Response {
    fn from_httparse(raw: httparse::Response<'h, 'b>) -> Result<Self> {
        if raw.version.expect("Bug: no HTTP version") < /*1.*/1 {
            return Err(Error::Protocol(ProtocolError::WrongHttpVersion));
        }

        let headers = HeaderMap::from_httparse(raw.headers)?;

        let mut response = Response::new(None);
        *response.status_mut() = StatusCode::from_u16(raw.code.expect("Bug: no HTTP status code"))?;
        *response.headers_mut() = headers;
        // TODO: httparse only supports HTTP 0.9/1.0/1.1 but not HTTP 2.0
        // so the only valid value we could get in the response would be 1.1.
        *response.version_mut() = http::Version::HTTP_11;

        Ok(response)
    }
}

/// Generate a random key for the `Sec-WebSocket-Key` header.
pub fn generate_key() -> String {
    // a base64-encoded (see Section 4 of [RFC4648]) value that,
    // when decoded, is 16 bytes in length (RFC 6455)
    let r: [u8; 16] = rand::random();
    data_encoding::BASE64.encode(&r)
}

#[cfg(test)]
mod tests {
    use super::{super::machine::TryParse, generate_key, generate_request, Response};
    use crate::client::IntoClientRequest;

    #[test]
    fn random_keys() {
        let k1 = generate_key();
        println!("Generated random key 1: {}", k1);
        let k2 = generate_key();
        println!("Generated random key 2: {}", k2);
        assert_ne!(k1, k2);
        assert_eq!(k1.len(), k2.len());
        assert_eq!(k1.len(), 24);
        assert_eq!(k2.len(), 24);
        assert!(k1.ends_with("=="));
        assert!(k2.ends_with("=="));
        assert!(k1[..22].find('=').is_none());
        assert!(k2[..22].find('=').is_none());
    }

    fn construct_expected(host: &str, key: &str) -> Vec<u8> {
        format!(
            "\
            GET /getCaseCount HTTP/1.1\r\n\
            Host: {host}\r\n\
            Connection: Upgrade\r\n\
            Upgrade: websocket\r\n\
            Sec-WebSocket-Version: 13\r\n\
            Sec-WebSocket-Key: {key}\r\n\
            \r\n",
            host = host,
            key = key
        )
        .into_bytes()
    }

    #[test]
    fn request_formatting() {
        let request = "ws://localhost/getCaseCount".into_client_request().unwrap();
        let (request, key) = generate_request(request).unwrap();
        let correct = construct_expected("localhost", &key);
        assert_eq!(&request[..], &correct[..]);
    }

    #[test]
    fn request_formatting_with_host() {
        let request = "wss://localhost:9001/getCaseCount".into_client_request().unwrap();
        let (request, key) = generate_request(request).unwrap();
        let correct = construct_expected("localhost:9001", &key);
        assert_eq!(&request[..], &correct[..]);
    }

    #[test]
    fn request_formatting_with_at() {
        let request = "wss://user:pass@localhost:9001/getCaseCount".into_client_request().unwrap();
        let (request, key) = generate_request(request).unwrap();
        let correct = construct_expected("localhost:9001", &key);
        assert_eq!(&request[..], &correct[..]);
    }

    #[test]
    fn response_parsing() {
        const DATA: &[u8] = b"HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n";
        let (_, resp) = Response::try_parse(DATA).unwrap().unwrap();
        assert_eq!(resp.status(), http::StatusCode::OK);
        assert_eq!(resp.headers().get("Content-Type").unwrap(), &b"text/html"[..],);
    }

    #[test]
    fn invalid_custom_request() {
        let request = http::Request::builder().method("GET").body(()).unwrap();
        assert!(generate_request(request).is_err());
    }
}
