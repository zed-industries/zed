// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifdef PARTITION_ALLOC_SHIM_ALLOCATOR_SHIM_OVERRIDE_GLIBC_WEAK_SYMBOLS_H_
#error This header is meant to be included only once by allocator_shim.cc
#endif

#ifndef PARTITION_ALLOC_SHIM_ALLOCATOR_SHIM_OVERRIDE_GLIBC_WEAK_SYMBOLS_H_
#define PARTITION_ALLOC_SHIM_ALLOCATOR_SHIM_OVERRIDE_GLIBC_WEAK_SYMBOLS_H_

// Alias the internal Glibc symbols to the shim entry points.
// This file is strongly inspired by tcmalloc's libc_override_glibc.h.
// Effectively this file does two things:
//  1) Re-define the  __malloc_hook & co symbols. Those symbols are defined as
//     weak in glibc and are meant to be defined strongly by client processes
//     to hook calls initiated from within glibc.
//  2) Re-define Glibc-specific symbols (__libc_malloc). The historical reason
//     is that in the past (in RedHat 9) we had instances of libraries that were
//     allocating via malloc() and freeing using __libc_free().
//     See tcmalloc's libc_override_glibc.h for more context.

#include "partition_alloc/buildflags.h"

#if PA_BUILDFLAG(USE_ALLOCATOR_SHIM)
#include <features.h>  // for __GLIBC__
#include <malloc.h>
#include <unistd.h>

#include <new>

#include "partition_alloc/shim/allocator_shim_internals.h"

// __MALLOC_HOOK_VOLATILE not defined in all Glibc headers.
#if !defined(__MALLOC_HOOK_VOLATILE)
#define MALLOC_HOOK_MAYBE_VOLATILE /**/
#else
#define MALLOC_HOOK_MAYBE_VOLATILE __MALLOC_HOOK_VOLATILE
#endif

extern "C" {

// 1) Re-define malloc_hook weak symbols.

// This ".h" file is not a header, but a source file meant to be included only
// once, exclusively from allocator_shim.cc. See the top-level check.
//
// A possible alternative: rename this file to .inc, at the expense of losing
// syntax highlighting in text editors.
//
// NOLINTNEXTLINE(google-build-namespaces)
namespace {

void* GlibcMallocHook(size_t size, const void* caller) {
  return ShimMalloc(size, nullptr);
}

void* GlibcReallocHook(void* ptr, size_t size, const void* caller) {
  return ShimRealloc(ptr, size, nullptr);
}

void GlibcFreeHook(void* ptr, const void* caller) {
  return ShimFree(ptr, nullptr);
}

void* GlibcMemalignHook(size_t align, size_t size, const void* caller) {
  return ShimMemalign(align, size, nullptr);
}

}  // namespace

__attribute__((visibility("default"))) void* (
    *MALLOC_HOOK_MAYBE_VOLATILE __malloc_hook)(size_t,
                                               const void*) = &GlibcMallocHook;

__attribute__((visibility("default"))) void* (
    *MALLOC_HOOK_MAYBE_VOLATILE __realloc_hook)(void*, size_t, const void*) =
    &GlibcReallocHook;

__attribute__((visibility("default"))) void (
    *MALLOC_HOOK_MAYBE_VOLATILE __free_hook)(void*,
                                             const void*) = &GlibcFreeHook;

__attribute__((visibility("default"))) void* (
    *MALLOC_HOOK_MAYBE_VOLATILE __memalign_hook)(size_t, size_t, const void*) =
    &GlibcMemalignHook;

// 2) Redefine libc symbols themselves.

SHIM_ALWAYS_EXPORT void* __libc_malloc(size_t size) {
  return ShimMalloc(size, nullptr);
}

SHIM_ALWAYS_EXPORT void __libc_free(void* ptr) {
  ShimFree(ptr, nullptr);
}

SHIM_ALWAYS_EXPORT void* __libc_realloc(void* ptr, size_t size) {
  return ShimRealloc(ptr, size, nullptr);
}

SHIM_ALWAYS_EXPORT void* __libc_calloc(size_t n, size_t size) {
  return ShimCalloc(n, size, nullptr);
}

SHIM_ALWAYS_EXPORT void __libc_cfree(void* ptr) {
  return ShimFree(ptr, nullptr);
}

SHIM_ALWAYS_EXPORT void* __libc_memalign(size_t align, size_t s) {
  return ShimMemalign(align, s, nullptr);
}

SHIM_ALWAYS_EXPORT void* __libc_valloc(size_t size) {
  return ShimValloc(size, nullptr);
}

SHIM_ALWAYS_EXPORT void* __libc_pvalloc(size_t size) {
  return ShimPvalloc(size);
}

SHIM_ALWAYS_EXPORT int __posix_memalign(void** r, size_t a, size_t s) {
  return ShimPosixMemalign(r, a, s);
}

}  // extern "C"

// Safety check.
#if !defined(__GLIBC__)
#error The target platform does not seem to use Glibc. Disable the allocator \
shim by setting use_allocator_shim=false in GN args.
#endif

#endif  // PA_BUILDFLAG(USE_ALLOCATOR_SHIM)

#endif  // PARTITION_ALLOC_SHIM_ALLOCATOR_SHIM_OVERRIDE_GLIBC_WEAK_SYMBOLS_H_
