// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MEMORY_DISCARDABLE_SHARED_MEMORY_H_
#define BASE_MEMORY_DISCARDABLE_SHARED_MEMORY_H_

#include <stddef.h>

#include "base/base_export.h"
#include "base/compiler_specific.h"
#include "base/containers/span.h"
#include "base/dcheck_is_on.h"
#include "base/memory/shared_memory_mapping.h"
#include "base/memory/unsafe_shared_memory_region.h"
#include "base/threading/thread_collision_warner.h"
#include "base/time/time.h"
#include "build/build_config.h"

#if DCHECK_IS_ON()
#include <set>
#endif

// Linux (including Android) support the MADV_REMOVE argument with madvise()
// which has the behavior of reliably causing zero-fill-on-demand pages to
// be returned after a call. Here we define
// DISCARDABLE_SHARED_MEMORY_ZERO_FILL_ON_DEMAND_PAGES_AFTER_PURGE on Linux
// and Android to indicate that this type of behavior can be expected on
// those platforms. Note that madvise() will still be used on other POSIX
// platforms but doesn't provide the zero-fill-on-demand pages guarantee.
#if BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS) || BUILDFLAG(IS_ANDROID)
#define DISCARDABLE_SHARED_MEMORY_ZERO_FILL_ON_DEMAND_PAGES_AFTER_PURGE
#endif

namespace base {

namespace trace_event {
class MemoryAllocatorDump;
class ProcessMemoryDump;
}  // namespace trace_event

// Platform abstraction for discardable shared memory.
//
// This class is not thread-safe. Clients are responsible for synchronizing
// access to an instance of this class.
class BASE_EXPORT DiscardableSharedMemory {
 public:
  enum LockResult { SUCCESS, PURGED, FAILED };

  DiscardableSharedMemory();

  // Create a new DiscardableSharedMemory object from an existing, open shared
  // memory file. Memory must be locked.
  explicit DiscardableSharedMemory(UnsafeSharedMemoryRegion region);

  DiscardableSharedMemory(const DiscardableSharedMemory&) = delete;
  DiscardableSharedMemory& operator=(const DiscardableSharedMemory&) = delete;

  // Closes any open files.
  virtual ~DiscardableSharedMemory();

  // Creates and maps a locked DiscardableSharedMemory object with |size|.
  // Returns true on success and false on failure.
  bool CreateAndMap(size_t size);

  // Maps the locked discardable memory into the caller's address space.
  // Returns true on success, false otherwise.
  bool Map(size_t size);

  // Unmaps the discardable shared memory from the caller's address space.
  // Unmapping won't unlock previously locked range.
  // Returns true if successful; returns false on error or if the memory is
  // not mapped.
  bool Unmap();

  // The actual size of the mapped memory (may be larger than requested). It is
  // 0 when the memory has not been mapped via `Map()`.
  size_t mapped_size() const {
    if (shared_memory_mapping_.IsValid()) {
      return mapped_memory().size();
    } else {
      return 0u;
    }
  }

  // Returns a duplicated shared memory region for this DiscardableSharedMemory
  // object.
  UnsafeSharedMemoryRegion DuplicateRegion() const {
    return shared_memory_region_.Duplicate();
  }

  // Returns an ID for the shared memory region. This is ID of the mapped region
  // consistent across all processes and is valid as long as the region is not
  // unmapped.
  const UnguessableToken& mapped_id() const LIFETIME_BOUND {
    return shared_memory_mapping_.guid();
  }

  // Locks a range of memory so that it will not be purged by the system.
  // The range of memory must be unlocked. The result of trying to lock an
  // already locked range is undefined. |offset| and |length| must both be
  // a multiple of the page size as returned by GetPageSize().
  // Passing 0 for |length| means "everything onward".
  // Returns SUCCESS if range was successfully locked and the memory is still
  // resident, PURGED if range was successfully locked but has been purged
  // since last time it was locked and FAILED if range could not be locked.
  // Locking can fail for two reasons; object might have been purged, our
  // last known usage timestamp might be out of date. Last known usage time
  // is updated to the actual last usage timestamp if memory is still resident
  // or 0 if not.
  LockResult Lock(size_t offset, size_t length);

