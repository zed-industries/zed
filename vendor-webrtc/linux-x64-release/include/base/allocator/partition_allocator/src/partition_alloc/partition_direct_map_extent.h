// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_PARTITION_DIRECT_MAP_EXTENT_H_
#define PARTITION_ALLOC_PARTITION_DIRECT_MAP_EXTENT_H_

#include "partition_alloc/partition_alloc_base/compiler_specific.h"
#include "partition_alloc/partition_alloc_check.h"
#include "partition_alloc/partition_bucket.h"
#include "partition_alloc/partition_page.h"

namespace partition_alloc::internal {

template <MetadataKind kind>
struct PartitionDirectMapExtent;

template <MetadataKind kind>
struct PartitionDirectMapExtentBase {
  MaybeConstT<kind, PartitionDirectMapExtent<MetadataKind::kReadOnly>*>
      next_extent;
  MaybeConstT<kind, PartitionDirectMapExtent<MetadataKind::kReadOnly>*>
      prev_extent;
  MaybeConstT<kind, const PartitionBucket*> bucket;

  // Size of the entire reservation, including guard pages, meta-data,
  // padding for alignment before allocation, and padding for granularity at the
  // end of the allocation.
  MaybeConstT<kind, size_t> reservation_size;

  // Padding between the first partition page (guard pages + meta-data) and
  // the allocation.
  MaybeConstT<kind, size_t> padding_for_alignment;
};

template <MetadataKind kind>
struct PartitionDirectMapExtent;

template <>
struct PartitionDirectMapExtent<MetadataKind::kReadOnly>
    : public PartitionDirectMapExtentBase<MetadataKind::kReadOnly> {
  PA_ALWAYS_INLINE static PartitionDirectMapExtent<MetadataKind::kReadOnly>*
  FromSlotSpanMetadata(SlotSpanMetadata<MetadataKind::kReadOnly>* slot_span);

  PA_ALWAYS_INLINE PartitionDirectMapExtent<MetadataKind::kWritable>*
  ToWritable(const PartitionRoot* root);

#if PA_BUILDFLAG(DCHECKS_ARE_ON)
  PA_ALWAYS_INLINE PartitionDirectMapExtent<MetadataKind::kReadOnly>*
  ToReadOnly();
#endif  // PA_BUILDFLAG(DCHECKS_ARE_ON)

 private:
  // In order to resolve circular dependencies, i.e. ToWritable() needs
  // PartitionRoot, define template method: ToWritableInternal() and
  // ToWritable() uses it.
  template <typename T>
  PartitionDirectMapExtent<MetadataKind::kWritable>* ToWritableInternal(
      [[maybe_unused]] T* root);
};

template <>
struct PartitionDirectMapExtent<MetadataKind::kWritable>
    : public PartitionDirectMapExtentBase<MetadataKind::kWritable> {
#if PA_BUILDFLAG(DCHECKS_ARE_ON)
  PA_ALWAYS_INLINE PartitionDirectMapExtent<MetadataKind::kReadOnly>*
  ToReadOnly(const PartitionRoot* root);

 private:
  // In order to resolve circular dependencies, i.e. ToReadOnly() needs
  // PartitionRoot, define template method: ToReadOnlyInternal() and
  // ToReadOnly() uses it.
  template <typename T>
  PartitionDirectMapExtent<MetadataKind::kReadOnly>* ToReadOnlyInternal(
      [[maybe_unused]] T* root);
#endif  // PA_BUILDFLAG(DCHECKS_ARE_ON)
};

// Metadata page for direct-mapped allocations.
template <MetadataKind kind>
struct PartitionDirectMapMetadataBase {
  // |page_metadata| and |second_page_metadata| are needed to match the
  // layout of normal buckets (specifically, of single-slot slot spans), with
  // the caveat that only the first subsequent page is needed (for
  // SubsequentPageMetadata) and others aren't used for direct map.
  PartitionPageMetadata<kind> page_metadata;
  PartitionPageMetadata<kind> second_page_metadata;

  // The following fields are metadata specific to direct map allocations. All
  // these fields will easily fit into the precalculated metadata region,
  // because a direct map allocation starts no further than half way through the
  // super page.
  MaybeConstT<kind, PartitionBucket> bucket;

