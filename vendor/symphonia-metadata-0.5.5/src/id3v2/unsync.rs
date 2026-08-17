// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::io;

use symphonia_core::errors::Result;
use symphonia_core::io::{FiniteStream, ReadBytes};

pub fn read_syncsafe_leq32<B: ReadBytes>(reader: &mut B, bit_width: u8) -> Result<u32> {
    debug_assert!(bit_width <= 32);

    let mut result = 0u32;
    let mut bits_read = 0;

    while bits_read < bit_width {
        // Ensure bits_read never exceeds the bit width which will cause an overflow
        let next_read = (bit_width - bits_read).min(7);
        bits_read += next_read;
        // The mask should have of the bits below 2 ^ nex_read set to 1
        let mask = (1 << next_read) - 1;
        result |= u32::from(reader.read_u8()? & mask) << (bit_width - bits_read);
    }

    Ok(result)
}

pub fn decode_unsynchronisation(buf: &mut [u8]) -> &mut [u8] {
    let len = buf.len();
    let mut src = 0;
    let mut dst = 0;

    // Decode the unsynchronisation scheme in-place.
    while src < len - 1 {
        buf[dst] = buf[src];
        dst += 1;
        src += 1;

        if buf[src - 1] == 0xff && buf[src] == 0x00 {
            src += 1;
        }
    }

    if src < len {
        buf[dst] = buf[src];
        dst += 1;
    }

    &mut buf[..dst]
}

pub struct UnsyncStream<B: ReadBytes + FiniteStream> {
    inner: B,
    byte: u8,
}

impl<B: ReadBytes + FiniteStream> UnsyncStream<B> {
    pub fn new(inner: B) -> Self {
        UnsyncStream { inner, byte: 0 }
    }

    /// Convert the `UnsyncStream` to the inner stream.
    pub fn into_inner(self) -> B {
        self.inner
    }
}

impl<B: ReadBytes + FiniteStream> FiniteStream for UnsyncStream<B> {
    #[inline(always)]
    fn byte_len(&self) -> u64 {
        self.inner.byte_len()
    }

    #[inline(always)]
    fn bytes_read(&self) -> u64 {
        self.inner.bytes_read()
    }

    #[inline(always)]
    fn bytes_available(&self) -> u64 {
        self.inner.bytes_available()
    }
}

impl<B: ReadBytes + FiniteStream> ReadBytes for UnsyncStream<B> {
    fn read_byte(&mut self) -> io::Result<u8> {
        let last = self.byte;

        self.byte = self.inner.read_byte()?;

        // If the last byte was 0xff, and the current byte is 0x00, the current byte should be
        // dropped and the next byte read instead.
        if last == 0xff && self.byte == 0x00 {
            self.byte = self.inner.read_byte()?;
        }

        Ok(self.byte)
    }

    fn read_double_bytes(&mut self) -> io::Result<[u8; 2]> {
        Ok([self.read_byte()?, self.read_byte()?])
    }

    fn read_triple_bytes(&mut self) -> io::Result<[u8; 3]> {
        Ok([self.read_byte()?, self.read_byte()?, self.read_byte()?])
    }

    fn read_quad_bytes(&mut self) -> io::Result<[u8; 4]> {
        Ok([self.read_byte()?, self.read_byte()?, self.read_byte()?, self.read_byte()?])
    }

    fn read_buf(&mut self, _: &mut [u8]) -> io::Result<usize> {
        // Not required.
        unimplemented!();
    }

    fn read_buf_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        let len = buf.len();

        if len > 0 {
            // Fill the provided buffer directly from the underlying reader.
            self.inner.read_buf_exact(buf)?;

            // If the last seen byte was 0xff, and the first byte in buf is 0x00, skip the first
            // byte of buf.
            let mut src = usize::from(self.byte == 0xff && buf[0] == 0x00);
            let mut dst = 0;

            // Record the last byte in buf to continue unsychronisation streaming later.
            self.byte = buf[len - 1];

            // Decode the unsynchronisation scheme in-place.
            while src < len - 1 {
                buf[dst] = buf[src];
                dst += 1;
                src += 1;

                if buf[src - 1] == 0xff && buf[src] == 0x00 {
                    src += 1;
                }
            }

            // When the final two src bytes are [ 0xff, 0x00 ], src will always equal len.
            // Therefore, if src < len, then the final byte should always be copied to dst.
            if src < len {
                buf[dst] = buf[src];
                dst += 1;
            }

            // If dst < len, then buf is not full. Read the remaining bytes manually to completely
            // fill buf.
            while dst < len {
                buf[dst] = self.read_byte()?;
                dst += 1;
            }
        }

        Ok(())
    }

    fn scan_bytes_aligned<'a>(
        &mut self,
        _: &[u8],
        _: usize,
        _: &'a mut [u8],
    ) -> io::Result<&'a mut [u8]> {
        // Not required.
        unimplemented!();
    }

    fn ignore_bytes(&mut self, count: u64) -> io::Result<()> {
        for _ in 0..count {
            self.inner.read_byte()?;
        }
        Ok(())
    }

    fn pos(&self) -> u64 {
        // Not required.
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::read_syncsafe_leq32;
    use symphonia_core::io::BufReader;

    #[test]
    fn verify_read_syncsafe_leq32() {
        let mut stream = BufReader::new(&[3, 4, 80, 1, 15]);
        assert_eq!(101875743, read_syncsafe_leq32(&mut stream, 32).unwrap());

        // Special case: for a bit depth that is not a multiple of 7 such as 32
        // we need to ensure the mask is correct.
        // In this case, the final iteration should read 4 bits and have a mask of 0b0000_1111.
        // 0b0000_1111 has a 0 in 16's place so testing mask & 16 will ensure this is working.
        let mut stream = BufReader::new(&[16, 16, 16, 16, 16]);
        assert_eq!(541098240, read_syncsafe_leq32(&mut stream, 32).unwrap());

        let mut stream = BufReader::new(&[3, 4, 80, 1]);
        assert_eq!(6367233, read_syncsafe_leq32(&mut stream, 28).unwrap());

        let mut stream = BufReader::new(&[3, 4, 80, 1]);
        assert_eq!(0, read_syncsafe_leq32(&mut stream, 0).unwrap());
    }
}
