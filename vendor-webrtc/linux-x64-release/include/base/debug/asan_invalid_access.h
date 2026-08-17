// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.
//
// Defines some functions that intentionally do an invalid memory access in
// order to trigger an AddressSanitizer (ASan) error report.

#ifndef BASE_DEBUG_ASAN_INVALID_ACCESS_H_
#define BASE_DEBUG_ASAN_INVALID_ACCESS_H_

#include "base/base_export.h"
#include "base/compiler_specific.h"
#include "base/sanitizer_buildflags.h"
#include "build/build_config.h"

namespace base {
namespace debug {

#if defined(ADDRESS_SANITIZER) || BUILDFLAG(IS_HWASAN)

// Generates an heap buffer overflow.
NOINLINE BASE_EXPORT void AsanHeapOverflow();

// Generates an heap buffer underflow.
NOINLINE BASE_EXPORT void AsanHeapUnderflow();

// Generates an use after free.
NOINLINE BASE_EXPORT void AsanHeapUseAfterFree();

// The "corrupt-block" and "corrupt-heap" classes of bugs is specific to
// Windows.
#if BUILDFLAG(IS_WIN)
// Corrupts a memory block and makes sure that the corruption gets detected when
// we try to free this block.
NOINLINE BASE_EXPORT void AsanCorruptHeapBlock();

// Corrupts the heap and makes sure that the corruption gets detected when a
// crash occur.
NOINLINE BASE_EXPORT void AsanCorruptHeap();

#endif  // BUILDFLAG(IS_WIN)
#endif  // ADDRESS_SANITIZER

}  // namespace debug
}  // namespace base

#endif  // BASE_DEBUG_ASAN_INVALID_ACCESS_H_
