/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! A [`Buf`] implementation that counts bytes read.

use bytes::Buf;

/// A [`Buf`] implementation that counts bytes read.
pub(crate) struct CountBuf<'a, B>
where
    B: Buf,
{
    buffer: &'a mut B,
    count: usize,
}

impl<'a, B> CountBuf<'a, B>
where
    B: Buf,
{
    /// Creates a new `CountBuf` by wrapping the given `buffer`.
    pub(crate) fn new(buffer: &'a mut B) -> Self {
        CountBuf { buffer, count: 0 }
    }

    /// Consumes the `CountBuf` and returns the number of bytes read.
    pub(crate) fn into_count(self) -> usize {
        self.count
    }
}

impl<B> Buf for CountBuf<'_, B>
where
    B: Buf,
{
    fn remaining(&self) -> usize {
        self.buffer.remaining()
    }

    fn chunk(&self) -> &[u8] {
        self.buffer.chunk()
    }

    fn advance(&mut self, cnt: usize) {
        self.count += cnt;
        self.buffer.advance(cnt);
    }
}

#[cfg(test)]
mod tests {
    use super::CountBuf;
    use bytes::Buf;

    #[test]
    fn count_no_data_read() {
        let mut data: &[u8] = &[];
        let buf = CountBuf::new(&mut data);
        assert_eq!(0, buf.into_count());
    }

    #[test]
    fn count_data_read() {
        let mut data: &[u8] = &[0, 0, 0, 5, 0, 10u8];
        let mut buf = CountBuf::new(&mut data);
        assert_eq!(5, buf.get_i32());
        assert_eq!(4, buf.into_count());

        let mut data: &[u8] = &[0, 0, 0, 5, 0, 10u8];
        let mut buf = CountBuf::new(&mut data);
        assert_eq!(5, buf.get_i32());
        assert_eq!(10, buf.get_i16());
        assert_eq!(6, buf.into_count());
    }

    #[test]
    fn chunk_called_multiple_times_before_advance() {
        let mut data: &[u8] = &[0, 0, 0, 5, 0, 10u8];
        let mut buf = CountBuf::new(&mut data);
        for _ in 0..3 {
            buf.chunk();
        }
        buf.advance(4);
        assert_eq!(10, buf.get_i16());
        assert_eq!(6, buf.into_count());
    }
}
