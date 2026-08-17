// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_PROCESS_MEMORY_H_
#define BASE_PROCESS_MEMORY_H_

#include <stddef.h>

#include "base/base_export.h"
#include "base/check.h"
#include "base/process/process_handle.h"
#include "build/build_config.h"
#include "partition_alloc/buildflags.h"

#if PA_BUILDFLAG(USE_PARTITION_ALLOC)
#include "partition_alloc/oom.h"  // nogncheck
#endif

namespace base {

// Enables 'terminate on heap corruption' flag. Helps protect against heap
// overflow. Has no effect if the OS doesn't provide the necessary facility.
BASE_EXPORT void EnableTerminationOnHeapCorruption();

// Turns on process termination if memory runs out.
BASE_EXPORT void EnableTerminationOnOutOfMemory();

#if PA_BUILDFLAG(USE_PARTITION_ALLOC)
using partition_alloc::TerminateBecauseOutOfMemory;
#else
inline void TerminateBecauseOutOfMemory(size_t) {
  logging::RawCheckFailure("Out of memory");
}
#endif

#if BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS) || BUILDFLAG(IS_ANDROID) || \
    BUILDFLAG(IS_AIX)
// The maximum allowed value for the OOM score.
const int kMaxOomScore = 1000;

// This adjusts /proc/<pid>/oom_score_adj so the Linux OOM killer will
// prefer to kill certain process types over others. The range for the
// adjustment is [-1000, 1000], with [0, 1000] being user accessible.
// If the Linux system doesn't support the newer oom_score_adj range
// of [0, 1000], then we revert to using the older oom_adj, and
// translate the given value into [0, 15].  Some aliasing of values
// may occur in that case, of course.
BASE_EXPORT bool AdjustOOMScore(ProcessId process, int score);
#endif

namespace internal {
// Returns true if address-space was released. Some configurations reserve part
// of the process address-space for special allocations (e.g. WASM).
bool ReleaseAddressSpaceReservation();
}  // namespace internal

#if BUILDFLAG(IS_WIN)
namespace win {

using partition_alloc::win::kOomExceptionCode;

}  // namespace win
#endif

// Special allocator functions for callers that want to check for OOM.
// These will not abort if the allocation fails even if
// EnableTerminationOnOutOfMemory has been called.
// This can be useful for huge and/or unpredictable size memory allocations.
// Please only use this if you really handle the case when the allocation
// fails. Doing otherwise would risk security.
// These functions may still crash on OOM when running under memory tools,
// specifically ASan and other sanitizers.
// Return value tells whether the allocation succeeded. If it fails |result| is
// set to NULL, otherwise it holds the memory address.
//
// Note: You *must* use UncheckedFree() to free() the memory allocated, not
// regular free(). This also means that this a pointer allocated below cannot be
// passed to realloc().
[[nodiscard]] BASE_EXPORT bool UncheckedMalloc(size_t size, void** result);
[[nodiscard]] BASE_EXPORT bool UncheckedCalloc(size_t num_items,
                                               size_t size,
                                               void** result);

// *Must* be used to free memory allocated with base::UncheckedMalloc() and
// base::UncheckedCalloc().
// TODO(crbug.com/40208525): Enforce it, when all callers are converted.
BASE_EXPORT void UncheckedFree(void* ptr);

// Function object which invokes 'UncheckedFree' on its parameter, which should
// be a pointer resulting from UncheckedMalloc or UncheckedCalloc. Can be used
// to store such pointers in std::unique_ptr:
//
// int* foo_ptr = nullptr;
// if (UncheckedMalloc(sizeof(*foo_ptr), reinterpret_cast<void**>(&foo_ptr))) {
//   std::unique_ptr<int, base::UncheckedFreeDeleter> unique_foo_ptr(foo_ptr);
//   ...
// }
struct UncheckedFreeDeleter {
  inline void operator()(void* ptr) const { UncheckedFree(ptr); }
};

}  // namespace base

#endif  // BASE_PROCESS_MEMORY_H_
