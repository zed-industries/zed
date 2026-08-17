// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_PARTITION_FREELIST_ENTRY_H_
#define PARTITION_ALLOC_PARTITION_FREELIST_ENTRY_H_

#include <cstddef>

#include "partition_alloc/buildflags.h"
#include "partition_alloc/partition_alloc_base/bits.h"
#include "partition_alloc/partition_alloc_base/compiler_specific.h"
#include "partition_alloc/partition_alloc_base/component_export.h"
#include "partition_alloc/partition_alloc_base/notreached.h"
#include "partition_alloc/partition_alloc_constants.h"

namespace partition_alloc::internal {

[[noreturn]] PA_NOINLINE PA_COMPONENT_EXPORT(
    PARTITION_ALLOC) void FreelistCorruptionDetected(size_t slot_size);

}  // namespace partition_alloc::internal

#include "partition_alloc/encoded_next_freelist.h"  // IWYU pragma: export

// PA defaults to a freelist whose "next" links are encoded pointers.
// We are assessing an alternate implementation using an alternate
// encoding (pool offsets). When build support is enabled, the
// freelist implementation is determined at runtime.
#if PA_BUILDFLAG(USE_FREELIST_DISPATCHER)
#include "partition_alloc/pool_offset_freelist.h"  // IWYU pragma: export
#endif

namespace partition_alloc::internal {

// Assertions that are agnostic to the implementation of the freelist.

static_assert(kSmallestBucket >= sizeof(EncodedNextFreelistEntry),
              "Need enough space for freelist entries in the smallest slot");
#if PA_BUILDFLAG(USE_FREELIST_DISPATCHER)
static_assert(kSmallestBucket >= sizeof(PoolOffsetFreelistEntry),
              "Need enough space for freelist entries in the smallest slot");
#endif

enum class PartitionFreelistEncoding {
  kEncodedFreeList,
  kPoolOffsetFreeList,
};

#if PA_BUILDFLAG(USE_FREELIST_DISPATCHER)
union PartitionFreelistEntry {
  EncodedNextFreelistEntry encoded_entry_;
  PoolOffsetFreelistEntry pool_offset_entry_;
};
#else
using PartitionFreelistEntry = EncodedNextFreelistEntry;
#endif  // PA_BUILDFLAG(USE_FREELIST_DISPATCHER)

#if PA_BUILDFLAG(USE_FREELIST_DISPATCHER)
static_assert(offsetof(PartitionFreelistEntry, encoded_entry_) == 0ull);
static_assert(offsetof(PartitionFreelistEntry, pool_offset_entry_) == 0ull);
#endif

struct PartitionFreelistDispatcher {
#if PA_BUILDFLAG(USE_FREELIST_DISPATCHER)
  static const PartitionFreelistDispatcher* Create(
      PartitionFreelistEncoding encoding);

  PA_ALWAYS_INLINE virtual PartitionFreelistEntry* EmplaceAndInitNull(
      void* slot_start_tagged) const = 0;
  PA_ALWAYS_INLINE virtual PartitionFreelistEntry* EmplaceAndInitNull(
      uintptr_t slot_start) const = 0;
  PA_ALWAYS_INLINE virtual PartitionFreelistEntry* EmplaceAndInitForThreadCache(
      uintptr_t slot_start,
      PartitionFreelistEntry* next) const = 0;
  PA_ALWAYS_INLINE virtual void EmplaceAndInitForTest(
      uintptr_t slot_start,
      void* next,
      bool make_shadow_match) const = 0;
  PA_ALWAYS_INLINE virtual void CorruptNextForTesting(
      PartitionFreelistEntry* entry,
      uintptr_t v) const = 0;
  PA_ALWAYS_INLINE virtual PartitionFreelistEntry* GetNextForThreadCacheTrue(
      PartitionFreelistEntry* entry,
      size_t slot_size) const = 0;
  PA_ALWAYS_INLINE virtual PartitionFreelistEntry* GetNextForThreadCacheFalse(
      PartitionFreelistEntry* entry,
      size_t slot_size) const = 0;
  PA_ALWAYS_INLINE virtual PartitionFreelistEntry* GetNextForThreadCacheBool(
      PartitionFreelistEntry* entry,
      bool crash_on_corruption,
      size_t slot_size) const = 0;
  PA_ALWAYS_INLINE virtual PartitionFreelistEntry* GetNext(
      PartitionFreelistEntry* entry,
      size_t slot_size) const = 0;
  PA_NOINLINE virtual void CheckFreeList(PartitionFreelistEntry* entry,
                                         size_t slot_size) const = 0;
  PA_NOINLINE virtual void CheckFreeListForThreadCache(
      PartitionFreelistEntry* entry,
      size_t slot_size) const = 0;
  PA_ALWAYS_INLINE virtual void SetNext(PartitionFreelistEntry* entry,
                                        PartitionFreelistEntry* next) const = 0;
  PA_ALWAYS_INLINE virtual uintptr_t ClearForAllocation(
      PartitionFreelistEntry* entry) const = 0;
  PA_ALWAYS_INLINE virtual bool IsEncodedNextPtrZero(
      PartitionFreelistEntry* entry) const = 0;
#else
  static const PartitionFreelistDispatcher* Create(
      PartitionFreelistEncoding encoding) {
    PA_CONSTINIT static PartitionFreelistDispatcher dispatcher =
        PartitionFreelistDispatcher();
    return &dispatcher;
  }

