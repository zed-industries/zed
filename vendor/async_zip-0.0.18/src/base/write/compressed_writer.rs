// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

use crate::base::write::io::offset::AsyncOffsetWriter;
use crate::spec::Compression;

use std::io::Error;
use std::pin::Pin;
use std::task::{Context, Poll};

#[cfg(any(feature = "deflate", feature = "bzip2", feature = "zstd", feature = "lzma", feature = "xz"))]
use async_compression::futures::write;
use futures_lite::io::AsyncWrite;

pub enum CompressedAsyncWriter<'b, W: AsyncWrite + Unpin> {
    Stored(ShutdownIgnoredWriter<&'b mut AsyncOffsetWriter<W>>),
    #[cfg(feature = "deflate")]
    Deflate(write::DeflateEncoder<ShutdownIgnoredWriter<&'b mut AsyncOffsetWriter<W>>>),
    #[cfg(feature = "bzip2")]
    Bz(write::BzEncoder<ShutdownIgnoredWriter<&'b mut AsyncOffsetWriter<W>>>),
    #[cfg(feature = "lzma")]
    Lzma(write::LzmaEncoder<ShutdownIgnoredWriter<&'b mut AsyncOffsetWriter<W>>>),
    #[cfg(feature = "zstd")]
    Zstd(write::ZstdEncoder<ShutdownIgnoredWriter<&'b mut AsyncOffsetWriter<W>>>),
    #[cfg(feature = "xz")]
    Xz(write::XzEncoder<ShutdownIgnoredWriter<&'b mut AsyncOffsetWriter<W>>>),
}

impl<'b, W: AsyncWrite + Unpin> CompressedAsyncWriter<'b, W> {
    pub fn from_raw(writer: &'b mut AsyncOffsetWriter<W>, compression: Compression, precompressed: bool) -> Self {
        if precompressed {
            return CompressedAsyncWriter::Stored(ShutdownIgnoredWriter(writer));
        }

        match compression {
            Compression::Stored => CompressedAsyncWriter::Stored(ShutdownIgnoredWriter(writer)),
            #[cfg(feature = "deflate")]
            Compression::Deflate => {
                CompressedAsyncWriter::Deflate(write::DeflateEncoder::new(ShutdownIgnoredWriter(writer)))
            }
            #[cfg(feature = "deflate64")]
            Compression::Deflate64 => panic!("writing deflate64 is not supported"),
            #[cfg(feature = "bzip2")]
            Compression::Bz => CompressedAsyncWriter::Bz(write::BzEncoder::new(ShutdownIgnoredWriter(writer))),
            #[cfg(feature = "lzma")]
            Compression::Lzma => CompressedAsyncWriter::Lzma(write::LzmaEncoder::new(ShutdownIgnoredWriter(writer))),
            #[cfg(feature = "zstd")]
            Compression::Zstd => CompressedAsyncWriter::Zstd(write::ZstdEncoder::new(ShutdownIgnoredWriter(writer))),
            #[cfg(feature = "xz")]
            Compression::Xz => CompressedAsyncWriter::Xz(write::XzEncoder::new(ShutdownIgnoredWriter(writer))),
        }
    }

    pub fn into_inner(self) -> &'b mut AsyncOffsetWriter<W> {
        match self {
            CompressedAsyncWriter::Stored(inner) => inner.into_inner(),
            #[cfg(feature = "deflate")]
            CompressedAsyncWriter::Deflate(inner) => inner.into_inner().into_inner(),
            #[cfg(feature = "bzip2")]
            CompressedAsyncWriter::Bz(inner) => inner.into_inner().into_inner(),
            #[cfg(feature = "lzma")]
            CompressedAsyncWriter::Lzma(inner) => inner.into_inner().into_inner(),
            #[cfg(feature = "zstd")]
            CompressedAsyncWriter::Zstd(inner) => inner.into_inner().into_inner(),
            #[cfg(feature = "xz")]
            CompressedAsyncWriter::Xz(inner) => inner.into_inner().into_inner(),
        }
    }
}

impl<'b, W: AsyncWrite + Unpin> AsyncWrite for CompressedAsyncWriter<'b, W> {
    fn poll_write(mut self: Pin<&mut Self>, cx: &mut Context, buf: &[u8]) -> Poll<std::result::Result<usize, Error>> {
        match *self {
            CompressedAsyncWriter::Stored(ref mut inner) => Pin::new(inner).poll_write(cx, buf),
            #[cfg(feature = "deflate")]
            CompressedAsyncWriter::Deflate(ref mut inner) => Pin::new(inner).poll_write(cx, buf),
            #[cfg(feature = "bzip2")]
            CompressedAsyncWriter::Bz(ref mut inner) => Pin::new(inner).poll_write(cx, buf),
            #[cfg(feature = "lzma")]
            CompressedAsyncWriter::Lzma(ref mut inner) => Pin::new(inner).poll_write(cx, buf),
            #[cfg(feature = "zstd")]
            CompressedAsyncWriter::Zstd(ref mut inner) => Pin::new(inner).poll_write(cx, buf),
            #[cfg(feature = "xz")]
            CompressedAsyncWriter::Xz(ref mut inner) => Pin::new(inner).poll_write(cx, buf),
        }
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<std::result::Result<(), Error>> {
        match *self {
            CompressedAsyncWriter::Stored(ref mut inner) => Pin::new(inner).poll_flush(cx),
            #[cfg(feature = "deflate")]
            CompressedAsyncWriter::Deflate(ref mut inner) => Pin::new(inner).poll_flush(cx),
            #[cfg(feature = "bzip2")]
            CompressedAsyncWriter::Bz(ref mut inner) => Pin::new(inner).poll_flush(cx),
            #[cfg(feature = "lzma")]
            CompressedAsyncWriter::Lzma(ref mut inner) => Pin::new(inner).poll_flush(cx),
            #[cfg(feature = "zstd")]
            CompressedAsyncWriter::Zstd(ref mut inner) => Pin::new(inner).poll_flush(cx),
            #[cfg(feature = "xz")]
            CompressedAsyncWriter::Xz(ref mut inner) => Pin::new(inner).poll_flush(cx),
        }
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<std::result::Result<(), Error>> {
        match *self {
            CompressedAsyncWriter::Stored(ref mut inner) => Pin::new(inner).poll_close(cx),
            #[cfg(feature = "deflate")]
            CompressedAsyncWriter::Deflate(ref mut inner) => Pin::new(inner).poll_close(cx),
            #[cfg(feature = "bzip2")]
            CompressedAsyncWriter::Bz(ref mut inner) => Pin::new(inner).poll_close(cx),
            #[cfg(feature = "lzma")]
            CompressedAsyncWriter::Lzma(ref mut inner) => Pin::new(inner).poll_close(cx),
            #[cfg(feature = "zstd")]
            CompressedAsyncWriter::Zstd(ref mut inner) => Pin::new(inner).poll_close(cx),
            #[cfg(feature = "xz")]
            CompressedAsyncWriter::Xz(ref mut inner) => Pin::new(inner).poll_close(cx),
        }
    }
}

pub struct ShutdownIgnoredWriter<W: AsyncWrite + Unpin>(W);

impl<W: AsyncWrite + Unpin> ShutdownIgnoredWriter<W> {
    pub fn into_inner(self) -> W {
        self.0
    }
}

impl<W: AsyncWrite + Unpin> AsyncWrite for ShutdownIgnoredWriter<W> {
    fn poll_write(mut self: Pin<&mut Self>, cx: &mut Context, buf: &[u8]) -> Poll<std::result::Result<usize, Error>> {
        Pin::new(&mut self.0).poll_write(cx, buf)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<std::result::Result<(), Error>> {
        Pin::new(&mut self.0).poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, _: &mut Context) -> Poll<std::result::Result<(), Error>> {
        Poll::Ready(Ok(()))
    }
}
