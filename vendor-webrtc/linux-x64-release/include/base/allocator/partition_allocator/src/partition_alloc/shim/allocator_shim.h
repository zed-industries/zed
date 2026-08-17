// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_SHIM_ALLOCATOR_SHIM_H_
#define PARTITION_ALLOC_SHIM_ALLOCATOR_SHIM_H_

#include <cstddef>
#include <cstdint>

#include "partition_alloc/buildflags.h"

#if PA_BUILDFLAG(USE_ALLOCATOR_SHIM)
#include "partition_alloc/build_config.h"
#include "partition_alloc/lightweight_quarantine.h"
#include "partition_alloc/partition_alloc_base/component_export.h"
#include "partition_alloc/partition_alloc_base/types/strong_alias.h"
#include "partition_alloc/shim/allocator_dispatch.h"
#include "partition_alloc/tagging.h"

namespace allocator_shim {

// Allocator Shim API. Allows to:
//  - Configure the behavior of the allocator (what to do on OOM failures).
//  - Install new hooks (AllocatorDispatch) in the allocator chain.

// When this shim layer is enabled, the route of an allocation is as-follows:
//
// [allocator_shim_override_*.h] Intercept malloc() / operator new calls:
//   The override_* headers define the symbols required to intercept calls to
//   malloc() and operator new (if not overridden by specific C++ classes).
//
// [allocator_shim.cc] Routing allocation calls to the shim:
//   The headers above route the calls to the internal ShimMalloc(), ShimFree(),
//   ShimCppNew() etc. methods defined in allocator_shim.cc.
//   These methods will: (1) forward the allocation call to the front of the
//   AllocatorDispatch chain. (2) perform security hardenings (e.g., might
//   call std::new_handler on OOM failure).
//
// [allocator_shim_default_dispatch_to_*.cc] The AllocatorDispatch chain:
//   It is a singly linked list where each element is a struct with function
//   pointers (|malloc_function|, |free_function|, etc). Normally the chain
//   consists of a single AllocatorDispatch element, herein called
//   the "default dispatch", which is statically defined at build time and
//   ultimately routes the calls to the actual allocator defined by the build
//   config (glibc, ...).
//
// It is possible to dynamically insert further AllocatorDispatch stages
// to the front of the chain, for debugging / profiling purposes.
//
// All the functions must be thread safe. The shim does not enforce any
// serialization. This is to route to thread-aware allocators without
// introducing unnecessary perf hits.

// When true makes malloc behave like new, w.r.t calling the new_handler if
// the allocation fails (see set_new_mode() in Windows).
PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
void SetCallNewHandlerOnMallocFailure(bool value);

// Allocates |size| bytes or returns nullptr. It does NOT call the new_handler,
// regardless of SetCallNewHandlerOnMallocFailure().
PA_COMPONENT_EXPORT(ALLOCATOR_SHIM) void* UncheckedAlloc(size_t size);

// Reallocates |ptr| to point at |size| bytes with the same alignment as |ptr|,
// or returns nullptr while leaving the |ptr| unchanged. It does NOT call the
// new_handler, regardless of SetCallNewHandlerOnMallocFailure().
PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
void* UncheckedRealloc(void* ptr, size_t size);

// Frees memory allocated with UncheckedAlloc().
PA_COMPONENT_EXPORT(ALLOCATOR_SHIM) void UncheckedFree(void* ptr);

#if PA_BUILDFLAG(USE_PARTITION_ALLOC_AS_MALLOC)

// The aligned allocation functions are only available when PartitionAlloc is
// acting as malloc. Otherwise there may be nothing to forward them to for the
// platform allocator.

// Allocates |size| bytes aligned to |align| or returns nullptr. It does NOT
// call the new_handler, regardless of SetCallNewHandlerOnMallocFailure().
PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
void* UncheckedAlignedAlloc(size_t align, size_t size);

// Reallocates |ptr| to point at |size| bytes with an alignment of |align|,
// or returns nullptr while leaving the |ptr| unchanged. It does NOT call the
// new_handler, regardless of SetCallNewHandlerOnMallocFailure().
PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
void* UncheckedAlignedRealloc(void* ptr, size_t size, size_t align);

// Frees memory allocated with UncheckedAlignedAlloc().
PA_COMPONENT_EXPORT(ALLOCATOR_SHIM) void UncheckedAlignedFree(void* ptr);

#endif

// Inserts |dispatch| in front of the allocator chain. This method is
// thread-safe w.r.t concurrent invocations of InsertAllocatorDispatch().
// The callers have responsibility for inserting a single dispatch no more
// than once.
PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
void InsertAllocatorDispatch(AllocatorDispatch* dispatch);

// Test-only. Rationale: (1) lack of use cases; (2) dealing safely with a
// removal of arbitrary elements from a singly linked list would require a lock
// in malloc(), which we really don't want.
PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
void RemoveAllocatorDispatchForTesting(AllocatorDispatch* dispatch);

PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
const AllocatorDispatch* GetAllocatorDispatchChainHeadForTesting();

class PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
    AutoResetAllocatorDispatchChainForTesting {
 public:
  AutoResetAllocatorDispatchChainForTesting();
  ~AutoResetAllocatorDispatchChainForTesting();

 private:
  const allocator_shim::AllocatorDispatch* original_dispatch_;
};

#if PA_BUILDFLAG(IS_APPLE)
// The fallback function to be called when try_free_default_function receives a
// pointer which doesn't belong to the allocator.
PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
void TryFreeDefaultFallbackToFindZoneAndFree(void* ptr);
#endif  // PA_BUILDFLAG(IS_APPLE)

#if PA_BUILDFLAG(IS_APPLE)
#if PA_BUILDFLAG(USE_PARTITION_ALLOC_AS_MALLOC)
PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
void InitializeDefaultAllocatorPartitionRoot();
bool IsDefaultAllocatorPartitionRootInitialized();
#endif  // PA_BUILDFLAG(USE_PARTITION_ALLOC_AS_MALLOC)
// On macOS, the allocator shim needs to be turned on during runtime.
PA_COMPONENT_EXPORT(ALLOCATOR_SHIM) void InitializeAllocatorShim();
#endif  // PA_BUILDFLAG(IS_APPLE)

#if PA_BUILDFLAG(USE_PARTITION_ALLOC_AS_MALLOC)
PA_COMPONENT_EXPORT(ALLOCATOR_SHIM) void EnablePartitionAllocMemoryReclaimer();

using EnableBrp =
    partition_alloc::internal::base::StrongAlias<class EnableBrpTag, bool>;
using EnableMemoryTagging =
    partition_alloc::internal::base::StrongAlias<class EnableMemoryTaggingTag,
                                                 bool>;
enum class BucketDistribution : uint8_t { kNeutral, kDenser };
using EventuallyZeroFreedMemory = partition_alloc::internal::base::
    StrongAlias<class EventuallyZeroFreedMemoryTag, bool>;
using FewerMemoryRegions =
    partition_alloc::internal::base::StrongAlias<class FewerMemoryRegionsTag,
                                                 bool>;
using UseSmallSingleSlotSpans = partition_alloc::internal::base::
    StrongAlias<class UseSmallSingleSlotSpansTag, bool>;

// If |thread_cache_on_non_quarantinable_partition| is specified, the
// thread-cache will be enabled on the non-quarantinable partition. The
// thread-cache on the main (malloc) partition will be disabled.
PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
void ConfigurePartitions(
    EnableBrp enable_brp,
    size_t brp_extra_extras_size,
    EnableMemoryTagging enable_memory_tagging,
    partition_alloc::TagViolationReportingMode memory_tagging_reporting_mode,
    BucketDistribution distribution,
    partition_alloc::internal::SchedulerLoopQuarantineConfig
        scheduler_loop_quarantine_global_config,
    partition_alloc::internal::SchedulerLoopQuarantineConfig
        scheduler_loop_quarantine_thread_local_config,
    EventuallyZeroFreedMemory eventually_zero_freed_memory,
    FewerMemoryRegions fewer_memory_regions,
    UseSmallSingleSlotSpans use_small_single_slot_spans);

PA_COMPONENT_EXPORT(ALLOCATOR_SHIM) uint32_t GetMainPartitionRootExtrasSize();

PA_COMPONENT_EXPORT(ALLOCATOR_SHIM) void AdjustDefaultAllocatorForForeground();
PA_COMPONENT_EXPORT(ALLOCATOR_SHIM) void AdjustDefaultAllocatorForBackground();

#endif  // PA_BUILDFLAG(USE_PARTITION_ALLOC_AS_MALLOC)

}  // namespace allocator_shim

#endif  // PA_BUILDFLAG(USE_ALLOCATOR_SHIM)

#endif  // PARTITION_ALLOC_SHIM_ALLOCATOR_SHIM_H_
