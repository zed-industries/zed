// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_POOL_OFFSET_FREELIST_H_
#define PARTITION_ALLOC_POOL_OFFSET_FREELIST_H_

#include <cstddef>
#include <cstdint>

#include "partition_alloc/build_config.h"
#include "partition_alloc/buildflags.h"
#include "partition_alloc/partition_address_space.h"
#include "partition_alloc/partition_alloc-inl.h"
#include "partition_alloc/partition_alloc_base/compiler_specific.h"
#include "partition_alloc/partition_alloc_config.h"
#include "partition_alloc/partition_alloc_constants.h"
#include "partition_alloc/tagging.h"

#if !PA_BUILDFLAG(PA_ARCH_CPU_BIG_ENDIAN)
#include "partition_alloc/reverse_bytes.h"
#endif

namespace partition_alloc::internal {

using PoolInfo = PartitionAddressSpace::PoolInfo;

class PoolOffsetFreelistEntry;

class EncodedPoolOffset {
#if PA_BUILDFLAG(PA_ARCH_CPU_BIG_ENDIAN)
  static constexpr uintptr_t kEncodeedNullptr = ~uintptr_t{0};
#else
  static constexpr uintptr_t kEncodeedNullptr = uintptr_t{0};
#endif

  PA_ALWAYS_INLINE constexpr explicit EncodedPoolOffset(std::nullptr_t)
      : encoded_(kEncodeedNullptr) {}
  PA_ALWAYS_INLINE explicit EncodedPoolOffset(void* ptr)
      // The encoded pointer stays MTE-tagged.
      : encoded_(Encode(ptr)) {}

  PA_ALWAYS_INLINE constexpr uintptr_t Inverted() const { return ~encoded_; }

  PA_ALWAYS_INLINE constexpr void Override(uintptr_t encoded) {
    encoded_ = encoded;
  }

  PA_ALWAYS_INLINE constexpr explicit operator bool() const { return encoded_; }

  // Transform() works the same in both directions, so can be used for
  // encoding and decoding.
  PA_ALWAYS_INLINE static constexpr uintptr_t Transform(uintptr_t offset) {
    // We use bswap on little endian as a fast transformation for two reasons:
    // 1) The offset is a canonical address, possibly pointing to valid memory,
    //    whereas, on 64 bit, the swapped offset is very unlikely to be a
    //    canonical address. Therefore, if an object is freed and its vtable is
    //    used where the attacker doesn't get the chance to run allocations
    //    between the free and use, the vtable dereference is likely to fault.
    // 2) If the attacker has a linear buffer overflow and elects to try and
    //    corrupt a freelist pointer, partial pointer overwrite attacks are
    //    thwarted.
    // For big endian, similar guarantees are arrived at with a negation.
#if PA_BUILDFLAG(PA_ARCH_CPU_BIG_ENDIAN)
    uintptr_t transformed = ~offset;
#else
    uintptr_t transformed = ReverseBytes(offset);
#endif
    return transformed;
  }

  // Determines the containing pool of `ptr` and returns `ptr`
  // represented as a tagged offset into that pool.
  PA_ALWAYS_INLINE static uintptr_t Encode(void* ptr) {
    if (!ptr) {
      return kEncodeedNullptr;
    }
    uintptr_t address = SlotStartPtr2Addr(ptr);
    PoolInfo pool_info = PartitionAddressSpace::GetPoolInfo(address);
    // Save a MTE tag as well as an offset.
    uintptr_t tagged_offset =
        reinterpret_cast<uintptr_t>(ptr) & (kPtrTagMask | ~pool_info.base_mask);
    return Transform(tagged_offset);
  }

  // Given `pool_info`, decodes a `tagged_offset` into a tagged pointer.
  PA_ALWAYS_INLINE PoolOffsetFreelistEntry* Decode(PoolInfo& pool_info) const {
    uintptr_t tagged_offset = Transform(encoded_);
    // We assume `tagged_offset` contains a proper MTE tag.
    return reinterpret_cast<PoolOffsetFreelistEntry*>(pool_info.base |
                                                      tagged_offset);
  }

  uintptr_t encoded_;

  friend PoolOffsetFreelistEntry;
};

// Freelist entries are encoded for security reasons. See
// //base/allocator/partition_allocator/PartitionAlloc.md
// and |Transform()| for the rationale and mechanism, respectively.
//
// We'd to especially point out, that as part of encoding, we store the entries
// as pool offsets. In a scenario that an attacker has a write primitive
// anywhere within the pool, they would not be able to corrupt the freelist
// in a way that would allow them to break out of the pool.
class PoolOffsetFreelistEntry {
  constexpr explicit PoolOffsetFreelistEntry(std::nullptr_t)
      : encoded_next_(nullptr)
#if PA_CONFIG(HAS_FREELIST_SHADOW_ENTRY)
        ,
        shadow_(encoded_next_.Inverted())
#endif
  {
  }
  explicit PoolOffsetFreelistEntry(PoolOffsetFreelistEntry* next)
      : encoded_next_(next)
#if PA_CONFIG(HAS_FREELIST_SHADOW_ENTRY)
        ,
        shadow_(encoded_next_.Inverted())
#endif
  {
  }
  // For testing only.
  PoolOffsetFreelistEntry(void* next, bool make_shadow_match)
      : encoded_next_(next)
#if PA_CONFIG(HAS_FREELIST_SHADOW_ENTRY)
        ,
        shadow_(make_shadow_match ? encoded_next_.Inverted() : 12345)
#endif
  {
  }

