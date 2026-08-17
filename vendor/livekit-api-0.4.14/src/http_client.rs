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

#[cfg(any(feature = "services-tokio", feature = "signal-client-tokio"))]
mod tokio {
    #[cfg(feature = "signal-client-tokio")]
    pub use reqwest::get;

    #[cfg(feature = "services-tokio")]
    pub use reqwest::Client;
}

#[cfg(any(feature = "services-tokio", feature = "signal-client-tokio"))]
pub use tokio::*;

#[cfg(any(feature = "__signal-client-async-compatible", feature = "services-async"))]
mod async_std {

    #[cfg(any(
        feature = "native-tls-vendored",
        feature = "rustls-tls-native-roots",
        feature = "rustls-tls-webpki-roots",
        feature = "__rustls-tls"
    ))]
    compile_error!("the async std compatible libraries do not support these features");

    #[cfg(any(feature = "__signal-client-async-compatible", feature = "services-async"))]
    pub struct Response(http::Response<isahc::AsyncBody>);

    #[cfg(feature = "__signal-client-async-compatible")]
    mod signal_client {
        use std::io;

        use isahc::AsyncReadResponseExt;

        use super::Response;

        impl Response {
            pub fn status(&self) -> http::StatusCode {
                self.0.status()
            }

            pub async fn text(mut self) -> io::Result<String> {
                self.0.text().await
            }
        }

        pub async fn get(url: &str) -> io::Result<Response> {
            let response = isahc::get_async(url).await?;
            Ok(Response(response))
        }
    }

    #[cfg(feature = "__signal-client-async-compatible")]
    pub use signal_client::*;

    #[cfg(feature = "services-async")]
    mod services {
        use std::io;

        use isahc::AsyncReadResponseExt;
        use prost::bytes::Bytes;

        use super::Response;

        use http::header::{Entry, OccupiedEntry};
        use url::Url;

        impl Response {
            pub async fn bytes(self) -> io::Result<Bytes> {
                Ok(self.0.bytes().await?.into())
            }

            pub async fn json<T: serde::de::DeserializeOwned + Unpin>(&mut self) -> io::Result<T> {
                self.0.json().await.map_err(|e| io::Error::new(io::ErrorKind::Other, e))
            }
        }

        #[derive(Debug)]
        pub struct Client(isahc::HttpClient);

        impl Client {
            pub fn new() -> Self {
                Self(isahc::HttpClient::new().unwrap())
            }
        }

        impl Client {
            pub fn post(&self, url: Url) -> RequestBuilder {
                RequestBuilder {
                    body: Vec::new(),
                    builder: isahc::http::Request::post(url.as_str()),
                    client: self.0.clone(),
                }
            }
        }

        pub struct RequestBuilder {
            builder: isahc::http::request::Builder,
            body: Vec<u8>,
            client: isahc::HttpClient,
        }

        impl RequestBuilder {
            pub fn headers(mut self, headers: http::HeaderMap) -> Self {
                // Copied from: https://docs.rs/reqwest/0.11.24/src/reqwest/util.rs.html#62-89
                let self_headers = self.builder.headers_mut().unwrap();
                let mut prev_entry: Option<OccupiedEntry<_>> = None;
                for (key, value) in headers {
                    match key {
                        Some(key) => match self_headers.entry(key) {
                            Entry::Occupied(mut e) => {
                                e.insert(value);
                                prev_entry = Some(e);
                            }
                            Entry::Vacant(e) => {
                                let e = e.insert_entry(value);
                                prev_entry = Some(e);
                            }
                        },
                        None => match prev_entry {
                            Some(ref mut entry) => {
                                entry.append(value);
                            }
                            None => unreachable!("HeaderMap::into_iter yielded None first"),
                        },
                    }
                }
                self
            }

            pub fn body(mut self, body: Vec<u8>) -> Self {
                self.body = body;
                self
            }

            pub async fn send(self) -> io::Result<Response> {
                let request = self.builder.body(self.body).unwrap();
                let response = self.client.send_async(request).await?;
                Ok(Response(response))
            }
        }
    }

    #[cfg(feature = "services-async")]
    pub use services::*;
}

#[cfg(any(feature = "__signal-client-async-compatible", feature = "services-async"))]
pub use async_std::*;
