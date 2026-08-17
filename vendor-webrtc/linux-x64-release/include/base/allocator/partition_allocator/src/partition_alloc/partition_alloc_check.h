// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_PARTITION_ALLOC_CHECK_H_
#define PARTITION_ALLOC_PARTITION_ALLOC_CHECK_H_

#include <cstdint>

#include "partition_alloc/build_config.h"
#include "partition_alloc/buildflags.h"
#include "partition_alloc/page_allocator_constants.h"
#include "partition_alloc/partition_alloc_base/check.h"
#include "partition_alloc/partition_alloc_base/compiler_specific.h"
#include "partition_alloc/partition_alloc_base/debug/alias.h"
#include "partition_alloc/partition_alloc_base/immediate_crash.h"

// When PartitionAlloc is used as the default allocator, we cannot use the
// regular (D)CHECK() macros, as they allocate internally. When an assertion is
// triggered, they format strings, leading to reentrancy in the code, which none
// of PartitionAlloc is designed to support (and especially not for error
// paths).
//
// As a consequence:
// - When PartitionAlloc is not malloc(), use the regular macros
// - Otherwise, crash immediately. This provides worse error messages though.
#if PA_BUILDFLAG(USE_PARTITION_ALLOC_AS_MALLOC) && !PA_BASE_CHECK_WILL_STREAM()

// For official build discard log strings to reduce binary bloat.
// See base/check.h for implementation details.
#define PA_CHECK(condition) PA_BASE_CHECK(condition)

#if PA_BUILDFLAG(DCHECKS_ARE_ON)
#define PA_DCHECK(condition) PA_CHECK(condition)
#else
#define PA_DCHECK(condition) PA_EAT_CHECK_STREAM_PARAMS(!(condition))
#endif  // PA_BUILDFLAG(DCHECKS_ARE_ON)

#define PA_PCHECK(condition)                                 \
  if (!(condition)) {                                        \
    int error = errno;                                       \
    ::partition_alloc::internal::base::debug::Alias(&error); \
    PA_IMMEDIATE_CRASH();                                    \
  }                                                          \
  static_assert(true)

#if PA_BUILDFLAG(DCHECKS_ARE_ON)
#define PA_DPCHECK(condition) PA_PCHECK(condition)
#else
#define PA_DPCHECK(condition) PA_EAT_CHECK_STREAM_PARAMS(!(condition))
#endif  // PA_BUILDFLAG(DCHECKS_ARE_ON)

#else  // PA_BUILDFLAG(USE_PARTITION_ALLOC_AS_MALLOC) &&
       // !PA_BASE_CHECK_WILL_STREAM()
#define PA_CHECK(condition) PA_BASE_CHECK(condition)
#define PA_DCHECK(condition) PA_BASE_DCHECK(condition)
#define PA_PCHECK(condition) PA_BASE_PCHECK(condition)
#define PA_DPCHECK(condition) PA_BASE_DPCHECK(condition)
#endif  // PA_BUILDFLAG(USE_PARTITION_ALLOC_AS_MALLOC) &&
        // !PA_BASE_CHECK_WILL_STREAM()

// Expensive dchecks that run within *Scan. These checks are only enabled in
// debug builds with dchecks enabled.
#if PA_BUILDFLAG(IS_DEBUG)
#define PA_SCAN_DCHECK_IS_ON() PA_BUILDFLAG(DCHECKS_ARE_ON)
#else
#define PA_SCAN_DCHECK_IS_ON() 0
#endif  // PA_BUILDFLAG(IS_DEBUG)

#if PA_SCAN_DCHECK_IS_ON()
#define PA_SCAN_DCHECK(expr) PA_DCHECK(expr)
#else
#define PA_SCAN_DCHECK(expr) PA_EAT_CHECK_STREAM_PARAMS(!(expr))
#endif

#if defined(PAGE_ALLOCATOR_CONSTANTS_ARE_CONSTEXPR)

