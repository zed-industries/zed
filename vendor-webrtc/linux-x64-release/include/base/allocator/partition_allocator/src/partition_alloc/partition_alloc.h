// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_PARTITION_ALLOC_H_
#define PARTITION_ALLOC_PARTITION_ALLOC_H_

#include "partition_alloc/partition_alloc_base/compiler_specific.h"
#include "partition_alloc/partition_alloc_base/component_export.h"
#include "partition_alloc/partition_alloc_forward.h"
#include "partition_alloc/partition_oom.h"
#include "partition_alloc/partition_root.h"

// *** HOUSEKEEPING RULES ***
//
// Throughout PartitionAlloc code, we avoid using generic variable names like
// |ptr| or |address|, and prefer names like |object|, |slot_start|, instead.
// This helps emphasize that terms like "object" and "slot" represent two
// different worlds. "Slot" is an indivisible allocation unit, internal to
// PartitionAlloc. It is generally represented as an address (uintptr_t), since
// arithmetic operations on it aren't uncommon, and for that reason it isn't
// MTE-tagged either. "Object" is the allocated memory that the app is given via
// interfaces like Alloc(), Free(), etc. An object is fully contained within a
// slot, and may be surrounded by internal PartitionAlloc structures or empty
// space. Is is generally represented as a pointer to its beginning (most
// commonly void*), and is MTE-tagged so it's safe to access.
//
// The best way to transition between these to worlds is via
// PartitionRoot::ObjectToSlotStart() and ::SlotStartToObject(). These take care
// of shifting between slot/object start, MTE-tagging/untagging and the cast for
// you. There are cases where these functions are insufficient. Internal
// PartitionAlloc structures, like free-list pointers, BRP ref-count, cookie,
// etc. are located in-slot thus accessing them requires an MTE tag.
// SlotStartPtr2Addr() and SlotStartAddr2Ptr() take care of this.
// There are cases where we have to do pointer arithmetic on an object pointer
// (like check belonging to a pool, etc.), in which case we want to strip MTE
// tag. ObjectInnerPtr2Addr() and ObjectPtr2Addr() take care of that.
//
// Avoid using UntagPtr/Addr() and TagPtr/Addr() directly, if possible. And
// definitely avoid using reinterpret_cast between uintptr_t and pointer worlds.
// When you do, add a comment explaining why it's safe from the point of MTE
// tagging.

namespace partition_alloc {

PA_COMPONENT_EXPORT(PARTITION_ALLOC)
void PartitionAllocGlobalInit(OomFunction on_out_of_memory);
PA_COMPONENT_EXPORT(PARTITION_ALLOC)
void PartitionAllocGlobalUninitForTesting();

struct PA_COMPONENT_EXPORT(PARTITION_ALLOC) PartitionAllocator {
  PartitionAllocator();
  explicit PartitionAllocator(PartitionOptions opts) { init(opts); }
  ~PartitionAllocator();

  void init(PartitionOptions);

  PA_ALWAYS_INLINE PartitionRoot* root() { return &partition_root_; }
  PA_ALWAYS_INLINE const PartitionRoot* root() const {
    return &partition_root_;
  }

 private:
  PartitionRoot partition_root_;
};

}  // namespace partition_alloc

#endif  // PARTITION_ALLOC_PARTITION_ALLOC_H_
