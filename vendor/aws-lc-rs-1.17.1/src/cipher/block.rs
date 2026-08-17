// Copyright 2018 Brian Smith.
// SPDX-License-Identifier: ISC
// Modifications copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

/// An array of 16 bytes that can (in the `x86_64` and `AAarch64` ABIs, at least)
/// be efficiently passed by value and returned by value (i.e. in registers),
/// and which meets the alignment requirements of `u32` and `u64` (at least)
/// for the target.
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Block {
    subblocks: [u64; 2],
}

/// Block length
pub(crate) const BLOCK_LEN: usize = 16;

impl Block {
    #[inline]
    pub(crate) fn zero() -> Self {
        Self { subblocks: [0, 0] }
    }
}

impl From<[u8; BLOCK_LEN]> for Block {
    #[inline]
    fn from(bytes: [u8; BLOCK_LEN]) -> Self {
        unsafe { core::mem::transmute(bytes) }
    }
}

impl AsRef<[u8; BLOCK_LEN]> for Block {
    #[allow(clippy::transmute_ptr_to_ptr)]
    #[inline]
    fn as_ref(&self) -> &[u8; BLOCK_LEN] {
        unsafe { core::mem::transmute(self) }
    }
}

impl AsMut<[u8; BLOCK_LEN]> for Block {
    #[allow(clippy::transmute_ptr_to_ptr)]
    #[inline]
    fn as_mut(&mut self) -> &mut [u8; BLOCK_LEN] {
        unsafe { core::mem::transmute(self) }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_block_clone() {
        use super::{Block, BLOCK_LEN};
        let block_a = Block::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
        #[allow(clippy::clone_on_copy)]
        let block_b = block_a.clone();

        for i in 0..BLOCK_LEN {
            assert_eq!(block_a.as_ref()[i], block_b.as_ref()[i]);
        }
    }

    #[test]
    fn test_block_clone_mut_ref() {
        use super::{Block, BLOCK_LEN};
        let mut block_a = Block::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
        #[allow(clippy::clone_on_copy)]
        let mut block_b = block_a.clone();

        for i in 0..BLOCK_LEN {
            assert_eq!(block_a.as_mut()[i], block_b.as_mut()[i]);
        }
    }
}
