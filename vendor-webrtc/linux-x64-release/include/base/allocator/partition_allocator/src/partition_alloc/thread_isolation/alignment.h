// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_THREAD_ISOLATION_ALIGNMENT_H_
#define PARTITION_ALLOC_THREAD_ISOLATION_ALIGNMENT_H_

#include "partition_alloc/buildflags.h"

#if PA_BUILDFLAG(ENABLE_THREAD_ISOLATION)

#include "partition_alloc/page_allocator_constants.h"

#define PA_THREAD_ISOLATED_ALIGN_SZ partition_alloc::internal::SystemPageSize()
#define PA_THREAD_ISOLATED_ALIGN_OFFSET_MASK (PA_THREAD_ISOLATED_ALIGN_SZ - 1)
#define PA_THREAD_ISOLATED_ALIGN_BASE_MASK \
  (~PA_THREAD_ISOLATED_ALIGN_OFFSET_MASK)
#define PA_THREAD_ISOLATED_ALIGN alignas(PA_THREAD_ISOLATED_ALIGN_SZ)

#define PA_THREAD_ISOLATED_FILL_PAGE_SZ(size)          \
  ((PA_THREAD_ISOLATED_ALIGN_SZ -                      \
    ((size) & PA_THREAD_ISOLATED_ALIGN_OFFSET_MASK)) % \
   PA_THREAD_ISOLATED_ALIGN_SZ)
// Calculate the required padding so that the last element of a page-aligned
// array lands on a page boundary. In other words, calculate that padding so
// that (count-1) elements are a multiple of page size.
// The offset parameter additionally skips bytes in the object, e.g.
// object+offset will be page aligned.
#define PA_THREAD_ISOLATED_ARRAY_PAD_SZ_WITH_OFFSET(Type, count, offset) \
  PA_THREAD_ISOLATED_FILL_PAGE_SZ(sizeof(Type) * (count - 1) + offset)

#define PA_THREAD_ISOLATED_ARRAY_PAD_SZ(Type, count) \
  PA_THREAD_ISOLATED_ARRAY_PAD_SZ_WITH_OFFSET(Type, count, 0)

#else  // PA_BUILDFLAG(ENABLE_THREAD_ISOLATION)

#define PA_THREAD_ISOLATED_ALIGN
#define PA_THREAD_ISOLATED_FILL_PAGE_SZ(size) 0
#define PA_THREAD_ISOLATED_ARRAY_PAD_SZ(Type, size) 0
#define PA_THREAD_ISOLATED_ARRAY_PAD_SZ_WITH_OFFSET(Type, size, offset) 0

#endif  // PA_BUILDFLAG(ENABLE_THREAD_ISOLATION)

#endif  // PARTITION_ALLOC_THREAD_ISOLATION_ALIGNMENT_H_
