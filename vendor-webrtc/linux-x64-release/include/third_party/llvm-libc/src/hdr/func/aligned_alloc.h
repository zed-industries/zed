//===-- Definition of the aligned_alloc.h proxy ---------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_HDR_FUNC_ALIGNED_ALLOC_H
#define LLVM_LIBC_HDR_FUNC_ALIGNED_ALLOC_H

#ifdef LIBC_FULL_BUILD
#include "hdr/types/size_t.h"
extern "C" void *aligned_alloc(size_t, size_t);

#else // Overlay mode

#include "hdr/stdlib_overlay.h"

#endif

#endif
