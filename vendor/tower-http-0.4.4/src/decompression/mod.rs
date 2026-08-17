//! Middleware that decompresses request and response bodies.
//!
//! # Examples
//!
//! #### Request
//! ```rust
//! use bytes::BytesMut;
//! use flate2::{write::GzEncoder, Compression};
//! use http::{header, HeaderValue, Request, Response};
//! use http_body::Body as _; // for Body::data
//! use hyper::Body;
//! use std::{error::Error, io::Write};
//! use tower::{Service, ServiceBuilder, service_fn, ServiceExt};
//! use tower_http::{BoxError, decompression::{DecompressionBody, RequestDecompressionLayer}};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), BoxError> {
//! // A request encoded with gzip coming from some HTTP client.
//! let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
//! encoder.write_all(b"Hello?")?;
//! let request = Request::builder()
//!     .header(header::CONTENT_ENCODING, "gzip")
//!     .body(Body::from(encoder.finish()?))?;
//!
//! // Our HTTP server
//! let mut server = ServiceBuilder::new()
//!     // Automatically decompress request bodies.
//!     .layer(RequestDecompressionLayer::new())
//!     .service(service_fn(handler));
//!
//! // Send the request, with the gzip encoded body, to our server.
//! let _response = server.ready().await?.call(request).await?;
//!
//! // Handler receives request whose body is decoded when read
//! async fn handler(mut req: Request<DecompressionBody<Body>>) -> Result<Response<Body>, BoxError>{
//!     let mut data = BytesMut::new();
//!     while let Some(chunk) = req.body_mut().data().await {
//!         let chunk = chunk?;
//!         data.extend_from_slice(&chunk[..]);
//!     }
//!     assert_eq!(data.freeze().to_vec(), b"Hello?");
//!     Ok(Response::new(Body::from("Hello, World!")))
//! }
//! # Ok(())
//! # }
//! ```
//!
//! #### Response
//! ```rust
//! use bytes::BytesMut;
//! use http::{Request, Response};
//! use http_body::Body as _; // for Body::data
//! use hyper::Body;
//! use std::convert::Infallible;
//! use tower::{Service, ServiceExt, ServiceBuilder, service_fn};
//! use tower_http::{compression::Compression, decompression::DecompressionLayer, BoxError};
//! #
//! # #[tokio::main]
//! # async fn main() -> Result<(), tower_http::BoxError> {
//! # async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible> {
//! #     let body = Body::from("Hello, World!");
//! #     Ok(Response::new(body))
//! # }
//!
//! // Some opaque service that applies compression.
//! let service = Compression::new(service_fn(handle));
//!
//! // Our HTTP client.
//! let mut client = ServiceBuilder::new()
//!     // Automatically decompress response bodies.
//!     .layer(DecompressionLayer::new())
//!     .service(service);
//!
//! // Call the service.
//! //
//! // `DecompressionLayer` takes care of setting `Accept-Encoding`.
//! let request = Request::new(Body::empty());
//!
//! let response = client
//!     .ready()
//!     .await?
//!     .call(request)
//!     .await?;
//!
//! // Read the body
//! let mut body = response.into_body();
//! let mut bytes = BytesMut::new();
//! while let Some(chunk) = body.data().await {
//!     let chunk = chunk?;
//!     bytes.extend_from_slice(&chunk[..]);
//! }
//! let body = String::from_utf8(bytes.to_vec()).map_err(Into::<BoxError>::into)?;
//!
//! assert_eq!(body, "Hello, World!");
//! #
//! # Ok(())
//! # }
//! ```

mod request;

mod body;
mod future;
mod layer;
mod service;

pub use self::{
    body::DecompressionBody, future::ResponseFuture, layer::DecompressionLayer,
    service::Decompression,
};

pub use self::request::future::RequestDecompressionFuture;
pub use self::request::layer::RequestDecompressionLayer;
pub use self::request::service::RequestDecompression;

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::*;
    use crate::compression::Compression;
    use bytes::BytesMut;
    use flate2::write::GzEncoder;
    use http::Response;
    use http_body::Body as _;
    use hyper::{Body, Client, Error, Request};
    use tower::{service_fn, Service, ServiceExt};

    #[tokio::test]
    async fn works() {
        let mut client = Decompression::new(Compression::new(service_fn(handle)));

        let req = Request::builder()
            .header("accept-encoding", "gzip")
            .body(Body::empty())
            .unwrap();
        let res = client.ready().await.unwrap().call(req).await.unwrap();

        // read the body, it will be decompressed automatically
        let mut body = res.into_body();
        let mut data = BytesMut::new();
        while let Some(chunk) = body.data().await {
            let chunk = chunk.unwrap();
            data.extend_from_slice(&chunk[..]);
        }
        let decompressed_data = String::from_utf8(data.freeze().to_vec()).unwrap();

        assert_eq!(decompressed_data, "Hello, World!");
    }

    #[tokio::test]
    async fn decompress_multi_gz() {
        let mut client = Decompression::new(service_fn(handle_multi_gz));

        let req = Request::builder()
            .header("accept-encoding", "gzip")
            .body(Body::empty())
            .unwrap();
        let res = client.ready().await.unwrap().call(req).await.unwrap();

        // read the body, it will be decompressed automatically
        let mut body = res.into_body();
        let mut data = BytesMut::new();
        while let Some(chunk) = body.data().await {
            let chunk = chunk.unwrap();
            data.extend_from_slice(&chunk[..]);
        }
        let decompressed_data = String::from_utf8(data.freeze().to_vec()).unwrap();

        assert_eq!(decompressed_data, "Hello, World!");
    }

    async fn handle(_req: Request<Body>) -> Result<Response<Body>, Error> {
        Ok(Response::new(Body::from("Hello, World!")))
    }

    async fn handle_multi_gz(_req: Request<Body>) -> Result<Response<Body>, Error> {
        let mut buf = Vec::new();
        let mut enc1 = GzEncoder::new(&mut buf, Default::default());
        enc1.write_all(b"Hello, ").unwrap();
        enc1.finish().unwrap();

        let mut enc2 = GzEncoder::new(&mut buf, Default::default());
        enc2.write_all(b"World!").unwrap();
        enc2.finish().unwrap();

        let mut res = Response::new(Body::from(buf));
        res.headers_mut()
            .insert("content-encoding", "gzip".parse().unwrap());
        Ok(res)
    }

    #[allow(dead_code)]
    async fn is_compatible_with_hyper() {
        let mut client = Decompression::new(Client::new());

        let req = Request::new(Body::empty());

        let _: Response<DecompressionBody<Body>> =
            client.ready().await.unwrap().call(req).await.unwrap();
    }
}
