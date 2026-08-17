// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifdef PARTITION_ALLOC_SHIM_ALLOCATOR_SHIM_OVERRIDE_APPLE_DEFAULT_ZONE_H_
#error This header is meant to be included only once by allocator_shim.cc
#endif

#ifndef PARTITION_ALLOC_SHIM_ALLOCATOR_SHIM_OVERRIDE_APPLE_DEFAULT_ZONE_H_
#define PARTITION_ALLOC_SHIM_ALLOCATOR_SHIM_OVERRIDE_APPLE_DEFAULT_ZONE_H_

#if !PA_BUILDFLAG(USE_PARTITION_ALLOC_AS_MALLOC)
#error This header must be included iff PartitionAlloc-Everywhere is enabled.
#endif

#include <atomic>
#include <cstring>
#include <tuple>

#include "partition_alloc/buildflags.h"
#include "partition_alloc/partition_alloc_base/apple/mach_logging.h"
#include "partition_alloc/partition_alloc_base/bits.h"
#include "partition_alloc/partition_alloc_base/logging.h"
#include "partition_alloc/partition_alloc_check.h"
#include "partition_alloc/partition_alloc_constants.h"
#include "partition_alloc/partition_root.h"
#include "partition_alloc/shim/early_zone_registration_constants.h"

