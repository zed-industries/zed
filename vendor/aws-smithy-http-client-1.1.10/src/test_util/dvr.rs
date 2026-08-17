/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Extremely Experimental Test Connection
//!
//! Warning: Extremely experimental, API likely to change.
//!
//! DVR is an extremely experimental record & replay framework that supports multi-frame HTTP request / response traffic.

use aws_smithy_runtime_api::client::orchestrator::{HttpRequest, HttpResponse};
use aws_smithy_runtime_api::http::Headers;
use aws_smithy_types::base64;
use bytes::Bytes;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

mod record;
mod replay;

pub use record::RecordingClient;
pub use replay::ReplayingClient;

/// A complete traffic recording
///
/// A traffic recording can be replayed with [`RecordingClient`].
#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkTraffic {
    events: Vec<Event>,
    docs: Option<String>,
    version: Version,
}

impl NetworkTraffic {
    /// Network events
    pub fn events(&self) -> &Vec<Event> {
        &self.events
    }

    /// Create a NetworkTraffic instance from a file
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&contents)?)
    }

    /// Create a NetworkTraffic instance from a file
    pub fn write_to_file(&self, path: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
        let serialized = serde_json::to_string_pretty(&self)?;
        Ok(std::fs::write(path, serialized)?)
    }

    /// Update the network traffic with all `content-length` fields fixed to match the contents
    pub fn correct_content_lengths(&mut self) {
        let mut content_lengths: HashMap<(ConnectionId, Direction), usize> = HashMap::new();
        for event in &self.events {
            if let Action::Data { data, direction } = &event.action {
                let entry = content_lengths.entry((event.connection_id, *direction));
                *entry.or_default() += data.copy_to_vec().len();
            }
        }
        for event in &mut self.events {
            let (headers, direction) = match &mut event.action {
                Action::Request {
                    request: Request { headers, .. },
                } => (headers, Direction::Request),
                Action::Response {
                    response: Ok(Response { headers, .. }),
                } => (headers, Direction::Response),
                _ => continue,
            };
            let Some(computed_content_length) =
                content_lengths.get(&(event.connection_id, direction))
            else {
                continue;
            };
            if headers.contains_key("content-length") {
                headers.insert(
                    "content-length".to_string(),
                    vec![computed_content_length.to_string()],
                );
            }
        }
    }
}

/// Serialization version of DVR data
#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Version {
    /// Initial network traffic version
    V0,
}

/// A network traffic recording may contain multiple different connections occurring simultaneously
#[derive(Copy, Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct ConnectionId(usize);

/// A network event
///
/// Network events consist of a connection identifier and an action. An event is sufficient to
/// reproduce traffic later during replay
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Event {
    connection_id: ConnectionId,
    action: Action,
}

/// An initial HTTP request, roughly equivalent to `http::Request<()>`
///
/// The initial request phase of an HTTP request. The body will be
/// sent later as a separate action.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Request {
    uri: String,
    headers: IndexMap<String, Vec<String>>,
    method: String,
}

/// An initial HTTP response roughly equivalent to `http::Response<()>`
///
/// The initial response phase of an HTTP request. The body will be
/// sent later as a separate action.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Response {
    status: u16,
    headers: IndexMap<String, Vec<String>>,
}

#[cfg(feature = "legacy-test-util")]
impl From<&Request> for http_02x::Request<()> {
    fn from(request: &Request) -> Self {
        let mut builder = http_02x::Request::builder().uri(request.uri.as_str());
        for (k, values) in request.headers.iter() {
            for v in values {
                builder = builder.header(k, v);
            }
        }
        builder.method(request.method.as_str()).body(()).unwrap()
    }
}

impl From<&Request> for http_1x::Request<()> {
    fn from(request: &Request) -> Self {
        let mut builder = http_1x::Request::builder().uri(request.uri.as_str());
        for (k, values) in request.headers.iter() {
            for v in values {
                builder = builder.header(k, v);
            }
        }
        builder.method(request.method.as_str()).body(()).unwrap()
    }
}

