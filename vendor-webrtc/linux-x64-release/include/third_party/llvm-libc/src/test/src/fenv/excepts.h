//===-- List of all FE_* constants for tests -----------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===---------------------------------------------------------------------===//

#ifndef LLVM_LIBC_TEST_SRC_FENV_EXCEPTS_H
#define LLVM_LIBC_TEST_SRC_FENV_EXCEPTS_H

#include "hdr/fenv_macros.h"

constexpr int EXCEPTS[] = {
    FE_DIVBYZERO, FE_INVALID, FE_INEXACT, FE_OVERFLOW, FE_UNDERFLOW,
};

// We '|' the individual exception flags instead of using FE_ALL_EXCEPT
// as it can include non-standard extensions. Note that we should be able
// to compile this file with headers from other libcs as well.
constexpr int ALL_EXCEPTS =
    FE_DIVBYZERO | FE_INVALID | FE_INEXACT | FE_OVERFLOW | FE_UNDERFLOW;

#endif // LLVM_LIBC_TEST_SRC_FENV_EXCEPTS_H
