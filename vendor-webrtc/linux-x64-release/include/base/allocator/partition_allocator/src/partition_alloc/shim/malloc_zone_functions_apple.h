// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_SHIM_MALLOC_ZONE_FUNCTIONS_APPLE_H_
#define PARTITION_ALLOC_SHIM_MALLOC_ZONE_FUNCTIONS_APPLE_H_

#include "partition_alloc/buildflags.h"

#if PA_BUILDFLAG(USE_ALLOCATOR_SHIM)
#include <malloc/malloc.h>

#include <cstddef>

#include "partition_alloc/partition_alloc_base/component_export.h"
#include "partition_alloc/partition_alloc_base/immediate_crash.h"
#include "partition_alloc/third_party/apple_apsl/malloc.h"

namespace allocator_shim {

typedef void* (*malloc_type)(struct _malloc_zone_t* zone, size_t size);
typedef void* (*calloc_type)(struct _malloc_zone_t* zone,
                             size_t num_items,
                             size_t size);
typedef void* (*valloc_type)(struct _malloc_zone_t* zone, size_t size);
typedef void (*free_type)(struct _malloc_zone_t* zone, void* ptr);
typedef void* (*realloc_type)(struct _malloc_zone_t* zone,
                              void* ptr,
                              size_t size);
typedef void* (*memalign_type)(struct _malloc_zone_t* zone,
                               size_t alignment,
                               size_t size);
typedef unsigned (*batch_malloc_type)(struct _malloc_zone_t* zone,
                                      size_t size,
                                      void** results,
                                      unsigned num_requested);
typedef void (*batch_free_type)(struct _malloc_zone_t* zone,
                                void** to_be_freed,
                                unsigned num_to_be_freed);
typedef void (*free_definite_size_type)(struct _malloc_zone_t* zone,
                                        void* ptr,
                                        size_t size);
typedef void (*try_free_default_type)(struct _malloc_zone_t* zone, void* ptr);
typedef size_t (*size_fn_type)(struct _malloc_zone_t* zone, const void* ptr);
typedef size_t (*good_size_fn_type)(struct _malloc_zone_t* zone, size_t size);
typedef boolean_t (*claimed_address_type)(struct _malloc_zone_t* zone,
                                          void* ptr);

struct MallocZoneFunctions {
  malloc_type malloc;
  calloc_type calloc;
  valloc_type valloc;
  free_type free;
  realloc_type realloc;
  memalign_type memalign;
  batch_malloc_type batch_malloc;
  batch_free_type batch_free;
  free_definite_size_type free_definite_size;
  try_free_default_type try_free_default;
  size_fn_type size;
  good_size_fn_type good_size;
  claimed_address_type claimed_address;
  const ChromeMallocZone* context;
};

PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
void StoreZoneFunctions(const ChromeMallocZone* zone,
                        MallocZoneFunctions* functions);
static constexpr int kMaxZoneCount = 30;
PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
extern MallocZoneFunctions g_malloc_zones[kMaxZoneCount];

// The array g_malloc_zones stores all information about malloc zones before
// they are shimmed. This information needs to be accessed during dispatch back
// into the zone, and additional zones may be added later in the execution fo
// the program, so the array needs to be both thread-safe and high-performance.
//
// We begin by creating an array of MallocZoneFunctions of fixed size. We will
// never modify the container, which provides thread-safety to iterators.  When
// we want to add a MallocZoneFunctions to the container, we:
//   1. Fill in all the fields.
//   2. Update the total zone count.
//   3. Insert a memory barrier.
//   4. Insert our shim.
//
// Each MallocZoneFunctions is uniquely identified by |context|, which is a
// pointer to the original malloc zone. When we wish to dispatch back to the
// original malloc zones, we iterate through the array, looking for a matching
// |context|.
//
// Most allocations go through the default allocator. We will ensure that the
// default allocator is stored as the first MallocZoneFunctions.
//
// Returns whether the zone was successfully stored.
PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
bool StoreMallocZone(ChromeMallocZone* zone);
PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
bool IsMallocZoneAlreadyStored(ChromeMallocZone* zone);
PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
bool DoesMallocZoneNeedReplacing(ChromeMallocZone* zone,
                                 const MallocZoneFunctions* functions);

PA_COMPONENT_EXPORT(ALLOCATOR_SHIM) int GetMallocZoneCountForTesting();
PA_COMPONENT_EXPORT(ALLOCATOR_SHIM) void ClearAllMallocZonesForTesting();

inline MallocZoneFunctions& GetFunctionsForZone(void* zone) {
  for (unsigned int i = 0; i < kMaxZoneCount; ++i) {
    if (g_malloc_zones[i].context == zone) {
      return g_malloc_zones[i];
    }
  }
  PA_IMMEDIATE_CRASH();
}

}  // namespace allocator_shim

#endif  // PA_BUILDFLAG(USE_ALLOCATOR_SHIM)

#endif  // PARTITION_ALLOC_SHIM_MALLOC_ZONE_FUNCTIONS_APPLE_H_
