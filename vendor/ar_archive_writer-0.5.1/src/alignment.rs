// Derived from code in LLVM, which is:
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

/// Returns a multiple of `align` needed to store `size` bytes.
pub(crate) fn align_to(size: u64, align: u64) -> u64 {
    (size + align - 1) & !(align - 1)
}

/*
/// Returns the offset to the next integer (mod 2**64) that is greater than
/// or equal to \p Value and is a multiple of \p Align.
inline uint64_t offsetToAlignment(uint64_t Value, Align Alignment) {
    return alignTo(Value, Alignment) - Value;
}
*/

pub(crate) fn offset_to_alignment(value: u64, alignment: u64) -> u64 {
    align_to(value, alignment) - value
}