// Use this macro to assert on things that are conditionally constexpr as
// determined by PAGE_ALLOCATOR_CONSTANTS_ARE_CONSTEXPR or
// PAGE_ALLOCATOR_CONSTANTS_DECLARE_CONSTEXPR. Where fixed at compile time, this
// is a static_assert. Where determined at run time, this is a PA_CHECK.
// Therefore, this macro must only be used where both a static_assert and a
// PA_CHECK would be viable, that is, within a function, and ideally a function
// that executes only once, early in the program, such as during initialization.
#define STATIC_ASSERT_OR_PA_CHECK(condition, message) \
  static_assert(condition, message)

#else

#define STATIC_ASSERT_OR_PA_CHECK(condition, message) \
  do {                                                \
    PA_CHECK(condition) << (message);                 \
  } while (false)

#endif

// alignas(16) DebugKv causes breakpad_unittests and sandbox_linux_unittests
// failures on android-marshmallow-x86-rel because of SIGSEGV.
#if PA_BUILDFLAG(IS_ANDROID) && PA_BUILDFLAG(PA_ARCH_CPU_X86_FAMILY) && \
    PA_BUILDFLAG(PA_ARCH_CPU_32_BITS)
#define PA_DEBUGKV_ALIGN alignas(8)
#else
#define PA_DEBUGKV_ALIGN alignas(16)
#endif

namespace partition_alloc::internal {

static constexpr size_t kDebugKeyMaxLength = 8ull;

// Used for PA_DEBUG_DATA_ON_STACK, below.
struct PA_DEBUGKV_ALIGN DebugKv {
  // 16 bytes object aligned on 16 bytes, to make it easier to see in crash
  // reports.
  char k[kDebugKeyMaxLength] = {};  // Not necessarily 0-terminated.
  uint64_t v = 0;

  DebugKv(const char* key, uint64_t value) : v(value) {
    // Fill with ' ', so that the stack dump is nicer to read.  Not using
    // memset() on purpose, this header is included from *many* places.
    for (size_t index = 0; index < sizeof k; index++) {
      k[index] = ' ';
    }

    for (size_t index = 0; index < sizeof k; index++) {
      k[index] = key[index];
      if (key[index] == '\0') {
        break;
      }
    }
  }
};

}  // namespace partition_alloc::internal

#define PA_CONCAT(x, y) x##y
#define PA_CONCAT2(x, y) PA_CONCAT(x, y)
#define PA_DEBUG_UNIQUE_NAME PA_CONCAT2(kv, __LINE__)

// Puts a key-value pair on the stack for debugging. `base::debug::Alias()`
// makes sure a local variable is saved on the stack, but the variables can be
// hard to find in crash reports, particularly if the frame pointer is not
// present / invalid.
//
// This puts a key right before the value on the stack. The key has to be a C
// string, which gets truncated if it's longer than 8 characters.
// Example use:
// PA_DEBUG_DATA_ON_STACK("size", 0x42)
//
// Sample output in lldb:
// (lldb) x 0x00007fffffffd0d0 0x00007fffffffd0f0
// 0x7fffffffd0d0: 73 69 7a 65 00 00 00 00 42 00 00 00 00 00 00 00
// size............
//
// With gdb, one can use:
// x/8g <STACK_POINTER>
// to see the data. With lldb, "x <STACK_POINTER> <FRAME_POJNTER>" can be used.
#define PA_DEBUG_DATA_ON_STACK(name, value)                               \
  static_assert(sizeof name <=                                            \
                ::partition_alloc::internal::kDebugKeyMaxLength + 1);     \
  ::partition_alloc::internal::DebugKv PA_DEBUG_UNIQUE_NAME{name, value}; \
  ::partition_alloc::internal::base::debug::Alias(&PA_DEBUG_UNIQUE_NAME)

#endif  // PARTITION_ALLOC_PARTITION_ALLOC_CHECK_H_