  // Unlock a previously successfully locked range of memory. The range of
  // memory must be locked. The result of trying to unlock a not
  // previously locked range is undefined.
  // |offset| and |length| must both be a multiple of the page size as returned
  // by GetPageSize().
  // Passing 0 for |length| means "everything onward".
  void Unlock(size_t offset, size_t length);

  // Gets a span over the opened discardable memory space. Discardable memory
  // must have been mapped via Map().
  //
  // This gives the logical memory region, matching the size of what was
  // requested. The actual mapped memory may be larger due to system alignment
  // requirements. See `SharedMemoryMapping::size()` vs
  // `SharedMemoryMapping::mapped_size()`.
  span<uint8_t> memory();
  span<const uint8_t> memory() const;

  // Returns the last known usage time for DiscardableSharedMemory object. This
  // may be earlier than the "true" usage time when memory has been used by a
  // different process. Returns NULL time if purged.
  Time last_known_usage() const { return last_known_usage_; }

  // This returns true and sets |last_known_usage_| to 0 if
  // DiscardableSharedMemory object was successfully purged. Purging can fail
  // for two reasons; object might be locked or our last known usage timestamp
  // might be out of date. Last known usage time is updated to |current_time|
  // if locked or the actual last usage timestamp if unlocked. It is often
  // necessary to call this function twice for the object to successfully be
  // purged. First call, updates |last_known_usage_|. Second call, successfully
  // purges the object using the updated |last_known_usage_|.
  // Note: there is no guarantee that multiple calls to this function will
  // successfully purge object. DiscardableSharedMemory object might be locked
  // or another thread/process might be able to lock and unlock it in between
  // each call.
  bool Purge(Time current_time);

  // Returns true if memory is still resident.
  bool IsMemoryResident() const;

  // Returns true if memory is locked.
  bool IsMemoryLocked() const;

  // Closes the open discardable memory segment.
  // It is safe to call Close repeatedly.
  void Close();

  // For tracing: Creates ownership edge to the underlying shared memory dump
  // which is cross process in the given |pmd|. |local_segment_dump| is the dump
  // associated with the local discardable shared memory segment and |is_owned|
  // is true when the current process owns the segment and the effective memory
  // is assigned to the current process.
  void CreateSharedMemoryOwnershipEdge(
      trace_event::MemoryAllocatorDump* local_segment_dump,
      trace_event::ProcessMemoryDump* pmd,
      bool is_owned) const;

#if BUILDFLAG(IS_ANDROID)
  // Returns true if the Ashmem device is supported on this system.
  // Only use this for unit-testing.
  static bool IsAshmemDeviceSupportedForTesting();
#endif

 private:
  // Returns the full mapped memory region after the internal bookkeeping
  // header. This may be larger than the region exposed through `memory()` due
  // to platform alignment requirements. Discardable memory must have been
  // mapped via Map().
  span<uint8_t> mapped_memory();
  span<const uint8_t> mapped_memory() const;

  // LockPages/UnlockPages are platform-native discardable page management
  // helper functions. Both expect |offset| to be specified relative to the
  // base address at which |memory| is mapped, and that |offset| and |length|
  // are page-aligned by the caller.
  // Returns SUCCESS on platforms which do not support discardable pages.
  static LockResult LockPages(const UnsafeSharedMemoryRegion& region,
                              size_t offset,
                              size_t length);
  // UnlockPages() is a no-op on platforms not supporting discardable pages.
  static void UnlockPages(const UnsafeSharedMemoryRegion& region,
                          size_t offset,
                          size_t length);

  // Virtual for tests.
  virtual Time Now() const;

  UnsafeSharedMemoryRegion shared_memory_region_;
  WritableSharedMemoryMapping shared_memory_mapping_;
  size_t locked_page_count_ = 0u;
#if DCHECK_IS_ON()
  std::set<size_t> locked_pages_;
#endif
  // Implementation is not thread-safe but still usable if clients are
  // synchronized somehow. Use a collision warner to detect incorrect usage.
  DFAKE_MUTEX(thread_collision_warner_);
  Time last_known_usage_;
};

}  // namespace base

#endif  // BASE_MEMORY_DISCARDABLE_SHARED_MEMORY_H_