 public:
  ~PoolOffsetFreelistEntry() = delete;

  // Emplaces the freelist entry at the beginning of the given slot span, and
  // initializes it as null-terminated.
  PA_ALWAYS_INLINE static PoolOffsetFreelistEntry* EmplaceAndInitNull(
      void* slot_start_tagged) {
    // |slot_start_tagged| is MTE-tagged.
    auto* entry = new (slot_start_tagged) PoolOffsetFreelistEntry(nullptr);
    return entry;
  }
  PA_ALWAYS_INLINE static PoolOffsetFreelistEntry* EmplaceAndInitNull(
      uintptr_t slot_start) {
    return EmplaceAndInitNull(SlotStartAddr2Ptr(slot_start));
  }

  // Emplaces the freelist entry at the beginning of the given slot span, and
  // initializes it with the given |next| pointer, but encoded.
  //
  // This freelist is built for the purpose of thread-cache. This means that we
  // can't perform a check that this and the next pointer belong to the same
  // super page, as thread-cache spans may chain slots across super pages.
  PA_ALWAYS_INLINE static PoolOffsetFreelistEntry* EmplaceAndInitForThreadCache(
      uintptr_t slot_start,
      PoolOffsetFreelistEntry* next) {
    auto* entry =
        new (SlotStartAddr2Ptr(slot_start)) PoolOffsetFreelistEntry(next);
    return entry;
  }

  // Emplaces the freelist entry at the beginning of the given slot span, and
  // initializes it with the given |next| pointer.
  //
  // This is for testing purposes only! |make_shadow_match| allows you to choose
  // if the shadow matches the next pointer properly or is trash.
  PA_ALWAYS_INLINE static void EmplaceAndInitForTest(uintptr_t slot_start,
                                                     void* next,
                                                     bool make_shadow_match) {
    new (SlotStartAddr2Ptr(slot_start))
        PoolOffsetFreelistEntry(next, make_shadow_match);
  }

  void CorruptNextForTesting(uintptr_t v) {
    // We just need a value that can never be a valid pool offset here.
    encoded_next_.Override(EncodedPoolOffset::Transform(v));
  }

  // Puts `slot_size` on the stack before crashing in case of memory
  // corruption. Meant to be used to report the failed allocation size.
  template <bool crash_on_corruption>
  PA_ALWAYS_INLINE PoolOffsetFreelistEntry* GetNextForThreadCache(
      size_t slot_size) const {
    return GetNextInternal<crash_on_corruption, /*for_thread_cache=*/true>(
        slot_size);
  }
  PA_ALWAYS_INLINE PoolOffsetFreelistEntry* GetNext(size_t slot_size) const {
    return GetNextInternal<true, /*for_thread_cache=*/false>(slot_size);
  }

  PA_NOINLINE void CheckFreeList(size_t slot_size) const {
    for (auto* entry = this; entry; entry = entry->GetNext(slot_size)) {
      // `GetNext()` calls `IsWellFormed()`.
    }
  }

  PA_NOINLINE void CheckFreeListForThreadCache(size_t slot_size) const {
    for (auto* entry = this; entry;
         entry = entry->GetNextForThreadCache<true>(slot_size)) {
      // `GetNextForThreadCache()` calls `IsWellFormed()`.
    }
  }

  PA_ALWAYS_INLINE void SetNext(PoolOffsetFreelistEntry* entry) {
    // SetNext() is either called on the freelist head, when provisioning new
    // slots, or when GetNext() has been called before, no need to pass the
    // size.
#if PA_BUILDFLAG(DCHECKS_ARE_ON)
    // Regular freelists always point to an entry within the same super page.
    //
    // This is most likely a PartitionAlloc bug if this triggers.
    if (entry && (SlotStartPtr2Addr(this) & kSuperPageBaseMask) !=
                     (SlotStartPtr2Addr(entry) & kSuperPageBaseMask))
        [[unlikely]] {
      FreelistCorruptionDetected(0);
    }
#endif  // PA_BUILDFLAG(DCHECKS_ARE_ON)

    encoded_next_ = EncodedPoolOffset(entry);
#if PA_CONFIG(HAS_FREELIST_SHADOW_ENTRY)
    shadow_ = encoded_next_.Inverted();
#endif
  }

  // Zeroes out |this| before returning the slot. The pointer to this memory
  // will be returned to the user (caller of Alloc()), thus can't have internal
  // data.
  PA_ALWAYS_INLINE uintptr_t ClearForAllocation() {
    encoded_next_.Override(0);
#if PA_CONFIG(HAS_FREELIST_SHADOW_ENTRY)
    shadow_ = 0;
#endif
    return SlotStartPtr2Addr(this);
  }

