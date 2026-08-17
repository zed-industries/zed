// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_PARTITION_PAGE_CONSTANTS_H_
#define PARTITION_ALLOC_PARTITION_PAGE_CONSTANTS_H_

#include "partition_alloc/build_config.h"
#include "partition_alloc/partition_alloc_base/compiler_specific.h"
#include "partition_alloc/partition_alloc_constants.h"

namespace partition_alloc::internal {

#if PA_BUILDFLAG(HAS_64_BIT_POINTERS) && PA_BUILDFLAG(IS_APPLE)
// System page size is not a constant on Apple OSes, but is either 4 or 16kiB
// (1 << 12 or 1 << 14), as checked in PartitionRoot::Init(). And
// PartitionPageSize() is 4 times the OS page size.
static constexpr size_t kMaxSlotsPerSlotSpan = 4 * (1 << 14) / kSmallestBucket;
#elif defined(PARTITION_ALLOCATOR_CONSTANTS_POSIX_NONCONST_PAGE_SIZE) && \
    PA_BUILDFLAG(IS_LINUX) &&                                            \
    (PA_BUILDFLAG(PA_ARCH_CPU_ARM64) || PA_BUILDFLAG(PA_ARCH_CPU_PPC64))
// System page size can be 4, 16, or 64 kiB on Linux on AArch64.
// System page size can be 4 or 64 kiB on Linux on ppc64.
// In both cases, use the 64 kiB maximum.
static constexpr size_t kMaxSlotsPerSlotSpan = 4 * (1 << 16) / kSmallestBucket;
#elif defined(PARTITION_ALLOCATOR_CONSTANTS_POSIX_NONCONST_PAGE_SIZE)
static constexpr size_t kMaxSlotsPerSlotSpan = 4 * (1 << 14) / kSmallestBucket;
#else
// A slot span can "span" multiple PartitionPages, but then its slot size is
// larger, so it doesn't have as many slots.
static constexpr size_t kMaxSlotsPerSlotSpan =
    PartitionPageSize() / kSmallestBucket;
#endif  // PA_BUILDFLAG(HAS_64_BIT_POINTERS) && PA_BUILDFLAG(IS_APPLE)

}  // namespace partition_alloc::internal

#endif  // PARTITION_ALLOC_PARTITION_PAGE_CONSTANTS_H_
