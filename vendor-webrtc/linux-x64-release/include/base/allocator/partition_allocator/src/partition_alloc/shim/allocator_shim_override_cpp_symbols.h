// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifdef PARTITION_ALLOC_SHIM_ALLOCATOR_SHIM_OVERRIDE_CPP_SYMBOLS_H_
#error This header is meant to be included only once by allocator_shim.cc
#endif

#ifndef PARTITION_ALLOC_SHIM_ALLOCATOR_SHIM_OVERRIDE_CPP_SYMBOLS_H_
#define PARTITION_ALLOC_SHIM_ALLOCATOR_SHIM_OVERRIDE_CPP_SYMBOLS_H_

#include "partition_alloc/buildflags.h"

#if PA_BUILDFLAG(USE_ALLOCATOR_SHIM)
// Preempt the default new/delete C++ symbols so they call the shim entry
// points. This file is strongly inspired by tcmalloc's
// libc_override_redefine.h.

#include <new>

#include "partition_alloc/build_config.h"
#include "partition_alloc/partition_alloc_base/compiler_specific.h"
#include "partition_alloc/shim/allocator_shim_internals.h"

#if !PA_BUILDFLAG(IS_APPLE)
#define SHIM_CPP_SYMBOLS_EXPORT SHIM_ALWAYS_EXPORT
#else
// On Apple OSes, prefer not exporting these symbols (as this reverts to the
// default behavior, they are still exported in e.g. component builds). This is
// partly due to intentional limits on exported symbols in the main library, but
// it is also needless, since no library used on macOS imports these.
//
// TODO(lizeb): It may not be necessary anywhere to export these.
#define SHIM_CPP_SYMBOLS_EXPORT PA_NOINLINE
#endif

SHIM_CPP_SYMBOLS_EXPORT void* operator new(size_t size) {
#if PA_BUILDFLAG(FORWARD_THROUGH_MALLOC)
  return malloc(size);
#else
  return ShimCppNew(size);
#endif
}

SHIM_CPP_SYMBOLS_EXPORT void operator delete(void* p) __THROW {
#if PA_BUILDFLAG(FORWARD_THROUGH_MALLOC)
  free(p);
#else
  ShimCppDelete(p);
#endif
}

SHIM_CPP_SYMBOLS_EXPORT void* operator new[](size_t size) {
#if PA_BUILDFLAG(FORWARD_THROUGH_MALLOC)
  return malloc(size);
#else
  return ShimCppNew(size);
#endif
}

SHIM_CPP_SYMBOLS_EXPORT void operator delete[](void* p) __THROW {
#if PA_BUILDFLAG(FORWARD_THROUGH_MALLOC)
  free(p);
#else
  ShimCppDelete(p);
#endif
}

SHIM_CPP_SYMBOLS_EXPORT void* operator new(size_t size,
                                           const std::nothrow_t&) __THROW {
#if PA_BUILDFLAG(FORWARD_THROUGH_MALLOC)
  return malloc(size);
#else
  return ShimCppNewNoThrow(size);
#endif
}

SHIM_CPP_SYMBOLS_EXPORT void* operator new[](size_t size,
                                             const std::nothrow_t&) __THROW {
#if PA_BUILDFLAG(FORWARD_THROUGH_MALLOC)
  return malloc(size);
#else
  return ShimCppNewNoThrow(size);
#endif
}

SHIM_CPP_SYMBOLS_EXPORT void operator delete(void* p,
                                             const std::nothrow_t&) __THROW {
#if PA_BUILDFLAG(FORWARD_THROUGH_MALLOC)
  free(p);
#else
  ShimCppDelete(p);
#endif
}

SHIM_CPP_SYMBOLS_EXPORT void operator delete[](void* p,
                                               const std::nothrow_t&) __THROW {
#if PA_BUILDFLAG(FORWARD_THROUGH_MALLOC)
  free(p);
#else
  ShimCppDelete(p);
#endif
}

SHIM_CPP_SYMBOLS_EXPORT void operator delete(void* p, size_t size) __THROW {
#if PA_BUILDFLAG(FORWARD_THROUGH_MALLOC)
  free(p);
#elif PA_BUILDFLAG(SHIM_SUPPORTS_SIZED_DEALLOC)
  ShimCppDeleteWithSize(p, size);
#else
  ShimCppDelete(p);
#endif
}

SHIM_CPP_SYMBOLS_EXPORT void operator delete[](void* p, size_t size) __THROW {
#if PA_BUILDFLAG(FORWARD_THROUGH_MALLOC)
  free(p);
#elif PA_BUILDFLAG(SHIM_SUPPORTS_SIZED_DEALLOC)
  ShimCppDeleteWithSize(p, size);
#else
  ShimCppDelete(p);
#endif
}

