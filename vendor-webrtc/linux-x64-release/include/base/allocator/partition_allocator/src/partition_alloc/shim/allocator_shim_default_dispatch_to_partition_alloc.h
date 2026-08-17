// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_SHIM_ALLOCATOR_SHIM_DEFAULT_DISPATCH_TO_PARTITION_ALLOC_H_
#define PARTITION_ALLOC_SHIM_ALLOCATOR_SHIM_DEFAULT_DISPATCH_TO_PARTITION_ALLOC_H_

#include "partition_alloc/buildflags.h"

#if PA_BUILDFLAG(USE_ALLOCATOR_SHIM)
#include "partition_alloc/partition_alloc.h"
#include "partition_alloc/partition_alloc_base/component_export.h"
#include "partition_alloc/shim/allocator_dispatch.h"
#include "partition_alloc/shim/allocator_shim.h"

namespace allocator_shim {

namespace internal {

class PA_COMPONENT_EXPORT(ALLOCATOR_SHIM) PartitionAllocMalloc {
 public:
  // Returns true if ConfigurePartitions() has completed, meaning that the
  // allocators are effectively set in stone.
  static bool AllocatorConfigurationFinalized();

  static partition_alloc::PartitionRoot* Allocator();
  // May return |nullptr|, will never return the same pointer as |Allocator()|.
  static partition_alloc::PartitionRoot* OriginalAllocator();
};

template <partition_alloc::AllocFlags base_alloc_flags,
          partition_alloc::FreeFlags base_free_flags>
class PartitionAllocFunctionsInternal {
 public:
  static void* Malloc(size_t size, void* context);

  static void* MallocUnchecked(size_t size, void* context);

  static void* Calloc(size_t n, size_t size, void* context);

  static void* Memalign(size_t alignment, size_t size, void* context);

  static void* AlignedAlloc(size_t size, size_t alignment, void* context);

  static void* AlignedAllocUnchecked(size_t size,
                                     size_t alignment,
                                     void* context);

  static void* AlignedRealloc(void* address,
                              size_t size,
                              size_t alignment,
                              void* context);

  static void* AlignedReallocUnchecked(void* address,
                                       size_t size,
                                       size_t alignment,
                                       void* context);

  static void* Realloc(void* address, size_t size, void* context);

  static void* ReallocUnchecked(void* address, size_t size, void* context);

  static void Free(void* object, void* context);

  static void FreeWithSize(void* object, size_t size, void* context);

  static void FreeWithAlignment(void* object, size_t alignment, void* context);

  static void FreeWithSizeAndAlignment(void* object,
                                       size_t size,
                                       size_t alignment,
                                       void* context);

  static size_t GetSizeEstimate(void* address, void* context);

#if PA_BUILDFLAG(IS_APPLE)
  static size_t GoodSize(size_t size, void* context);

  static bool ClaimedAddress(void* address, void* context);
#endif  // PA_BUILDFLAG(IS_APPLE)

  static unsigned BatchMalloc(size_t size,
                              void** results,
                              unsigned num_requested,
                              void* context);

  static void BatchFree(void** to_be_freed,
                        unsigned num_to_be_freed,
                        void* context);

#if PA_BUILDFLAG(IS_APPLE)
  static void TryFreeDefault(void* address, void* context);
#endif  // PA_BUILDFLAG(IS_APPLE)