impl<'a> From<&'a HttpRequest> for Request {
    fn from(req: &'a HttpRequest) -> Self {
        let uri = req.uri().to_string();
        let headers = headers_to_map_http(req.headers());
        let method = req.method().to_string();
        Self {
            uri,
            headers,
            method,
        }
    }
}

fn headers_to_map_http(headers: &Headers) -> IndexMap<String, Vec<String>> {
    let mut out: IndexMap<_, Vec<_>> = IndexMap::new();
    for (header_name, header_value) in headers.iter() {
        let entry = out.entry(header_name.to_string()).or_default();
        entry.push(header_value.to_string());
    }
    out
}

fn headers_to_map(headers: &Headers) -> IndexMap<String, Vec<String>> {
    let mut out: IndexMap<_, Vec<_>> = IndexMap::new();
    for (header_name, header_value) in headers.iter() {
        let entry = out.entry(header_name.to_string()).or_default();
        entry.push(
            std::str::from_utf8(header_value.as_ref())
                .unwrap()
                .to_string(),
        );
    }
    out
}

#[cfg(feature = "legacy-test-util")]
fn headers_to_map_02x(headers: &http_02x::HeaderMap) -> IndexMap<String, Vec<String>> {
    let mut out: IndexMap<_, Vec<_>> = IndexMap::new();
    for (header_name, header_value) in headers.iter() {
        let entry = out.entry(header_name.to_string()).or_default();
        entry.push(
            std::str::from_utf8(header_value.as_ref())
                .unwrap()
                .to_string(),
        );
    }
    out
}

#[cfg(feature = "legacy-test-util")]
impl<'a, B> From<&'a http_02x::Response<B>> for Response {
    fn from(resp: &'a http_02x::Response<B>) -> Self {
        let status = resp.status().as_u16();
        let headers = headers_to_map_02x(resp.headers());
        Self { status, headers }
    }
}

fn headers_to_map_1x(headers: &http_1x::HeaderMap) -> IndexMap<String, Vec<String>> {
    let mut out: IndexMap<_, Vec<_>> = IndexMap::new();
    for (header_name, header_value) in headers.iter() {
        let entry = out.entry(header_name.to_string()).or_default();
        entry.push(
            std::str::from_utf8(header_value.as_ref())
                .unwrap()
                .to_string(),
        );
    }
    out
}

impl<'a, B> From<&'a http_1x::Response<B>> for Response {
    fn from(resp: &'a http_1x::Response<B>) -> Self {
        let status = resp.status().as_u16();
        let headers = headers_to_map_1x(resp.headers());
        Self { status, headers }
    }
}

impl From<&HttpResponse> for Response {
    fn from(resp: &HttpResponse) -> Self {
        Self {
            status: resp.status().into(),
            headers: headers_to_map(resp.headers()),
        }
    }
}

/// Error response wrapper
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Error(String);

/// Network Action
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum Action {
    /// Initial HTTP Request
    Request {
        /// HTTP Request headers, method, and URI
        request: Request,
    },

    /// Initial HTTP response or failure
    Response {
        /// HTTP response or failure
        response: Result<Response, Error>,
    },

    /// Data segment
    Data {
        /// Body Data
        data: BodyData,
        /// Direction: request vs. response
        direction: Direction,
    },

    /// End of data
    Eof {
        /// Succesful vs. failed termination
        ok: bool,
        /// Direction: request vs. response
        direction: Direction,
    },
}

/// Event direction
///
/// During replay, this is used to replay data in the right direction
#[derive(Copy, Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum Direction {
    /// Request phase
    Request,
    /// Response phase
    Response,
}

impl Direction {
    /// The opposite of a given direction
    pub fn opposite(self) -> Self {
        match self {
            Direction::Request => Direction::Response,
            Direction::Response => Direction::Request,
        }
    }
}

