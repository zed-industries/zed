// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ALLOCATOR_DISPATCHER_INTERNAL_DISPATCHER_INTERNAL_H_
#define BASE_ALLOCATOR_DISPATCHER_INTERNAL_DISPATCHER_INTERNAL_H_

#include "base/allocator/dispatcher/configuration.h"
#include "base/allocator/dispatcher/internal/dispatch_data.h"
#include "base/allocator/dispatcher/internal/tools.h"
#include "base/allocator/dispatcher/memory_tagging.h"
#include "base/allocator/dispatcher/notification_data.h"
#include "base/allocator/dispatcher/subsystem.h"
#include "base/check.h"
#include "base/compiler_specific.h"
#include "partition_alloc/buildflags.h"

#if PA_BUILDFLAG(USE_PARTITION_ALLOC)
#include "partition_alloc/partition_alloc_allocation_data.h"  // nogncheck
#endif

#if PA_BUILDFLAG(USE_ALLOCATOR_SHIM)
#include "partition_alloc/shim/allocator_shim.h"
#endif

#include <tuple>

namespace base::allocator::dispatcher::internal {

#if PA_BUILDFLAG(USE_ALLOCATOR_SHIM)
using allocator_shim::AllocatorDispatch;
#endif

template <typename CheckObserverPredicate,
          typename... ObserverTypes,
          size_t... Indices>
void inline PerformObserverCheck(const std::tuple<ObserverTypes...>& observers,
                                 std::index_sequence<Indices...>,
                                 CheckObserverPredicate check_observer) {
  ([](bool b) { DCHECK(b); }(check_observer(std::get<Indices>(observers))),
   ...);
}

template <typename... ObserverTypes, size_t... Indices>
ALWAYS_INLINE void PerformAllocationNotification(
    const std::tuple<ObserverTypes...>& observers,
    std::index_sequence<Indices...>,
    const AllocationNotificationData& notification_data) {
  ((std::get<Indices>(observers)->OnAllocation(notification_data)), ...);
}

template <typename... ObserverTypes, size_t... Indices>
ALWAYS_INLINE void PerformFreeNotification(
    const std::tuple<ObserverTypes...>& observers,
    std::index_sequence<Indices...>,
    const FreeNotificationData& notification_data) {
  ((std::get<Indices>(observers)->OnFree(notification_data)), ...);
}

// DispatcherImpl provides hooks into the various memory subsystems. These hooks
// are responsible for dispatching any notification to the observers.
// In order to provide as many information on the exact type of the observer and
// prevent any conditional jumps in the hot allocation path, observers are
// stored in a std::tuple. DispatcherImpl performs a CHECK at initialization
// time to ensure they are valid.
template <typename... ObserverTypes>
struct DispatcherImpl {
  using AllObservers = std::index_sequence_for<ObserverTypes...>;

  template <std::enable_if_t<
                internal::LessEqual(sizeof...(ObserverTypes),
                                    configuration::kMaximumNumberOfObservers),
                bool> = true>
  static DispatchData GetNotificationHooks(
      std::tuple<ObserverTypes*...> observers) {
    s_observers = std::move(observers);

    PerformObserverCheck(s_observers, AllObservers{}, IsValidObserver{});

    return CreateDispatchData();
  }

 private:
  static DispatchData CreateDispatchData() {
    return DispatchData()
#if PA_BUILDFLAG(USE_PARTITION_ALLOC)
        .SetAllocationObserverHooks(&PartitionAllocatorAllocationHook,
                                    &PartitionAllocatorFreeHook)
#endif
#if PA_BUILDFLAG(USE_ALLOCATOR_SHIM)
        .SetAllocatorDispatch(&allocator_dispatch_)
#endif
        ;
  }

#if PA_BUILDFLAG(USE_PARTITION_ALLOC)
  static void PartitionAllocatorAllocationHook(
      const partition_alloc::AllocationNotificationData& pa_notification_data) {
    AllocationNotificationData dispatcher_notification_data(
        pa_notification_data.address(), pa_notification_data.size(),
        pa_notification_data.type_name(),
        AllocationSubsystem::kPartitionAllocator);

#if PA_BUILDFLAG(HAS_MEMORY_TAGGING)
    dispatcher_notification_data.SetMteReportingMode(
        ConvertToMTEMode(pa_notification_data.mte_reporting_mode()));
#endif

    DoNotifyAllocation(dispatcher_notification_data);
  }

