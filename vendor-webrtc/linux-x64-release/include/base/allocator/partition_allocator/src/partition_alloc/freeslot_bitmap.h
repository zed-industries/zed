// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_FREESLOT_BITMAP_H_
#define PARTITION_ALLOC_FREESLOT_BITMAP_H_

#include <climits>
#include <cstdint>
#include <utility>

#include "partition_alloc/buildflags.h"
#include "partition_alloc/freeslot_bitmap_constants.h"
#include "partition_alloc/partition_alloc_base/bits.h"
#include "partition_alloc/partition_alloc_base/compiler_specific.h"
#include "partition_alloc/partition_alloc_constants.h"

#if PA_BUILDFLAG(USE_FREESLOT_BITMAP)

namespace partition_alloc::internal {

PA_ALWAYS_INLINE uintptr_t GetFreeSlotBitmapAddressForPointer(uintptr_t ptr) {
  uintptr_t super_page = ptr & kSuperPageBaseMask;
  return SuperPageFreeSlotBitmapAddr(super_page);
}

// Calculates the cell address and the offset inside the cell corresponding to
// the |slot_start|.
PA_ALWAYS_INLINE std::pair<FreeSlotBitmapCellType*, size_t>
GetFreeSlotBitmapCellPtrAndBitIndex(uintptr_t slot_start) {
  uintptr_t slot_superpage_offset = slot_start & kSuperPageOffsetMask;
  uintptr_t superpage_bitmap_start =
      GetFreeSlotBitmapAddressForPointer(slot_start);
  uintptr_t cell_addr = base::bits::AlignDown(
      superpage_bitmap_start +
          (slot_superpage_offset / kSmallestBucket) / CHAR_BIT,
      sizeof(FreeSlotBitmapCellType));
  PA_DCHECK(cell_addr < superpage_bitmap_start + kFreeSlotBitmapSize);
  size_t bit_index =
      (slot_superpage_offset / kSmallestBucket) & kFreeSlotBitmapOffsetMask;
  PA_DCHECK(bit_index < kFreeSlotBitmapBitsPerCell);
  return {reinterpret_cast<FreeSlotBitmapCellType*>(cell_addr), bit_index};
}

// This bitmap marks the used slot as 0 and free one as 1. This is because we
// would like to set all the slots as "used" by default to prevent allocating a
// used slot when the freelist entry is overwritten. The state of the bitmap is
// expected to be synced with freelist (i.e. the bitmap is set to 1 if and only
// if the slot is in the freelist).

PA_ALWAYS_INLINE FreeSlotBitmapCellType CellWithAOne(size_t n) {
  return static_cast<FreeSlotBitmapCellType>(1) << n;
}

PA_ALWAYS_INLINE FreeSlotBitmapCellType CellWithTrailingOnes(size_t n) {
  return (static_cast<FreeSlotBitmapCellType>(1) << n) -
         static_cast<FreeSlotBitmapCellType>(1);
}

// Returns true if the bit corresponding to |slot_start| is used( = 0)
PA_ALWAYS_INLINE bool FreeSlotBitmapSlotIsUsed(uintptr_t slot_start) {
  auto [cell, bit_index] = GetFreeSlotBitmapCellPtrAndBitIndex(slot_start);
  return (*cell & CellWithAOne(bit_index)) == 0;
}

// Mark the bit corresponding to |slot_start| as used( = 0).
PA_ALWAYS_INLINE void FreeSlotBitmapMarkSlotAsUsed(uintptr_t slot_start) {
  PA_CHECK(!FreeSlotBitmapSlotIsUsed(slot_start));
  auto [cell, bit_index] = GetFreeSlotBitmapCellPtrAndBitIndex(slot_start);
  *cell &= ~CellWithAOne(bit_index);
}

// Mark the bit corresponding to |slot_start| as free( = 1).
PA_ALWAYS_INLINE void FreeSlotBitmapMarkSlotAsFree(uintptr_t slot_start) {
  PA_CHECK(FreeSlotBitmapSlotIsUsed(slot_start));
  auto [cell, bit_index] = GetFreeSlotBitmapCellPtrAndBitIndex(slot_start);
  *cell |= CellWithAOne(bit_index);
}

// Resets (= set to 0) all the bits corresponding to the slot-start addresses
// within [begin_addr, end_addr). |begin_addr| has to be the beginning of a
// slot, but |end_addr| does not.
PA_ALWAYS_INLINE void FreeSlotBitmapReset(uintptr_t begin_addr,
                                          uintptr_t end_addr,
                                          uintptr_t slot_size) {
  PA_DCHECK(begin_addr <= end_addr);
  // |end_addr| has to be kSmallestBucket-aligned.
  PA_DCHECK((end_addr & (kSmallestBucket - 1)) == 0u);
  for (uintptr_t slot_start = begin_addr; slot_start < end_addr;
       slot_start += slot_size) {
    auto [cell, bit_index] = GetFreeSlotBitmapCellPtrAndBitIndex(slot_start);
    *cell &= ~CellWithAOne(bit_index);
  }

#if PA_BUILDFLAG(DCHECKS_ARE_ON)
  // Checks if the cells that are meant to contain only unset bits are really 0.
  auto [begin_cell, begin_bit_index] =
      GetFreeSlotBitmapCellPtrAndBitIndex(begin_addr);
  auto [end_cell, end_bit_index] =
      GetFreeSlotBitmapCellPtrAndBitIndex(end_addr);

  // The bits that should be marked to 0 are |begin_bit_index|th bit of
  // |begin_cell| to |end_bit_index - 1|th bit of |end_cell|. We verify all the
  // bits are set to 0 for the cells between [begin_cell + 1, end_cell). For the
  // |begin_cell| and |end_cell|, we have to handle them separately to only
  // check the partial bits.
  // | begin_cell |     |...|     | end_cell |
  // |11...100...0|0...0|...|0...0|0...01...1|
  //        ^                           ^
  //        |                           |
  //    begin_addr                   end_addr

  if (begin_cell == end_cell) {
    PA_DCHECK((*begin_cell & (~CellWithTrailingOnes(begin_bit_index) &
                              CellWithTrailingOnes(end_bit_index))) == 0u);
    return;
  }

  if (begin_bit_index != 0) {
    // Checks the bits between [begin_bit_index, kFreeSlotBitmapBitsPerCell) in
    // the begin_cell are 0
    PA_DCHECK((*begin_cell & ~CellWithTrailingOnes(begin_bit_index)) == 0u);
    ++begin_cell;
  }

  if (end_bit_index != 0) {
    // Checks the bits between [0, end_bit_index) in the end_cell are 0
    PA_DCHECK((*end_cell & CellWithTrailingOnes(end_bit_index)) == 0u);
  }

  for (FreeSlotBitmapCellType* cell = begin_cell; cell < end_cell; ++cell) {
    PA_DCHECK(*cell == 0u);
  }
#endif  // PA_BUILDFLAG(DCHECKS_ARE_ON)
}

}  // namespace partition_alloc::internal

#endif  // PA_BUILDFLAG(USE_FREESLOT_BITMAP)

#endif  // PARTITION_ALLOC_FREESLOT_BITMAP_H_
