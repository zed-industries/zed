// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_PARTITION_PAGE_H_
#define PARTITION_ALLOC_PARTITION_PAGE_H_

#include <cstdint>

#include "partition_alloc/address_pool_manager.h"
#include "partition_alloc/address_pool_manager_types.h"
#include "partition_alloc/build_config.h"
#include "partition_alloc/buildflags.h"
#include "partition_alloc/freeslot_bitmap_constants.h"
#include "partition_alloc/partition_address_space.h"
#include "partition_alloc/partition_alloc_base/bits.h"
#include "partition_alloc/partition_alloc_base/compiler_specific.h"
#include "partition_alloc/partition_alloc_base/component_export.h"
#include "partition_alloc/partition_alloc_base/thread_annotations.h"
#include "partition_alloc/partition_alloc_check.h"
#include "partition_alloc/partition_alloc_config.h"
#include "partition_alloc/partition_alloc_constants.h"
#include "partition_alloc/partition_alloc_forward.h"
#include "partition_alloc/partition_bucket.h"
#include "partition_alloc/partition_dcheck_helper.h"
#include "partition_alloc/partition_freelist_entry.h"
#include "partition_alloc/partition_page_constants.h"
#include "partition_alloc/partition_superpage_extent_entry.h"
#include "partition_alloc/reservation_offset_table.h"

#if PA_BUILDFLAG(DCHECKS_ARE_ON)
#include "partition_alloc/tagging.h"
#endif

namespace partition_alloc::internal {

template <MetadataKind kind>
struct SlotSpanMetadata;

// Metadata of the slot span.
//
// Some notes on slot span states. It can be in one of four major states:
// 1) Active.
// 2) Full.
// 3) Empty.
// 4) Decommitted.
// An active slot span has available free slots, as well as allocated ones.
// A full slot span has no free slots. An empty slot span has no allocated
// slots, and a decommitted slot span is an empty one that had its backing
// memory released back to the system.
//
// There are three linked lists tracking slot spans. The "active" list is an
// approximation of a list of active slot spans. It is an approximation because
// full, empty and decommitted slot spans may briefly be present in the list
// until we next do a scan over it. The "empty" list holds mostly empty slot
// spans, but may briefly hold decommitted ones too. The "decommitted" list
// holds only decommitted slot spans.
//
// The significant slot span transitions are:
// - Free() will detect when a full slot span has a slot freed and immediately
//   return the slot span to the head of the active list.
// - Free() will detect when a slot span is fully emptied. It _may_ add it to
//   the empty list or it _may_ leave it on the active list until a future
//   list scan.
// - Alloc() _may_ scan the active page list in order to fulfil the request.
//   If it does this, full, empty and decommitted slot spans encountered will be
//   booted out of the active list. If there are no suitable active slot spans
//   found, an empty or decommitted slot spans (if one exists) will be pulled
//   from the empty/decommitted list on to the active list.
#pragma pack(push, 1)
template <MetadataKind kind>
struct SlotSpanMetadataBase {
 protected:
  MaybeConstT<kind, PartitionFreelistEntry*> freelist_head = nullptr;

 public:
  // TODO(lizeb): Make as many fields as possible private or const, to
  // encapsulate things more clearly.
  MaybeConstT<kind, SlotSpanMetadata<MetadataKind::kReadOnly>*> next_slot_span =
      nullptr;
  PartitionBucket* const bucket = nullptr;

  // CHECK()ed in AllocNewSlotSpan().
  // The maximum number of bits needed to cover all currently supported OSes.
  static constexpr size_t kMaxSlotsPerSlotSpanBits = 15;
  static_assert(kMaxSlotsPerSlotSpan < (1 << kMaxSlotsPerSlotSpanBits), "");

  // |num_allocated_slots| is 0 for empty or decommitted slot spans, which can
  // be further differentiated by checking existence of the freelist.
  MaybeConstT<kind, uint32_t> num_allocated_slots : kMaxSlotsPerSlotSpanBits =
                                                        0u;
  MaybeConstT<kind, uint32_t> num_unprovisioned_slots
      : kMaxSlotsPerSlotSpanBits = 0u;

  // |marked_full| isn't equivalent to being full. Slot span is marked as full
  // iff it isn't on the active slot span list (or any other list).
  MaybeConstT<kind, uint32_t> marked_full : 1 = 0u;

 protected:
  const uint32_t can_store_raw_size_ : 1 = 0u;
  MaybeConstT<kind, uint16_t> freelist_is_sorted_ : 1 = 1u;
  // If |in_empty_cache_|==1, |empty_cache_index| is undefined and mustn't be
  // used.
  MaybeConstT<kind, uint16_t> in_empty_cache_ : 1 = 0u;
  // Index of the page in the empty cache. This is in the range
  // [0, `kMaxEmptySlotSpanRingSize - 1`] so it fits in
  // `BitWidth(kMaxEmptySlotSpanRingSize - 1)`.
  MaybeConstT<kind, uint16_t> empty_cache_index_
      : internal::base::bits::BitWidth(kMaxEmptySlotSpanRingSize - 1) = 0u;
  // Can use only 48 bits (6B) in this bitfield, as this structure is embedded
  // in PartitionPage which has 2B worth of fields and must fit in 32B.

 public:
  // Methods required by both SlotSpanMetadata<kReadOnly> and <kWritable>.

  // Checks if it is feasible to store raw_size.
  PA_ALWAYS_INLINE bool CanStoreRawSize() const { return can_store_raw_size_; }

  // Returns the total size of the slots that are currently provisioned.
  PA_ALWAYS_INLINE size_t GetProvisionedSize() const {
    size_t num_provisioned_slots =
        bucket->get_slots_per_span() - num_unprovisioned_slots;
    size_t provisioned_size = num_provisioned_slots * bucket->slot_size;
    PA_DCHECK(provisioned_size <= bucket->get_bytes_per_span());
    return provisioned_size;
  }

  // Return the number of entries in the freelist.
  size_t GetFreelistLength() const {
    size_t num_provisioned_slots =
        bucket->get_slots_per_span() - num_unprovisioned_slots;
    return num_provisioned_slots - num_allocated_slots;
  }

  PA_ALWAYS_INLINE bool in_empty_cache() const { return in_empty_cache_; }

