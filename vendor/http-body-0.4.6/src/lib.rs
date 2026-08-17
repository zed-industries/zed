#![doc(html_root_url = "https://docs.rs/http-body/0.4.6")]
#![deny(
    missing_debug_implementations,
    missing_docs,
    unreachable_pub,
    broken_intra_doc_links
)]
#![cfg_attr(test, deny(warnings))]

//! Asynchronous HTTP request or response body.
//!
//! See [`Body`] for more details.
//!
//! [`Body`]: trait.Body.html

mod collect;
mod empty;
mod full;
mod limited;
mod next;
mod size_hint;

pub mod combinators;

pub use self::collect::Collected;
pub use self::empty::Empty;
pub use self::full::Full;
pub use self::limited::{LengthLimitError, Limited};
pub use self::next::{Data, Trailers};
pub use self::size_hint::SizeHint;

use self::combinators::{BoxBody, MapData, MapErr, UnsyncBoxBody};
use bytes::{Buf, Bytes};
use http::HeaderMap;
use std::convert::Infallible;
use std::ops;
use std::pin::Pin;
use std::task::{Context, Poll};

/// Trait representing a streaming body of a Request or Response.
///
/// Data is streamed via the `poll_data` function, which asynchronously yields `T: Buf` values. The
/// `size_hint` function provides insight into the total number of bytes that will be streamed.
///
/// The `poll_trailers` function returns an optional set of trailers used to finalize the request /
/// response exchange. This is mostly used when using the HTTP/2.0 protocol.
///
pub trait Body {
    /// Values yielded by the `Body`.
    type Data: Buf;

    /// The error type this `Body` might generate.
    type Error;

    /// Attempt to pull out the next data buffer of this stream.
    fn poll_data(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Self::Data, Self::Error>>>;

    /// Poll for an optional **single** `HeaderMap` of trailers.
    ///
    /// This function should only be called once `poll_data` returns `None`.
    fn poll_trailers(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<Option<HeaderMap>, Self::Error>>;

    /// Returns `true` when the end of stream has been reached.
    ///
    /// An end of stream means that both `poll_data` and `poll_trailers` will
    /// return `None`.
    ///
    /// A return value of `false` **does not** guarantee that a value will be
    /// returned from `poll_stream` or `poll_trailers`.
    fn is_end_stream(&self) -> bool {
        false
    }

    /// Returns the bounds on the remaining length of the stream.
    ///
    /// When the **exact** remaining length of the stream is known, the upper bound will be set and
    /// will equal the lower bound.
    fn size_hint(&self) -> SizeHint {
        SizeHint::default()
    }

    /// Returns future that resolves to next data chunk, if any.
    fn data(&mut self) -> Data<'_, Self>
    where
        Self: Unpin + Sized,
    {
        Data(self)
    }

    /// Returns future that resolves to trailers, if any.
    fn trailers(&mut self) -> Trailers<'_, Self>
    where
        Self: Unpin + Sized,
    {
        Trailers(self)
    }

    /// Maps this body's data value to a different value.
    fn map_data<F, B>(self, f: F) -> MapData<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Data) -> B,
        B: Buf,
    {
        MapData::new(self, f)
    }

    /// Maps this body's error value to a different value.
    fn map_err<F, E>(self, f: F) -> MapErr<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Error) -> E,
    {
        MapErr::new(self, f)
    }

    /// Turn this body into [`Collected`] body which will collect all the DATA frames
    /// and trailers.
    fn collect(self) -> crate::collect::Collect<Self>
    where
        Self: Sized,
    {
        collect::Collect::new(self)
    }

    /// Turn this body into a boxed trait object.
    fn boxed(self) -> BoxBody<Self::Data, Self::Error>
    where
        Self: Sized + Send + Sync + 'static,
    {
        BoxBody::new(self)
    }

    /// Turn this body into a boxed trait object that is !Sync.
    fn boxed_unsync(self) -> UnsyncBoxBody<Self::Data, Self::Error>
    where
        Self: Sized + Send + 'static,
    {
        UnsyncBoxBody::new(self)
    }
}