  PartitionDirectMapExtent<kind> direct_map_extent;
};

template <MetadataKind kind>
struct PartitionDirectMapMetadata;

template <>
struct PartitionDirectMapMetadata<MetadataKind::kReadOnly>
    : public PartitionDirectMapMetadataBase<MetadataKind::kReadOnly> {
  PA_ALWAYS_INLINE static PartitionDirectMapMetadata<MetadataKind::kReadOnly>*
  FromSlotSpanMetadata(SlotSpanMetadata<MetadataKind::kReadOnly>* slot_span);

  PA_ALWAYS_INLINE PartitionDirectMapMetadata<MetadataKind::kWritable>*
  ToWritable(const PartitionRoot* root);

#if PA_BUILDFLAG(DCHECKS_ARE_ON)
  PA_ALWAYS_INLINE PartitionDirectMapMetadata<MetadataKind::kReadOnly>*
  ToReadOnly();
#endif  // PA_BUILDFLAG(DCHECKS_ARE_ON)

 private:
  // In order to resolve circular dependencies, i.e. ToWritable() needs
  // PartitionRoot, define template method: ToWritableInternal() and
  // ToWritable() uses it.
  template <typename T>
  PartitionDirectMapMetadata<MetadataKind::kWritable>* ToWritableInternal(
      [[maybe_unused]] T* root);
};

template <>
struct PartitionDirectMapMetadata<MetadataKind::kWritable>
    : public PartitionDirectMapMetadataBase<MetadataKind::kWritable> {
#if PA_BUILDFLAG(DCHECKS_ARE_ON)
  PA_ALWAYS_INLINE PartitionDirectMapMetadata<MetadataKind::kReadOnly>*
  ToReadOnly(const PartitionRoot* root);

 private:
  // In order to resolve circular dependencies, i.e. ToReadOnly() needs
  // PartitionRoot, define template method: ToReadOnlyInternal() and
  // ToReadOnly() uses it.
  template <typename T>
  PartitionDirectMapMetadata<MetadataKind::kReadOnly>* ToReadOnlyInternal(
      [[maybe_unused]] T* root);
#endif  // PA_BUILDFLAG(DCHECKS_ARE_ON)
};

PA_ALWAYS_INLINE PartitionDirectMapMetadata<MetadataKind::kReadOnly>*
PartitionDirectMapMetadata<MetadataKind::kReadOnly>::FromSlotSpanMetadata(
    SlotSpanMetadata<MetadataKind::kReadOnly>* slot_span) {
  PA_DCHECK(slot_span->bucket->is_direct_mapped());
  // |*slot_span| is the first field of |PartitionDirectMapMetadata|, just cast.
  auto* metadata =
      reinterpret_cast<PartitionDirectMapMetadata<MetadataKind::kReadOnly>*>(
          slot_span);
  PA_DCHECK(&metadata->page_metadata.slot_span_metadata == slot_span);
  return metadata;
}

PA_ALWAYS_INLINE PartitionDirectMapExtent<MetadataKind::kReadOnly>*
PartitionDirectMapExtent<MetadataKind::kReadOnly>::FromSlotSpanMetadata(
    SlotSpanMetadata<MetadataKind::kReadOnly>* slot_span) {
  PA_DCHECK(slot_span->bucket->is_direct_mapped());
  return &PartitionDirectMapMetadata<
              MetadataKind::kReadOnly>::FromSlotSpanMetadata(slot_span)
              ->direct_map_extent;
}

PA_ALWAYS_INLINE PartitionDirectMapMetadata<MetadataKind::kWritable>*
PartitionDirectMapMetadata<MetadataKind::kReadOnly>::ToWritable(
    const PartitionRoot* root) {
  return ToWritableInternal(root);
}

template <typename T>
PartitionDirectMapMetadata<MetadataKind::kWritable>*
PartitionDirectMapMetadata<MetadataKind::kReadOnly>::ToWritableInternal(
    [[maybe_unused]] T* root) {
#if PA_CONFIG(ENABLE_SHADOW_METADATA)
  return reinterpret_cast<PartitionDirectMapMetadata<MetadataKind::kWritable>*>(
      reinterpret_cast<intptr_t>(this) + root->ShadowPoolOffset());
#else
  return reinterpret_cast<PartitionDirectMapMetadata<MetadataKind::kWritable>*>(
      this);
#endif  // PA_CONFIG(ENABLE_SHADOW_METADATA)
}

PA_ALWAYS_INLINE PartitionDirectMapExtent<MetadataKind::kWritable>*
PartitionDirectMapExtent<MetadataKind::kReadOnly>::ToWritable(
    const PartitionRoot* root) {
  return ToWritableInternal(root);
}

template <typename T>
PartitionDirectMapExtent<MetadataKind::kWritable>*
PartitionDirectMapExtent<MetadataKind::kReadOnly>::ToWritableInternal(
    [[maybe_unused]] T* root) {
#if PA_CONFIG(ENABLE_SHADOW_METADATA)
  return reinterpret_cast<PartitionDirectMapExtent<MetadataKind::kWritable>*>(
      reinterpret_cast<intptr_t>(this) + root->ShadowPoolOffset());
#else
  return reinterpret_cast<PartitionDirectMapExtent<MetadataKind::kWritable>*>(
      this);
#endif  // PA_CONFIG(ENABLE_SHADOW_METADATA)
}

#if PA_BUILDFLAG(DCHECKS_ARE_ON)
PA_ALWAYS_INLINE PartitionDirectMapMetadata<MetadataKind::kReadOnly>*
PartitionDirectMapMetadata<MetadataKind::kReadOnly>::ToReadOnly() {
  return this;
}

PA_ALWAYS_INLINE PartitionDirectMapMetadata<MetadataKind::kReadOnly>*
PartitionDirectMapMetadata<MetadataKind::kWritable>::ToReadOnly(
    const PartitionRoot* root) {
  return ToReadOnlyInternal(root);
}

template <typename T>
PartitionDirectMapMetadata<MetadataKind::kReadOnly>*
PartitionDirectMapMetadata<MetadataKind::kWritable>::ToReadOnlyInternal(
    [[maybe_unused]] T* root) {
#if PA_CONFIG(ENABLE_SHADOW_METADATA)
  return reinterpret_cast<PartitionDirectMapMetadata<MetadataKind::kReadOnly>*>(
      reinterpret_cast<intptr_t>(this) - root->ShadowPoolOffset());
#else
  // must be no-op.
  return reinterpret_cast<PartitionDirectMapMetadata<MetadataKind::kReadOnly>*>(
      this);
#endif  // PA_CONFIG(ENABLE_SHADOW_METADATA)
}

PA_ALWAYS_INLINE PartitionDirectMapExtent<MetadataKind::kReadOnly>*
PartitionDirectMapExtent<MetadataKind::kReadOnly>::ToReadOnly() {
  return this;
}

PA_ALWAYS_INLINE PartitionDirectMapExtent<MetadataKind::kReadOnly>*
PartitionDirectMapExtent<MetadataKind::kWritable>::ToReadOnly(
    const PartitionRoot* root) {
  return ToReadOnlyInternal(root);
}

template <typename T>
PartitionDirectMapExtent<MetadataKind::kReadOnly>*
PartitionDirectMapExtent<MetadataKind::kWritable>::ToReadOnlyInternal(
    [[maybe_unused]] T* root) {
#if PA_CONFIG(ENABLE_SHADOW_METADATA)
  return reinterpret_cast<PartitionDirectMapExtent<MetadataKind::kReadOnly>*>(
      reinterpret_cast<intptr_t>(this) - root->ShadowPoolOffset());
#else
  return reinterpret_cast<PartitionDirectMapExtent<MetadataKind::kReadOnly>*>(
      this);
#endif  // PA_CONFIG(ENABLE_SHADOW_METADATA)
}
#endif  // PA_BUILDFLAG(DCHECKS_ARE_ON)

}  // namespace partition_alloc::internal

#endif  // PARTITION_ALLOC_PARTITION_DIRECT_MAP_EXTENT_H_