 protected:
  constexpr SlotSpanMetadataBase() noexcept = default;
  explicit SlotSpanMetadataBase(PartitionBucket* b)
      : bucket(b), can_store_raw_size_(b->CanStoreRawSize()) {}

  bool is_decommitted_internal() const {
    bool ret = (!num_allocated_slots && !freelist_head);
    if (ret) {
      PA_DCHECK(!marked_full);
      PA_DCHECK(!num_unprovisioned_slots);
      PA_DCHECK(!in_empty_cache_);
    }
    return ret;
  }

  bool is_empty_internal() const {
    bool ret = (!num_allocated_slots && freelist_head);
    if (ret) {
      PA_DCHECK(!marked_full);
    }
    return ret;
  }
};

template <>
struct SlotSpanMetadata<MetadataKind::kReadOnly>
    : public SlotSpanMetadataBase<MetadataKind::kReadOnly> {
#if PA_CONFIG(ENABLE_SHADOW_METADATA)
  // We don't need to directly create read-only SlotSpanMetadata. We will do:
  // (1) we know the address of read-only SlotSpanMetadata.
  // (2) we use ToWritable() to obtain its writable address.
  // (3) we invoke writable SlotSpanMetadata's constructor.
  // (4) we see that the read-only one has been initialized.
  explicit SlotSpanMetadata<MetadataKind::kReadOnly>(PartitionBucket*) = delete;
#else
  explicit SlotSpanMetadata<MetadataKind::kReadOnly>(PartitionBucket* b)
      : SlotSpanMetadataBase<MetadataKind::kReadOnly>(b) {}
#endif  // PA_CONFIG(ENABLE_SHADOW_METADATA)
  // pa_tcache_inspect needs the copy constructor.
  SlotSpanMetadata(const SlotSpanMetadata<MetadataKind::kReadOnly>&) = default;

  // Public API
  // Pointer/address manipulation functions. These must be static as the input
  // |slot_span| pointer may be the result of an offset calculation and
  // therefore cannot be trusted. The objective of these functions is to
  // sanitize this input.
  PA_ALWAYS_INLINE static uintptr_t ToSlotSpanStart(
      const SlotSpanMetadata<MetadataKind::kReadOnly>* slot_span);
  PA_ALWAYS_INLINE static SlotSpanMetadata<MetadataKind::kReadOnly>* FromAddr(
      uintptr_t address);
  PA_ALWAYS_INLINE static SlotSpanMetadata<MetadataKind::kReadOnly>*
  FromSlotStart(uintptr_t slot_start);
  PA_ALWAYS_INLINE static SlotSpanMetadata<MetadataKind::kReadOnly>* FromObject(
      const void* object);
  PA_ALWAYS_INLINE static SlotSpanMetadata<MetadataKind::kReadOnly>*
  FromObjectInnerAddr(uintptr_t address);
  PA_ALWAYS_INLINE static SlotSpanMetadata<MetadataKind::kReadOnly>*
  FromObjectInnerPtr(const void* ptr);

  PA_ALWAYS_INLINE PartitionSuperPageExtentEntry<MetadataKind::kReadOnly>*
  ToSuperPageExtent() const;

  PA_ALWAYS_INLINE size_t GetRawSize() const;

  PA_ALWAYS_INLINE PartitionFreelistEntry* get_freelist_head() const {
    return freelist_head;
  }

  // Returns size of the region used within a slot. The used region comprises
  // of actual allocated data, extras and possibly empty space in the middle.
  PA_ALWAYS_INLINE size_t GetUtilizedSlotSize() const {
    // The returned size can be:
    // - The slot size for small buckets.
    // - Exact size needed to satisfy allocation (incl. extras), for large
    //   buckets and direct-mapped allocations (see also the comment in
    //   CanStoreRawSize() for more info).
    if (!CanStoreRawSize()) [[likely]] {
      return bucket->slot_size;
    }
    return GetRawSize();
  }

  // This includes padding due to rounding done at allocation; we don't know the
  // requested size at deallocation, so we use this in both places.
  PA_ALWAYS_INLINE size_t GetSlotSizeForBookkeeping() const {
    // This could be more precise for allocations where CanStoreRawSize()
    // returns true (large allocations). However this is called for *every*
    // allocation, so we don't want an extra branch there.
    return bucket->slot_size;
  }

  // TODO(ajwong): Can this be made private?  https://crbug.com/787153
  PA_COMPONENT_EXPORT(PARTITION_ALLOC)
  static const SlotSpanMetadata<MetadataKind::kReadOnly>*
  get_sentinel_slot_span();
  // The sentinel is not supposed to be modified and hence we mark it as const
  // under the hood. However, we often store it together with mutable metadata
  // objects and need a non-const pointer.
  // You can use this function for this case, but you need to ensure that the
  // returned object will not be written to.
  static SlotSpanMetadata<MetadataKind::kReadOnly>*
  get_sentinel_slot_span_non_const();

  // Slot span state getters.
  PA_ALWAYS_INLINE bool is_active() const;
  PA_ALWAYS_INLINE bool is_full() const;
  PA_ALWAYS_INLINE bool is_empty() const;
  PA_ALWAYS_INLINE bool is_decommitted() const;
  PA_ALWAYS_INLINE bool freelist_is_sorted() const {
    return freelist_is_sorted_;
  }

  PA_ALWAYS_INLINE SlotSpanMetadata<MetadataKind::kWritable>* ToWritable(
      const PartitionRoot* root) {
    return ToWritableInternal(root);
  }

  PA_ALWAYS_INLINE SlotSpanMetadata<MetadataKind::kReadOnly>* ToReadOnly() {
    return this;
  }

 private:
  void IncrementNumberOfNonemptySlotSpans();

  template <typename T>
  SlotSpanMetadata<MetadataKind::kWritable>* ToWritableInternal(
      [[maybe_unused]] T* root) {
#if PA_CONFIG(ENABLE_SHADOW_METADATA)
    // Must not make writable slot_span from sentinel slot_span.
    PA_DCHECK(this != get_sentinel_slot_span());
    return reinterpret_cast<SlotSpanMetadata<MetadataKind::kWritable>*>(
        reinterpret_cast<intptr_t>(this) + root->ShadowPoolOffset());
#else
    return reinterpret_cast<SlotSpanMetadata<MetadataKind::kWritable>*>(this);
#endif  // PA_CONFIG(ENABLE_SHADOW_METADATA)
  }

  // sentinel_slot_span_ is used as a sentinel to indicate that there is no slot
  // span in the active list. We could use nullptr, but in that case we need to
  // add a null-check branch to the hot allocation path. We want to avoid that.
  //
  // Note, this declaration is kept in the header as opposed to an anonymous
  // namespace so the getter can be fully inlined.
  static const SlotSpanMetadata<MetadataKind::kReadOnly> sentinel_slot_span_;
  // For the sentinel.
  inline constexpr SlotSpanMetadata<MetadataKind::kReadOnly>() noexcept =
      default;
};

template <>
struct SlotSpanMetadata<MetadataKind::kWritable>
    : public SlotSpanMetadataBase<MetadataKind::kWritable> {
  PA_COMPONENT_EXPORT(PARTITION_ALLOC)
  explicit SlotSpanMetadata<MetadataKind::kWritable>(PartitionBucket* b)
      : SlotSpanMetadataBase<MetadataKind::kWritable>(b) {}

  PA_ALWAYS_INLINE PartitionSuperPageExtentEntry<MetadataKind::kWritable>*
  ToSuperPageExtent();

  // Note the matching Alloc() functions are in PartitionPage.
  PA_NOINLINE PA_COMPONENT_EXPORT(PARTITION_ALLOC) void FreeSlowPath(
      size_t number_of_freed,
      PartitionRoot* root);
  // Note the matching Alloc() functions are in PartitionPage.
  PA_NOINLINE PA_COMPONENT_EXPORT(PARTITION_ALLOC) void FreeSlowPath(
      size_t number_of_freed);
  PA_ALWAYS_INLINE PartitionFreelistEntry* PopForAlloc(
      size_t size,
      const PartitionFreelistDispatcher* freelist_dispatcher);
  PA_ALWAYS_INLINE void Free(
      uintptr_t ptr,
      PartitionRoot* root,
      const PartitionFreelistDispatcher* freelist_dispatcher)
      PA_EXCLUSIVE_LOCKS_REQUIRED(PartitionRootLock(root));
  // Appends the passed freelist to the slot-span's freelist. Please note that
  // the function doesn't increment the tags of the passed freelist entries,
  // since FreeInline() did it already.
  PA_ALWAYS_INLINE void AppendFreeList(
      PartitionFreelistEntry* head,
      PartitionFreelistEntry* tail,
      size_t number_of_freed,
      PartitionRoot* root,
      const PartitionFreelistDispatcher* freelist_dispatcher)
      PA_EXCLUSIVE_LOCKS_REQUIRED(PartitionRootLock(root));

  void Decommit(PartitionRoot* root);
  void DecommitIfPossible(PartitionRoot* root);

  // Sorts the freelist in ascending addresses order.
  void SortFreelist(PartitionRoot* root);
  // Inserts the slot span into the empty ring, making space for the new slot
  // span, and potentially shrinking the ring.
  void RegisterEmpty();

  // The caller is responsible for ensuring that raw_size can be stored before
  // calling Set/GetRawSize.
  PA_ALWAYS_INLINE void SetRawSize(size_t raw_size);

  PA_ALWAYS_INLINE void SetFreelistHead(PartitionFreelistEntry* new_head,
                                        PartitionRoot* root);

  PA_ALWAYS_INLINE void Reset();

#if PA_BUILDFLAG(DCHECKS_ARE_ON)
  // Slot span state getters.
  PA_ALWAYS_INLINE bool is_empty() const;
  PA_ALWAYS_INLINE bool is_decommitted() const;
#endif  // PA_BUILDFLAG(DCHECKS_ARE_ON)

  PA_ALWAYS_INLINE void set_freelist_sorted() { freelist_is_sorted_ = true; }

  PA_ALWAYS_INLINE SlotSpanMetadata<MetadataKind::kWritable>* ToWritable() {
    return this;
  }

  PA_ALWAYS_INLINE SlotSpanMetadata<MetadataKind::kReadOnly>* ToReadOnly(
      const PartitionRoot* root) {
    return ToReadOnlyInternal(root);
  }

 private:
  void IncrementNumberOfNonemptySlotSpans();

  template <typename T>
  PA_ALWAYS_INLINE SlotSpanMetadata<MetadataKind::kReadOnly>*
  ToReadOnlyInternal([[maybe_unused]] const T* root) {
#if PA_CONFIG(ENABLE_SHADOW_METADATA)
    return reinterpret_cast<SlotSpanMetadata<MetadataKind::kReadOnly>*>(
        reinterpret_cast<intptr_t>(this) - root->ShadowPoolOffset());
#else
    return reinterpret_cast<SlotSpanMetadata<MetadataKind::kReadOnly>*>(this);
#endif  // PA_CONFIG(ENABLE_SHADOW_METADATA)
  }
};
#pragma pack(pop)
static_assert(sizeof(SlotSpanMetadata<MetadataKind::kReadOnly>) <=
                  kPageMetadataSize,
              "SlotSpanMetadata<MetadataKind::kReadOnly> must fit into a Page "
              "Metadata slot.");

// Metadata of a non-first partition page in a slot span.
template <MetadataKind kind>
struct SubsequentPageMetadata {
  // Raw size is the size needed to satisfy the allocation (requested size +
  // extras). If available, it can be used to report better statistics or to
  // bring protective cookie closer to the allocated memory.
  //
  // It can be used only if:
  // - there is no more than one slot in the slot span (otherwise we wouldn't
  //   know which slot the raw size applies to)
  // - there is more than one partition page in the slot span (the metadata of
  //   the first one is used to store slot information, but the second one is
  //   available for extra information)
  MaybeConstT<kind, size_t> raw_size;
};

// Each partition page has metadata associated with it. The metadata of the
// first page of a slot span, describes that slot span. If a slot span spans
// more than 1 page, the page metadata may contain rudimentary additional
// information.
// "Pack" the union so that common page metadata still fits within
// kPageMetadataSize. (SlotSpanMetadata is also "packed".)
#pragma pack(push, 1)
template <MetadataKind kind>
struct PartitionPageMetadataBase {
  union {
    SlotSpanMetadata<kind> slot_span_metadata;

