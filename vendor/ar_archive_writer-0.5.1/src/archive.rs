// Derived from code in LLVM, which is:
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

/// Size field is 10 decimal digits long
pub(crate) const MAX_MEMBER_SIZE: u64 = 9999999999;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ArchiveKind {
    Gnu,
    Gnu64,
    Bsd,
    Darwin,
    Darwin64,
    Coff,
    AixBig,
}

pub(crate) mod big_archive {
    #[repr(C)]
    pub(crate) struct BigArMemHdrType {
        /// File member size in decimal
        size: [u8; 20],

        /// Next member offset in decimal
        next_offset: [u8; 20],

        /// Previous member offset in decimal
        prev_offset: [u8; 20],

        last_modified: [u8; 12],

        uid: [u8; 12],
        gid: [u8; 12],

        access_mode: [u8; 12],

        /// File member name length in decimal
        name_len: [u8; 4],

        terminator: [u8; 2],
    }

    /// Fixed-Length Header.
    #[repr(C)]
    pub(crate) struct FixLenHdr {
        /// Big archive magic string.
        magic: [u8; 8],

        /// Offset to member table.
        mem_offset: [u8; 20],

        /// Offset to global symbol table.
        glob_sym_offset: [u8; 20],

        /// Offset global symbol table for 64-bit objects.
        glob_sym64_offset: [u8; 20],

        /// Offset to first archive member.
        first_child_offset: [u8; 20],

        /// Offset to last archive member.
        last_child_offset: [u8; 20],

        /// Offset to first mem on free list.
        free_offset: [u8; 20],
    }
}
