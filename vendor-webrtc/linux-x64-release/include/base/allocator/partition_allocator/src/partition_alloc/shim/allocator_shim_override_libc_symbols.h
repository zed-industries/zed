// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Its purpose is to preempt the Libc symbols for malloc/new so they call the
// shim layer entry points.

#ifdef PARTITION_ALLOC_SHIM_ALLOCATOR_SHIM_OVERRIDE_LIBC_SYMBOLS_H_
#error This header is meant to be included only once by allocator_shim.cc
#endif

#ifndef PARTITION_ALLOC_SHIM_ALLOCATOR_SHIM_OVERRIDE_LIBC_SYMBOLS_H_
#define PARTITION_ALLOC_SHIM_ALLOCATOR_SHIM_OVERRIDE_LIBC_SYMBOLS_H_

#include "partition_alloc/buildflags.h"

#if PA_BUILDFLAG(USE_ALLOCATOR_SHIM)
#include "partition_alloc/build_config.h"

#if PA_BUILDFLAG(IS_APPLE)
#include <malloc/malloc.h>
#else
#include <malloc.h>
#endif

#include "partition_alloc/shim/allocator_shim_internals.h"

extern "C" {

// WARNING: Whenever a new function is added there (which, surprisingly enough,
// happens. For instance glibc 2.33 introduced mallinfo2(), which we don't
// support... yet?), it MUST be added to build/linux/chrome.map.
//
// Otherwise the new symbol is not exported from Chromium's main binary, which
// is necessary to override libc's weak symbol, which in turn is necessary to
// intercept calls made by dynamic libraries. See crbug.com/1292206 for such
// an example.

SHIM_ALWAYS_EXPORT void* malloc(size_t size) __THROW {
  return ShimMalloc(size, nullptr);
}

SHIM_ALWAYS_EXPORT void free(void* ptr) __THROW {
  ShimFree(ptr, nullptr);
}

SHIM_ALWAYS_EXPORT void* realloc(void* ptr, size_t size) __THROW {
  return ShimRealloc(ptr, size, nullptr);
}

SHIM_ALWAYS_EXPORT void* calloc(size_t n, size_t size) __THROW {
  return ShimCalloc(n, size, nullptr);
}

SHIM_ALWAYS_EXPORT void cfree(void* ptr) __THROW {
  ShimFree(ptr, nullptr);
}

SHIM_ALWAYS_EXPORT void* memalign(size_t align, size_t s) __THROW {
  return ShimMemalign(align, s, nullptr);
}

SHIM_ALWAYS_EXPORT void* aligned_alloc(size_t align, size_t s) __THROW {
  return ShimMemalign(align, s, nullptr);
}

SHIM_ALWAYS_EXPORT void* valloc(size_t size) __THROW {
  return ShimValloc(size, nullptr);
}

SHIM_ALWAYS_EXPORT void* pvalloc(size_t size) __THROW {
  return ShimPvalloc(size);
}

SHIM_ALWAYS_EXPORT int posix_memalign(void** r, size_t a, size_t s) __THROW {
  return ShimPosixMemalign(r, a, s);
}

SHIM_ALWAYS_EXPORT size_t malloc_size(const void* address) __THROW {
  return ShimGetSizeEstimate(address, nullptr);
}

SHIM_ALWAYS_EXPORT size_t malloc_usable_size(void* address) __THROW {
  return ShimGetSizeEstimate(address, nullptr);
}

// The default dispatch translation unit has to define also the following
// symbols (unless they are ultimately routed to the system symbols):
//   void malloc_stats(void);
//   int mallopt(int, int);
//   struct mallinfo mallinfo(void);

}  // extern "C"

#endif  // PA_BUILDFLAG(USE_ALLOCATOR_SHIM)

#endif  // PARTITION_ALLOC_SHIM_ALLOCATOR_SHIM_OVERRIDE_LIBC_SYMBOLS_H_