  static constexpr AllocatorDispatch MakeDispatch() {
    return {
        &Malloc,                    // alloc_function
        &MallocUnchecked,           // alloc_unchecked_function
        &Calloc,                    // alloc_zero_initialized_function
        &Memalign,                  // alloc_aligned_function
        &Realloc,                   // realloc_function
        &ReallocUnchecked,          // realloc_unchecked_function
        &Free,                      // free_function
        &FreeWithSize,              // free_with_size_function
        &FreeWithAlignment,         // free_with_alignment_function
        &FreeWithSizeAndAlignment,  // free_with_size_and_alignment_function
        &GetSizeEstimate,           // get_size_estimate_function
#if PA_BUILDFLAG(IS_APPLE)
        &GoodSize,        // good_size
        &ClaimedAddress,  // claimed_address
#else
        nullptr,  // good_size
        nullptr,  // claimed_address
#endif
        &BatchMalloc,  // batch_malloc_function
        &BatchFree,    // batch_free_function
#if PA_BUILDFLAG(IS_APPLE)
        // On Apple OSes, try_free_default() is sometimes called as an
        // optimization of free().
        &TryFreeDefault,
#else
        nullptr,  // try_free_default_function
#endif
        &AlignedAlloc,             // aligned_malloc_function
        &AlignedAllocUnchecked,    // aligned_malloc_unchecked_function
        &AlignedRealloc,           // aligned_realloc_function
        &AlignedReallocUnchecked,  // aligned_realloc_unchecked_function
        &Free,                     // aligned_free_function
        nullptr,                   // next
    };
  }
};

using PartitionAllocFunctions =
    PartitionAllocFunctionsInternal<partition_alloc::AllocFlags::kNoHooks,
                                    partition_alloc::FreeFlags::kNoHooks>;
using PartitionAllocWithAdvancedChecksFunctions =
    PartitionAllocFunctionsInternal<
        partition_alloc::AllocFlags::kNoHooks,
        partition_alloc::FreeFlags::kNoHooks |
            partition_alloc::FreeFlags::kSchedulerLoopQuarantine>;

// `PartitionAllocFunctions` in instantiated in cc file.
extern template class PA_EXPORT_TEMPLATE_DECLARE(
    PA_COMPONENT_EXPORT(ALLOCATOR_SHIM))
    PartitionAllocFunctionsInternal<partition_alloc::AllocFlags::kNoHooks,
                                    partition_alloc::FreeFlags::kNoHooks>;
// `PartitionAllocWithAdvancedChecksFunctions` in instantiated in cc file.
extern template class PA_EXPORT_TEMPLATE_DECLARE(
    PA_COMPONENT_EXPORT(ALLOCATOR_SHIM))
    PartitionAllocFunctionsInternal<
        partition_alloc::AllocFlags::kNoHooks,
        partition_alloc::FreeFlags::kNoHooks |
            partition_alloc::FreeFlags::kSchedulerLoopQuarantine>;

}  // namespace internal

#if PA_BUILDFLAG(USE_PARTITION_ALLOC_AS_MALLOC)
// Provide a ConfigurePartitions() helper, to mimic what Chromium uses. This way
// we're making it more resilient to ConfigurePartitions() interface changes, so
// that we don't have to modify multiple callers. This is particularly important
// when callers are in a different repo, like PDFium or Dawn.
// -----------------------------------------------------------------------------
// DO NOT MODIFY this signature. This is meant for partition_alloc's embedders
// only, so that partition_alloc can evolve without breaking them.
// Chromium/PartitionAlloc are part of the same repo, they must not depend on
// this function. They should call ConfigurePartitions() directly.
PA_ALWAYS_INLINE void ConfigurePartitionsForTesting() {
  auto enable_brp = allocator_shim::EnableBrp(true);
  size_t brp_extra_extras_size = 0;

  // Embedders's tests might benefit from MTE checks. However, this is costly
  // and shouldn't be used in benchmarks.
  auto enable_memory_tagging = allocator_shim::EnableMemoryTagging(
      PA_BUILDFLAG(HAS_MEMORY_TAGGING) && PA_BUILDFLAG(DCHECKS_ARE_ON));

  // Since the only user of this function is a test function, we use
  // synchronous reporting mode, if MTE is enabled.
  auto memory_tagging_reporting_mode =
      enable_memory_tagging
          ? partition_alloc::TagViolationReportingMode::kSynchronous
          : partition_alloc::TagViolationReportingMode::kDisabled;
  auto distribution = BucketDistribution::kNeutral;

  auto scheduler_loop_quarantine_global_config =
      partition_alloc::internal::SchedulerLoopQuarantineConfig();
  auto scheduler_loop_quarantine_thread_local_config =
      partition_alloc::internal::SchedulerLoopQuarantineConfig();

  auto eventually_zero_freed_memory = EventuallyZeroFreedMemory(false);
  auto fewer_memory_regions = FewerMemoryRegions(false);
  auto use_small_single_slot_spans = UseSmallSingleSlotSpans(true);

  ConfigurePartitions(enable_brp, brp_extra_extras_size, enable_memory_tagging,
                      memory_tagging_reporting_mode, distribution,
                      scheduler_loop_quarantine_global_config,
                      scheduler_loop_quarantine_thread_local_config,
                      eventually_zero_freed_memory, fewer_memory_regions,
                      use_small_single_slot_spans);
}
#endif  // PA_BUILDFLAG(USE_PARTITION_ALLOC_AS_MALLOC)

}  // namespace allocator_shim

#endif  // PA_BUILDFLAG(USE_ALLOCATOR_SHIM)

#endif  // PARTITION_ALLOC_SHIM_ALLOCATOR_SHIM_DEFAULT_DISPATCH_TO_PARTITION_ALLOC_H_
