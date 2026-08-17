// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_SHIM_ALLOCATOR_SHIM_INTERNALS_H_
#define PARTITION_ALLOC_SHIM_ALLOCATOR_SHIM_INTERNALS_H_

#include "partition_alloc/build_config.h"

#if defined(__GNUC__)

#if PA_BUILDFLAG(IS_POSIX)
#include <sys/cdefs.h>  // for __THROW
#endif

#ifndef __THROW   // Not a glibc system
#ifdef _NOEXCEPT  // LLVM libc++ uses noexcept instead
#define __THROW _NOEXCEPT
#else
#define __THROW
#endif  // !_NOEXCEPT
#endif

// Shim layer symbols need to be ALWAYS exported, regardless of component build.
//
// If an exported symbol is linked into a DSO, it may be preempted by a
// definition in the main executable. If this happens to an allocator symbol, it
// will mean that the DSO will use the main executable's allocator. This is
// normally relatively harmless -- regular allocations should all use the same
// allocator, but if the DSO tries to hook the allocator it will not see any
// allocations.
//
// However, if LLVM LTO is enabled, the compiler may inline the shim layer
// symbols into callers. The end result is that allocator calls in DSOs may use
// either the main executable's allocator or the DSO's allocator, depending on
// whether the call was inlined. This is arguably a bug in LLVM caused by its
// somewhat irregular handling of symbol interposition (see llvm.org/PR23501).
// To work around the bug we use noinline to prevent the symbols from being
// inlined.
//
// In the long run we probably want to avoid linking the allocator bits into
// DSOs altogether. This will save a little space and stop giving DSOs the false
// impression that they can hook the allocator.
#define SHIM_ALWAYS_EXPORT __attribute__((visibility("default"), noinline))

#elif PA_BUILDFLAG(IS_WIN)  // __GNUC__

#define __THROW
#define SHIM_ALWAYS_EXPORT __declspec(noinline)

#endif  // __GNUC__

#endif  // PARTITION_ALLOC_SHIM_ALLOCATOR_SHIM_INTERNALS_H_
