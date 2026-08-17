// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_PARTITION_DCHECK_HELPER_H_
#define PARTITION_ALLOC_PARTITION_DCHECK_HELPER_H_

#include "partition_alloc/partition_alloc_base/compiler_specific.h"
#include "partition_alloc/partition_alloc_base/component_export.h"
#include "partition_alloc/partition_alloc_forward.h"
#include "partition_alloc/partition_lock.h"

namespace partition_alloc::internal {

template <MetadataKind kind>
struct PartitionSuperPageExtentEntry;

#if PA_BUILDFLAG(DCHECKS_ARE_ON)

// To allow these asserts to have empty bodies in no-DCHECK() builds, while
// avoiding issues with circular includes.
#define PA_EMPTY_BODY_IF_DCHECK_IS_OFF()
// Export symbol if dcheck-is-on. Because the body is not empty.
#define PA_EXPORT_IF_DCHECK_IS_ON() PA_COMPONENT_EXPORT(PARTITION_ALLOC)

#else  // PA_BUILDFLAG(DCHECKS_ARE_ON)

// The static_assert() eats follow-on semicolons.
#define PA_EMPTY_BODY_IF_DCHECK_IS_OFF() \
  {}                                     \
  static_assert(true)
// inline if dcheck-is-off so it's no overhead.
#define PA_EXPORT_IF_DCHECK_IS_ON() PA_ALWAYS_INLINE

#endif  // PA_BUILDFLAG(DCHECKS_ARE_ON)

PA_EXPORT_IF_DCHECK_IS_ON()
void DCheckNumberOfPartitionPagesInSuperPagePayload(
    PartitionSuperPageExtentEntry<MetadataKind::kWritable>* entry,
    const PartitionRoot* root,
    size_t number_of_nonempty_slot_spans) PA_EMPTY_BODY_IF_DCHECK_IS_OFF();

PA_EXPORT_IF_DCHECK_IS_ON()
void DCheckIsValidShiftFromSlotStart(
    const SlotSpanMetadata<MetadataKind::kReadOnly>* slot_span,
    size_t shift_from_slot_start) PA_EMPTY_BODY_IF_DCHECK_IS_OFF();

// Checks that the object is a multiple of slot size (i.e. at a slot start).
PA_EXPORT_IF_DCHECK_IS_ON()
void DCheckIsValidObjectAddress(
    const SlotSpanMetadata<MetadataKind::kReadOnly>* slot_span,
    uintptr_t object_addr) PA_EMPTY_BODY_IF_DCHECK_IS_OFF();

PA_EXPORT_IF_DCHECK_IS_ON()
void DCheckRootLockIsAcquired(PartitionRoot* root)
    PA_EMPTY_BODY_IF_DCHECK_IS_OFF();

// This is not a `DCHECK()`, but historically it sat in here. It's
// implemented in terms of `PartitionRoot` but also used by
// `partition_page.h`, and so can't be moved into the latter (layering
// violation).
PA_COMPONENT_EXPORT(PARTITION_ALLOC)
bool DeducedRootIsValid(SlotSpanMetadata<MetadataKind::kReadOnly>* slot_span);

}  // namespace partition_alloc::internal

#endif  // PARTITION_ALLOC_PARTITION_DCHECK_HELPER_H_
