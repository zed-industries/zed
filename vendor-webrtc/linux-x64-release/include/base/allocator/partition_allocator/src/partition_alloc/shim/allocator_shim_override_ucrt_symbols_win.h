// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This header defines symbols to override the same functions in the Visual C++
// CRT implementation.

#ifdef PARTITION_ALLOC_SHIM_ALLOCATOR_SHIM_OVERRIDE_UCRT_SYMBOLS_WIN_H_
#error This header is meant to be included only once by allocator_shim_win_static.cc or allocator_shim_win_component.cc
#endif

#ifndef PARTITION_ALLOC_SHIM_ALLOCATOR_SHIM_OVERRIDE_UCRT_SYMBOLS_WIN_H_
#define PARTITION_ALLOC_SHIM_ALLOCATOR_SHIM_OVERRIDE_UCRT_SYMBOLS_WIN_H_

#include "partition_alloc/buildflags.h"

#if PA_BUILDFLAG(USE_ALLOCATOR_SHIM)
#include "partition_alloc/partition_alloc_base/component_export.h"
#include "partition_alloc/partition_alloc_check.h"
#include "partition_alloc/shim/allocator_shim_internals.h"
#include "partition_alloc/shim/shim_alloc_functions.h"
#include "partition_alloc/shim/winheap_stubs_win.h"

#if defined(COMPONENT_BUILD)
#include <cstdlib>
#include <cstring>
#endif

namespace allocator_shim::internal {

size_t CheckedMultiply(size_t multiplicand, size_t multiplier);

}  // namespace allocator_shim::internal

// Even though most C++ allocation operators can be left alone since the
// interception works at a lower level, these ones should be
// overridden. Otherwise they redirect to malloc(), which is configured to crash
// with an OOM in failure cases, such as allocation requests that are too large.
PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
SHIM_ALWAYS_EXPORT void* operator new(size_t size,
                                      const std::nothrow_t&) noexcept {
  return ShimCppNewNoThrow(size);
}

PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
SHIM_ALWAYS_EXPORT void* operator new[](size_t size,
                                        const std::nothrow_t&) noexcept {
  return ShimCppNewNoThrow(size);
}