  PA_ALWAYS_INLINE PartitionFreelistEntry* EmplaceAndInitNull(
      void* slot_start_tagged) const {
    return reinterpret_cast<PartitionFreelistEntry*>(
        EncodedNextFreelistEntry::EmplaceAndInitNull(slot_start_tagged));
  }

  PA_ALWAYS_INLINE PartitionFreelistEntry* EmplaceAndInitNull(
      uintptr_t slot_start) const {
    return reinterpret_cast<PartitionFreelistEntry*>(
        EncodedNextFreelistEntry::EmplaceAndInitNull(slot_start));
  }

  PA_ALWAYS_INLINE PartitionFreelistEntry* EmplaceAndInitForThreadCache(
      uintptr_t slot_start,
      PartitionFreelistEntry* next) const {
    return reinterpret_cast<PartitionFreelistEntry*>(
        EncodedNextFreelistEntry::EmplaceAndInitForThreadCache(slot_start,
                                                               next));
  }

  PA_ALWAYS_INLINE void EmplaceAndInitForTest(uintptr_t slot_start,
                                              void* next,
                                              bool make_shadow_match) const {
    return EncodedNextFreelistEntry::EmplaceAndInitForTest(slot_start, next,
                                                           make_shadow_match);
  }

  PA_ALWAYS_INLINE void CorruptNextForTesting(PartitionFreelistEntry* entry,
                                              uintptr_t v) const {
    return entry->CorruptNextForTesting(v);
  }

  template <bool crash_on_corruption>
  PA_ALWAYS_INLINE PartitionFreelistEntry* GetNextForThreadCache(
      PartitionFreelistEntry* entry,
      size_t slot_size) const {
    return reinterpret_cast<PartitionFreelistEntry*>(
        entry->GetNextForThreadCache<crash_on_corruption>(slot_size));
  }

  PA_ALWAYS_INLINE PartitionFreelistEntry* GetNext(
      PartitionFreelistEntry* entry,
      size_t slot_size) const {
    return reinterpret_cast<PartitionFreelistEntry*>(entry->GetNext(slot_size));
  }

  PA_NOINLINE void CheckFreeList(PartitionFreelistEntry* entry,
                                 size_t slot_size) const {
    return entry->CheckFreeList(slot_size);
  }

  PA_NOINLINE void CheckFreeListForThreadCache(PartitionFreelistEntry* entry,
                                               size_t slot_size) const {
    return entry->CheckFreeListForThreadCache(slot_size);
  }

  PA_ALWAYS_INLINE void SetNext(PartitionFreelistEntry* entry,
                                PartitionFreelistEntry* next) const {
    return entry->SetNext(next);
  }

  PA_ALWAYS_INLINE uintptr_t
  ClearForAllocation(PartitionFreelistEntry* entry) const {
    return entry->ClearForAllocation();
  }

  PA_ALWAYS_INLINE bool IsEncodedNextPtrZero(
      PartitionFreelistEntry* entry) const {
    return entry->IsEncodedNextPtrZero();
  }

  ~PartitionFreelistDispatcher() = default;
#endif  // PA_BUILDFLAG(USE_FREELIST_DISPATCHER)
};

#if PA_BUILDFLAG(USE_FREELIST_DISPATCHER)
template <PartitionFreelistEncoding encoding>
struct PartitionFreelistDispatcherImpl final : PartitionFreelistDispatcher {
  using Entry =
      std::conditional_t<encoding ==
                             PartitionFreelistEncoding::kEncodedFreeList,
                         EncodedNextFreelistEntry,
                         PoolOffsetFreelistEntry>;

  // `entry` can be passed in as `nullptr`
  Entry* GetEntryImpl(PartitionFreelistEntry* entry) const {
    return reinterpret_cast<Entry*>(entry);
  }

  PA_ALWAYS_INLINE PartitionFreelistEntry* EmplaceAndInitNull(
      void* slot_start_tagged) const override {
    return reinterpret_cast<PartitionFreelistEntry*>(
        Entry::EmplaceAndInitNull(slot_start_tagged));
  }

  PA_ALWAYS_INLINE PartitionFreelistEntry* EmplaceAndInitNull(
      uintptr_t slot_start) const override {
    return reinterpret_cast<PartitionFreelistEntry*>(
        Entry::EmplaceAndInitNull(slot_start));
  }

