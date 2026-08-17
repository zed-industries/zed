// Copyright © 2023-2025 Andrea Corbellini and contributors
// SPDX-License-Identifier: BSD-3-Clause

use crate::CircularBuffer;
use core::convert::Infallible;

#[cfg(feature = "embedded-io")]
use embedded_io::ErrorType;

#[cfg(all(feature = "embedded-io-async", not(feature = "embedded-io")))]
use embedded_io_async::ErrorType;

impl<const N: usize> ErrorType for CircularBuffer<N, u8> {
    type Error = Infallible;
}

#[cfg(feature = "embedded-io")]
impl<const N: usize> embedded_io::Write for CircularBuffer<N, u8> {
    #[inline]
    fn write(&mut self, src: &[u8]) -> Result<usize, Self::Error> {
        self.extend_from_slice(src);
        Ok(src.len())
    }

    #[inline]
    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[cfg(feature = "embedded-io")]
impl<const N: usize> embedded_io::Read for CircularBuffer<N, u8> {
    fn read(&mut self, dst: &mut [u8]) -> Result<usize, Self::Error> {
        let (mut front, mut back) = self.as_slices();
        let mut count = front.read(dst)?;
        count += back.read(&mut dst[count..])?;
        self.truncate_front(self.len() - count);
        Ok(count)
    }
}

#[cfg(feature = "embedded-io")]
impl<const N: usize> embedded_io::BufRead for CircularBuffer<N, u8> {
    fn fill_buf(&mut self) -> Result<&[u8], Self::Error> {
        let (front, back) = self.as_slices();
        if !front.is_empty() {
            Ok(front)
        } else {
            Ok(back)
        }
    }

    fn consume(&mut self, amt: usize) {
        let amt = core::cmp::min(amt, self.len());
        self.drain(..amt);
    }
}

#[cfg(feature = "embedded-io-async")]
impl<const N: usize> embedded_io_async::Write for CircularBuffer<N, u8> {
    async fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn write(&mut self, src: &[u8]) -> Result<usize, Self::Error> {
        self.extend_from_slice(src);
        Ok(src.len())
    }
}

#[cfg(feature = "embedded-io-async")]
impl<const N: usize> embedded_io_async::Read for CircularBuffer<N, u8> {
    async fn read(&mut self, dst: &mut [u8]) -> Result<usize, Self::Error> {
        let (mut front, mut back) = self.as_slices();
        let mut count = front.read(dst).await?;
        count += back.read(&mut dst[count..]).await?;
        self.truncate_front(self.len() - count);
        Ok(count)
    }
}

#[cfg(feature = "embedded-io-async")]
impl<const N: usize> embedded_io_async::BufRead for CircularBuffer<N, u8> {
    async fn fill_buf(&mut self) -> Result<&[u8], Self::Error> {
        let (front, back) = self.as_slices();
        if !front.is_empty() {
            Ok(front)
        } else {
            Ok(back)
        }
    }

    fn consume(&mut self, amt: usize) {
        let amt = core::cmp::min(amt, self.len());
        self.drain(..amt);
    }
}
