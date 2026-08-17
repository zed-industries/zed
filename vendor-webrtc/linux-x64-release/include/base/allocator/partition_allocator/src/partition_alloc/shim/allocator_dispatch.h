// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_SHIM_ALLOCATOR_DISPATCH_H_
#define PARTITION_ALLOC_SHIM_ALLOCATOR_DISPATCH_H_

#include <cstddef>

#include "partition_alloc/partition_alloc_check.h"

namespace allocator_shim {

struct AllocatorDispatch {
  using AllocFn = void*(size_t size, void* context);
  using AllocUncheckedFn = void*(size_t size, void* context);
  using AllocZeroInitializedFn = void*(size_t n, size_t size, void* context);
  using AllocAlignedFn = void*(size_t alignment, size_t size, void* context);
  using ReallocFn = void*(void* address, size_t size, void* context);
  using ReallocUncheckedFn = void*(void* ptr, size_t size, void* context);
  using FreeFn = void(void* address, void* context);
  // Returns the allocated size of user data (not including heap overhead).
  // Can be larger than the requested size.
  using GetSizeEstimateFn = size_t(void* address, void* context);
  using GoodSizeFn = size_t(size_t size, void* context);
  using ClaimedAddressFn = bool(void* address, void* context);
  using BatchMallocFn = unsigned(size_t size,
                                 void** results,
                                 unsigned num_requested,
                                 void* context);
  using BatchFreeFn = void(void** to_be_freed,
                           unsigned num_to_be_freed,
                           void* context);
  using FreeWithSizeFn = void(void* ptr, size_t size, void* context);
  using FreeWithAlignmentFn = void(void* ptr, size_t alignment, void* context);
  using FreeWithSizeAndAlignmentFn = void(void* ptr,
                                          size_t size,
                                          size_t alignment,
                                          void* context);
  using TryFreeDefaultFn = void(void* ptr, void* context);
  using AlignedMallocFn = void*(size_t size, size_t alignment, void* context);
  using AlignedMallocUncheckedFn = void*(size_t size,
                                         size_t alignment,
                                         void* context);
  using AlignedReallocFn = void*(void* address,
                                 size_t size,
                                 size_t alignment,
                                 void* context);
  using AlignedReallocUncheckedFn = void*(void* address,
                                          size_t size,
                                          size_t alignment,
                                          void* context);
  using AlignedFreeFn = void(void* address, void* context);

  AllocFn* alloc_function;
  AllocUncheckedFn* alloc_unchecked_function;
  AllocZeroInitializedFn* alloc_zero_initialized_function;
  AllocAlignedFn* alloc_aligned_function;
  ReallocFn* realloc_function;
  ReallocUncheckedFn* realloc_unchecked_function;
  FreeFn* free_function;
  FreeWithSizeFn* free_with_size_function;
  FreeWithAlignmentFn* free_with_alignment_function;
  FreeWithSizeAndAlignmentFn* free_with_size_and_alignment_function;
  GetSizeEstimateFn* get_size_estimate_function;
  GoodSizeFn* good_size_function;
  // claimed_address, batch_malloc, batch_free and
  // try_free_default are specific to the OSX and iOS allocators.
  ClaimedAddressFn* claimed_address_function;
  BatchMallocFn* batch_malloc_function;
  BatchFreeFn* batch_free_function;
  TryFreeDefaultFn* try_free_default_function;
  // _aligned_malloc, _aligned_realloc, and _aligned_free are specific to the
  // Windows allocator.
  AlignedMallocFn* aligned_malloc_function;
  AlignedMallocUncheckedFn* aligned_malloc_unchecked_function;
  AlignedReallocFn* aligned_realloc_function;
  AlignedReallocUncheckedFn* aligned_realloc_unchecked_function;
  AlignedFreeFn* aligned_free_function;

  const AllocatorDispatch* next;

  // |default_dispatch| is statically defined by one (and only one) of the
  // allocator_shim_default_dispatch_to_*.cc files, depending on the build
  // configuration.
  static const AllocatorDispatch default_dispatch;

  // Optimizes this AllocatorDispatch in order to avoid function-call
  // trampolines, i.e. just calling `next->alloc_function`, etc.
  //
  // Given the two tables (`this` and `next`) as follows in pseudo code:
  //     this = {this_alloc, nullptr}
  //     next = {next_alloc, next_free}
  // this optimization produces the following table:
  //     this = {this_alloc, next_free}
  // which is more efficient than having {this_alloc, this_free} where
  // this_free is a function just calls next_free.
  //
  // Given its performance sensitivity, it is recommended to use tail-call
  // optimizations wherever possible. Use MUSTTAIL on return statements in
  // AllocatorDispatch.
  //
  // Note that this optimization works well because there is no case to remove
  // a shim in the middle of the allocator shim chain nor to reorder the shims
  // in the chain. `RemoveAllocatorDispatchForTesting` is the only case that
  // removes a shim, and it removes a shim from the chain head.
  //
  // As of 2024 Apr, on the mac-m1_mini_2020-perf bot, this optimization
  // improves Speedometer3 score by 0.1+% per a trampoline shim.
  //
  // Args:
  //     this = The AllocatorDispatch to be optimized.
  //     original_table = A copy of the original state of `this`. This is
  //         necessary because of a race failure in `InsertAllocatorDispatch`.
  //     next_table = A table that this->next will point to.
  void OptimizeAllocatorDispatchTable(const AllocatorDispatch* original_table,
                                      const AllocatorDispatch* next_table) {
    // `original_table` must be a copy of `this`, not `this` itself.
    PA_DCHECK(this != original_table);

#define COPY_IF_NULLPTR(FUNC)    \
  do {                           \
    if (!original_table->FUNC) { \
      FUNC = next_table->FUNC;   \
    }                            \
  } while (false)

    COPY_IF_NULLPTR(alloc_function);
    COPY_IF_NULLPTR(alloc_unchecked_function);
    COPY_IF_NULLPTR(alloc_zero_initialized_function);
    COPY_IF_NULLPTR(alloc_aligned_function);
    COPY_IF_NULLPTR(realloc_function);
    COPY_IF_NULLPTR(realloc_unchecked_function);
    COPY_IF_NULLPTR(free_function);
    COPY_IF_NULLPTR(free_with_size_function);
    COPY_IF_NULLPTR(free_with_alignment_function);
    COPY_IF_NULLPTR(free_with_size_and_alignment_function);
    COPY_IF_NULLPTR(get_size_estimate_function);
    COPY_IF_NULLPTR(good_size_function);
    COPY_IF_NULLPTR(claimed_address_function);
    COPY_IF_NULLPTR(batch_malloc_function);
    COPY_IF_NULLPTR(batch_free_function);
    COPY_IF_NULLPTR(try_free_default_function);
    COPY_IF_NULLPTR(aligned_malloc_function);
    COPY_IF_NULLPTR(aligned_malloc_unchecked_function);
    COPY_IF_NULLPTR(aligned_realloc_function);
    COPY_IF_NULLPTR(aligned_realloc_unchecked_function);
    COPY_IF_NULLPTR(aligned_free_function);

#undef COPY_IF_NULLPTR
  }
};

}  // namespace allocator_shim

#endif  // PARTITION_ALLOC_SHIM_ALLOCATOR_DISPATCH_H_