  PA_ALWAYS_INLINE constexpr bool IsEncodedNextPtrZero() const {
    return !encoded_next_;
  }

 private:
  template <bool crash_on_corruption, bool for_thread_cache>
  PA_ALWAYS_INLINE PoolOffsetFreelistEntry* GetNextInternal(
      size_t slot_size) const {
    // GetNext() can be called on discarded memory, in which case
    // |encoded_next_| is 0, and none of the checks apply. Don't prefetch
    // nullptr either.
    if (IsEncodedNextPtrZero()) {
      return nullptr;
    }

    PoolInfo pool_info = GetPoolInfo(SlotStartPtr2Addr(this));
    // We verify that `(next_ & pool_info.base_mask) == 0` in `IsWellFormed()`,
    // which is meant to prevent from breaking out of the pool in face of
    // a corruption (see PoolOffsetFreelistEntry class-level comment).
    auto* ret = encoded_next_.Decode(pool_info);
    if (!IsWellFormed<for_thread_cache>(pool_info, this, ret)) [[unlikely]] {
      if constexpr (crash_on_corruption) {
        // Put the corrupted data on the stack, it may give us more information
        // about what kind of corruption that was.
        PA_DEBUG_DATA_ON_STACK("first",
                               static_cast<size_t>(encoded_next_.encoded_));
#if PA_CONFIG(HAS_FREELIST_SHADOW_ENTRY)
        PA_DEBUG_DATA_ON_STACK("second", static_cast<size_t>(shadow_));
#endif
        FreelistCorruptionDetected(slot_size);
      }
      return nullptr;
    }

    // In real-world profiles, the load of |encoded_next_| above is responsible
    // for a large fraction of the allocation cost. However, we cannot
    // anticipate it enough since it is accessed right after we know its
    // address.
    //
    // In the case of repeated allocations, we can prefetch the access that will
    // be done at the *next* allocation, which will touch *ret, prefetch it.
    PA_PREFETCH(ret);
    return ret;
  }

  template <bool for_thread_cache>
  PA_ALWAYS_INLINE static bool IsWellFormed(
      const PoolInfo& pool_info,
      const PoolOffsetFreelistEntry* here,
      const PoolOffsetFreelistEntry* next) {
    // Don't allow the freelist to be blindly followed to any location.
    // Checks following constraints:
    // - `here->shadow_` must match an inversion of `here->next_` (if present).
    // - `next` mustn't have bits set in the pool base mask, except MTE tag.
    // - `next` mustn't point inside the super page metadata area.
    // - Unless this is a thread-cache freelist, `here` and `next` must belong
    //   to the same super page (as a matter of fact, they must belong to the
    //   same slot span, but that'd be too expensive to check here).
    // - `next` is marked as free in the free slot bitmap (if present).

    const uintptr_t here_address = SlotStartPtr2Addr(here);
    const uintptr_t next_address = SlotStartPtr2Addr(next);

#if PA_CONFIG(HAS_FREELIST_SHADOW_ENTRY)
    bool shadow_ptr_ok = here->encoded_next_.Inverted() == here->shadow_;
#else
    constexpr bool shadow_ptr_ok = true;
#endif

    // `next_address` is MTE-untagged and `pool_info.base` does not contain a
    // tag.
    const bool pool_base_mask_matches =
        (next_address & pool_info.base_mask) == pool_info.base;

    // This is necessary but not sufficient when quarantine is enabled, see
    // SuperPagePayloadBegin() in partition_page.h. However we don't want to
    // fetch anything from the root in this function.
    const bool not_in_metadata =
        (next_address & kSuperPageOffsetMask) >= PartitionPageSize();

    if constexpr (for_thread_cache) {
      return pool_base_mask_matches & shadow_ptr_ok & not_in_metadata;
    }

    const bool same_super_page = (here_address & kSuperPageBaseMask) ==
                                 (next_address & kSuperPageBaseMask);

#if PA_BUILDFLAG(USE_FREESLOT_BITMAP)
    // TODO(crbug.com/40274683): Add support for freeslot bitmaps.
    static_assert(false, "USE_FREESLOT_BITMAP not supported");
#else
    constexpr bool marked_as_free_in_bitmap = true;
#endif

    return pool_base_mask_matches & shadow_ptr_ok & same_super_page &
           marked_as_free_in_bitmap & not_in_metadata;
  }

  // Expresses the next entry in the freelist as an offset in the
  // same pool as `this`.
  EncodedPoolOffset encoded_next_;
  // This is intended to detect unintentional corruptions of the freelist.
  // These can happen due to a Use-after-Free, or overflow of the previous
  // allocation in the slot span.
#if PA_CONFIG(HAS_FREELIST_SHADOW_ENTRY)
  uintptr_t shadow_;
#endif
};

}  // namespace partition_alloc::internal

#endif  // PARTITION_ALLOC_POOL_OFFSET_FREELIST_H_
