// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.
#ifndef PARTITION_ALLOC_PARTITION_BUCKET_H_
#define PARTITION_ALLOC_PARTITION_BUCKET_H_

#include <cstddef>
#include <cstdint>

#include "partition_alloc/partition_alloc_base/compiler_specific.h"
#include "partition_alloc/partition_alloc_base/component_export.h"
#include "partition_alloc/partition_alloc_base/thread_annotations.h"
#include "partition_alloc/partition_alloc_check.h"
#include "partition_alloc/partition_alloc_constants.h"
#include "partition_alloc/partition_alloc_forward.h"
#include "partition_alloc/partition_page_constants.h"

namespace partition_alloc::internal {

constexpr inline int kPartitionNumSystemPagesPerSlotSpanBits = 8;

// Visible for testing.
PA_COMPONENT_EXPORT(PARTITION_ALLOC)
uint8_t ComputeSystemPagesPerSlotSpan(size_t slot_size,
                                      bool prefer_smaller_slot_spans);

// Visible for testing.
PA_COMPONENT_EXPORT(PARTITION_ALLOC)
bool CompareSlotSpans(const SlotSpanMetadata<MetadataKind::kReadOnly>* a,
                      const SlotSpanMetadata<MetadataKind::kReadOnly>* b);

struct PartitionBucket {
  // Accessed most in hot path => goes first. Only nullptr for invalid buckets,
  // may be pointing to the sentinel.
  SlotSpanMetadata<MetadataKind::kReadOnly>* active_slot_spans_head;

  SlotSpanMetadata<MetadataKind::kReadOnly>* empty_slot_spans_head;
  SlotSpanMetadata<MetadataKind::kReadOnly>* decommitted_slot_spans_head;
  uint32_t slot_size;
  uint32_t num_system_pages_per_slot_span
      : kPartitionNumSystemPagesPerSlotSpanBits;
  uint32_t num_full_slot_spans : 24;

  // `slot_size_reciprocal` is used to improve the performance of
  // `GetSlotOffset`. It is computed as `(1 / size) * (2 ** M)` where M is
  // chosen to provide the desired accuracy. As a result, we can replace a slow
  // integer division (or modulo) operation with a pair of multiplication and a
  // bit shift, i.e. `value / size` becomes `(value * size_reciprocal) >> M`.
  uint64_t slot_size_reciprocal;
  bool can_store_raw_size;

  // This is `M` from the formula above. For accurate results, both `value` and
  // `size`, which are bound by `kMaxBucketed` for our purposes, must be less
  // than `2 ** (M / 2)`. On the other hand, the result of the expression
  // `3 * M / 2` must be less than 64, otherwise integer overflow can occur.
  static constexpr uint64_t kReciprocalShift = 42;
  static constexpr uint64_t kReciprocalMask = (1ull << kReciprocalShift) - 1;
  static_assert(
      kMaxBucketed < (1 << (kReciprocalShift / 2)),
      "GetSlotOffset may produce an incorrect result when kMaxBucketed is too "
      "large.");

  static constexpr size_t kMaxSlotSpansToSort = 200;

  // Public API.
  PA_COMPONENT_EXPORT(PARTITION_ALLOC)
  void Init(uint32_t new_slot_size, bool use_small_single_slot_spans);

  // Sets |is_already_zeroed| to true if the allocation was satisfied by
  // requesting (a) new page(s) from the operating system, or false otherwise.
  // This enables an optimization for when callers use
  // |AllocFlags::kZeroFill|: there is no need to call memset on fresh
  // pages; the OS has already zeroed them. (See
  // |PartitionRoot::AllocFromBucket|.)
  //
  // Note the matching Free() functions are in SlotSpanMetadata.
  PA_NOINLINE PA_COMPONENT_EXPORT(PARTITION_ALLOC) uintptr_t
      SlowPathAlloc(PartitionRoot* root,
                    AllocFlags flags,
                    size_t raw_size,
                    size_t slot_span_alignment,
                    SlotSpanMetadata<MetadataKind::kReadOnly>** slot_span,
                    bool* is_already_zeroed)
          PA_EXCLUSIVE_LOCKS_REQUIRED(PartitionRootLock(root));