    SubsequentPageMetadata<kind> subsequent_page_metadata;

    // sizeof(PartitionPageMetadata) must always be:
    // - a power of 2 (for fast modulo operations)
    // - below kPageMetadataSize
    //
    // This makes sure that this is respected no matter the architecture.
    MaybeConstT<kind, char>
        optional_padding[kPageMetadataSize - sizeof(uint8_t) - sizeof(bool)];
  };

  // The first PartitionPage of the slot span holds its metadata. This offset
  // tells how many pages in from that first page we are.
  // For direct maps, the first page metadata (that isn't super page extent
  // entry) uses this field to tell how many pages to the right the direct map
  // metadata starts.
  //
  // 6 bits is enough to represent all possible offsets, given that the smallest
  // partition page is 16kiB and the offset won't exceed 1MiB.
  static constexpr uint16_t kMaxSlotSpanMetadataBits = 6;
  static constexpr uint16_t kMaxSlotSpanMetadataOffset =
      (1 << kMaxSlotSpanMetadataBits) - 1;
  MaybeConstT<kind, uint8_t> slot_span_metadata_offset
      : kMaxSlotSpanMetadataBits;

  // |is_valid| tells whether the page is part of a slot span. If |false|,
  // |has_valid_span_after_this| tells whether it's an unused region in between
  // slot spans within the super page.
  // Note, |is_valid| has been added for clarity, but if we ever need to save
  // this bit, it can be inferred from:
  //   |!slot_span_metadata_offset && slot_span_metadata->bucket|.
  MaybeConstT<kind, bool> is_valid : 1;
  MaybeConstT<kind, bool> has_valid_span_after_this : 1;
  MaybeConstT<kind, uint8_t> unused;
};

template <MetadataKind kind>
struct PartitionPageMetadata;

template <>
struct PartitionPageMetadata<MetadataKind::kReadOnly>
    : public PartitionPageMetadataBase<MetadataKind::kReadOnly> {
  PA_ALWAYS_INLINE static PartitionPageMetadata<MetadataKind::kReadOnly>*
  FromAddr(uintptr_t address);

  PA_ALWAYS_INLINE PartitionPageMetadata<MetadataKind::kWritable>* ToWritable(
      PartitionRoot* root) {
    return ToWritableInternal(root);
  }

  // In order to resolve circular dependencies, i.e. ToWritable() needs
  // PartitionRoot, define template method: ToWritableInternal() here and
  // ToWritable() uses it.
  template <typename T>
  PartitionPageMetadata<MetadataKind::kWritable>* ToWritableInternal(
      [[maybe_unused]] T* root) {
#if PA_CONFIG(ENABLE_SHADOW_METADATA)
    return reinterpret_cast<PartitionPageMetadata<MetadataKind::kWritable>*>(
        reinterpret_cast<intptr_t>(this) + root->ShadowPoolOffset());
#else
    return reinterpret_cast<PartitionPageMetadata<MetadataKind::kWritable>*>(
        this);
#endif  // PA_CONFIG(ENABLE_SHADOW_METADATA)
  }
};

template <>
struct PartitionPageMetadata<MetadataKind::kWritable>
    : public PartitionPageMetadataBase<MetadataKind::kWritable> {
  PA_ALWAYS_INLINE PartitionPageMetadata<MetadataKind::kReadOnly>* ToReadOnly(
      PartitionRoot* root) {
    return ToReadOnlyInternal(root);
  }

  // In order to resolve circular dependencies, i.e. ToReadOnly() needs
  // PartitionRoot, define template method: ToReadOnlyInternal() here and
  // ToReadOnly() uses it.
  template <typename T>
  PartitionPageMetadata<MetadataKind::kReadOnly>* ToReadOnlyInternal(
      [[maybe_unused]] T* root) {
#if PA_CONFIG(ENABLE_SHADOW_METADATA)
    return reinterpret_cast<PartitionPageMetadata<MetadataKind::kReadOnly>*>(
        reinterpret_cast<intptr_t>(this) - root->ShadowPoolOffset());
#else
    return reinterpret_cast<PartitionPageMetadata<MetadataKind::kReadOnly>*>(
        this);
#endif  // PA_CONFIG(ENABLE_SHADOW_METADATA)
  }
};
#pragma pack(pop)

static_assert(sizeof(PartitionPageMetadata<MetadataKind::kWritable>) ==
                  kPageMetadataSize,
              "PartitionPage must be able to fit in a metadata slot");
static_assert(sizeof(PartitionPageMetadata<MetadataKind::kReadOnly>) ==
                  sizeof(PartitionPageMetadata<MetadataKind::kWritable>),
              "The size of PartitionPageMetadata<MetadataKind::kWritable> must "
              "be equal to PartitionPageMetadata<MetadataKind::kReadOnly>.");

// Certain functions rely on PartitionPageMetadata being either SlotSpanMetadata
// or SubsequentPageMetadata, and therefore freely casting between each other.
// TODO(crbug.com/40940915) Stop ignoring the -Winvalid-offsetof warning.
#if defined(__clang__) || defined(__GNUC__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Winvalid-offsetof"
#endif
static_assert(offsetof(PartitionPageMetadata<MetadataKind::kReadOnly>,
                       slot_span_metadata) == 0,
              "slot_span_metadata must be placed at the beginning of "
              "PartitionPageMetadata<MetadataKind::kReadOnly>.");
static_assert(offsetof(PartitionPageMetadata<MetadataKind::kReadOnly>,
                       subsequent_page_metadata) == 0,
              "subsequent_page_metadata must be placed at the beginning of "
              "PartitionPageMetadata<MetadataKind::kReadOnly>.");
static_assert(offsetof(PartitionPageMetadata<MetadataKind::kWritable>,
                       slot_span_metadata) == 0,
              "slot_span_metadata must be placed at the beginning of "
              "PartitionPageMetadata<MetadataKind::kWritable>.");
static_assert(offsetof(PartitionPageMetadata<MetadataKind::kWritable>,
                       subsequent_page_metadata) == 0,
              "subsequent_page_metadata must be placed at the beginning of "
              "PartitionPageMetadata<MetadataKind::kWritable>.");
#if defined(__clang__) || defined(__GNUC__)
#pragma GCC diagnostic pop
#endif

PA_ALWAYS_INLINE PartitionPageMetadata<MetadataKind::kReadOnly>*
PartitionSuperPageToMetadataArea(uintptr_t super_page) {
  // This can't be just any super page, but it has to be the first super page of
  // the reservation, as we assume here that the metadata is near its beginning.
  PA_DCHECK(IsReservationStart(super_page));
  PA_DCHECK(!(super_page & kSuperPageOffsetMask));
  // The metadata area is exactly one system page (the guard page) into the
  // super page.
  return reinterpret_cast<PartitionPageMetadata<MetadataKind::kReadOnly>*>(
      super_page + SystemPageSize());
}

PA_ALWAYS_INLINE const SubsequentPageMetadata<MetadataKind::kReadOnly>*
GetSubsequentPageMetadata(
    const PartitionPageMetadata<MetadataKind::kReadOnly>* page_metadata) {
  return &(page_metadata + 1)->subsequent_page_metadata;
}

PA_ALWAYS_INLINE SubsequentPageMetadata<MetadataKind::kWritable>*
GetSubsequentPageMetadata(
    PartitionPageMetadata<MetadataKind::kWritable>* page_metadata) {
  return &(page_metadata + 1)->subsequent_page_metadata;
}

PA_ALWAYS_INLINE PartitionSuperPageExtentEntry<MetadataKind::kReadOnly>*
PartitionSuperPageToExtent(uintptr_t super_page) {
  // The very first entry of the metadata is the super page extent entry.
  return reinterpret_cast<
      PartitionSuperPageExtentEntry<MetadataKind::kReadOnly>*>(
      PartitionSuperPageToMetadataArea(super_page));
}

PA_ALWAYS_INLINE PAGE_ALLOCATOR_CONSTANTS_DECLARE_CONSTEXPR size_t
ReservedStateBitmapSize() {
  return 0ull;
}

PA_ALWAYS_INLINE uintptr_t
SuperPagePayloadStartOffset(bool is_managed_by_normal_buckets) {
  return PartitionPageSize() +
         (is_managed_by_normal_buckets ? ReservedFreeSlotBitmapSize() : 0);
}

PA_ALWAYS_INLINE uintptr_t SuperPagePayloadBegin(uintptr_t super_page) {
  PA_DCHECK(!(super_page % kSuperPageAlignment));
  return super_page +
         SuperPagePayloadStartOffset(IsManagedByNormalBuckets(super_page));
}

PA_ALWAYS_INLINE uintptr_t SuperPagePayloadEndOffset() {
  return kSuperPageSize - PartitionPageSize();
}

PA_ALWAYS_INLINE uintptr_t SuperPagePayloadEnd(uintptr_t super_page) {
  PA_DCHECK(!(super_page % kSuperPageAlignment));
  return super_page + SuperPagePayloadEndOffset();
}

PA_ALWAYS_INLINE size_t SuperPagePayloadSize(uintptr_t super_page) {
  return SuperPagePayloadEnd(super_page) - SuperPagePayloadBegin(super_page);
}

PA_ALWAYS_INLINE PartitionSuperPageExtentEntry<MetadataKind::kReadOnly>*
SlotSpanMetadata<MetadataKind::kReadOnly>::ToSuperPageExtent() const {
  uintptr_t super_page = reinterpret_cast<uintptr_t>(this) & kSuperPageBaseMask;
  return PartitionSuperPageToExtent(super_page);
}

PA_ALWAYS_INLINE PartitionSuperPageExtentEntry<MetadataKind::kWritable>*
SlotSpanMetadata<MetadataKind::kWritable>::ToSuperPageExtent() {
#if PA_CONFIG(ENABLE_SHADOW_METADATA)
  uintptr_t super_page_extent_entry =
      reinterpret_cast<uintptr_t>(this) & SystemPageBaseMask();
  return reinterpret_cast<
      PartitionSuperPageExtentEntry<MetadataKind::kWritable>*>(
      super_page_extent_entry);
#else
  // Must be no-op.
  return ToReadOnly(nullptr)->ToSuperPageExtent()->ToWritable(nullptr);
#endif  // PA_CONFIG(ENABLE_SHADOW_METADATA)
}

// Returns whether the pointer lies within the super page's payload area (i.e.
// area devoted to slot spans). It doesn't check whether it's within a valid
// slot span. It merely ensures it doesn't fall in a meta-data region that would
// surely never contain user data.
PA_ALWAYS_INLINE bool IsWithinSuperPagePayload(uintptr_t address) {
  uintptr_t super_page = address & kSuperPageBaseMask;
  uintptr_t payload_start = SuperPagePayloadBegin(super_page);
  uintptr_t payload_end = SuperPagePayloadEnd(super_page);
  return address >= payload_start && address < payload_end;
}

// Converts from an address inside a super page into a pointer to the
// PartitionPageMetadata object (within super pages's metadata) that describes
// the partition page where |address| is located. |address| doesn't have to be
// located within a valid (i.e. allocated) slot span, but must be within the
// super page's payload area (i.e. area devoted to slot spans).
//
// While it is generally valid for |ptr| to be in the middle of an allocation,
// care has to be taken with direct maps that span multiple super pages. This
// function's behavior is undefined if |ptr| lies in a subsequent super page.
PA_ALWAYS_INLINE PartitionPageMetadata<MetadataKind::kReadOnly>*
PartitionPageMetadata<MetadataKind::kReadOnly>::FromAddr(uintptr_t address) {
  uintptr_t super_page = address & kSuperPageBaseMask;

#if PA_BUILDFLAG(DCHECKS_ARE_ON)
  PA_DCHECK(IsReservationStart(super_page));
  PA_DCHECK(IsWithinSuperPagePayload(address));
#endif  // PA_BUILDFLAG(DCHECKS_ARE_ON)

  uintptr_t partition_page_index =
      (address & kSuperPageOffsetMask) >> PartitionPageShift();
  // Index 0 is invalid because it is the super page extent metadata and the
  // last index is invalid because the whole PartitionPage is set as guard
  // pages. This repeats part of the payload PA_DCHECK above, which also checks
  // for other exclusions.
  PA_DCHECK(partition_page_index);
  PA_DCHECK(partition_page_index < NumPartitionPagesPerSuperPage() - 1);
  return PartitionSuperPageToMetadataArea(super_page) + partition_page_index;
}

// Converts from a pointer to the SlotSpanMetadata object (within a super
// pages's metadata) into a pointer to the beginning of the slot span. This
// works on direct maps too.
PA_ALWAYS_INLINE uintptr_t
SlotSpanMetadata<MetadataKind::kReadOnly>::ToSlotSpanStart(
    const SlotSpanMetadata<MetadataKind::kReadOnly>* slot_span) {
  uintptr_t pointer_as_uint = reinterpret_cast<uintptr_t>(slot_span);
  uintptr_t super_page_offset = (pointer_as_uint & kSuperPageOffsetMask);

  // A valid |page| must be past the first guard System page and within
  // the following metadata region.
  PA_DCHECK(super_page_offset > SystemPageSize());
  // Must be less than total metadata region.
  PA_DCHECK(super_page_offset <
            SystemPageSize() +
                (NumPartitionPagesPerSuperPage() * kPageMetadataSize));
  uintptr_t partition_page_index =
      (super_page_offset - SystemPageSize()) >> kPageMetadataShift;
  // Index 0 is invalid because it is the super page extent metadata and the
  // last index is invalid because the whole PartitionPage is set as guard
  // pages.
  PA_DCHECK(partition_page_index);
  PA_DCHECK(partition_page_index < NumPartitionPagesPerSuperPage() - 1);
  uintptr_t super_page_base = (pointer_as_uint & kSuperPageBaseMask);
  return super_page_base + (partition_page_index << PartitionPageShift());
}

// Converts an address inside a slot span into a pointer to the SlotSpanMetadata
// object (within super pages's metadata) that describes the slot span
// containing that slot.
//
// CAUTION! For direct-mapped allocation, |address| has to be within the first
// partition page.
PA_ALWAYS_INLINE SlotSpanMetadata<MetadataKind::kReadOnly>*
SlotSpanMetadata<MetadataKind::kReadOnly>::FromAddr(uintptr_t address) {
  auto* page_metadata =
      PartitionPageMetadata<MetadataKind::kReadOnly>::FromAddr(address);
  PA_DCHECK(page_metadata->is_valid);
  // Partition pages in the same slot span share the same SlotSpanMetadata
  // object (located in the first PartitionPageMetadata object of that span).
  // Adjust for that.
  page_metadata -= page_metadata->slot_span_metadata_offset;
  PA_DCHECK(page_metadata->is_valid);
  PA_DCHECK(!page_metadata->slot_span_metadata_offset);
  auto* slot_span = &page_metadata->slot_span_metadata;
  PA_DCHECK(DeducedRootIsValid(slot_span));
  // For direct map, if |address| doesn't point within the first partition page,
  // |slot_span_metadata_offset| will be 0, |page_metadata| won't get shifted,
  // leaving |slot_size| at 0.
  PA_DCHECK(slot_span->bucket->slot_size);
  return slot_span;
}

// Like |FromAddr|, but asserts that |slot_start| indeed points to the
// beginning of a slot. It doesn't check if the slot is actually allocated.
//
// This works on direct maps too.
PA_ALWAYS_INLINE SlotSpanMetadata<MetadataKind::kReadOnly>*
SlotSpanMetadata<MetadataKind::kReadOnly>::FromSlotStart(uintptr_t slot_start) {
  auto* slot_span = FromAddr(slot_start);
#if PA_BUILDFLAG(DCHECKS_ARE_ON)
  // Checks that the pointer is a multiple of slot size.
  uintptr_t slot_span_start = ToSlotSpanStart(slot_span);
  PA_DCHECK(!((slot_start - slot_span_start) % slot_span->bucket->slot_size));
#endif  // PA_BUILDFLAG(DCHECKS_ARE_ON)
  return slot_span;
}

// Like |FromAddr|, but asserts that |object| indeed points to the beginning of
// an object. It doesn't check if the object is actually allocated.
//
// This works on direct maps too.
PA_ALWAYS_INLINE SlotSpanMetadata<MetadataKind::kReadOnly>*
SlotSpanMetadata<MetadataKind::kReadOnly>::FromObject(const void* object) {
  uintptr_t object_addr = ObjectPtr2Addr(object);
  auto* slot_span = FromAddr(object_addr);
  DCheckIsValidObjectAddress(slot_span, object_addr);
  return slot_span;
}

// Like |FromAddr|, but asserts that |address| indeed points within an object.
// It doesn't check if the object is actually allocated.
//
// CAUTION! For direct-mapped allocation, |address| has to be within the first
// partition page.
PA_ALWAYS_INLINE SlotSpanMetadata<MetadataKind::kReadOnly>* SlotSpanMetadata<
    MetadataKind::kReadOnly>::FromObjectInnerAddr(uintptr_t address) {
  auto* slot_span = FromAddr(address);
#if PA_BUILDFLAG(DCHECKS_ARE_ON)
  // Checks that the address is within the expected object boundaries.
  uintptr_t slot_span_start = ToSlotSpanStart(slot_span);
  uintptr_t shift_from_slot_start =
      (address - slot_span_start) % slot_span->bucket->slot_size;
  DCheckIsValidShiftFromSlotStart(slot_span, shift_from_slot_start);
#endif  // PA_BUILDFLAG(DCHECKS_ARE_ON)
  return slot_span;
}

PA_ALWAYS_INLINE SlotSpanMetadata<MetadataKind::kReadOnly>*
SlotSpanMetadata<MetadataKind::kReadOnly>::FromObjectInnerPtr(const void* ptr) {
  return FromObjectInnerAddr(ObjectInnerPtr2Addr(ptr));
}

PA_ALWAYS_INLINE void SlotSpanMetadata<MetadataKind::kWritable>::SetRawSize(
    size_t raw_size) {
  PA_DCHECK(CanStoreRawSize());
  auto* subsequent_page_metadata = GetSubsequentPageMetadata(
      reinterpret_cast<PartitionPageMetadata<MetadataKind::kWritable>*>(this));
  subsequent_page_metadata->raw_size = raw_size;
}

PA_ALWAYS_INLINE size_t
SlotSpanMetadata<MetadataKind::kReadOnly>::GetRawSize() const {
  PA_DCHECK(CanStoreRawSize());
  const auto* subsequent_page_metadata = GetSubsequentPageMetadata(
      reinterpret_cast<const PartitionPageMetadata<MetadataKind::kReadOnly>*>(
          this));
  return subsequent_page_metadata->raw_size;
}

PA_ALWAYS_INLINE void
SlotSpanMetadata<MetadataKind::kWritable>::SetFreelistHead(
    PartitionFreelistEntry* new_head,
    [[maybe_unused]] PartitionRoot* root) {
#if PA_BUILDFLAG(DCHECKS_ARE_ON)
  // |this| is in the metadata region, hence isn't MTE-tagged. Untag |new_head|
  // as well.
  uintptr_t new_head_untagged = UntagPtr(new_head);
  PA_DCHECK(!new_head ||
            (reinterpret_cast<uintptr_t>(ToReadOnly(root)) &
             kSuperPageBaseMask) == (new_head_untagged & kSuperPageBaseMask));
#endif  // PA_BUILDFLAG(DCHECKS_ARE_ON)
  freelist_head = new_head;
  // Inserted something new in the freelist, assume that it is not sorted
  // anymore.
  freelist_is_sorted_ = false;
}

PA_ALWAYS_INLINE PartitionFreelistEntry*
SlotSpanMetadata<MetadataKind::kWritable>::PopForAlloc(
    size_t size,
    const PartitionFreelistDispatcher* freelist_dispatcher) {
  // Not using bucket->slot_size directly as the compiler doesn't know that
  // |bucket->slot_size| is the same as |size|.
  PA_DCHECK(size == bucket->slot_size);
  PartitionFreelistEntry* result = freelist_head;
  // Not setting freelist_is_sorted_ to false since this doesn't destroy
  // ordering.
  freelist_head = freelist_dispatcher->GetNext(freelist_head, size);

  num_allocated_slots++;
  return result;
}

PA_ALWAYS_INLINE void SlotSpanMetadata<MetadataKind::kWritable>::Free(
    uintptr_t slot_start,
    PartitionRoot* root,
    const PartitionFreelistDispatcher* freelist_dispatcher)
    // PartitionRootLock() is not defined inside partition_page.h, but
    // static analysis doesn't require the implementation.
    PA_EXCLUSIVE_LOCKS_REQUIRED(PartitionRootLock(root)) {
  DCheckRootLockIsAcquired(root);
  auto* entry =
      static_cast<PartitionFreelistEntry*>(SlotStartAddr2Ptr(slot_start));
  // Catches an immediate double free.
  PA_CHECK(entry != freelist_head);

  // Look for double free one level deeper in debug.
  PA_DCHECK(!freelist_head || entry != freelist_dispatcher->GetNext(
                                           freelist_head, bucket->slot_size));
  freelist_dispatcher->SetNext(entry, freelist_head);
  SetFreelistHead(entry, root);
  // A best effort double-free check. Works only on empty slot spans.
  PA_CHECK(num_allocated_slots);
  --num_allocated_slots;
  // If the span is marked full, or became empty, take the slow path to update
  // internal state.
  if (marked_full || num_allocated_slots == 0) [[unlikely]] {
    FreeSlowPath(1, root);
  } else {
    // All single-slot allocations must go through the slow path to
    // correctly update the raw size.
    PA_DCHECK(!CanStoreRawSize());
  }
}

PA_ALWAYS_INLINE void SlotSpanMetadata<MetadataKind::kWritable>::AppendFreeList(
    PartitionFreelistEntry* head,
    PartitionFreelistEntry* tail,
    size_t number_of_freed,
    PartitionRoot* root,
    const PartitionFreelistDispatcher* freelist_dispatcher)
    PA_EXCLUSIVE_LOCKS_REQUIRED(PartitionRootLock(root)) {
#if PA_BUILDFLAG(DCHECKS_ARE_ON)
  DCheckRootLockIsAcquired(root);
  PA_DCHECK(!(freelist_dispatcher->GetNext(tail, bucket->slot_size)));
  PA_DCHECK(number_of_freed);
  PA_DCHECK(num_allocated_slots);
  if (CanStoreRawSize()) {
    PA_DCHECK(number_of_freed == 1);
  }
  {
    size_t number_of_entries = 0;
    for (auto* entry = head; entry;
         entry = freelist_dispatcher->GetNext(entry, bucket->slot_size),
               ++number_of_entries) {
      uintptr_t untagged_entry = UntagPtr(entry);
      // Check that all entries belong to this slot span.
      PA_DCHECK(SlotSpanMetadata<MetadataKind::kReadOnly>::ToSlotSpanStart(
                    ToReadOnly(root)) <= untagged_entry);
      PA_DCHECK(untagged_entry <
                SlotSpanMetadata<MetadataKind::kReadOnly>::ToSlotSpanStart(
                    ToReadOnly(root)) +
                    bucket->get_bytes_per_span());
    }
    PA_DCHECK(number_of_entries == number_of_freed);
  }
#endif  // PA_BUILDFLAG(DCHECKS_ARE_ON)

  freelist_dispatcher->SetNext(tail, freelist_head);
  SetFreelistHead(head, root);
  PA_DCHECK(num_allocated_slots >= number_of_freed);
  num_allocated_slots -= number_of_freed;
  // If the span is marked full, or became empty, take the slow path to update
  // internal state.
  if (marked_full || num_allocated_slots == 0) [[unlikely]] {
    FreeSlowPath(number_of_freed, root);
  } else {
    // All single-slot allocations must go through the slow path to
    // correctly update the raw size.
    PA_DCHECK(!CanStoreRawSize());
  }
}

PA_ALWAYS_INLINE bool SlotSpanMetadata<MetadataKind::kReadOnly>::is_active()
    const {
  PA_DCHECK(this != get_sentinel_slot_span());
  bool ret =
      (num_allocated_slots > 0 && (freelist_head || num_unprovisioned_slots));
  if (ret) {
    PA_DCHECK(!marked_full);
    PA_DCHECK(num_allocated_slots < bucket->get_slots_per_span());
  }
  return ret;
}

PA_ALWAYS_INLINE bool SlotSpanMetadata<MetadataKind::kReadOnly>::is_full()
    const {
  PA_DCHECK(this != get_sentinel_slot_span());
  bool ret = (num_allocated_slots == bucket->get_slots_per_span());
  if (ret) {
    PA_DCHECK(!freelist_head);
    PA_DCHECK(!num_unprovisioned_slots);
    // May or may not be marked full, so don't check for that.
  }
  return ret;
}

PA_ALWAYS_INLINE bool SlotSpanMetadata<MetadataKind::kReadOnly>::is_empty()
    const {
  PA_DCHECK(this != get_sentinel_slot_span());
  return is_empty_internal();
}

PA_ALWAYS_INLINE bool
SlotSpanMetadata<MetadataKind::kReadOnly>::is_decommitted() const {
  PA_DCHECK(this != get_sentinel_slot_span());
  return is_decommitted_internal();
}

PA_ALWAYS_INLINE void SlotSpanMetadata<MetadataKind::kWritable>::Reset() {
#if PA_BUILDFLAG(DCHECKS_ARE_ON)
  PA_DCHECK(is_decommitted_internal());
#endif  // PA_BUILDFLAG(DCHECKS_ARE_ON)

  size_t num_slots_per_span = bucket->get_slots_per_span();
  PA_DCHECK(num_slots_per_span <= kMaxSlotsPerSlotSpan);
  num_unprovisioned_slots = static_cast<uint16_t>(num_slots_per_span);
  PA_DCHECK(num_unprovisioned_slots);

  IncrementNumberOfNonemptySlotSpans();

  next_slot_span = nullptr;
}

// Iterates over all slot spans in a super-page. |Callback| must return true if
// early return is needed.
template <typename Callback>
void IterateSlotSpans(uintptr_t super_page, Callback callback) {
#if PA_BUILDFLAG(DCHECKS_ARE_ON)
  PA_DCHECK(!(super_page % kSuperPageAlignment));
  auto* extent_entry = PartitionSuperPageToExtent(super_page);
  DCheckRootLockIsAcquired(extent_entry->root);
#endif  // PA_BUILDFLAG(DCHECKS_ARE_ON)

  auto* const first_page_metadata =
      PartitionPageMetadata<MetadataKind::kReadOnly>::FromAddr(
          SuperPagePayloadBegin(super_page));
  auto* const last_page_metadata =
      PartitionPageMetadata<MetadataKind::kReadOnly>::FromAddr(
          SuperPagePayloadEnd(super_page) - PartitionPageSize());
  PartitionPageMetadata<MetadataKind::kReadOnly>* page_metadata = nullptr;
  SlotSpanMetadata<MetadataKind::kReadOnly>* slot_span = nullptr;
  for (page_metadata = first_page_metadata;
       page_metadata <= last_page_metadata;) {
    PA_DCHECK(!page_metadata
                   ->slot_span_metadata_offset);  // Ensure slot span beginning.
    if (!page_metadata->is_valid) {
      if (page_metadata->has_valid_span_after_this) {
        // page_metadata doesn't represent a valid slot span, but there is
        // another one somewhere after this. Keep iterating to find it.
        ++page_metadata;
        continue;
      }
      // There are currently no valid spans from here on. No need to iterate
      // the rest of the super page_metadata.
      break;
    }
    slot_span = &page_metadata->slot_span_metadata;
    if (callback(slot_span)) {
      return;
    }
    page_metadata += slot_span->bucket->get_pages_per_slot_span();
  }
  // Each super page must have at least one valid slot span.
  PA_DCHECK(page_metadata > first_page_metadata);
  // Just a quick check that the search ended at a valid slot span and there
  // was no unnecessary iteration over gaps afterwards.
  PA_DCHECK(page_metadata ==
            reinterpret_cast<PartitionPageMetadata<MetadataKind::kReadOnly>*>(
                slot_span) +
                slot_span->bucket->get_pages_per_slot_span());
}

// Helper class derived from the implementation of `SlotSpanMetadata`
// that can (but does not _have_ to) enforce that it is in fact a slot
// start.
//
// Behavior is not well-defined if this class is used outside
// PartitionAlloc internals, e.g. if PA is deferring to sanitizers.
// In such cases, the return value from PA's `Alloc()` may not be
// a slot start - it might not be managed by PartitionAlloc at all.
class PA_COMPONENT_EXPORT(PARTITION_ALLOC) SlotStart {
 public:
  template <bool enforce = PA_CONFIG(ENFORCE_SLOT_STARTS)>
  PA_ALWAYS_INLINE static SlotStart FromUntaggedAddr(
      uintptr_t untagged_slot_start) {
    auto result = SlotStart(untagged_slot_start);
    if constexpr (enforce) {
      result.CheckIsSlotStart();
    }
    return result;
  }

