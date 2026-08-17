use crate::{buffer::Buffer, input_buffer::InputBuffer};
use std::cmp::min;

// With Deflate64 we can have up to a 65536 length as well as up to a 65538 distance. This means we need a Window that is at
// least 131074 bytes long so we have space to retrieve up to a full 64kb in look-back and place it in our buffer without
// overwriting existing data. OutputWindow requires that the WINDOW_SIZE be an exponent of 2, so we round up to 2^18.
const WINDOW_SIZE: usize = 262144;
const WINDOW_MASK: usize = 262143;

/// <summary>
/// This class maintains a window for decompressed output.
/// We need to keep this because the decompressed information can be
/// a literal or a length/distance pair. For length/distance pair,
/// we need to look back in the output window and copy bytes from there.
/// We use a byte array of WINDOW_SIZE circularly.
/// </summary>
#[derive(Debug)]
pub(crate) struct OutputWindow {
    window: [u8; WINDOW_SIZE],
    end: usize,
    bytes_used: usize,
}

impl OutputWindow {
    pub fn new() -> Self {
        Self {
            window: [0; WINDOW_SIZE],
            end: 0,
            bytes_used: 0,
        }
    }

    pub(crate) fn clear_bytes_used(&mut self) {
        self.bytes_used = 0;
    }

    /// <summary>Add a byte to output window.</summary>
    pub fn write(&mut self, b: u8) {
        debug_assert!(
            self.bytes_used < WINDOW_SIZE,
            "Can't add byte when window is full!"
        );
        self.window[self.end] = b;
        self.end += 1;
        self.end &= WINDOW_MASK;
        self.bytes_used += 1;
    }

    pub fn write_length_distance(&mut self, mut length: usize, distance: usize) {
        debug_assert!((self.bytes_used + length) <= WINDOW_SIZE, "No Enough space");

        // move backwards distance bytes in the output stream,
        // and copy length bytes from this position to the output stream.
        self.bytes_used += length;
        let mut copy_start = (self.end.overflowing_sub(distance).0) & WINDOW_MASK; // start position for coping.

        let border = WINDOW_SIZE - length;
        if copy_start <= border && self.end < border {
            if length <= distance {
                // src, srcIdx, dst, dstIdx, len
                // Array.copy(self._window, copy_start, self._window, self._end, length);
                self.window
                    .copy_within(copy_start..(copy_start + length), self.end);
                self.end += length;
            } else {
                // The referenced string may overlap the current
                // position; for example, if the last 2 bytes decoded have values
                // X and Y, a string reference with <length = 5, distance = 2>
                // adds X,Y,X,Y,X to the output stream.
                while length > 0 {
                    length -= 1;
                    self.window[self.end] = self.window[copy_start];
                    self.end += 1;
                    copy_start += 1;
                }
            }
        } else {
            // copy byte by byte
            while length > 0 {
                length -= 1;
                self.window[self.end] = self.window[copy_start];
                self.end += 1;
                copy_start += 1;
                self.end &= WINDOW_MASK;
                copy_start &= WINDOW_MASK;
            }
        }
    }

    /// <summary>
    /// Copy up to length of bytes from input directly.
    /// This is used for uncompressed block.
    /// </summary>
    pub fn copy_from(&mut self, input: &mut InputBuffer<'_>, mut length: usize) -> usize {
        length = min(
            min(length, WINDOW_SIZE - self.bytes_used),
            input.available_bytes(),
        );
        let mut copied: usize;

        // We might need wrap around to copy all bytes.
        let tail_len = WINDOW_SIZE - self.end;
        if length > tail_len {
            // copy the first part
            copied = input.copy_to(&mut self.window[self.end..][..tail_len]);
            if copied == tail_len {
                // only try to copy the second part if we have enough bytes in input
                copied += input.copy_to(&mut self.window[..length - tail_len]);
            }
        } else {
            // only one copy is needed if there is no wrap around.
            copied = input.copy_to(&mut self.window[self.end..][..length]);
        }

        self.end = (self.end + copied) & WINDOW_MASK;
        self.bytes_used += copied;
        copied
    }

    /// <summary>Free space in output window.</summary>
    pub fn free_bytes(&self) -> usize {
        WINDOW_SIZE - self.bytes_used
    }

    /// <summary>Bytes not consumed in output window.</summary>
    pub fn available_bytes(&self) -> usize {
        self.bytes_used
    }

    /// <summary>Copy the decompressed bytes to output buffer.</summary>
    pub fn copy_to(&mut self, output: Buffer<'_>) -> usize {
        let (copy_end, mut output) = if output.len() > self.bytes_used {
            // we can copy all the decompressed bytes out
            (self.end, output.index_mut(..self.bytes_used))
        } else {
            // copy length of bytes
            (
                (self
                    .end
                    .overflowing_sub(self.bytes_used)
                    .0
                    .overflowing_add(output.len())
                    .0)
                    & WINDOW_MASK,
                output,
            )
        };

        let copied = output.len();

        let mut output = if output.len() > copy_end {
            let tail_len = output.len() - copy_end;
            // this means we need to copy two parts separately
            // copy the tail_len bytes from the end of the output window
            output
                .reborrow()
                .index_mut(..tail_len)
                .copy_from_slice(&self.window[WINDOW_SIZE - tail_len..][..tail_len]);
            output.index_mut(tail_len..).index_mut(..copy_end)
        } else {
            output
        };
        output.copy_from_slice(&self.window[copy_end - output.len()..][..output.len()]);
        self.bytes_used -= copied;
        //debug_assert!(self.bytes_used >= 0, "check this function and find why we copied more bytes than we have");
        copied
    }
}
