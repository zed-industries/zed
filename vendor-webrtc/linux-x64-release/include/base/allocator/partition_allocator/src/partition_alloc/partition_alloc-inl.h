// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_PARTITION_ALLOC_INL_H_
#define PARTITION_ALLOC_PARTITION_ALLOC_INL_H_

#include <cstring>

#include "partition_alloc/build_config.h"
#include "partition_alloc/buildflags.h"
#include "partition_alloc/in_slot_metadata.h"
#include "partition_alloc/partition_alloc_base/compiler_specific.h"
#include "partition_alloc/partition_alloc_base/cxx_wrapper/algorithm.h"
#include "partition_alloc/partition_alloc_config.h"
#include "partition_alloc/random.h"
#include "partition_alloc/tagging.h"
#include "partition_alloc/thread_isolation/thread_isolation.h"

// Prefetch *x into memory.
#if defined(__clang__) || PA_BUILDFLAG(PA_COMPILER_GCC)
#define PA_PREFETCH(x) __builtin_prefetch(x, 0)
#else
#define PA_PREFETCH(x)
#endif

#if defined(__clang__) || PA_BUILDFLAG(PA_COMPILER_GCC)
#define PA_PREFETCH_FOR_WRITE(x) __builtin_prefetch(x, 1)
#else
#define PA_PREFETCH_FOR_WRITE(x)
#endif

namespace partition_alloc::internal {

// This is a `memset` that resists being optimized away. Adapted from
// boringssl/src/crypto/mem.c. (Copying and pasting is bad, but //base can't
// depend on //third_party, and this is small enough.)
#if PA_CONFIG(IS_NONCLANG_MSVC)
// MSVC only supports inline assembly on x86. This preprocessor directive
// is intended to be a replacement for the same.
//
// TODO(crbug.com/40234441): Make sure inlining doesn't degrade this into
// a no-op or similar. The documentation doesn't say.
#pragma optimize("", off)
#endif
PA_ALWAYS_INLINE void SecureMemset(void* ptr, uint8_t value, size_t size) {
  memset(ptr, value, size);

#if !PA_CONFIG(IS_NONCLANG_MSVC)
  // As best as we can tell, this is sufficient to break any optimisations that
  // might try to eliminate "superfluous" memsets. If there's an easy way to
  // detect memset_s, it would be better to use that.
  __asm__ __volatile__("" : : "r"(ptr) : "memory");
#endif  // !PA_CONFIG(IS_NONCLANG_MSVC)
}
#if PA_CONFIG(IS_NONCLANG_MSVC)
#pragma optimize("", on)
#endif

#if PA_BUILDFLAG(EXPENSIVE_DCHECKS_ARE_ON)
// Used to memset() memory for debugging purposes only.
PA_ALWAYS_INLINE void DebugMemset(void* ptr, int value, size_t size) {
  // Only set the first 512kiB of the allocation. This is enough to detect uses
  // of uininitialized / freed memory, and makes tests run significantly
  // faster. Note that for direct-mapped allocations, memory is decomitted at
  // free() time, so freed memory usage cannot happen.

#if PA_BUILDFLAG(ENABLE_THREAD_ISOLATION) && PA_BUILDFLAG(ENABLE_PKEYS)
  LiftThreadIsolationScope lift_thread_isolation_restrictions;
#endif
  size_t size_to_memset = std::min(size, size_t{1} << 19);
  memset(ptr, value, size_to_memset);
}
#endif  // PA_BUILDFLAG(EXPENSIVE_DCHECKS_ARE_ON)

// Returns true if we've hit the end of a random-length period. We don't want to
// invoke `RandomValue` too often, because we call this function in a hot spot
// (`Free`), and `RandomValue` incurs the cost of atomics.
#if !PA_BUILDFLAG(DCHECKS_ARE_ON)
PA_ALWAYS_INLINE bool RandomPeriod() {
  static thread_local uint8_t counter = 0;
  if (counter == 0) [[unlikely]] {
    // It's OK to truncate this value.
    counter = static_cast<uint8_t>(RandomValue());
  }
  // If `counter` is 0, this will wrap. That is intentional and OK.
  counter--;
  return counter == 0;
}
#endif  // !PA_BUILDFLAG(DCHECKS_ARE_ON)

PA_ALWAYS_INLINE uintptr_t ObjectInnerPtr2Addr(const void* ptr) {
  return UntagPtr(ptr);
}
PA_ALWAYS_INLINE uintptr_t ObjectPtr2Addr(const void* object) {
  // TODO(bartekn): Check that |object| is indeed an object start.
  return ObjectInnerPtr2Addr(object);
}
PA_ALWAYS_INLINE void* SlotStartAddr2Ptr(uintptr_t slot_start) {
  // TODO(bartekn): Check that |slot_start| is indeed a slot start.
  return TagAddr(slot_start);
}
PA_ALWAYS_INLINE uintptr_t SlotStartPtr2Addr(const void* slot_start) {
  // TODO(bartekn): Check that |slot_start| is indeed a slot start.
  return UntagPtr(slot_start);
}

}  // namespace partition_alloc::internal

#endif  // PARTITION_ALLOC_PARTITION_ALLOC_INL_H_