SHIM_CPP_SYMBOLS_EXPORT void* operator new(std::size_t size,
                                           std::align_val_t alignment) {
#if PA_BUILDFLAG(FORWARD_THROUGH_MALLOC)
  return aligned_alloc(static_cast<size_t>(alignment), size);
#else
  return ShimCppAlignedNew(size, static_cast<size_t>(alignment));
#endif
}

SHIM_CPP_SYMBOLS_EXPORT void* operator new(std::size_t size,
                                           std::align_val_t alignment,
                                           const std::nothrow_t&) __THROW {
#if PA_BUILDFLAG(FORWARD_THROUGH_MALLOC)
  return aligned_alloc(static_cast<size_t>(alignment), size);
#else
  return ShimCppAlignedNew(size, static_cast<size_t>(alignment));
#endif
}

SHIM_CPP_SYMBOLS_EXPORT void operator delete(
    void* p,
    std::align_val_t alignment) __THROW {
#if PA_BUILDFLAG(FORWARD_THROUGH_MALLOC)
  free(p);
#elif PA_BUILDFLAG(SHIM_SUPPORTS_SIZED_DEALLOC)
  ShimCppDeleteWithAlignment(p, static_cast<size_t>(alignment));
#else
  ShimCppDelete(p);
#endif
}

SHIM_CPP_SYMBOLS_EXPORT void
operator delete(void* p, std::size_t size, std::align_val_t alignment) __THROW {
#if PA_BUILDFLAG(FORWARD_THROUGH_MALLOC)
  free(p);
#elif PA_BUILDFLAG(SHIM_SUPPORTS_SIZED_DEALLOC)
  ShimCppDeleteWithSizeAndAlignment(p, size, static_cast<size_t>(alignment));
#else
  ShimCppDelete(p);
#endif
}

SHIM_CPP_SYMBOLS_EXPORT void operator delete(void* p,
                                             std::align_val_t alignment,
                                             const std::nothrow_t&) __THROW {
#if PA_BUILDFLAG(FORWARD_THROUGH_MALLOC)
  free(p);
#elif PA_BUILDFLAG(SHIM_SUPPORTS_SIZED_DEALLOC)
  ShimCppDeleteWithAlignment(p, static_cast<size_t>(alignment));
#else
  ShimCppDelete(p);
#endif
}

SHIM_CPP_SYMBOLS_EXPORT void* operator new[](std::size_t size,
                                             std::align_val_t alignment) {
#if PA_BUILDFLAG(FORWARD_THROUGH_MALLOC)
  return aligned_alloc(static_cast<size_t>(alignment), size);
#else
  return ShimCppAlignedNew(size, static_cast<size_t>(alignment));
#endif
}

SHIM_CPP_SYMBOLS_EXPORT void* operator new[](std::size_t size,
                                             std::align_val_t alignment,
                                             const std::nothrow_t&) __THROW {
#if PA_BUILDFLAG(FORWARD_THROUGH_MALLOC)
  return aligned_alloc(static_cast<size_t>(alignment), size);
#else
  return ShimCppAlignedNew(size, static_cast<size_t>(alignment));
#endif
}

SHIM_CPP_SYMBOLS_EXPORT void operator delete[](
    void* p,
    std::align_val_t alignment) __THROW {
#if PA_BUILDFLAG(FORWARD_THROUGH_MALLOC)
  free(p);
#elif PA_BUILDFLAG(SHIM_SUPPORTS_SIZED_DEALLOC)
  ShimCppDeleteWithAlignment(p, static_cast<size_t>(alignment));
#else
  ShimCppDelete(p);
#endif
}

SHIM_CPP_SYMBOLS_EXPORT void operator delete[](
    void* p,
    std::size_t size,
    std::align_val_t alignment) __THROW {
#if PA_BUILDFLAG(FORWARD_THROUGH_MALLOC)
  free(p);
#elif PA_BUILDFLAG(SHIM_SUPPORTS_SIZED_DEALLOC)
  ShimCppDeleteWithSizeAndAlignment(p, size, static_cast<size_t>(alignment));
#else
  ShimCppDelete(p);
#endif
}

SHIM_CPP_SYMBOLS_EXPORT void operator delete[](void* p,
                                               std::align_val_t alignment,
                                               const std::nothrow_t&) __THROW {
#if PA_BUILDFLAG(FORWARD_THROUGH_MALLOC)
  free(p);
#elif PA_BUILDFLAG(SHIM_SUPPORTS_SIZED_DEALLOC)
  ShimCppDeleteWithAlignment(p, static_cast<size_t>(alignment));
#else
  ShimCppDelete(p);
#endif
}

#endif  // PA_BUILDFLAG(USE_ALLOCATOR_SHIM)

#endif  // PARTITION_ALLOC_SHIM_ALLOCATOR_SHIM_OVERRIDE_CPP_SYMBOLS_H_
