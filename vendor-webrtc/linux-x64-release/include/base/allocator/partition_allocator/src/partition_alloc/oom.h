// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_OOM_H_
#define PARTITION_ALLOC_OOM_H_

#include <cstddef>

#include "partition_alloc/allocation_guard.h"
#include "partition_alloc/build_config.h"
#include "partition_alloc/partition_alloc_base/compiler_specific.h"
#include "partition_alloc/partition_alloc_base/component_export.h"

#if PA_BUILDFLAG(IS_WIN)
#include "partition_alloc/partition_alloc_base/win/windows_types.h"
#endif

namespace partition_alloc {

// Terminates process. Should be called only for out of memory errors.
// |size| is the size of the failed allocation, or 0 if not known.
// Crash reporting classifies such crashes as OOM.
// Must be allocation-safe.
[[noreturn]] PA_NOT_TAIL_CALLED PA_COMPONENT_EXPORT(
    PARTITION_ALLOC) void TerminateBecauseOutOfMemory(size_t size);

// Records the size of the allocation that caused the current OOM crash, for
// consumption by Breakpad.
// TODO: this can be removed when Breakpad is no longer supported.
PA_COMPONENT_EXPORT(PARTITION_ALLOC) extern size_t g_oom_size;

#if PA_BUILDFLAG(IS_WIN)
namespace win {

// Custom Windows exception code chosen to indicate an out of memory error.
// See https://msdn.microsoft.com/en-us/library/het71c37.aspx.
// "To make sure that you do not define a code that conflicts with an existing
// exception code" ... "The resulting error code should therefore have the
// highest four bits set to hexadecimal E."
// 0xe0000008 was chosen arbitrarily, as 0x00000008 is ERROR_NOT_ENOUGH_MEMORY.
const DWORD kOomExceptionCode = 0xe0000008;

}  // namespace win
#endif

namespace internal {

// The crash is generated in a PA_NOINLINE function so that we can classify the
// crash as an OOM solely by analyzing the stack trace. It is tagged as
// PA_NOT_TAIL_CALLED to ensure that its parent function stays on the stack.
[[noreturn]] PA_NOT_TAIL_CALLED PA_COMPONENT_EXPORT(
    PARTITION_ALLOC) void OnNoMemory(size_t size);

#if PA_BUILDFLAG(IS_POSIX)
// See above for annotations.
//
// THis is used to identify cases where the kernel return ENOMEM for memory
// management calls. This can indicate several things, but in particular on
// Linux that the current process has exceeded the per-process VMA limit.
[[noreturn]] PA_NOT_TAIL_CALLED PA_COMPONENT_EXPORT(
    PARTITION_ALLOC) void OnErrnoNoMem();
#endif

// OOM_CRASH(size) - Specialization of IMMEDIATE_CRASH which will raise a custom
// exception on Windows to signal this is OOM and not a normal assert.
// OOM_CRASH(size) is called by users of PageAllocator (including
// PartitionAlloc) to signify an allocation failure from the platform.
#define OOM_CRASH(size)                                     \
  do {                                                      \
    /* Raising an exception might allocate, allow that.  */ \
    ::partition_alloc::ScopedAllowAllocations guard{};      \
    ::partition_alloc::internal::OnNoMemory(size);          \
  } while (0)

}  // namespace internal

}  // namespace partition_alloc

#endif  // PARTITION_ALLOC_OOM_H_