impl<T: Body + Unpin + ?Sized> Body for &mut T {
    type Data = T::Data;
    type Error = T::Error;

    fn poll_data(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Self::Data, Self::Error>>> {
        Pin::new(&mut **self).poll_data(cx)
    }

    fn poll_trailers(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<Option<HeaderMap>, Self::Error>> {
        Pin::new(&mut **self).poll_trailers(cx)
    }

    fn is_end_stream(&self) -> bool {
        Pin::new(&**self).is_end_stream()
    }

    fn size_hint(&self) -> SizeHint {
        Pin::new(&**self).size_hint()
    }
}

impl<P> Body for Pin<P>
where
    P: Unpin + ops::DerefMut,
    P::Target: Body,
{
    type Data = <<P as ops::Deref>::Target as Body>::Data;
    type Error = <<P as ops::Deref>::Target as Body>::Error;

    fn poll_data(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Self::Data, Self::Error>>> {
        Pin::get_mut(self).as_mut().poll_data(cx)
    }

    fn poll_trailers(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<Option<HeaderMap>, Self::Error>> {
        Pin::get_mut(self).as_mut().poll_trailers(cx)
    }

    fn is_end_stream(&self) -> bool {
        self.as_ref().is_end_stream()
    }

    fn size_hint(&self) -> SizeHint {
        self.as_ref().size_hint()
    }
}

impl<T: Body + Unpin + ?Sized> Body for Box<T> {
    type Data = T::Data;
    type Error = T::Error;

    fn poll_data(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Self::Data, Self::Error>>> {
        Pin::new(&mut **self).poll_data(cx)
    }

    fn poll_trailers(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<Option<HeaderMap>, Self::Error>> {
        Pin::new(&mut **self).poll_trailers(cx)
    }

    fn is_end_stream(&self) -> bool {
        self.as_ref().is_end_stream()
    }

    fn size_hint(&self) -> SizeHint {
        self.as_ref().size_hint()
    }
}

impl<B: Body> Body for http::Request<B> {
    type Data = B::Data;
    type Error = B::Error;

    fn poll_data(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Self::Data, Self::Error>>> {
        unsafe {
            self.map_unchecked_mut(http::Request::body_mut)
                .poll_data(cx)
        }
    }

    fn poll_trailers(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<Option<HeaderMap>, Self::Error>> {
        unsafe {
            self.map_unchecked_mut(http::Request::body_mut)
                .poll_trailers(cx)
        }
    }

    fn is_end_stream(&self) -> bool {
        self.body().is_end_stream()
    }

    fn size_hint(&self) -> SizeHint {
        self.body().size_hint()
    }
}

impl<B: Body> Body for http::Response<B> {
    type Data = B::Data;
    type Error = B::Error;

    fn poll_data(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Self::Data, Self::Error>>> {
        unsafe {
            self.map_unchecked_mut(http::Response::body_mut)
                .poll_data(cx)
        }
    }

    fn poll_trailers(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<Option<HeaderMap>, Self::Error>> {
        unsafe {
            self.map_unchecked_mut(http::Response::body_mut)
                .poll_trailers(cx)
        }
    }

    fn is_end_stream(&self) -> bool {
        self.body().is_end_stream()
    }

    fn size_hint(&self) -> SizeHint {
        self.body().size_hint()
    }
}

impl Body for String {
    type Data = Bytes;
    type Error = Infallible;

    fn poll_data(
        mut self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Self::Data, Self::Error>>> {
        if !self.is_empty() {
            let s = std::mem::take(&mut *self);
            Poll::Ready(Some(Ok(s.into_bytes().into())))
        } else {
            Poll::Ready(None)
        }
    }

    fn poll_trailers(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Result<Option<HeaderMap>, Self::Error>> {
        Poll::Ready(Ok(None))
    }

    fn is_end_stream(&self) -> bool {
        self.is_empty()
    }

    fn size_hint(&self) -> SizeHint {
        SizeHint::with_exact(self.len() as u64)
    }
}

#[cfg(test)]
fn _assert_bounds() {
    fn can_be_trait_object(_: &dyn Body<Data = std::io::Cursor<Vec<u8>>, Error = std::io::Error>) {}
}