  // `next` can be passed in as `nullptr`
  PA_ALWAYS_INLINE PartitionFreelistEntry* EmplaceAndInitForThreadCache(
      uintptr_t slot_start,
      PartitionFreelistEntry* next) const override {
    return reinterpret_cast<PartitionFreelistEntry*>(
        Entry::EmplaceAndInitForThreadCache(slot_start, GetEntryImpl(next)));
  }

  PA_ALWAYS_INLINE void EmplaceAndInitForTest(
      uintptr_t slot_start,
      void* next,
      bool make_shadow_match) const override {
    return Entry::EmplaceAndInitForTest(slot_start, next, make_shadow_match);
  }

  PA_ALWAYS_INLINE void CorruptNextForTesting(PartitionFreelistEntry* entry,
                                              uintptr_t v) const override {
    return GetEntryImpl(entry)->CorruptNextForTesting(v);
  }

  PA_ALWAYS_INLINE PartitionFreelistEntry* GetNextForThreadCacheTrue(
      PartitionFreelistEntry* entry,
      size_t slot_size) const override {
    return reinterpret_cast<PartitionFreelistEntry*>(
        GetEntryImpl(entry)->template GetNextForThreadCache<true>(slot_size));
  }

  PA_ALWAYS_INLINE PartitionFreelistEntry* GetNextForThreadCacheFalse(
      PartitionFreelistEntry* entry,
      size_t slot_size) const override {
    return reinterpret_cast<PartitionFreelistEntry*>(
        GetEntryImpl(entry)->template GetNextForThreadCache<false>(slot_size));
  }

  PA_ALWAYS_INLINE PartitionFreelistEntry* GetNextForThreadCacheBool(
      PartitionFreelistEntry* entry,
      bool crash_on_corruption,
      size_t slot_size) const override {
    if (crash_on_corruption) {
      return GetNextForThreadCacheTrue(entry, slot_size);
    } else {
      return GetNextForThreadCacheFalse(entry, slot_size);
    }
  }

  PA_ALWAYS_INLINE PartitionFreelistEntry* GetNext(
      PartitionFreelistEntry* entry,
      size_t slot_size) const override {
    return reinterpret_cast<PartitionFreelistEntry*>(
        GetEntryImpl(entry)->GetNext(slot_size));
  }

  PA_NOINLINE void CheckFreeList(PartitionFreelistEntry* entry,
                                 size_t slot_size) const override {
    return GetEntryImpl(entry)->CheckFreeList(slot_size);
  }

  PA_NOINLINE void CheckFreeListForThreadCache(
      PartitionFreelistEntry* entry,
      size_t slot_size) const override {
    return GetEntryImpl(entry)->CheckFreeListForThreadCache(slot_size);
  }

  // `next` can be passed in as `nullptr`
  PA_ALWAYS_INLINE void SetNext(PartitionFreelistEntry* entry,
                                PartitionFreelistEntry* next) const override {
    return GetEntryImpl(entry)->SetNext(GetEntryImpl(next));
  }

  PA_ALWAYS_INLINE uintptr_t
  ClearForAllocation(PartitionFreelistEntry* entry) const override {
    return GetEntryImpl(entry)->ClearForAllocation();
  }

  PA_ALWAYS_INLINE bool IsEncodedNextPtrZero(
      PartitionFreelistEntry* entry) const override {
    return GetEntryImpl(entry)->IsEncodedNextPtrZero();
  }
};

// Both dispatchers are constexpr
// 1. to avoid "declaration requires an exit-time destructor" error
//    e.g. on android-cronet-mainline-clang-arm64-dbg.
// 2. to not create re-entrancy issues with Windows CRT
//    (crbug.com/336007395).
inline static constexpr PartitionFreelistDispatcherImpl<
    PartitionFreelistEncoding::kEncodedFreeList>
    kEncodedImplDispatcher{};
inline static constexpr PartitionFreelistDispatcherImpl<
    PartitionFreelistEncoding::kPoolOffsetFreeList>
    kPoolOffsetImplDispatcher{};

PA_ALWAYS_INLINE const PartitionFreelistDispatcher*
PartitionFreelistDispatcher::Create(PartitionFreelistEncoding encoding) {
  switch (encoding) {
    case PartitionFreelistEncoding::kEncodedFreeList: {
      return &kEncodedImplDispatcher;
    }
    case PartitionFreelistEncoding::kPoolOffsetFreeList: {
      return &kPoolOffsetImplDispatcher;
    }
  }
  PA_NOTREACHED();
}

#endif  // PA_BUILDFLAG(USE_FREELIST_DISPATCHER)

}  // namespace partition_alloc::internal

#endif  // PARTITION_ALLOC_PARTITION_FREELIST_ENTRY_H_