  static void PartitionAllocatorFreeHook(
      const partition_alloc::FreeNotificationData& pa_notification_data) {
    FreeNotificationData dispatcher_notification_data(
        pa_notification_data.address(),
        AllocationSubsystem::kPartitionAllocator);

#if PA_BUILDFLAG(HAS_MEMORY_TAGGING)
    dispatcher_notification_data.SetMteReportingMode(
        ConvertToMTEMode(pa_notification_data.mte_reporting_mode()));
#endif

    DoNotifyFree(dispatcher_notification_data);
  }
#endif  // PA_BUILDFLAG(USE_PARTITION_ALLOC)

#if PA_BUILDFLAG(USE_ALLOCATOR_SHIM)
  static void* AllocFn(size_t size, void* context) {
    void* const address =
        allocator_dispatch_.next->alloc_function(size, context);

    DoNotifyAllocationForShim(address, size);

    return address;
  }

  static void* AllocUncheckedFn(size_t size, void* context) {
    void* const address =
        allocator_dispatch_.next->alloc_unchecked_function(size, context);

    DoNotifyAllocationForShim(address, size);

    return address;
  }

  static void* AllocZeroInitializedFn(size_t n, size_t size, void* context) {
    void* const address =
        allocator_dispatch_.next->alloc_zero_initialized_function(n, size,
                                                                  context);

    DoNotifyAllocationForShim(address, n * size);

    return address;
  }

  static void* AllocAlignedFn(size_t alignment, size_t size, void* context) {
    void* const address = allocator_dispatch_.next->alloc_aligned_function(
        alignment, size, context);

    DoNotifyAllocationForShim(address, size);

    return address;
  }

  static void* ReallocFn(void* address, size_t size, void* context) {
    // Note: size == 0 actually performs free.
    DoNotifyFreeForShim(address);
    void* const reallocated_address =
        allocator_dispatch_.next->realloc_function(address, size, context);

    DoNotifyAllocationForShim(reallocated_address, size);

    return reallocated_address;
  }

  static void* ReallocUncheckedFn(void* address, size_t size, void* context) {
    // Note: size == 0 actually performs free.
    DoNotifyFreeForShim(address);
    void* const reallocated_address =
        allocator_dispatch_.next->realloc_unchecked_function(address, size,
                                                             context);

    DoNotifyAllocationForShim(reallocated_address, size);

    return reallocated_address;
  }

  static void FreeFn(void* address, void* context) {
    // Note: DoNotifyFree should be called before free_function (here and in
    // other places). That is because observers need to handle the allocation
    // being freed before calling free_function, as once the latter is executed
    // the address becomes available and can be allocated by another thread.
    // That would be racy otherwise.
    DoNotifyFreeForShim(address);
    MUSTTAIL return allocator_dispatch_.next->free_function(address, context);
  }

  static void FreeWithSizeFn(void* address, size_t size, void* context) {
    DoNotifyFreeForShim(address);
    MUSTTAIL return allocator_dispatch_.next->free_with_size_function(
        address, size, context);
  }

  static void FreeWithAlignmentFn(void* address,
                                  size_t alignment,
                                  void* context) {
    DoNotifyFreeForShim(address);
    MUSTTAIL return allocator_dispatch_.next->free_with_alignment_function(
        address, alignment, context);
  }

  static void FreeWithSizeAndAlignmentFn(void* address,
                                         size_t size,
                                         size_t alignment,
                                         void* context) {
    DoNotifyFreeForShim(address);
    MUSTTAIL return allocator_dispatch_.next
        ->free_with_size_and_alignment_function(address, size, alignment,
                                                context);
  }

  static unsigned BatchMallocFn(size_t size,
                                void** results,
                                unsigned num_requested,
                                void* context) {
    unsigned const num_allocated =
        allocator_dispatch_.next->batch_malloc_function(size, results,
                                                        num_requested, context);
    for (unsigned i = 0; i < num_allocated; ++i) {
      DoNotifyAllocationForShim(results[i], size);
    }
    return num_allocated;
  }

  static void BatchFreeFn(void** to_be_freed,
                          unsigned num_to_be_freed,
                          void* context) {
    for (unsigned i = 0; i < num_to_be_freed; ++i) {
      DoNotifyFreeForShim(to_be_freed[i]);
    }

    MUSTTAIL return allocator_dispatch_.next->batch_free_function(
        to_be_freed, num_to_be_freed, context);
  }

  static void TryFreeDefaultFn(void* address, void* context) {
    DoNotifyFreeForShim(address);
    MUSTTAIL return allocator_dispatch_.next->try_free_default_function(
        address, context);
  }

  static void* AlignedMallocFn(size_t size, size_t alignment, void* context) {
    void* const address = allocator_dispatch_.next->aligned_malloc_function(
        size, alignment, context);

    DoNotifyAllocationForShim(address, size);

    return address;
  }

  static void* AlignedMallocUncheckedFn(size_t size,
                                        size_t alignment,
                                        void* context) {
    void* const address =
        allocator_dispatch_.next->aligned_malloc_unchecked_function(
            size, alignment, context);

    DoNotifyAllocationForShim(address, size);

    return address;
  }

