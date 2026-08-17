// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_SHIM_ALLOCATOR_INTERCEPTION_APPLE_H_
#define PARTITION_ALLOC_SHIM_ALLOCATOR_INTERCEPTION_APPLE_H_

#include "partition_alloc/buildflags.h"

#if PA_BUILDFLAG(USE_ALLOCATOR_SHIM)
#include <cstddef>

#include "partition_alloc/partition_alloc_base/component_export.h"
#include "partition_alloc/third_party/apple_apsl/malloc.h"

namespace allocator_shim {

struct MallocZoneFunctions;

// This initializes AllocatorDispatch::default_dispatch by saving pointers to
// the functions in the current default malloc zone. This must be called before
// the default malloc zone is changed to have its intended effect.
void InitializeDefaultDispatchToMacAllocator();

// Saves the function pointers currently used by the default zone.
void StoreFunctionsForDefaultZone();

// Same as StoreFunctionsForDefaultZone, but for all malloc zones.
void StoreFunctionsForAllZones();

// For all malloc zones that have been stored, replace their functions with
// |functions|.
void ReplaceFunctionsForStoredZones(const MallocZoneFunctions* functions);

PA_COMPONENT_EXPORT(ALLOCATOR_SHIM) extern bool g_replaced_default_zone;

// Calls the original implementation of malloc/calloc prior to interception.
PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
bool UncheckedMallocMac(size_t size, void** result);
PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
bool UncheckedCallocMac(size_t num_items, size_t size, void** result);

// Intercepts calls to default and purgeable malloc zones. Intercepts Core
// Foundation and Objective-C allocations.
// Has no effect on the default malloc zone if the allocator shim already
// performs that interception.
PA_COMPONENT_EXPORT(ALLOCATOR_SHIM) void InterceptAllocationsMac();

// Updates all malloc zones to use their original functions.
// Also calls ClearAllMallocZonesForTesting.
PA_COMPONENT_EXPORT(ALLOCATOR_SHIM) void UninterceptMallocZonesForTesting();

// Returns true if allocations are successfully being intercepted for all malloc
// zones.
bool AreMallocZonesIntercepted();

// heap_profiling::ProfilingClient needs to shim all malloc zones even ones
// that are registered after the start-up time. ProfilingClient periodically
// calls this API to make it sure that all malloc zones are shimmed.
PA_COMPONENT_EXPORT(ALLOCATOR_SHIM) void ShimNewMallocZones();

// Exposed for testing.
PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
void ReplaceZoneFunctions(ChromeMallocZone* zone,
                          const MallocZoneFunctions* functions);

}  // namespace allocator_shim

#endif  // PA_BUILDFLAG(USE_ALLOCATOR_SHIM)

#endif  // PARTITION_ALLOC_SHIM_ALLOCATOR_INTERCEPTION_APPLE_H_