  template <bool enforce = PA_CONFIG(ENFORCE_SLOT_STARTS)>
  PA_ALWAYS_INLINE static SlotStart FromObject(const void* tagged_object) {
    uintptr_t untagged_slot_start =
        internal::UntagAddr(reinterpret_cast<uintptr_t>(tagged_object));
    return SlotStart::FromUntaggedAddr<enforce>(untagged_slot_start);
  }

  // Tagging objects is not free. Avoid calling this repeatedly.
  PA_ALWAYS_INLINE void* ToObject() const {
    return internal::TagAddr(untagged_slot_start_);
  }

  PA_ALWAYS_INLINE
  void CheckIsSlotStart() const {
    auto* slot_span_metadata =
        SlotSpanMetadata<MetadataKind::kReadOnly>::FromAddr(
            untagged_slot_start_);
    uintptr_t slot_span =
        SlotSpanMetadata<MetadataKind::kReadOnly>::ToSlotSpanStart(
            slot_span_metadata);
    PA_CHECK(!((untagged_slot_start_ - slot_span) %
               slot_span_metadata->bucket->slot_size));
  }

  uintptr_t untagged_slot_start_;

 private:
  PA_ALWAYS_INLINE
  explicit SlotStart(uintptr_t untagged_slot_start)
      : untagged_slot_start_(untagged_slot_start) {}
};

}  // namespace partition_alloc::internal

#endif  // PARTITION_ALLOC_PARTITION_PAGE_H_
