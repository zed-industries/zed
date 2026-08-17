//===-- nsan_allocator.h ----------------------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef NSAN_ALLOCATOR_H
#define NSAN_ALLOCATOR_H

#include "sanitizer_common/sanitizer_common.h"

namespace __nsan {

struct NsanThreadLocalMallocStorage {
  // Allocator cache contains atomic_uint64_t which must be 8-byte aligned.
  alignas(8) uptr allocator_cache[96 * (512 * 8 + 16)]; // Opaque.
  void Init();
  void CommitBack();

private:
  // These objects are allocated via mmap() and are zero-initialized.
  NsanThreadLocalMallocStorage() {}
};

void NsanAllocatorInit();
void NsanDeallocate(void *ptr);

void *nsan_malloc(uptr size);
void *nsan_calloc(uptr nmemb, uptr size);
void *nsan_realloc(void *ptr, uptr size);
void *nsan_reallocarray(void *ptr, uptr nmemb, uptr size);
void *nsan_valloc(uptr size);
void *nsan_pvalloc(uptr size);
void *nsan_aligned_alloc(uptr alignment, uptr size);
void *nsan_memalign(uptr alignment, uptr size);
int nsan_posix_memalign(void **memptr, uptr alignment, uptr size);

} // namespace __nsan
#endif // NSAN_ALLOCATOR_H
