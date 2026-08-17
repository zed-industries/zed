macro_rules! encoder {
    ($(#[$attr:meta])* $name:ident<$inner:ident> $({ $($inherent_methods:tt)* })*) => {
        pin_project_lite::pin_project! {
            $(#[$attr])*
            ///
            /// This structure implements an [`AsyncWrite`](futures_io::AsyncWrite) interface and will
            /// take in uncompressed data and write it compressed to an underlying stream.
            #[derive(Debug)]
            pub struct $name<$inner> {
                #[pin]
                inner: crate::futures::write::Encoder<$inner, crate::codecs::$name>,
            }
        }

        impl<$inner: futures_io::AsyncWrite> $name<$inner> {
            $(
                /// Creates a new encoder which will take in uncompressed data and write it
                /// compressed to the given stream.
                ///
                $($inherent_methods)*
            )*

            /// Creates a new encoder with the given codec, which will take in uncompressed data and write it
            /// compressed to the given stream.
            pub fn with_codec(read: $inner, codec: crate::codecs::$name) -> $name<$inner> {
                $name {
                   inner: crate::futures::write::Encoder::new(read, codec)
                }
            }
        }

        impl<$inner> $name<$inner> {
            /// Acquires a reference to the underlying writer that this encoder is wrapping.
            pub fn get_ref(&self) -> &$inner {
                self.inner.get_ref()
            }

            /// Acquires a mutable reference to the underlying writer that this encoder is
            /// wrapping.
            ///
            /// Note that care must be taken to avoid tampering with the state of the writer which
            /// may otherwise confuse this encoder.
            pub fn get_mut(&mut self) -> &mut $inner {
                self.inner.get_mut()
            }

            /// Acquires a pinned mutable reference to the underlying writer that this encoder is
            /// wrapping.
            ///
            /// Note that care must be taken to avoid tampering with the state of the writer which
            /// may otherwise confuse this encoder.
            pub fn get_pin_mut(self: std::pin::Pin<&mut Self>) -> std::pin::Pin<&mut $inner> {
                self.project().inner.get_pin_mut()
            }

            /// Consumes this encoder returning the underlying writer.
            ///
            /// Note that this may discard internal state of this encoder, so care should be taken
            /// to avoid losing resources when this is called.
            pub fn into_inner(self) -> $inner {
                self.inner.into_inner()
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
        }

        impl<$inner: futures_io::AsyncRead> futures_io::AsyncRead for $name<$inner> {
            fn poll_read(
                self: std::pin::Pin<&mut Self>,
                cx: &mut std::task::Context<'_>,
                buf: &mut [u8]
            ) -> std::task::Poll<std::io::Result<usize>> {
                self.get_pin_mut().poll_read(cx, buf)
            }

            fn poll_read_vectored(
                self: std::pin::Pin<&mut Self>,
                cx: &mut std::task::Context<'_>,
                bufs: &mut [futures_io::IoSliceMut<'_>]
            ) -> std::task::Poll<std::io::Result<usize>> {
                self.get_pin_mut().poll_read_vectored(cx, bufs)
            }
        }

        impl<$inner: futures_io::AsyncBufRead> futures_io::AsyncBufRead for $name<$inner> {
            fn poll_fill_buf(
                self: std::pin::Pin<&mut Self>,
                cx: &mut std::task::Context<'_>
            ) -> std::task::Poll<std::io::Result<&[u8]>> {
                self.get_pin_mut().poll_fill_buf(cx)
            }

            fn consume(self: std::pin::Pin<&mut Self>, amt: usize) {
                self.get_pin_mut().consume(amt)
            }
        }

        const _: () = {
            use crate::core::util::{_assert_send, _assert_sync};
            use core::pin::Pin;
            use futures_io::AsyncWrite;

            _assert_send::<$name<Pin<Box<dyn AsyncWrite + Send>>>>();
            _assert_sync::<$name<Pin<Box<dyn AsyncWrite + Sync>>>>();
        };
    }
}
