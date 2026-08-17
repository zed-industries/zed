macro_rules! encoder {
    ($(#[$attr:meta])* $name:ident<$inner:ident> $({ $($inherent_methods:tt)* })*) => {
        pin_project_lite::pin_project! {
            $(#[$attr])*
            ///
            /// This structure implements an [`AsyncRead`](futures_io::AsyncRead) interface and will
            /// read uncompressed data from an underlying stream and emit a stream of compressed data.
            #[derive(Debug)]
            pub struct $name<$inner> {
                #[pin]
                inner: crate::futures::bufread::Encoder<$inner, crate::codecs::$name>,
            }
        }

        impl<$inner: futures_io::AsyncBufRead> $name<$inner> {
            $(
                /// Creates a new encoder which will read uncompressed data from the given stream
                /// and emit an compressed stream.
                ///
                $($inherent_methods)*
            )*

            /// Creates a new encoder with the given codec, which will read uncompressed data from the given stream
            /// and emit an compressed stream.
            pub fn with_codec(read: $inner, codec: crate::codecs::$name) -> $name<$inner> {
                $name {
                   inner: crate::futures::bufread::Encoder::new(read, codec)
                }
            }

            /// Acquires a reference to the underlying reader that this encoder is wrapping.
            pub fn get_ref(&self) -> &$inner {
                self.inner.get_ref()
            }

            /// Acquires a mutable reference to the underlying reader that this encoder is
            /// wrapping.
            ///
            /// Note that care must be taken to avoid tampering with the state of the reader which
            /// may otherwise confuse this encoder.
            pub fn get_mut(&mut self) -> &mut $inner {
                self.inner.get_mut()
            }

            /// Acquires a pinned mutable reference to the underlying reader that this encoder is
            /// wrapping.
            ///
            /// Note that care must be taken to avoid tampering with the state of the reader which
            /// may otherwise confuse this encoder.
            pub fn get_pin_mut(self: std::pin::Pin<&mut Self>) -> std::pin::Pin<&mut $inner> {
                self.project().inner.get_pin_mut()
            }

            /// Consumes this encoder returning the underlying reader.
            ///
            /// Note that this may discard internal state of this encoder, so care should be taken
            /// to avoid losing resources when this is called.
            pub fn into_inner(self) -> $inner {
                self.inner.into_inner()
            }
        }

        impl<$inner: futures_io::AsyncBufRead> futures_io::AsyncRead for $name<$inner> {
            fn poll_read(
                self: std::pin::Pin<&mut Self>,
                cx: &mut std::task::Context<'_>,
                buf: &mut [u8],
            ) -> std::task::Poll<std::io::Result<usize>> {
                self.project().inner.poll_read(cx, buf)
            }
        }

        impl<$inner: futures_io::AsyncWrite> futures_io::AsyncWrite for $name<$inner> {
            fn poll_write(
                self: std::pin::Pin<&mut Self>,
                cx: &mut std::task::Context<'_>,
                buf: &[u8],
            ) -> std::task::Poll<std::io::Result<usize>> {
                self.project().inner.poll_write(cx, buf)
            }

            fn poll_flush(
                self: std::pin::Pin<&mut Self>,
                cx: &mut std::task::Context<'_>,
            ) -> std::task::Poll<std::io::Result<()>> {
                self.project().inner.poll_flush(cx)
            }

            fn poll_close(
                self: std::pin::Pin<&mut Self>,
                cx: &mut std::task::Context<'_>,
            ) -> std::task::Poll<std::io::Result<()>> {
                self.project().inner.poll_close(cx)
            }

            fn poll_write_vectored(
                self: std::pin::Pin<&mut Self>,
                cx: &mut std::task::Context<'_>,
                bufs: &[std::io::IoSlice<'_>]
            ) -> std::task::Poll<std::io::Result<usize>> {
                self.project().inner.poll_write_vectored(cx, bufs)
            }
        }

        const _: () = {
            use crate::core::util::{_assert_send, _assert_sync};
            use core::pin::Pin;
            use futures_io::AsyncBufRead;

            _assert_send::<$name<Pin<Box<dyn AsyncBufRead + Send>>>>();
            _assert_sync::<$name<Pin<Box<dyn AsyncBufRead + Sync>>>>();
        };
    }
}