/// HTTP Body Data Abstraction
///
/// When the data is a UTF-8 encoded string, it will be serialized as a string for readability.
/// Otherwise, it will be base64 encoded.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[non_exhaustive]
pub enum BodyData {
    /// UTF-8 encoded data
    Utf8(String),

    /// Base64 encoded binary data
    Base64(String),
}

impl BodyData {
    /// Convert [`BodyData`] into Bytes.
    pub fn into_bytes(self) -> Vec<u8> {
        match self {
            BodyData::Utf8(string) => string.into_bytes(),
            BodyData::Base64(string) => base64::decode(string).unwrap(),
        }
    }

    /// Copy [`BodyData`] into a `Vec<u8>`.
    pub fn copy_to_vec(&self) -> Vec<u8> {
        match self {
            BodyData::Utf8(string) => string.as_bytes().into(),
            BodyData::Base64(string) => base64::decode(string).unwrap(),
        }
    }
}

impl From<Bytes> for BodyData {
    fn from(data: Bytes) -> Self {
        match std::str::from_utf8(data.as_ref()) {
            Ok(string) => BodyData::Utf8(string.to_string()),
            Err(_) => BodyData::Base64(base64::encode(data)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::error::Error;
    use std::fs;

    use aws_smithy_runtime_api::client::http::HttpConnector;
    use aws_smithy_runtime_api::client::http::SharedHttpConnector;
    use aws_smithy_types::body::SdkBody;
    use aws_smithy_types::byte_stream::ByteStream;

    #[tokio::test]
    async fn correctly_fixes_content_lengths() -> Result<(), Box<dyn Error>> {
        let network_traffic = fs::read_to_string("test-data/example.com.json")?;
        let mut network_traffic: NetworkTraffic = serde_json::from_str(&network_traffic)?;
        network_traffic.correct_content_lengths();
        let Action::Request {
            request: Request { headers, .. },
        } = &network_traffic.events[0].action
        else {
            panic!("unexpected event")
        };
        // content length is not added when it wasn't initially present
        assert_eq!(headers.get("content-length"), None);

        let Action::Response {
            response: Ok(Response { headers, .. }),
        } = &network_traffic.events[3].action
        else {
            panic!("unexpected event: {:?}", network_traffic.events[3].action);
        };
        // content length is not added when it wasn't initially present
        let expected_length = "hello from example.com".len();
        assert_eq!(
            headers.get("content-length"),
            Some(&vec![expected_length.to_string()])
        );
        Ok(())
    }

    #[cfg(feature = "legacy-test-util")]
    #[tokio::test]
    async fn turtles_all_the_way_down() -> Result<(), Box<dyn Error>> {
        // create a replaying connection from a recording, wrap a recording connection around it,
        // make a request, then verify that the same traffic was recorded.
        let network_traffic = fs::read_to_string("test-data/example.com.json")?;
        let mut network_traffic: NetworkTraffic = serde_json::from_str(&network_traffic)?;
        network_traffic.correct_content_lengths();
        let inner = ReplayingClient::new(network_traffic.events.clone());
        let connection = RecordingClient::new(SharedHttpConnector::new(inner.clone()));
        let req = http_02x::Request::post("https://www.example.com")
            .body(SdkBody::from("hello world"))
            .unwrap();
        let mut resp = connection.call(req.try_into().unwrap()).await.expect("ok");
        let body = std::mem::replace(resp.body_mut(), SdkBody::taken());
        let data = ByteStream::new(body).collect().await.unwrap().into_bytes();
        assert_eq!(
            String::from_utf8(data.to_vec()).unwrap(),
            "hello from example.com"
        );
        assert_eq!(
            connection.events().as_slice(),
            network_traffic.events.as_slice()
        );
        let requests = inner.take_requests().await;
        assert_eq!(
            requests[0].uri(),
            &http_02x::Uri::from_static("https://www.example.com")
        );
        assert_eq!(
            requests[0].body(),
            &Bytes::from_static("hello world".as_bytes())
        );
        Ok(())
    }
}
