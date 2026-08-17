// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_PARTITION_ALLOC_BASE_DEBUG_STACK_TRACE_H_
#define PARTITION_ALLOC_PARTITION_ALLOC_BASE_DEBUG_STACK_TRACE_H_

#include <cstddef>
#include <cstdint>

#include "partition_alloc/build_config.h"
#include "partition_alloc/buildflags.h"
#include "partition_alloc/partition_alloc_base/component_export.h"

namespace partition_alloc::internal::base::debug {

// Returns end of the stack, or 0 if we couldn't get it.
#if PA_BUILDFLAG(CAN_UNWIND_WITH_FRAME_POINTERS)
PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE)
uintptr_t GetStackEnd();
#endif

// Record a stack trace with up to |count| frames into |trace|. Returns the
// number of frames read.
PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE)
size_t CollectStackTrace(const void** trace, size_t count);

PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE)
void PrintStackTrace(const void** trace, size_t count);

#if PA_BUILDFLAG(IS_POSIX)
PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE)
void OutputStackTrace(unsigned index,
                      uintptr_t address,
                      uintptr_t base_address,
                      const char* module_name,
                      uintptr_t offset);
#endif

#if PA_BUILDFLAG(CAN_UNWIND_WITH_FRAME_POINTERS)

// For stack scanning to be efficient it's very important for the thread to
// be started by Chrome. In that case we naturally terminate unwinding once
// we reach the origin of the stack (i.e. GetStackEnd()). If the thread is
// not started by Chrome (e.g. Android's main thread), then we end up always
// scanning area at the origin of the stack, wasting time and not finding any
// frames (since Android libraries don't have frame pointers). Scanning is not
// enabled on other posix platforms due to legacy reasons.
#if PA_BUILDFLAG(IS_LINUX) || PA_BUILDFLAG(IS_CHROMEOS)
constexpr bool kEnableScanningByDefault = true;
#else
constexpr bool kEnableScanningByDefault = false;
#endif

// Traces the stack by using frame pointers. This function is faster but less
// reliable than StackTrace. It should work for debug and profiling builds,
// but not for release builds (although there are some exceptions).
//
// Writes at most |max_depth| frames (instruction pointers) into |out_trace|
// after skipping |skip_initial| frames. Note that the function itself is not
// added to the trace so |skip_initial| should be 0 in most cases.
// Returns number of frames written. |enable_scanning| enables scanning on
// platforms that do not enable scanning by default.
PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE)
size_t TraceStackFramePointers(const void** out_trace,
                               size_t max_depth,
                               size_t skip_initial,
                               bool enable_scanning = kEnableScanningByDefault);

#endif  // PA_BUILDFLAG(CAN_UNWIND_WITH_FRAME_POINTERS)

}  // namespace partition_alloc::internal::base::debug

#endif  // PARTITION_ALLOC_PARTITION_ALLOC_BASE_DEBUG_STACK_TRACE_H_