namespace allocator_shim {

// This ".h" file is not a header, but a source file meant to be included only
// once, exclusively from allocator_shim.cc. See the top-level check.
//
// A possible alternative: rename this file to .inc, at the expense of losing
// syntax highlighting in text editors.
//
// NOLINTNEXTLINE(google-build-namespaces)
namespace {

// malloc_introspection_t's callback functions for our own zone

kern_return_t MallocIntrospectionEnumerator(task_t task,
                                            void*,
                                            unsigned type_mask,
                                            vm_address_t zone_address,
                                            memory_reader_t reader,
                                            vm_range_recorder_t recorder) {
  // Should enumerate all memory regions allocated by this allocator, but not
  // implemented just because of no use case for now.
  return KERN_FAILURE;
}

size_t MallocIntrospectionGoodSize(malloc_zone_t* zone, size_t size) {
  return ShimGoodSize(size, nullptr);
}

boolean_t MallocIntrospectionCheck(malloc_zone_t* zone) {
  // Should check the consistency of the allocator implementing this malloc
  // zone, but not implemented just because of no use case for now.
  return true;
}

void MallocIntrospectionPrint(malloc_zone_t* zone, boolean_t verbose) {
  // Should print the current states of the zone for debugging / investigation
  // purpose, but not implemented just because of no use case for now.
}

void MallocIntrospectionLog(malloc_zone_t* zone, void* address) {
  // Should enable logging of the activities on the given `address`, but not
  // implemented just because of no use case for now.
}

void MallocIntrospectionForceLock(malloc_zone_t* zone) {
  // Called before fork(2) to acquire the lock.
  partition_alloc::PartitionAllocMallocHookOnBeforeForkInParent();
}

void MallocIntrospectionForceUnlock(malloc_zone_t* zone) {
  // Called in the parent process after fork(2) to release the lock.
  partition_alloc::PartitionAllocMallocHookOnAfterForkInParent();
}

void MallocIntrospectionStatistics(malloc_zone_t* zone,
                                   malloc_statistics_t* stats) {
  // Should report the memory usage correctly, but not implemented just because
  // of no use case for now.
  stats->blocks_in_use = 0;
  stats->size_in_use = 0;
  stats->max_size_in_use = 0;  // High water mark of touched memory
  stats->size_allocated = 0;   // Reserved in memory
}

boolean_t MallocIntrospectionZoneLocked(malloc_zone_t* zone) {
  // Should return true if the underlying PartitionRoot is locked, but not
  // implemented just because this function seems not used effectively.
  return false;
}

boolean_t MallocIntrospectionEnableDischargeChecking(malloc_zone_t* zone) {
  // 'discharge' is not supported.
  return false;
}

void MallocIntrospectionDisableDischargeChecking(malloc_zone_t* zone) {
  // 'discharge' is not supported.
}

void MallocIntrospectionDischarge(malloc_zone_t* zone, void* memory) {
  // 'discharge' is not supported.
}

void MallocIntrospectionEnumerateDischargedPointers(
    malloc_zone_t* zone,
    void (^report_discharged)(void* memory, void* info)) {
  // 'discharge' is not supported.
}

void MallocIntrospectionReinitLock(malloc_zone_t* zone) {
  // Called in a child process after fork(2) to re-initialize the lock.
  partition_alloc::PartitionAllocMallocHookOnAfterForkInChild();
}

void MallocIntrospectionPrintTask(task_t task,
                                  unsigned level,
                                  vm_address_t zone_address,
                                  memory_reader_t reader,
                                  print_task_printer_t printer) {
  // Should print the current states of another process's zone for debugging /
  // investigation purpose, but not implemented just because of no use case
  // for now.
}

void MallocIntrospectionTaskStatistics(task_t task,
                                       vm_address_t zone_address,
                                       memory_reader_t reader,
                                       malloc_statistics_t* stats) {
  // Should report the memory usage in another process's zone, but not
  // implemented just because of no use case for now.
  stats->blocks_in_use = 0;
  stats->size_in_use = 0;
  stats->max_size_in_use = 0;  // High water mark of touched memory
  stats->size_allocated = 0;   // Reserved in memory
}

// malloc_zone_t's callback functions for our own zone

size_t MallocZoneSize(malloc_zone_t* zone, const void* ptr) {
  return ShimGetSizeEstimate(ptr, nullptr);
}

void* MallocZoneMalloc(malloc_zone_t* zone, size_t size) {
  return ShimMalloc(size, nullptr);
}

void* MallocZoneCalloc(malloc_zone_t* zone, size_t n, size_t size) {
  return ShimCalloc(n, size, nullptr);
}

void* MallocZoneValloc(malloc_zone_t* zone, size_t size) {
  return ShimValloc(size, nullptr);
}

void MallocZoneFree(malloc_zone_t* zone, void* ptr) {
  return ShimFree(ptr, nullptr);
}

void* MallocZoneRealloc(malloc_zone_t* zone, void* ptr, size_t size) {
  return ShimRealloc(ptr, size, nullptr);
}

void MallocZoneDestroy(malloc_zone_t* zone) {
  // No support to destroy the zone for now.
}

void* MallocZoneMemalign(malloc_zone_t* zone, size_t alignment, size_t size) {
  return ShimMemalign(alignment, size, nullptr);
}

void MallocZoneFreeDefiniteSize(malloc_zone_t* zone, void* ptr, size_t size) {
  return ShimFreeWithSize(ptr, size, nullptr);
}

unsigned MallocZoneBatchMalloc(malloc_zone_t* zone,
                               size_t size,
                               void** results,
                               unsigned num_requested) {
  return ShimBatchMalloc(size, results, num_requested, nullptr);
}

void MallocZoneBatchFree(malloc_zone_t* zone,
                         void** to_be_freed,
                         unsigned num) {
  return ShimBatchFree(to_be_freed, num, nullptr);
}

boolean_t MallocZoneClaimedAddress(malloc_zone_t* zone, void* ptr) {
  return static_cast<boolean_t>(ShimClaimedAddress(ptr, nullptr));
}

#if PA_TRY_FREE_DEFAULT_IS_AVAILABLE
void MallocZoneTryFreeDefault(malloc_zone_t* zone, void* ptr) {
  return ShimTryFreeDefault(ptr, nullptr);
}
#endif

malloc_introspection_t g_mac_malloc_introspection{};
malloc_zone_t g_mac_malloc_zone{};

malloc_zone_t* GetDefaultMallocZone() {
  // malloc_default_zone() does not return... the default zone, but the initial
  // one. The default one is the first element of the default zone array.
  unsigned int zone_count = 0;
  vm_address_t* zones = nullptr;
  kern_return_t result =
      malloc_get_all_zones(mach_task_self(), nullptr, &zones, &zone_count);
  PA_MACH_CHECK(result == KERN_SUCCESS, result) << "malloc_get_all_zones";
  return reinterpret_cast<malloc_zone_t*>(zones[0]);
}

bool IsAlreadyRegistered() {
  // HACK: This should really only be called once, but it is not.
  //
  // This function is a static constructor of its binary. If it is included in a
  // dynamic library, then the same process may end up executing this code
  // multiple times, once per library. As a consequence, each new library will
  // add its own allocator as the default zone. Aside from splitting the heap
  // further, the main issue arises if/when the last library to be loaded
  // (dlopen()-ed) gets dlclose()-ed.
  //
  // See crbug.com/1271139 for details.
  //
  // In this case, subsequent free() will be routed by libmalloc to the deleted
  // zone (since its code has been unloaded from memory), and crash inside
  // libsystem's free(). This in practice happens as soon as dlclose() is
  // called, inside the dynamic linker (dyld).
  //
  // Since we are talking about different library, and issues inside the dynamic
  // linker, we cannot use a global static variable (which would be
  // per-library), or anything from pthread.
  //
  // The solution used here is to check whether the current default zone is
  // already ours, in which case we are not the first dynamic library here, and
  // should do nothing. This is racy, and hacky.
  vm_address_t* zones = nullptr;
  unsigned int zone_count = 0;
  // *Not* using malloc_default_zone(), as it seems to be hardcoded to return
  // something else than the default zone. See the difference between
  // malloc_default_zone() and inline_malloc_default_zone() in Apple's malloc.c
  // (in libmalloc).
  kern_return_t result =
      malloc_get_all_zones(mach_task_self(), nullptr, &zones, &zone_count);
  PA_MACH_CHECK(result == KERN_SUCCESS, result) << "malloc_get_all_zones";
  // Checking all the zones, in case someone registered their own zone on top of
  // our own.
  for (unsigned int i = 0; i < zone_count; i++) {
    malloc_zone_t* zone = reinterpret_cast<malloc_zone_t*>(zones[i]);

    // strcmp() and not a pointer comparison, as the zone was registered from
    // another library, the pointers don't match.
    if (zone->zone_name &&
        (strcmp(zone->zone_name, kPartitionAllocZoneName) == 0)) {
      // This zone is provided by PartitionAlloc, so this function has been
      // called from another library (or the main executable), nothing to do.
      //
      // This should be a crash, ideally, but callers do it, so only warn, for
      // now.
      PA_RAW_LOG(ERROR,
                 "Trying to load the allocator multiple times. This is *not* "
                 "supported.");
      return true;
    }
  }

  return false;
}

void InitializeZone() {
  g_mac_malloc_introspection.enumerator = MallocIntrospectionEnumerator;
  g_mac_malloc_introspection.good_size = MallocIntrospectionGoodSize;
  g_mac_malloc_introspection.check = MallocIntrospectionCheck;
  g_mac_malloc_introspection.print = MallocIntrospectionPrint;
  g_mac_malloc_introspection.log = MallocIntrospectionLog;
  g_mac_malloc_introspection.force_lock = MallocIntrospectionForceLock;
  g_mac_malloc_introspection.force_unlock = MallocIntrospectionForceUnlock;
  g_mac_malloc_introspection.statistics = MallocIntrospectionStatistics;
  g_mac_malloc_introspection.zone_locked = MallocIntrospectionZoneLocked;
  g_mac_malloc_introspection.enable_discharge_checking =
      MallocIntrospectionEnableDischargeChecking;
  g_mac_malloc_introspection.disable_discharge_checking =
      MallocIntrospectionDisableDischargeChecking;
  g_mac_malloc_introspection.discharge = MallocIntrospectionDischarge;
  g_mac_malloc_introspection.enumerate_discharged_pointers =
      MallocIntrospectionEnumerateDischargedPointers;
  g_mac_malloc_introspection.reinit_lock = MallocIntrospectionReinitLock;
  g_mac_malloc_introspection.print_task = MallocIntrospectionPrintTask;
  g_mac_malloc_introspection.task_statistics =
      MallocIntrospectionTaskStatistics;
  // `version` member indicates which APIs are supported in this zone.
  //   version >= 5: memalign is supported
  //   version >= 6: free_definite_size is supported
  //   version >= 7: introspect's discharge family is supported
  //   version >= 8: pressure_relief is supported
  //   version >= 9: introspect.reinit_lock is supported
  //   version >= 10: claimed_address is supported
  //   version >= 11: introspect.print_task is supported
  //   version >= 12: introspect.task_statistics is supported
  //   version >= 13: try_free_default is supported
  g_mac_malloc_zone.version = kZoneVersion;
  g_mac_malloc_zone.zone_name = kPartitionAllocZoneName;
  g_mac_malloc_zone.introspect = &g_mac_malloc_introspection;
  g_mac_malloc_zone.size = MallocZoneSize;
  g_mac_malloc_zone.malloc = MallocZoneMalloc;
  g_mac_malloc_zone.calloc = MallocZoneCalloc;
  g_mac_malloc_zone.valloc = MallocZoneValloc;
  g_mac_malloc_zone.free = MallocZoneFree;
  g_mac_malloc_zone.realloc = MallocZoneRealloc;
  g_mac_malloc_zone.destroy = MallocZoneDestroy;
  g_mac_malloc_zone.batch_malloc = MallocZoneBatchMalloc;
  g_mac_malloc_zone.batch_free = MallocZoneBatchFree;
  g_mac_malloc_zone.memalign = MallocZoneMemalign;
  g_mac_malloc_zone.free_definite_size = MallocZoneFreeDefiniteSize;
  g_mac_malloc_zone.pressure_relief = nullptr;
  g_mac_malloc_zone.claimed_address = MallocZoneClaimedAddress;
#if PA_TRY_FREE_DEFAULT_IS_AVAILABLE
  g_mac_malloc_zone.try_free_default = MallocZoneTryFreeDefault;
#endif
}

// This ".h" file is not a header, but a source file meant to be included only
// once, exclusively from allocator_shim_win_static.cc or
// allocator_shim_win_component.cc. See the top-level check.
//
// A possible alternative: rename this file to .inc, at the expense of losing
// syntax highlighting in text editors.
//
// NOLINTNEXTLINE(google-build-namespaces)
namespace {
static std::atomic<bool> g_initialization_is_done;
}

// Replaces the default malloc zone with our own malloc zone backed by
// PartitionAlloc.  Since we'd like to make as much code as possible to use our
// own memory allocator (and reduce bugs caused by mixed use of the system
// allocator and our own allocator), run the following function
// `InitializeDefaultAllocatorPartitionRoot` with the highest priority.
//
// Note that, despite of the highest priority of the initialization order,
// [NSThread init] runs before InitializeDefaultMallocZoneWithPartitionAlloc
// unfortunately and allocates memory with the system allocator.  Plus, the
// allocated memory will be deallocated with the default zone's `free` at that
// moment without using a zone dispatcher.  Hence, our own `free` function
// receives an address allocated by the system allocator.
__attribute__((constructor(0))) void
InitializeDefaultMallocZoneWithPartitionAlloc() {
  if (IsAlreadyRegistered()) {
    return;
  }

  // Instantiate the existing regular and purgeable zones in order to make the
  // existing purgeable zone use the existing regular zone since PartitionAlloc
  // doesn't support a purgeable zone.
  std::ignore = malloc_default_zone();
  std::ignore = malloc_default_purgeable_zone();

  // Initialize the default allocator's PartitionRoot with the existing zone.
  InitializeDefaultAllocatorPartitionRoot();

  // Create our own malloc zone.
  InitializeZone();

  malloc_zone_t* system_default_zone = GetDefaultMallocZone();
  if (strcmp(system_default_zone->zone_name, kDelegatingZoneName) == 0) {
    // The first zone is our zone, we can unregister it, replacing it with the
    // new one. This relies on a precise zone setup, done in
    // |EarlyMallocZoneRegistration()|.
    malloc_zone_register(&g_mac_malloc_zone);
    malloc_zone_unregister(system_default_zone);
    g_initialization_is_done.store(true, std::memory_order_release);
    return;
  }

  // Not in the path where the zone was registered early. This is either racy,
  // or fine if the current process is not hosting multiple threads.
  //
  // This path is fine for e.g. most unit tests.
  //
  // Make our own zone the default zone.
  //
  // Put our own zone at the last position, so that it promotes to the default
  // zone.  The implementation logic of malloc_zone_unregister is:
  //   zone_table.swap(unregistered_zone, last_zone);
  //   zone_table.shrink_size_by_1();
  malloc_zone_register(&g_mac_malloc_zone);
  malloc_zone_unregister(system_default_zone);
  // Between malloc_zone_unregister(system_default_zone) (above) and
  // malloc_zone_register(system_default_zone) (below), i.e. while absence of
  // system_default_zone, it's possible that another thread calls free(ptr) and
  // "no zone found" error is hit, crashing the process.
  malloc_zone_register(system_default_zone);

  // Confirm that our own zone is now the default zone.
  PA_CHECK(GetDefaultMallocZone() == &g_mac_malloc_zone);
  g_initialization_is_done.store(true, std::memory_order_release);
}

}  // namespace

bool IsDefaultAllocatorPartitionRootInitialized() {
  // Even though zone registration is not thread-safe, let's not make it worse,
  // and use acquire/release ordering.
  return g_initialization_is_done.load(std::memory_order_acquire);
}

}  // namespace allocator_shim

#endif  // PARTITION_ALLOC_SHIM_ALLOCATOR_SHIM_OVERRIDE_APPLE_DEFAULT_ZONE_H_
