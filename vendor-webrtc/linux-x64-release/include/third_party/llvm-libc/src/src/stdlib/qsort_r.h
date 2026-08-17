//===-- Implementation header for qsort_r -----------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_STDLIB_QSORT_R_H
#define LLVM_LIBC_SRC_STDLIB_QSORT_R_H

#include "hdr/types/size_t.h"
#include "src/__support/macros/config.h"

namespace LIBC_NAMESPACE_DECL {

// This qsort_r uses the glibc argument ordering instead of the BSD argument
// ordering (which puts arg before the function pointer). Putting arg after the
// function pointer more closely matches the ordering for qsort_s, which is the
// standardized equivalent of qsort_r.

void qsort_r(void *array, size_t array_size, size_t elem_size,
             int (*compare)(const void *, const void *, void *), void *arg);

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_STDLIB_QSORT_R_H