  static void* AlignedReallocFn(void* address,
                                size_t size,
                                size_t alignment,
                                void* context) {
    // Note: size == 0 actually performs free.
    DoNotifyFreeForShim(address);
    address = allocator_dispatch_.next->aligned_realloc_function(
        address, size, alignment, context);

    DoNotifyAllocationForShim(address, size);

    return address;
  }

  static void* AlignedReallocUncheckedFn(void* address,
                                         size_t size,
                                         size_t alignment,
                                         void* context) {
    // Note: size == 0 actually performs free.
    DoNotifyFreeForShim(address);
    address = allocator_dispatch_.next->aligned_realloc_unchecked_function(
        address, size, alignment, context);

    DoNotifyAllocationForShim(address, size);

    return address;
  }

  static void AlignedFreeFn(void* address, void* context) {
    DoNotifyFreeForShim(address);
    MUSTTAIL return allocator_dispatch_.next->aligned_free_function(address,
                                                                    context);
  }

  ALWAYS_INLINE static void DoNotifyAllocationForShim(void* address,
                                                      size_t size) {
    AllocationNotificationData notification_data(
        address, size, nullptr, AllocationSubsystem::kAllocatorShim);

    DoNotifyAllocation(notification_data);
  }

  ALWAYS_INLINE static void DoNotifyFreeForShim(void* address) {
    FreeNotificationData notification_data(address,
                                           AllocationSubsystem::kAllocatorShim);

    DoNotifyFree(notification_data);
  }

  static AllocatorDispatch allocator_dispatch_;
#endif  // PA_BUILDFLAG(USE_ALLOCATOR_SHIM)

  ALWAYS_INLINE static void DoNotifyAllocation(
      const AllocationNotificationData& notification_data) {
    PerformAllocationNotification(s_observers, AllObservers{},
                                  notification_data);
  }

  ALWAYS_INLINE static void DoNotifyFree(
      const FreeNotificationData& notification_data) {
    PerformFreeNotification(s_observers, AllObservers{}, notification_data);
  }

  static std::tuple<ObserverTypes*...> s_observers;
};

template <typename... ObserverTypes>
std::tuple<ObserverTypes*...> DispatcherImpl<ObserverTypes...>::s_observers;

#if PA_BUILDFLAG(USE_ALLOCATOR_SHIM)
template <typename... ObserverTypes>
AllocatorDispatch DispatcherImpl<ObserverTypes...>::allocator_dispatch_ = {
    AllocFn,                     // alloc_function
    AllocUncheckedFn,            // alloc_unchecked_function
    AllocZeroInitializedFn,      // alloc_zero_initialized_function
    AllocAlignedFn,              // alloc_aligned_function
    ReallocFn,                   // realloc_function
    ReallocUncheckedFn,          // realloc_unchecked_function
    FreeFn,                      // free_function
    FreeWithSizeFn,              // free_with_size_function
    FreeWithAlignmentFn,         // free_with_alignment_function
    FreeWithSizeAndAlignmentFn,  // free_with_size_and_alignment_function
    nullptr,                     // get_size_estimate_function
    nullptr,                     // good_size_function
    nullptr,                     // claimed_address_function
    BatchMallocFn,               // batch_malloc_function
    BatchFreeFn,                 // batch_free_function
    TryFreeDefaultFn,            // try_free_default_function
    AlignedMallocFn,             // aligned_malloc_function
    AlignedMallocUncheckedFn,    // aligned_malloc_unchecked_function
    AlignedReallocFn,            // aligned_realloc_function
    AlignedReallocUncheckedFn,   // aligned_realloc_unchecked_function
    AlignedFreeFn,               // aligned_free_function
    nullptr                      // next
};
#endif  // PA_BUILDFLAG(USE_ALLOCATOR_SHIM)

// Specialization of DispatcherImpl in case we have no observers to notify. In
// this special case we return a set of null pointers as the Dispatcher must not
// install any hooks at all.
template <>
struct DispatcherImpl<> {
  static DispatchData GetNotificationHooks(std::tuple<> /*observers*/) {
    return DispatchData()
#if PA_BUILDFLAG(USE_PARTITION_ALLOC)
        .SetAllocationObserverHooks(nullptr, nullptr)
#endif
#if PA_BUILDFLAG(USE_ALLOCATOR_SHIM)
        .SetAllocatorDispatch(nullptr)
#endif
        ;
  }
};

// A little utility function that helps using DispatcherImpl by providing
// automated type deduction for templates.
template <typename... ObserverTypes>
inline DispatchData GetNotificationHooks(
    std::tuple<ObserverTypes*...> observers) {
  return DispatcherImpl<ObserverTypes...>::GetNotificationHooks(
      std::move(observers));
}

}  // namespace base::allocator::dispatcher::internal

#endif  // BASE_ALLOCATOR_DISPATCHER_INTERNAL_DISPATCHER_INTERNAL_H_
