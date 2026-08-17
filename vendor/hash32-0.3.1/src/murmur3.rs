use core::slice;
use core::mem::MaybeUninit;

use byteorder::{ByteOrder, LE};

use crate::Hasher as _;

/// 32-bit MurmurHash3 hasher
pub struct Hasher {
    buf: Buffer,
    index: Index,
    processed: u32,
    state: State,
}

struct State(u32);

#[derive(Clone, Copy)]
#[repr(align(4))]
struct Buffer {
    bytes: MaybeUninit<[u8; 4]>,
}

#[derive(Clone, Copy, PartialEq)]
enum Index {
    _0,
    _1,
    _2,
    _3,
}

impl Index {
    fn usize(&self) -> usize {
        match *self {
            Index::_0 => 0,
            Index::_1 => 1,
            Index::_2 => 2,
            Index::_3 => 3,
        }
    }
}

impl From<usize> for Index {
    fn from(x: usize) -> Self {
        match x % 4 {
            0 => Index::_0,
            1 => Index::_1,
            2 => Index::_2,
            3 => Index::_3,
            _ => unreachable!(),
        }
    }
}

impl Hasher {
    fn push(&mut self, buf: &[u8]) {
        let start = self.index.usize();
        let len = buf.len();
        // NOTE(unsafe) avoid calling `memcpy` on a 0-3 byte copy
        // self.buf.bytes[start..start+len].copy_from(buf);
        for i in 0..len {
            unsafe {
                *self.buf.bytes.assume_init_mut().get_unchecked_mut(start + i) = *buf.get_unchecked(i);
            }
        }
        self.index = Index::from(start + len);
    }
}

impl Default for Hasher {
    #[allow(deprecated)]
    fn default() -> Self {
        Hasher {
            buf: Buffer { bytes: MaybeUninit::uninit() },
            index: Index::_0,
            processed: 0,
            state: State(0),
        }
    }
}

impl crate::Hasher for Hasher {
    fn finish32(&self) -> u32 {
        // tail
        let mut state = match self.index {
            Index::_3 => {
                let mut block = 0;
                unsafe {
                    block ^= u32::from(self.buf.bytes.assume_init_ref()[2]) << 16;
                    block ^= u32::from(self.buf.bytes.assume_init_ref()[1]) << 8;
                    block ^= u32::from(self.buf.bytes.assume_init_ref()[0]);
                }
                self.state.0 ^ pre_mix(block)
            }
            Index::_2 => {
                let mut block = 0;
                unsafe {
                    block ^= u32::from(self.buf.bytes.assume_init_ref()[1]) << 8;
                    block ^= u32::from(self.buf.bytes.assume_init_ref()[0]);
                }
                self.state.0 ^ pre_mix(block)
            }
            Index::_1 => {
                let mut block = 0;
                unsafe {
                    block ^= u32::from(self.buf.bytes.assume_init_ref()[0]);
                }
                self.state.0 ^ pre_mix(block)
            }
            Index::_0 => self.state.0,
        };

        // finalization mix
        state ^= self.processed;
        state ^= state >> 16;
        state = state.wrapping_mul(0x85ebca6b);
        state ^= state >> 13;
        state = state.wrapping_mul(0xc2b2ae35);
        state ^= state >> 16;

        state
    }
}

impl core::hash::Hasher for Hasher {
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        let len = bytes.len();
        self.processed += len as u32;

        let body = if self.index == Index::_0 {
            bytes
        } else {
            let index = self.index.usize();
            if len + index >= 4 {
                // we can complete a block using the data left in the buffer
                // NOTE(unsafe) avoid panicking branch (`slice_index_len_fail`)
                // let (head, body) = bytes.split_at(4 - index);
                let mid = 4 - index;
                let head = unsafe { slice::from_raw_parts(bytes.as_ptr(), mid) };
                let body = unsafe {
                    slice::from_raw_parts(bytes.as_ptr().offset(mid as isize), len - mid)
                };

                // NOTE(unsafe) avoid calling `memcpy` on a 0-3 byte copy
                // self.buf.bytes[index..].copy_from_slice(head);
                for i in 0..4 - index {
                    unsafe {
                        *self.buf.bytes.assume_init_mut().get_unchecked_mut(index + i) = *head.get_unchecked(i);
                    }
                }

                self.index = Index::_0;

                self.state.process_block(&self.buf.bytes);

                body
            } else {
                bytes
            }
        };

        for block in body.chunks(4) {
            if block.len() == 4 {
                self.state
                    .process_block(unsafe { &*(block.as_ptr() as *const _) });
            } else {
                self.push(block);
            }
        }

        // XXX is this faster?
        // for block in body.exact_chunks(4) {
        //     self.state
        //         .process_block(unsafe { &*(block.as_ptr() as *const _) });
        // }

        // let tail = body.split_at(body.len() / 4 * 4).1;

        // self.push(tail);
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.finish32().into()
    }
}

const C1: u32 = 0xcc9e2d51;
const C2: u32 = 0x1b873593;
const R1: u32 = 15;

impl State {
    fn process_block(&mut self, block: &MaybeUninit<[u8; 4]>) {
        self.0 ^= pre_mix(LE::read_u32(unsafe { block.assume_init_ref() }));
        self.0 = self.0.rotate_left(13);
        self.0 = 5u32.wrapping_mul(self.0).wrapping_add(0xe6546b64);
    }
}

fn pre_mix(mut block: u32) -> u32 {
    block = block.wrapping_mul(C1);
    block = block.rotate_left(R1);
    block = block.wrapping_mul(C2);
    block
}
