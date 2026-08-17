// Derived from code in LLVM, which is:
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub(crate) const fn align_to_power_of2(value: u64, align: u64) -> u64 {
    assert!(
        align != 0 && (align & (align - 1)) == 0,
        "Align must be a power of 2"
    );
    // Replace unary minus to avoid compilation error on Windows:
    // "unary minus operator applied to unsigned type, result still unsigned"
    let neg_align = align.wrapping_neg();
    (value + align - 1) & neg_align
}