extern "C" {

// This ".h" file is not a header, but a source file meant to be included only
// once, exclusively from allocator_shim_win_static.cc or
// allocator_shim_win_component.cc. See the top-level check.
//
// A possible alternative: rename this file to .inc, at the expense of losing
// syntax highlighting in text editors.
//
// NOLINTNEXTLINE(google-build-namespaces)
namespace {

int win_new_mode = 0;

}  // namespace

// This function behaves similarly to MSVC's _set_new_mode.
// If flag is 0 (default), calls to malloc will behave normally.
// If flag is 1, calls to malloc will behave like calls to new,
// and the std_new_handler will be invoked on failure.
// Returns the previous mode.
//
// Replaces _set_new_mode in ucrt\heap\new_mode.cpp
PA_COMPONENT_EXPORT(ALLOCATOR_SHIM) int _set_new_mode(int flag) {
  // The MS CRT calls this function early on in startup, so this serves as a low
  // overhead proof that the allocator shim is in place for this process.
  allocator_shim::g_is_win_shim_layer_initialized = true;
  int old_mode = win_new_mode;
  win_new_mode = flag;

  allocator_shim::SetCallNewHandlerOnMallocFailure(win_new_mode != 0);

  return old_mode;
}

// Replaces _query_new_mode in ucrt\heap\new_mode.cpp
PA_COMPONENT_EXPORT(ALLOCATOR_SHIM) int _query_new_mode() {
  return win_new_mode;
}

// These symbols override the CRT's implementation of the same functions.
PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
__declspec(restrict) void* malloc(size_t size) {
  return ShimMalloc(size, nullptr);
}

PA_COMPONENT_EXPORT(ALLOCATOR_SHIM) void free(void* ptr) {
  ShimFree(ptr, nullptr);
}

PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
__declspec(restrict) void* realloc(void* ptr, size_t size) {
  return ShimRealloc(ptr, size, nullptr);
}

PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
__declspec(restrict) void* calloc(size_t n, size_t size) {
  return ShimCalloc(n, size, nullptr);
}

// _msize() is the Windows equivalent of malloc_size().
PA_COMPONENT_EXPORT(ALLOCATOR_SHIM) size_t _msize(void* memblock) {
  return ShimGetSizeEstimate(memblock, nullptr);
}

PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
__declspec(restrict) void* _aligned_malloc(size_t size, size_t alignment) {
  return ShimAlignedMalloc(size, alignment, nullptr);
}

PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
__declspec(restrict) void* _aligned_realloc(void* address,
                                            size_t size,
                                            size_t alignment) {
  return ShimAlignedRealloc(address, size, alignment, nullptr);
}

PA_COMPONENT_EXPORT(ALLOCATOR_SHIM) void _aligned_free(void* address) {
  ShimAlignedFree(address, nullptr);
}

// _recalloc_base is called by CRT internally.
PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
__declspec(restrict) void* _recalloc_base(void* block,
                                          size_t count,
                                          size_t size) {
  const size_t old_block_size = (block != nullptr) ? _msize(block) : 0;
  const size_t new_block_size =
      allocator_shim::internal::CheckedMultiply(count, size);
  void* const new_block = realloc(block, new_block_size);

  if (new_block != nullptr && old_block_size < new_block_size) {
    memset(static_cast<char*>(new_block) + old_block_size, 0,
           new_block_size - old_block_size);
  }

  return new_block;
}

PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
__declspec(restrict) void* _malloc_base(size_t size) {
  return malloc(size);
}

PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
__declspec(restrict) void* _calloc_base(size_t n, size_t size) {
  return calloc(n, size);
}

PA_COMPONENT_EXPORT(ALLOCATOR_SHIM) void _free_base(void* block) {
  free(block);
}

PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
__declspec(restrict) void* _recalloc(void* block, size_t count, size_t size) {
  return _recalloc_base(block, count, size);
}

// The following uncommon _aligned_* routines are not used in Chromium and have
// been shimmed to immediately crash to ensure that implementations are added if
// uses are introduced.
PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
__declspec(restrict) void* _aligned_recalloc(void* address,
                                             size_t num,
                                             size_t size,
                                             size_t alignment) {
  PA_CHECK(false) << "This routine has not been implemented";
  __builtin_unreachable();
}

PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
size_t _aligned_msize(void* address, size_t alignment, size_t offset) {
  PA_CHECK(false) << "This routine has not been implemented";
  __builtin_unreachable();
}

PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
__declspec(restrict) void* _aligned_offset_malloc(size_t size,
                                                  size_t alignment,
                                                  size_t offset) {
  PA_CHECK(false) << "This routine has not been implemented";
  __builtin_unreachable();
}

PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
__declspec(restrict) void* _aligned_offset_realloc(void* address,
                                                   size_t size,
                                                   size_t alignment,
                                                   size_t offset) {
  PA_CHECK(false) << "This routine has not been implemented";
  __builtin_unreachable();
}

PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
__declspec(restrict) void* _aligned_offset_recalloc(void* address,
                                                    size_t num,
                                                    size_t size,
                                                    size_t alignment,
                                                    size_t offset) {
  PA_CHECK(false) << "This routine has not been implemented";
  __builtin_unreachable();
}

#if defined(COMPONENT_BUILD)
// Overrides CRT functions which internally call malloc() and expect callers
// will free().
PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
char* _strdup(const char* strSource) {
  char* dest = static_cast<char*>(malloc(strlen(strSource) + 1));
  strcpy(dest, strSource);
  return dest;
}

PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
wchar_t* _wcsdup(const wchar_t* strSource) {
  wchar_t* dest =
      static_cast<wchar_t*>(malloc(sizeof(wchar_t) * (wcslen(strSource) + 1)));
  wcscpy(dest, strSource);
  return dest;
}

PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
errno_t _dupenv_s(char** buffer,
                  size_t* number_of_elements,
                  const char* varname) {
  if (buffer == nullptr || varname == nullptr) {
    return EINVAL;
  }
  size_t size = 0;
  errno_t err = getenv_s(&size, nullptr, 0, varname);
  if (err != 0) {
    *buffer = nullptr;
    if (number_of_elements) {
      *number_of_elements = 0;
    }
    return err;
  }
  if (number_of_elements) {
    *number_of_elements = size;
  }
  // If `varname` is not found, return 0 with *buffer=nullptr and
  // *number_of_elements=0.
  if (size == 0) {
    *buffer = nullptr;
    return 0;
  }
  *buffer = static_cast<char*>(malloc(size));
  return getenv_s(&size, *buffer, size, varname);
}

PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
errno_t _wdupenv_s(wchar_t** buffer,
                   size_t* number_of_elements,
                   const wchar_t* varname) {
  if (buffer == nullptr || varname == nullptr) {
    return EINVAL;
  }
  size_t size = 0;
  errno_t err = _wgetenv_s(&size, nullptr, 0, varname);
  if (err != 0) {
    *buffer = nullptr;
    if (number_of_elements) {
      *number_of_elements = 0;
    }
    return err;
  }
  if (number_of_elements) {
    *number_of_elements = size;
  }
  // If `varname` is not found, return 0 with *buffer=nullptr and
  // *number_of_elements=0.
  if (size == 0) {
    *buffer = nullptr;
    return 0;
  }
  *buffer = static_cast<wchar_t*>(malloc(sizeof(wchar_t) * size));
  return _wgetenv_s(&size, *buffer, size, varname);
}
#endif

#if PA_BUILDFLAG(IS_DEBUG)
typedef void (*_CRT_DUMP_CLIENT)(void*, size_t);

int _crtDbgFlag = 0;

PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
_CRT_DUMP_CLIENT _CrtSetDumpClient(_CRT_DUMP_CLIENT) {
  PA_CHECK(false) << "This routine has not been implemented";
  __builtin_unreachable();
  return nullptr;
}

PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
int _CrtDumpMemoryLeaks() {
  PA_CHECK(false) << "This routine has not been implemented";
  __builtin_unreachable();
  return 0;
}

PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
int _CrtSetDbgFlag(int new_flag) {
  int old_flag = _crtDbgFlag;
  _crtDbgFlag = new_flag;
  return old_flag;
}

PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
__declspec(restrict) void* _malloc_dbg(size_t size, int, const char*, int) {
  return ShimMalloc(size, nullptr);
}

PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
void _free_dbg(void* ptr, int) {
  ShimFree(ptr, nullptr);
}

PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
__declspec(restrict) void* _realloc_dbg(void* ptr,
                                        size_t size,
                                        int,
                                        const char*,
                                        int) {
  return ShimRealloc(ptr, size, nullptr);
}

PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
__declspec(restrict) void* _calloc_dbg(size_t n,
                                       size_t size,
                                       int,
                                       const char*,
                                       int) {
  return ShimCalloc(n, size, nullptr);
}

// _msize() is the Windows equivalent of malloc_size().
PA_COMPONENT_EXPORT(ALLOCATOR_SHIM) size_t _msize_dbg(void* memblock, int) {
  return ShimGetSizeEstimate(memblock, nullptr);
}

PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
__declspec(restrict) void* _aligned_malloc_dbg(size_t size,
                                               size_t alignment,
                                               const char*,
                                               int) {
  return ShimAlignedMalloc(size, alignment, nullptr);
}

PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
__declspec(restrict) void* _aligned_realloc_dbg(void* address,
                                                size_t size,
                                                size_t alignment,
                                                const char*,
                                                int) {
  return ShimAlignedRealloc(address, size, alignment, nullptr);
}

PA_COMPONENT_EXPORT(ALLOCATOR_SHIM) void _aligned_free_dbg(void* address) {
  ShimAlignedFree(address, nullptr);
}

PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
__declspec(restrict) void* _recalloc_dbg(void* block,
                                         size_t count,
                                         size_t size,
                                         int,
                                         const char*,
                                         int) {
  return _recalloc_base(block, count, size);
}

PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
void* _expand_dbg(void*, size_t, int, const char*, int) {
  PA_CHECK(false) << "This routine has not been implemented";
  __builtin_unreachable();
}

// The following uncommon _aligned_* routines are not used in Chromium and have
// been shimmed to immediately crash to ensure that implementations are added if
// uses are introduced.
PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
__declspec(restrict) void* _aligned_recalloc_dbg(void* address,
                                                 size_t num,
                                                 size_t size,
                                                 size_t alignment,
                                                 const char*,
                                                 int) {
  PA_CHECK(false) << "This routine has not been implemented";
  __builtin_unreachable();
}

PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
size_t _aligned_msize_dbg(void* address, size_t alignment, size_t offset) {
  PA_CHECK(false) << "This routine has not been implemented";
  __builtin_unreachable();
}

PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
__declspec(restrict) void* _aligned_offset_malloc_dbg(size_t const size,
                                                      size_t const alignment,
                                                      size_t const offset,
                                                      const char*,
                                                      int const) {
  PA_CHECK(false) << "This routine has not been implemented";
  __builtin_unreachable();
}

PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
__declspec(restrict) void* _aligned_offset_realloc_dbg(void* address,
                                                       size_t size,
                                                       size_t alignment,
                                                       size_t offset) {
  PA_CHECK(false) << "This routine has not been implemented";
  __builtin_unreachable();
}

PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
__declspec(restrict) void* _aligned_offset_recalloc_dbg(void* address,
                                                        size_t num,
                                                        size_t size,
                                                        size_t alignment,
                                                        size_t offset,
                                                        const char*,
                                                        int) {
  PA_CHECK(false) << "This routine has not been implemented";
  __builtin_unreachable();
}

#if defined(COMPONENT_BUILD)
// Overrides CRT functions which internally call malloc() and expect callers
// will free().
PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
char* _strdup_dbg(const char* strSource) {
  return _strdup(strSource);
}

PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
wchar_t* _wcsdup_dbg(const wchar_t* strSource) {
  return _wcsdup(strSource);
}

PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
errno_t _dupenv_s_dbg(char** buffer,
                      size_t* number_of_elements,
                      const char* varname) {
  return _dupenv_s(buffer, number_of_elements, varname);
}

PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
errno_t _wdupenv_s_dbg(wchar_t** buffer,
                       size_t* number_of_elements,
                       const wchar_t* varname) {
  return _wdupenv_s(buffer, number_of_elements, varname);
}
#endif  // defined(COMPONENT_BUILD)

#endif  // PA_BUILDFLAG(IS_DEBUG)

}       // extern "C"
#endif  // PA_BUILDFLAG(USE_ALLOCATOR_SHIM)

#endif  // PARTITION_ALLOC_SHIM_ALLOCATOR_SHIM_OVERRIDE_UCRT_SYMBOLS_WIN_H_
