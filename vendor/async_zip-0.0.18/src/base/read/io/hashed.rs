// Copyright (c) 2022 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

use crate::base::read::io::poll_result_ok;

use std::pin::Pin;
use std::task::{ready, Context, Poll};

use crc32fast::Hasher;
use futures_lite::io::AsyncRead;
use pin_project::pin_project;

/// A wrapping reader which computes the CRC32 hash of data read via [`AsyncRead`].
#[pin_project]
pub(crate) struct HashedReader<R> {
    #[pin]
    pub(crate) reader: R,
    pub(crate) hasher: Hasher,
}

impl<R> HashedReader<R>
where
    R: AsyncRead + Unpin,
{
    /// Constructs a new wrapping reader from a generic [`AsyncRead`] implementer.
    pub(crate) fn new(reader: R) -> Self {
        Self { reader, hasher: Hasher::default() }
    }

    /// Swaps the internal hasher and returns the computed CRC32 hash.
    ///
    /// The internal hasher is taken and replaced with a newly-constructed one. As a result, this method should only be
    /// called once EOF has been reached and it's known that no more data will be read, else the computed hash(s) won't
    /// accurately represent the data read in.
    pub(crate) fn swap_and_compute_hash(&mut self) -> u32 {
        std::mem::take(&mut self.hasher).finalize()
    }

    /// Consumes this reader and returns the inner value.
    pub(crate) fn into_inner(self) -> R {
        self.reader
    }
}

impl<R> AsyncRead for HashedReader<R>
where
    R: AsyncRead + Unpin,
{
    fn poll_read(self: Pin<&mut Self>, c: &mut Context<'_>, b: &mut [u8]) -> Poll<std::io::Result<usize>> {
        let project = self.project();
        let written = poll_result_ok!(ready!(project.reader.poll_read(c, b)));
        project.hasher.update(&b[..written]);

        Poll::Ready(Ok(written))
    }
}
