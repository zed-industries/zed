macro_rules! decoder {
    ($(#[$attr:meta])* $name:ident<$inner:ident> $({ $($inherent_methods:tt)* })*) => {
        pin_project_lite::pin_project! {
            $(#[$attr])*
            ///
            /// This structure implements an [`AsyncRead`](tokio::io::AsyncRead) interface and will
            /// read compressed data from an underlying stream and emit a stream of uncompressed data.
            #[derive(Debug)]
            pub struct $name<$inner> {
                #[pin]
                inner: crate::tokio::bufread::Decoder<$inner, crate::codecs::$name>,
            }
        }

        impl<$inner: tokio::io::AsyncBufRead> $name<$inner> {
            /// Creates a new decoder which will read compressed data from the given stream and
            /// emit an uncompressed stream.
            pub fn new(read: $inner) -> $name<$inner> {
                $name {
                    inner: crate::tokio::bufread::Decoder::new(read, crate::codecs::$name::new()),
                }
            }

            /// Creates a new decoder with the given codec, which will read compressed data from the given stream and
            /// emit an uncompressed stream.
            pub fn with_codec(read: $inner, codec: crate::codecs::$name) -> $name<$inner> {
                $name {
                    inner: crate::tokio::bufread::Decoder::new(read, codec)
                }
            }

            $($($inherent_methods)*)*
        }

        impl<$inner> $name<$inner> {
            /// Configure multi-member/frame decoding, if enabled this will reset the decoder state
            /// when reaching the end of a compressed member/frame and expect either EOF or another
            /// compressed member/frame to follow it in the stream.
            pub fn multiple_members(&mut self, enabled: bool) {
                self.inner.multiple_members(enabled);
            }

            /// Acquires a reference to the underlying reader that this decoder is wrapping.
            pub fn get_ref(&self) -> &$inner {
                self.inner.get_ref()
            }

            /// Acquires a mutable reference to the underlying reader that this decoder is
            /// wrapping.
            ///
            /// Note that care must be taken to avoid tampering with the state of the reader which
            /// may otherwise confuse this decoder.
            pub fn get_mut(&mut self) -> &mut $inner {
                self.inner.get_mut()
            }

            /// Acquires a pinned mutable reference to the underlying reader that this decoder is
            /// wrapping.
            ///
            /// Note that care must be taken to avoid tampering with the state of the reader which
            /// may otherwise confuse this decoder.
            pub fn get_pin_mut(self: std::pin::Pin<&mut Self>) -> std::pin::Pin<&mut $inner> {
                self.project().inner.get_pin_mut()
            }

            /// Consumes this decoder returning the underlying reader.
            ///
            /// Note that this may discard internal state of this decoder, so care should be taken
            /// to avoid losing resources when this is called.
            pub fn into_inner(self) -> $inner {
                self.inner.into_inner()
            }
        }

        impl<$inner: tokio::io::AsyncBufRead> tokio::io::AsyncRead for $name<$inner> {
            fn poll_read(
                self: std::pin::Pin<&mut Self>,
                cx: &mut std::task::Context<'_>,
                buf: &mut tokio::io::ReadBuf<'_>,
            ) -> std::task::Poll<std::io::Result<()>> {
                self.project().inner.poll_read(cx, buf)
            }
        }

        impl<$inner: tokio::io::AsyncWrite> tokio::io::AsyncWrite for $name<$inner> {
            fn poll_write(
                self: std::pin::Pin<&mut Self>,
                cx: &mut std::task::Context<'_>,
                buf: &[u8],
            ) -> std::task::Poll<std::io::Result<usize>> {
                self.get_pin_mut().poll_write(cx, buf)
            }

            fn poll_flush(
                self: std::pin::Pin<&mut Self>,
                cx: &mut std::task::Context<'_>,
            ) -> std::task::Poll<std::io::Result<()>> {
                self.get_pin_mut().poll_flush(cx)
            }

            fn poll_shutdown(
                self: std::pin::Pin<&mut Self>,
                cx: &mut std::task::Context<'_>,
            ) -> std::task::Poll<std::io::Result<()>> {
                self.get_pin_mut().poll_shutdown(cx)
            }

            fn poll_write_vectored(
                self: std::pin::Pin<&mut Self>,
                cx: &mut std::task::Context<'_>,
                bufs: &[std::io::IoSlice<'_>],
            ) -> std::task::Poll<std::io::Result<usize>> {
                self.get_pin_mut().poll_write_vectored(cx, bufs)
            }

            fn is_write_vectored(&self) -> bool {
                self.get_ref().is_write_vectored()
            }
        }

        const _: () = {
            use crate::core::util::{_assert_send, _assert_sync};
            use core::pin::Pin;
            use tokio::io::AsyncBufRead;

            _assert_send::<$name<Pin<Box<dyn AsyncBufRead + Send>>>>();
            _assert_sync::<$name<Pin<Box<dyn AsyncBufRead + Sync>>>>();
        };
    }
}
