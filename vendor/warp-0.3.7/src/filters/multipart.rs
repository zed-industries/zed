//! Multipart body filters
//!
//! [`Filter`](crate::Filter)s that extract a multipart body for a route.

use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::{fmt, io};

use bytes::{Buf, Bytes};
use futures_util::{future, Stream};
use headers::ContentType;
use hyper::Body;
use mime::Mime;
use multer::{Field as PartInner, Multipart as FormDataInner};

use crate::filter::{Filter, FilterBase, Internal};
use crate::reject::{self, Rejection};

// If not otherwise configured, default to 2MB.
const DEFAULT_FORM_DATA_MAX_LENGTH: u64 = 1024 * 1024 * 2;

/// A [`Filter`](crate::Filter) to extract a `multipart/form-data` body from a request.
///
/// Create with the `warp::multipart::form()` function.
#[derive(Debug, Clone)]
pub struct FormOptions {
    max_length: Option<u64>,
}

/// A `Stream` of multipart/form-data `Part`s.
///
/// Extracted with a `warp::multipart::form` filter.
pub struct FormData {
    inner: FormDataInner<'static>,
}

/// A single "part" of a multipart/form-data body.
///
/// Yielded from the `FormData` stream.
pub struct Part {
    part: PartInner<'static>,
}

/// Create a [`Filter`](crate::Filter) to extract a `multipart/form-data` body from a request.
///
/// The extracted `FormData` type is a `Stream` of `Part`s, and each `Part`
/// in turn is a `Stream` of bytes.
pub fn form() -> FormOptions {
    FormOptions {
        max_length: Some(DEFAULT_FORM_DATA_MAX_LENGTH),
    }
}

// ===== impl Form =====

impl FormOptions {
    /// Set the maximum byte length allowed for this body.
    ///
    /// `max_length(None)` means that maximum byte length is not checked.
    /// Defaults to 2MB.
    pub fn max_length(mut self, max: impl Into<Option<u64>>) -> Self {
        self.max_length = max.into();
        self
    }
}

type FormFut = Pin<Box<dyn Future<Output = Result<(FormData,), Rejection>> + Send>>;

impl FilterBase for FormOptions {
    type Extract = (FormData,);
    type Error = Rejection;
    type Future = FormFut;

    fn filter(&self, _: Internal) -> Self::Future {
        let boundary = super::header::header2::<ContentType>().and_then(|ct| {
            let mime = Mime::from(ct);
            let mime = mime
                .get_param("boundary")
                .map(|v| v.to_string())
                .ok_or_else(|| reject::invalid_header("content-type"));
            future::ready(mime)
        });

        let filt = boundary
            .and(super::body::body())
            .map(|boundary: String, body| {
                let body = BodyIoError(body);
                FormData {
                    inner: FormDataInner::new(body, &boundary),
                }
            });

        if let Some(max_length) = self.max_length {
            Box::pin(
                super::body::content_length_limit(max_length)
                    .and(filt)
                    .filter(Internal),
            )
        } else {
            Box::pin(filt.filter(Internal))
        }
    }
}

// ===== impl FormData =====

impl fmt::Debug for FormData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormData").finish()
    }
}

impl Stream for FormData {
    type Item = Result<Part, crate::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.inner.poll_next_field(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Ok(Some(part))) => {
                if part.name().is_some() || part.file_name().is_some() {
                    Poll::Ready(Some(Ok(Part { part })))
                } else {
                    Poll::Ready(Some(Err(crate::Error::new(MultipartFieldMissingName))))
                }
            }
            Poll::Ready(Ok(None)) => Poll::Ready(None),
            Poll::Ready(Err(err)) => Poll::Ready(Some(Err(crate::Error::new(err)))),
        }
    }
}

// ===== impl Part =====

impl Part {
    /// Get the name of this part.
    pub fn name(&self) -> &str {
        self.part
            .name()
            .unwrap_or_else(|| self.part.file_name().expect("checked for name previously"))
    }

    /// Get the filename of this part, if present.
    pub fn filename(&self) -> Option<&str> {
        self.part.file_name()
    }

    /// Get the content-type of this part, if present.
    pub fn content_type(&self) -> Option<&str> {
        let content_type = self.part.content_type();
        content_type.map(|t| t.as_ref())
    }

    /// Asynchronously get some of the data for this `Part`.
    pub async fn data(&mut self) -> Option<Result<impl Buf, crate::Error>> {
        future::poll_fn(|cx| self.poll_next(cx)).await
    }

    /// Convert this `Part` into a `Stream` of `Buf`s.
    pub fn stream(self) -> impl Stream<Item = Result<impl Buf, crate::Error>> {
        PartStream(self)
    }

    fn poll_next(&mut self, cx: &mut Context<'_>) -> Poll<Option<Result<Bytes, crate::Error>>> {
        match Pin::new(&mut self.part).poll_next(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Some(Ok(bytes))) => Poll::Ready(Some(Ok(bytes))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Ready(Some(Err(err))) => Poll::Ready(Some(Err(crate::Error::new(err)))),
        }
    }
}

impl fmt::Debug for Part {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut builder = f.debug_struct("Part");
        builder.field("name", &self.name());

        if let Some(ref filename) = self.part.file_name() {
            builder.field("filename", filename);
        }

        if let Some(ref mime) = self.part.content_type() {
            builder.field("content_type", mime);
        }

        builder.finish()
    }
}

struct PartStream(Part);

impl Stream for PartStream {
    type Item = Result<Bytes, crate::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.0.poll_next(cx)
    }
}

struct BodyIoError(Body);

impl Stream for BodyIoError {
    type Item = io::Result<Bytes>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match Pin::new(&mut self.0).poll_next(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Some(Ok(bytes))) => Poll::Ready(Some(Ok(bytes))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Ready(Some(Err(err))) => {
                Poll::Ready(Some(Err(io::Error::new(io::ErrorKind::Other, err))))
            }
        }
    }
}

/// An error used when a multipart field is missing a name.
#[derive(Debug)]
struct MultipartFieldMissingName;

impl Display for MultipartFieldMissingName {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Multipart field is missing a name")
    }
}

impl StdError for MultipartFieldMissingName {}
