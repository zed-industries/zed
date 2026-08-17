// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_PARTITION_ALLOC_HOOKS_H_
#define PARTITION_ALLOC_PARTITION_ALLOC_HOOKS_H_

#include <atomic>
#include <cstddef>

#include "partition_alloc/partition_alloc_base/compiler_specific.h"
#include "partition_alloc/partition_alloc_base/component_export.h"
#include "partition_alloc/partition_alloc_constants.h"

namespace partition_alloc {

class AllocationNotificationData;
class FreeNotificationData;

// PartitionAlloc supports setting hooks to observe allocations/frees as they
// occur as well as 'override' hooks that allow overriding those operations.
class PA_COMPONENT_EXPORT(PARTITION_ALLOC) PartitionAllocHooks {
 public:
  // Log allocation and free events.
  typedef void AllocationObserverHook(
      const AllocationNotificationData& notification_data);
  typedef void FreeObserverHook(const FreeNotificationData& notification_data);

  // If it returns true, the allocation has been overridden with the pointer in
  // *out.
  typedef bool AllocationOverrideHook(void** out,
                                      AllocFlags flags,
                                      size_t size,
                                      const char* type_name);
  // If it returns true, then the allocation was overridden and has been freed.
  typedef bool FreeOverrideHook(void* address);
  // If it returns true, the underlying allocation is overridden and *out holds
  // the size of the underlying allocation.
  typedef bool ReallocOverrideHook(size_t* out, void* address);

  // Special hook type, independent of the rest. Triggered when `free()` detects
  // outstanding references to the allocation.
  // IMPORTANT: Make sure the hook always overwrites `[address, address + size)`
  // with a bit pattern that cannot be interpreted as a valid memory address.
  typedef void QuarantineOverrideHook(void* address, size_t size);

  // To unhook, call Set*Hooks with nullptrs.
  static void SetObserverHooks(AllocationObserverHook* alloc_hook,
                               FreeObserverHook* free_hook);
  static void SetOverrideHooks(AllocationOverrideHook* alloc_hook,
                               FreeOverrideHook* free_hook,
                               ReallocOverrideHook realloc_hook);

  // Helper method to check whether hooks are enabled. This is an optimization
  // so that if a function needs to call observer and override hooks in two
  // different places this value can be cached and only loaded once.
  static bool AreHooksEnabled() {
    return hooks_enabled_.load(std::memory_order_relaxed);
  }

  static void AllocationObserverHookIfEnabled(
      const partition_alloc::AllocationNotificationData& notification_data);
  static bool AllocationOverrideHookIfEnabled(void** out,
                                              AllocFlags flags,
                                              size_t size,
                                              const char* type_name);

  static void FreeObserverHookIfEnabled(
      const FreeNotificationData& notification_data);
  static bool FreeOverrideHookIfEnabled(void* address);

  static void ReallocObserverHookIfEnabled(
      const FreeNotificationData& free_notification_data,
      const AllocationNotificationData& allocation_notification_data);
  static bool ReallocOverrideHookIfEnabled(size_t* out, void* address);

  PA_ALWAYS_INLINE static QuarantineOverrideHook* GetQuarantineOverrideHook() {
    return quarantine_override_hook_.load(std::memory_order_acquire);
  }

  static void SetQuarantineOverrideHook(QuarantineOverrideHook* hook);

 private:
  // Single bool that is used to indicate whether observer or allocation hooks
  // are set to reduce the numbers of loads required to check whether hooking is
  // enabled.
  static std::atomic<bool> hooks_enabled_;

  // Lock used to synchronize Set*Hooks calls.
  static std::atomic<AllocationObserverHook*> allocation_observer_hook_;
  static std::atomic<FreeObserverHook*> free_observer_hook_;

  static std::atomic<AllocationOverrideHook*> allocation_override_hook_;
  static std::atomic<FreeOverrideHook*> free_override_hook_;
  static std::atomic<ReallocOverrideHook*> realloc_override_hook_;

  static std::atomic<QuarantineOverrideHook*> quarantine_override_hook_;
};

}  // namespace partition_alloc

#endif  // PARTITION_ALLOC_PARTITION_ALLOC_HOOKS_H_
