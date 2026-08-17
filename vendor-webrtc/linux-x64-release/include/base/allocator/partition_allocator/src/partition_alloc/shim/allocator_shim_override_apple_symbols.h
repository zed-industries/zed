// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifdef PARTITION_ALLOC_SHIM_ALLOCATOR_SHIM_OVERRIDE_APPLE_SYMBOLS_H_
#error This header is meant to be included only once by allocator_shim.cc
#endif

#ifndef PARTITION_ALLOC_SHIM_ALLOCATOR_SHIM_OVERRIDE_APPLE_SYMBOLS_H_
#define PARTITION_ALLOC_SHIM_ALLOCATOR_SHIM_OVERRIDE_APPLE_SYMBOLS_H_

#include "partition_alloc/buildflags.h"

#if PA_BUILDFLAG(USE_ALLOCATOR_SHIM)
#include "partition_alloc/shim/malloc_zone_functions_apple.h"
#include "partition_alloc/third_party/apple_apsl/malloc.h"

namespace allocator_shim {

MallocZoneFunctions MallocZoneFunctionsToReplaceDefault() {
  MallocZoneFunctions new_functions;
  memset(&new_functions, 0, sizeof(MallocZoneFunctions));
  new_functions.size = [](malloc_zone_t* zone, const void* ptr) -> size_t {
    return ShimGetSizeEstimate(ptr, zone);
  };
  new_functions.good_size = [](malloc_zone_t* zone, size_t size) -> size_t {
    return ShimGoodSize(size, zone);
  };
  new_functions.claimed_address = [](malloc_zone_t* zone,
                                     void* ptr) -> boolean_t {
    return ShimClaimedAddress(ptr, zone);
  };
  new_functions.malloc = [](malloc_zone_t* zone, size_t size) -> void* {
    return ShimMalloc(size, zone);
  };
  new_functions.calloc = [](malloc_zone_t* zone, size_t n,
                            size_t size) -> void* {
    return ShimCalloc(n, size, zone);
  };
  new_functions.valloc = [](malloc_zone_t* zone, size_t size) -> void* {
    return ShimValloc(size, zone);
  };
  new_functions.free = [](malloc_zone_t* zone, void* ptr) {
    ShimFree(ptr, zone);
  };
  new_functions.realloc = [](malloc_zone_t* zone, void* ptr,
                             size_t size) -> void* {
    return ShimRealloc(ptr, size, zone);
  };
  new_functions.batch_malloc = [](struct _malloc_zone_t* zone, size_t size,
                                  void** results,
                                  unsigned num_requested) -> unsigned {
    return ShimBatchMalloc(size, results, num_requested, zone);
  };
  new_functions.batch_free = [](struct _malloc_zone_t* zone, void** to_be_freed,
                                unsigned num_to_be_freed) -> void {
    ShimBatchFree(to_be_freed, num_to_be_freed, zone);
  };
  new_functions.memalign = [](malloc_zone_t* zone, size_t alignment,
                              size_t size) -> void* {
    return ShimMemalign(alignment, size, zone);
  };
  new_functions.free_definite_size = [](malloc_zone_t* zone, void* ptr,
                                        size_t size) {
    ShimFreeWithSize(ptr, size, zone);
  };
  new_functions.try_free_default = [](malloc_zone_t* zone, void* ptr) {
    ShimTryFreeDefault(ptr, zone);
  };
  return new_functions;
}

}  // namespace allocator_shim

#endif  // PA_BUILDFLAG(USE_ALLOCATOR_SHIM)

#endif  // PARTITION_ALLOC_SHIM_ALLOCATOR_SHIM_OVERRIDE_APPLE_SYMBOLS_H_
