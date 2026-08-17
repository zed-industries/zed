// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifdef PARTITION_ALLOC_SHIM_ALLOCATOR_SHIM_OVERRIDE_LINKER_WRAPPED_SYMBOLS_H_
#error This header is meant to be included only once by allocator_shim.cc
#endif

#ifndef PARTITION_ALLOC_SHIM_ALLOCATOR_SHIM_OVERRIDE_LINKER_WRAPPED_SYMBOLS_H_
#define PARTITION_ALLOC_SHIM_ALLOCATOR_SHIM_OVERRIDE_LINKER_WRAPPED_SYMBOLS_H_

// This header overrides the __wrap_X symbols when using the link-time
// -Wl,-wrap,malloc shim-layer approach (see README.md).
// All references to malloc, free, etc. within the linker unit that gets the
// -wrap linker flags (e.g., libchrome.so) will be rewritten to the
// linker as references to __wrap_malloc, __wrap_free, which are defined here.

#include "partition_alloc/buildflags.h"

#if PA_BUILDFLAG(USE_ALLOCATOR_SHIM)
#include <stdlib.h>

#include <algorithm>
#include <cstring>

#include "partition_alloc/shim/allocator_shim_internals.h"

extern "C" {

SHIM_ALWAYS_EXPORT void* __wrap_aligned_alloc(size_t alignment, size_t size) {
  return ShimMemalign(alignment, size, nullptr);
}

SHIM_ALWAYS_EXPORT void* __wrap_calloc(size_t n, size_t size) {
  return ShimCalloc(n, size, nullptr);
}

SHIM_ALWAYS_EXPORT void __wrap_free(void* ptr) {
  ShimFree(ptr, nullptr);
}

SHIM_ALWAYS_EXPORT void* __wrap_malloc(size_t size) {
  return ShimMalloc(size, nullptr);
}

SHIM_ALWAYS_EXPORT void* __wrap_memalign(size_t align, size_t size) {
  return ShimMemalign(align, size, nullptr);
}

SHIM_ALWAYS_EXPORT int __wrap_posix_memalign(void** res,
                                             size_t align,
                                             size_t size) {
  return ShimPosixMemalign(res, align, size);
}

SHIM_ALWAYS_EXPORT void* __wrap_pvalloc(size_t size) {
  return ShimPvalloc(size);
}

SHIM_ALWAYS_EXPORT void* __wrap_realloc(void* address, size_t size) {
  return ShimRealloc(address, size, nullptr);
}

SHIM_ALWAYS_EXPORT void* __wrap_valloc(size_t size) {
  return ShimValloc(size, nullptr);
}

SHIM_ALWAYS_EXPORT size_t __wrap_malloc_usable_size(void* address) {
  return ShimGetSizeEstimate(address, nullptr);
}

inline constexpr size_t kPathMaxSize = 8192;
static_assert(kPathMaxSize >= PATH_MAX, "");

extern char* __wrap_strdup(const char* str);

// Override <cstdlib>

extern char* __real_realpath(const char* path, char* resolved_path);

SHIM_ALWAYS_EXPORT char* __wrap_realpath(const char* path,
                                         char* resolved_path) {
  if (resolved_path) {
    return __real_realpath(path, resolved_path);
  }

  char buffer[kPathMaxSize];
  if (!__real_realpath(path, buffer)) {
    return nullptr;
  }
  return __wrap_strdup(buffer);
}

// Override <cstring> functions

SHIM_ALWAYS_EXPORT char* __wrap_strdup(const char* str) {
  std::size_t length = std::strlen(str) + 1;
  void* buffer = ShimMalloc(length, nullptr);
  if (!buffer) {
    return nullptr;
  }
  return reinterpret_cast<char*>(std::memcpy(buffer, str, length));
}

SHIM_ALWAYS_EXPORT char* __wrap_strndup(const char* str, size_t n) {
  std::size_t length = std::min(std::strlen(str), n);
  char* buffer = reinterpret_cast<char*>(ShimMalloc(length + 1, nullptr));
  if (!buffer) {
    return nullptr;
  }
  std::memcpy(buffer, str, length);
  buffer[length] = '\0';
  return buffer;
}

// Override <unistd.h>

extern char* __real_getcwd(char* buffer, size_t size);

SHIM_ALWAYS_EXPORT char* __wrap_getcwd(char* buffer, size_t size) {
  if (buffer) {
    return __real_getcwd(buffer, size);
  }

  if (!size) {
    size = kPathMaxSize;
  }
  char local_buffer[kPathMaxSize];
  if (!__real_getcwd(local_buffer, size)) {
    return nullptr;
  }
  return __wrap_strdup(local_buffer);
}

// Override stdio.h

// This is non-standard (_GNU_SOURCE only), but implemented by Bionic on
// Android, and used by libc++.
SHIM_ALWAYS_EXPORT int __wrap_vasprintf(char** strp,
                                        const char* fmt,
                                        va_list va_args) {
  // There are cases where we need to use the list of arguments twice, namely
  // when the original buffer is too small. It is not allowed to walk the list
  // twice, so make a copy for the second invocation of vsnprintf().
  va_list va_args_copy;
  va_copy(va_args_copy, va_args);

  constexpr int kInitialSize = 128;
  *strp = static_cast<char*>(
      malloc(kInitialSize));  // Our malloc() doesn't return nullptr.

  int actual_size = vsnprintf(*strp, kInitialSize, fmt, va_args);
  if (actual_size < 0) {
    va_end(va_args_copy);
    return actual_size;
  }
  *strp =
      static_cast<char*>(realloc(*strp, static_cast<size_t>(actual_size + 1)));

  // Now we know the size. This is not very efficient, but we cannot really do
  // better without accessing internal libc functions, or reimplementing
  // *printf().
  //
  // This is very lightly used in Chromium in practice, see crbug.com/116558 for
  // details.
  if (actual_size >= kInitialSize) {
    int ret = vsnprintf(*strp, static_cast<size_t>(actual_size + 1), fmt,
                        va_args_copy);
    va_end(va_args_copy);
    return ret;
  }

  va_end(va_args_copy);
  return actual_size;
}

SHIM_ALWAYS_EXPORT int __wrap_asprintf(char** strp, const char* fmt, ...) {
  va_list va_args;
  va_start(va_args, fmt);
  int retval = vasprintf(strp, fmt, va_args);
  va_end(va_args);
  return retval;
}

}  // extern "C"

#endif  // PA_BUILDFLAG(USE_ALLOCATOR_SHIM)

#endif  // PARTITION_ALLOC_SHIM_ALLOCATOR_SHIM_OVERRIDE_LINKER_WRAPPED_SYMBOLS_H_
