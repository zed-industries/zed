// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_FREESLOT_BITMAP_CONSTANTS_H_
#define PARTITION_ALLOC_FREESLOT_BITMAP_CONSTANTS_H_

#include <cstdint>

#include "partition_alloc/buildflags.h"
#include "partition_alloc/partition_alloc_base/bits.h"
#include "partition_alloc/partition_alloc_base/compiler_specific.h"
#include "partition_alloc/partition_alloc_constants.h"
#include "partition_alloc/partition_alloc_forward.h"
#include "partition_alloc/reservation_offset_table.h"

namespace partition_alloc::internal {

using FreeSlotBitmapCellType = uint64_t;
constexpr size_t kFreeSlotBitmapBitsPerCell =
    sizeof(FreeSlotBitmapCellType) * CHAR_BIT;
constexpr size_t kFreeSlotBitmapOffsetMask = kFreeSlotBitmapBitsPerCell - 1;

// The number of bits necessary for the bitmap is equal to the maximum number of
// slots in a super page.
constexpr size_t kFreeSlotBitmapSize =
    (kSuperPageSize / kSmallestBucket) / CHAR_BIT;

PA_ALWAYS_INLINE PAGE_ALLOCATOR_CONSTANTS_DECLARE_CONSTEXPR size_t
ReservedFreeSlotBitmapSize() {
#if PA_BUILDFLAG(USE_FREESLOT_BITMAP)
  return base::bits::AlignUp(kFreeSlotBitmapSize, PartitionPageSize());
#else
  return 0;
#endif
}

PA_ALWAYS_INLINE PAGE_ALLOCATOR_CONSTANTS_DECLARE_CONSTEXPR size_t
CommittedFreeSlotBitmapSize() {
#if PA_BUILDFLAG(USE_FREESLOT_BITMAP)
  return base::bits::AlignUp(kFreeSlotBitmapSize, SystemPageSize());
#else
  return 0;
#endif
}

PA_ALWAYS_INLINE PAGE_ALLOCATOR_CONSTANTS_DECLARE_CONSTEXPR size_t
NumPartitionPagesPerFreeSlotBitmap() {
  return ReservedFreeSlotBitmapSize() / PartitionPageSize();
}

#if PA_BUILDFLAG(USE_FREESLOT_BITMAP)
PA_ALWAYS_INLINE uintptr_t SuperPageFreeSlotBitmapAddr(uintptr_t super_page) {
  PA_DCHECK(!(super_page % kSuperPageAlignment));
  return super_page + PartitionPageSize();
}
#endif

}  // namespace partition_alloc::internal

#endif  // PARTITION_ALLOC_FREESLOT_BITMAP_CONSTANTS_H_