  PA_ALWAYS_INLINE bool CanStoreRawSize() const { return can_store_raw_size; }

  // Some buckets are pseudo-buckets, which are disabled because they would
  // otherwise not fulfill alignment constraints.
  PA_ALWAYS_INLINE bool is_valid() const {
    return active_slot_spans_head != nullptr;
  }
  PA_ALWAYS_INLINE bool is_direct_mapped() const {
    return !num_system_pages_per_slot_span;
  }
  PA_ALWAYS_INLINE size_t get_bytes_per_span() const {
    // Cannot overflow, num_system_pages_per_slot_span is a bitfield, and 255
    // pages fit in a size_t.
    static_assert(kPartitionNumSystemPagesPerSlotSpanBits <= 8, "");
    return static_cast<size_t>(num_system_pages_per_slot_span)
           << SystemPageShift();
  }
  PA_ALWAYS_INLINE size_t get_slots_per_span() const {
    size_t ret = GetSlotNumber(get_bytes_per_span());
    PA_DCHECK(ret <= kMaxSlotsPerSlotSpan);
    return ret;
  }
  // Returns a natural number of partition pages (calculated by
  // ComputeSystemPagesPerSlotSpan()) to allocate from the current super page
  // when the bucket runs out of slots.
  PA_ALWAYS_INLINE size_t get_pages_per_slot_span() const {
    // Rounds up to nearest multiple of NumSystemPagesPerPartitionPage().
    return (num_system_pages_per_slot_span +
            (NumSystemPagesPerPartitionPage() - 1)) /
           NumSystemPagesPerPartitionPage();
  }

  // This helper function scans a bucket's active slot span list for a suitable
  // new active slot span.  When it finds a suitable new active slot span (one
  // that has free slots and is not empty), it is set as the new active slot
  // span. If there is no suitable new active slot span, the current active slot
  // span is set to SlotSpanMetadata::get_sentinel_slot_span(). As potential
  // slot spans are scanned, they are tidied up according to their state. Empty
  // slot spans are swept on to the empty list, decommitted slot spans on to the
  // decommitted list and full slot spans are unlinked from any list.
  //
  // This is where the guts of the bucket maintenance is done!
  bool SetNewActiveSlotSpan(PartitionRoot* root);

  // Walks the entire active slot span list, and perform regular maintenance,
  // where empty, decommitted and full slot spans are moved to their
  // steady-state place.
  PA_COMPONENT_EXPORT(PARTITION_ALLOC)
  void MaintainActiveList(PartitionRoot* root);

  // Returns a slot number starting from the beginning of the slot span.
  PA_ALWAYS_INLINE size_t GetSlotNumber(size_t offset_in_slot_span) const {
    // See the static assertion for `kReciprocalShift` above.
    // TODO(casey.smalley@arm.com): triggers on Aarch64/Linux
    // systems with 64k system pages. Constants need to be
    // adjusted to prevent different parts of the allocator
    // from overlapping. For now this will allow 64k pages
    // to function on Aarch64/Linux systems, albeit not
    // very efficiently.
    PA_DCHECK(internal::SystemPageSize() == (size_t{1} << 16) ||
              offset_in_slot_span <= kMaxBucketed);
    PA_DCHECK(slot_size <= kMaxBucketed);

    const size_t offset_in_slot =
        ((offset_in_slot_span * slot_size_reciprocal) >> kReciprocalShift);
    PA_DCHECK(offset_in_slot_span / slot_size == offset_in_slot);

    return offset_in_slot;
  }

  // Sort the freelists of all slot spans.
  void SortSmallerSlotSpanFreeLists(PartitionRoot* root);
  // Sort the active slot span list in ascending freelist length.
  PA_COMPONENT_EXPORT(PARTITION_ALLOC)
  void SortActiveSlotSpans(PartitionRoot* root);

