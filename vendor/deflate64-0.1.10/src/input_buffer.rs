use crate::InternalErr;
use std::cmp::min;

#[derive(Copy, Clone, Debug)]
pub(crate) struct BitsBuffer {
    bit_buffer: u32,
    bits_in_buffer: i32,
}

impl BitsBuffer {
    pub(crate) fn new() -> BitsBuffer {
        Self {
            bit_buffer: 0,
            bits_in_buffer: 0,
        }
    }
}

pub(crate) struct InputBuffer<'a> {
    pub bits: BitsBuffer,
    pub buffer: &'a [u8],
    pub read_bytes: usize,
}

impl<'a> InputBuffer<'a> {
    pub fn new(bits: BitsBuffer, buffer: &'a [u8]) -> Self {
        Self {
            bits,
            buffer,
            read_bytes: 0,
        }
    }

    pub fn available_bits(&self) -> i32 {
        self.bits.bits_in_buffer
    }

    pub fn available_bytes(&self) -> usize {
        self.buffer.len() + (self.bits.bits_in_buffer / 4) as usize
    }

    pub fn ensure_bits_available(&mut self, count: i32) -> bool {
        debug_assert!(0 < count && count <= 16, "count is invalid.");

        // manual inlining to improve perf
        if self.bits.bits_in_buffer < count {
            if self.needs_input() {
                return false;
            }

            // insert a byte to bitbuffer
            self.bits.bit_buffer |= (self.buffer[0] as u32) << self.bits.bits_in_buffer;
            self.advance(1);
            self.bits.bits_in_buffer += 8;

            if self.bits.bits_in_buffer < count {
                if self.needs_input() {
                    return false;
                }
                // insert a byte to bitbuffer
                self.bits.bit_buffer |= (self.buffer[0] as u32) << self.bits.bits_in_buffer;
                self.advance(1);
                self.bits.bits_in_buffer += 8;
            }
        }

        true
    }

    pub fn try_load_16bits(&mut self) -> u32 {
        if self.bits.bits_in_buffer < 8 {
            if self.buffer.len() > 1 {
                self.bits.bit_buffer |= (self.buffer[0] as u32) << self.bits.bits_in_buffer;
                self.bits.bit_buffer |= (self.buffer[1] as u32) << (self.bits.bits_in_buffer + 8);
                self.advance(2);
                self.bits.bits_in_buffer += 16;
            } else if !self.buffer.is_empty() {
                self.bits.bit_buffer |= (self.buffer[0] as u32) << self.bits.bits_in_buffer;
                self.advance(1);
                self.bits.bits_in_buffer += 8;
            }
        } else if self.bits.bits_in_buffer < 16 && !self.buffer.is_empty() {
            self.bits.bit_buffer |= (self.buffer[0] as u32) << self.bits.bits_in_buffer;
            self.advance(1);
            self.bits.bits_in_buffer += 8;
        }

        self.bits.bit_buffer
    }

    fn get_bit_mask(&self, count: i32) -> u32 {
        (1 << count) - 1
    }

    pub fn get_bits(&mut self, count: i32) -> Result<u16, InternalErr> {
        debug_assert!(0 < count && count <= 16, "count is invalid.");

        if !self.ensure_bits_available(count) {
            return Err(InternalErr::DataNeeded);
        }

        let result = (self.bits.bit_buffer & self.get_bit_mask(count)) as u16;
        self.bits.bit_buffer >>= count;
        self.bits.bits_in_buffer -= count;
        Ok(result)
    }

    pub fn copy_to(&mut self, mut output: &mut [u8]) -> usize {
        debug_assert!(self.bits.bits_in_buffer % 8 == 0);

        // Copy the bytes in bitBuffer first.
        let mut bytes_from_bit_buffer = 0;
        while self.bits.bits_in_buffer > 0 && !output.is_empty() {
            output[0] = self.bits.bit_buffer as u8;
            output = &mut output[1..];
            self.bits.bit_buffer >>= 8;
            self.bits.bits_in_buffer -= 8;
            bytes_from_bit_buffer += 1;
        }

        if output.is_empty() {
            return bytes_from_bit_buffer;
        }

        let length = min(output.len(), self.buffer.len());
        output[..length].copy_from_slice(&self.buffer[..length]);
        self.advance(length);
        bytes_from_bit_buffer + length
    }

    pub fn needs_input(&self) -> bool {
        self.buffer.is_empty()
    }

    /// <summary>Skip n bits in the buffer.</summary>
    pub fn skip_bits(&mut self, n: i32) {
        debug_assert!(
            self.bits.bits_in_buffer >= n,
            "No enough bits in the buffer, Did you call ensure_bits_available?"
        );
        self.bits.bit_buffer >>= n;
        self.bits.bits_in_buffer -= n;
    }

    /// <summary>Skips to the next byte boundary.</summary>
    pub fn skip_to_byte_boundary(&mut self) {
        self.bits.bit_buffer >>= self.bits.bits_in_buffer % 8;
        self.bits.bits_in_buffer -= self.bits.bits_in_buffer % 8;
    }

    fn advance(&mut self, buf: usize) {
        self.buffer = &self.buffer[buf..];
        self.read_bytes += buf;
    }
}