  // We need `AllocNewSuperPageSpan` and `InitializeSlotSpan` to stay
  // PA_ALWAYS_INLINE for speed, but we also need to use them from a separate
  // compilation unit.
  uintptr_t AllocNewSuperPageSpanForGwpAsan(PartitionRoot* root,
                                            size_t super_page_count,
                                            AllocFlags flags)
      PA_EXCLUSIVE_LOCKS_REQUIRED(PartitionRootLock(root));
  void InitializeSlotSpanForGwpAsan(
      SlotSpanMetadata<MetadataKind::kReadOnly>* slot_span,
      PartitionRoot* root);

  size_t SlotSpanCommittedSize(PartitionRoot* root) const;

 private:
  // Sets `this->can_store_raw_size`.
  void InitCanStoreRawSize(bool use_small_single_slot_spans);

  // Allocates several consecutive super pages. Returns the address of the first
  // super page.
  PA_ALWAYS_INLINE uintptr_t AllocNewSuperPageSpan(PartitionRoot* root,
                                                   size_t super_page_count,
                                                   AllocFlags flags)
      PA_EXCLUSIVE_LOCKS_REQUIRED(PartitionRootLock(root));
  // Allocates a new slot span with size |num_partition_pages| from the
  // current extent. Metadata within this slot span will be initialized.
  // Returns nullptr on error.
  PA_ALWAYS_INLINE SlotSpanMetadata<MetadataKind::kReadOnly>* AllocNewSlotSpan(
      PartitionRoot* root,
      AllocFlags flags,
      size_t slot_span_alignment)
      PA_EXCLUSIVE_LOCKS_REQUIRED(PartitionRootLock(root));

  // Allocates a new super page from the current extent, if possible. All
  // slot-spans will be in the decommitted state. Returns the address of the
  // super page's payload, or 0 on error.
  PA_ALWAYS_INLINE uintptr_t AllocNewSuperPage(PartitionRoot* root,
                                               AllocFlags flags)
      PA_EXCLUSIVE_LOCKS_REQUIRED(PartitionRootLock(root));

  // Each bucket allocates a slot span when it runs out of slots.
  // A slot span's size is equal to get_pages_per_slot_span() number of
  // partition pages. This function initializes all PartitionPage within the
  // span to point to the first PartitionPage which holds all the metadata
  // for the span (in PartitionPage::SlotSpanMetadata) and registers this bucket
  // as the owner of the span. It does NOT put the slots into the bucket's
  // freelist.
  PA_ALWAYS_INLINE void InitializeSlotSpan(
      SlotSpanMetadata<MetadataKind::kReadOnly>* slot_span,
      PartitionRoot* root);

  // Initializes a super page. Returns the address of the super page's payload.
  PA_ALWAYS_INLINE uintptr_t InitializeSuperPage(PartitionRoot* root,
                                                 uintptr_t super_page,
                                                 uintptr_t requested_address)
      PA_EXCLUSIVE_LOCKS_REQUIRED(PartitionRootLock(root));
  // Commit 1 or more pages in |slot_span|, enough to get the next slot, which
  // is returned by this function. If more slots fit into the committed pages,
  // they'll be added to the free list of the slot span (note that next pointers
  // are stored inside the slots).
  // The free list must be empty when calling this function.
  //
  // If |slot_span| was freshly allocated, it must have been passed through
  // InitializeSlotSpan() first.
  PA_ALWAYS_INLINE uintptr_t ProvisionMoreSlotsAndAllocOne(
      PartitionRoot* root,
      AllocFlags flags,
      SlotSpanMetadata<MetadataKind::kReadOnly>* slot_span)
      PA_EXCLUSIVE_LOCKS_REQUIRED(PartitionRootLock(root));
};

}  // namespace partition_alloc::internal

#endif  // PARTITION_ALLOC_PARTITION_BUCKET_H_
